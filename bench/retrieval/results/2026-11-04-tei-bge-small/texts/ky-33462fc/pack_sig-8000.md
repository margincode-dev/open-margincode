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
- sym:41961 initHook (test-d/hooks.ts:18-29)
  const initHook: InitHook = options =>
- sym:41888 newHookValue (source/utils/merge.ts:197-215)
  function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K]
- sym:41750 Ky (source/core/Ky.ts:267-1585)
  class Ky
- sym:41904 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:41832 Hooks (source/types/hooks.ts:90-371)
  type Hooks = { /** This hook enables you to modify the options before they are used to construct the request. The hook function receives the mutable options object and can modify it in place. You could, for example, modify `searchParams`, `headers`, or `json` here. The `headers` option is always a plain object with lowercase names, where a header removed with `undefined` keeps an `undefined` value. Unlike other hooks, `init` hooks are synchronous. Any error thrown will propagate synchronously and will not be caught by `beforeError` hooks. The `body` option is passed through by reference, so mutating a `FormData` or `URLSearchParams` body in place changes your own object and repeats on every request from the same instance. Replace `options.body` with a new instance instead, the way the `beforeRequest` workflow does. A common use case is to add a search parameter to every request: @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { init: [ options => { options.searchParams = {apiKey: getApiKey()}; }, ], }, }); const response = await api.get('https://example.com/api/users'); // URL: https://example.com/api/users?apiKey=123 ``` @default [] */ init?: readonly InitHook[] | undefined; /** This hook enables you to modify the request right before it is sent. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, and retry count. You could, for example, modify `request.headers` here. The `retryCount` is always `0`, since `beforeRequest` hooks run once before retry handling begins. The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing request (remaining hooks will still run with the updated request). It can also return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to completely avoid making an HTTP request, in which case remaining `beforeRequest` hooks are skipped. This can be used to mock a request, check an internal cache, etc. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. Any error thrown by `beforeRequest` hooks is treated as fatal and will not trigger Ky's retry logic. @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { beforeRequest: [ ({request}) => { request.headers.set('Authorization', 'token initial-token'); } ] } }); const response = await api.get('https://example.com/api/users'); ``` **Modifying the request URL:** @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { beforeRequest: [ ({request}) => { const url = new URL(request.url); url.searchParams.set('token', 'secret-token'); return new Request(url, request); } ] } }); const response = await api.get('https://example.com/api/users'); ``` @default [] */ beforeRequest?: readonly BeforeRequestHook[] | undefined; /** This hook enables you to modify the request right before retry. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, an error instance, and retry count. You could, for example, modify `request.headers` here. The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing retry request, or return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to skip the retry and use that response instead. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. **Note:** Returning a request or response skips remaining `beforeRetry` hooks. **Warning:** Returned `Request` objects are used as-is. If you point one at another origin, remove any credentials you do not want forwarded. The `retryCount` is always `>= 1`, since this hook is only called during retries, not on the initial request. If the request received a response, the error will be of type `HTTPError`. The `Response` object will be available at `error.response`, and the pre-parsed response body may be available at `error.data`. Be aware that some types of errors, such as network errors, inherently mean that a response was not received. In that case, the error will be an instance of `NetworkError` instead of `HTTPError`. You can prevent Ky from retrying the request by throwing an error. Ky will not handle it in any way and the error will be propagated to the request initiator. The rest of the `beforeRetry` hooks will not be called in this case. Alternatively, you can return the [`ky.stop`](#kystop) symbol to do the same thing but without propagating an error (this has some limitations, see `ky.stop` docs for details). **Modifying headers:** @example ``` import ky from 'ky'; const response = await ky('https://example.com', { hooks: { beforeRetry: [ async ({request, options, error, retryCount}) => { const token = await ky('https://example.com/refresh-token'); request.headers.set('Authorization', `token ${token}`); } ] } }); ``` **Modifying the request URL:** @example ``` import ky, {isHTTPError} from 'ky'; const response = await ky('https://example.com/api', { hooks: { beforeRetry: [ ({request, error}) => { // Add query parameters based on error response if ( isHTTPError(error) && typeof error.data === 'object' && error.data !== null && 'processId' in error.data ) { const url = new URL(request.url); url.searchParams.set('processId', String(error.data.processId)); return new Request(url, request); } } ] } }); ``` **Returning a cached response:** @example ``` import ky from 'ky'; const response = await ky('https://example.com/api', { hooks: { beforeRetry: [ ({error, retryCount}) => { // Use cached response instead of retrying if (retryCount > 1 && cachedResponse) { return cachedResponse; } } ] } }); ``` @default [] */ beforeRetry?: readonly BeforeRetryHook[] | undefined; /** This hook enables you to modify any error right before it is thrown. The hook function receives a state object with the current request, the normalized Ky options, the error, and retry count, and should return an `Error` instance. This hook receives errors Ky handles during its request lifecycle, including `HTTPError`, `NetworkError`, `TimeoutError`, `ResponseSizeError`, and `ForceRetryError` (when retry limit is exceeded via `ky.retry()`). Use type guards like `isHTTPError()`, `isNetworkError()`, `isTimeoutError()`, or `isResponseSizeError()` to handle specific error types. Errors raised while consuming an already returned `Response` are outside the request lifecycle. Use a body method shortcut like `await ky(url).json()` to process body-read `ResponseSizeError`, `NetworkError`, and `TimeoutError` instances through this hook. The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). If a `beforeRequest` or `beforeRetry` hook returns a new `Request`, inspect `request` for the final request state. `options` remains Ky's normalized options and may not mirror every property of a replacement `Request`. @default [] @example ``` import ky, {isHTTPError} from 'ky'; await ky('https://example.com', { hooks: { beforeError: [ ({request, options, error}) => { if (isHTTPError(error)) { if ( typeof error.data === 'object' && error.data !== null && 'message' in error.data ) { error.name = 'GitHubError'; error.message = `${String(error.data.message)} (${error.response.status})`; } } // `request` and `options` are always available console.log(`Request to ${request.url} failed`, options.context); return error; } ] } }); ``` */ beforeError?: readonly BeforeErrorHook[] | undefined; /** This hook enables you to read and optionally modify the response. The hook function receives a state object with the normalized request, options, a clone of the response, and retry count. The return value of the hook function will be used by Ky as the response object if it's an instance of [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response). You can also force a retry by returning [`ky.retry(options)`](#kyretryoptions). This is useful when you need to retry based on the response body content, even if the response has a successful status code. The retry will respect the `retry.limit` option and be observable in `beforeRetry` hooks. **Warning:** `ky.retry({request})` uses the replacement request as-is. If it targets another origin, remove any credentials you do not want forwarded. Any non-`ky.retry()` error thrown by `afterResponse` hooks is treated as fatal and will not trigger Ky's retry logic. The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). @default [] @example ``` import ky from 'ky'; const response = await ky('https://example.com', { hooks: { afterResponse: [ ({response}) => { // You could do something with the response, for example, logging. log(response); // Or return a `Response` instance to overwrite the response. return new Response('A different response', {status: 200}); }, // Or retry with a fresh token on a 401 error async ({request, response, retryCount}) => { if (response.status === 401 && retryCount === 0) { // Only refresh on first 401, not on subsequent retries const {token} = await ky.post('https://example.com/auth/refresh').json(); const headers = new Headers(request.headers); headers.set('Authorization', `Bearer ${token}`); return ky.retry({ request: new Request(request, {headers}), code: 'TOKEN_REFRESHED' }); } }, // Or force retry based on response body content async ({response}) => { if (response.status === 200) { const data = await response.json(); if (data.error?.code === 'RATE_LIMIT') { // Retry with custom delay from API response return ky.retry({ delay: data.error.retryAfter * 1000, code: 'RATE_LIMIT' }); } } }, // Or show a notification only on the last retry for 5xx errors ({options, response, retryCount}) => { if (response.status >= 500 && response.status <= 599) { if (retryCount === options.retry.limit) { showNotification('Request failed after all retries'); } } } ] } }); ``` */ afterResponse?: readonly AfterResponseHook[] | undefined; }
