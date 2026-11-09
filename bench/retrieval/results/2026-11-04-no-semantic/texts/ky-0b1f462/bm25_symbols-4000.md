- streamResponse (source/utils/body.ts:198-211)
```typescript
export const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) => {
	// See `limitResponseSize`: there is nothing left to stream and nothing to report once a hook consumed the body.
	if (!response.body || response.bodyUsed || response.body.locked || nullBodyStatuses.has(response.status)) {
		return response;
	}

	// `content-length` counts encoded bytes on the wire, while the progress stream counts the bytes after decompression. Using it for a content-coded response would report a total below what actually arrives, so every event would sit at ~100%. The total is then unknown, which `Progress` already models with `0`. `identity` is a registered no-op coding (RFC 9110 §8.4.2), so it leaves the length valid.
	const contentEncoding = response.headers.get('content-encoding')?.toLowerCase();
	const isContentCoded = Boolean(contentEncoding) && contentEncoding !== 'identity';
	const totalBytes = isContentCoded ? 0 : Math.max(0, Number(response.headers.get('content-length')) || 0);
	const body = withProgress(response.body, totalBytes, onDownloadProgress);

	return copyResponseMetadata(new Response(body, response), response);
};
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
- getProgressCallbackError (source/utils/body.ts:104-105)
```typescript
export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
	stream ? progressCallbackErrors.get(stream)?.() : undefined;
```
- Ky.#raceBodyRead (source/core/Ky.ts:883-949)
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
- KyOptions (source/types/options.ts:45-430)
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

	// On https://example.com

	const response = await ky('users', {prefix: '/api/'});
	//=> 'https://example.com/api/users'

	const response = await ky('/users', {prefix: '/api/'});
	//=> 'https://example.com/api/users'
	```
	*/
	prefix?: URL | string | undefined;

	/**
	Controls retry behavior. Each field is documented in the `RetryOptions` type.

	If `retry` is a number, it will be used as `limit` and other defaults will remain in place.

	`retry.limit`, including numeric shorthand, must be a finite, non-negative integer.

	Network errors (e.g., DNS failures, connection refused, offline) are automatically retried for retriable methods. Only errors recognized as network errors are retried; other errors (e.g., programming bugs) are thrown immediately. Use `shouldRetry` to customize this behavior.

	`413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`.

	When the response status is contained in `afterStatusCodes` and the retry is allowed by `statusCodes` or `shouldRetry`, Ky uses retry timing headers to choose the retry delay. [`Retry-After`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Retry-After) may provide a delay in seconds or an HTTP-date. If `Retry-After` is missing, Ky falls back to rate-limit timing headers (`RateLimit-Reset`, `X-RateLimit-Retry-After`, `X-RateLimit-Reset`, and `X-Rate-Limit-Reset`). Numeric `Retry-After` and `X-RateLimit-Retry-After` values are interpreted as delay seconds. Numeric `RateLimit-Reset`, `X-RateLimit-Reset`, and `X-Rate-Limit-Reset` values may also be interpreted as Unix timestamps, from 2001-09-09 onwards. A value below that is read as delay seconds, and a timestamp already in the past means the retry happens immediately. If the status code is not in `afterStatusCodes`, retry timing headers will be ignored.

	If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`.

	@example
	```
	import ky from 'ky';

	const json = await ky('https://example.com', {
		retry: {
			limit: 10,
			methods: ['get'],
			statusCodes: [500]
		}
	}).json();
	```
	*/
	retry?: RetryOptions | number | undefined;

	/**
	Per-attempt timeout in milliseconds for getting a response, applied independently to each retry. Ky shortcut methods also use this value as a separate timeout for reading the response body. Cannot be greater than 2147483647. See also `totalTimeout`.

	If set to `false`, there will be no per-attempt timeout.

	@default 10000
	*/
	timeout?: number | false | undefined;

	/**
	Overall timeout in milliseconds for the entire operation, including retries and delays. Throws a `TimeoutError` if exceeded. Cannot be greater than 2147483647.

	`beforeError` hooks run after an error is produced and are not bounded by `totalTimeout`.

	If set to `false` or not specified, there is no overall timeout.

	@default false

	@example
	```
	import ky from 'ky';

	// Each attempt gets 5s, but the whole operation must complete within 30s
	const json = await ky('https://example.com', {
		timeout: 5000,
		totalTimeout: 30_000,
		retry: {
			limit: 3,
			retryOnTimeout: true,
		}
	}).json();
	```
	*/
	totalTimeout?: number | false | undefined;

	/**
	Maximum response body size in bytes. Must be a non-negative safe integer or `Infinity`. Set to `0` to allow only empty bodies.

	The limit counts bytes from the response stream after decompression, independently of `Content-Length`. It applies as the body is consumed, including in `afterResponse` hooks and for responses returned by hooks. Exceeding the limit cancels the stream and throws a `ResponseSizeError`, without automatically retrying.

