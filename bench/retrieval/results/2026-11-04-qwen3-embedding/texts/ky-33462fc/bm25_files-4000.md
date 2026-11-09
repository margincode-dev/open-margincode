### source/utils/options.ts
```typescript
import {kyOptionKeys, requestOptionsRegistry} from '../core/constants.js';
import type {SearchParamsOption} from '../types/options.js';
import {deletedParametersSymbol} from './merge.js';

export const findUnknownOptions = (
	options: Record<string, unknown>,
): Record<string, unknown> => {
	const unknownOptions: Record<string, unknown> = {};

	for (const key in options) {
		// Skip inherited properties
		if (!Object.hasOwn(options, key)) {
			continue;
		}

		// Forward every non-standard, non-Ky option to fetch().
		// We intentionally do not check whether the key also exists on `Request`, because some runtimes
		// patch `Request.prototype` with fetch-only extensions. For example, Next.js adds `next`, and the
		// old `key in request` heuristic dropped it unless Ky kept a special-case allowlist.
		// Passing all non-standard keys makes that allowlist unnecessary and preserves future fetch extensions too.
		// The registries are plain objects, so `Object.hasOwn()` is required: `in` would also match
		// `Object.prototype` members such as `constructor` and `toString`, silently dropping those options.
		if (!Object.hasOwn(requestOptionsRegistry, key) && !Object.hasOwn(kyOptionKeys, key)) {
			unknownOptions[key] = options[key];
		}
	}

	return unknownOptions;
};

export const hasSearchParameters = (search: SearchParamsOption): boolean => {
	if (search === undefined) {
		return false;
	}

	// Option merging turns `null` into an absent value, but an `init` hook assigns straight onto the options object. `null` is not accepted, so report it instead of treating it as absent.
	if (search === null) {
		throw new TypeError('The `searchParams` option must not be `null`. Use `undefined` to clear it.');
	}

	// The `typeof array` still gives "object", so we need different checking for array.
	if (Array.isArray(search)) {
		return search.length > 0;
	}

	if (search instanceof URLSearchParams) {
		return search.size > 0 || Boolean((search as any)[deletedParametersSymbol]?.size);
	}

	// Record
	if (typeof search === 'object') {
		return Object.keys(search).length > 0;
	}

	if (typeof search === 'string') {
		return search.trim().length > 0;
	}

	return Boolean(search);
};
```
### test/validation.ts
```typescript
import test from 'ava';
import ky from '../source/index.js';

const okFetch = async () => new Response('ok');

// `timeout` and `totalTimeout` are documented as milliseconds or `false`. Anything else used to be passed
// straight to `setTimeout()`, which clamps it to ~1ms, or was silently ignored, so the request never got the
// timeout the caller asked for. `null` is left out: it is nullish, so it selects the default.
test('rejects non-numeric and negative `timeout` values', async t => {
	for (const timeout of ['1000', Number.NaN, -1, -0.5, {}] as unknown[]) {
		// eslint-disable-next-line no-await-in-loop
		await t.throwsAsync(
			ky('https://example.com', {timeout: timeout as never, fetch: okFetch}).text(),
			{
				name: 'TypeError',
				message: 'The `timeout` option must be a non-negative number or `false`',
			},
			`timeout: ${JSON.stringify(timeout)}`,
		);
	}
});

test('rejects non-numeric and negative `totalTimeout` values', async t => {
	for (const totalTimeout of ['1000', Number.NaN, -1, -0.5, {}] as unknown[]) {
		// eslint-disable-next-line no-await-in-loop
		await t.throwsAsync(
			ky('https://example.com', {totalTimeout: totalTimeout as never, fetch: okFetch}).text(),
			{
				name: 'TypeError',
				message: 'The `totalTimeout` option must be a non-negative number or `false`',
			},
			`totalTimeout: ${JSON.stringify(totalTimeout)}`,
		);
	}
});

test('treats a nullish timeout as absent', async t => {
	t.is(await ky('https://example.com', {timeout: null as never, totalTimeout: null as never, fetch: okFetch}).text(), 'ok');
});

test('rejects `timeout` and `totalTimeout` above the maximum safe value', async t => {
	for (const option of ['timeout', 'totalTimeout'] as const) {
		// eslint-disable-next-line no-await-in-loop
		await t.throwsAsync(
			ky('https://example.com', {[option]: 2_147_483_648, fetch: okFetch}).text(),
			{
				instanceOf: RangeError,
				message: `The \`${option}\` option cannot be greater than 2147483647`,
			},
			`option: ${option}`,
		);
	}
});

test('accepts `false`, `0` and non-negative numbers for the timeout options', async t => {
	t.is(await ky('https://example.com', {timeout: false, totalTimeout: false, fetch: okFetch}).text(), 'ok');
	t.is(await ky('https://example.com', {timeout: 0, totalTimeout: 5000, fetch: async () => new Response('a')}).text(), 'a');
});

