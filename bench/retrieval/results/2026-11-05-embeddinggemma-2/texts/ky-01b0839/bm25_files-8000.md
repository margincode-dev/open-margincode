### source/types/hooks.ts
```typescript
import {type stop, type RetryMarker} from '../core/constants.js';
import type {KyRequest, KyResponse} from '../index.js';
import type {InitOptions, NormalizedOptions} from './options.js';

/**
This hook enables you to modify the options before they are used to construct the request. The hook function receives the mutable options object and can modify it in place. You could, for example, modify `searchParams`, `headers`, or `json` here. The `headers` option is always a plain object with lowercase names, where a header removed with `undefined` keeps an `undefined` value.

The `body` option is the exception: it is passed through by reference, so mutating a `FormData` or `URLSearchParams` body in place changes your own object and repeats on every request from the same instance. Replace `options.body` with a new instance instead, the way the `beforeRequest` workflow does.

Unlike other hooks, `init` hooks are synchronous. Any error thrown will propagate synchronously and will not be caught by `beforeError` hooks.

@example
```
import ky from 'ky';

const api = ky.extend({
	hooks: {
		init: [
			options => {
				options.searchParams = {apiKey: getApiKey()};
			},
		],
	},
});

const response = await api.get('https://example.com/api/users');
// URL: https://example.com/api/users?apiKey=123
```
*/
export type InitHook = (options: InitOptions) => void;

export type BeforeRequestState = {
	request: KyRequest;
	options: Readonly<NormalizedOptions>;

	/**
	The number of retries attempted. Always `0`, since `beforeRequest` hooks run once before retry handling begins.
	*/
	retryCount: 0;
};

export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;

export type BeforeRetryState = {
	request: KyRequest;
	options: Readonly<NormalizedOptions>;
	error: Error;

	/**
	The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request.
	*/
	retryCount: number;
};

export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;

export type BeforeErrorState = {
	request: KyRequest;
	options: Readonly<NormalizedOptions>;

	// `Error` (not `KyError`) because this receives all errors, including non-Ky ones.
	error: Error;

	/**
	The number of retries attempted. `0` for the initial request, increments with each retry.

	This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt).
	*/
	retryCount: number;
};

// Returns `Error` to allow replacing Ky errors with custom non-Ky error types.
export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;

export type AfterResponseState = {
	request: KyRequest;
	options: Readonly<NormalizedOptions>;
	response: KyResponse;

	/**
	The number of retries attempted. `0` for the initial request, increments with each retry.

	This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry).
	*/
	retryCount: number;
};

export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;

export type Hooks = {
	/**
	This hook enables you to modify the options before they are used to construct the request. The hook function receives the mutable options object and can modify it in place. You could, for example, modify `searchParams`, `headers`, or `json` here. The `headers` option is always a plain object with lowercase names, where a header removed with `undefined` keeps an `undefined` value.

	Unlike other hooks, `init` hooks are synchronous. Any error thrown will propagate synchronously and will not be caught by `beforeError` hooks.

	The `body` option is passed through by reference, so mutating a `FormData` or `URLSearchParams` body in place changes your own object and repeats on every request from the same instance. Replace `options.body` with a new instance instead, the way the `beforeRequest` workflow does.

	A common use case is to add a search parameter to every request:

	@example
	```
	import ky from 'ky';

	const api = ky.extend({
		hooks: {
			init: [
				options => {
					options.searchParams = {apiKey: getApiKey()};
				},
			],
		},
	});

	const response = await api.get('https://example.com/api/users');
	// URL: https://example.com/api/users?apiKey=123
	```

	@default []
	*/
	init?: readonly InitHook[] | undefined;

	/**
	This hook enables you to modify the request right before it is sent. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, and retry count. You could, for example, modify `request.headers` here.

	The `retryCount` is always `0`, since `beforeRequest` hooks run once before retry handling begins.

	The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing request (remaining hooks will still run with the updated request). It can also return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to completely avoid making an HTTP request, in which case remaining `beforeRequest` hooks are skipped. This can be used to mock a request, check an internal cache, etc. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work.

	Any error thrown by `beforeRequest` hooks is treated as fatal and will not trigger Ky's retry logic.

	@example
	```
	import ky from 'ky';

	const api = ky.extend({
		hooks: {
			beforeRequest: [
				({request}) => {
					request.headers.set('Authorization', 'token initial-token');
				}
			]
		}
	});

	const response = await api.get('https://example.com/api/users');
	```

	**Modifying the request URL:**

	@example
	```
	import ky from 'ky';

	const api = ky.extend({
		hooks: {
			beforeRequest: [
				({request}) => {
					const url = new URL(request.url);
					url.searchParams.set('token', 'secret-token');
					return new Request(url, request);
				}
			]
		}
	});

	const response = await api.get('https://example.com/api/users');
	```

	@default []
	*/
	beforeRequest?: readonly BeforeRequestHook[] | undefined;

	/**
	This hook enables you to modify the request right before retry. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, an error instance, and retry count. You could, for example, modify `request.headers` here.

	The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing retry request, or return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to skip the retry and use that response instead. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. **Note:** Returning a request or response skips remaining `beforeRetry` hooks.

	**Warning:** Returned `Request` objects are used as-is. If you point one at another origin, remove any credentials you do not want forwarded.

	The `retryCount` is always `>= 1`, since this hook is only called during retries, not on the initial request.

	If the request received a response, the error will be of type `HTTPError`. The `Response` object will be available at `error.response`, and the pre-parsed response body may be available at `error.data`. Be aware that some types of errors, such as network errors, inherently mean that a response was not received. In that case, the error will be an instance of `NetworkError` instead of `HTTPError`.

	You can prevent Ky from retrying the request by throwing an error. Ky will not handle it in any way and the error will be propagated to the request initiator. The rest of the `beforeRetry` hooks will not be called in this case. Alternatively, you can return the [`ky.stop`](#kystop) symbol to do the same thing but without propagating an error (this has some limitations, see `ky.stop` docs for details).

	**Modifying headers:**

	@example
	```
	import ky from 'ky';

	const response = await ky('https://example.com', {
		hooks: {
			beforeRetry: [
				async ({request, options, error, retryCount}) => {
					const token = await ky('https://example.com/refresh-token');
					request.headers.set('Authorization', `token ${token}`);
				}
			]
		}
	});
	```

	**Modifying the request URL:**

	@example
	```
	import ky, {isHTTPError} from 'ky';

	const response = await ky('https://example.com/api', {
		hooks: {
			beforeRetry: [
				({request, error}) => {
					// Add query parameters based on error response
					if (
						isHTTPError(error)
						&& typeof error.data === 'object'
						&& error.data !== null
						&& 'processId' in error.data
					) {
						const url = new URL(request.url);
						url.searchParams.set('processId', String(error.data.processId));
						return new Request(url, request);
					}
				}
			]
		}
	});
	```

	**Returning a cached response:**

	@example
	```
	import ky from 'ky';

	const response = await ky('https://example.com/api', {
		hooks: {
			beforeRetry: [
				({error, retryCount}) => {
					// Use cached response instead of retrying
					if (retryCount > 1 && cachedResponse) {
						return cachedResponse;
					}
				}
			]
		}
	});
	```

	@default []
	*/
	beforeRetry?: readonly BeforeRetryHook[] | undefined;

	/**
	This hook enables you to modify any error right before it is thrown. The hook function receives a state object with the current request, the normalized Ky options, the error, and retry count, and should return an `Error` instance.

	This hook receives errors Ky handles during its request lifecycle, including `HTTPError`, `NetworkError`, `TimeoutError`, `ResponseSizeError`, and `ForceRetryError` (when retry limit is exceeded via `ky.retry()`). Use type guards like `isHTTPError()`, `isNetworkError()`, `isTimeoutError()`, or `isResponseSizeError()` to handle specific error types.

	Errors raised while consuming an already returned `Response` are outside the request lifecycle. Use a body method shortcut like `await ky(url).json()` to process body-read `ResponseSizeError`, `NetworkError`, and `TimeoutError` instances through this hook.

	The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt).

	If a `beforeRequest` or `beforeRetry` hook returns a new `Request`, inspect `request` for the final request state. `options` remains Ky's normalized options and may not mirror every property of a replacement `Request`.

	@default []

	@example
	```
	import ky, {isHTTPError} from 'ky';

	await ky('https://example.com', {
		hooks: {
			beforeError: [
				({request, options, error}) => {
					if (isHTTPError(error)) {
						if (
							typeof error.data === 'object'
							&& error.data !== null
							&& 'message' in error.data
						) {
							error.name = 'GitHubError';
							error.message = `${String(error.data.message)} (${error.response.status})`;
						}
					}

					// `request` and `options` are always available
					console.log(`Request to ${request.url} failed`, options.context);

					return error;
				}
			]
		}
	});
	```
	*/
	beforeError?: readonly BeforeErrorHook[] | undefined;

	/**
	This hook enables you to read and optionally modify the response. The hook function receives a state object with the normalized request, options, a clone of the response, and retry count. The return value of the hook function will be used by Ky as the response object if it's an instance of [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response).

	You can also force a retry by returning [`ky.retry(options)`](#kyretryoptions). This is useful when you need to retry based on the response body content, even if the response has a successful status code. The retry will respect the `retry.limit` option and be observable in `beforeRetry` hooks.

	**Warning:** `ky.retry({request})` uses the replacement request as-is. If it targets another origin, remove any credentials you do not want forwarded.

	Any non-`ky.retry()` error thrown by `afterResponse` hooks is treated as fatal and will not trigger Ky's retry logic.

	The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry).

	@default []

	@example
	```
	import ky from 'ky';

	const response = await ky('https://example.com', {
		hooks: {
			afterResponse: [
				({response}) => {
					// You could do something with the response, for example, logging.
					log(response);

					// Or return a `Response` instance to overwrite the response.
					return new Response('A different response', {status: 200});
				},

				// Or retry with a fresh token on a 401 error
				async ({request, response, retryCount}) => {
					if (response.status === 401 && retryCount === 0) {
						// Only refresh on first 401, not on subsequent retries
						const {token} = await ky.post('https://example.com/auth/refresh').json();

						const headers = new Headers(request.headers);
						headers.set('Authorization', `Bearer ${token}`);

						return ky.retry({
							request: new Request(request, {headers}),
							code: 'TOKEN_REFRESHED'
						});
					}
				},

				// Or force retry based on response body content
				async ({response}) => {
					if (response.status === 200) {
						const data = await response.json();
						if (data.error?.code === 'RATE_LIMIT') {
							// Retry with custom delay from API response
							return ky.retry({
								delay: data.error.retryAfter * 1000,
								code: 'RATE_LIMIT'
							});
						}
					}
				},

				// Or show a notification only on the last retry for 5xx errors
				({options, response, retryCount}) => {
					if (response.status >= 500 && response.status <= 599) {
						if (retryCount === options.retry.limit) {
							showNotification('Request failed after all retries');
						}
					}
				}
			]
		}
	});
	```
	*/
	afterResponse?: readonly AfterResponseHook[] | undefined;
};

/**
Hooks after normalization: every hook type is present as an array.
*/
export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
```
### source/core/constants.ts
```typescript
import type {Expect, Equal} from '@type-challenges/utils';
import {type KyOptionsRegistry, type RequestHttpMethod} from '../types/options.js';

export const supportsRequestStreams = (() => {
	let duplexAccessed = false;
	let hasContentType = false;
	const supportsReadableStream = typeof globalThis.ReadableStream === 'function';
	const supportsRequest = typeof globalThis.Request === 'function';

	if (supportsReadableStream && supportsRequest) {
		try {
			hasContentType = new globalThis.Request('https://empty.invalid', {
				body: new globalThis.ReadableStream(),
				method: 'POST',
				// @ts-expect-error - Types are outdated.
				get duplex() {
					duplexAccessed = true;
					return 'half';
				},
			}).headers.has('Content-Type');
		} catch {
			// Unsupported implementations throw different errors, such as QQBrowser on iOS throwing "unsupported BodyInit type" (see issue #581).
			return false;
		}
	}

	// Bun and Deno support streaming request bodies but never read the `duplex` option, which the check relies on to tell streaming support apart from WebKit, where the `Request` constructor accepts a stream body that `fetch()` then rejects.
	const isBunOrDeno = 'Bun' in globalThis || 'Deno' in globalThis;

	return (duplexAccessed || isBunOrDeno) && !hasContentType;
})();

export const supportsAbortController = typeof globalThis.AbortController === 'function';
export const supportsAbortSignal = typeof globalThis.AbortSignal === 'function' && typeof globalThis.AbortSignal.any === 'function';
export const supportsResponseStreams = typeof globalThis.ReadableStream === 'function';
export const supportsFormData = typeof globalThis.FormData === 'function';

export const requestMethods = ['get', 'post', 'put', 'patch', 'head', 'delete', 'query'] as const;

const validate = <T extends Array<true>>() => undefined as unknown as T;
validate<[
	Expect<Equal<typeof requestMethods[number], RequestHttpMethod>>,
]>();

// Each media type is one Ky can actually parse, so a server that only serves one of them does not answer 406 to a
// request the matching shortcut would have handled.
export const responseTypes = {
	json: 'application/json',
	// `.text()` returns JSON as-is, and `.formData()` parses URL-encoded bodies as well as multipart ones.
	text: 'text/*, application/json',
	formData: 'multipart/form-data, application/x-www-form-urlencoded',
	arrayBuffer: '*/*',
	blob: '*/*',
	// Supported in modern Fetch implementations (for example, browsers and recent Node.js/undici).
	// We still feature-check at runtime before exposing the shortcut.
	bytes: '*/*',
} as const;

// The maximum value of a 32bit int (see issue #117)
export const maxSafeTimeout = 2_147_483_647;

// Size in bytes of a typical form boundary (e.g., '------WebKitFormBoundaryaxpyiPgbbPti10Rw'), used to help estimate upload size
export const usualFormBoundarySize = 40;

/**
Symbol that can be returned by a `beforeRetry` hook to stop retrying without throwing an error.
*/
export const stop = Symbol('stop');

/**
Options for forcing a retry via `ky.retry()`.
*/
export type ForceRetryOptions = {
	/**
	Custom delay in milliseconds before retrying.

	If not provided, uses the default retry delay calculation based on `retry.delay` configuration.

	**Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified.

	Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry.
	*/
	delay?: number | undefined;

	/**
	Error code for the retry.

	This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries.

	@example
	```
	return ky.retry({code: 'RATE_LIMIT'});
	// Resulting error message: 'Forced retry: RATE_LIMIT'
	```
	*/
	code?: string | undefined;

	/**
	Original error that caused the retry.

	This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal.

	@example
	```
	try {
		const data = await response.json();
		validateBusinessLogic(data);
	} catch (error) {
		return ky.retry({
			code: 'VALIDATION_FAILED',
			cause: error // Preserves original error in chain
		});
	}
	```
	*/
	cause?: Error | undefined;

	/**
	Custom request to use for the retry.

	This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed.

	**Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming.

	**Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded.

	@example
	```
	// Fallback to a different endpoint
	return ky.retry({
		request: new Request('https://backup-api.com/endpoint', {
			method: request.method,
			headers: request.headers,
		}),
		code: 'BACKUP_ENDPOINT'
	});

	// Retry with refreshed authentication token
	const data = await response.json();
	return ky.retry({
		request: new Request(request, {
			headers: {
				...Object.fromEntries(request.headers),
				'Authorization': `Bearer ${data.newToken}`
			}
		}),
		code: 'TOKEN_REFRESHED'
	});
	```
	*/
	request?: Request | undefined;
};

/**
Marker returned by `ky.retry()` to signal a forced retry from `afterResponse` hooks.
*/
export class RetryMarker {
	options: ForceRetryOptions | undefined;

	constructor(options?: ForceRetryOptions) {
		this.options = options;
	}
}

/**
Force a retry from an `afterResponse` hook.

This allows you to retry a request based on the response content, even if the response has a successful status code. The retry will respect the `retry.limit` option and skip the `shouldRetry` check. The forced retry is observable in `beforeRetry` hooks, where the error will be a `ForceRetryError`.

@param options - Optional configuration for the retry.

@example
```
import ky, {isForceRetryError} from 'ky';

