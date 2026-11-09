## Task
# ky-6e27594 · ky-6e27594 · open
Goal: Type-check `onDownloadProgress` before the request is sent
Workspace: /root/bench/av_gemma/ws/rec728b-ky-6e27594/ky-6e27594
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
- sym:42422 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
  type ExpectedProgressCallbacks = { readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>; readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>; }
- sym:42325 streamResponse (source/utils/body.ts:197-212)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:42321 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42245 Ky.#wrapRequestWithUploadProgress (source/core/Ky.ts:1553-1559)
  #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request
- sym:42323 copyResponseMetadata (source/utils/body.ts:107-170)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42281 BeforeRequestState (source/types/hooks.ts:32-40)
  type BeforeRequestState = { request: KyRequest; options: Readonly<NormalizedOptions>; /** The number of retries attempted. Always `0`, since `beforeRequest` hooks run once before retry handling begins. */ retryCount: 0; }
- sym:42326 streamRequest (source/utils/body.ts:215-234)
  const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) =>
- sym:42282 BeforeRequestHook (source/types/hooks.ts:42-42)
  type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>
- sym:42230 Ky.#runBeforeRequestHooks (source/core/Ky.ts:1211-1232)
  async #runBeforeRequestHooks(): Promise<Response | undefined>
- sym:42297 Progress (source/types/options.ts:21-36)
  type Progress = { /** A number between `0` and `1` representing the progress percentage. */ percent: number; /** The number of bytes transferred so far. */ transferredBytes: number; /** The total number of bytes to be transferred. This is an estimate and may be `0` for an empty transfer or when the total size cannot be determined. */ totalBytes: number; }
- sym:42306 NormalizedOptions (source/types/options.ts:551-564)
  interface NormalizedOptions extends Readonly<RequestInit>
