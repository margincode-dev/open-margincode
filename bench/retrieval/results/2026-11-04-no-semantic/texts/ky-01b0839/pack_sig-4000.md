## Task
# ky-01b0839 · ky-01b0839-2 · open
Goal: Recognise another copy of Ky: the stop symbol and the retry marker
Workspace: /root/bench/ws/rebf815-ky-01b0839/ky-01b0839-2
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
- sym:43190 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:43246 cloneShallow (source/utils/merge.ts:91-113)
  const cloneShallow = <T>(value: T): T =>
- sym:43194 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:43156 RetryMarker (source/core/constants.ts:157-163)
  class RetryMarker
- sym:43158 retry (source/core/constants.ts:248-248)
  const retry = (options?: ForceRetryOptions) =>
- sym:43157 RetryMarker.constructor (source/core/constants.ts:160-162)
  constructor(options?: ForceRetryOptions)
- sym:43124 Ky.#throwProcessedError (source/core/Ky.ts:782-812)
  async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
- sym:43139 Ky.#runAfterResponseHooks (source/core/Ky.ts:1204-1270)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:43247 cloneDeep (source/utils/merge.ts:116-154)
  const cloneDeep = <T>(value: T, seen: WeakMap<Record<string, unknown> | unknown[], unknown> = new WeakMap()): T =>
- sym:43184 Primitive (source/types/common.ts:2-2)
  type Primitive = null | undefined | string | number | boolean | symbol | bigint
- sym:43141 Ky.#retryFromError (source/core/Ky.ts:1280-1367)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:43238 ReplaceMarked (source/utils/merge.ts:8-11)
  type ReplaceMarked<T> = { [replaceSymbol]: true; value: T; }
- sym:43229 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43114 cloneSearchParametersForInitHook (source/core/Ky.ts:145-151)
  const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined =>
- sym:43117 Ky (source/core/Ky.ts:252-1527)
  class Ky
- sym:43118 Ky.create (source/core/Ky.ts:253-452)
  static create(input: Input, options: Options): ResponsePromise
- sym:43197 KyInstance (source/types/ky.ts:5-204)
  type KyInstance = { /** Fetch the given `url`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` method added. @example ``` import ky from 'ky'; const json = await ky('https://example.com', {json: {foo: true}}).json(); console.log(json); //=> `{data: '🦄'}` ``` */ <T>(url: Input, options?: Options): ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'get'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ get: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'post'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ post: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'put'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ put: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'delete'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ delete: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'patch'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ patch: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Fetch the given `url` using the option `{method: 'head'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ head: (url: Input, options?: Options) => ResponsePromise; /** Fetch the given `url` using the option `{method: 'query'}`. @param url - `Request` object, `URL` object, or URL string. @returns A promise with `Body` methods added. */ query: <T>(url: Input, options?: Options) => ResponsePromise<T>; /** Create a new Ky instance with complete new defaults, without inheriting from any parent instance. @returns A new Ky instance. */ create: (defaultOptions?: Options) => KyInstance; /** Create a new Ky instance with some defaults overridden with your own. In contrast to `ky.create()`, `ky.extend()` inherits defaults from its parent. You can pass headers as a `Headers` instance or a plain object. Headers are normalized to a plain object with lowercase names when options are merged, so `init` hooks always see a plain object with lowercase keys. A header removed with `undefined` stays in that object with an `undefined` value. You can remove a header with `.extend()` by passing the header with an `undefined` value. Passing `undefined` as a string removes the header only if it comes from a `Headers` instance. Similarly, you can remove existing `hooks` entries by extending the hook with an explicit `undefined`. By default, `.extend()` deep-merges options: hooks are appended, headers are merged, and search parameters are accumulated. Use `replaceOption` when you want to fully replace a merged property instead. When extending an instance, signals are combined. Use `replaceOption(signal)` to replace inherited signals, or `signal: undefined` to remove them. You can also refer to parent defaults by providing a function to `.extend()`. @example ``` import ky from 'ky'; const api = ky.create({prefix: 'https://example.com/api'}); const usersApi = api.extend((options) => ({prefix: `${options.prefix}/users`})); const response = await usersApi.get('123'); //=> 'https://example.com/api/users/123' const response = await api.get('version'); //=> 'https://example.com/api/version' ``` @returns A new Ky instance. */ extend: (defaultOptions: Options | ((parentOptions: Options) => Options)) => KyInstance; /** A `Symbol` that can be returned by a `beforeRetry` hook to stop the retry. This will also short circuit the remaining `beforeRetry` hooks. Note: Returning this symbol makes Ky abort and return with an `undefined` response. Be sure to check for a response before accessing any properties on it or use [optional chaining](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining). It is also incompatible with body methods, such as `.json()` or `.text()`, because there is no response to parse. In general, we recommend throwing an error instead of returning this symbol, as that will cause Ky to abort and then throw, which avoids these limitations. A valid use-case for `ky.stop` is to prevent retries when making requests for side effects, where the returned data is not important. For example, logging client activity to the server. @example ``` import ky from 'ky'; const options = { hooks: { beforeRetry: [ async ({request, options, error, retryCount}) => { const shouldStopRetry = await ky('https://example.com/api'); if (shouldStopRetry) { return ky.stop; } } ] } }; // Note that response will be `undefined` in case `ky.stop` is returned. const response = await ky.post('https://example.com', options); // Using `.text()` or other body methods is not supported. const text = await ky('https://example.com', options).text(); ``` */ readonly stop: typeof stop; /** Force a retry from an `afterResponse` hook. This allows you to retry a request based on the response content, even if the response has a successful status code. The retry will respect the `retry.limit` option and skip the `shouldRetry` check. The forced retry is observable in `beforeRetry` hooks, where the error will be a `ForceRetryError`. @example ``` import ky, {isForceRetryError} from 'ky'; const api = ky.extend({ hooks: { afterResponse: [ async ({response}) => { // Retry based on response body content if (response.status === 200) { const data = await response.json(); // Simple retry with default delay if (data.error?.code === 'TEMPORARY_ERROR') { return ky.retry(); } // Retry with custom delay from API response if (data.error?.code === 'RATE_LIMIT') { return ky.retry({ delay: data.error.retryAfter * 1000, code: 'RATE_LIMIT' }); } } } ], beforeRetry: [ ({error, retryCount}) => { // Observable in beforeRetry hooks if (isForceRetryError(error)) { console.log(`Forced retry #${retryCount}: ${error.message}`); // Example output: "Forced retry #1: Forced retry: RATE_LIMIT" } } ] } }); const response = await api.get('https://example.com/api'); ``` */ readonly retry: typeof retry; }
- sym:43202 Input (source/types/options.ts:19-19)
  type Input = string | URL | Request
- sym:43208 Options (source/types/options.ts:448-502)
  interface Options extends KyOptions, RequestOptions
- sym:43242 validateAndMerge (source/utils/merge.ts:54-62)
  const validateAndMerge = (...sources: Array<Partial<Options> | undefined>): Partial<Options> =>
- sym:43244 isPlainObject (source/utils/merge.ts:79-86)
  const isPlainObject = (value: unknown): value is Record<string, unknown> =>
- sym:43298 createRequestLike (test/hooks.ts:23-40)
  const createRequestLike = (request: Request): any =>
- sym:43299 createResponseLike (test/hooks.ts:42-65)
  const createResponseLike = (response: Response): any =>

---
tokens: 2949/4000
