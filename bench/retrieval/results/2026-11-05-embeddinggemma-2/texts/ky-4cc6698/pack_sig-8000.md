## Task
# ky-4cc6698 · ky-4cc6698 · open
Goal: Recognise a cross-realm `beforeRetry` error and keep the retried error as the cause
Workspace: /root/bench/av_gemma/ws/rec728b-ky-4cc6698/ky-4cc6698
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
- sym:41986 Ky.#throwProcessedError (source/core/Ky.ts:805-835)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42057 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:42055 BeforeRetryState (source/types/hooks.ts:44-53)
  type BeforeRetryState = { request: KyRequest; options: Readonly<NormalizedOptions>; error: Error; /** The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request. */ retryCount: number; }
- sym:42033 ForceRetryError.constructor (source/errors/ForceRetryError.ts:16-38)
  constructor(options?: ForceRetryOptions)
- sym:42032 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:42006 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1421-1425)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:42056 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:41984 Ky.#calculateRetryDelay (source/core/Ky.ts:694-774)
  async #calculateRetryDelay(error: unknown)
- sym:42019 ForceRetryOptions (source/core/constants.ts:73-152)
  type ForceRetryOptions = { /** Custom delay in milliseconds before retrying. If not provided, uses the default retry delay calculation based on `retry.delay` configuration. **Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified. Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry. */ delay?: number | undefined; /** Error code for the retry. This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries. @example ``` return ky.retry({code: 'RATE_LIMIT'}); // Resulting error message: 'Forced retry: RATE_LIMIT' ``` */ code?: string | undefined; /** Original error that caused the retry. This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal. @example ``` try { const data = await response.json(); validateBusinessLogic(data); } catch (error) { return ky.retry({ code: 'VALIDATION_FAILED', cause: error // Preserves original error in chain }); } ``` */ cause?: Error | undefined; /** Custom request to use for the retry. This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed. **Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming. **Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded. @example ``` // Fallback to a different endpoint return ky.retry({ request: new Request('https://backup-api.com/endpoint', { method: request.method, headers: request.headers, }), code: 'BACKUP_ENDPOINT' }); // Retry with refreshed authentication token const data = await response.json(); return ky.retry({ request: new Request(request, { headers: { ...Object.fromEntries(request.headers), 'Authorization': `Bearer ${data.newToken}` } }), code: 'TOKEN_REFRESHED' }); ``` */ request?: Request | undefined; }
- sym:42005 Ky.#retryFromError (source/core/Ky.ts:1332-1419)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:42053 BeforeRequestState (source/types/hooks.ts:32-40)
  type BeforeRequestState = { request: KyRequest; options: Readonly<NormalizedOptions>; /** The number of retries attempted. Always `0`, since `beforeRequest` hooks run once before retry handling begins. */ retryCount: 0; }
- sym:42004 Ky.#retry (source/core/Ky.ts:1324-1330)
  async #retry(): Promise<Response | void>
- sym:42082 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:42058 BeforeErrorHook (source/types/hooks.ts:73-73)
  type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>
- sym:42139 isForceRetryError (source/utils/type-guards.ts:129-131)
  function isForceRetryError(error: unknown): error is ForceRetryError
- sym:42189 beforeErrorHook (test-d/hooks.ts:5-5)
  const beforeErrorHook: BeforeErrorHook = ({error}) =>
- sym:42129 normalizeRetryOptions (source/utils/normalize.ts:52-109)
  const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions =>
- sym:41979 Ky (source/core/Ky.ts:267-1582)
  class Ky
- sym:41980 Ky.create (source/core/Ky.ts:268-464)
  static create(input: Input, options: Options): ResponsePromise
- sym:42118 mergeHooks (source/utils/merge.ts:217-231)
  const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks =>
- sym:42192 createNetworkError (test-d/network-error-cause.ts:8-8)
  const createNetworkError = (cause?: Error) =>
- sym:42039 NetworkError.constructor (source/errors/NetworkError.ts:15-18)
  constructor(request: Request, options?: {cause?: Error | undefined})
- sym:42195 initHook (test-d/retry.ts:17-26)
  const initHook: InitHook = options =>
- sym:42060 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:42038 NetworkError (source/errors/NetworkError.ts:11-19)
  class NetworkError extends KyError
- sym:42023 isRetryMarker (source/core/constants.ts:170-171)
  const isRetryMarker = (value: unknown): value is RetryMarker =>
- sym:42127 InternalRetryOptions (source/utils/normalize.ts:23-23)
  type InternalRetryOptions = InternalOptions['retry']
- sym:42024 retry (source/core/constants.ts:256-256)
  const retry = (options?: ForceRetryOptions) =>
- sym:42059 AfterResponseState (source/types/hooks.ts:75-86)
  type AfterResponseState = { request: KyRequest; options: Readonly<NormalizedOptions>; response: KyResponse; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). */ retryCount: number; }
