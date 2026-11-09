## Task
# ky-01b0839 · ky-01b0839 · open
Goal: Recognise another copy of Ky: the stop symbol and the retry marker
Workspace: /root/bench/ws/rebf815-ky-01b0839/ky-01b0839
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
- sym:43156 RetryMarker (source/core/constants.ts:157-163)
  class RetryMarker
- sym:43158 retry (source/core/constants.ts:248-248)
  const retry = (options?: ForceRetryOptions) =>
- sym:43190 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:43157 RetryMarker.constructor (source/core/constants.ts:160-162)
  constructor(options?: ForceRetryOptions)
- sym:43141 Ky.#retryFromError (source/core/Ky.ts:1280-1367)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:43216 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:43122 Ky.#calculateRetryDelay (source/core/Ky.ts:678-758)
  async #calculateRetryDelay(error: unknown)
- sym:43109 cloneRetryOptions (source/core/Ky.ts:101-122)
  const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number =>
- sym:43139 Ky.#runAfterResponseHooks (source/core/Ky.ts:1204-1270)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:43161 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
  const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
- sym:43182 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:43189 BeforeRetryState (source/types/hooks.ts:44-53)
  type BeforeRetryState = { request: KyRequest; options: Readonly<NormalizedOptions>; error: Error; /** The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request. */ retryCount: number; }
- sym:43191 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:43246 cloneShallow (source/utils/merge.ts:91-113)
  const cloneShallow = <T>(value: T): T =>
- sym:43194 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:43193 AfterResponseState (source/types/hooks.ts:75-86)
  type AfterResponseState = { request: KyRequest; options: Readonly<NormalizedOptions>; response: KyResponse; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). */ retryCount: number; }
- sym:43267 isKyError (source/utils/type-guards.ts:36-38)
  function isKyError(error: unknown): error is KyError
- sym:43124 Ky.#throwProcessedError (source/core/Ky.ts:782-812)
  async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
- sym:43171 KyError.isKyError (source/errors/KyError.ts:11-13)
  get isKyError(): true
- sym:43247 cloneDeep (source/utils/merge.ts:116-154)
  const cloneDeep = <T>(value: T, seen: WeakMap<Record<string, unknown> | unknown[], unknown> = new WeakMap()): T =>
- sym:43155 ForceRetryOptions (source/core/constants.ts:73-152)
  type ForceRetryOptions = { /** Custom delay in milliseconds before retrying. If not provided, uses the default retry delay calculation based on `retry.delay` configuration. **Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified. Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry. */ delay?: number | undefined; /** Error code for the retry. This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries. @example ``` return ky.retry({code: 'RATE_LIMIT'}); // Resulting error message: 'Forced retry: RATE_LIMIT' ``` */ code?: string | undefined; /** Original error that caused the retry. This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal. @example ``` try { const data = await response.json(); validateBusinessLogic(data); } catch (error) { return ky.retry({ code: 'VALIDATION_FAILED', cause: error // Preserves original error in chain }); } ``` */ cause?: Error | undefined; /** Custom request to use for the retry. This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed. **Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming. **Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded. @example ``` // Fallback to a different endpoint return ky.retry({ request: new Request('https://backup-api.com/endpoint', { method: request.method, headers: request.headers, }), code: 'BACKUP_ENDPOINT' }); // Retry with refreshed authentication token const data = await response.json(); return ky.retry({ request: new Request(request, { headers: { ...Object.fromEntries(request.headers), 'Authorization': `Bearer ${data.newToken}` } }), code: 'TOKEN_REFRESHED' }); ``` */ request?: Request | undefined; }
- sym:43184 Primitive (source/types/common.ts:2-2)
  type Primitive = null | undefined | string | number | boolean | symbol | bigint
- sym:43140 Ky.#retry (source/core/Ky.ts:1272-1278)
  async #retry(): Promise<Response | void>
- sym:43238 ReplaceMarked (source/utils/merge.ts:8-11)
  type ReplaceMarked<T> = { [replaceSymbol]: true; value: T; }
- sym:43142 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1369-1373)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:43166 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:43229 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43271 isForceRetryError (source/utils/type-guards.ts:129-131)
  function isForceRetryError(error: unknown): error is ForceRetryError
- sym:43241 replaceOption (source/utils/merge.ts:49-52)
  const replaceOption = <T>(value: T): T =>
- sym:43254 appendSearchParameters (source/utils/merge.ts:263-317)
  const appendSearchParameters = (target: any, source: any): URLSearchParams =>