- sym:42284 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:42289 Hooks (source/types/hooks.ts:90-371)
  type Hooks = { /** This hook enables you to modify the options before they are used to construct the request. The hook function receives the mutable options object and can modify it in place. You could, for example, modify `searchParams`, `headers`, or `json` here. The `headers` option is always a plain object with lowercase names, where a header removed with `undefined` keeps an `undefined` value. Unlike other hooks, `init` hooks are synchronous. Any error thrown will propagate synchronously and will not be caught by `beforeError` hooks. The `body` option is passed through by reference, so mutating a `FormData` or `URLSearchParams` body in place changes your own object and repeats on every request from the same instance. Replace `options.body` with a new instance instead, the way the `beforeRequest` workflow does. A common use case is to add a search parameter to every request: @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { init: [ options => { options.searchParams = {apiKey: getApiKey()}; }, ], }, }); const response = await api.get('https://example.com/api/users'); // URL: https://example.com/api/users?apiKey=123 ``` @default [] */ init?: readonly InitHook[] | undefined; /** This hook enables you to modify the request right before it is sent. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, and retry count. You could, for example, modify `request.headers` here. The `retryCount` is always `0`, since `beforeRequest` hooks run once before retry handling begins. The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing request (remaining hooks will still run with the updated request). It can also return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to completely avoid making an HTTP request, in which case remaining `beforeRequest` hooks are skipped. This can be used to mock a request, check an internal cache, etc. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. Any error thrown by `beforeRequest` hooks is treated as fatal and will not trigger Ky's retry logic. @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { beforeRequest: [ ({request}) => { request.headers.set('Authorization', 'token initial-token'); } ] } }); const response = await api.get('https://example.com/api/users'); ``` **Modifying the request URL:** @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { beforeRequest: [ ({request}) => { const url = new URL(request.url); url.searchParams.set('token', 'secret-token'); return new Request(url, request); } ] } }); const response = await api.get('https://example.com/api/users'); ``` @default [] */ beforeRequest?: readonly BeforeRequestHook[] | undefined; /** This hook enables you to modify the request right before retry. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, an error instance, and retry count. You could, for example, modify `request.headers` here. The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing retry request, or return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to skip the retry and use that response instead. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. **Note:** Returning a request or response skips remaining `beforeRetry` hooks. **Warning:** Returned `Request` objects are used as-is. If you point one at another origin, remove any credentials you do not want forwarded. The `retryCount` is always `>= 1`, since this hook is only called during retries, not on the initial request. If the request received a response, the error will be of type `HTTPError`. The `Response` object will be available at `error.response`, and the pre-parsed response body may be available at `error.data`. Be aware that some types of errors, such as network errors, inherently mean that a response was not received. In that case, the error will be an instance of `NetworkError` instead of `HTTPError`. You can prevent Ky from retrying the request by throwing an error. Ky will not handle it in any way and the error will be propagated to the request initiator. The rest of the `beforeRetry` hooks will not be called in this case. Alternatively, you can return the [`ky.stop`](#kystop) symbol to do the same thing but without propagating an error (this has some limitations, see `ky.stop` docs for details). **Modifying headers:** @example ``` import ky from 'ky'; const response = await ky('https://example.com', { hooks: { beforeRetry: [ async ({request, options, error, retryCount}) => { const token = await ky('https://example.com/refresh-token'); request.headers.set('Authorization', `token ${token}`); } ] } }); ``` **Modifying the request URL:** @example ``` import ky, {isHTTPError} from 'ky'; const response = await ky('https://example.com/api', { hooks: { beforeRetry: [ ({request, error}) => { // Add query parameters based on error response if ( isHTTPError(error) && typeof error.data === 'object' && error.data !== null && 'processId' in error.data ) { const url = new URL(request.url); url.searchParams.set('processId', String(error.data.processId)); return new Request(url, request); } } ] } }); ``` **Returning a cached response:** @example ``` import ky from 'ky'; const response = await ky('https://example.com/api', { hooks: { beforeRetry: [ ({error, retryCount}) => { // Use cached response instead of retrying if (retryCount > 1 && cachedResponse) { return cachedResponse; } } ] } }); ``` @default [] */ beforeRetry?: readonly BeforeRetryHook[] | undefined; /** This hook enables you to modify any error right before it is thrown. The hook function receives a state object with the current request, the normalized Ky options, the error, and retry count, and should return an `Error` instance. This hook receives errors Ky handles during its request lifecycle, including `HTTPError`, `NetworkError`, `TimeoutError`, `ResponseSizeError`, and `ForceRetryError` (when retry limit is exceeded via `ky.retry()`). Use type guards like `isHTTPError()`, `isNetworkError()`, `isTimeoutError()`, or `isResponseSizeError()` to handle specific error types. Errors raised while consuming an already returned `Response` are outside the request lifecycle. Use a body method shortcut like `await ky(url).json()` to process body-read `ResponseSizeError`, `NetworkError`, and `TimeoutError` instances through this hook. The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). If a `beforeRequest` or `beforeRetry` hook returns a new `Request`, inspect `request` for the final request state. `options` remains Ky's normalized options and may not mirror every property of a replacement `Request`. @default [] @example ``` import ky, {isHTTPError} from 'ky'; await ky('https://example.com', { hooks: { beforeError: [ ({request, options, error}) => { if (isHTTPError(error)) { if ( typeof error.data === 'object' && error.data !== null && 'message' in error.data ) { error.name = 'GitHubError'; error.message = `${String(error.data.message)} (${error.response.status})`; } } // `request` and `options` are always available console.log(`Request to ${request.url} failed`, options.context); return error; } ] } }); ``` */ beforeError?: readonly BeforeErrorHook[] | undefined; /** This hook enables you to read and optionally modify the response. The hook function receives a state object with the normalized request, options, a clone of the response, and retry count. The return value of the hook function will be used by Ky as the response object if it's an instance of [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response). You can also force a retry by returning [`ky.retry(options)`](#kyretryoptions). This is useful when you need to retry based on the response body content, even if the response has a successful status code. The retry will respect the `retry.limit` option and be observable in `beforeRetry` hooks. **Warning:** `ky.retry({request})` uses the replacement request as-is. If it targets another origin, remove any credentials you do not want forwarded. Any non-`ky.retry()` error thrown by `afterResponse` hooks is treated as fatal and will not trigger Ky's retry logic. The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). @default [] @example ``` import ky from 'ky'; const response = await ky('https://example.com', { hooks: { afterResponse: [ ({response}) => { // You could do something with the response, for example, logging. log(response); // Or return a `Response` instance to overwrite the response. return new Response('A different response', {status: 200}); }, // Or retry with a fresh token on a 401 error async ({request, response, retryCount}) => { if (response.status === 401 && retryCount === 0) { // Only refresh on first 401, not on subsequent retries const {token} = await ky.post('https://example.com/auth/refresh').json(); const headers = new Headers(request.headers); headers.set('Authorization', `Bearer ${token}`); return ky.retry({ request: new Request(request, {headers}), code: 'TOKEN_REFRESHED' }); } }, // Or force retry based on response body content async ({response}) => { if (response.status === 200) { const data = await response.json(); if (data.error?.code === 'RATE_LIMIT') { // Retry with custom delay from API response return ky.retry({ delay: data.error.retryAfter * 1000, code: 'RATE_LIMIT' }); } } }, // Or show a notification only on the last retry for 5xx errors ({options, response, retryCount}) => { if (response.status >= 500 && response.status <= 599) { if (retryCount === options.retry.limit) { showNotification('Request failed after all retries'); } } } ] } }); ``` */ afterResponse?: readonly AfterResponseHook[] | undefined; }
- sym:42283 BeforeRetryState (source/types/hooks.ts:44-53)
  type BeforeRetryState = { request: KyRequest; options: Readonly<NormalizedOptions>; error: Error; /** The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request. */ retryCount: number; }
- sym:42219 Ky.#raceBodyRead (source/core/Ky.ts:914-980)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42215 Ky.#throwProcessedError (source/core/Ky.ts:806-836)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42229 Ky.#throwIfTotalTimeoutExhausted (source/core/Ky.ts:1204-1209)
  #throwIfTotalTimeoutExhausted(request: Request = this.#sentRequest): void
