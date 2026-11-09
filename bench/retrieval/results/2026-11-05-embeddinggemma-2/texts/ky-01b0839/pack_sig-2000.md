## Task
# ky-01b0839 · ky-01b0839 · open
Goal: Recognise another copy of Ky: the stop symbol and the retry marker
Workspace: /root/bench/av_gemma/ws/rec728b-ky-01b0839/ky-01b0839
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
- sym:43182 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:43158 retry (source/core/constants.ts:248-248)
  const retry = (options?: ForceRetryOptions) =>
- sym:43140 Ky.#retry (source/core/Ky.ts:1272-1278)
  async #retry(): Promise<Response | void>
- sym:43142 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1369-1373)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:43190 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:43246 cloneShallow (source/utils/merge.ts:91-113)
  const cloneShallow = <T>(value: T): T =>
- sym:43144 Ky.#throwIfAbortedByUser (source/core/Ky.ts:1442-1444)
  #throwIfAbortedByUser(): void
- sym:43194 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:43156 RetryMarker (source/core/constants.ts:157-163)
  class RetryMarker
- sym:43139 Ky.#runAfterResponseHooks (source/core/Ky.ts:1204-1270)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:43157 RetryMarker.constructor (source/core/constants.ts:160-162)
  constructor(options?: ForceRetryOptions)
- sym:43170 KyError (source/errors/KyError.ts:8-14)
  class KyError extends Error
- sym:43117 Ky (source/core/Ky.ts:252-1527)
  class Ky
- sym:43124 Ky.#throwProcessedError (source/core/Ky.ts:782-812)
  async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
- sym:43109 cloneRetryOptions (source/core/Ky.ts:101-122)
  const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number =>
- sym:43115 cloneInitHookOptions (source/core/Ky.ts:157-220)
  function cloneInitHookOptions(options: Options): InitOptions
- sym:43118 Ky.create (source/core/Ky.ts:253-452)
  static create(input: Input, options: Options): ResponsePromise
- sym:43120 Ky.constructor (source/core/Ky.ts:485-652)
  constructor(input: Input, options: Options = {})

---
tokens: 1139/2000