- sym:43131 Ky.#readResponseText (source/core/Ky.ts:1055-1123)
  async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined>
- sym:43217 RetryOptions (source/types/retry.ts:21-187)
  type RetryOptions = { /** The number of times to retry failed requests. Must be a finite, non-negative integer. @default 2 */ limit?: number | undefined; /** The HTTP methods allowed to retry. @default ['get', 'put', 'head', 'delete', 'options', 'trace', 'query'] */ methods?: readonly HttpMethod[] | undefined; /** The HTTP status codes allowed to retry. `413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`. @default [408, 413, 429, 500, 502, 503, 504] */ statusCodes?: readonly number[] | undefined; /** The retriable HTTP status codes that should respect retry timing headers. These status codes must also be included in `statusCodes`, unless `shouldRetry` returns `true`. @default [413, 429, 503] */ afterStatusCodes?: readonly number[] | undefined; /** If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`. @default Infinity */ maxRetryAfter?: number | undefined; /** The upper limit of the delay per retry in milliseconds, for the delay calculated by `retry.delay`. To clamp the delay, set `backoffLimit` to 1000, for example. A delay from a retry timing header is not subject to this limit; use `maxRetryAfter` to bound it. By default, the delay is calculated in the following way: ``` 0.3 * (2 ** (attemptCount - 1)) * 1000 ``` The delay increases exponentially. @default Infinity */ backoffLimit?: number | undefined; /** A function to calculate the delay in milliseconds between retries given `attemptCount` (starts from 1). @default attemptCount => 0.3 * (2 ** (attemptCount - 1)) * 1000 */ delay?: ((attemptCount: number) => number) | undefined; /** Add random jitter to retry delays to prevent thundering herd problems. When many clients retry simultaneously (e.g., after hitting a rate limit), they can overwhelm the server again. Jitter adds randomness to break this synchronization. Set to `true` to use full jitter, which randomizes the delay between 0 and the computed delay. Alternatively, pass a function to implement custom jitter strategies. Note: Jitter is not applied when the server provides a retry timing header, as the server's explicit timing should be respected. @default undefined (no jitter) @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 5, // Full jitter (randomizes delay between 0 and computed value) jitter: true // Percentage jitter (80-120% of delay) // jitter: delay => delay * (0.8 + Math.random() * 0.4) // Absolute jitter (±100ms) // jitter: delay => delay + (Math.random() * 200 - 100) } }).json(); ``` */ jitter?: boolean | ((delay: number) => number) | undefined; /** Whether to retry when the request times out before a response is returned. Timeouts while reading a response body through Ky shortcut methods are not retried because the response has already been received. @default false @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 3, retryOnTimeout: true } }).json(); ``` */ retryOnTimeout?: boolean | undefined; /** A function to determine whether a retry should be attempted. This function takes precedence over the default retry checks (`retryOnTimeout`, status code checks, etc.) for retriable methods. It is only called after the retry limit and method checks pass. **Note:** This is different from the `beforeRetry` hook: - `shouldRetry`: Controls WHETHER to retry (called before the retry decision is made) - `beforeRetry`: Called AFTER retry is confirmed, allowing you to modify the request Should return: - `true` to force a retry (bypasses `retryOnTimeout`, status code checks, and other validations) - `false` to prevent a retry (no retry will occur) - `undefined` (or nothing) to use the default retry logic (`retryOnTimeout`, status codes, network errors). Unrecognized error types are not retried. Returning `true` overrides retry eligibility, not timing. For HTTP errors with a status in `afterStatusCodes`, retry timing headers and `maxRetryAfter` still apply, without jitter. Otherwise, Ky uses the configured backoff delay. @default undefined @example ``` import ky, {HTTPError} from 'ky'; const json = await ky('https://example.com', { retry: { limit: 3, shouldRetry: ({error, retryCount}) => { // Retry on specific business logic errors from API if (error instanceof HTTPError) { const status = error.response.status; // Retry on 429 (rate limit) but only for first 2 attempts if (status === 429 && retryCount <= 2) { return true; } // Don't retry on 4xx errors except rate limits if (status >= 400 && status < 500) { return false; } } // Use default retry logic for other errors return undefined; } } }).json(); ``` */ shouldRetry?: ((state: ShouldRetryState) => boolean | void | Promise<boolean | void>) | undefined; }
- sym:43159 RetryTimingHeader (source/core/retry-timing.ts:11-14)
  type RetryTimingHeader = { value: string; allowTimestamp: boolean; }