const api = ky.extend({
	hooks: {
		afterResponse: [
			async ({request, response}) => {
				// Retry based on response body content
				if (response.status === 200) {
					const data = await response.json();

					// Simple retry with default delay
					if (data.error?.code === 'TEMPORARY_ERROR') {
						return ky.retry();
					}

					// Retry with custom delay from API response
					if (data.error?.code === 'RATE_LIMIT') {
						return ky.retry({
							delay: data.error.retryAfter * 1000,
							code: 'RATE_LIMIT'
						});
					}

					// Retry with a modified request (e.g., fallback endpoint)
					if (data.error?.code === 'FALLBACK_TO_BACKUP') {
						return ky.retry({
							request: new Request('https://backup-api.com/endpoint', {
								method: request.method,
								headers: request.headers,
							}),
							code: 'BACKUP_ENDPOINT'
						});
					}

					// Retry with refreshed authentication
					if (data.error?.code === 'TOKEN_REFRESH' && data.newToken) {
						return ky.retry({
							request: new Request(request, {
								headers: {
									...Object.fromEntries(request.headers),
									'Authorization': `Bearer ${data.newToken}`
								}
							}),
							code: 'TOKEN_REFRESHED'
						});
					}

					// Retry with cause to preserve error chain
					try {
						validateResponse(data);
					} catch (error) {
						return ky.retry({
							code: 'VALIDATION_FAILED',
							cause: error
						});
					}
				}
			}
		],
		beforeRetry: [
			({error, retryCount}) => {
				// Observable in beforeRetry hooks
				if (isForceRetryError(error)) {
					console.log(`Forced retry #${retryCount}: ${error.message}`);
					// Example output: "Forced retry #1: Forced retry: RATE_LIMIT"
				}
			}
		]
	}
});

