## Task
# ky-33462fc · ky-33462fc · open
Goal: Reject a nullish hook list instead of treating it as absent
Workspace: /root/bench/av_tei/ws/rec09fc-ky-33462fc/ky-33462fc
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
- sym:41902 hasSearchParameters (source/utils/options.ts:31-60)
  const hasSearchParameters = (search: SearchParamsOption): boolean =>
- sym:41960 beforeErrorHook (test-d/hooks.ts:5-5)
  const beforeErrorHook: BeforeErrorHook = ({error}) =>
- sym:41751 Ky.create (source/core/Ky.ts:268-464)
  static create(input: Input, options: Options): ResponsePromise
- sym:41888 newHookValue (source/utils/merge.ts:197-215)
  function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K]
- sym:41750 Ky (source/core/Ky.ts:267-1585)
  class Ky
- sym:41961 initHook (test-d/hooks.ts:18-29)
  const initHook: InitHook = options =>
- sym:41768 Ky.#cancelReturnedBody (source/core/Ky.ts:1196-1202)
  #cancelReturnedBody(value: unknown): void
- sym:41901 findUnknownOptions (source/utils/options.ts:5-29)
  const findUnknownOptions = ( options: Record<string, unknown>, ): Record<string, unknown> =>
- sym:41871 delay (source/utils/delay.ts:9-29)
  async function delay( ms: number, {signal}: DelayOptions, ): Promise<void>
- sym:41762 Ky.#raceWithTotalTimeout (source/core/Ky.ts:983-1067)
  async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation>
- sym:41832 Hooks (source/types/hooks.ts:90-371)
- sym:41904 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:41753 Ky.constructor (source/core/Ky.ts:498-668)
  constructor(input: Input, options: Options = {})
- sym:41766 Ky.#cancelBody (source/core/Ky.ts:1182-1189)
  #cancelBody(body: ReadableStream | undefined): void
- sym:41846 InitOptions (source/types/options.ts:522-529)
  type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & { get headers(): Record<string, string | undefined>; set headers(value: KyHeadersInit | undefined); get context(): Record<string, unknown>; set context(value: Record<string, unknown> | undefined); retry?: MutableRetryOptions | number | undefined; hooks?: {[Key in keyof Hooks]?: NormalizedHooks[Key] | undefined} | undefined; }
- sym:41866 copyResponseMetadata (source/utils/body.ts:107-170)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:41783 Ky.#assignRequest (source/core/Ky.ts:1535-1544)
  #assignRequest(request: Request, requestBodyCanBeCancelled = false): void
- sym:41804 ForceRetryError.constructor (source/errors/ForceRetryError.ts:16-38)
  constructor(options?: ForceRetryOptions)
- sym:41799 createTimestamp (source/core/retry-timing.ts:51-84)
  const createTimestamp = ({year, month, day, hours, minutes, seconds}: RetryTimingDateParts): number | undefined =>
- sym:41777 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1424-1428)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:41824 BeforeRequestState (source/types/hooks.ts:32-40)
  type BeforeRequestState = { request: KyRequest; options: Readonly<NormalizedOptions>; /** The number of retries attempted. Always `0`, since `beforeRequest` hooks run once before retry handling begins. */ retryCount: 0; }
- sym:41767 Ky.#cancelResponseBody (source/core/Ky.ts:1191-1194)
  #cancelResponseBody(response: Response): void
- sym:41803 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:41957 createAbortDuringFetch (test/retry.ts:4040-4056)
  const createAbortDuringFetch = (onRequest?: () => void): typeof fetch =>
- sym:41833 NormalizedHooks (source/types/hooks.ts:376-376)
  type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>}
- sym:41836 SearchParamsOption (source/types/options.ts:10-14)
  type SearchParamsOption = | Exclude<SearchParamsInit, string[][]> | Record<string, string | number | boolean | undefined> | Array<Array<string | number | boolean>> | ReadonlyArray<ReadonlyArray<string | number | boolean>>
- sym:41873 isObject (source/utils/is.ts:2-2)
  const isObject = (value: unknown): value is object =>
- sym:41894 deepMergeInternal (source/utils/merge.ts:335-478)
  const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T =>
- sym:41931 promiseWithTimeout (test/helpers/with-page.ts:19-37)
  const promiseWithTimeout = async <Value>( promise: Promise<Value>, timeoutInMilliseconds: number, errorMessage: string, ): Promise<Value> =>
- sym:41956 createAbortDuringBodyFetch (test/retry.ts:4019-4038)
  const createAbortDuringBodyFetch = (): typeof fetch =>

---
tokens: 1815/4000
