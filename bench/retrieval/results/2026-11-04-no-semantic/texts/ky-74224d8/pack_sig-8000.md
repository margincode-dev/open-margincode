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
- sym:42441 Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
  async #calculateRetryDelay(error: unknown)
- sym:42550 getProgressCallbackError (source/utils/body.ts:104-105)
  const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
- sym:42447 Ky.#raceBodyRead (source/core/Ky.ts:913-979)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42551 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42549 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42650 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
  type ExpectedProgressCallbacks = { readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>; readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>; }
- sym:42439 Ky.#validateCallbackOptions (source/core/Ky.ts:662-675)
  #validateCallbackOptions(): void
- sym:42539 RetryOptions (source/types/retry.ts:21-187)
  type RetryOptions = { /** The number of times to retry failed requests. Must be a finite, non-negative integer. @default 2 */ limit?: number | undefined; /** The HTTP methods allowed to retry. @default ['get', 'put', 'head', 'delete', 'options', 'trace', 'query'] */ methods?: readonly HttpMethod[] | undefined; /** The HTTP status codes allowed to retry. `413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`. @default [408, 413, 429, 500, 502, 503, 504] */ statusCodes?: readonly number[] | undefined; /** The retriable HTTP status codes that should respect retry timing headers. These status codes must also be included in `statusCodes`, unless `shouldRetry` returns `true`. @default [413, 429, 503] */ afterStatusCodes?: readonly number[] | undefined; /** If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`. @default Infinity */ maxRetryAfter?: number | undefined; /** The upper limit of the delay per retry in milliseconds, for the delay calculated by `retry.delay`. To clamp the delay, set `backoffLimit` to 1000, for example. A delay from a retry timing header is not subject to this limit; use `maxRetryAfter` to bound it. By default, the delay is calculated in the following way: ``` 0.3 * (2 ** (attemptCount - 1)) * 1000 ``` The delay increases exponentially. @default Infinity */ backoffLimit?: number | undefined; /** A function to calculate the delay in milliseconds between retries given `attemptCount` (starts from 1). @default attemptCount => 0.3 * (2 ** (attemptCount - 1)) * 1000 */ delay?: ((attemptCount: number) => number) | undefined; /** Add random jitter to retry delays to prevent thundering herd problems. When many clients retry simultaneously (e.g., after hitting a rate limit), they can overwhelm the server again. Jitter adds randomness to break this synchronization. Set to `true` to use full jitter, which randomizes the delay between 0 and the computed delay. Alternatively, pass a function to implement custom jitter strategies. Note: Jitter is not applied when the server provides a retry timing header, as the server's explicit timing should be respected. @default undefined (no jitter) @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 5, // Full jitter (randomizes delay between 0 and computed value) jitter: true // Percentage jitter (80-120% of delay) // jitter: delay => delay * (0.8 + Math.random() * 0.4) // Absolute jitter (±100ms) // jitter: delay => delay + (Math.random() * 200 - 100) } }).json(); ``` */ jitter?: boolean | ((delay: number) => number) | undefined; /** Whether to retry when the request times out before a response is returned. Timeouts while reading a response body through Ky shortcut methods are not retried because the response has already been received. @default false @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 3, retryOnTimeout: true } }).json(); ``` */ retryOnTimeout?: boolean | undefined; /** A function to determine whether a retry should be attempted. This function takes precedence over the default retry checks (`retryOnTimeout`, status code checks, etc.) for retriable methods. It is only called after the retry limit and method checks pass. **Note:** This is different from the `beforeRetry` hook: - `shouldRetry`: Controls WHETHER to retry (called before the retry decision is made) - `beforeRetry`: Called AFTER retry is confirmed, allowing you to modify the request Should return: - `true` to force a retry (bypasses `retryOnTimeout`, status code checks, and other validations) - `false` to prevent a retry (no retry will occur) - `undefined` (or nothing) to use the default retry logic (`retryOnTimeout`, status codes, network errors). Unrecognized error types are not retried. Returning `true` overrides retry eligibility, not timing. For HTTP errors with a status in `afterStatusCodes`, retry timing headers and `maxRetryAfter` still apply, without jitter. Otherwise, Ky uses the configured backoff delay. @default undefined @example ``` import ky, {HTTPError} from 'ky'; const json = await ky('https://example.com', { retry: { limit: 3, shouldRetry: ({error, retryCount}) => { // Retry on specific business logic errors from API if (error instanceof HTTPError) { const status = error.response.status; // Retry on 429 (rate limit) but only for first 2 attempts if (status === 429 && retryCount <= 2) { return true; } // Don't retry on 4xx errors except rate limits if (status >= 400 && status < 500) { return false; } } // Use default retry logic for other errors return undefined; } } }).json(); ``` */ shouldRetry?: ((state: ShouldRetryState) => boolean | void | Promise<boolean | void>) | undefined; }