const response = await api.get('https://example.com/api');
```
*/
export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);

export const kyOptionKeys: KyOptionsRegistry = {
	json: true,
	parseJson: true,
	stringifyJson: true,
	searchParams: true,
	baseUrl: true,
	prefix: true,
	retry: true,
	timeout: true,
	totalTimeout: true,
	maxResponseSize: true,
	hooks: true,
	throwHttpErrors: true,
	onDownloadProgress: true,
	onUploadProgress: true,
	fetch: true,
	context: true,
};

// Standard RequestInit options that should NOT be passed separately to fetch()
// because they're already applied to the Request object.
// Note: `dispatcher` and `priority` are NOT included here - they're fetch-only
// options that the Request constructor doesn't accept, so they need to be passed
// separately to fetch().
export const requestOptionsRegistry = {
	method: true,
	headers: true,
	body: true,
	mode: true,
	credentials: true,
	cache: true,
	redirect: true,
	referrer: true,
	referrerPolicy: true,
	integrity: true,
	keepalive: true,
	signal: true,
	window: true,
	duplex: true,
} as const;
```
### source/utils/merge.ts
```typescript
import type {KyHeadersInit, Options} from '../types/options.js';
import type {Hooks, NormalizedHooks} from '../types/hooks.js';
import {supportsAbortSignal} from '../core/constants.js';
import {isObject} from './is.js';

const replaceSymbol: unique symbol = Symbol('replaceOption');

type ReplaceMarked<T> = {
	[replaceSymbol]: true;
	value: T;
};

type ReplaceState<T> = {
	isReplace: boolean;
	value: T;
};

const getReplaceState = <T>(value: T): ReplaceState<T> =>
	isObject(value) && (value as any)[replaceSymbol] === true
		? {
			isReplace: true,
			value: (value as unknown as ReplaceMarked<T>).value,
		}
		: {
			isReplace: false,
			value,
		};

/**
Wraps a value so that `ky.extend()` will replace the parent value instead of merging with it. Works with hooks, headers, search parameters, context, and any other deep-merged option.

By default, `.extend()` deep-merges options with the parent instance: hooks get appended, headers get merged, and search parameters get accumulated. Use `replaceOption` when you want to fully replace a merged property instead.

@example
```
import ky, {replaceOption} from 'ky';