- sym:41962 initHook (test-d/init-search-params.ts:4-10)
  const initHook: InitHook = options =>
- sym:41757 Ky.#throwProcessedError (source/core/Ky.ts:807-837)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:41829 BeforeErrorHook (source/types/hooks.ts:73-73)
  type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>
- sym:41823 InitHook (source/types/hooks.ts:30-30)
  type InitHook = (options: InitOptions) => void
- sym:41774 Ky.#runAfterResponseHooks (source/core/Ky.ts:1258-1324)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:41825 BeforeRequestHook (source/types/hooks.ts:42-42)
  type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>
- sym:41768 Ky.#cancelReturnedBody (source/core/Ky.ts:1196-1202)
  #cancelReturnedBody(value: unknown): void
- sym:41901 findUnknownOptions (source/utils/options.ts:5-29)
  const findUnknownOptions = ( options: Record<string, unknown>, ): Record<string, unknown> =>
- sym:41871 delay (source/utils/delay.ts:9-29)
  async function delay( ms: number, {signal}: DelayOptions, ): Promise<void>
- sym:41762 Ky.#raceWithTotalTimeout (source/core/Ky.ts:983-1067)
  async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation>
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
- sym:41966 initHook (test-d/retry.ts:17-26)
  const initHook: InitHook = options =>
