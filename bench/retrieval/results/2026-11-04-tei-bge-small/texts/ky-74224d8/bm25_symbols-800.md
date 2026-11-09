- Ky.#fetch (source/core/Ky.ts:1404-1468)
```typescript
	async #fetch(): Promise<Response> {
		// A previous attempt can return without consuming its upload. Release that unused branch before replacing the request reference.
		if (this.#originalRequest && this.#originalRequest.body !== this.request.body) {
			this.#cancelBody(this.#originalRequest.body ?? undefined);
		}

		const nonRequestOptions = findUnknownOptions(this.#options);
		this.#retryLimit = normalizeRetryOptions(this.#options.retry).limit;
		// Reattach the managed signal because Node.js can garbage-collect the abort controller used by Request.clone().
		const retryRequest = this.#retryLimit > 0 ? this.#withManagedSignal(this.request.clone()) : undefined;
		const request = this.#wrapRequestWithUploadProgress(this.request, this.#options.body ?? undefined);

		// Cloning is done here to prepare in advance for retries.
		// Skip cloning when retries are disabled - cloning a streaming body calls ReadableStream#tee()
		// which buffers the entire stream in memory, causing excessive memory usage for large uploads.
		this.#originalRequest = request;
		if (retryRequest) {
			this.request = retryRequest;
			this.#requestBodyCanBeCancelled = true;
		}

		try {
			const effectiveTimeout = this.#getEffectiveTimeout();

			// Called unbound so a native `window.fetch` is not invoked with the options object as `this`, which throws "Illegal invocation" in browsers.
			const {fetch} = this.#options;
			const response = effectiveTimeout === undefined
				? await fetch(request, nonRequestOptions)
				: await timeout(request, nonRequestOptions, this.#abortController, {
					timeout: effectiveTimeout,
					fetch,
				});

			if (this.#getRemainingTotalTimeout() === 0) {
				this.#abortController?.abort();
				this.#cancelResponseBody(response);
				throw new TimeoutError(request);
			}

			return this.#setResponseRequest(response, request);
		} catch (error) {
			this.#throwIfAbortedByUser();
			if (this.#getRemainingTotalTimeout() === 0) {
				this.#abortController?.abort();
				throw new TimeoutError(request);
			}

			// The upload progress wrapper errors the request body stream when its callback throws, which the runtime reports as a network failure. Surface the callback error instead.
			const progressCallbackError = getProgressCallbackError(this.#originalRequest?.body ?? undefined);
			if (progressCallbackError !== undefined) {
				if (isError(progressCallbackError)) {
					this.#inFlightCallbackErrors.add(progressCallbackError);
				}

				// eslint-disable-next-line @typescript-eslint/only-throw-error -- The callback can throw any value, and non-Error throws are propagated as-is elsewhere.
				throw progressCallbackError;
			}

			if (isRawNetworkError(error)) {
				throw new NetworkError(request, {cause: error as Error});
			}

			throw error;
		}
	}
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

