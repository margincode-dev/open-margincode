### test/hooks.ts
```typescript
import {setTimeout as delay} from 'node:timers/promises';
import {runInNewContext} from 'node:vm';
import test, {type ExecutionContext} from 'ava';
import ky, {
	HTTPError,
	KyError,
	isHTTPError,
	isNetworkError,
	isTimeoutError,
	isForceRetryError,
	TimeoutError,
	replaceOption,
} from '../source/index.js';
import {type Options, type NormalizedOptions} from '../source/types/options.js';
import {createHttpTestServer} from './helpers/create-http-test-server.js';

const withHeader = (request: Request, name: string, value: string) => {
	const headers = new Headers(request.headers);
	headers.set(name, value);
	return new Request(request, {headers});
};

const createRequestLike = (request: Request): any => ({
	get headers() {
		return request.headers;
	},
	get method() {
		return request.method;
	},
	get signal() {
		return request.signal;
	},
	get url() {
		return request.url;
	},
	[Symbol.toStringTag]: 'Request',
	clone() {
		return createRequestLike(request.clone());
	},
});

const createResponseLike = (response: Response): any => ({
	get body() {
		return response.body;
	},
	get headers() {
		return response.headers;
	},
	get ok() {
		return response.ok;
	},
	get status() {
		return response.status;
	},
	get type() {
		return response.type;
	},
	[Symbol.toStringTag]: 'Response',
	clone() {
		return createResponseLike(response.clone());
	},
	async text() {
		return response.text();
	},
});

const requestFixtureUrl = 'about:blank';

test('beforeRequest can inspect cloned JSON without consuming the outgoing body', async t => {
	const body = {name: 'Ada'};
	let inspected = false;
	const result = await ky.post('https://example.com', {
		json: body,
		hooks: {
			beforeRequest: [async ({request}) => {
				const clone = request.clone();
				const nestedClone = clone.clone();
				t.deepEqual(await clone.json<{name: string}>(), body);
				t.deepEqual(await nestedClone.json<{name: string}>(), body);
				t.false(request.bodyUsed);
				inspected = true;
			}],
		},
		async fetch(input) {
			t.true(inspected);
			return Response.json(await (input as Request).json());
		},
	}).json();

	t.deepEqual(result, body);
});

const createStreamBody = (text: string) => new ReadableStream<Uint8Array>({
	start(controller) {
		controller.enqueue(new TextEncoder().encode(text));
		controller.close();
	},
});

const createStreamFetch = ({
	text = 'ok',
	onResponse,
}: {
	text?: string;
	onResponse?: (response: Response) => void;
} = {}): typeof fetch => async request => {
	if (!(request instanceof Request)) {
		throw new TypeError('Expected input to be a Request');
	}

	const response = new Response(createStreamBody(text));
	onResponse?.(response);
	return response;
};

test('forced retry uses defaults for undefined options', async t => {
	let requestCount = 0;
	let delayCalls = 0;
	let beforeRetryCalls = 0;
	const url = 'https://example.com';
	const response = await ky(url, {
		retry: {
			delay() {
				delayCalls++;
				return 0;
			},
		},
		async fetch(request) {
			requestCount++;
			t.is(request.url, `${url}/`);
			return new Response('Success');
		},
		hooks: {
			afterResponse: [({retryCount}) => {
				if (retryCount === 0) {
					return ky.retry({
						delay: undefined,
						code: undefined,
						cause: undefined,
						request: undefined,
					});
				}
			}],
			beforeRetry: [({error}) => {
				beforeRetryCalls++;
				t.assert(isForceRetryError(error));
				t.is(error.message, 'Forced retry');
				t.is(error.customDelay, undefined);
				t.is(error.code, undefined);
				t.is(error.cause, undefined);
				t.is(error.customRequest, undefined);
			}],
		},
	}).text();

	t.is(response, 'Success');
	t.is(requestCount, 2);
	t.is(delayCalls, 1);
	t.is(beforeRetryCalls, 1);
});

test('frozen hook lists work through extension and retries', async t => {
	const calls: string[] = [];
	const hooks = {
		init: Object.freeze([() => {
			calls.push('init');
		}]),
		beforeRequest: Object.freeze([() => {
			calls.push('beforeRequest');
		}]),
		beforeRetry: Object.freeze([() => {
			calls.push('beforeRetry');
		}]),
		afterResponse: Object.freeze([() => {
			calls.push('afterResponse');
		}]),
		beforeError: Object.freeze([({error}: {error: Error}) => {
			calls.push('beforeError');
			return error;
		}]),
	};
	const instance = ky.create({
		hooks,
		retry: {limit: 1, delay: () => 0},
		fetch: async () => new Response('Unavailable', {status: 503}),
	}).extend({
		hooks: {
			beforeRequest: [() => {
				calls.push('extended beforeRequest');
			}],
		},
	});

	await t.throwsAsync(instance('https://example.com'), {instanceOf: HTTPError});
	t.deepEqual(calls, ['init', 'beforeRequest', 'extended beforeRequest', 'afterResponse', 'beforeRetry', 'afterResponse', 'beforeError']);
});

test('hooks can be async', async t => {
	const server = await createHttpTestServer(t);
	server.post('/', async (request, response) => {
		response.json(request.body);
	});

	const json = {
		foo: true,
	};

	const responseJson = await ky
		.post(server.url, {
			json,
			hooks: {
				beforeRequest: [
					async ({request, options}) => {
						await delay(100);
						const bodyJson = JSON.parse(options.body as string);
						bodyJson.foo = false;
						return new Request(request, {body: JSON.stringify(bodyJson)});
					},
				],
			},
		})
		.json<typeof json>();

	t.false(responseJson.foo);
});

test('custom fetch can return Response-like object', async t => {
	// This URL is only a Request fixture. The custom `fetch` below handles the request, so no network request is made.
	const responseText = await ky(requestFixtureUrl, {
		fetch: async () => createResponseLike(new Response('ok')),
	}).text();

	t.is(responseText, 'ok');
});

test('custom fetch can return Response-like object tagged as Response', async t => {
	const responseText = await ky(requestFixtureUrl, {
		fetch: async () => createResponseLike(new Response('ok')),
	}).text();

	t.is(Object.prototype.toString.call(createResponseLike(new Response('ok'))), '[object Response]');
	t.is(responseText, 'ok');
});

test('awaited Response-like object uses parseJson for response.json()', async t => {
	const response = await ky(requestFixtureUrl, {
		fetch: async () => createResponseLike(new Response('{"foo":true}', {
			headers: {
				'content-type': 'application/json',
			},
		})),
		parseJson(text) {
			return {
				...JSON.parse(text),
				foo: false,
			};
		},
	});

	t.deepEqual(await response.json(), {foo: false});
});

test('hooks can be empty object', async t => {
	const expectedResponse = 'empty hook';
	const server = await createHttpTestServer(t);

	server.get('/', (_request, response) => {
		response.end(expectedResponse);
	});

	const response = await ky.get(server.url, {hooks: {}}).text();

	t.is(response, expectedResponse);
});

test('beforeRequest hook accepts Request-like object tagged as Request', async t => {
	const responseText = await ky(requestFixtureUrl, {
		fetch: async request => new Response(request.headers.get('x-tagged-request')),
		hooks: {
			beforeRequest: [
				({request}) => createRequestLike(withHeader(request, 'x-tagged-request', 'yes')),
			],
		},
	}).text();

	t.is(responseText, 'yes');
});

test('beforeRequest hook returning a Request-like object still retries after a timeout', async t => {
	let attempts = 0;
	const neverSettlingPromise = new Promise<never>(() => {
		void 0;
	});

	const text = await ky(requestFixtureUrl, {
		timeout: 40,
		retry: {limit: 1, retryOnTimeout: true, delay: () => 1},
		async fetch(request) {
			attempts++;
			t.is((request as Request).headers.get('x-tagged-request'), 'yes');
			return attempts === 1 ? neverSettlingPromise : new Response('retried');
		},
		hooks: {
			beforeRequest: [
				({request}) => createRequestLike(withHeader(request, 'x-tagged-request', 'yes')),
			],
		},
	}).text();

	t.is(text, 'retried');
	t.is(attempts, 2);
});

