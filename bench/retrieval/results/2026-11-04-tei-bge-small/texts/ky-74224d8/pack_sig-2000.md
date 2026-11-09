## Task
# ky-74224d8 · ky-74224d8 · open
Goal: Make a non-Error progress-callback throw non-retriable too
Workspace: /root/bench/av_tei/ws/rec09fc-ky-74224d8/ky-74224d8
Changed (0): nothing yet

## Map
- repo map (106 files):
  .editorconfig
  .gitattributes
  .github/ (3 files)
    funding.yml
    security.md
    workflows/ (1 files)
  .gitignore
  .npmrc
  AGENTS.md
  license
  media/ (3 files)
    logo.ai
    logo.png
    logo.svg
  package.json
  readme.md
  source/ (31 files)
    core/ (3 files)
    errors/ (8 files)
    index.ts
    types/ (9 files)
    utils/ (10 files)
  test-d/ (20 files)
    error-names.ts
    fetch-options.ts
    fetch-request.ts
    forced-retry.ts
    headers.ts
    hooks.ts
    init-search-params.ts
    json-undefined-schema.ts
    method-options.ts
    network-error-cause.ts
    normalized-options.ts
    request-clone.ts
    request-options.ts
    response-clone.ts
    response-promise.ts
    response-size.ts
    retry.ts
    search-params.ts
    timeout-options.ts
    url-options.ts
  test/ (38 files)
    base-url.ts
    body-size.ts
    browser.ts
    bytes.ts
    context.ts
    fetch.ts
    formdata-searchparams.ts
    headers.ts
    helpers/ (6 files)
    hooks.ts
    http-error.ts
    http-metadata.ts
    http-redirects.ts
    http-response-bodies.ts
    http-semantics.ts
    main.ts
    memory-leak.ts
    methods.ts
    network-error.ts
    normalize.ts
    prefix.ts
    request-body-serialization.ts
    request-options.ts
    request-stream-support-runtimes.ts
    request-stream-support.ts
    response-consumption.ts
    response-size.ts
    retry-timing.ts
    retry.ts
    stream.ts
    type-guards.ts
    validation.ts
    with-page.ts
  tsconfig.dist.json
  tsconfig.json
  tsconfig.test.json

## Code
- sym:42550 getProgressCallbackError (source/utils/body.ts:104-105)
  const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
- sym:42447 Ky.#raceBodyRead (source/core/Ky.ts:913-979)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42439 Ky.#validateCallbackOptions (source/core/Ky.ts:662-675)
  #validateCallbackOptions(): void
- sym:42441 Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
  async #calculateRetryDelay(error: unknown)
- sym:42488 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:42551 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42549 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42489 ForceRetryError.constructor (source/errors/ForceRetryError.ts:16-38)
  constructor(options?: ForceRetryOptions)
- sym:42650 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
  type ExpectedProgressCallbacks = { readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>; readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>; }
- sym:42461 Ky.#retryFromError (source/core/Ky.ts:1309-1396)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:42539 RetryOptions (source/types/retry.ts:21-187)
- sym:42463 Ky.#fetch (source/core/Ky.ts:1404-1468)
  async #fetch(): Promise<Response>
- sym:42494 NetworkError (source/errors/NetworkError.ts:11-19)
  class NetworkError extends KyError
- sym:42502 TimeoutError (source/errors/TimeoutError.ts:7-15)
  class TimeoutError extends KyError
- sym:42557 isRawNetworkError (source/utils/is-network-error.ts:15-46)
  function isRawNetworkError(error: unknown): error is TypeError
- sym:42642 createAbortDuringFetch (test/retry.ts:4040-4056)
  const createAbortDuringFetch = (onRequest?: () => void): typeof fetch =>
- sym:42641 createAbortDuringBodyFetch (test/retry.ts:4019-4038)
  const createAbortDuringBodyFetch = (): typeof fetch =>

---
tokens: 1151/2000
