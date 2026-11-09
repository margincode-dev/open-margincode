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
