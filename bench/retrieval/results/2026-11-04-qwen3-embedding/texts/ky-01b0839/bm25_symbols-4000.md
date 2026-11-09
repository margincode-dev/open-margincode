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
