- Ky.#fetch (source/core/Ky.ts:1404-1468)
```typescript
	async #fetch(): Promise<Response> {
		// A previous attempt can return without consuming its upload. Release that unused branch before replacing the request reference.
		if (this.#originalRequest && this.#originalRequest.body !== this.request.body) {
			this.#cancelBody(this.#originalRequest.body ?? undefined);
		}

		const nonRequestOptions = findUnknownOptions(this.#options);
		this.#retryLimit = normalizeRetryOptions(this.#options.retry).limit;
		// Reattach the managed signal because Node.js can garbage-collect the abort controller used by Request.clone().
		const retryRequest = this.#retryLimit > 0 ? this.#withManagedSignal(this.request.clone()) : undefined;
		const request = this.#wrapRequestWithUploadProgress(this.request, this.#options.body ?? undefined);

		// Cloning is done here to prepare in advance for retries.
		// Skip cloning when retries are disabled - cloning a streaming body calls ReadableStream#tee()
		// which buffers the entire stream in memory, causing excessive memory usage for large uploads.
		this.#originalRequest = request;
		if (retryRequest) {
			this.request = retryRequest;
			this.#requestBodyCanBeCancelled = true;
		}

		try {
			const effectiveTimeout = this.#getEffectiveTimeout();

			// Called unbound so a native `window.fetch` is not invoked with the options object as `this`, which throws "Illegal invocation" in browsers.
			const {fetch} = this.#options;
			const response = effectiveTimeout === undefined
				? await fetch(request, nonRequestOptions)
				: await timeout(request, nonRequestOptions, this.#abortController, {
					timeout: effectiveTimeout,
					fetch,
				});

			if (this.#getRemainingTotalTimeout() === 0) {
				this.#abortController?.abort();
				this.#cancelResponseBody(response);
				throw new TimeoutError(request);
			}

			return this.#setResponseRequest(response, request);
		} catch (error) {
			this.#throwIfAbortedByUser();
			if (this.#getRemainingTotalTimeout() === 0) {
				this.#abortController?.abort();
				throw new TimeoutError(request);
			}

			// The upload progress wrapper errors the request body stream when its callback throws, which the runtime reports as a network failure. Surface the callback error instead.
			const progressCallbackError = getProgressCallbackError(this.#originalRequest?.body ?? undefined);
			if (progressCallbackError !== undefined) {
				if (isError(progressCallbackError)) {
					this.#inFlightCallbackErrors.add(progressCallbackError);
				}

				// eslint-disable-next-line @typescript-eslint/only-throw-error -- The callback can throw any value, and non-Error throws are propagated as-is elsewhere.
				throw progressCallbackError;
			}

			if (isRawNetworkError(error)) {
				throw new NetworkError(request, {cause: error as Error});
			}

			throw error;
		}
	}
```
- copyResponseMetadata (source/utils/body.ts:107-171)
```typescript
const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response => {
	const nativeClone = response.clone.bind(response);
	const getProgressError = response.body
		? progressCallbackErrors.get(response.body) ?? (originalResponse.body ? progressCallbackErrors.get(originalResponse.body) : undefined)
		: undefined;
	if (response.body && getProgressError) {
		progressCallbackErrors.set(response.body, getProgressError);
	}

	if (response.body && getError) {
		responseSizeErrors.set(response.body, getError);

		// Chromium replaces stream errors with a generic TypeError in native body methods.
		// Restore the size error, including on clones and download-progress wrappers.
		for (const type of Object.keys(responseTypes) as Array<keyof typeof responseTypes>) {
			if (typeof response[type] !== 'function') {
				continue;
			}

			const nativeMethod = response[type].bind(response);
			Object.defineProperty(response, type, {
				async value() {
					try {
						return await nativeMethod();
					} catch (error) {
						throw getError() ?? error;
					}
				},
				writable: true,
				configurable: true,
			});
		}
	}

	Object.defineProperties(response, {
		// The `Response` constructor cannot set these, so copy them over from the original response.
		url: {value: originalResponse.url},
		redirected: {value: originalResponse.redirected},
		type: {value: originalResponse.type},
		// The constructor also rebuilds the header list under the mutable "response" guard, so a network response
		// that arrived immutable would silently become mutable. Reuse the original's headers to keep the guard.
		headers: {value: originalResponse.headers},
		// Native `clone()` creates a new `Response`, which would drop them again.
		// Keep the shim replaceable like `Response.prototype.clone` for instrumentation and mocks.
		clone: {
			value() {
				const clone = nativeClone();
				// Cloning replaces the original body too, so retain the error accessors on both branches.
				if (response.body && getError) {
					responseSizeErrors.set(response.body, getError);
				}

				if (response.body && getProgressError) {
					progressCallbackErrors.set(response.body, getProgressError);
				}

				return copyResponseMetadata(clone, response);
			},
			writable: true,
			configurable: true,
		},
	});

	return response;
};
```
- Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
```typescript
	async #calculateRetryDelay(error: unknown) {
		if (isError(error) && this.#inFlightCallbackErrors.has(error)) {
			throw error;
		}

		const retry = normalizeRetryOptions(this.#options.retry);
		if (this.#retryCount >= Math.min(retry.limit, this.#retryLimit)) {
			throw error;
		}

		// Wrap non-Error throws to ensure consistent error handling
		const errorObject = error instanceof Error ? error : new NonError(error);

		// Handle forced retry from afterResponse hook - skip method check and shouldRetry
		if (isForceRetryError(errorObject)) {
			return errorObject.customDelay ?? this.#calculateDelay(retry);
		}

		// Check if method is retriable for non-forced retries
		if (!retry.methods.includes(normalizeRetryMethod(this.request.method))) {
			throw error;
		}

		let shouldRetryOverride = false;
		const {shouldRetry} = retry;
		if (shouldRetry !== undefined) {
			const result = await this.#raceWithTotalTimeout(async () => shouldRetry({error: errorObject, retryCount: this.#retryCount + 1}), this.#userProvidedAbortSignal);
			this.#throwIfAbortedByUser();
			if (result === timedOutOperation) {
				throw new TimeoutError(this.#sentRequest);
			}

			// Only exact booleans override the default retry checks.
			if (result === false) {
				throw error;
			}

			shouldRetryOverride = result === true;
		}

		// Default timeout behavior
		if (isTimeoutError(error)) {
			if (!shouldRetryOverride && !retry.retryOnTimeout) {
				throw error;
			}

			return this.#calculateDelay(retry);
		}

		// The `isHTTPError()` brand only checks `name`, so a cross-realm error may not carry a `response`.
		if (isHTTPError(error) && error.response) {
			if (!shouldRetryOverride && !retry.statusCodes.includes(error.response.status)) {
				throw error;
			}

			const retryTimingHeader = getRetryTimingHeader(error.response.headers);
			if (retryTimingHeader && retry.afterStatusCodes.includes(error.response.status)) {
				const after = calculateRetryTimingDelay(retryTimingHeader);
				if (after === undefined) {
					// Malformed retry timing headers should not disable retries; they only lose their server-provided timing.
					return this.#calculateDelay(retry);
				}

				// Don't apply jitter when server provides explicit retry timing
				return Math.min(retry.maxRetryAfter, after);
			}

			if (!shouldRetryOverride && error.response.status === 413) {
				throw error;
			}

			return this.#calculateDelay(retry);
		}

		// Only retry known retriable error types. Unknown errors (e.g., programming bugs) are not retried.
		if (!shouldRetryOverride && !isNetworkError(error)) {
			throw error;
		}

		return this.#calculateDelay(retry);
	}
```
- getProgressCallbackError (source/utils/body.ts:104-105)
```typescript
export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
	stream ? progressCallbackErrors.get(stream)?.() : undefined;
```
- Ky.#raceBodyRead (source/core/Ky.ts:913-979)
```typescript
	async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
		const failedRequest = this.#getResponseRequest(response);
		let timeoutMs: number | undefined;
		try {
			timeoutMs = this.#getEffectiveTimeout();
		} catch (error: unknown) {
			await this.#throwProcessedError(error, failedRequest);
		}

		const bodyPromise = createBodyPromise();
		const timeoutPromise = timeoutMs === undefined
			? undefined
			: new Promise<typeof timedOutResponseData>(resolve => {
				const timeoutId = setTimeout(() => {
					resolve(timedOutResponseData);
				}, timeoutMs);
				void bodyPromise.finally(() => {
					clearTimeout(timeoutId);
				}).catch(() => undefined);
			});

		let result: unknown;
		try {
			result = timeoutPromise === undefined
				? await bodyPromise
				: await Promise.race([bodyPromise, timeoutPromise]);
		} catch (error: unknown) {
			if (this.#userProvidedAbortSignal?.aborted) {
				await this.#throwProcessedError(this.#userProvidedAbortSignal.reason, failedRequest);
			}

			if (this.#getRemainingTotalTimeout() !== 0) {
				// A throwing progress callback errors the response stream, which a browser may report as a raw `TypeError` that would otherwise be mistaken for a dropped connection. Surface the callback error instead.
				const progressCallbackError = getProgressCallbackError(response.body ?? undefined);
				if (progressCallbackError !== undefined) {
					await this.#throwProcessedError(progressCallbackError, failedRequest);
				}

				// A connection dropped while streaming the body surfaces as a raw runtime `TypeError`. Wrap it like fetch-phase network errors so it is recognizable and runs `beforeError` hooks.
				// This only happens on the awaited path, so a body that fails after the timeout already won does not run the hooks again.
				if (isRawNetworkError(error)) {
					await this.#throwProcessedError(new NetworkError(failedRequest, {cause: error as Error}), failedRequest);
				}

				await this.#throwProcessedError(error, failedRequest);
			}

			result = timedOutResponseData;
		}

		// A cancellation is part of the request lifecycle, so it must not resolve as a successful body read just
		// because the bytes happened to arrive first. `.json()` already gets this check for free through
		// `#raceWithTotalTimeout()`, which is why the shortcuts used to disagree here.
		if (this.#userProvidedAbortSignal?.aborted) {
			await this.#throwProcessedError(this.#userProvidedAbortSignal.reason, failedRequest);
		}

		if (result === timedOutResponseData || this.#getRemainingTotalTimeout() === 0) {
			// The stream is locked by the native body method's own reader by this point, so
			// `response.body.cancel()` would reject as "already locked". Aborting the request's
			// signal is what actually interrupts the underlying network read.
			this.#abortController?.abort();
			await this.#throwProcessedError(new TimeoutError(failedRequest), failedRequest);
		}

		return result;
	}
