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

	When many clients retry simultaneously (e.g., after hitting a rate limit), they can overwhelm the server again. Jitter adds randomness to break this synchronization.

	Set to `true` to use full jitter, which randomizes the delay between 0 and the computed delay.

	Alternatively, pass a function to implement custom jitter strategies.

	Note: Jitter is not applied when the server provides a retry timing header, as the server's explicit timing should be respected.

	@default undefined (no jitter)

	@example
	```
	import ky from 'ky';

	const json = await ky('https://example.com', {
		retry: {
			limit: 5,

			// Full jitter (randomizes delay between 0 and computed value)
			jitter: true

			// Percentage jitter (80-120% of delay)
			// jitter: delay => delay * (0.8 + Math.random() * 0.4)

			// Absolute jitter (±100ms)
			// jitter: delay => delay + (Math.random() * 200 - 100)
		}
	}).json();
	```
	*/
	jitter?: boolean | ((delay: number) => number) | undefined;

	/**
	Whether to retry when the request times out before a response is returned. Timeouts while reading a response body through Ky shortcut methods are not retried because the response has already been received.

	@default false

	@example
	```
	import ky from 'ky';

	const json = await ky('https://example.com', {
		retry: {
			limit: 3,
			retryOnTimeout: true
		}
	}).json();
	```
	*/
	retryOnTimeout?: boolean | undefined;

	/**
	A function to determine whether a retry should be attempted.

	This function takes precedence over the default retry checks (`retryOnTimeout`, status code checks, etc.) for retriable methods. It is only called after the retry limit and method checks pass.

	**Note:** This is different from the `beforeRetry` hook:
	- `shouldRetry`: Controls WHETHER to retry (called before the retry decision is made)
	- `beforeRetry`: Called AFTER retry is confirmed, allowing you to modify the request

	Should return:
	- `true` to force a retry (bypasses `retryOnTimeout`, status code checks, and other validations)
	- `false` to prevent a retry (no retry will occur)
	- `undefined` (or nothing) to use the default retry logic (`retryOnTimeout`, status codes, network errors). Unrecognized error types are not retried.

	Returning `true` overrides retry eligibility, not timing. For HTTP errors with a status in `afterStatusCodes`, retry timing headers and `maxRetryAfter` still apply, without jitter. Otherwise, Ky uses the configured backoff delay.

	@default undefined

	@example
	```
	import ky, {HTTPError} from 'ky';

	const json = await ky('https://example.com', {
		retry: {
			limit: 3,
			shouldRetry: ({error, retryCount}) => {
				// Retry on specific business logic errors from API
				if (error instanceof HTTPError) {
					const status = error.response.status;

					// Retry on 429 (rate limit) but only for first 2 attempts
					if (status === 429 && retryCount <= 2) {
						return true;
					}

					// Don't retry on 4xx errors except rate limits
					if (status >= 400 && status < 500) {
						return false;
					}
				}

				// Use default retry logic for other errors
				return undefined;
			}
		}
	}).json();
	```
	*/
	shouldRetry?: ((state: ShouldRetryState) => boolean | void | Promise<boolean | void>) | undefined;
};
```
- ResponsePromise (source/types/ResponsePromise.ts:7-69)
```typescript
export type ResponsePromise<T = unknown> = {
	arrayBuffer: () => Promise<ArrayBuffer>;

	blob: () => Promise<Blob>;

	formData: () => Promise<FormData>;

	/**
	Get the response body as raw bytes.

	Note: This shortcut is only available when the runtime supports `Response.prototype.bytes()`.
	*/
	bytes: () => Promise<Uint8Array<ArrayBuffer>>;

	// TODO: Use `json<T extends JSONValue>(): Promise<T>;` when it's fixed in TS.
	// See https://github.com/microsoft/TypeScript/issues/15300 and https://github.com/sindresorhus/ky/pull/80
	json: {
		/**
		Get the response body as JSON.

		@example
		```
		import ky from 'ky';

		const json = await ky(…).json();
		```

		@example
		```
		import ky from 'ky';

		interface Result {
			value: number;
		}

		const result1 = await ky(…).json<Result>();
		// or
		const result2 = await ky<Result>(…).json();
		```
		*/
		<JsonType = T>(schema?: undefined): Promise<JsonType>;

		/**
		Get the response body as JSON and validate it with a Standard Schema.
		Use a Standard Schema compatible validator (for example, Zod 3.24+).

		Throws a `SchemaValidationError` when validation fails.

		@example
		```
		import ky from 'ky';
		import {z} from 'zod';

		const userSchema = z.object({name: z.string()});

