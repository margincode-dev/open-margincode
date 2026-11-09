## Task
# ky-01b0839 · ky-01b0839 · open
Goal: Recognise another copy of Ky: the stop symbol and the retry marker
Workspace: /root/bench/av_tei/ws/rec09fc-ky-01b0839/ky-01b0839
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
- sym:43246 cloneShallow (source/utils/merge.ts:91-113)
  const cloneShallow = <T>(value: T): T =>
- sym:43141 Ky.#retryFromError (source/core/Ky.ts:1280-1367)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:43109 cloneRetryOptions (source/core/Ky.ts:101-122)
  const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number =>
- sym:43124 Ky.#throwProcessedError (source/core/Ky.ts:782-812)
  async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
- sym:43139 Ky.#runAfterResponseHooks (source/core/Ky.ts:1204-1270)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:43122 Ky.#calculateRetryDelay (source/core/Ky.ts:678-758)
  async #calculateRetryDelay(error: unknown)
- sym:43121 Ky.#calculateDelay (source/core/Ky.ts:654-676)
  #calculateDelay(retry: InternalOptions['retry']): number
- sym:43131 Ky.#readResponseText (source/core/Ky.ts:1055-1123)
  async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined>
- sym:43254 appendSearchParameters (source/utils/merge.ts:263-317)
  const appendSearchParameters = (target: any, source: any): URLSearchParams =>
- sym:43140 Ky.#retry (source/core/Ky.ts:1272-1278)
  async #retry(): Promise<Response | void>
- sym:43182 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:43190 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:43142 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1369-1373)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:43194 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:43156 RetryMarker (source/core/constants.ts:157-163)
  class RetryMarker
- sym:43163 getCapture (source/core/retry-timing.ts:86-86)
  const getCapture = (match: RegExpExecArray, index: number): string =>
- sym:43158 retry (source/core/constants.ts:248-248)
  const retry = (options?: ForceRetryOptions) =>
- sym:43157 RetryMarker.constructor (source/core/constants.ts:160-162)
  constructor(options?: ForceRetryOptions)
- sym:43117 Ky (source/core/Ky.ts:252-1527)
  class Ky
- sym:43247 cloneDeep (source/utils/merge.ts:116-154)
  const cloneDeep = <T>(value: T, seen: WeakMap<Record<string, unknown> | unknown[], unknown> = new WeakMap()): T =>
- sym:43115 cloneInitHookOptions (source/core/Ky.ts:157-220)
  function cloneInitHookOptions(options: Options): InitOptions
- sym:43184 Primitive (source/types/common.ts:2-2)
  type Primitive = null | undefined | string | number | boolean | symbol | bigint
- sym:43120 Ky.constructor (source/core/Ky.ts:485-652)
  constructor(input: Input, options: Options = {})
- sym:43238 ReplaceMarked (source/utils/merge.ts:8-11)
  type ReplaceMarked<T> = { [replaceSymbol]: true; value: T; }
- sym:43253 resolveReplaceMarkers (source/utils/merge.ts:230-261)
  const resolveReplaceMarkers = (value: unknown, seen: Set<Record<string, unknown> | unknown[]>): unknown =>
- sym:43143 Ky.#fetch (source/core/Ky.ts:1375-1439)
  async #fetch(): Promise<Response>
- sym:43171 KyError.isKyError (source/errors/KyError.ts:11-13)
  get isKyError(): true
- sym:43229 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43155 ForceRetryOptions (source/core/constants.ts:73-152)
  type ForceRetryOptions = { /** Custom delay in milliseconds before retrying. If not provided, uses the default retry delay calculation based on `retry.delay` configuration. **Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified. Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry. */ delay?: number | undefined; /** Error code for the retry. This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries. @example ``` return ky.retry({code: 'RATE_LIMIT'}); // Resulting error message: 'Forced retry: RATE_LIMIT' ``` */ code?: string | undefined; /** Original error that caused the retry. This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal. @example ``` try { const data = await response.json(); validateBusinessLogic(data); } catch (error) { return ky.retry({ code: 'VALIDATION_FAILED', cause: error // Preserves original error in chain }); } ``` */ cause?: Error | undefined; /** Custom request to use for the retry. This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed. **Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming. **Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded. @example ``` // Fallback to a different endpoint return ky.retry({ request: new Request('https://backup-api.com/endpoint', { method: request.method, headers: request.headers, }), code: 'BACKUP_ENDPOINT' }); // Retry with refreshed authentication token const data = await response.json(); return ky.retry({ request: new Request(request, { headers: { ...Object.fromEntries(request.headers), 'Authorization': `Bearer ${data.newToken}` } }), code: 'TOKEN_REFRESHED' }); ``` */ request?: Request | undefined; }
