## Task
# ky-74224d8 · ky-74224d8-2 · open
Goal: Make a non-Error progress-callback throw non-retriable too
Workspace: /root/bench/ws/rebf815-ky-74224d8/ky-74224d8-2
Changed (0): nothing yet

## Map
- repo map (106 files):
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
  test-d/ (20 files)
  test/ (38 files)
  tsconfig.dist.json
  tsconfig.json
  tsconfig.test.json

## Code
- sym:42441 Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
  async #calculateRetryDelay(error: unknown)
- sym:42550 getProgressCallbackError (source/utils/body.ts:104-105)
```typescript
export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
	stream ? progressCallbackErrors.get(stream)?.() : undefined;
```
- sym:42447 Ky.#raceBodyRead (source/core/Ky.ts:913-979)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42551 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42549 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42650 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
```typescript
type ExpectedProgressCallbacks = {
	readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>;
	readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>;
};
```
- sym:42439 Ky.#validateCallbackOptions (source/core/Ky.ts:662-675)
  #validateCallbackOptions(): void
- sym:42539 RetryOptions (source/types/retry.ts:21-187)
- sym:42463 Ky.#fetch (source/core/Ky.ts:1404-1468)
  async #fetch(): Promise<Response>
- sym:42480 retry (source/core/constants.ts:256-256)
  const retry = (options?: ForceRetryOptions) =>
- sym:42483 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
  const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
- sym:42487 calculateRetryTimingDelay (source/core/retry-timing.ts:151-173)
  const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined =>

---
tokens: 681/800
