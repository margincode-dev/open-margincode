- TimeoutError (source/errors/TimeoutError.ts:7-15)
```typescript
export class TimeoutError extends KyError {
	override name = 'TimeoutError';
	request: KyRequest;

	constructor(request: Request) {
		super(`Request timed out: ${request.method} ${request.url}`);
		this.request = request;
	}
}
```
- Ky.#getResponseData (source/core/Ky.ts:837-874)
```typescript
	async #getResponseData(response: Response): Promise<unknown> {
		// `request` is the request that actually produced this response, which is not `this.request` once a retry clone has been prepared.
		const request = this.#getResponseRequest(response);

		// Even with request timeouts disabled, bound error-body reads so retries and error propagation
		// cannot be stalled indefinitely by never-ending response streams.
		const readTimeout = this.#getErrorDataTimeout(request);
		const text = await this.#readResponseText(response, readTimeout.milliseconds);
		if (text === timedOutResponseData) {
			if (readTimeout.fromTotalTimeout) {
				throw new TimeoutError(request);
			}

			this.#throwIfTotalTimeoutExhausted(request);
			return undefined;
		}

		if (!text) {
			return undefined;
		}

		if (!this.#isJsonContentType(response.headers.get('content-type') ?? '')) {
			return text;
		}

		const parseTimeout = this.#getErrorDataTimeout(request);
		const data = await this.#parseJson(text, response, parseTimeout.milliseconds, request);
		if (data === timedOutResponseData) {
			if (parseTimeout.fromTotalTimeout) {
				throw new TimeoutError(request);
			}

			this.#throwIfTotalTimeoutExhausted(request);
			return undefined;
		}

		return data;
	}
```
- Ky.#getErrorDataTimeout (source/core/Ky.ts:876-894)
```typescript
	#getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
		const errorDataTimeout = this.#options.timeout === false ? 10_000 : this.#options.timeout;
		const remainingTotal = this.#getRemainingTotalTimeout();
		if (remainingTotal === undefined) {
			return {
				milliseconds: errorDataTimeout,
				fromTotalTimeout: false,
			};
		}

		if (remainingTotal <= 0) {
			throw new TimeoutError(request);
		}

		return {
			milliseconds: Math.min(errorDataTimeout, remainingTotal),
			fromTotalTimeout: remainingTotal <= errorDataTimeout,
		};
	}
```
- Ky.#throwIfTotalTimeoutExhausted (source/core/Ky.ts:1197-1202)
```typescript
	#throwIfTotalTimeoutExhausted(request: Request = this.request): void {
		const remaining = this.#getRemainingTotalTimeout();
		if (remaining !== undefined && remaining <= 0) {
			throw new TimeoutError(request);
		}
	}
```
- isTimeoutError (source/utils/type-guards.ts:102-104)
```typescript
export function isTimeoutError(error: unknown): error is TimeoutError {
	return isErrorType(error, TimeoutError);
}
```
- Ky.#retryFromError (source/core/Ky.ts:1303-1390)
```typescript
	async #retryFromError(error: unknown): Promise<Response | void> {
		this.#throwIfAbortedByUser();
		this.#returnedResponseFromBeforeRetryHook = false;

		const retryDelay = Math.min(await this.#calculateRetryDelay(error), maxSafeTimeout);
		const delayOptions = {signal: this.#userProvidedAbortSignal};

		const remainingTimeout = this.#getRemainingTotalTimeout();
		if (remainingTimeout !== undefined) {
			if (remainingTimeout <= 0) {
				throw new TimeoutError(this.request);
			}

			// If waiting would consume all remaining budget, time out without starting another request.
			if (retryDelay >= remainingTimeout) {
				await delay(remainingTimeout, delayOptions);
				throw new TimeoutError(this.request);
			}
		}

		// Only use user-provided signal for delay, not our internal abortController
		await delay(retryDelay, delayOptions);

		this.#throwIfTotalTimeoutExhausted();

		// Reset abortController if it was aborted (happens on timeout retry) so hooks and the retried request get a fresh signal
		if (this.#abortController?.signal.aborted) {
			this.#abortController = new globalThis.AbortController();
			this.#options.signal = this.#createManagedSignal();
			this.#assignRequest(this.#withManagedSignal(this.request), this.#requestBodyCanBeCancelled);
		}

		// Apply custom request from forced retry before beforeRetry hooks
		// Ensure the custom request has the correct managed signal for timeouts and user aborts
		if (isForceRetryError(error) && error.customRequest) {
			// Replacement Requests are authoritative by design. Do not rewrite headers here,
			// even for cross-origin retries. Callers using `ky.retry({request})` explicitly
			// opted into the exact Request they constructed.
			this.#assignRequest(this.#withManagedSignal(error.customRequest));
		}

		for (const hook of this.#options.hooks.beforeRetry) {
			let hookResult: Awaited<ReturnType<typeof hook>> | typeof timedOutOperation;
			try {
				// eslint-disable-next-line no-await-in-loop
				hookResult = await this.#raceWithTotalTimeout(async () => hook({
					request: this.request,
					options: this.#getNormalizedOptions(),
					error: error instanceof Error ? error : new NonError(error),
					retryCount: this.#retryCount + 1,
				}), this.#userProvidedAbortSignal);
			} catch (hookError) {
				// Preserve the original request error path (`throw error`) so beforeError hooks can still run.
				if (hookError instanceof Error && hookError !== error) {
					this.#beforeRetryHookErrors.add(hookError);
				}

				throw hookError;
			}

			if (hookResult === timedOutOperation) {
				throw new TimeoutError(this.request);
			}

			if (isRequestInstance(hookResult)) {
				// Same contract as `ky.retry({request})`: a Request returned from `beforeRetry`
				// is used as-is rather than being sanitized or otherwise rewritten by Ky.
				this.#assignRequest(this.#withManagedSignal(hookResult));
				break;
			}

			if (isResponseInstance(hookResult)) {
				this.#returnedResponseFromBeforeRetryHook = true;
				this.#retryCount++;
				return hookResult;
			}

			// If `stop` is returned from the hook, the retry process is stopped
			if (hookResult === stop) {
				return;
			}
		}

		this.#throwIfTotalTimeoutExhausted();

		this.#retryCount++;
		return this.#retry();
	}
```
- validateTimeoutOption (source/core/Ky.ts:87-99)
```typescript
const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
	if (value === false) {
		return;
	}

	if (typeof value !== 'number' || !Number.isFinite(value) || value < 0) {
		throw new TypeError(`The \`${name}\` option must be a non-negative number or \`false\``);
	}

	if (value > maxSafeTimeout) {
		throw new RangeError(`The \`${name}\` option cannot be greater than ${maxSafeTimeout}`);
	}
};
```
- Ky (source/core/Ky.ts:252-1550)
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
				const acceptRequest = ky.#originalRequest ?? ky.request;
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