- sym:43241 replaceOption (source/utils/merge.ts:49-52)
  const replaceOption = <T>(value: T): T =>
- sym:43125 Ky.#getResponseData (source/core/Ky.ts:814-851)
  async #getResponseData(response: Response): Promise<unknown>
- sym:43148 Ky.#assignRequest (source/core/Ky.ts:1482-1491)
  #assignRequest(request: Request, requestBodyCanBeCancelled = false): void
- sym:43240 getReplaceState (source/utils/merge.ts:18-27)
  const getReplaceState = <T>(value: T): ReplaceState<T> =>
- sym:43232 streamRequest (source/utils/body.ts:226-245)
  const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) =>
- sym:43193 AfterResponseState (source/types/hooks.ts:75-86)
  type AfterResponseState = { request: KyRequest; options: Readonly<NormalizedOptions>; response: KyResponse; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). */ retryCount: number; }
- sym:43197 KyInstance (source/types/ky.ts:5-204)
  type KyInstance = { /** Fetch the given `url`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` method added. @example ``` import ky from 'ky'; const json = await ky('https://example.com', {json: {foo: true}}).json(); console.log(json); //=> `{data: '🦄'}` ``` */ <T>(url: Input, options?: Options): ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'get'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ get: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'post'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ post: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'put'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ put: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'delete'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ delete: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'patch'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ patch: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'head'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ head: (url: Input, options?: Options) => ResponsePromise; /** Fetch the given `url` using the option `{method: 'query'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ query: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Create a new Ky instance with complete new defaults, without inheriting from any parent instance. @returns A new Ky instance. */ create: (defaultOptions?: Options) => KyInstance; /** Create a new Ky instance with some defaults overridden with your own. In contrast to `ky.create()`, `ky.extend()` inherits defaults from its parent. You can pass headers as a `Headers` instance or a plain object. Headers are normalized to a plain object with lowercase names when options are merged, so `init` hooks always see a plain object with lowercase keys. A header removed with `undefined` stays in that object with an `undefined` value. You can remove a header with `.extend()` by passing the header with an `undefined` value. Passing `undefined` as a string removes the header only if it comes from a `Headers` instance. Similarly, you can remove existing `hooks` entries by extending the hook with an explicit `undefined`. By default, `.extend()` deep-merges options: hooks are appended, headers are merged, and search parameters are accumulated. Use `replaceOption` when you want to fully replace a merged property instead. When extending an instance, signals are combined. Use `replaceOption(signal)` to replace inherited signals, or `signal: undefined` to remove them. You can also refer to parent defaults by providing a function to `.extend()`. @example ``` import ky from 'ky'; const api = ky.create({prefix: 'https://example.com/api'}); const usersApi = api.extend((options) => ({prefix: `${options.prefix}/users`})); const response = await usersApi.get('123'); //=> 'https://example.com/api/users/123' const response = await api.get('version'); //=> 'https://example.com/api/version' ``` @returns A new Ky instance. */ extend: (defaultOptions: Options | ((parentOptions: Options) => Options)) => KyInstance; /** A `Symbol` that can be returned by a `beforeRetry` hook to stop the retry. This will also short circuit the remaining `beforeRetry` hooks. Note: Returning this symbol makes Ky abort and return with an `undefined` response. Be sure to check for a response before accessing any properties on it or use [optional chaining](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining). It is also incompatible with body methods, such as `.json()` or `.text()`, because there is no response to parse. In general, we recommend throwing an error instead of returning this symbol, as that will cause Ky to abort and then throw, which avoids these limitations. A valid use-case for `ky.stop` is to prevent retries when making requests for side effects, where the returned data is not important. For example, logging client activity to the server. @example ``` import ky from 'ky'; const options = { hooks: { beforeRetry: [ async ({request, options, error, retryCount}) => { const shouldStopRetry = await ky('https://example.com/api'); if (shouldStopRetry) { return ky.stop; } } ] } }; // Note that response will be `undefined` in case `ky.stop` is returned. const response = await ky.post('https://example.com', options); // Using `.text()` or other body methods is not supported. const text = await ky('https://example.com', options).text(); ``` */ readonly stop: typeof stop; /** Force a retry from an `afterResponse` hook. This allows you to retry a request based on the response content, even if the response has a successful status code. The retry will respect the `retry.limit` option and skip the `shouldRetry` check. The forced retry is observable in `beforeRetry` hooks, where the error will be a `ForceRetryError`. @example ``` import ky, {isForceRetryError} from 'ky'; const api = ky.extend({ hooks: { afterResponse: [ async ({response}) => { // Retry based on response body content if (response.status === 200) { const data = await response.json(); // Simple retry with default delay if (data.error?.code === 'TEMPORARY_ERROR') { return ky.retry(); } // Retry with custom delay from API response if (data.error?.code === 'RATE_LIMIT') { return ky.retry({ delay: data.error.retryAfter * 1000, code: 'RATE_LIMIT' }); } } } ], beforeRetry: [ ({error, retryCount}) => { // Observable in beforeRetry hooks if (isForceRetryError(error)) { console.log(`Forced retry #${retryCount}: ${error.message}`); // Example output: "Forced retry #1: Forced retry: RATE_LIMIT" } } ] } }); const response = await api.get('https://example.com/api'); ``` */ readonly retry: typeof retry; }