- sym:42061 Hooks (source/types/hooks.ts:90-371)
  type Hooks = { /** This hook enables you to modify the options before they are used to construct the request. The hook function receives the mutable options object and can modify it in place. You could, for example, modify `searchParams`, `headers`, or `json` here. The `headers` option is always a plain object with lowercase names, where a header removed with `undefined` keeps an `undefined` value. Unlike other hooks, `init` hooks are synchronous. Any error thrown will propagate synchronously and will not be caught by `beforeError` hooks. The `body` option is passed through by reference, so mutating a `FormData` or `URLSearchParams` body in place changes your own object and repeats on every request from the same instance. Replace `options.body` with a new instance instead, the way the `beforeRequest` workflow does. A common use case is to add a search parameter to every request: @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { init: [ options => { options.searchParams = {apiKey: getApiKey()}; }, ], }, }); const response = await api.get('https://example.com/api/users'); // URL: https://example.com/api/users?apiKey=123 ``` @default [] */ init?: readonly InitHook[] | undefined; /** This hook enables you to modify the request right before it is sent. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, and retry count. You could, for example, modify `request.headers` here. The `retryCount` is always `0`, since `beforeRequest` hooks run once before retry handling begins. The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing request (remaining hooks will still run with the updated request). It can also return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to completely avoid making an HTTP request, in which case remaining `beforeRequest` hooks are skipped. This can be used to mock a request, check an internal cache, etc. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. Any error thrown by `beforeRequest` hooks is treated as fatal and will not trigger Ky's retry logic. @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { beforeRequest: [ ({request}) => { request.headers.set('Authorization', 'token initial-token'); } ] } }); const response = await api.get('https://example.com/api/users'); ``` **Modifying the request URL:** @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { beforeRequest: [ ({request}) => { const url = new URL(request.url); url.searchParams.set('token', 'secret-token'); return new Request(url, request); } ] } }); const response = await api.get('https://example.com/api/users'); ``` @default [] */ beforeRequest?: readonly BeforeRequestHook[] | undefined; /** This hook enables you to modify the request right before retry. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, an error instance, and retry count. You could, for example, modify `request.headers` here. The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing retry request, or return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to skip the retry and use that response instead. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. **Note:** Returning a request or response skips remaining `beforeRetry` hooks. **Warning:** Returned `Request` objects are used as-is. If you point one at another origin, remove any credentials you do not want forwarded. The `retryCount` is always `>= 1`, since this hook is only called during retries, not on the initial request. If the request received a response, the error will be of type `HTTPError`. The `Response` object will be available at `error.response`, and the pre-parsed response body may be available at `error.data`. Be aware that some types of errors, such as network errors, inherently mean that a response was not received. In that case, the error will be an instance of `NetworkError` instead of `HTTPError`. You can prevent Ky from retrying the request by throwing an error. Ky will not handle it in any way and the error will be propagated to the request initiator. The rest of the `beforeRetry` hooks will not be called in this case. Alternatively, you can return the [`ky.stop`](#kystop) symbol to do the same thing but without propagating an error (this has some limitations, see `ky.stop` docs for details). **Modifying headers:** @example ``` import ky from 'ky'; const response = await ky('https://example.com', { hooks: { beforeRetry: [ async ({request, options, error, retryCount}) => { const token = await ky('https://example.com/refresh-token'); request.headers.set('Authorization', `token ${token}`); } ] } }); ``` **Modifying the request URL:** @example ``` import ky, {isHTTPError} from 'ky'; const response = await ky('https://example.com/api', { hooks: { beforeRetry: [ ({request, error}) => { // Add query parameters based on error response if ( isHTTPError(error) && typeof error.data === 'object' && error.data !== null && 'processId' in error.data ) { const url = new URL(request.url); url.searchParams.set('processId', String(error.data.processId)); return new Request(url, request); } } ] } }); ``` **Returning a cached response:** @example ``` import ky from 'ky'; const response = await ky('https://example.com/api', { hooks: { beforeRetry: [ ({error, retryCount}) => { // Use cached response instead of retrying if (retryCount > 1 && cachedResponse) { return cachedResponse; } } ] } }); ``` @default [] */ beforeRetry?: readonly BeforeRetryHook[] | undefined; /** This hook enables you to modify any error right before it is thrown. The hook function receives a state object with the current request, the normalized Ky options, the error, and retry count, and should return an `Error` instance. This hook receives errors Ky handles during its request lifecycle, including `HTTPError`, `NetworkError`, `TimeoutError`, `ResponseSizeError`, and `ForceRetryError` (when retry limit is exceeded via `ky.retry()`). Use type guards like `isHTTPError()`, `isNetworkError()`, `isTimeoutError()`, or `isResponseSizeError()` to handle specific error types. Errors raised while consuming an already returned `Response` are outside the request lifecycle. Use a body method shortcut like `await ky(url).json()` to process body-read `ResponseSizeError`, `NetworkError`, and `TimeoutError` instances through this hook. The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). If a `beforeRequest` or `beforeRetry` hook returns a new `Request`, inspect `request` for the final request state. `options` remains Ky's normalized options and may not mirror every property of a replacement `Request`. @default [] @example ``` import ky, {isHTTPError} from 'ky'; await ky('https://example.com', { hooks: { beforeError: [ ({request, options, error}) => { if (isHTTPError(error)) { if ( typeof error.data === 'object' && error.data !== null && 'message' in error.data ) { error.name = 'GitHubError'; error.message = `${String(error.data.message)} (${error.response.status})`; } } // `request` and `options` are always available console.log(`Request to ${request.url} failed`, options.context); return error; } ] } }); ``` */ beforeError?: readonly BeforeErrorHook[] | undefined; /** This hook enables you to read and optionally modify the response. The hook function receives a state object with the normalized request, options, a clone of the response, and retry count. The return value of the hook function will be used by Ky as the response object if it's an instance of [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response). You can also force a retry by returning [`ky.retry(options)`](#kyretryoptions). This is useful when you need to retry based on the response body content, even if the response has a successful status code. The retry will respect the `retry.limit` option and be observable in `beforeRetry` hooks. **Warning:** `ky.retry({request})` uses the replacement request as-is. If it targets another origin, remove any credentials you do not want forwarded. Any non-`ky.retry()` error thrown by `afterResponse` hooks is treated as fatal and will not trigger Ky's retry logic. The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). @default [] @example ``` import ky from 'ky'; const response = await ky('https://example.com', { hooks: { afterResponse: [ ({response}) => { // You could do something with the response, for example, logging. log(response); // Or return a `Response` instance to overwrite the response. return new Response('A different response', {status: 200}); }, // Or retry with a fresh token on a 401 error async ({request, response, retryCount}) => { if (response.status === 401 && retryCount === 0) { // Only refresh on first 401, not on subsequent retries const {token} = await ky.post('https://example.com/auth/refresh').json(); const headers = new Headers(request.headers); headers.set('Authorization', `Bearer ${token}`); return ky.retry({ request: new Request(request, {headers}), code: 'TOKEN_REFRESHED' }); } }, // Or force retry based on response body content async ({response}) => { if (response.status === 200) { const data = await response.json(); if (data.error?.code === 'RATE_LIMIT') { // Retry with custom delay from API response return ky.retry({ delay: data.error.retryAfter * 1000, code: 'RATE_LIMIT' }); } } }, // Or show a notification only on the last retry for 5xx errors ({options, response, retryCount}) => { if (response.status >= 500 && response.status <= 599) { if (retryCount === options.retry.limit) { showNotification('Request failed after all retries'); } } } ] } }); ``` */ afterResponse?: readonly AfterResponseHook[] | undefined; }
