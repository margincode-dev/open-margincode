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