test('rejects invalid timeout values set from an `init` hook', async t => {
	await t.throwsAsync(
		ky('https://example.com', {
			fetch: okFetch,
			hooks: {
				init: [options => {
					options.timeout = 'soon' as never;
				}],
			},
		}).text(),
		{
			name: 'TypeError',
			message: 'The `timeout` option must be a non-negative number or `false`',
		},
	);
});

// These callbacks are called directly, so a non-function value surfaced as a runtime error naming Ky's own
// internals (`this[#options].stringifyJson`, `initHookOptions.parseJson`) instead of the option the user set, and
// only once the request had already been built. The constructor calls `stringifyJson` itself, so that one has to be
// checked before anything else happens.
test('rejects non-function callback options', t => {
	for (const [key, value] of [
		['parseJson', 'x'],
		['stringifyJson', 5],
		['fetch', 'x'],
	] as Array<[string, unknown]>) {
		const options: Record<string, unknown> = {[key]: value};

		t.throws(() => {
			void ky('https://example.com', options as never);
		}, {
			name: 'TypeError',
			message: `The \`${key}\` option must be a function`,
		}, `${key}: ${JSON.stringify(value)}`);
	}
});

test('rejects a non-function `stringifyJson` before it is called', t => {
	t.throws(() => {
		void ky.post('https://example.com', {
			json: {a: 1},
			stringifyJson: 'nope' as never,
			fetch: okFetch,
		});
	}, {
		name: 'TypeError',
		message: 'The `stringifyJson` option must be a function',
	});
});

test('rejects a `throwHttpErrors` value that is neither a boolean nor a function', t => {
	for (const value of ['yes', 1, {}] as unknown[]) {
		const options: Record<string, unknown> = {
			throwHttpErrors: value,
			fetch: async () => new Response('server error', {status: 500}),
		};

		t.throws(() => {
			void ky('https://example.com', options as never);
		}, {
			name: 'TypeError',
			message: 'The `throwHttpErrors` option must be a boolean or a function',
		}, `throwHttpErrors: ${JSON.stringify(value)}`);
	}
});

test('accepts a valid `throwHttpErrors` function', async t => {
	const options: Record<string, unknown> = {
		throwHttpErrors: (status: number) => status === 404,
		fetch: async () => new Response('server error', {status: 500}),
	};

	t.is(await ky('https://example.com', options as never).text(), 'server error');
});

// The `input` type check ran after the options were built, so a nullish input dereferenced `.headers` first and
// reported an internal TypeError instead of the option the caller got wrong.
test('a nullish or non-string `input` reports the input error', t => {
	for (const input of [null, undefined, 5, {}, true] as unknown[]) {
		t.throws(() => {
			void ky(input as never, {fetch: okFetch});
		}, {
			name: 'TypeError',
			message: '`input` must be a string, URL, or Request',
		}, `input: ${String(input)}`);
	}
});

// `onDownloadProgress` was type-checked at the very end of the pipeline, so an invalid value ran the whole request
// and only then reported the typo, leaving the response body with nobody able to release it.
test('rejects a non-function `onDownloadProgress` before the request is sent', t => {
	let fetchCalled = false;
	let beforeRequestCalls = 0;

	t.throws(() => {
		void ky('https://example.com', {
			onDownloadProgress: 'x' as never,
			retry: 0,
			hooks: {
				beforeRequest: [() => {
					beforeRequestCalls++;
				}],
			},
			async fetch() {
				fetchCalled = true;
				return new Response('ok');
			},
		});
	}, {
		name: 'TypeError',
		message: 'The `onDownloadProgress` option must be a function',
	});

	t.false(fetchCalled);
	t.is(beforeRequestCalls, 0);
});
```
### test-d/hooks.ts
```typescript
import {expectTypeOf} from 'expect-type';
import type {BeforeErrorHook, Hooks, InitHook} from 'ky';

const hookList = [() => undefined] as const;
const beforeErrorHook: BeforeErrorHook = ({error}) => error;
const beforeErrorHooks = [beforeErrorHook] as const;
const exampleKey = String('example');
const hooks: Hooks = {
	init: hookList,
	beforeRequest: hookList,
	beforeRetry: hookList,
	beforeError: beforeErrorHooks,
	afterResponse: hookList,
};

expectTypeOf(hooks).toEqualTypeOf<Hooks>();

const initHook: InitHook = options => {
	options.hooks?.beforeRequest?.push(() => undefined);
	expectTypeOf(options.headers).toEqualTypeOf<Record<string, string | undefined>>();
	expectTypeOf(options.context).toEqualTypeOf<Record<string, unknown>>();
	options.headers = new Headers();
	options.headers[exampleKey] = 'value';
	options.context = undefined;
	options.context[exampleKey] = 'value';
	if (options.hooks) {
		options.hooks.beforeRequest = undefined;
	}
};

expectTypeOf(initHook).toEqualTypeOf<InitHook>();
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