- sym:43327 initHook (test-d/retry.ts:17-26)
  const initHook: InitHook = options =>
- sym:43191 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:43210 NormalizedRetryOptions (source/types/options.ts:513-515)
  type NormalizedRetryOptions = { [Key in Exclude<keyof MutableRetryOptions, 'shouldRetry'>]-?: Key extends 'jitter' ? MutableRetryOptions[Key] : Exclude<MutableRetryOptions[Key], undefined>; } & Pick<MutableRetryOptions, 'shouldRetry'>
- sym:43114 cloneSearchParametersForInitHook (source/core/Ky.ts:145-151)
  const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined =>
- sym:43161 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
  const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
- sym:43213 KyRequest (source/types/request.ts:1-4)
  type KyRequest<T = unknown> = { clone: () => KyRequest<T>; json: <J = T>() => Promise<J>; } & Request
- sym:43129 Ky.#raceWithTotalTimeout (source/core/Ky.ts:958-1031)
  async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation>
- sym:43216 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:43243 mergeHeaders (source/utils/merge.ts:64-77)
  const mergeHeaders = (source1: KyHeadersInit = {}, source2: KyHeadersInit = {}) =>
- sym:43259 InternalRetryOptions (source/utils/normalize.ts:23-23)
  type InternalRetryOptions = InternalOptions['retry']
- sym:43133 Ky.#cancelBody (source/core/Ky.ts:1146-1153)
  #cancelBody(body: ReadableStream | undefined): void
- sym:43261 normalizeRetryOptions (source/utils/normalize.ts:52-109)
  const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions =>
- sym:43170 KyError (source/errors/KyError.ts:8-14)
  class KyError extends Error
- sym:43260 normalizeRetryLimit (source/utils/normalize.ts:40-50)
  const normalizeRetryLimit = (retryLimit: unknown): number =>
- sym:43149 Ky.#withManagedSignal (source/core/Ky.ts:1494-1504)
  #withManagedSignal(request: Request): Request
- sym:43325 ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
  type ExpectedNormalizedRetryOptions = { limit: number; methods: string[]; statusCodes: number[]; afterStatusCodes: number[]; maxRetryAfter: number; backoffLimit: number; delay: (attemptCount: number) => number; jitter: boolean | ((delay: number) => number) | undefined; retryOnTimeout: boolean; shouldRetry?: RetryOptions['shouldRetry']; }
- sym:43110 isRequestInstance (source/core/Ky.ts:130-131)
  const isRequestInstance = (value: unknown): value is Request =>
- sym:43111 isResponseInstance (source/core/Ky.ts:136-137)
  const isResponseInstance = (value: unknown): value is Response =>
- sym:43174 NonError (source/errors/NonError.ts:6-28)
  class NonError extends Error
- sym:43180 TimeoutError (source/errors/TimeoutError.ts:7-15)
  class TimeoutError extends KyError
- sym:43234 delay (source/utils/delay.ts:9-29)
  async function delay( ms: number, {signal}: DelayOptions, ): Promise<void>
- sym:43244 isPlainObject (source/utils/merge.ts:79-86)
  const isPlainObject = (value: unknown): value is Record<string, unknown> =>
- sym:43255 deepMergeInternal (source/utils/merge.ts:320-455)
  const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T =>
- sym:43271 isForceRetryError (source/utils/type-guards.ts:129-131)
  function isForceRetryError(error: unknown): error is ForceRetryError
- sym:43277 KY_SCRIPT (test/browser.ts:33-39)
  const KY_SCRIPT = {
- sym:43298 createRequestLike (test/hooks.ts:23-40)
  const createRequestLike = (request: Request): any =>
- sym:43299 createResponseLike (test/hooks.ts:42-65)
  const createResponseLike = (response: Response): any =>

---
tokens: 5167/8000
