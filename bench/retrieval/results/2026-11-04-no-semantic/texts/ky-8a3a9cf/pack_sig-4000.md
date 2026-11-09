## Task
# ky-8a3a9cf · ky-8a3a9cf-2 · open
Goal: Advertise the shortcut media type on the attempt that produced the response
Workspace: /root/bench/ws/rebf815-ky-8a3a9cf/ky-8a3a9cf-2
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
- sym:42959 ResponsePromise (source/types/ResponsePromise.ts:7-69)
  type ResponsePromise<T = unknown> = { arrayBuffer: () => Promise<ArrayBuffer>; blob: () => Promise<Blob>; formData: () => Promise<FormData>; /** Get the response body as raw bytes. Note: This shortcut is only available when the runtime supports `Response.prototype.bytes()`. */ bytes: () => Promise<Uint8Array<ArrayBuffer>>; // TODO: Use `json<T extends JSONValue>(): Promise<T>;` when it's fixed in TS. // See https://github.com/microsoft/TypeScript/issues/15300 and https://github.com/sindresorhus/ky/pull/80 json: { /** Get the response body as JSON. @example ``` import ky from 'ky'; const json = await ky(…).json(); ``` @example ``` import ky from 'ky'; interface Result { value: number; } const result1 = await ky(…).json<Result>(); // or const result2 = await ky<Result>(…).json(); ``` */ <JsonType = T>(schema?: undefined): Promise<JsonType>; /** Get the response body as JSON and validate it with a Standard Schema. Use a Standard Schema compatible validator (for example, Zod 3.24+). Throws a `SchemaValidationError` when validation fails. @example ``` import ky from 'ky'; import {z} from 'zod'; const userSchema = z.object({name: z.string()}); const user = await ky('/api/user').json(userSchema); ``` */ <Schema extends StandardSchemaV1>(schema: Schema): Promise<StandardSchemaV1InferOutput<Schema>>; }; text: () => Promise<string>; } & Promise<KyResponse<T>>
- sym:42899 Ky.#getResponseData (source/core/Ky.ts:833-870)
  async #getResponseData(response: Response): Promise<unknown>
- sym:42981 KyOptions (source/types/options.ts:45-432)
- sym:42967 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:43103 ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
  type ExpectedNormalizedRetryOptions = { limit: number; methods: string[]; statusCodes: number[]; afterStatusCodes: number[]; maxRetryAfter: number; backoffLimit: number; delay: (attemptCount: number) => number; jitter: boolean | ((delay: number) => number) | undefined; retryOnTimeout: boolean; shouldRetry?: RetryOptions['shouldRetry']; }
- sym:43043 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:42993 RetryOptions (source/types/retry.ts:21-187)
  type RetryOptions = { /** The number of times to retry failed requests. Must be a finite, non-negative integer. @default 2 */ limit?: number | undefined; /** The HTTP methods allowed to retry. @default ['get', 'put', 'head', 'delete', 'options', 'trace', 'query'] */ methods?: readonly HttpMethod[] | undefined; /** The HTTP status codes allowed to retry. `413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`. @default [408, 413, 429, 500, 502, 503, 504] */ statusCodes?: readonly number[] | undefined; /** The retriable HTTP status codes that should respect retry timing headers. These status codes must also be included in `statusCodes`, unless `shouldRetry` returns `true`. @default [413, 429, 503] */ afterStatusCodes?: readonly number[] | undefined; /** If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`. @default Infinity */ maxRetryAfter?: number | undefined; /** The upper limit of the delay per retry in milliseconds, for the delay calculated by `retry.delay`. To clamp the delay, set `backoffLimit` to 1000, for example. A delay from a retry timing header is not subject to this limit; use `maxRetryAfter` to bound it. By default, the delay is calculated in the following way: ``` 0.3 * (2 ** (attemptCount - 1)) * 1000 ``` The delay increases exponentially. @default Infinity */ backoffLimit?: number | undefined; /** A function to calculate the delay in milliseconds between retries given `attemptCount` (starts from 1). @default attemptCount => 0.3 * (2 ** (attemptCount - 1)) * 1000 */ delay?: ((attemptCount: number) => number) | undefined; /** Add random jitter to retry delays to prevent thundering herd problems. When many clients retry simultaneously (e.g., after hitting a rate limit), they can overwhelm the server again. Jitter adds randomness to break this synchronization. Set to `true` to use full jitter, which randomizes the delay between 0 and the computed delay. Alternatively, pass a function to implement custom jitter strategies. Note: Jitter is not applied when the server provides a retry timing header, as the server's explicit timing should be respected. @default undefined (no jitter) @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 5, // Full jitter (randomizes delay between 0 and computed value) jitter: true // Percentage jitter (80-120% of delay) // jitter: delay => delay * (0.8 + Math.random() * 0.4) // Absolute jitter (±100ms) // jitter: delay => delay + (Math.random() * 200 - 100) } }).json(); ``` */ jitter?: boolean | ((delay: number) => number) | undefined; /** Whether to retry when the request times out before a response is returned. Timeouts while reading a response body through Ky shortcut methods are not retried because the response has already been received. @default false @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 3, retryOnTimeout: true } }).json(); ``` */ retryOnTimeout?: boolean | undefined; /** A function to determine whether a retry should be attempted. This function takes precedence over the default retry checks (`retryOnTimeout`, status code checks, etc.) for retriable methods. It is only called after the retry limit and method checks pass. **Note:** This is different from the `beforeRetry` hook: - `shouldRetry`: Controls WHETHER to retry (called before the retry decision is made) - `beforeRetry`: Called AFTER retry is confirmed, allowing you to modify the request Should return: - `true` to force a retry (bypasses `retryOnTimeout`, status code checks, and other validations) - `false` to prevent a retry (no retry will occur) - `undefined` (or nothing) to use the default retry logic (`retryOnTimeout`, status codes, network errors). Unrecognized error types are not retried. Returning `true` overrides retry eligibility, not timing. For HTTP errors with a status in `afterStatusCodes`, retry timing headers and `maxRetryAfter` still apply, without jitter. Otherwise, Ky uses the configured backoff delay. @default undefined @example ``` import ky, {HTTPError} from 'ky'; const json = await ky('https://example.com', { retry: { limit: 3, shouldRetry: ({error, retryCount}) => { // Retry on specific business logic errors from API if (error instanceof HTTPError) { const status = error.response.status; // Retry on 429 (rate limit) but only for first 2 attempts if (status === 429 && retryCount <= 2) { return true; } // Don't retry on 4xx errors except rate limits if (status >= 400 && status < 500) { return false; } } // Use default retry logic for other errors return undefined; } } }).json(); ``` */ shouldRetry?: ((state: ShouldRetryState) => boolean | void | Promise<boolean | void>) | undefined; }
- sym:42917 Ky.#fetch (source/core/Ky.ts:1394-1458)
  async #fetch(): Promise<Response>
- sym:42990 KyResponse (source/types/response.ts:1-4)
  type KyResponse<T = unknown> = { clone: () => KyResponse<T>; json: <J = T>() => Promise<J>; } & Response
- sym:43005 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42970 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:42964 BeforeRequestHook (source/types/hooks.ts:42-42)
  type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>
- sym:43050 isResponseSizeError (source/utils/type-guards.ts:152-154)
  function isResponseSizeError(error: unknown): error is ResponseSizeError
- sym:42966 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:42905 Ky.#readResponseText (source/core/Ky.ts:1074-1142)
  async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined>
- sym:42891 Ky.create (source/core/Ky.ts:253-453)
  static create(input: Input, options: Options): ResponsePromise
- sym:42893 Ky.constructor (source/core/Ky.ts:486-654)
  constructor(input: Input, options: Options = {})
- sym:42900 Ky.#getErrorDataTimeout (source/core/Ky.ts:872-890)
  #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout
- sym:42901 Ky.#getEffectiveTimeout (source/core/Ky.ts:892-905)
  #getEffectiveTimeout(): number | undefined
- sym:42956 TimeoutError (source/errors/TimeoutError.ts:7-15)
  class TimeoutError extends KyError
- sym:42987 InternalOptions (source/types/options.ts:517-528)
  type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & { headers: Headers; hooks: NormalizedHooks; retry: NormalizedRetryOptions; fetch: NonNullable<Options['fetch']>; prefix: string; timeout: number | false; totalTimeout: number | false; maxResponseSize: number; context: Record<string, unknown>; throwHttpErrors: boolean | ((status: number) => boolean); }
- sym:43042 TimeoutOptions (source/utils/timeout.ts:4-7)
  type TimeoutOptions = { timeout: number; fetch: InternalOptions['fetch']; }
- sym:43077 createResponseLike (test/hooks.ts:42-65)
  const createResponseLike = (response: Response): any =>
- sym:43071 browserTest (test/helpers/with-page.ts:39-78)
  const browserTest = (title: string, browserTypes: BrowserType[], run: Run) =>

---
tokens: 3139/4000