- sym:42235 Ky.#fetch (source/core/Ky.ts:1405-1467)
  async #fetch(): Promise<Response>
- sym:42285 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:42322 getProgressCallbackError (source/utils/body.ts:104-105)
  const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
- sym:42199 validateCallbackOptions (source/core/Ky.ts:103-114)
  const validateCallbackOptions = (options: Record<string, unknown>): void =>
- sym:42228 Ky.#sentRequest (source/core/Ky.ts:1200-1202)
  get #sentRequest(): Request
- sym:42233 Ky.#retryFromError (source/core/Ky.ts:1310-1397)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:42213 Ky.#calculateRetryDelay (source/core/Ky.ts:702-782)
  async #calculateRetryDelay(error: unknown)
- sym:42217 Ky.#getErrorDataTimeout (source/core/Ky.ts:877-895)
  #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout
- sym:42225 Ky.#cancelResponseBody (source/core/Ky.ts:1179-1182)
  #cancelResponseBody(response: Response): void
- sym:42204 normalizeInputForProtocolCheck (source/core/Ky.ts:157-158)
  const normalizeInputForProtocolCheck = (input: string): string =>
- sym:42329 isRawNetworkError (source/utils/is-network-error.ts:15-46)
  function isRawNetworkError(error: unknown): error is TypeError
- sym:42418 initHook (test-d/hooks.ts:18-29)
  const initHook: InitHook = options =>
- sym:42286 BeforeErrorHook (source/types/hooks.ts:73-73)
  type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>
- sym:42226 Ky.#cancelReturnedBody (source/core/Ky.ts:1184-1190)
  #cancelReturnedBody(value: unknown): void
- sym:42346 mergeHooks (source/utils/merge.ts:208-222)
  const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks =>
- sym:42196 ErrorDataTimeout (source/core/Ky.ts:65-68)
  type ErrorDataTimeout = { milliseconds: number; fromTotalTimeout: boolean; }
- sym:42299 KyOptions (source/types/options.ts:45-432)
- sym:42240 Ky.#assignRequest (source/core/Ky.ts:1510-1519)
  #assignRequest(request: Request, requestBodyCanBeCancelled = false): void
- sym:42234 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1399-1403)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:42247 ForceRetryOptions (source/core/constants.ts:73-152)
  type ForceRetryOptions = { /** Custom delay in milliseconds before retrying. If not provided, uses the default retry delay calculation based on `retry.delay` configuration. **Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified. Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry. */ delay?: number | undefined; /** Error code for the retry. This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries. @example ``` return ky.retry({code: 'RATE_LIMIT'}); // Resulting error message: 'Forced retry: RATE_LIMIT' ``` */ code?: string | undefined; /** Original error that caused the retry. This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal. @example ``` try { const data = await response.json(); validateBusinessLogic(data); } catch (error) { return ky.retry({ code: 'VALIDATION_FAILED', cause: error // Preserves original error in chain }); } ``` */ cause?: Error | undefined; /** Custom request to use for the retry. This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed. **Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming. **Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded. @example ``` // Fallback to a different endpoint return ky.retry({ request: new Request('https://backup-api.com/endpoint', { method: request.method, headers: request.headers, }), code: 'BACKUP_ENDPOINT' }); // Retry with refreshed authentication token const data = await response.json(); return ky.retry({ request: new Request(request, { headers: { ...Object.fromEntries(request.headers), 'Authorization': `Bearer ${data.newToken}` } }), code: 'TOKEN_REFRESHED' }); ``` */ request?: Request | undefined; }
- sym:42231 Ky.#runAfterResponseHooks (source/core/Ky.ts:1234-1300)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:42358 findUnknownOptions (source/utils/options.ts:5-29)
  const findUnknownOptions = ( options: Record<string, unknown>, ): Record<string, unknown> =>
- sym:42415 createUploadProgressTestServer (test/stream.ts:1185-1195)
  const createUploadProgressTestServer = async (t: ExecutionContext) =>
- sym:42209 Ky.create (source/core/Ky.ts:268-472)
  static create(input: Input, options: Options): ResponsePromise
- sym:42302 Options (source/types/options.ts:448-520)
  interface Options extends KyOptions, RequestOptions
- sym:42397 createStreamFetch (test/hooks.ts:100-114)
  const createStreamFetch = ({ text = 'ok', onResponse, }: { text?: string; onResponse?: (response: Response) => void; } = {}): typeof fetch =>
- sym:42414 createAbortDuringFetch (test/retry.ts:4040-4056)
  const createAbortDuringFetch = (onRequest?: () => void): typeof fetch =>
- sym:42413 createAbortDuringBodyFetch (test/retry.ts:4019-4038)
  const createAbortDuringBodyFetch = (): typeof fetch =>

---
tokens: 5331/8000
