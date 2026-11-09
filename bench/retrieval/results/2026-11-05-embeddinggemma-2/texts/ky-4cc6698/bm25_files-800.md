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