- sym:41828 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:41770 Ky.#shouldThrowHttpErrors (source/core/Ky.ts:1212-1220)
  #shouldThrowHttpErrors(response: Response): boolean
- sym:41772 Ky.#throwIfTotalTimeoutExhausted (source/core/Ky.ts:1228-1233)
  #throwIfTotalTimeoutExhausted(request: Request = this.#sentRequest): void
- sym:41831 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:41827 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:41848 InternalOptions (source/types/options.ts:535-546)
  type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & { headers: Headers; hooks: NormalizedHooks; retry: NormalizedRetryOptions; fetch: NonNullable<Options['fetch']>; prefix: string; timeout: number | false; totalTimeout: number | false; maxResponseSize: number; context: Record<string, unknown>; throwHttpErrors: boolean | ((status: number) => boolean); }
- sym:41826 BeforeRetryState (source/types/hooks.ts:44-53)
  type BeforeRetryState = { request: KyRequest; options: Readonly<NormalizedOptions>; error: Error; /** The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request. */ retryCount: number; }
- sym:41748 cloneInitHookOptions (source/core/Ky.ts:172-235)
  function cloneInitHookOptions(options: Options): InitOptions
- sym:41778 Ky.#fetch (source/core/Ky.ts:1430-1492)
  async #fetch(): Promise<Response>
- sym:41747 cloneSearchParametersForInitHook (source/core/Ky.ts:160-166)
  const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined =>
- sym:41773 Ky.#runBeforeRequestHooks (source/core/Ky.ts:1235-1256)
  async #runBeforeRequestHooks(): Promise<Response | undefined>
- sym:41893 appendSearchParameters (source/utils/merge.ts:278-332)
  const appendSearchParameters = (target: any, source: any): URLSearchParams =>
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
- sym:41937 createRequestLike (test/hooks.ts:23-40)
  const createRequestLike = (request: Request): any =>
- sym:41940 createStreamFetch (test/hooks.ts:100-114)
  const createStreamFetch = ({ text = 'ok', onResponse, }: { text?: string; onResponse?: (response: Response) => void; } = {}): typeof fetch =>

---
tokens: 5205/8000
