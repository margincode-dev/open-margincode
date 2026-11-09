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