```
- withProgress (source/utils/body.ts:53-99)
```typescript
const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> => {
	let previousChunk: Uint8Array | undefined;
	let transferredBytes = 0;
	let progressCallbackError: unknown;
	const report = (progress: Progress, chunk: Uint8Array) => {
		try {
			onProgress?.(progress, chunk);
		} catch (error) {
			// Remember the error so Ky can surface it instead of the network error the runtime reports for the failed stream.
			progressCallbackError = error;
			throw error;
		}
	};

	// The transformer only runs after construction, so its error accessor can be registered immediately afterwards.
	const progressStream: ReadableStream<Uint8Array> = stream.pipeThrough(new TransformStream<Uint8Array, Uint8Array>({
		transform(currentChunk, controller) {
			controller.enqueue(currentChunk);

			if (previousChunk) {
				transferredBytes += previousChunk.byteLength;

				let percent = totalBytes === 0 ? 0 : transferredBytes / totalBytes;
				// Avoid reporting 100% progress before the stream is actually finished (in case totalBytes is inaccurate)
				if (percent >= 1) {
					// Epsilon is used here to get as close as possible to 100% without reaching it.
					// If we were to use 0.99 here, percent could potentially go backwards.
					percent = 1 - Number.EPSILON;
				}

				// An unknown total stays `0`, matching the `percent` computed from the same estimate and the documented progress shape.
				const reportedTotalBytes = totalBytes === 0 ? 0 : Math.max(totalBytes, transferredBytes);
				report({percent, totalBytes: reportedTotalBytes, transferredBytes}, previousChunk);
			}

			previousChunk = currentChunk;
		},
		flush() {
			const finalChunk = previousChunk ?? new Uint8Array();
			transferredBytes += finalChunk.byteLength;
			report({percent: 1, totalBytes: transferredBytes, transferredBytes}, finalChunk);
		},
	}));
	progressCallbackErrors.set(progressStream, () => progressCallbackError);

	return progressStream;
};
```
- Ky (source/core/Ky.ts:252-1556)
```typescript
export class Ky {
	static create(input: Input, options: Options): ResponsePromise {
		const initHooks = options.hooks?.init ?? [];
		const initHookOptions = initHooks.length > 0 ? cloneInitHookOptions(options) : options;

		for (const hook of initHooks) {
			hook(initHookOptions as InitOptions);
		}

		const ky = new Ky(input, initHookOptions);

		const function_ = async (): Promise<Response | void> => {
			validateTimeoutOption(ky.#options.totalTimeout, 'totalTimeout');
			validateTimeoutOption(ky.#options.timeout, 'timeout');
			ky.#validateCallbackOptions();

			// Delay the fetch so that body method shortcuts can set the Accept header
			await Promise.resolve();
			const beforeRequestResponse = await ky.#runBeforeRequestHooks();
			if (beforeRequestResponse !== undefined) {
				ky.#retryLimit = normalizeRetryOptions(ky.#options.retry).limit;
			}

			let response = beforeRequestResponse ?? await ky.#retry();
			let responseFromHook = beforeRequestResponse !== undefined
				|| ky.#consumeReturnedResponseFromBeforeRetryHook();

			for (;;) {
				// `undefined` means a hook stopped the flow without providing a response.
				// Non-native Responses still continue through Ky if they pass `isResponseInstance()`.
				if (response === undefined) {
					return response;
				}

				if (isResponseInstance(response)) {
					try {
						// eslint-disable-next-line no-await-in-loop
						response = await ky.#runAfterResponseHooks(response);
					} catch (error) {
						if (!isForceRetryError(error)) {
							throw error;
						}

						// eslint-disable-next-line no-await-in-loop
						const retriedResponse: Response | void = await ky.#retryFromError(error);
						if (retriedResponse === undefined) {
							return retriedResponse;
						}

						response = retriedResponse;
						responseFromHook = ky.#consumeReturnedResponseFromBeforeRetryHook();
						continue;
					}
				}

				const currentResponse: Response = response;

				// Opaque responses (`response.type === 'opaque'`) from `no-cors` requests always have `status: 0` and `ok: false`, but this is not a failure - the actual status is hidden by the browser.
				if (!currentResponse.ok && currentResponse.type !== 'opaque' && (
					typeof ky.#options.throwHttpErrors === 'function'
						? ky.#options.throwHttpErrors(currentResponse.status)
						: ky.#options.throwHttpErrors
				)) {
					// `request` must reflect the request that actually failed, but `options` stays as Ky's
					// normalized options snapshot. Replacement `Request` instances do not preserve the
					// original `BodyInit`, so trying to make `options` mirror arbitrary requests would be lossy.
					const httpError: HTTPError = new HTTPError(currentResponse, ky.#getResponseRequest(currentResponse), ky.#getNormalizedOptions());
					// eslint-disable-next-line no-await-in-loop
					httpError.data = await ky.#getResponseData(currentResponse);
					ky.#throwIfAbortedByUser();
					ky.#throwIfTotalTimeoutExhausted();

					if (responseFromHook) {
						throw httpError;
					}

					// eslint-disable-next-line no-await-in-loop
					const retriedResponse: Response | void = await ky.#retryFromError(httpError);
					if (retriedResponse === undefined) {
						return retriedResponse;
					}

					response = retriedResponse;
					responseFromHook = ky.#consumeReturnedResponseFromBeforeRetryHook();
					continue;
				}

				break;
			}

			if (!isResponseInstance(response)) {
				return response;
			}

			ky.#decorateResponse(response);

			// If `onDownloadProgress` is passed, it uses the stream API internally
			if (ky.#options.onDownloadProgress) {
				if (typeof ky.#options.onDownloadProgress !== 'function') {
					throw new TypeError('The `onDownloadProgress` option must be a function');
				}

				if (!supportsResponseStreams) {
					throw new Error('Streams are not supported in your environment. `ReadableStream` is missing.');
				}

				const progressResponse = streamResponse(response, ky.#options.onDownloadProgress);
				ky.#setResponseRequest(progressResponse, ky.#getResponseRequest(response));
				return ky.#decorateResponse(progressResponse);
			}

			return response;
		};

		const result = (async () => {
			let response: Response | undefined;
			try {
				response = (await function_()) ?? undefined;
				return response;
			} catch (error: unknown) {
				return await ky.#throwProcessedError(error);
			} finally {
				const originalRequest = ky.#originalRequest;

				// Ignore cancellation errors from already-locked or already-consumed streams.
				// A custom fetch or hook can return the request body as its response body; ownership then belongs to the caller.
				if (originalRequest?.body !== response?.body) {
					ky.#cancelBody(originalRequest?.body ?? undefined);
				}

				// Only cancel the current request body if it's distinct from the original (i.e. it was cloned for
				// retries). `#originalRequest` is only set once a request is handed to fetch, so a pipeline that
				// never started, for example one that failed validation, leaves the caller's own body alone.
				if (originalRequest && ky.request !== originalRequest && ky.request.body !== response?.body) {
					ky.#cancelBody(ky.request.body ?? undefined);
				}
			}
		})() as ResponsePromise;

		for (const [type, mimeType] of Object.entries(responseTypes) as ObjectEntries<typeof responseTypes>) {
			// Only expose `.bytes()` when the environment implements it.
			if (
				type === 'bytes'
				&& typeof (globalThis.Response?.prototype as unknown as {bytes?: unknown})?.bytes !== 'function'
			) {
				continue;
			}

			result[type] = async (schema?: StandardSchemaV1) => {
				// `#fetch()` replaces `ky.request` with the clone it prepares for a possible retry, so a shortcut called
				// after the request was dispatched would otherwise advertise its media type on that clone, which is only
				// sent if a retry happens, instead of on the attempt that produced the response.
				const acceptRequest = ky.#sentRequest;
				// eslint-disable-next-line @typescript-eslint/prefer-nullish-coalescing
				acceptRequest.headers.set('accept', acceptRequest.headers.get('accept') || mimeType);

				const response = await result;

				// `ky.stop` from a `beforeRetry` hook resolves the request with no response at all. Reading a body
				// method would otherwise crash deep inside Ky with a TypeError that names Ky internals.
				if (response === undefined) {
					throw new TypeError(missingResponseMessage);
				}

				if (type !== 'json') {
					return ky.#raceBodyRead(async () => response[type](), response);
				}

				const text = await ky.#raceBodyRead(async () => response.text(), response) as string;
				const request = ky.#getResponseRequest(response);
				// This implementation is shared by every body method, so its return type has to satisfy all of them. The parsed value is whatever `parseJson` or `JSON.parse` produced, which is `any` at this boundary.
				let parsedResult: any;
				try {
					parsedResult = await ky.#raceWithTotalTimeout(async () => {
						const jsonValue = initHookOptions.parseJson
							? await initHookOptions.parseJson(text, {request, response})
							: (text === '' && schema !== undefined
								? undefined
								: JSON.parse(text));

						if (schema === undefined) {
							return jsonValue;
						}

						// eslint-disable-next-line no-return-await, @typescript-eslint/return-await -- Awaiting here preserves the caller's async stack when schema validation fails.
						return await validateJsonWithSchema(jsonValue, schema);
					}, ky.#userProvidedAbortSignal);
				} catch (error: unknown) {
					// A cancellation is part of the request lifecycle, so it reaches `beforeError` like every other abort. Other failures, such as invalid JSON, stay as they are.
					if (ky.#userProvidedAbortSignal?.aborted) {
						await ky.#throwProcessedError(ky.#userProvidedAbortSignal.reason, request);
					}

					throw error;
				}

				if (parsedResult === timedOutOperation) {
					await ky.#throwProcessedError(new TimeoutError(request), request);
				}

				return parsedResult;
			};
		}

		return result;
	}

	// eslint-disable-next-line unicorn/prevent-abbreviations
	static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption {
		// Filter out undefined values from plain objects
		if (searchParams && typeof searchParams === 'object' && !Array.isArray(searchParams) && !(searchParams instanceof URLSearchParams)) {
			return Object.fromEntries(Object.entries(searchParams).filter(([, value]) => value !== undefined));
		}

		return searchParams;
	}

	public request: Request;
	#abortController?: AbortController;
	#retryCount = 0;
	#retryLimit: number;
	readonly #options: InternalOptions;
	// Keep the input Request alive because Node.js stops forwarding its signal when the Request is garbage-collected.
	readonly #requestInput: Request | undefined;
	#originalRequest?: Request;
	#requestBodyCanBeCancelled = false;
	readonly #userProvidedAbortSignal: AbortSignal | undefined;
	readonly #beforeRetryHookErrors = new WeakSet<Error>();
	// Errors thrown by a user callback that runs while the request is in flight. Retrying would re-send the body and
	// run the callback again, so they are never retriable, whatever `shouldRetry` says.
	readonly #inFlightCallbackErrors = new WeakSet<Error>();
	#cachedNormalizedOptions: NormalizedOptions | undefined;
	readonly #startTime: number | undefined;
	#returnedResponseFromBeforeRetryHook = false;
	readonly #responseRequests = new WeakMap<Response, Request>();
	readonly #decoratedResponses = new WeakSet<Response>();

	// eslint-disable-next-line complexity
	constructor(input: Input, options: Options = {}) {
		// Checked before the options are built, since building them reads `input.headers`.
		if (typeof input !== 'string' && !(input instanceof URL || input instanceof globalThis.Request)) {
			throw new TypeError('`input` must be a string, URL, or Request');
		}

		const {maxResponseSize = Number.POSITIVE_INFINITY} = options;
		if (Object.hasOwn(options, 'prefixUrl')) {
			throw new Error(prefixUrlRenamedErrorMessage);
		}

		this.#options = {
			...options,
			headers: mergeHeaders((input as Request).headers, options.headers),
			hooks: mergeHooks({}, options.hooks),
			method: normalizeRequestMethod(options.method ?? (input as Request).method ?? 'GET'),
			referrer: options.referrer ?? (input as Request).referrer,
			referrerPolicy: options.referrerPolicy ?? (input as Request).referrerPolicy,
			// eslint-disable-next-line @typescript-eslint/prefer-nullish-coalescing
			prefix: String(options.prefix || ''),
			retry: normalizeRetryOptions(options.retry),
			throwHttpErrors: options.throwHttpErrors ?? true,
			timeout: options.timeout ?? 10_000,
			totalTimeout: options.totalTimeout ?? false,
			maxResponseSize,
			fetch: options.fetch ?? globalThis.fetch.bind(globalThis),
			// Deep-cloned so a hook mutating a nested plain object or array cannot write back to the instance defaults or the caller's object. Non-plain values such as class instances are kept by reference, matching how `json` and the shallow `context` merge treat them.
			context: cloneDeep(options.context) ?? {},
		};
		this.#retryLimit = this.#options.retry.limit;

		if (maxResponseSize !== Number.POSITIVE_INFINITY && (!Number.isSafeInteger(maxResponseSize) || maxResponseSize < 0)) {
			throw new TypeError('The `maxResponseSize` option must be a non-negative safe integer or Infinity');
		}

		this.#requestInput = input instanceof globalThis.Request ? input : undefined;

		if (typeof input === 'string') {
			if (this.#options.prefix) {
				const normalizedPrefix = this.#options.prefix.replace(/\/+$/, '');
				const normalizedInput = input.replace(/^\/+/, '');
				input = `${normalizedPrefix}/${normalizedInput}`;
			}

			if (this.#options.baseUrl) {
				const normalizedInput = normalizeInputForProtocolCheck(input);

				if (malformedHttpProtocolPattern.test(normalizedInput)) {
					throw new TypeError('`input` url protocol must be followed by `//` when using `baseUrl`');
				}

				if (!isAbsoluteInput(normalizedInput)) {
					input = new URL(input, (new Request(this.#options.baseUrl)).url);
				}
			}
		}

		if (supportsAbortController && supportsAbortSignal) {
			this.#userProvidedAbortSignal = this.#options.signal ?? this.#requestInput?.signal;
			this.#abortController = new globalThis.AbortController();
			this.#options.signal = this.#createManagedSignal();
		}

		if (supportsRequestStreams) {
			// @ts-expect-error - Types are outdated.
			this.#options.duplex = 'half';
		}

		// A `Request` input's headers are already merged in, so only a content-type from `options.headers` counts as user-provided.
		const userProvidedContentType = options.headers !== undefined && mergeHeaders({}, options.headers).has('content-type');

		if (this.#options.json !== undefined) {
			this.#options.body = this.#options.stringifyJson?.(this.#options.json) ?? JSON.stringify(this.#options.json);
			if (!userProvidedContentType) {
				this.#options.headers.set('content-type', 'application/json');
			}
		}

		// To provide correct form boundary, Content-Type header should be deleted when creating Request from another Request with FormData/URLSearchParams body
		// Only delete if user didn't explicitly provide a custom content-type
		if (
			input instanceof globalThis.Request
			&& ((supportsFormData && this.#options.body instanceof globalThis.FormData) || this.#options.body instanceof URLSearchParams)
			&& !userProvidedContentType
		) {
			this.#options.headers.delete('content-type');
		}

		this.request = new globalThis.Request(input, this.#options as RequestInit);
		this.#requestBodyCanBeCancelled = typeof globalThis.ReadableStream === 'function' && this.#options.body instanceof globalThis.ReadableStream;

		if (hasSearchParameters(this.#options.searchParams)) {
			const url = new URL(this.request.url);
			const deleted = (this.#options.searchParams as any)?.[deletedParametersSymbol] as Set<string> | undefined;

			if (deleted) {
				// Remove keys from the input URL first so later searchParams entries can intentionally re-add them.
				for (const key of deleted) {
					url.searchParams.delete(key);
				}
			}

			if (typeof this.#options.searchParams === 'string') {
				const stringSearchParameters = this.#options.searchParams.replace(/^\?/, '');
				if (stringSearchParameters !== '') {
					url.search = url.search ? `${url.search}&${stringSearchParameters}` : `?${stringSearchParameters}`;
				}
			} else {
				const optionsSearchParameters = new URLSearchParams(Ky.#normalizeSearchParams(this.#options.searchParams) as unknown as SearchParamsInit);

				for (const [key, value] of optionsSearchParameters.entries()) {
					url.searchParams.append(key, value);
				}
			}

			if (
				this.#options.searchParams
				&& typeof this.#options.searchParams === 'object'
				&& !Array.isArray(this.#options.searchParams)
				&& !(this.#options.searchParams instanceof URLSearchParams)
			) {
				for (const [key, value] of Object.entries(this.#options.searchParams)) {
					if (value === undefined) {
						url.searchParams.delete(key);
					}
				}
			}

			// Recreate request with the updated URL. We already have all options in this.#options, including duplex.
			// Rebuilding is also what drops an inherited `Request` body, so it is skipped when the search parameters leave the URL unchanged.
			if (url.href !== this.request.url) {
				// Request options are read back from the current request so values inherited from a `Request` input (like `credentials`) survive, while explicit options already won when that request was built.
				// A `Request` input's body is only available as a stream, so keeping it requires request stream support.
				const {
					body: inheritedBody,
					cache,
					credentials,
					integrity,
					keepalive,
					mode,
					redirect,
					referrer,
					referrerPolicy,
				} = this.request;
				const canUseInheritedBody = supportsRequestStreams && !keepalive && mode !== 'no-cors';

				this.request = new globalThis.Request(url, {
					...this.#options,
					cache,
					credentials,
					integrity,
					keepalive,
					mode,
					redirect,
					referrer,
					referrerPolicy,
					body: this.#options.body ?? (canUseInheritedBody ? inheritedBody : undefined),
				} as RequestInit);
			}
		}

		if (this.#options.onUploadProgress && typeof this.#options.onUploadProgress !== 'function') {
			throw new TypeError('The `onUploadProgress` option must be a function');
		}

		// `totalTimeout` starts when the request pipeline is created, so it also includes
		// Ky's internal scheduling and user hook time before the first fetch attempt.
		this.#startTime = typeof this.#options.totalTimeout === 'number' ? this.#getCurrentTime() : undefined;
	}

	// These callbacks are called directly, so a non-function value otherwise surfaces as a runtime error naming Ky's
	// own internals rather than the option the caller set.
	#validateCallbackOptions(): void {
		for (const key of ['parseJson', 'stringifyJson', 'fetch'] as const) {
			if (this.#options[key] !== undefined && typeof this.#options[key] !== 'function') {
				throw new TypeError(`The \`${key}\` option must be a function`);
			}
		}

		if (
			typeof this.#options.throwHttpErrors !== 'boolean'
			&& typeof this.#options.throwHttpErrors !== 'function'
		) {
			throw new TypeError('The `throwHttpErrors` option must be a boolean or a function');
		}
	}

	#calculateDelay(retry: InternalOptions['retry']): number {
		const retryDelay = retry.delay(this.#retryCount + 1);

		// A `retry.delay` that does not return a usable number would turn every delay into `NaN`, which
		// `setTimeout()` clamps to 1ms, silently disabling the configured backoff. The `jitter` function form
		// already guards its own result, so the input is checked here for every jitter form.
		if (typeof retryDelay !== 'number' || Number.isNaN(retryDelay) || retryDelay < 0) {
			throw new TypeError('`retry.delay` must return a non-negative number or `Infinity`');
		}

		let jitteredDelay = retryDelay;
		if (retry.jitter === true) {
			jitteredDelay = Math.random() * retryDelay;
		} else if (typeof retry.jitter === 'function') {
			jitteredDelay = retry.jitter(retryDelay);

			if (!Number.isFinite(jitteredDelay) || jitteredDelay < 0) {
				jitteredDelay = retryDelay;
			}
		}

		return Math.min(retry.backoffLimit, jitteredDelay);
	}

	async #calculateRetryDelay(error: unknown) {
		if (isError(error) && this.#inFlightCallbackErrors.has(error)) {
			throw error;
		}

		const retry = normalizeRetryOptions(this.#options.retry);
		if (this.#retryCount >= Math.min(retry.limit, this.#retryLimit)) {
			throw error;
		}

		// Wrap non-Error throws to ensure consistent error handling
		const errorObject = error instanceof Error ? error : new NonError(error);

		// Handle forced retry from afterResponse hook - skip method check and shouldRetry
		if (isForceRetryError(errorObject)) {
			return errorObject.customDelay ?? this.#calculateDelay(retry);
		}

		// Check if method is retriable for non-forced retries
		if (!retry.methods.includes(normalizeRetryMethod(this.request.method))) {
			throw error;
		}

		let shouldRetryOverride = false;
		const {shouldRetry} = retry;
		if (shouldRetry !== undefined) {
			const result = await this.#raceWithTotalTimeout(async () => shouldRetry({error: errorObject, retryCount: this.#retryCount + 1}), this.#userProvidedAbortSignal);
			this.#throwIfAbortedByUser();
			if (result === timedOutOperation) {
				throw new TimeoutError(this.#sentRequest);
			}

			// Only exact booleans override the default retry checks.
			if (result === false) {
				throw error;
			}

			shouldRetryOverride = result === true;
		}

		// Default timeout behavior
		if (isTimeoutError(error)) {
			if (!shouldRetryOverride && !retry.retryOnTimeout) {
				throw error;
			}

			return this.#calculateDelay(retry);
		}

		// The `isHTTPError()` brand only checks `name`, so a cross-realm error may not carry a `response`.
		if (isHTTPError(error) && error.response) {
			if (!shouldRetryOverride && !retry.statusCodes.includes(error.response.status)) {
				throw error;
			}

			const retryTimingHeader = getRetryTimingHeader(error.response.headers);
			if (retryTimingHeader && retry.afterStatusCodes.includes(error.response.status)) {
				const after = calculateRetryTimingDelay(retryTimingHeader);
