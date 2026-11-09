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

	With `await ky(url)`, the response can resolve before the limit is exceeded; the body read will reject instead. This limits body bytes, not total memory usage. Parsing, buffering, and concurrent requests can use additional memory.

	Responses with a [null body status](https://fetch.spec.whatwg.org/#null-body-status), such as `204` and `205`, are never wrapped, because the `Response` constructor rejects a body for those statuses. A runtime that still exposes a body for such a status, such as WebKit for `205`, is therefore not limited.

	@default Infinity

	@example
	```
	import ky from 'ky';

	const data = await ky('https://example.com/data', {
		maxResponseSize: 20 * 1024 * 1024,
	}).json();
	```
	*/
	maxResponseSize?: number | undefined;

	/**
	Hooks allow modifications during the request lifecycle. Hook functions may be async and are run serially, unless otherwise noted.
	*/
	hooks?: Hooks | undefined;

	/**
	Throw an `HTTPError` when, after following redirects, the response has a non-2xx status code. To also throw for redirects instead of following them, set the [`redirect`](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch#Parameters) option to `'manual'`.

	Setting this to `false` may be useful if you are checking for resource availability and are expecting error responses.

	You can also pass a function that accepts the HTTP status code and returns a boolean for selective error handling. Note that this can violate the principle of least surprise, so it's recommended to use the boolean form unless you have a specific use case like treating 404 responses differently.

	Note: If `false`, error responses are considered successful and the request will not be retried.

	Note: [Opaque responses](https://developer.mozilla.org/en-US/docs/Web/API/Response/type) from `no-cors` requests are returned as-is (without throwing `HTTPError`), since the actual status is hidden by the browser.

	@default true
	*/
	throwHttpErrors?: boolean | ((status: number) => boolean) | undefined;

	/**
	Download progress event handler.

	@param progress - Object containing download progress information.
	@param chunk - Data that was received. When an empty response body stream completes, the callback receives an empty chunk.

	`content-length` is only used as the total for a response that is not content-coded, since it counts encoded bytes while the progress stream counts the bytes after decompression. A compressed response therefore reports `totalBytes: 0` until it completes, which means the percentage cannot be calculated while downloading.

	Responses with a [null body status](https://fetch.spec.whatwg.org/#null-body-status) are not streamed, so no progress events are emitted for them.

	When the callback throws, the error is reported by Ky's body method shortcuts. A response read directly with `response.text()` or `response.json()` may instead report the runtime's own stream error, such as Chromium's generic `TypeError`.

	@example
	```
	import ky from 'ky';

	const response = await ky('https://example.com', {
		onDownloadProgress: (progress, chunk) => {
			// Example output:
			// `100% - 1271 of 1271 bytes`
			console.log(`${progress.percent * 100}% - ${progress.transferredBytes} of ${progress.totalBytes} bytes`);
		}
	});
	```
	*/
	onDownloadProgress?: ((progress: Progress, chunk: Uint8Array) => void) | undefined;

	/**
	Upload progress event handler.

	Note: Requires [request stream support](https://caniuse.com/wf-fetch-request-streams) and, in Chromium-based browsers, an HTTP/2 or HTTP/3 connection (streaming uploads over HTTP/1.1 fail with a network error, even over plain HTTP). This handler is silently ignored in unsupported environments and for requests with `keepalive: true` or `mode: 'no-cors'`, since they cannot use streaming request bodies.

	@param progress - Object containing upload progress information.
	@param chunk - Data that was sent. When an empty request body stream completes, the callback receives an empty chunk.

	@example
	```
	import ky from 'ky';

	const response = await ky.post('https://example.com/upload', {
		body: largeFile,
		onUploadProgress: (progress, chunk) => {
			// Example output:
			// `100% - 1271 of 1271 bytes`
			console.log(`${progress.percent * 100}% - ${progress.transferredBytes} of ${progress.totalBytes} bytes`);
		}
	});
	```
	*/
	onUploadProgress?: ((progress: Progress, chunk: Uint8Array) => void) | undefined;

	/**
	User-defined `fetch` function.
	Has to be fully compatible with the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) standard.

	Use-cases:
	1. Use the `fetch` wrapper function provided by some frameworks that use server-side rendering (SSR).
	2. Add custom instrumentation or logging to all requests.

	@default fetch

	@example
	```
	import ky from 'ky';

	const api = ky.create({
		fetch: async (request, init) => {
			const start = performance.now();
			const response = await fetch(request, init);
			const duration = performance.now() - start;
			console.log(`${request.method} ${request.url} - ${response.status} (${Math.round(duration)}ms)`);
			return response;
		}
	});

	const json = await api('https://example.com').json();
	```
	*/
	fetch?: ((input: Request, init?: RequestInit) => Promise<Response>) | undefined;

	/**
	User-defined data passed to hooks.

	This option allows you to pass arbitrary contextual data to hooks without polluting the request itself. The context is available in all hooks and is **guaranteed to always be an object** (never `undefined`), so you can safely access properties without optional chaining.

	Use cases:
	- Pass authentication tokens or API keys to hooks
	- Attach request metadata for logging or debugging
	- Implement conditional logic in hooks based on the request context
	- Pass serverless environment bindings (e.g., Cloudflare Workers)

	**Note:** Context is shallow merged. Top-level properties are merged, but nested objects are replaced. Only enumerable properties are copied.

	**Note:** Each request gets its own deep copy of the context, so a hook that mutates a nested plain object or array cannot write back to the instance defaults or to the object you passed in. Non-plain values such as class instances are shared by reference. A hook that caches data in the context, for example with `options.context.token ??= await getToken()`, therefore recomputes it once per request rather than once per instance.

	@example
	```
	import ky from 'ky';

	// Pass data to hooks
	const api = ky.create({
		hooks: {
			beforeRequest: [
				({request, options}) => {
					const {token} = options.context;
					if (token) {
						request.headers.set('Authorization', `Bearer ${token}`);
					}
				}
			]
		}
	});

	await api('https://example.com', {
		context: {
			token: 'secret123'
		}
	}).json();

	// Shallow merge: only top-level properties are merged
	const instance = ky.create({
		context: {
			a: 1,
			b: {
				nested: true
			}
		}
	});

	const extended = instance.extend({
		context: {
			b: {
				updated: true
			},
			c: 3
		}
	});
	// Result: {a: 1, b: {updated: true}, c: 3}
	// Note: The original `b.nested` is gone (shallow merge)
	```

	@default {}
	*/
	context?: Record<string, unknown> | undefined;
};
```
- createBodyServer (test/request-body-serialization.ts:12-22)
```typescript
async function createBodyServer(t: ExecutionContext) {
	const server = await createHttpTestServer(t, {bodyParser: false});
	server.post('/', async (request, response) => {
		response.json({
			bytes: [...await buffer(request)],
			contentType: request.headers['content-type'],
			contentLength: request.headers['content-length'],
		});
	});
	return server;
}
```
- Ky.#fetch (source/core/Ky.ts:1368-1428)
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
- Ky.create (source/core/Ky.ts:253-452)
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
