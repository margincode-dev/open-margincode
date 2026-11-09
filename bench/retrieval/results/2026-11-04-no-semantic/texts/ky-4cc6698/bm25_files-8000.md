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

test('beforeRetry hook returning a Request-like object still retries after a later timeout', async t => {
	let attempts = 0;
	const neverSettlingPromise = new Promise<never>(() => {
		void 0;
	});

	const text = await ky(requestFixtureUrl, {
		timeout: 40,
		retry: {limit: 2, retryOnTimeout: true, delay: () => 1},
		async fetch() {
			attempts++;
			if (attempts === 1) {
				throw new TypeError('fetch failed');
			}

			return attempts === 2 ? neverSettlingPromise : new Response('retried');
		},
		hooks: {
			beforeRetry: [
				({request, retryCount}) => retryCount === 1 ? createRequestLike(request) : undefined,
			],
		},
	}).text();

	t.is(text, 'retried');
	t.is(attempts, 3);
});

test('beforeRequest hook allows modifications', async t => {
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
					({request, options}) => {
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

test('afterResponse hook accepts success response', async t => {
	const server = await createHttpTestServer(t);
	server.post('/', async (request, response) => {
		response.json(request.body);
	});

	const json = {
		foo: true,
	};

	await t.notThrowsAsync(
		ky
			.post(server.url, {
				json,
				hooks: {
					afterResponse: [
						async ({response}) => {
							t.is(response.status, 200);
							t.deepEqual(await response.json(), json);
						},
					],
				},
			})
			.json(),
	);
});

test('afterResponse hook cancels unused cloned response body', async t => {
	const server = await createHttpTestServer(t);
	server.get('/', (_request, response) => {
		response.end('ok');
	});

	let didCancelClonedResponseBody = false;

	await ky.get(server.url, {
		hooks: {
			afterResponse: [
				async ({response}) => {
					if (response.body) {
						const originalCancel = response.body.cancel.bind(response.body);
						response.body.cancel = async () => {
							didCancelClonedResponseBody = true;
							return originalCancel();
						};
					}
				},
			],
		},
	}).text();

	t.true(didCancelClonedResponseBody);
});

test('afterResponse hook cancels both clone and original when it returns a new response', async t => {
	let originalResponse: Response | undefined;
	let clonedResponse: Response | undefined;

	const customFetch = createStreamFetch({
		onResponse(response) {
			originalResponse = response;
		},
	});

	const responseText = await ky('https://example.com', {
		fetch: customFetch,
		hooks: {
			afterResponse: [
				({response}) => {
					clonedResponse = response;
					return new Response('replacement');
				},
			],
		},
	}).text();

	t.is(responseText, 'replacement');
	t.true(originalResponse?.bodyUsed);
	t.true(clonedResponse?.bodyUsed);
});

test('afterResponse hook can return the provided response', async t => {
	let originalResponse: Response | undefined;

	const customFetch = createStreamFetch({
		onResponse(response) {
			originalResponse = response;
		},
	});

	const responseText = await ky('https://example.com', {
		fetch: customFetch,
		hooks: {
			afterResponse: [
				({response}) => response,
			],
		},
	}).text();

	t.is(responseText, 'ok');
	t.true(originalResponse?.bodyUsed);
});

test('afterResponse hook ignores non-Response return values', async t => {
	const responseText = await ky(requestFixtureUrl, {
		fetch: async () => new Response('ok'),
		hooks: {
			afterResponse: [
				() => false as false | Response,
			],
		},
	}).text();

	t.is(responseText, 'ok');
});

test('afterResponse hook runs for Response-like objects returned by custom fetch', async t => {
	const responseText = await ky(requestFixtureUrl, {
		fetch: async () => createResponseLike(new Response('ok')),
		hooks: {
			afterResponse: [
				() => new Response('replacement'),
			],
		},
	}).text();

	t.is(responseText, 'replacement');
});

test('afterResponse hook can read Response-like objects without consuming final body', async t => {
	const responseText = await ky(requestFixtureUrl, {
		fetch: async () => createResponseLike(new Response('ok')),
		hooks: {
			afterResponse: [
				async ({response}) => {
					t.is(await response.text(), 'ok');
				},
			],
		},
	}).text();

	t.is(responseText, 'ok');
});

test('afterResponse hook can wrap the provided body in a new response', async t => {
	const responseText = await ky('https://example.com', {
		fetch: async () => new Response('ok', {
			headers: {
				'content-type': 'text/plain',
			},
		}),
		hooks: {
			afterResponse: [
				({response}) => new Response(response.body, {
					headers: response.headers,
				}),
			],
		},
	}).text();

	t.is(responseText, 'ok');
});

test('afterResponse hook can wrap a streaming body in a new response', async t => {
	const customFetch = createStreamFetch({text: 'streamed'});

	const responseText = await ky('https://example.com', {
		fetch: customFetch,
		hooks: {
			afterResponse: [
				({response}) => new Response(response.body, {
					headers: response.headers,
				}),
			],
		},
	}).text();

	t.is(responseText, 'streamed');
});

test('afterResponse hook chain can forward the same body through multiple hooks', async t => {
	let originalResponse: Response | undefined;

	const customFetch = createStreamFetch({
		text: 'chained',
		onResponse(response) {
			originalResponse = response;
		},
	});

	const responseText = await ky('https://example.com', {
		fetch: customFetch,
		hooks: {
			afterResponse: [
				({response}) => new Response(response.body, {
					headers: response.headers,
				}),
				({response}) => new Response(response.body, {
					headers: response.headers,
				}),
			],
		},
	}).text();

	t.is(responseText, 'chained');
	// The original fetch response body should be cancelled since hooks created new wrappers
	t.true(originalResponse?.bodyUsed);
});

test('afterResponse hook with multiple hooks cancels all unused clones', async t => {
	let originalResponse: Response | undefined;
	const clones: Response[] = [];

	const customFetch = createStreamFetch({
		onResponse(response) {
			originalResponse = response;
		},
	});

	const responseText = await ky('https://example.com', {
		fetch: customFetch,
		hooks: {
			afterResponse: [
				({response}) => {
					clones.push(response);
					// Return nothing - clone should be cancelled
				},
				({response}) => {
					clones.push(response);
					// Return nothing - clone should be cancelled
				},
				({response}) => {
					clones.push(response);
					// Return nothing - clone should be cancelled
				},
			],
		},
	}).text();

	t.is(responseText, 'ok');
	t.is(clones.length, 3);

	// All clones should be cancelled (bodyUsed becomes true after cancel)
	for (const clone of clones) {
		t.true(clone.bodyUsed);
	}
});

test('afterResponse hook cancels response bodies when it throws', async t => {
	let originalResponse: Response | undefined;
	let clonedResponse: Response | undefined;

	const customFetch = createStreamFetch({
		onResponse(response) {
			originalResponse = response;
		},
	});

	const expectError = new Error('Hook error');

	await t.throwsAsync(
		ky('https://example.com', {
			fetch: customFetch,
			hooks: {
				afterResponse: [
					({response}) => {
						clonedResponse = response;
						throw expectError;
					},
				],
			},
		}).text(),
		{is: expectError},
	);

	t.true(originalResponse?.bodyUsed);
	t.true(clonedResponse?.bodyUsed);
});

test('afterResponse hook accepts failed response', async t => {
	const server = await createHttpTestServer(t);
	server.post('/', async (request, response) => {
		response.status(500).send(request.body);
	});

	const json = {
		foo: true,
	};

	await t.throwsAsync(
		ky
			.post(server.url, {
				json,
				hooks: {
					afterResponse: [
						async ({response}) => {
							t.is(response.status, 500);
							t.deepEqual(await response.json(), json);
						},
					],
				},
			})
			.json(),
	);
});

test('afterResponse hook can change response instance by sequence', async t => {
	const server = await createHttpTestServer(t);
	server.post('/', (_request, response) => {
		response.status(500).send();
	});

	const modifiedBody1 = 'hello ky';
	const modifiedStatus1 = 400;
	const modifiedBody2 = 'hello ky again';
	const modifiedStatus2 = 200;

	await t.notThrowsAsync(async () => {
		const responseBody = await ky
			.post(server.url, {
				hooks: {
					afterResponse: [
						() =>
							new Response(modifiedBody1, {
								status: modifiedStatus1,
							}),
						async ({response}) => {
							t.is(response.status, modifiedStatus1);
							t.is(await response.text(), modifiedBody1);

							return new Response(modifiedBody2, {
								status: modifiedStatus2,
							});
						},
					],
				},
			})
			.text();

		t.is(responseBody, modifiedBody2);
	});
});

test('afterResponse hook can throw error to reject the request promise', async t => {
	let requestCount = 0;
	const server = await createHttpTestServer(t);
	server.get('/', (_request, response) => {
		requestCount++;
		response.status(200).send();
	});

	const expectError = new Error('Error from `afterResponse` hook');

	// Sync hook function
	await t.throwsAsync(
		ky
			.get(server.url, {
				hooks: {
					afterResponse: [
						() => {
							throw expectError;
						},
					],
				},
			})
			.text(),
		{
			is: expectError,
		},
	);

	// Async hook function
	await t.throwsAsync(
		ky
			.get(server.url, {
				hooks: {
					afterResponse: [
						async () => {
							throw expectError;
						},
					],
				},
			})
			.text(),
		{
			is: expectError,
		},
	);

	// Two calls (sync + async), each making exactly 1 request (no retries triggered by hook errors)
	t.is(requestCount, 2);
});

test('`afterResponse` hook gets called even if using body shortcuts', async t => {
	const server = await createHttpTestServer(t);
	server.get('/', (_request, response) => {
		response.json({});
	});

	let called = false;
	await ky
		.get(server.url, {
			hooks: {
				afterResponse: [
					({response}) => {
						called = true;
						return response;
					},
				],
			},
		})
		.json();

	t.true(called);
});

test('`afterResponse` hook is called with request, normalized options, and response which can be used to retry', async t => {
	const server = await createHttpTestServer(t);
	server.post('/', async (request, response) => {
		const json = request.body;
		if (json.token === 'valid:token') {
			response.json(json);
		} else {
			response.sendStatus(403);
		}
	});

	const json = {
		foo: true,
		token: 'invalid:token',
	};

	t.deepEqual(
		await ky
			.post(server.url, {
				json,
				hooks: {
					afterResponse: [
						async ({request, options, response}) => {
							if (response.status === 403) {
								// Retry request with valid token
								return ky(request, {
									...options,
									json: {
										...json,
										token: 'valid:token',
									},
								});
							}

							return undefined;
						},
					],
				},
			})
			.json(),
		{
			foo: true,
			token: 'valid:token',
		},
	);
});

test('afterResponse hook with parseJson and response.json()', async t => {
	t.plan(5);

	const server = await createHttpTestServer(t);
	server.get('/', async (_request, response) => {
		response.end('text');
	});

	const json = await ky
		.get(server.url, {
			parseJson(text) {
				t.is(text, 'text');
				return {awesome: true};
			},
			hooks: {
				afterResponse: [
					async ({response}) => {
						t.true(response instanceof Response);
						t.deepEqual(await response.json(), {awesome: true});
					},
				],
			},
		})
		.json();

	t.deepEqual(json, {awesome: true});
});

test('beforeRetry hook is never called for the initial request', async t => {
	const fixture = 'fixture';
	const server = await createHttpTestServer(t);
	server.get('/', async (request, response) => {
		response.end(request.headers.unicorn);
	});

	t.not(
		await ky
			.get(server.url, {
				hooks: {
					beforeRetry: [
						({options}) => {
							(options.headers as Headers | undefined)?.set('unicorn', fixture);
						},
					],
				},
			})
			.text(),
		fixture,
	);
});

test('beforeRequest hook on initial request cannot bypass totalTimeout budget', async t => {
	let fetchCallCount = 0;

	const customFetch: typeof fetch = async () => {
		fetchCallCount++;
		return new Response('ok');
	};

	const error = await t.throwsAsync(
		ky('https://example.com', {
			fetch: customFetch,
			totalTimeout: 100,
			hooks: {
				beforeRequest: [
					async () => {
						await delay(200);
					},
				],
			},
		}).text(),
		{
			name: 'TimeoutError',
		},
	);

	t.true(error instanceof TimeoutError);
	t.is(fetchCallCount, 0);
});

test('totalTimeout bounds a never-ending beforeRequest hook', async t => {
	let markHookStarted: () => void;
	const hookStarted = new Promise<void>(resolve => {
		markHookStarted = resolve;
	});
	const neverSettlingPromise = new Promise<never>(() => {
		void 0;
	});

	const request = ky('https://example.com', {
		fetch: async () => new Response('ok'),
		totalTimeout: 500,
		hooks: {
			beforeRequest: [
				async () => {
					markHookStarted();
					await neverSettlingPromise;
				},
			],
		},
	}).text();

	await hookStarted;

	const result = await Promise.race([
		request.catch((error: unknown) => error),
		delay(2000).then(() => 'still pending'),
	]);

	t.true(result instanceof TimeoutError);
});

test.serial('totalTimeout rejects a hook result produced after the deadline', async t => {
	const originalPerformanceNow = globalThis.performance.now;
	let currentTime = 0;
	let didCancelBody = false;
	globalThis.performance.now = () => currentTime;

	t.teardown(() => {
		globalThis.performance.now = originalPerformanceNow;
	});

	await t.throwsAsync(
		ky('https://example.com', {
			totalTimeout: 500,
			hooks: {
				beforeRequest: [
					() => {
						currentTime = 501;
						return new Response(new ReadableStream({
							cancel() {
								didCancelBody = true;
							},
						}));
					},
				],
			},
		}),
		{
			instanceOf: TimeoutError,
		},
	);

	await delay(0);
	t.true(didCancelBody);
});

test('totalTimeout cancels a hook response produced after the timer wins', async t => {
	let resolveHook: (response: Response) => void;
	let didCancelBody = false;
	let markHookStarted: () => void;
	const hookStarted = new Promise<void>(resolve => {
		markHookStarted = resolve;
	});

	const request = ky('https://example.com', {
		totalTimeout: 500,
		hooks: {
			beforeRequest: [
				async () => new Promise<Response>(resolve => {
					resolveHook = resolve;
					markHookStarted();
				}),
			],
		},
	});

	await hookStarted;
	await t.throwsAsync(request, {instanceOf: TimeoutError});

	resolveHook(new Response(new ReadableStream({
		cancel() {
			didCancelBody = true;
		},
	})));
	await delay(0);

	t.true(didCancelBody);
});

test.serial('totalTimeout takes precedence over a hook error produced after the deadline', async t => {
	const originalPerformanceNow = globalThis.performance.now;
	let currentTime = 0;
	globalThis.performance.now = () => currentTime;

	t.teardown(() => {
		globalThis.performance.now = originalPerformanceNow;
	});

	await t.throwsAsync(
		ky('https://example.com', {
			totalTimeout: 500,
			hooks: {
				beforeRequest: [
					() => {
						currentTime = 501;
						throw new Error('late hook error');
					},
				],
			},
		}),
		{
			instanceOf: TimeoutError,
		},
	);
});

test('totalTimeout bounds a never-ending afterResponse hook', async t => {
	let markHookStarted: () => void;
	let didAbort = false;
	const hookStarted = new Promise<void>(resolve => {
		markHookStarted = resolve;
	});
	const neverSettlingPromise = new Promise<never>(() => {
		void 0;
	});

	const request = ky('https://example.com', {
		async fetch(request) {
			request.signal.addEventListener('abort', () => {
				didAbort = true;
			}, {once: true});

			return new Response('ok');
		},
		totalTimeout: 500,
		hooks: {
			afterResponse: [
				async () => {
					markHookStarted();
					await neverSettlingPromise;
				},
			],
		},
	}).text();

	await hookStarted;

	const result = await Promise.race([
		request.catch((error: unknown) => error),
		delay(2000).then(() => 'still pending'),
	]);

	t.true(result instanceof TimeoutError);
	t.true(didAbort);
});

test('beforeRequest hook runs only once and does not run again on retry', async t => {
	let fetchCallCount = 0;
	let beforeRequestCallCount = 0;
	const beforeRequestRetryCounts: number[] = [];

	const customFetch: typeof fetch = async () => {
		fetchCallCount++;
		if (fetchCallCount === 1) {
			return new Response('first-attempt', {status: 500});
		}

		return new Response('second-attempt-success');
	};

	t.is(
		await ky('https://example.com', {
			fetch: customFetch,
			retry: {
				limit: 1,
				delay: () => 0,
			},
			hooks: {
				beforeRequest: [
					({retryCount}) => {
						beforeRequestCallCount++;
						beforeRequestRetryCounts.push(retryCount);
					},
				],
			},
		}).text(),
		'second-attempt-success',
	);

	t.is(beforeRequestCallCount, 1);
	t.deepEqual(beforeRequestRetryCounts, [0]);
	t.is(fetchCallCount, 2);
});

test('beforeRetry hook allows modifications of non initial requests', async t => {
	let requestCount = 0;

	const fixture = 'fixture';
	const server = await createHttpTestServer(t);
	server.get('/', async (request, response) => {
		requestCount++;

		if (requestCount > 1) {
			response.end(request.headers.unicorn);
		} else {
			response.sendStatus(408);
		}
	});

	t.is(
		await ky
			.get(server.url, {
				hooks: {
					beforeRetry: [
						({request}) => {
							request.headers.set('unicorn', fixture);
						},
					],
				},
			})
			.text(),
		fixture,
	);
});

test('beforeRetry hook is called with error and retryCount', async t => {
	let requestCount = 0;

	const server = await createHttpTestServer(t);
	server.get('/', async (request, response) => {
		requestCount++;

		if (requestCount > 1) {
			response.end(request.headers.unicorn);
		} else {
			response.sendStatus(408);
		}
	});

	await ky.get(server.url, {
		hooks: {
			beforeRetry: [
				({error, retryCount}) => {
					t.true(error instanceof HTTPError);
					t.true(isHTTPError(error));
					t.true(retryCount >= 1);
				},
			],
		},
	});
});

test('beforeRetry hook is called even if the error has no response', async t => {
	t.plan(6);

	let requestCount = 0;

	const server = await createHttpTestServer(t);
	server.get('/', async (_request, response) => {
		requestCount++;
		response.end('unicorn');
	});

	const text = await ky
		.get(server.url, {
			retry: 1,
			async fetch(request) {
				if (requestCount === 0) {
					requestCount++;
					throw new TypeError('Failed to fetch');
				}

				return globalThis.fetch(request);
			},
			hooks: {
				beforeRetry: [
					({error, retryCount}) => {
						t.true(isNetworkError(error));
						// @ts-expect-error
						t.is(error.response, undefined);
						t.is(retryCount, 1);
						t.is(requestCount, 1);
					},
				],
			},
		})
		.text();

	t.is(text, 'unicorn');
	t.is(requestCount, 2);
});

test('beforeRetry hook with parseJson and error.data', async t => {
	t.plan(11);

	let requestCount = 0;

	const server = await createHttpTestServer(t);
	server.get('/', async (_request, response) => {
		requestCount++;
		if (requestCount === 1) {
			response.status(502).type('application/json').send('text');
		} else {
			response.end('text');
		}
	});

	const json = await ky
		.get(server.url, {
			retry: 1,
			parseJson(text) {
				t.is(text, 'text');
				return {awesome: true};
			},
			hooks: {
				beforeRetry: [
					async ({error, retryCount}) => {
						t.true(error instanceof HTTPError);
						t.true(isHTTPError(error));
						t.is(error.message, `Request failed with status code 502 Bad Gateway: GET ${server.url}/`);
						t.true((error as HTTPError).response instanceof Response);
						t.deepEqual((error as HTTPError).data, {awesome: true});
						t.is(retryCount, 1);
						t.is(requestCount, 1);
					},
				],
			},
		})
		.json();

	t.deepEqual(json, {awesome: true});
	t.is(requestCount, 2);
});

test('beforeRetry hook with async parseJson and error.data', async t => {
	t.plan(12);

	let requestCount = 0;

	const server = await createHttpTestServer(t);
	server.get('/', async (_request, response) => {
		requestCount++;
		if (requestCount === 1) {
			response.status(502).type('application/json').send('text');
		} else {
			response.end('text');
		}
	});

	const json = await ky
		.get(server.url, {
			retry: 1,
			async parseJson(text) {
				t.is(text, 'text');
				await Promise.resolve();
				return {awesome: true};
			},
			hooks: {
				beforeRetry: [
					async ({error, retryCount}) => {
						t.true(error instanceof HTTPError);
						t.true(isHTTPError(error));
						t.is(error.message, `Request failed with status code 502 Bad Gateway: GET ${server.url}/`);
						t.true((error as HTTPError).response instanceof Response);
						t.deepEqual((error as HTTPError).data, {awesome: true});
						t.false((error as HTTPError).data instanceof Promise);
						t.is(retryCount, 1);
						t.is(requestCount, 1);
					},
				],
			},
		})
		.json();

	t.deepEqual(json, {awesome: true});
	t.is(requestCount, 2);
});

test('beforeRetry hook gets HTTPError when async parseJson rejects', async t => {
	t.plan(7);

	let requestCount = 0;

	const server = await createHttpTestServer(t);
	server.get('/', async (_request, response) => {
		requestCount++;
		if (requestCount === 1) {
			response.status(502).type('application/json').send('text');
		} else {
			response.end('ok');
		}
	});

	const text = await ky
		.get(server.url, {
			retry: 1,
			async parseJson() {
				throw new Error('custom parse failure');
			},
			hooks: {
				beforeRetry: [
					({error, retryCount}) => {
						t.true(error instanceof HTTPError);
						t.true(isHTTPError(error));
						t.is((error as HTTPError).data, undefined);
						t.is(retryCount, 1);
						t.is(requestCount, 1);
					},
				],
			},
		})
		.text();

	t.is(text, 'ok');
	t.is(requestCount, 2);
});

test('beforeRetry hook can cancel retries by returning `stop`', async t => {
	let requestCount = 0;

	const server = await createHttpTestServer(t);
	server.get('/', async (request, response) => {
		requestCount++;

		if (requestCount > 2) {
			response.end(request.headers.unicorn);
		} else {
			response.sendStatus(408);
		}
	});

	await ky.get(server.url, {
		hooks: {
			beforeRetry: [
				({error, retryCount}) => {
					t.truthy(error);
					t.is(retryCount, 1);

					return ky.stop;
				},
			],
		},
	});

	t.is(requestCount, 1);
});

test('beforeRetry hook respects totalTimeout budget', async t => {
