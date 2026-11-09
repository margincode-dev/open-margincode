### test/response-consumption.ts
```typescript
import test from 'ava';
import ky, {HTTPError} from '../source/index.js';
import {createHttpTestServer} from './helpers/create-http-test-server.js';

const binary = Buffer.from([0, 255, 128, 13, 10, 0, 195, 40]);

// Fetch's Body mixin consumes bytes without text decoding for these methods.
// https://fetch.spec.whatwg.org/#body-mixin
for (const method of ['arrayBuffer', 'blob', 'bytes'] as const) {
	const run = method === 'bytes' && typeof Response.prototype.bytes !== 'function' ? test.skip : test;
	run(`${method} preserves non-text bytes through download progress`, async t => {
		const server = await createHttpTestServer(t);
		server.get('/', (_request, response) => {
			response.set({'content-type': 'application/octet-stream', 'content-length': String(binary.length)}).end(binary);
		});
		const chunks: Uint8Array[] = [];
		let finalTransferred = 0;
		const result = await ky(server.url, {
			onDownloadProgress(progress, chunk) {
				chunks.push(chunk);
				finalTransferred = progress.transferredBytes;
			},
		})[method]();
		if (method === 'blob') {
			t.true(result instanceof Blob);
		} else if (method === 'arrayBuffer') {
			t.true(result instanceof ArrayBuffer);
		} else {
			t.true(result instanceof Uint8Array);
		}

		const bytes = result instanceof Blob ? new Uint8Array(await result.arrayBuffer()) : new Uint8Array(result);
		t.deepEqual(Buffer.from(bytes), binary);
		t.deepEqual(Buffer.concat(chunks), binary);
		t.is(finalTransferred, binary.length);
		if (result instanceof Blob) {
			t.is(result.type, 'application/octet-stream');
			t.is(result.size, binary.length);
		}
	});
}

test('formData parses URL-encoded repeated fields, plus signs, and UTF-8 through progress', async t => {
	const server = await createHttpTestServer(t);
	const text = 'tag=one&tag=two&message=caf%C3%A9+time&literal=%2B&empty=';
	server.get('/', (_request, response) => {
		response.set('content-type', 'application/x-www-form-urlencoded').end(text);
	});
	const chunks: Uint8Array[] = [];
	const form = await ky(server.url, {
		onDownloadProgress(_progress, chunk) {
			chunks.push(chunk);
		},
	}).formData();

	t.deepEqual(form.getAll('tag'), ['one', 'two']);
	t.is(form.get('message'), 'café time');
	t.is(form.get('literal'), '+');
	t.is(form.get('empty'), '');
	t.is(Buffer.concat(chunks).toString(), text);
});

test('formData parses multipart file bytes and repeated fields', async t => {
	const server = await createHttpTestServer(t);
	const form = new FormData();
	form.append('tag', 'one');
	form.append('tag', 'two');
	form.append('file', new Blob([binary], {type: 'application/octet-stream'}), 'sample.bin');
	const encoded = new Response(form);
	const body = Buffer.from(await encoded.arrayBuffer());
	server.get('/', (_request, response) => {
		response.set('content-type', encoded.headers.get('content-type')!).end(body);
	});
	const parsed = await ky(server.url).formData();
	const file = parsed.get('file') as File;

	t.deepEqual(parsed.getAll('tag'), ['one', 'two']);
	t.true(file instanceof File);
	t.is(file.name, 'sample.bin');
	t.is(file.type, 'application/octet-stream');
	t.deepEqual(Buffer.from(await file.arrayBuffer()), binary);
});

test('formData rejects unsupported response media types without retrying', async t => {
	const server = await createHttpTestServer(t);
	let requests = 0;
	server.get('/', (_request, response) => {
		requests++;
		response.set('content-type', 'application/json').end('{}');
	});

	await t.throwsAsync(ky(server.url).formData(), {instanceOf: TypeError});
	t.is(requests, 1);
});

for (const firstMethod of ['text', 'json', 'arrayBuffer'] as const) {
	test(`${firstMethod} shortcut consumes the shared response exactly once`, async t => {
		const server = await createHttpTestServer(t);
		let requests = 0;
		server.get('/', (_request, response) => {
			requests++;
			response.json({value: 42});
		});
		const pending = ky(server.url);
		await pending[firstMethod]();
		const response = await pending;

		t.true(response.bodyUsed);
		await t.throwsAsync(pending.text(), {instanceOf: TypeError});
		t.throws(() => response.clone(), {instanceOf: TypeError});
		t.is(requests, 1);
	});
}

test('redirected progress clones retain metadata and use their own parser response context', async t => {
	const server = await createHttpTestServer(t);
	server.get('/start', (_request, response) => {
		response.redirect('/end');
	});
	server.get('/end', (_request, response) => {
		response.set('x-final', 'yes').json({value: 42});
	});
	const parserResponses: Response[] = [];
	const chunks: Uint8Array[] = [];
	const response = await ky(`${server.url}/start`, {
		parseJson(text, context) {
			parserResponses.push(context.response);
			return JSON.parse(text);
		},
		onDownloadProgress(_progress, chunk) {
			chunks.push(chunk);
		},
	});
	const clone = response.clone().clone();

	t.is(clone.url, `${server.url}/end`);
	t.true(clone.redirected);
	t.is(clone.headers.get('x-final'), 'yes');
	t.deepEqual(await clone.json(), {value: 42});
	t.false(response.bodyUsed);
	t.deepEqual(await response.json(), {value: 42});
	t.is(parserResponses.length, 2);
	t.is(parserResponses[0], clone);
	t.is(parserResponses[1], response);
	t.is(Buffer.concat(chunks).toString(), '{"value":42}');
});

// JSON texts can contain primitive values, including falsy ones (RFC 8259 §3).
// https://www.rfc-editor.org/rfc/rfc8259.html#section-3
for (const value of [false, true, 0, '', ['one', 2]]) {
	test(`JSON value ${JSON.stringify(value)} is preserved in successful and error responses`, async t => {
		const server = await createHttpTestServer(t);
		const text = JSON.stringify(value);
		server.get('/success', (_request, response) => {
			response.set('content-type', 'application/json').end(text);
		});
		server.get('/error', (_request, response) => {
			response.status(422).set('content-type', 'application/json').end(text);
		});

		t.deepEqual(await ky(`${server.url}/success`).json(), value);
		const error = await t.throwsAsync(ky(`${server.url}/error`), {instanceOf: HTTPError});
		t.deepEqual(error.data, value);
	});
}

test('disabling HTTP errors allows consuming a binary error response without JSON parsing', async t => {
	const server = await createHttpTestServer(t);
	let parserCalls = 0;
	server.get('/', (_request, response) => {
		response.status(422).set('content-type', 'application/octet-stream').end(binary);
	});
	const buffer = await ky(server.url, {
		throwHttpErrors: false,
		parseJson() {
			parserCalls++;
			return {};
		},
	}).arrayBuffer();

	t.deepEqual(Buffer.from(buffer), binary);
	t.is(parserCalls, 0);
});

test('formData parsing errors pass through beforeError exactly once without retrying', async t => {
	const server = await createHttpTestServer(t);
	let requests = 0;
	server.get('/', (_request, response) => {
		requests++;
		response.set('content-type', 'application/json').end('{}');
	});
	const replacement = new Error('Unexpected response format');
	let hookCalls = 0;

	await t.throwsAsync(ky(server.url, {
		hooks: {
			beforeError: [({error, request, retryCount}) => {
				hookCalls++;
				t.true(error instanceof TypeError);
				t.is(request.url, `${server.url}/`);
				t.is(retryCount, 0);
				return replacement;
			}],
		},
	}).formData(), {is: replacement});
	t.is(hookCalls, 1);
	t.is(requests, 1);
});

test('beforeError failures while handling body-reader errors propagate without running hooks again', async t => {
	const readError = new Error('Response body could not be read');
	const hookError = new Error('Error handler failed');
	let fetchCalls = 0;
	let hookCalls = 0;

	await t.throwsAsync(ky('https://example.com', {
		async fetch() {
			fetchCalls++;
			return new Response(new ReadableStream({
				start(controller) {
					controller.error(readError);
				},
			}));
		},
		hooks: {
			beforeError: [({error}) => {
				hookCalls++;
				t.is(error, readError);
				throw hookError;
			}],
		},
	}).text(), {is: hookError});

	t.is(fetchCalls, 1);
	t.is(hookCalls, 1);
});

// The media type each shortcut advertises is deliberately narrow, so a caller that wants another representation asks for it through `headers` instead.
test('each body shortcut advertises exactly one media type', async t => {
	const accepts: Record<string, string | undefined> = {};

	for (const type of ['json', 'text', 'formData', 'arrayBuffer', 'blob'] as const) {
		// eslint-disable-next-line no-await-in-loop
		await ky('https://example.com', {
			retry: 0,
			async fetch(request) {
				accepts[type] = request.headers.get('accept') ?? undefined;
				return new Response('x', {headers: {'content-type': 'application/json'}});
			},
		})[type]().catch(() => undefined);
	}

	t.deepEqual(accepts, {
		json: 'application/json',
		text: 'text/*',
		formData: 'multipart/form-data',
		arrayBuffer: '*/*',
		blob: '*/*',
	});
});
```
### source/types/options.ts
```typescript
import type {LiteralUnion} from './common.js';
import type {Hooks, NormalizedHooks} from './hooks.js';
import type {MutableRetryOptions, RetryOptions} from './retry.js';

// eslint-disable-next-line unicorn/prevent-abbreviations
export type SearchParamsInit = string | string[][] | Record<string, string> | URLSearchParams | undefined;

// `null` is intentionally not allowed in the object form even though the runtime sends it as the string `'null'` (like `URLSearchParams` does), so accidental nulls are caught by the type checker.
// eslint-disable-next-line unicorn/prevent-abbreviations
export type SearchParamsOption =
	| Exclude<SearchParamsInit, string[][]>
	| Record<string, string | number | boolean | undefined>
	| Array<Array<string | number | boolean>>
	| ReadonlyArray<ReadonlyArray<string | number | boolean>>;

export type RequestHttpMethod = 'get' | 'post' | 'put' | 'patch' | 'head' | 'delete' | 'query';
export type HttpMethod = LiteralUnion<RequestHttpMethod | 'options' | 'trace', string>;

export type Input = string | URL | Request;

export type Progress = {
	/**
	A number between `0` and `1` representing the progress percentage.
	*/
	percent: number;

	/**
	The number of bytes transferred so far.
	*/
	transferredBytes: number;

	/**
	The total number of bytes to be transferred. This is an estimate and may be `0` for an empty transfer or when the total size cannot be determined.
	*/
	totalBytes: number;
};

// Not HeadersInit directly because @types/node doesn't export it
export type KyHeadersInit = NonNullable<RequestInit['headers']> | Record<string, string | undefined> | ReadonlyArray<readonly [string, string]>;

/**
Custom Ky options
*/

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

	Responses with no body at all are not streamed, so no progress events are emitted for them. That covers a [null body status](https://fetch.spec.whatwg.org/#null-body-status) such as `204`, and a `HEAD` request, which has no body even on an ordinary status.

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

	A `ReadableStream` body cannot be measured, so `totalBytes` falls back to a `content-length` header you set. Browsers do not allow that header on a request, so there the total stays `0`.

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

/**
Each key from KyOptions is present and set to `true`.

This type is used for identifying and working with the known keys in KyOptions.
*/
export type KyOptionsRegistry = {[K in keyof KyOptions]-?: true};

type RequestOptions = {
	[Key in Exclude<keyof RequestInit, 'headers' | 'signal' | 'method'>]?: RequestInit[Key] | undefined;
};

/**
Options are the same as `window.fetch`, except for the KyOptions
*/
export interface Options extends KyOptions, RequestOptions { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
	/**
	HTTP method used to make the request.

	Internally, the standard methods (`GET`, `POST`, `PUT`, `PATCH`, `HEAD`, `DELETE`, and `QUERY`) are uppercased in order to avoid server errors due to case sensitivity.
	*/
	method?: LiteralUnion<HttpMethod, string> | undefined;

	/**
	HTTP headers used to make the request.

	You can pass a `Headers` instance or a plain object. Headers are normalized to a plain object with lowercase names when options are merged, so `init` hooks always see a plain object with lowercase keys. A header removed with `undefined` stays in that object with an `undefined` value.

	You can remove a header with `.extend()` by passing the header with an `undefined` value. Passing `undefined` as a string removes the header only if it comes from a `Headers` instance.

	@example
	```
	import ky from 'ky';

	const url = 'https://sindresorhus.com';

	const original = ky.create({
		headers: {
			rainbow: 'rainbow',
			unicorn: 'unicorn'
		}
	});

	const extended = original.extend({
		headers: {
			rainbow: undefined
		}
	});

	const response = await extended(url).json();

	console.log('rainbow' in response);
	//=> false

	console.log('unicorn' in response);
	//=> true
	```
	*/
	headers?: KyHeadersInit | undefined;

	/**
	An `AbortSignal` to abort the request.

	When extending an instance, signals are combined. Use `replaceOption(signal)` to replace inherited signals, or `signal: undefined` to remove them.

	`null` is accepted for `RequestInit` compatibility and is treated like an absent signal, so it does not remove an inherited signal.
	*/
	// eslint-disable-next-line @typescript-eslint/no-restricted-types
	signal?: AbortSignal | null | undefined;
}

export type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & {
	get headers(): Record<string, string | undefined>;
	set headers(value: KyHeadersInit | undefined);
	get context(): Record<string, unknown>;
	set context(value: Record<string, unknown> | undefined);
	retry?: MutableRetryOptions | number | undefined;
	hooks?: {[Key in keyof Hooks]?: NormalizedHooks[Key] | undefined} | undefined;
};

type NormalizedRetryOptions = {
	[Key in Exclude<keyof MutableRetryOptions, 'shouldRetry'>]-?: Key extends 'jitter' ? MutableRetryOptions[Key] : Exclude<MutableRetryOptions[Key], undefined>;
} & Pick<MutableRetryOptions, 'shouldRetry'>;

export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
	headers: Headers;
	hooks: NormalizedHooks;
	retry: NormalizedRetryOptions;
	fetch: NonNullable<Options['fetch']>;
	prefix: string;
	timeout: number | false;
	totalTimeout: number | false;
	maxResponseSize: number;
	context: Record<string, unknown>;
	throwHttpErrors: boolean | ((status: number) => boolean);
};

/**
Normalized options passed to the `fetch` call and hooks.
*/
export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
	// Extended from `RequestInit`, but ensured to be set (not optional).
	readonly method: NonNullable<RequestInit['method']>;
	readonly credentials?: NonNullable<RequestInit['credentials']>;
	readonly headers: Headers;

	// Extended from custom `KyOptions`, but ensured to be set (not optional).
	readonly retry: NormalizedRetryOptions;
	readonly baseUrl?: Options['baseUrl'];
	readonly prefix: string;
	readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>;
	readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>;
	readonly context: Record<string, unknown>;
}

export type {RetryOptions, ShouldRetryState} from './retry.js';
```
### source/types/retry.ts
```typescript
import type {HttpMethod} from './options.js';

export type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & {
	methods?: HttpMethod[] | undefined;
	statusCodes?: number[] | undefined;
	afterStatusCodes?: number[] | undefined;
};

export type ShouldRetryState = {
	/**
	The error that caused the request to fail.
	*/
	error: Error;

	/**
	The number of retries attempted. Starts at 1 for the first retry.
	*/
	retryCount: number;
};

export type RetryOptions = {
	/**
	The number of times to retry failed requests.
	Must be a finite, non-negative integer.

