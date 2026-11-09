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

		const user = await ky('/api/user').json(userSchema);
		```
		*/
		<Schema extends StandardSchemaV1>(schema: Schema): Promise<StandardSchemaV1InferOutput<Schema>>;
	};

	text: () => Promise<string>;
} & Promise<KyResponse<T>>;
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
- ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
```typescript
type ExpectedNormalizedRetryOptions = {
	limit: number;
	methods: string[];
	statusCodes: number[];
	afterStatusCodes: number[];
	maxRetryAfter: number;
	backoffLimit: number;
	delay: (attemptCount: number) => number;
	jitter: boolean | ((delay: number) => number) | undefined;
	retryOnTimeout: boolean;
	shouldRetry?: RetryOptions['shouldRetry'];
};
```
- KyOptions (source/types/options.ts:45-432)
```typescript
export type KyOptions = {
	/**
	Shortcut for sending JSON. Use this instead of the `body` option.

	Accepts any plain object or value, which will be stringified using `JSON.stringify()` and sent in the body with the correct header set.

	The `Content-Type` header is set to `application/json` unless you set a `Content-Type` in the `headers` option, which always takes precedence. A `Content-Type` on a `Request` input is replaced, because the `Request` constructor sets one automatically from its body.
	*/
	json?: unknown;

	/**
	User-defined JSON-parsing function.

	The function receives the response text as the first argument and a context object as the second argument containing the `request` and `response`.

	Use-cases:
	1. Parse JSON via the [`bourne` package](https://github.com/hapijs/bourne) to protect from prototype pollution.
	2. Parse JSON with [`reviver` option of `JSON.parse()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse).
	3. Log or handle JSON parse errors with request context.

	@default JSON.parse()

	@example
	```
	import ky from 'ky';
	import bourne from '@hapijs/bourne';

	const json = await ky('https://example.com', {
		parseJson: text => bourne(text)
	}).json();
	```

	@example
	```
	import ky from 'ky';

	const json = await ky('https://example.com', {
		parseJson: (text, {request, response}) => {
			console.log(`Parsing JSON from ${request.url} (status: ${response.status})`);
			return JSON.parse(text);
		}
	}).json();
	```
	*/
	// `options` is intentionally not included in the context to avoid exposing Ky internals through a parsing callback. `request`/`response` already provide the metadata needed for logging.
	parseJson?: ((text: string, context: {request: Request; response: Response}) => unknown) | undefined;

	/**
	User-defined JSON-stringifying function.

	Use-cases:
	1. Stringify JSON with a custom `replacer` function.

	@default JSON.stringify()

	@example
	```
	import ky from 'ky';
	import {DateTime} from 'luxon';

	const json = await ky('https://example.com', {
		stringifyJson: data => JSON.stringify(data, (key, value) => {
			if (key.endsWith('_at')) {
				return DateTime.fromISO(value).toSeconds();
			}

			return value;
		})
	}).json();
	```
	*/
	stringifyJson?: ((data: unknown) => string) | undefined;

	/**
	Search parameters to include in the request URL. Setting this will merge with any existing search parameters in the input URL.

	Accepts any value supported by [`URLSearchParams()`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams).

	When passing an object, setting a value to `undefined` deletes the parameter, including from the input URL, even when a later option layer adds the parameter again. `null` values are preserved and converted to the string `'null'`.

	When `input` is a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) with a body, the body is sent as a stream, which requires [request stream support](https://caniuse.com/wf-fetch-request-streams) and, in Chromium-based browsers, an HTTP/2 or HTTP/3 connection (streaming uploads over HTTP/1.1 fail with a network error, even over plain HTTP). The inherited body is dropped when the search parameters change the URL and the request has to be rebuilt in an environment that cannot reuse it, which is the case without request stream support, when `keepalive` is true, or when the effective mode is `'no-cors'`. A search parameter value that leaves the URL unchanged does not rebuild the request, so the body is kept. A compatible body passed explicitly with the `body` option is still used.
	*/
	searchParams?: SearchParamsOption;

	/**
	A base URL to [resolve](https://developer.mozilla.org/en-US/docs/Web/API/URL_API/Resolving_relative_references) the `input` against. When the `input` (after applying the `prefix` option) is only a relative URL, such as `'users'`, `'/users'`, or `'//my-site.com'`, it will be resolved against the `baseUrl` to determine the destination of the request. Otherwise, the `input` is absolute, such as `'https://my-site.com'`, and it will bypass the `baseUrl`.

	Useful when used with [`ky.extend()`](#kyextenddefaultoptions) to create niche-specific Ky instances.

	If the `baseUrl` itself is relative, it will be resolved against the environment's base URL, such as [`document.baseURI`](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI) in browsers or `location.href` in Deno (see the `--location` flag).

	**Tip:** When setting a `baseUrl` that has a path, we recommend that it include a trailing slash `/`, as in `'/api/'` rather than `/api`. This ensures more intuitive behavior for page-relative `input` URLs, such as `'users'` or `'./users'`, where they will _extend_ from the full path of the `baseUrl` rather than _replacing_ its last path segment.

	@example
	```
	import ky from 'ky';

	// On https://example.com

	const response = await ky('users', {baseUrl: '/api/'});
	//=> 'https://example.com/api/users'

	const response = await ky('/users', {baseUrl: '/api/'});
	//=> 'https://example.com/users'
	```
	*/
	baseUrl?: URL | string | undefined;

	/**
	A prefix to prepend to the `input` before making the request (and before it is resolved against the `baseUrl`). It can be any valid path or URL, either relative or absolute. A trailing slash `/` is optional and will be added automatically, if needed, when it is joined with `input`. Only takes effect when `input` is a string.

	Useful when used with [`ky.extend()`](#kyextenddefaultoptions) to create niche-specific Ky instances.

	*In most cases, you should use the `baseUrl` option instead, as it is more consistent with web standards. However, `prefix` is useful if you want origin-relative `input` URLs, such as `/users`, to be treated as if they were page-relative. In other words, the leading slash of the `input` will essentially be ignored, because the `prefix` will become part of the `input` before URL resolution happens.*

	Notes:
	- The `prefix` and `input` are joined with a slash `/`, and slashes are normalized at the join boundary by trimming trailing slashes from `prefix` and leading slashes from `input`.
	- After `prefix` and `input` are joined, the result is resolved against the `baseUrl` option, if present.

	@example
	```
	import ky from 'ky';