- sym:42020 RetryMarker (source/core/constants.ts:157-168)
  class RetryMarker
- sym:42083 RetryOptions (source/types/retry.ts:21-187)
  type RetryOptions = { /** The number of times to retry failed requests. Must be a finite, non-negative integer. @default 2 */ limit?: number | undefined; /** The HTTP methods allowed to retry. @default ['get', 'put', 'head', 'delete', 'options', 'trace', 'query'] */ methods?: readonly HttpMethod[] | undefined; /** The HTTP status codes allowed to retry. `413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`. @default [408, 413, 429, 500, 502, 503, 504] */ statusCodes?: readonly number[] | undefined; /** The retriable HTTP status codes that should respect retry timing headers. These status codes must also be included in `statusCodes`, unless `shouldRetry` returns `true`. @default [413, 429, 503] */ afterStatusCodes?: readonly number[] | undefined; /** If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`. @default Infinity */ maxRetryAfter?: number | undefined; /** The upper limit of the delay per retry in milliseconds, for the delay calculated by `retry.delay`. To clamp the delay, set `backoffLimit` to 1000, for example. A delay from a retry timing header is not subject to this limit; use `maxRetryAfter` to bound it. By default, the delay is calculated in the following way: ``` 0.3 * (2 ** (attemptCount - 1)) * 1000 ``` The delay increases exponentially. @default Infinity */ backoffLimit?: number | undefined; /** A function to calculate the delay in milliseconds between retries given `attemptCount` (starts from 1). @default attemptCount => 0.3 * (2 ** (attemptCount - 1)) * 1000 */ delay?: ((attemptCount: number) => number) | undefined; /** Add random jitter to retry delays to prevent thundering herd problems. When many clients retry simultaneously (e.g., after hitting a rate limit), they can overwhelm the server again. Jitter adds randomness to break this synchronization. Set to `true` to use full jitter, which randomizes the delay between 0 and the computed delay. Alternatively, pass a function to implement custom jitter strategies. Note: Jitter is not applied when the server provides a retry timing header, as the server's explicit timing should be respected. @default undefined (no jitter) @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 5, // Full jitter (randomizes delay between 0 and computed value) jitter: true // Percentage jitter (80-120% of delay) // jitter: delay => delay * (0.8 + Math.random() * 0.4) // Absolute jitter (±100ms) // jitter: delay => delay + (Math.random() * 200 - 100) } }).json(); ``` */ jitter?: boolean | ((delay: number) => number) | undefined; /** Whether to retry when the request times out before a response is returned. Timeouts while reading a response body through Ky shortcut methods are not retried because the response has already been received. @default false @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 3, retryOnTimeout: true } }).json(); ``` */ retryOnTimeout?: boolean | undefined; /** A function to determine whether a retry should be attempted. This function takes precedence over the default retry checks (`retryOnTimeout`, status code checks, etc.) for retriable methods. It is only called after the retry limit and method checks pass. **Note:** This is different from the `beforeRetry` hook: - `shouldRetry`: Controls WHETHER to retry (called before the retry decision is made) - `beforeRetry`: Called AFTER retry is confirmed, allowing you to modify the request Should return: - `true` to force a retry (bypasses `retryOnTimeout`, status code checks, and other validations) - `false` to prevent a retry (no retry will occur) - `undefined` (or nothing) to use the default retry logic (`retryOnTimeout`, status codes, network errors). Unrecognized error types are not retried. Returning `true` overrides retry eligibility, not timing. For HTTP errors with a status in `afterStatusCodes`, retry timing headers and `maxRetryAfter` still apply, without jitter. Otherwise, Ky uses the configured backoff delay. @default undefined @example ``` import ky, {HTTPError} from 'ky'; const json = await ky('https://example.com', { retry: { limit: 3, shouldRetry: ({error, retryCount}) => { // Retry on specific business logic errors from API if (error instanceof HTTPError) { const status = error.response.status; // Retry on 429 (rate limit) but only for first 2 attempts if (status === 429 && retryCount <= 2) { return true; } // Don't retry on 4xx errors except rate limits if (status >= 400 && status < 500) { return false; } } // Use default retry logic for other errors return undefined; } } }).json(); ``` */ shouldRetry?: ((state: ShouldRetryState) => boolean | void | Promise<boolean | void>) | undefined; }