const base = ky.create({
	hooks: {beforeRequest: [addAuth, addTracking]},
});

// Replaces instead of appending
const extended = base.extend({
	hooks: replaceOption({beforeRequest: [onlyThis]}),
});
// hooks.beforeRequest is now [onlyThis], not [addAuth, addTracking, onlyThis]
```
*/
export const replaceOption = <T>(value: T): T => {
	const markedValue: ReplaceMarked<T> = {[replaceSymbol]: true, value};
	return markedValue as unknown as T;
};

export const validateAndMerge = (...sources: Array<Partial<Options> | undefined>): Partial<Options> => {
	for (const source of sources) {
		if ((!isObject(source) || Array.isArray(source)) && source !== undefined) {
			throw new TypeError('The `options` argument must be an object');
		}
	}

	return deepMerge({}, ...sources);
};

export const mergeHeaders = (source1: KyHeadersInit = {}, source2: KyHeadersInit = {}) => {
	// The base source is copied as-is. Only the second source carries deletion markers: `undefined` in a plain object (for example, from `init` hooks or merged options), or the string `'undefined'` in a `Headers` instance.
	const result = new globalThis.Headers(source1 as RequestInit['headers']);

	for (const [key, value] of Object.entries(toHeaderObject(source2))) {
		if (value === undefined) {
			result.delete(key);
		} else {
			result.set(key, value);
		}
	}

	return result;
};

const isPlainObject = (value: unknown): value is Record<string, unknown> => {
	if (!isObject(value) || Array.isArray(value)) {
		return false;
	}

	const prototype = Object.getPrototypeOf(value);
	return prototype === Object.prototype || prototype === null;
};

// Only plain objects and arrays are merged. Class instances (for example, a `Date` in `json` or an undici `Agent` in `dispatcher`) are replaced as a whole, since merging their own properties into a plain object would strip their prototype.
const isMergeable = (value: unknown): value is Record<string, unknown> | unknown[] => isPlainObject(value) || Array.isArray(value);

export const cloneShallow = <T>(value: T): T => {
	if (value instanceof URLSearchParams) {
		const copy = new URLSearchParams(value) as URLSearchParams & {[deletedParametersSymbol]?: Set<string>};
		const deleted = (value as URLSearchParams & {[deletedParametersSymbol]?: Set<string>})[deletedParametersSymbol];
		if (deleted) {
			// Preserve internal deletion markers so init-hook cloning does not resurrect params removed during option merging.
			copy[deletedParametersSymbol] = new Set(deleted);
		}

		return copy as T;
	}

	if (Array.isArray(value)) {
		return [...value] as T;
	}

	if (isPlainObject(value)) {
		const copy = {...value};
		return copy as T;
	}

	return value;
};

// Recursively clone plain objects and arrays so that `init`-hook mutations to nested `json` or `context` values do not leak into the next request. Non-plain values (functions, class instances, `Date`, `URLSearchParams`, …) are kept by reference, matching how option merging treats them as whole values. The `seen` map keeps shared and cyclic references intact.
export const cloneDeep = <T>(value: T, seen: WeakMap<Record<string, unknown> | unknown[], unknown> = new WeakMap()): T => {
	if (!isMergeable(value)) {
		return value;
	}

	const existingCopy = seen.get(value);
	if (existingCopy) {
		return existingCopy as T;
	}

	if (Array.isArray(value)) {
		const copy: unknown[] = [];
		seen.set(value, copy);
		for (const item of value) {
			copy.push(cloneDeep(item, seen));
		}

		return copy as T;
	}

	// Preserve a null prototype, which callers use to avoid inheriting from `Object.prototype`.
	const copy: Record<string, unknown> = Object.getPrototypeOf(value) === null ? Object.create(null) : {};
	seen.set(value, copy);
	for (const key of Reflect.ownKeys(value)) {
		if (!Object.prototype.propertyIsEnumerable.call(value, key)) {
			continue;
		}

		// Define instead of assign so a `__proto__` key becomes an own property rather than changing the prototype.
		Object.defineProperty(copy, key, {
			value: cloneDeep((value as Record<PropertyKey, unknown>)[key], seen),
			writable: true,
			enumerable: true,
			configurable: true,
		});
	}

	return copy as T;
};

// Header names are case-insensitive, so they are normalized to lowercase (like `Headers` does) so that overrides and `undefined` deletions match regardless of how the name was spelled and `init` hooks can rely on lowercase keys.
// An `undefined` value is kept as a deletion marker so it can still remove a header inherited from a `Request` input when the request is created.
const mergeHeaderObjects = (source1: Record<string, unknown>, source2: Record<string, unknown>): Record<string, string | undefined> => {
	const result = new Map<string, string | undefined>();

	for (const [key, value] of [...Object.entries(source1), ...Object.entries(source2)]) {
		result.set(key.toLowerCase(), value as string | undefined);
	}

	return Object.fromEntries(result);
};

// Every header source is merged as a plain object so deletion markers survive no matter how the headers were provided. A `Headers` instance cannot hold `undefined`, so its string `'undefined'` counts as a deletion. A plain object is returned as-is, so callers must not mutate the result.
const toHeaderObject = (source: KyHeadersInit): Record<string, string | undefined> => {
	if (isPlainObject(source)) {
		return source as Record<string, string | undefined>;
	}

	const isHeadersInstance = source instanceof globalThis.Headers;
	const result: Record<string, string | undefined> = {};

	// A header named `__proto__` is dropped here, because assigning it sets the prototype instead of an own property. This is too much of an edge case to be worth supporting.
	for (const [key, value] of new globalThis.Headers(source as RequestInit['headers']).entries()) {
		result[key] = isHeadersInstance && value === 'undefined' ? undefined : value;
	}

	return result;
};

export const mergeHeaderContainers = (source1: KyHeadersInit, source2: KyHeadersInit): Record<string, string | undefined> =>
	mergeHeaderObjects(toHeaderObject(source1), toHeaderObject(source2));

function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K] {
	if (Object.hasOwn(incoming, property) && incoming[property] === undefined) {
		return [];
	}

	// A single hook type can be wrapped in `replaceOption()` to replace only that array instead of the whole `hooks` object.
	const {isReplace, value} = getReplaceState(incoming[property]);
	if (value !== undefined && !Array.isArray(value)) {
		// Merging ignores anything that is neither an array nor an object, so a single function, a string or `null`
		// used to be dropped without a word and the hook simply never ran.
		throw new TypeError(`\`hooks.${property}\` must be an array`);
	}

	if (isReplace) {
		return [...(value ?? [])] as NormalizedHooks[K];
	}

	return deepMerge<NormalizedHooks[K]>(original[property] ?? [], value ?? []);
}

