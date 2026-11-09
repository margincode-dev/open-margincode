## Task
# ky-33462fc · ky-33462fc · open
Goal: Reject a nullish hook list instead of treating it as absent
Workspace: /root/bench/av_gemma/ws/rec728b-ky-33462fc/ky-33462fc
Changed (0): nothing yet

## Map
- repo map (107 files):
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
  test-d/ (21 files)
    error-names.ts
    fetch-extensions.ts
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
- sym:41889 mergeHooks (source/utils/merge.ts:217-231)
  const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks =>
- sym:41960 beforeErrorHook (test-d/hooks.ts:5-5)
  const beforeErrorHook: BeforeErrorHook = ({error}) =>
- sym:41777 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1424-1428)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:41833 NormalizedHooks (source/types/hooks.ts:376-376)
  type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>}
- sym:41902 hasSearchParameters (source/utils/options.ts:31-60)
  const hasSearchParameters = (search: SearchParamsOption): boolean =>
- sym:41888 newHookValue (source/utils/merge.ts:197-215)
  function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K]
- sym:41750 Ky (source/core/Ky.ts:267-1585)
  class Ky
- sym:41751 Ky.create (source/core/Ky.ts:268-464)
  static create(input: Input, options: Options): ResponsePromise
- sym:41961 initHook (test-d/hooks.ts:18-29)
  const initHook: InitHook = options =>
- sym:41823 InitHook (source/types/hooks.ts:30-30)
  type InitHook = (options: InitOptions) => void
- sym:41825 BeforeRequestHook (source/types/hooks.ts:42-42)
  type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>
- sym:41871 delay (source/utils/delay.ts:9-29)
  async function delay( ms: number, {signal}: DelayOptions, ): Promise<void>
- sym:41966 initHook (test-d/retry.ts:17-26)
  const initHook: InitHook = options =>
- sym:41762 Ky.#raceWithTotalTimeout (source/core/Ky.ts:983-1067)
  async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation>
- sym:41844 RequestOptions (source/types/options.ts:441-443)
  type RequestOptions = { [Key in Exclude<keyof RequestInit, 'headers' | 'signal' | 'method'>]?: RequestInit[Key] | undefined; }
- sym:41904 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:41753 Ky.constructor (source/core/Ky.ts:498-668)
  constructor(input: Input, options: Options = {})
- sym:41829 BeforeErrorHook (source/types/hooks.ts:73-73)
  type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>
- sym:41866 copyResponseMetadata (source/utils/body.ts:107-170)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:41831 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:41747 cloneSearchParametersForInitHook (source/core/Ky.ts:160-166)
  const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined =>
- sym:41804 ForceRetryError.constructor (source/errors/ForceRetryError.ts:16-38)
  constructor(options?: ForceRetryOptions)
- sym:41789 validate (source/core/constants.ts:40-40)
  const validate = <T extends Array<true>>() =>
- sym:41799 createTimestamp (source/core/retry-timing.ts:51-84)
  const createTimestamp = ({year, month, day, hours, minutes, seconds}: RetryTimingDateParts): number | undefined =>
- sym:41883 cloneShallow (source/utils/merge.ts:100-122)
  const cloneShallow = <T>(value: T): T =>
- sym:41803 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:41832 Hooks (source/types/hooks.ts:90-371)
- sym:41873 isObject (source/utils/is.ts:2-2)
  const isObject = (value: unknown): value is object =>
- sym:41894 deepMergeInternal (source/utils/merge.ts:335-478)
  const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T =>
- sym:41931 promiseWithTimeout (test/helpers/with-page.ts:19-37)
  const promiseWithTimeout = async <Value>( promise: Promise<Value>, timeoutInMilliseconds: number, errorMessage: string, ): Promise<Value> =>
- sym:41957 createAbortDuringFetch (test/retry.ts:4040-4056)
  const createAbortDuringFetch = (onRequest?: () => void): typeof fetch =>
- sym:41937 createRequestLike (test/hooks.ts:23-40)
  const createRequestLike = (request: Request): any =>

---
tokens: 1732/4000