- sym:42002 Ky.#runBeforeRequestHooks (source/core/Ky.ts:1233-1254)
  async #runBeforeRequestHooks(): Promise<Response | undefined>
- sym:41982 Ky.constructor (source/core/Ky.ts:498-668)
  constructor(input: Input, options: Options = {})
- sym:42126 normalizeRetryMethod (source/utils/normalize.ts:13-14)
  const normalizeRetryMethod = (method: string): string =>
- sym:41971 cloneRetryOptions (source/core/Ky.ts:116-137)
  const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number =>
- sym:42007 Ky.#fetch (source/core/Ky.ts:1427-1489)
  async #fetch(): Promise<Response>
- sym:42027 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
  const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
- sym:42135 isKyError (source/utils/type-guards.ts:36-38)
  function isKyError(error: unknown): error is KyError
- sym:42054 BeforeRequestHook (source/types/hooks.ts:42-42)
  type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>
- sym:42122 appendSearchParameters (source/utils/merge.ts:278-332)
  const appendSearchParameters = (target: any, source: any): URLSearchParams =>
- sym:42021 RetryMarker.isRetryMarker (source/core/constants.ts:159-161)
  get isRetryMarker(): true
- sym:42081 MutableRetryOptions (source/types/retry.ts:3-7)
  type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & { methods?: HttpMethod[] | undefined; statusCodes?: number[] | undefined; afterStatusCodes?: number[] | undefined; }
- sym:42128 normalizeRetryLimit (source/utils/normalize.ts:40-50)
  const normalizeRetryLimit = (retryLimit: unknown): number =>
- sym:42003 Ky.#runAfterResponseHooks (source/core/Ky.ts:1256-1322)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:42093 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42040 NonError (source/errors/NonError.ts:6-28)
  class NonError extends Error
- sym:42100 delay (source/utils/delay.ts:9-29)
  async function delay( ms: number, {signal}: DelayOptions, ): Promise<void>
- sym:42103 isError (source/utils/is.ts:7-8)
  const isError = (value: unknown): value is Error =>
- sym:42186 createAbortDuringFetch (test/retry.ts:4040-4056)
  const createAbortDuringFetch = (onRequest?: () => void): typeof fetch =>

---
tokens: 6566/8000
