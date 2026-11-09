## Task
# ky-6e27594 · ky-6e27594 · open
Goal: Type-check `onDownloadProgress` before the request is sent

## Map
- repo map (107 files):
  .editorconfig
  .gitattributes
  .github/ (3 files)
  .gitignore
  .npmrc
  AGENTS.md
  license
  media/ (3 files)
  package.json
  readme.md
  source/ (31 files)
  test-d/ (21 files)
  test/ (38 files)
  tsconfig.dist.json
  tsconfig.json
  tsconfig.test.json

## Code
- sym:42422 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
```typescript
type ExpectedProgressCallbacks = {
	readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>;
	readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>;
};
```
- sym:42325 streamResponse (source/utils/body.ts:197-212)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:42321 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42245 Ky.#wrapRequestWithUploadProgress (source/core/Ky.ts:1553-1559)
```typescript
	#wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request {
		if (!this.#options.onUploadProgress || !supportsRequestStreams || !request.body) {
			return request;
		}

		return streamRequest(request, this.#options.onUploadProgress, originalBody ?? this.#options.body ?? undefined);
	}
```
- sym:42306 NormalizedOptions (source/types/options.ts:551-564)
  interface NormalizedOptions extends Readonly<RequestInit>
- sym:42199 validateCallbackOptions (source/core/Ky.ts:103-114)
  const validateCallbackOptions = (options: Record<string, unknown>): void =>
- sym:42323 copyResponseMetadata (source/utils/body.ts:107-170)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42289 Hooks (source/types/hooks.ts:90-371)
- sym:42326 streamRequest (source/utils/body.ts:215-234)
- sym:42215 Ky.#throwProcessedError (source/core/Ky.ts:806-836)
- sym:42297 Progress (source/types/options.ts:21-36)
- sym:42282 BeforeRequestHook (source/types/hooks.ts:42-42)
- sym:42209 Ky.create (source/core/Ky.ts:268-472)
  static create(input: Input, options: Options): ResponsePromise
- sym:42302 Options (source/types/options.ts:448-520)
- sym:42397 createStreamFetch (test/hooks.ts:100-114)

---
tokens: 681/800
