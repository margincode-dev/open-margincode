- cloneShallow (source/utils/merge.ts:91-113)
```typescript
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
```
- createInstance (source/index.ts:10-33)
```typescript
const createInstance = (defaults?: Partial<Options>): KyInstance => {
	// eslint-disable-next-line @typescript-eslint/promise-function-async
	const ky: Partial<Mutable<KyInstance>> = (input: Input, options?: Options) => Ky.create(input, validateAndMerge(defaults, options));

	for (const method of requestMethods) {
		// eslint-disable-next-line @typescript-eslint/promise-function-async
		ky[method] = (input: Input, options?: Options) => Ky.create(input, validateAndMerge(defaults, options, {method}));
	}

	ky.create = (newDefaults?: Partial<Options>) => createInstance(validateAndMerge(newDefaults));
	ky.extend = (newDefaults?: Partial<Options> | ((parentDefaults: Partial<Options>) => Partial<Options>)) => {
		if (typeof newDefaults === 'function') {
			// Pass a deep copy so mutations inside the callback cannot leak into this instance's defaults. Deep, because options like `context` are merged shallowly, so a nested value would still point at the parent's object.
			newDefaults = newDefaults(validateAndMerge(cloneDeep(defaults)));
		}

		return createInstance(validateAndMerge(defaults, newDefaults));
	};

	ky.stop = stop;
	ky.retry = retry;

	return ky as KyInstance;
};
```
- RetryMarker (source/core/constants.ts:157-163)
```typescript
export class RetryMarker {
	options: ForceRetryOptions | undefined;

	constructor(options?: ForceRetryOptions) {
		this.options = options;
	}
}
```
- KyInstance (source/types/ky.ts:5-204)
```typescript
export type KyInstance = {
	/**
	Fetch the given `url`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` method added.

	@example
	```
	import ky from 'ky';

	const json = await ky('https://example.com', {json: {foo: true}}).json();

	console.log(json);
	//=> `{data: '🦄'}`
	```
	*/
	<T>(url: Input, options?: Options): ResponsePromise<T>;

	/**
	Fetch the given `url` using the option `{method: 'get'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
	get: <T>(url: Input, options?: Options) => ResponsePromise<T>;

	/**
	Fetch the given `url` using the option `{method: 'post'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
	post: <T>(url: Input, options?: Options) => ResponsePromise<T>;

	/**
	Fetch the given `url` using the option `{method: 'put'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
	put: <T>(url: Input, options?: Options) => ResponsePromise<T>;

	/**
	Fetch the given `url` using the option `{method: 'delete'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
	delete: <T>(url: Input, options?: Options) => ResponsePromise<T>;

	/**
	Fetch the given `url` using the option `{method: 'patch'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
	patch: <T>(url: Input, options?: Options) => ResponsePromise<T>;

	/**
	Fetch the given `url` using the option `{method: 'head'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
	head: (url: Input, options?: Options) => ResponsePromise;

	/**
	Fetch the given `url` using the option `{method: 'query'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
	query: <T>(url: Input, options?: Options) => ResponsePromise<T>;

	/**
	Create a new Ky instance with complete new defaults, without inheriting from any parent instance.

	@returns A new Ky instance.
	*/
	create: (defaultOptions?: Options) => KyInstance;

	/**
	Create a new Ky instance with some defaults overridden with your own.

	In contrast to `ky.create()`, `ky.extend()` inherits defaults from its parent.

	You can pass headers as a `Headers` instance or a plain object. Headers are normalized to a plain object with lowercase names when options are merged, so `init` hooks always see a plain object with lowercase keys. A header removed with `undefined` stays in that object with an `undefined` value.

	You can remove a header with `.extend()` by passing the header with an `undefined` value. Passing `undefined` as a string removes the header only if it comes from a `Headers` instance.

	Similarly, you can remove existing `hooks` entries by extending the hook with an explicit `undefined`.

	By default, `.extend()` deep-merges options: hooks are appended, headers are merged, and search parameters are accumulated. Use `replaceOption` when you want to fully replace a merged property instead.

	When extending an instance, signals are combined. Use `replaceOption(signal)` to replace inherited signals, or `signal: undefined` to remove them.

	You can also refer to parent defaults by providing a function to `.extend()`.

	@example
	```
	import ky from 'ky';

	const api = ky.create({prefix: 'https://example.com/api'});

	const usersApi = api.extend((options) => ({prefix: `${options.prefix}/users`}));

	const response = await usersApi.get('123');
	//=> 'https://example.com/api/users/123'

	const response = await api.get('version');
	//=> 'https://example.com/api/version'
	```

	@returns A new Ky instance.
	*/
	extend: (defaultOptions: Options | ((parentOptions: Options) => Options)) => KyInstance;

	/**
	A `Symbol` that can be returned by a `beforeRetry` hook to stop the retry. This will also short circuit the remaining `beforeRetry` hooks.

	Note: Returning this symbol makes Ky abort and return with an `undefined` response. Be sure to check for a response before accessing any properties on it or use [optional chaining](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining). It is also incompatible with body methods, such as `.json()` or `.text()`, because there is no response to parse. In general, we recommend throwing an error instead of returning this symbol, as that will cause Ky to abort and then throw, which avoids these limitations.

	A valid use-case for `ky.stop` is to prevent retries when making requests for side effects, where the returned data is not important. For example, logging client activity to the server.

	@example
	```
	import ky from 'ky';

	const options = {
		hooks: {
			beforeRetry: [
				async ({request, options, error, retryCount}) => {
					const shouldStopRetry = await ky('https://example.com/api');
					if (shouldStopRetry) {
						return ky.stop;
					}
				}
			]
		}
	};

	// Note that response will be `undefined` in case `ky.stop` is returned.
	const response = await ky.post('https://example.com', options);

	// Using `.text()` or other body methods is not supported.
	const text = await ky('https://example.com', options).text();
	```
	*/
	readonly stop: typeof stop;

	/**
	Force a retry from an `afterResponse` hook.

	This allows you to retry a request based on the response content, even if the response has a successful status code. The retry will respect the `retry.limit` option and skip the `shouldRetry` check. The forced retry is observable in `beforeRetry` hooks, where the error will be a `ForceRetryError`.

	@example
	```
	import ky, {isForceRetryError} from 'ky';

	const api = ky.extend({
		hooks: {
			afterResponse: [
				async ({response}) => {
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
	readonly retry: typeof retry;
};
```
- AfterResponseHook (source/types/hooks.ts:88-88)
```typescript
export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
```
- BeforeRetryHook (source/types/hooks.ts:55-55)
```typescript
export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
```
- retry (source/core/constants.ts:248-248)
```typescript
export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);
```
- Ky.#retryFromError (source/core/Ky.ts:1280-1367)
```typescript
	async #retryFromError(error: unknown): Promise<Response | void> {
		this.#throwIfAbortedByUser();
		this.#returnedResponseFromBeforeRetryHook = false;

		const retryDelay = Math.min(await this.#calculateRetryDelay(error), maxSafeTimeout);
		const delayOptions = {signal: this.#userProvidedAbortSignal};

		const remainingTimeout = this.#getRemainingTotalTimeout();
		if (remainingTimeout !== undefined) {
			if (remainingTimeout <= 0) {
				throw new TimeoutError(this.request);
			}

			// If waiting would consume all remaining budget, time out without starting another request.
			if (retryDelay >= remainingTimeout) {
				await delay(remainingTimeout, delayOptions);
				throw new TimeoutError(this.request);
			}
		}

		// Only use user-provided signal for delay, not our internal abortController
		await delay(retryDelay, delayOptions);

		this.#throwIfTotalTimeoutExhausted();

		// Reset abortController if it was aborted (happens on timeout retry) so hooks and the retried request get a fresh signal
		if (this.#abortController?.signal.aborted) {
			this.#abortController = new globalThis.AbortController();
			this.#options.signal = this.#createManagedSignal();
			this.#assignRequest(this.#withManagedSignal(this.request), this.#requestBodyCanBeCancelled);
		}

		// Apply custom request from forced retry before beforeRetry hooks
		// Ensure the custom request has the correct managed signal for timeouts and user aborts
		if (isForceRetryError(error) && error.customRequest) {
			// Replacement Requests are authoritative by design. Do not rewrite headers here,
			// even for cross-origin retries. Callers using `ky.retry({request})` explicitly
			// opted into the exact Request they constructed.
			this.#assignRequest(this.#withManagedSignal(error.customRequest));
		}

		for (const hook of this.#options.hooks.beforeRetry) {
			let hookResult: Awaited<ReturnType<typeof hook>> | typeof timedOutOperation;
			try {
				// eslint-disable-next-line no-await-in-loop
				hookResult = await this.#raceWithTotalTimeout(async () => hook({
					request: this.request,
					options: this.#getNormalizedOptions(),
					error: error instanceof Error ? error : new NonError(error),
					retryCount: this.#retryCount + 1,
				}), this.#userProvidedAbortSignal);
			} catch (hookError) {
				// Preserve the original request error path (`throw error`) so beforeError hooks can still run.
				if (hookError instanceof Error && hookError !== error) {
					this.#beforeRetryHookErrors.add(hookError);
				}

				throw hookError;
			}

			if (hookResult === timedOutOperation) {
				throw new TimeoutError(this.request);
			}

			if (isRequestInstance(hookResult)) {
				// Same contract as `ky.retry({request})`: a Request returned from `beforeRetry`
				// is used as-is rather than being sanitized or otherwise rewritten by Ky.
				this.#assignRequest(this.#withManagedSignal(hookResult));
				break;
			}

			if (isResponseInstance(hookResult)) {
				this.#returnedResponseFromBeforeRetryHook = true;
				this.#retryCount++;
				return hookResult;
			}

			// If `stop` is returned from the hook, the retry process is stopped
			if (hookResult === stop) {
				return;
			}
		}

		this.#throwIfTotalTimeoutExhausted();

		this.#retryCount++;
		return this.#retry();
	}
```
- Ky.#throwProcessedError (source/core/Ky.ts:782-812)
```typescript
	async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never> {
		// Non-Error throws (e.g., thrown strings) pass through unchanged. `isError` reads the internal brand, so an
		// error from another realm still runs the hooks, the same way the Ky type guards accept branded errors.
		if (!isError(error)) {
			throw error;
		}

		// Errors thrown by beforeRetry hooks must propagate unchanged.
		if (this.#beforeRetryHookErrors.has(error)) {
			throw error;
		}

		let processedError: Error = error;
		for (const hook of this.#options.hooks.beforeError) {
			// `options` intentionally remains the stable normalized Ky options snapshot for the same reason as `HTTPError` above.
			// eslint-disable-next-line no-await-in-loop
			const hookResult: unknown = await hook({
				request,
				options: this.#getNormalizedOptions(),
				error: processedError,
				retryCount: this.#retryCount,
			});

			// Only overwrite if the hook returns a valid Error, including one from another realm.
			if (isError(hookResult)) {
				processedError = hookResult;
			}
		}

		throw processedError;
	}
```
- cloneDeep (source/utils/merge.ts:116-154)
```typescript
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
```
- Ky.#runAfterResponseHooks (source/core/Ky.ts:1204-1270)
```typescript
	async #runAfterResponseHooks(response: Response): Promise<Response> {
		const responseRequest = this.#getResponseRequest(response);
		response = this.#limitResponseSize(response);

		for (const hook of this.#options.hooks.afterResponse) {
			const hookResponse = this.#setResponseRequest(response.clone(), responseRequest);
			this.#decorateResponse(hookResponse);

			let modifiedResponse;
			try {
				// eslint-disable-next-line no-await-in-loop
				modifiedResponse = await this.#raceWithTotalTimeout(async () => hook({
					// Deliberately `this.request`, which `#fetch()` has already replaced with the clone prepared for a
					// retry, so its body is still unread. A hook that forces a retry with `ky.retry({request: new
					// Request(request)})` depends on being able to copy an unconsumed request from here.
					request: this.request,
					options: this.#getNormalizedOptions(),
					response: hookResponse,
					retryCount: this.#retryCount,
				}), this.#userProvidedAbortSignal);

				if (modifiedResponse === timedOutOperation) {
					throw new TimeoutError(this.request);
				}
			} catch (error) {
				// Cancel both responses to prevent memory leaks when hook throws
				if (hookResponse !== response) {
					this.#cancelResponseBody(hookResponse);
				}

				this.#cancelResponseBody(response);
				throw error;
			}

			if (modifiedResponse instanceof RetryMarker) {
				// Cancel both the cloned response passed to the hook and the current response to prevent resource leaks (especially important in Deno/Bun).
				// Do not await cancellation since hooks can clone the response, leaving extra tee branches that keep cancel promises pending per the Streams spec.
				if (hookResponse !== response) {
					this.#cancelResponseBody(hookResponse);
				}

				this.#cancelResponseBody(response);
				throw new ForceRetryError(modifiedResponse.options);
			}

			const nextResponse = isResponseInstance(modifiedResponse)
				? this.#setResponseRequest(modifiedResponse, responseRequest)
				: response;

			// Cancel any response bodies we won't use to prevent memory leaks.
			// Uses fire-and-forget since hooks may have cloned the response, creating tee branches that block cancellation.
			// If the hook wrapped an existing body into a new Response, both Response objects can still point at the same stream.
			if (hookResponse !== response && hookResponse !== nextResponse && hookResponse.body !== nextResponse.body) {
				this.#cancelResponseBody(hookResponse);
			}

			if (response !== nextResponse && response.body !== nextResponse.body) {
				this.#cancelResponseBody(response);
			}

			if (nextResponse !== response) {
				response = this.#limitResponseSize(nextResponse);
			}
		}

		return response;
	}
```
- Hooks (source/types/hooks.ts:90-371)
```typescript
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
```
- Primitive (source/types/common.ts:2-2)
```typescript
export type Primitive = null | undefined | string | number | boolean | symbol | bigint;
```
- ReplaceMarked (source/utils/merge.ts:8-11)
```typescript
type ReplaceMarked<T> = {
	[replaceSymbol]: true;
	value: T;
};
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
- replaceOption (source/utils/merge.ts:49-52)
```typescript
export const replaceOption = <T>(value: T): T => {
	const markedValue: ReplaceMarked<T> = {[replaceSymbol]: true, value};
	return markedValue as unknown as T;
};
```
- Ky (source/core/Ky.ts:252-1527)
```typescript
export class Ky {
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
