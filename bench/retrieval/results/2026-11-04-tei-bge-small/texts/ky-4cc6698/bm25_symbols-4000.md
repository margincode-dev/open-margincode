- Ky.#throwProcessedError (source/core/Ky.ts:805-835)
```typescript
	async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never> {
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
- Ky.#retryFromError (source/core/Ky.ts:1332-1419)
```typescript
	async #retryFromError(error: unknown): Promise<Response | void> {
		this.#throwIfAbortedByUser();
		this.#returnedResponseFromBeforeRetryHook = false;

		const retryDelay = Math.min(await this.#calculateRetryDelay(error), maxSafeTimeout);
		const delayOptions = {signal: this.#userProvidedAbortSignal};

		const remainingTimeout = this.#getRemainingTotalTimeout();
		if (remainingTimeout !== undefined) {
			if (remainingTimeout <= 0) {
				throw new TimeoutError(this.#sentRequest);
			}

			// If waiting would consume all remaining budget, time out without starting another request.
			if (retryDelay >= remainingTimeout) {
				await delay(remainingTimeout, delayOptions);
				throw new TimeoutError(this.#sentRequest);
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
				throw new TimeoutError(this.#sentRequest);
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
- ForceRetryOptions (source/core/constants.ts:73-152)
```typescript
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
```
- mergeHooks (source/utils/merge.ts:217-231)
```typescript
export const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks => {
	// `null` selects the default, the same way option merging treats a nullish value as absent.
	const incomingHooks = incoming ?? {};
	if (!isObject(incomingHooks) || Array.isArray(incomingHooks)) {
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
```
- Ky.#calculateRetryDelay (source/core/Ky.ts:694-774)
```typescript
	async #calculateRetryDelay(error: unknown) {
		if (this.#inFlightCallbackErrors.has(error)) {
			throw error;
		}

		const retry = normalizeRetryOptions(this.#options.retry);
		if (this.#retryCount >= Math.min(retry.limit, this.#retryLimit)) {
			throw error;
		}

		// Wrap non-Error throws to ensure consistent error handling
		const errorObject = error instanceof Error ? error : new NonError(error);

		// Handle forced retry from afterResponse hook - skip method check and shouldRetry
		if (isForceRetryError(errorObject)) {
			return errorObject.customDelay ?? this.#calculateDelay(retry);
		}

		// Check if method is retriable for non-forced retries
		if (!retry.methods.includes(normalizeRetryMethod(this.request.method))) {
			throw error;
		}

		let shouldRetryOverride = false;
		const {shouldRetry} = retry;
		if (shouldRetry !== undefined) {
			const result = await this.#raceWithTotalTimeout(async () => shouldRetry({error: errorObject, retryCount: this.#retryCount + 1}), this.#userProvidedAbortSignal);
			this.#throwIfAbortedByUser();
			if (result === timedOutOperation) {
				throw new TimeoutError(this.#sentRequest);
			}

			// Only exact booleans override the default retry checks.
			if (result === false) {
				throw error;
			}

			shouldRetryOverride = result === true;
		}

		// Default timeout behavior
		if (isTimeoutError(error)) {
			if (!shouldRetryOverride && !retry.retryOnTimeout) {
				throw error;
			}

			return this.#calculateDelay(retry);
		}

		// The `isHTTPError()` brand only checks `name`, so a cross-realm error may not carry a `response`.
		if (isHTTPError(error) && error.response) {
			if (!shouldRetryOverride && !retry.statusCodes.includes(error.response.status)) {
				throw error;
			}

			const retryTimingHeader = getRetryTimingHeader(error.response.headers);
			if (retryTimingHeader && retry.afterStatusCodes.includes(error.response.status)) {
				const after = calculateRetryTimingDelay(retryTimingHeader);
				if (after === undefined) {
					// Malformed retry timing headers should not disable retries; they only lose their server-provided timing.
					return this.#calculateDelay(retry);
				}

				// Don't apply jitter when server provides explicit retry timing
				return Math.min(retry.maxRetryAfter, after);
			}

			if (!shouldRetryOverride && error.response.status === 413) {
				throw error;
			}

			return this.#calculateDelay(retry);
		}

		// Only retry known retriable error types. Unknown errors (e.g., programming bugs) are not retried.
		if (!shouldRetryOverride && !isNetworkError(error)) {
			throw error;
		}

		return this.#calculateDelay(retry);
	}
```
- RetryOptions (source/types/retry.ts:21-187)
```typescript
export type RetryOptions = {
	/**
	The number of times to retry failed requests.
	Must be a finite, non-negative integer.

	@default 2
	*/
	limit?: number | undefined;

	/**
	The HTTP methods allowed to retry.

	@default ['get', 'put', 'head', 'delete', 'options', 'trace', 'query']
	*/
	methods?: readonly HttpMethod[] | undefined;

	/**
	The HTTP status codes allowed to retry.

	`413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`.

	@default [408, 413, 429, 500, 502, 503, 504]
	*/
	statusCodes?: readonly number[] | undefined;

	/**
	The retriable HTTP status codes that should respect retry timing headers. These status codes must also be included in `statusCodes`, unless `shouldRetry` returns `true`.

	@default [413, 429, 503]
	*/
	afterStatusCodes?: readonly number[] | undefined;

	/**
	If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`.

	@default Infinity
	*/
	maxRetryAfter?: number | undefined;

	/**
	The upper limit of the delay per retry in milliseconds, for the delay calculated by `retry.delay`.
	To clamp the delay, set `backoffLimit` to 1000, for example.
	A delay from a retry timing header is not subject to this limit; use `maxRetryAfter` to bound it.

	By default, the delay is calculated in the following way:

	```
	0.3 * (2 ** (attemptCount - 1)) * 1000
	```

	The delay increases exponentially.

	@default Infinity
	*/
	backoffLimit?: number | undefined;

	/**
	A function to calculate the delay in milliseconds between retries given `attemptCount` (starts from 1).

	@default attemptCount => 0.3 * (2 ** (attemptCount - 1)) * 1000
	*/
	delay?: ((attemptCount: number) => number) | undefined;

	/**
	Add random jitter to retry delays to prevent thundering herd problems.

	When many clients retry simultaneously (e.g., after hitting a rate limit), they can overwhelm the server again. Jitter adds randomness to break this synchronization.

	Set to `true` to use full jitter, which randomizes the delay between 0 and the computed delay.

	Alternatively, pass a function to implement custom jitter strategies.

	Note: Jitter is not applied when the server provides a retry timing header, as the server's explicit timing should be respected.

	@default undefined (no jitter)

	@example
	```
	import ky from 'ky';

	const json = await ky('https://example.com', {
		retry: {
			limit: 5,

			// Full jitter (randomizes delay between 0 and computed value)
			jitter: true

			// Percentage jitter (80-120% of delay)
			// jitter: delay => delay * (0.8 + Math.random() * 0.4)

			// Absolute jitter (±100ms)
			// jitter: delay => delay + (Math.random() * 200 - 100)
		}
	}).json();
	```
	*/
	jitter?: boolean | ((delay: number) => number) | undefined;

	/**
	Whether to retry when the request times out before a response is returned. Timeouts while reading a response body through Ky shortcut methods are not retried because the response has already been received.

	@default false

	@example
	```
	import ky from 'ky';

	const json = await ky('https://example.com', {
		retry: {
			limit: 3,
			retryOnTimeout: true
		}
	}).json();
	```
	*/
	retryOnTimeout?: boolean | undefined;

	/**
	A function to determine whether a retry should be attempted.

	This function takes precedence over the default retry checks (`retryOnTimeout`, status code checks, etc.) for retriable methods. It is only called after the retry limit and method checks pass.

	**Note:** This is different from the `beforeRetry` hook:
	- `shouldRetry`: Controls WHETHER to retry (called before the retry decision is made)
	- `beforeRetry`: Called AFTER retry is confirmed, allowing you to modify the request

	Should return:
	- `true` to force a retry (bypasses `retryOnTimeout`, status code checks, and other validations)
	- `false` to prevent a retry (no retry will occur)
	- `undefined` (or nothing) to use the default retry logic (`retryOnTimeout`, status codes, network errors). Unrecognized error types are not retried.

	Returning `true` overrides retry eligibility, not timing. For HTTP errors with a status in `afterStatusCodes`, retry timing headers and `maxRetryAfter` still apply, without jitter. Otherwise, Ky uses the configured backoff delay.

	@default undefined

	@example
	```
	import ky, {HTTPError} from 'ky';

	const json = await ky('https://example.com', {
		retry: {
			limit: 3,
			shouldRetry: ({error, retryCount}) => {
				// Retry on specific business logic errors from API
				if (error instanceof HTTPError) {
					const status = error.response.status;

					// Retry on 429 (rate limit) but only for first 2 attempts
					if (status === 429 && retryCount <= 2) {
						return true;
					}

					// Don't retry on 4xx errors except rate limits
					if (status >= 400 && status < 500) {
						return false;
					}
				}

				// Use default retry logic for other errors
				return undefined;
			}
		}
	}).json();
	```
	*/
	shouldRetry?: ((state: ShouldRetryState) => boolean | void | Promise<boolean | void>) | undefined;
};
```
- Ky.create (source/core/Ky.ts:268-464)
```typescript
	static create(input: Input, options: Options): ResponsePromise {
		const initHooks = options.hooks?.init ?? [];
		const initHookOptions = initHooks.length > 0 ? cloneInitHookOptions(options) : options;

		for (const hook of initHooks) {
			hook(initHookOptions as InitOptions);
		}

		const ky = new Ky(input, initHookOptions);

		const function_ = async (): Promise<Response | void> => {