- sym:43240 getReplaceState (source/utils/merge.ts:18-27)
  const getReplaceState = <T>(value: T): ReplaceState<T> =>
- sym:43144 Ky.#throwIfAbortedByUser (source/core/Ky.ts:1442-1444)
  #throwIfAbortedByUser(): void
- sym:43232 streamRequest (source/utils/body.ts:226-245)
  const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) =>
- sym:43327 initHook (test-d/retry.ts:17-26)
  const initHook: InitHook = options =>
- sym:43106 ErrorDataTimeout (source/core/Ky.ts:65-68)
  type ErrorDataTimeout = { milliseconds: number; fromTotalTimeout: boolean; }
- sym:43210 NormalizedRetryOptions (source/types/options.ts:513-515)
  type NormalizedRetryOptions = { [Key in Exclude<keyof MutableRetryOptions, 'shouldRetry'>]-?: Key extends 'jitter' ? MutableRetryOptions[Key] : Exclude<MutableRetryOptions[Key], undefined>; } & Pick<MutableRetryOptions, 'shouldRetry'>
- sym:43187 BeforeRequestState (source/types/hooks.ts:32-40)
  type BeforeRequestState = { request: KyRequest; options: Readonly<NormalizedOptions>; /** The number of retries attempted. Always `0`, since `beforeRequest` hooks run once before retry handling begins. */ retryCount: 0; }
- sym:43170 KyError (source/errors/KyError.ts:8-14)
  class KyError extends Error
- sym:43165 calculateRetryTimingDelay (source/core/retry-timing.ts:151-173)
  const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined =>
- sym:43120 Ky.constructor (source/core/Ky.ts:485-652)
  constructor(input: Input, options: Options = {})
- sym:43121 Ky.#calculateDelay (source/core/Ky.ts:654-676)
  #calculateDelay(retry: InternalOptions['retry']): number
- sym:43117 Ky (source/core/Ky.ts:252-1527)
  class Ky
- sym:43259 InternalRetryOptions (source/utils/normalize.ts:23-23)
  type InternalRetryOptions = InternalOptions['retry']
- sym:43143 Ky.#fetch (source/core/Ky.ts:1375-1439)
  async #fetch(): Promise<Response>
- sym:43261 normalizeRetryOptions (source/utils/normalize.ts:52-109)
  const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions =>