export const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks => {
	// `null` selects the default, the same way option merging treats a nullish value as absent.
	const incomingHooks = incoming ?? {};
	if (!isObject(incomingHooks)) {
		throw new TypeError('The `hooks` option must be an object');
	}

	return {
		init: newHookValue(original, incomingHooks, 'init'),
		beforeRequest: newHookValue(original, incomingHooks, 'beforeRequest'),
		beforeRetry: newHookValue(original, incomingHooks, 'beforeRetry'),
		beforeError: newHookValue(original, incomingHooks, 'beforeError'),
		afterResponse: newHookValue(original, incomingHooks, 'afterResponse'),
	};
};

export const deletedParametersSymbol = Symbol('deletedParameters');

// `replaceOption()` can wrap a value at any depth of a deep-merged option, but a wrapper is only unwrapped where a
// merge actually runs, so a wrapper on a key with no parent value kept its `{value}` envelope. This resolves the
// leftovers in one pass, copying only the branches that actually contained a wrapper so every other value keeps its
// identity. `seen` keeps a cyclic option from recursing forever; a cyclic branch is left as-is.
const resolveReplaceMarkers = (value: unknown, seen: Set<Record<string, unknown> | unknown[]>): unknown => {
	if (!isMergeable(value) || seen.has(value)) {
		return value;
	}

	seen.add(value);
	const resolveItem = (item: unknown) => resolveReplaceMarkers(getReplaceState(item).value, seen);

	if (Array.isArray(value)) {
		let resolved: unknown[] | undefined;
		for (const [index, item] of value.entries()) {
			const replacement = resolveItem(item);
			if (replacement !== item) {
				resolved ??= [...value];
				resolved[index] = replacement;
			}
		}

		return resolved ?? value;
	}

