## Task
# ky-74224d8 · ky-74224d8 · open
Goal: Make a non-Error progress-callback throw non-retriable too

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
- sym:42550 getProgressCallbackError (source/utils/body.ts:104-105)
  const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
- sym:42447 Ky.#raceBodyRead (source/core/Ky.ts:913-979)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42549 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42650 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
  type ExpectedProgressCallbacks = { readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>; readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>; }
- sym:42441 Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
  async #calculateRetryDelay(error: unknown)
- sym:42443 Ky.#throwProcessedError (source/core/Ky.ts:805-835)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42489 ForceRetryError.constructor (source/errors/ForceRetryError.ts:16-38)
  constructor(options?: ForceRetryOptions)
- sym:42551 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42488 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
- sym:42513 BeforeErrorState (source/types/hooks.ts:57-70)
- sym:42439 Ky.#validateCallbackOptions (source/core/Ky.ts:662-675)
  #validateCallbackOptions(): void
- sym:42539 RetryOptions (source/types/retry.ts:21-187)
- sym:42463 Ky.#fetch (source/core/Ky.ts:1404-1468)
- sym:42494 NetworkError (source/errors/NetworkError.ts:11-19)
  class NetworkError extends KyError
- sym:42502 TimeoutError (source/errors/TimeoutError.ts:7-15)
- sym:42557 isRawNetworkError (source/utils/is-network-error.ts:15-46)

---
tokens: 684/800
