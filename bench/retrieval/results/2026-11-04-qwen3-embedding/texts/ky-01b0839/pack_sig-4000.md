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
- sym:43141 Ky.#retryFromError (source/core/Ky.ts:1280-1367)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:43182 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:43189 BeforeRetryState (source/types/hooks.ts:44-53)
  type BeforeRetryState = { request: KyRequest; options: Readonly<NormalizedOptions>; error: Error; /** The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request. */ retryCount: number; }
- sym:43190 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:43191 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:43216 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:43246 cloneShallow (source/utils/merge.ts:91-113)
  const cloneShallow = <T>(value: T): T =>
- sym:43194 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:43193 AfterResponseState (source/types/hooks.ts:75-86)
  type AfterResponseState = { request: KyRequest; options: Readonly<NormalizedOptions>; response: KyResponse; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). */ retryCount: number; }
- sym:43122 Ky.#calculateRetryDelay (source/core/Ky.ts:678-758)
  async #calculateRetryDelay(error: unknown)
- sym:43157 RetryMarker.constructor (source/core/constants.ts:160-162)
  constructor(options?: ForceRetryOptions)
- sym:43267 isKyError (source/utils/type-guards.ts:36-38)
  function isKyError(error: unknown): error is KyError
- sym:43124 Ky.#throwProcessedError (source/core/Ky.ts:782-812)
  async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
- sym:43171 KyError.isKyError (source/errors/KyError.ts:11-13)
  get isKyError(): true
- sym:43109 cloneRetryOptions (source/core/Ky.ts:101-122)
  const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number =>
- sym:43139 Ky.#runAfterResponseHooks (source/core/Ky.ts:1204-1270)
  async #runAfterResponseHooks(response: Response): Promise<Response>
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
- sym:43110 isRequestInstance (source/core/Ky.ts:130-131)
  const isRequestInstance = (value: unknown): value is Request =>
- sym:43111 isResponseInstance (source/core/Ky.ts:136-137)
  const isResponseInstance = (value: unknown): value is Response =>
- sym:43115 cloneInitHookOptions (source/core/Ky.ts:157-220)
  function cloneInitHookOptions(options: Options): InitOptions
- sym:43118 Ky.create (source/core/Ky.ts:253-452)
  static create(input: Input, options: Options): ResponsePromise
- sym:43120 Ky.constructor (source/core/Ky.ts:485-652)
  constructor(input: Input, options: Options = {})
- sym:43121 Ky.#calculateDelay (source/core/Ky.ts:654-676)
  #calculateDelay(retry: InternalOptions['retry']): number
- sym:43143 Ky.#fetch (source/core/Ky.ts:1375-1439)
  async #fetch(): Promise<Response>
- sym:43174 NonError (source/errors/NonError.ts:6-28)
  class NonError extends Error
- sym:43298 createRequestLike (test/hooks.ts:23-40)
  const createRequestLike = (request: Request): any =>
- sym:43299 createResponseLike (test/hooks.ts:42-65)
  const createResponseLike = (response: Response): any =>

---
tokens: 2559/4000
