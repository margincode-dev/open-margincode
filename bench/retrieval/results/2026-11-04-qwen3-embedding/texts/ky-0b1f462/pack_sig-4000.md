## Task
# ky-0b1f462 · ky-0b1f462 · open
Goal: Progress from a declared content-length, and a throwing callback is never retried
Workspace: /root/bench/ws/rebf815-ky-0b1f462/ky-0b1f462
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
- sym:43450 getProgressCallbackError (source/utils/body.ts:104-105)
  const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
- sym:43453 streamResponse (source/utils/body.ts:198-211)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:43350 Ky.#raceBodyRead (source/core/Ky.ts:883-949)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:43449 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:43454 streamRequest (source/utils/body.ts:214-232)
  const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) =>
- sym:43425 Progress (source/types/options.ts:21-36)
  type Progress = { /** A number between `0` and `1` representing the progress percentage. */ percent: number; /** The number of bytes transferred so far. */ transferredBytes: number; /** The total number of bytes to be transferred. This is an estimate and may be `0` for an empty transfer or when the total size cannot be determined. */ totalBytes: number; }
- sym:43451 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43365 Ky.#fetch (source/core/Ky.ts:1368-1428)
  async #fetch(): Promise<Response>
- sym:43448 getBodySize (source/utils/body.ts:14-51)
  const getBodySize = (body?: BodyInit | null): number =>
- sym:43344 Ky.#calculateRetryDelay (source/core/Ky.ts:675-751)
  async #calculateRetryDelay(error: unknown)
- sym:43340 Ky.create (source/core/Ky.ts:253-452)
  static create(input: Input, options: Options): ResponsePromise
- sym:43438 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:43339 Ky (source/core/Ky.ts:252-1516)
  class Ky
- sym:43548 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
  type ExpectedProgressCallbacks = { readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>; readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>; }
- sym:43347 Ky.#getResponseData (source/core/Ky.ts:807-844)
  async #getResponseData(response: Response): Promise<unknown>
- sym:43363 Ky.#retryFromError (source/core/Ky.ts:1273-1360)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:43346 Ky.#throwProcessedError (source/core/Ky.ts:775-805)
  async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
- sym:43427 KyOptions (source/types/options.ts:45-430)
- sym:43477 deepMergeInternal (source/utils/merge.ts:320-455)
  const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T =>
- sym:43366 Ky.#throwIfAbortedByUser (source/core/Ky.ts:1431-1433)
  #throwIfAbortedByUser(): void
- sym:43407 LiteralUnion (source/types/common.ts:4-6)
  type LiteralUnion<LiteralType extends BaseType, BaseType extends Primitive> = | LiteralType | (BaseType & {_?: never})
- sym:43430 Options (source/types/options.ts:446-500)
  interface Options extends KyOptions, RequestOptions
- sym:43530 ReceivedBody (test/request-body-serialization.ts:6-10)
  type ReceivedBody = { bytes: number[]; contentType?: string; contentLength: string; }
- sym:43539 createAbortDuringBodyFetch (test/retry.ts:4019-4038)
  const createAbortDuringBodyFetch = (): typeof fetch =>
- sym:43531 createBodyServer (test/request-body-serialization.ts:12-22)
  async function createBodyServer(t: ExecutionContext)
- sym:43541 createUploadProgressTestServer (test/stream.ts:1185-1195)
  const createUploadProgressTestServer = async (t: ExecutionContext) =>
- sym:43532 assertBytes (test/request-body-serialization.ts:24-27)
  function assertBytes(t: ExecutionContext, received: ReceivedBody, expected: Uint8Array)
- sym:43540 createAbortDuringFetch (test/retry.ts:4040-4056)
  const createAbortDuringFetch = (onRequest?: () => void): typeof fetch =>

---
tokens: 1682/4000
