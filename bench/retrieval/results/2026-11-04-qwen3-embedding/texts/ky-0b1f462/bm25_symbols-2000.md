- streamResponse (source/utils/body.ts:198-211)
```typescript
export const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) => {
	// See `limitResponseSize`: there is nothing left to stream and nothing to report once a hook consumed the body.
	if (!response.body || response.bodyUsed || response.body.locked || nullBodyStatuses.has(response.status)) {
		return response;
	}

	// `content-length` counts encoded bytes on the wire, while the progress stream counts the bytes after decompression. Using it for a content-coded response would report a total below what actually arrives, so every event would sit at ~100%. The total is then unknown, which `Progress` already models with `0`. `identity` is a registered no-op coding (RFC 9110 §8.4.2), so it leaves the length valid.
	const contentEncoding = response.headers.get('content-encoding')?.toLowerCase();
	const isContentCoded = Boolean(contentEncoding) && contentEncoding !== 'identity';
	const totalBytes = isContentCoded ? 0 : Math.max(0, Number(response.headers.get('content-length')) || 0);
	const body = withProgress(response.body, totalBytes, onDownloadProgress);

	return copyResponseMetadata(new Response(body, response), response);
};
```
- withProgress (source/utils/body.ts:53-99)
```typescript
const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> => {
	let previousChunk: Uint8Array | undefined;
	let transferredBytes = 0;
	let progressCallbackError: unknown;
	const report = (progress: Progress, chunk: Uint8Array) => {
		try {
			onProgress?.(progress, chunk);
		} catch (error) {
			// Remember the error so Ky can surface it instead of the network error the runtime reports for the failed stream.
			progressCallbackError = error;
			throw error;
		}
	};

	// The transformer only runs after construction, so its error accessor can be registered immediately afterwards.
	const progressStream: ReadableStream<Uint8Array> = stream.pipeThrough(new TransformStream<Uint8Array, Uint8Array>({
		transform(currentChunk, controller) {
			controller.enqueue(currentChunk);

			if (previousChunk) {
				transferredBytes += previousChunk.byteLength;

				let percent = totalBytes === 0 ? 0 : transferredBytes / totalBytes;
				// Avoid reporting 100% progress before the stream is actually finished (in case totalBytes is inaccurate)
				if (percent >= 1) {
					// Epsilon is used here to get as close as possible to 100% without reaching it.
					// If we were to use 0.99 here, percent could potentially go backwards.
					percent = 1 - Number.EPSILON;
				}

				// An unknown total stays `0`, matching the `percent` computed from the same estimate and the documented progress shape.
				const reportedTotalBytes = totalBytes === 0 ? 0 : Math.max(totalBytes, transferredBytes);
				report({percent, totalBytes: reportedTotalBytes, transferredBytes}, previousChunk);
			}

			previousChunk = currentChunk;
		},
		flush() {
			const finalChunk = previousChunk ?? new Uint8Array();
			transferredBytes += finalChunk.byteLength;
			report({percent: 1, totalBytes: transferredBytes, transferredBytes}, finalChunk);
		},
	}));
	progressCallbackErrors.set(progressStream, () => progressCallbackError);

	return progressStream;
};
```
- getProgressCallbackError (source/utils/body.ts:104-105)
```typescript
export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
	stream ? progressCallbackErrors.get(stream)?.() : undefined;
```
- Ky.#raceBodyRead (source/core/Ky.ts:883-949)
```typescript
	async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
		const failedRequest = this.#getResponseRequest(response);
		let timeoutMs: number | undefined;
		try {
			timeoutMs = this.#getEffectiveTimeout();
		} catch (error: unknown) {
			await this.#throwProcessedError(error, failedRequest);
		}

		const bodyPromise = createBodyPromise();
		const timeoutPromise = timeoutMs === undefined
			? undefined
			: new Promise<typeof timedOutResponseData>(resolve => {
				const timeoutId = setTimeout(() => {
					resolve(timedOutResponseData);
				}, timeoutMs);
				void bodyPromise.finally(() => {
					clearTimeout(timeoutId);
				}).catch(() => undefined);
			});

		let result: unknown;
		try {
			result = timeoutPromise === undefined
				? await bodyPromise
				: await Promise.race([bodyPromise, timeoutPromise]);
		} catch (error: unknown) {
			if (this.#userProvidedAbortSignal?.aborted) {
				await this.#throwProcessedError(this.#userProvidedAbortSignal.reason, failedRequest);
			}

			if (this.#getRemainingTotalTimeout() !== 0) {
				// A throwing progress callback errors the response stream, which a browser may report as a raw `TypeError` that would otherwise be mistaken for a dropped connection. Surface the callback error instead.
				const progressCallbackError = getProgressCallbackError(response.body ?? undefined);
				if (progressCallbackError !== undefined) {
					await this.#throwProcessedError(progressCallbackError, failedRequest);
				}

				// A connection dropped while streaming the body surfaces as a raw runtime `TypeError`. Wrap it like fetch-phase network errors so it is recognizable and runs `beforeError` hooks.
				// This only happens on the awaited path, so a body that fails after the timeout already won does not run the hooks again.
				if (isRawNetworkError(error)) {
					await this.#throwProcessedError(new NetworkError(failedRequest, {cause: error as Error}), failedRequest);
				}

				await this.#throwProcessedError(error, failedRequest);
			}

			result = timedOutResponseData;
		}

		// A cancellation is part of the request lifecycle, so it must not resolve as a successful body read just
		// because the bytes happened to arrive first. `.json()` already gets this check for free through
		// `#raceWithTotalTimeout()`, which is why the shortcuts used to disagree here.
		if (this.#userProvidedAbortSignal?.aborted) {
			await this.#throwProcessedError(this.#userProvidedAbortSignal.reason, failedRequest);
		}

		if (result === timedOutResponseData || this.#getRemainingTotalTimeout() === 0) {
			// The stream is locked by the native body method's own reader by this point, so
			// `response.body.cancel()` would reject as "already locked". Aborting the request's
			// signal is what actually interrupts the underlying network read.
			this.#abortController?.abort();
			await this.#throwProcessedError(new TimeoutError(failedRequest), failedRequest);
		}

		return result;
	}
```
- KyOptions (source/types/options.ts:45-430)
```typescript
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