- sym:42534 NormalizedOptions (source/types/options.ts:533-546)
  interface NormalizedOptions extends Readonly<RequestInit>
- sym:42443 Ky.#throwProcessedError (source/core/Ky.ts:805-835)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42436 Ky.create (source/core/Ky.ts:253-457)
  static create(input: Input, options: Options): ResponsePromise
- sym:42586 findUnknownOptions (source/utils/options.ts:5-29)
  const findUnknownOptions = ( options: Record<string, unknown>, ): Record<string, unknown> =>
- sym:42426 validateTimeoutOption (source/core/Ky.ts:87-99)
  const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void =>
- sym:42489 ForceRetryError.constructor (source/errors/ForceRetryError.ts:16-38)
  constructor(options?: ForceRetryOptions)
- sym:42585 normalizeRetryOptions (source/utils/normalize.ts:52-109)
  const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions =>
- sym:42435 Ky (source/core/Ky.ts:252-1556)
  class Ky
- sym:42488 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:42438 Ky.constructor (source/core/Ky.ts:490-658)
  constructor(input: Input, options: Options = {})
- sym:42463 Ky.#fetch (source/core/Ky.ts:1404-1468)
  async #fetch(): Promise<Response>
- sym:42496 NonError (source/errors/NonError.ts:6-28)
  class NonError extends Error
- sym:42497 NonError.constructor (source/errors/NonError.ts:10-27)
  constructor(value: unknown)
- sym:42533 InternalOptions (source/types/options.ts:517-528)
  type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & { headers: Headers; hooks: NormalizedHooks; retry: NormalizedRetryOptions; fetch: NonNullable<Options['fetch']>; prefix: string; timeout: number | false; totalTimeout: number | false; maxResponseSize: number; context: Record<string, unknown>; throwHttpErrors: boolean | ((status: number) => boolean); }
- sym:42513 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:42554 streamRequest (source/utils/body.ts:216-235)
  const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) =>
- sym:42525 Progress (source/types/options.ts:21-36)
  type Progress = { /** A number between `0` and `1` representing the progress percentage. */ percent: number; /** The number of bytes transferred so far. */ transferredBytes: number; /** The total number of bytes to be transferred. This is an estimate and may be `0` for an empty transfer or when the total size cannot be determined. */ totalBytes: number; }
- sym:42473 Ky.#wrapRequestWithUploadProgress (source/core/Ky.ts:1549-1555)
  #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request
- sym:42427 cloneRetryOptions (source/core/Ky.ts:101-122)
  const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number =>
- sym:42553 streamResponse (source/utils/body.ts:198-213)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:42440 Ky.#calculateDelay (source/core/Ky.ts:677-699)
  #calculateDelay(retry: InternalOptions['retry']): number
- sym:42434 validateJsonWithSchema (source/core/Ky.ts:222-250)
  const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> =>
- sym:42444 Ky.#getResponseData (source/core/Ky.ts:837-874)
  async #getResponseData(response: Response): Promise<unknown>
- sym:42571 toHeaderObject (source/utils/merge.ts:169-183)
  const toHeaderObject = (source: KyHeadersInit): Record<string, string | undefined> =>
- sym:42480 retry (source/core/constants.ts:256-256)
  const retry = (options?: ForceRetryOptions) =>
- sym:42483 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
  const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
- sym:42487 calculateRetryTimingDelay (source/core/retry-timing.ts:151-173)
  const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined =>
- sym:42502 TimeoutError (source/errors/TimeoutError.ts:7-15)
  class TimeoutError extends KyError
- sym:42559 isError (source/utils/is.ts:7-8)
  const isError = (value: unknown): value is Error =>
- sym:42582 normalizeRetryMethod (source/utils/normalize.ts:13-14)
  const normalizeRetryMethod = (method: string): string =>
- sym:42592 isHTTPError (source/utils/type-guards.ts:58-60)
  function isHTTPError<T = unknown>(error: unknown): error is HTTPError<T>
- sym:42593 isNetworkError (source/utils/type-guards.ts:80-82)
  function isNetworkError(error: unknown): error is NetworkError
- sym:42594 isTimeoutError (source/utils/type-guards.ts:102-104)
  function isTimeoutError(error: unknown): error is TimeoutError
- sym:42595 isForceRetryError (source/utils/type-guards.ts:129-131)
  function isForceRetryError(error: unknown): error is ForceRetryError

---
tokens: 3426/8000
