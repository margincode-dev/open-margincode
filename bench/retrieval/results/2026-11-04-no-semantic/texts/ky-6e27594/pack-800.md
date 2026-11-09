## Task
# ky-6e27594 · ky-6e27594-2 · open
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
- sym:42306 NormalizedOptions (source/types/options.ts:551-564)
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
- sym:42321 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42323 copyResponseMetadata (source/utils/body.ts:107-170)
- sym:42289 Hooks (source/types/hooks.ts:90-371)
- sym:42215 Ky.#throwProcessedError (source/core/Ky.ts:806-836)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42282 BeforeRequestHook (source/types/hooks.ts:42-42)
- sym:42209 Ky.create (source/core/Ky.ts:268-472)
  static create(input: Input, options: Options): ResponsePromise
- sym:42297 Progress (source/types/options.ts:21-36)
- sym:42302 Options (source/types/options.ts:448-520)
- sym:42326 streamRequest (source/utils/body.ts:215-234)

---
tokens: 685/800
