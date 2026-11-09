- Ky.#getResponseData (source/core/Ky.ts:833-870)
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
- RetryOptions (source/types/retry.ts:21-187)
```typescript
export type RetryOptions = {
	/**
	The number of times to retry failed requests.
	Must be a finite, non-negative integer.

	@default 2
	*/
	limit?: number | undefined;

	/**
	The HTTP methods allowed to retry.

	@default ['get', 'put', 'head', 'delete', 'options', 'trace', 'query']
	*/
	methods?: readonly HttpMethod[] | undefined;

	/**
	The HTTP status codes allowed to retry.

	`413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`.

	@default [408, 413, 429, 500, 502, 503, 504]
	*/
	statusCodes?: readonly number[] | undefined;

	/**
	The retriable HTTP status codes that should respect retry timing headers. These status codes must also be included in `statusCodes`, unless `shouldRetry` returns `true`.

	@default [413, 429, 503]
	*/
	afterStatusCodes?: readonly number[] | undefined;

	/**
	If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`.

	@default Infinity
	*/
	maxRetryAfter?: number | undefined;

	/**
	The upper limit of the delay per retry in milliseconds, for the delay calculated by `retry.delay`.
	To clamp the delay, set `backoffLimit` to 1000, for example.
	A delay from a retry timing header is not subject to this limit; use `maxRetryAfter` to bound it.

	By default, the delay is calculated in the following way:

	```
	0.3 * (2 ** (attemptCount - 1)) * 1000
	```

	The delay increases exponentially.

	@default Infinity
	*/
	backoffLimit?: number | undefined;

	/**
	A function to calculate the delay in milliseconds between retries given `attemptCount` (starts from 1).

	@default attemptCount => 0.3 * (2 ** (attemptCount - 1)) * 1000
	*/
	delay?: ((attemptCount: number) => number) | undefined;

	/**
	Add random jitter to retry delays to prevent thundering herd problems.

