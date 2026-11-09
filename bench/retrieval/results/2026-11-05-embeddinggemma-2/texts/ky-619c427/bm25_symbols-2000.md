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
