- ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
```typescript
type ExpectedProgressCallbacks = {
	readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>;
	readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>;
};
```
- NormalizedOptions (source/types/options.ts:551-564)
```typescript
export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
	// Extended from `RequestInit`, but ensured to be set (not optional).
	readonly method: NonNullable<RequestInit['method']>;
	readonly credentials?: NonNullable<RequestInit['credentials']>;
	readonly headers: Headers;

	// Extended from custom `KyOptions`, but ensured to be set (not optional).
	readonly retry: NormalizedRetryOptions;
	readonly baseUrl?: Options['baseUrl'];
	readonly prefix: string;
	readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>;
	readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>;
	readonly context: Record<string, unknown>;
}
```
- Ky.create (source/core/Ky.ts:268-472)
```typescript
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
			if (beforeRequestResponse !== undefined) {
				ky.#retryLimit = normalizeRetryOptions(ky.#options.retry).limit;
			}

			let response = beforeRequestResponse ?? await ky.#retry();
			let responseFromHook = beforeRequestResponse !== undefined
				|| ky.#consumeReturnedResponseFromBeforeRetryHook();

			for (;;) {
				// A nullish value means a hook stopped the flow, or a custom `fetch` resolved with nothing. The caller
				// sees it normalized to `undefined` either way.
				// Non-native Responses still continue through Ky if they pass `isResponseInstance()`.
				if (response === undefined || response === null) {
					return response;
				}

				if (isResponseInstance(response)) {
					try {
						// eslint-disable-next-line no-await-in-loop
						response = await ky.#runAfterResponseHooks(response);
					} catch (error) {
						if (!isForceRetryError(error)) {
							throw error;
						}

						// eslint-disable-next-line no-await-in-loop
						const retriedResponse: Response | void = await ky.#retryFromError(error);
						if (retriedResponse === undefined) {
							return retriedResponse;
						}

						response = retriedResponse;
						responseFromHook = ky.#consumeReturnedResponseFromBeforeRetryHook();
						continue;
					}
				}

				const currentResponse: Response = response;

				// Opaque responses (`response.type === 'opaque'`) from `no-cors` requests always have `status: 0` and `ok: false`, but this is not a failure - the actual status is hidden by the browser.
				if (!currentResponse.ok && currentResponse.type !== 'opaque' && (
