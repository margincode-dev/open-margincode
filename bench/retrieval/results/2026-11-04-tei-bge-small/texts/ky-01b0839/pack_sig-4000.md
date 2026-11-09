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
- sym:43140 Ky.#retry (source/core/Ky.ts:1272-1278)
  async #retry(): Promise<Response | void>
- sym:43182 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:43190 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:43109 cloneRetryOptions (source/core/Ky.ts:101-122)
  const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number =>
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
- sym:43121 Ky.#calculateDelay (source/core/Ky.ts:654-676)
  #calculateDelay(retry: InternalOptions['retry']): number
- sym:43124 Ky.#throwProcessedError (source/core/Ky.ts:782-812)
  async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
- sym:43122 Ky.#calculateRetryDelay (source/core/Ky.ts:678-758)
  async #calculateRetryDelay(error: unknown)
- sym:43139 Ky.#runAfterResponseHooks (source/core/Ky.ts:1204-1270)
  async #runAfterResponseHooks(response: Response): Promise<Response>
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
- sym:43131 Ky.#readResponseText (source/core/Ky.ts:1055-1123)
  async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined>
- sym:43229 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43110 isRequestInstance (source/core/Ky.ts:130-131)
  const isRequestInstance = (value: unknown): value is Request =>
- sym:43111 isResponseInstance (source/core/Ky.ts:136-137)
  const isResponseInstance = (value: unknown): value is Response =>
- sym:43114 cloneSearchParametersForInitHook (source/core/Ky.ts:145-151)
  const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined =>
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
- sym:43277 KY_SCRIPT (test/browser.ts:33-39)
  const KY_SCRIPT = {
- sym:43298 createRequestLike (test/hooks.ts:23-40)
  const createRequestLike = (request: Request): any =>
- sym:43299 createResponseLike (test/hooks.ts:42-65)
  const createResponseLike = (response: Response): any =>

---
tokens: 1887/4000
