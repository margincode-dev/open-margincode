- hasSearchParameters (source/utils/options.ts:31-60)
```typescript
export const hasSearchParameters = (search: SearchParamsOption): boolean => {
	if (search === undefined) {
		return false;
	}

	// Option merging turns `null` into an absent value, but an `init` hook assigns straight onto the options object. `null` is not accepted, so report it instead of treating it as absent.
	if (search === null) {
		throw new TypeError('The `searchParams` option must not be `null`. Use `undefined` to clear it.');
	}

	// The `typeof array` still gives "object", so we need different checking for array.
	if (Array.isArray(search)) {
		return search.length > 0;
	}

	if (search instanceof URLSearchParams) {
		return search.size > 0 || Boolean((search as any)[deletedParametersSymbol]?.size);
	}

	// Record
	if (typeof search === 'object') {
		return Object.keys(search).length > 0;
	}

	if (typeof search === 'string') {
		return search.trim().length > 0;
	}

	return Boolean(search);
};
```
- mergeHooks (source/utils/merge.ts:217-231)
```typescript
export const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks => {
	// `null` selects the default, the same way option merging treats a nullish value as absent.
	const incomingHooks = incoming ?? {};
	if (!isObject(incomingHooks) || Array.isArray(incomingHooks)) {
		throw new TypeError('The `hooks` option must be an object');
	}

	return {
		init: newHookValue(original, incomingHooks, 'init'),
		beforeRequest: newHookValue(original, incomingHooks, 'beforeRequest'),
		beforeRetry: newHookValue(original, incomingHooks, 'beforeRetry'),
		beforeError: newHookValue(original, incomingHooks, 'beforeError'),
		afterResponse: newHookValue(original, incomingHooks, 'afterResponse'),
	};
};
```
- promiseWithTimeout (test/helpers/with-page.ts:19-37)
```typescript
export const promiseWithTimeout = async <Value>(
	promise: Promise<Value>,
	timeoutInMilliseconds: number,
	errorMessage: string,
): Promise<Value> => new Promise((resolve, reject) => {
	const timeoutIdentifier = setTimeout(() => {
		reject(new Error(errorMessage));
	}, timeoutInMilliseconds);

	promise
		.then(value => {
			clearTimeout(timeoutIdentifier);
			resolve(value);
		})
		.catch((error: unknown) => {
			clearTimeout(timeoutIdentifier);
			reject(error instanceof Error ? error : new Error(String(error)));
		});
});
```
- createAbortDuringFetch (test/retry.ts:4040-4056)
```typescript
const createAbortDuringFetch = (onRequest?: () => void): typeof fetch => async input => {
	onRequest?.();
	const {signal} = input as Request;

	return new Promise<Response>((_resolve, reject) => {
		const fail = () => {
			reject(new TypeError('Failed to fetch'));
		};

		if (signal.aborted) {
			fail();
			return;
		}

		signal.addEventListener('abort', fail, {once: true});
	});
};
```
- delay (source/utils/delay.ts:9-29)
```typescript
export default async function delay(
	ms: number,
	{signal}: DelayOptions,
): Promise<void> {
	return new Promise((resolve, reject) => {
		if (signal) {
			signal.throwIfAborted();
			signal.addEventListener('abort', abortHandler, {once: true});
		}

		function abortHandler() {
			clearTimeout(timeoutId);
			reject(signal!.reason as Error);
		}

		const timeoutId = setTimeout(() => {
			signal?.removeEventListener('abort', abortHandler);
			resolve();
		}, ms);
	});
}
```
- timeout (source/utils/timeout.ts:10-47)
```typescript
export default async function timeout(
	request: Request,
	init: RequestInit,
	abortController: AbortController | undefined,
	options: TimeoutOptions,
): Promise<Response> {
	let timeoutId: ReturnType<typeof setTimeout> | undefined;
	let timedOut = false;

	try {
		return await new Promise((resolve, reject) => {
			timeoutId = setTimeout(() => {
				timedOut = true;

				if (abortController) {
					abortController.abort();
				}

				reject(new TimeoutError(request));
			}, options.timeout);

			// Called unbound so a native `window.fetch` is not invoked with `options` as `this`, which throws "Illegal invocation" in browsers.
			// A synchronous throw rejects the promise, and `finally` still clears the timer so it cannot abort a later retry attempt.
			const {fetch} = options;
			fetch(request, init).then(response => {
				// A response arriving after the timeout already won is discarded, so its unused body is cancelled. Fire-and-forget: cancellation failures are ignored and never delay the rejection.
				if (timedOut) {
					void response.body?.cancel().catch(() => undefined);
					return;
				}

				resolve(response);
			}).catch(reject);
		});
	} finally {
		clearTimeout(timeoutId);
	}
}
```
- Ky.create (source/core/Ky.ts:268-464)
```typescript
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
				// A nullish value means a hook stopped the flow, or a custom `fetch` resolved with nothing. The caller
				// sees it normalized to `undefined` either way.
				// Non-native Responses still continue through Ky if they pass `isResponseInstance()`.
				if (response === undefined || response === null) {
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
				if (!currentResponse.ok && currentResponse.type !== 'opaque' && ky.#shouldThrowHttpErrors(currentResponse)) {
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
```
- Ky (source/core/Ky.ts:267-1585)
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
				// A nullish value means a hook stopped the flow, or a custom `fetch` resolved with nothing. The caller
				// sees it normalized to `undefined` either way.
				// Non-native Responses still continue through Ky if they pass `isResponseInstance()`.
				if (response === undefined || response === null) {
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
				if (!currentResponse.ok && currentResponse.type !== 'opaque' && ky.#shouldThrowHttpErrors(currentResponse)) {
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
	// Values thrown by a user callback that runs while the request is in flight. Retrying would re-send the body and
	// run the callback again, so they are never retriable, whatever `shouldRetry` says. A strong `Set` is safe because
	// a `Ky` instance lives for exactly one request, and the callback can throw a non-`Error` value.
	readonly #inFlightCallbackErrors = new Set<unknown>();
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
		// Checked here rather than on the way out, because the constructor already calls `stringifyJson`.
		validateCallbackOptions(this.#options);
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

	#calculateDelay(retry: InternalOptions['retry'], error: unknown): number {
		const retryDelay = retry.delay(this.#retryCount + 1);

		// A `retry.delay` that does not return a usable number would turn every delay into `NaN`, which
		// `setTimeout()` clamps to 1ms, silently disabling the configured backoff. The `jitter` function form
		// already guards its own result, so the input is checked here for every jitter form.
		if (typeof retryDelay !== 'number' || Number.isNaN(retryDelay) || retryDelay < 0) {
			// The failure that was being retried is chained as the cause, so `beforeError` can still inspect it
			// instead of losing the response and data to this configuration mistake.
			throw new TypeError('`retry.delay` must return a non-negative number or `Infinity`', {cause: error});
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
		if (this.#inFlightCallbackErrors.has(error)) {
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
			return errorObject.customDelay ?? this.#calculateDelay(retry, error);
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

			return this.#calculateDelay(retry, error);
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
					return this.#calculateDelay(retry, error);
				}

				// Don't apply jitter when server provides explicit retry timing
				return Math.min(retry.maxRetryAfter, after);
			}

			if (!shouldRetryOverride && error.response.status === 413) {
				throw error;
			}

			return this.#calculateDelay(retry, error);
		}

		// Only retry known retriable error types. Unknown errors (e.g., programming bugs) are not retried.
		if (!shouldRetryOverride && !isNetworkError(error)) {
			throw error;
		}

		return this.#calculateDelay(retry, error);
	}

	#decorateResponse(response: Response): Response {
		if (!this.#options.parseJson || this.#decoratedResponses.has(response)) {
			return response;
		}

		this.#decoratedResponses.add(response);
		const request = this.#getResponseRequest(response);

		try {
			response.json = async () => {
				const text = await response.text();
				return this.#options.parseJson!(text, {request, response});
			};

