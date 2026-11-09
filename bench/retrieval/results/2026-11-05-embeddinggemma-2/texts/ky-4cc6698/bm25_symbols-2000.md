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