- sym:43197 KyInstance (source/types/ky.ts:5-204)
  type KyInstance = { /** Fetch the given `url`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` method added. @example ``` import ky from 'ky'; const json = await ky('https://example.com', {json: {foo: true}}).json(); console.log(json); //=> `{data: '🦄'}` ``` */ <T>(url: Input, options?: Options): ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'get'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ get: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'post'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ post: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'put'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ put: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'delete'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ delete: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'patch'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ patch: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'head'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ head: (url: Input, options?: Options) => ResponsePromise; /** Fetch the given `url` using the option `{method: 'query'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ query: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Create a new Ky instance with complete new defaults, without inheriting from any parent instance. @returns A new Ky instance. */ create: (defaultOptions?: Options) => KyInstance; /** Create a new Ky instance with some defaults overridden with your own. In contrast to `ky.create()`, `ky.extend()` inherits defaults from its parent. You can pass headers as a `Headers` instance or a plain object. Headers are normalized to a plain object with lowercase names when options are merged, so `init` hooks always see a plain object with lowercase keys. A header removed with `undefined` stays in that object with an `undefined` value. You can remove a header with `.extend()` by passing the header with an `undefined` value. Passing `undefined` as a string removes the header only if it comes from a `Headers` instance. Similarly, you can remove existing `hooks` entries by extending the hook with an explicit `undefined`. By default, `.extend()` deep-merges options: hooks are appended, headers are merged, and search parameters are accumulated. Use `replaceOption` when you want to fully replace a merged property instead. When extending an instance, signals are combined. Use `replaceOption(signal)` to replace inherited signals, or `signal: undefined` to remove them. You can also refer to parent defaults by providing a function to `.extend()`. @example ``` import ky from 'ky'; const api = ky.create({prefix: 'https://example.com/api'}); const usersApi = api.extend((options) => ({prefix: `${options.prefix}/users`})); const response = await usersApi.get('123'); //=> 'https://example.com/api/users/123' const response = await api.get('version'); //=> 'https://example.com/api/version' ``` @returns A new Ky instance. */ extend: (defaultOptions: Options | ((parentOptions: Options) => Options)) => KyInstance; /** A `Symbol` that can be returned by a `beforeRetry` hook to stop the retry. This will also short circuit the remaining `beforeRetry` hooks. Note: Returning this symbol makes Ky abort and return with an `undefined` response. Be sure to check for a response before accessing any properties on it or use [optional chaining](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining). It is also incompatible with body methods, such as `.json()` or `.text()`, because there is no response to parse. In general, we recommend throwing an error instead of returning this symbol, as that will cause Ky to abort and then throw, which avoids these limitations. A valid use-case for `ky.stop` is to prevent retries when making requests for side effects, where the returned data is not important. For example, logging client activity to the server. @example ``` import ky from 'ky'; const options = { hooks: { beforeRetry: [ async ({request, options, error, retryCount}) => { const shouldStopRetry = await ky('https://example.com/api'); if (shouldStopRetry) { return ky.stop; } } ] } }; // Note that response will be `undefined` in case `ky.stop` is returned. const response = await ky.post('https://example.com', options); // Using `.text()` or other body methods is not supported. const text = await ky('https://example.com', options).text(); ``` */ readonly stop: typeof stop; /** Force a retry from an `afterResponse` hook. This allows you to retry a request based on the response content, even if the response has a successful status code. The retry will respect the `retry.limit` option and skip the `shouldRetry` check. The forced retry is observable in `beforeRetry` hooks, where the error will be a `ForceRetryError`. @example ``` import ky, {isForceRetryError} from 'ky'; const api = ky.extend({ hooks: { afterResponse: [ async ({response}) => { // Retry based on response body content if (response.status === 200) { const data = await response.json(); // Simple retry with default delay if (data.error?.code === 'TEMPORARY_ERROR') { return ky.retry(); } // Retry with custom delay from API response if (data.error?.code === 'RATE_LIMIT') { return ky.retry({ delay: data.error.retryAfter * 1000, code: 'RATE_LIMIT' }); } } } ], beforeRetry: [ ({error, retryCount}) => { // Observable in beforeRetry hooks if (isForceRetryError(error)) { console.log(`Forced retry #${retryCount}: ${error.message}`); // Example output: "Forced retry #1: Forced retry: RATE_LIMIT" } } ] } }); const response = await api.get('https://example.com/api'); ``` */ readonly retry: typeof retry; }
- sym:43260 normalizeRetryLimit (source/utils/normalize.ts:40-50)
  const normalizeRetryLimit = (retryLimit: unknown): number =>
- sym:43136 Ky.#createManagedSignal (source/core/Ky.ts:1168-1172)
  #createManagedSignal(): AbortSignal
- sym:43325 ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
  type ExpectedNormalizedRetryOptions = { limit: number; methods: string[]; statusCodes: number[]; afterStatusCodes: number[]; maxRetryAfter: number; backoffLimit: number; delay: (attemptCount: number) => number; jitter: boolean | ((delay: number) => number) | undefined; retryOnTimeout: boolean; shouldRetry?: RetryOptions['shouldRetry']; }
- sym:43115 cloneInitHookOptions (source/core/Ky.ts:157-220)
  function cloneInitHookOptions(options: Options): InitOptions
- sym:43118 Ky.create (source/core/Ky.ts:253-452)
  static create(input: Input, options: Options): ResponsePromise
- sym:43205 KyOptions (source/types/options.ts:45-432)
- sym:43209 InitOptions (source/types/options.ts:504-511)
  type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & { get headers(): Record<string, string | undefined>; set headers(value: KyHeadersInit | undefined); get context(): Record<string, unknown>; set context(value: Record<string, unknown> | undefined); retry?: MutableRetryOptions | number | undefined; hooks?: {[Key in keyof Hooks]?: NormalizedHooks[Key] | undefined} | undefined; }
- sym:43211 InternalOptions (source/types/options.ts:517-528)
  type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & { headers: Headers; hooks: NormalizedHooks; retry: NormalizedRetryOptions; fetch: NonNullable<Options['fetch']>; prefix: string; timeout: number | false; totalTimeout: number | false; maxResponseSize: number; context: Record<string, unknown>; throwHttpErrors: boolean | ((status: number) => boolean); }
- sym:43212 NormalizedOptions (source/types/options.ts:533-546)
  interface NormalizedOptions extends Readonly<RequestInit>
- sym:43298 createRequestLike (test/hooks.ts:23-40)
  const createRequestLike = (request: Request): any =>
- sym:43299 createResponseLike (test/hooks.ts:42-65)
  const createResponseLike = (response: Response): any =>

---
tokens: 6354/8000
