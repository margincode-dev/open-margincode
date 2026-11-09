## Task
# ky-619c427 · ky-619c427 · open
Goal: Name the request that was sent on every `TimeoutError`
Workspace: /root/bench/ws/rebf815-ky-619c427/ky-619c427
Changed (0): nothing yet

## Map
- source/errors/TimeoutError.ts
  L7: class TimeoutError extends KyError
  L11: constructor(request: Request)
- source/core/Ky.ts
  L65: type ErrorDataTimeout = { milliseconds: number; fromTotalTimeout: boolean; }
  L70: const createTextDecoder = (contentType: string): TextDecoder =>
  L87: const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void =>
  L101: const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number =>
  L130: const isRequestInstance = (value: unknown): value is Request =>
  L136: const isResponseInstance = (value: unknown): value is Response =>
  L139: const isAbsoluteInput = (input: string): boolean =>
  L142: const normalizeInputForProtocolCheck = (input: string): string =>
  L145: const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined =>
  L157: function cloneInitHookOptions(options: Options): InitOptions
  L222: const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> =>
  L252: class Ky
  L253: static create(input: Input, options: Options): ResponsePromise
  L460: static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption
  L490: constructor(input: Input, options: Options = {})
  L662: #validateCallbackOptions(): void
  L677: #calculateDelay(retry: InternalOptions['retry']): number
  L701: async #calculateRetryDelay(error: unknown)
  L783: #decorateResponse(response: Response): Response
  L805: async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
  L837: async #getResponseData(response: Response): Promise<unknown>
  L876: #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout
  L896: #getEffectiveTimeout(): number | undefined
  L913: async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
  L981: async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation>
  L1056: #isJsonContentType(contentType: string): boolean
  L1078: async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined>
  L1148: async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown>
  L1169: #cancelBody(body: ReadableStream | undefined): void
  L1178: #cancelResponseBody(response: Response): void
- source/errors/KyError.ts
  L8: class KyError extends Error
  L11: get isKyError(): true
- source/utils/timeout.ts
  L4: type TimeoutOptions = { timeout: number; fetch: InternalOptions['fetch']; }
  L10: async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- source/utils/type-guards.ts
  L9: const isErrorType = (error: unknown, cls: {name: string}): boolean =>
  L36: function isKyError(error: unknown): error is KyError
  L58: function isHTTPError<T = unknown>(error: unknown): error is HTTPError<T>
  L80: function isNetworkError(error: unknown): error is NetworkError
  L102: function isTimeoutError(error: unknown): error is TimeoutError
  L129: function isForceRetryError(error: unknown): error is ForceRetryError
  L152: function isResponseSizeError(error: unknown): error is ResponseSizeError
- source/core/constants.ts
  L40: const validate = <T extends Array<true>>() =>
  L73: type ForceRetryOptions = { /** Custom delay in milliseconds before retrying. If not provided, uses the default retry delay calculation based on `retry.delay` configuration. **Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified. Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry. */ delay?: number | undefined; /** Error code for the retry. This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries. @example ``` return ky.retry({code: 'RATE_LIMIT'}); // Resulting error message: 'Forced retry: RATE_LIMIT' ``` */ code?: string | undefined; /** Original error that caused the retry. This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal. @example ``` try { const data = await response.json(); validateBusinessLogic(data); } catch (error) { return ky.retry({ code: 'VALIDATION_FAILED', cause: error // Preserves original error in chain }); } ``` */ cause?: Error | undefined; /** Custom request to use for the retry. This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed. **Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming. **Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded. @example ``` // Fallback to a different endpoint return ky.retry({ request: new Request('https://backup-api.com/endpoint', { method: request.method, headers: request.headers, }), code: 'BACKUP_ENDPOINT' }); // Retry with refreshed authentication token const data = await response.json(); return ky.retry({ request: new Request(request, { headers: { ...Object.fromEntries(request.headers), 'Authorization': `Bearer ${data.newToken}` } }), code: 'TOKEN_REFRESHED' }); ``` */ request?: Request | undefined; }
  L157: class RetryMarker
  L159: get isRetryMarker(): true
  L165: constructor(options?: ForceRetryOptions)
  L170: const isRetryMarker = (value: unknown): value is RetryMarker =>
  L256: const retry = (options?: ForceRetryOptions) =>
- source/core/retry-timing.ts
  L11: type RetryTimingHeader = { value: string; allowTimestamp: boolean; }
  L16: type RetryTimingDateParts = { year: number; month: string; day: string; hours: string; minutes: string; seconds: string; }
  L25: const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
  L51: const createTimestamp = ({year, month, day, hours, minutes, seconds}: RetryTimingDateParts): number | undefined =>
  L86: const getCapture = (match: RegExpExecArray, index: number): string =>
  L88: const parseDate = (value: string): number | undefined =>
  L151: const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined =>
- source/errors/ForceRetryError.ts
  L10: class ForceRetryError extends KyError
  L16: constructor(options?: ForceRetryOptions)
- source/errors/HTTPError.ts
  L15: class HTTPError<T = unknown> extends KyError
  L22: constructor(response: Response, request: Request, options: Readonly<NormalizedOptions>)
- source/errors/NetworkError.ts
  L11: class NetworkError extends KyError
  L15: constructor(request: Request, options?: {cause?: Error | undefined})
- source/errors/NonError.ts
  L6: class NonError extends Error
  L10: constructor(value: unknown)
- source/errors/ResponseSizeError.ts
  L20: class ResponseSizeError extends KyError
  L25: constructor(request: Request, maxResponseSize: number)
- source/index.ts
  L10: const createInstance = (defaults?: Partial<Options>): KyInstance =>
- source/types/ResponsePromise.ts
  L7: type ResponsePromise<T = unknown> = { arrayBuffer: () => Promise<ArrayBuffer>; blob: () => Promise<Blob>; formData: () => Promise<FormData>; /** Get the response body as raw bytes. Note: This shortcut is only available when the runtime supports `Response.prototype.bytes()`. */ bytes: () => Promise<Uint8Array<ArrayBuffer>>; // TODO: Use `json<T extends JSONValue>(): Promise<T>;` when it's fixed in TS. // See https://github.com/microsoft/TypeScript/issues/15300 and https://github.com/sindresorhus/ky/pull/80 json: { /** Get the response body as JSON. @example ``` import ky from 'ky'; const json = await ky(…).json(); ``` @example ``` import ky from 'ky'; interface Result { value: number; } const result1 = await ky(…).json<Result>(); // or const result2 = await ky<Result>(…).json(); ``` */ <JsonType = T>(schema?: undefined): Promise<JsonType>; /** Get the response body as JSON and validate it with a Standard Schema. Use a Standard Schema compatible validator (for example, Zod 3.24+). Throws a `SchemaValidationError` when validation fails. @example ``` import ky from 'ky'; import {z} from 'zod'; const userSchema = z.object({name: z.string()}); const user = await ky('/api/user').json(userSchema); ``` */ <Schema extends StandardSchemaV1>(schema: Schema): Promise<StandardSchemaV1InferOutput<Schema>>; }; text: () => Promise<string>; } & Promise<KyResponse<T>>
- source/types/ky.ts
- source/types/options.ts
- repo map: .editorconfig .gitattributes .github/ .gitignore .npmrc AGENTS.md license media/ package.json readme.md source/ test-d/ (106 files)

## Code
- sym:42729 TimeoutError (source/errors/TimeoutError.ts:7-15)
  class TimeoutError extends KyError
- sym:42673 Ky.#getErrorDataTimeout (source/core/Ky.ts:876-894)
  #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout
- sym:42684 Ky.#throwIfTotalTimeoutExhausted (source/core/Ky.ts:1197-1202)
  #throwIfTotalTimeoutExhausted(request: Request = this.request): void
- sym:42821 isTimeoutError (source/utils/type-guards.ts:102-104)
  function isTimeoutError(error: unknown): error is TimeoutError
- sym:42674 Ky.#getEffectiveTimeout (source/core/Ky.ts:896-909)
  #getEffectiveTimeout(): number | undefined
- sym:42688 Ky.#retryFromError (source/core/Ky.ts:1303-1390)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:42669 Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
  async #calculateRetryDelay(error: unknown)
- sym:42816 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:42690 Ky.#fetch (source/core/Ky.ts:1398-1462)
  async #fetch(): Promise<Response>
- sym:42730 TimeoutError.constructor (source/errors/TimeoutError.ts:11-14)
  constructor(request: Request)
- sym:42685 Ky.#runBeforeRequestHooks (source/core/Ky.ts:1204-1225)
  async #runBeforeRequestHooks(): Promise<Response | undefined>
- sym:42721 NetworkError (source/errors/NetworkError.ts:11-19)
  class NetworkError extends KyError
- sym:42675 Ky.#raceBodyRead (source/core/Ky.ts:913-979)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42717 HTTPError (source/errors/HTTPError.ts:15-34)
  class HTTPError<T = unknown> extends KyError
- sym:42715 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:42784 isRawNetworkError (source/utils/is-network-error.ts:15-46)
  function isRawNetworkError(error: unknown): error is TypeError
- sym:42672 Ky.#getResponseData (source/core/Ky.ts:837-874)
  async #getResponseData(response: Response): Promise<unknown>
- sym:42719 KyError (source/errors/KyError.ts:8-14)
  class KyError extends Error
- sym:42686 Ky.#runAfterResponseHooks (source/core/Ky.ts:1227-1293)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:42664 Ky.create (source/core/Ky.ts:253-457)
  static create(input: Input, options: Options): ResponsePromise
- sym:42652 ErrorDataTimeout (source/core/Ky.ts:65-68)
  type ErrorDataTimeout = { milliseconds: number; fromTotalTimeout: boolean; }
- sym:42654 validateTimeoutOption (source/core/Ky.ts:87-99)
  const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void =>
- sym:42725 ResponseSizeError (source/errors/ResponseSizeError.ts:20-30)
  class ResponseSizeError extends KyError
- sym:42744 Hooks (source/types/hooks.ts:90-371)
  type Hooks = { /** This hook enables you to modify the options before they are used to construct the request. The hook function receives the mutable options object and can modify it in place. You could, for example, modify `searchParams`, `headers`, or `json` here. The `headers` option is always a plain object with lowercase names, where a header removed with `undefined` keeps an `undefined` value. Unlike other hooks, `init` hooks are synchronous. Any error thrown will propagate synchronously and will not be caught by `beforeError` hooks. The `body` option is passed through by reference, so mutating a `FormData` or `URLSearchParams` body in place changes your own object and repeats on every request from the same instance. Replace `options.body` with a new instance instead, the way the `beforeRequest` workflow does. A common use case is to add a search parameter to every request: @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { init: [ options => { options.searchParams = {apiKey: getApiKey()}; }, ], }, }); const response = await api.get('https://example.com/api/users'); // URL: https://example.com/api/users?apiKey=123 ``` @default [] */ init?: readonly InitHook[] | undefined; /** This hook enables you to modify the request right before it is sent. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, and retry count. You could, for example, modify `request.headers` here. The `retryCount` is always `0`, since `beforeRequest` hooks run once before retry handling begins. The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing request (remaining hooks will still run with the updated request). It can also return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to completely avoid making an HTTP request, in which case remaining `beforeRequest` hooks are skipped. This can be used to mock a request, check an internal cache, etc. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. Any error thrown by `beforeRequest` hooks is treated as fatal and will not trigger Ky's retry logic. @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { beforeRequest: [ ({request}) => { request.headers.set('Authorization', 'token initial-token'); } ] } }); const response = await api.get('https://example.com/api/users'); ``` **Modifying the request URL:** @example ``` import ky from 'ky'; const api = ky.extend({ hooks: { beforeRequest: [ ({request}) => { const url = new URL(request.url); url.searchParams.set('token', 'secret-token'); return new Request(url, request); } ] } }); const response = await api.get('https://example.com/api/users'); ``` @default [] */ beforeRequest?: readonly BeforeRequestHook[] | undefined; /** This hook enables you to modify the request right before retry. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, an error instance, and retry count. You could, for example, modify `request.headers` here. The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing retry request, or return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to skip the retry and use that response instead. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. **Note:** Returning a request or response skips remaining `beforeRetry` hooks. **Warning:** Returned `Request` objects are used as-is. If you point one at another origin, remove any credentials you do not want forwarded. The `retryCount` is always `>= 1`, since this hook is only called during retries, not on the initial request. If the request received a response, the error will be of type `HTTPError`. The `Response` object will be available at `error.response`, and the pre-parsed response body may be available at `error.data`. Be aware that some types of errors, such as network errors, inherently mean that a response was not received. In that case, the error will be an instance of `NetworkError` instead of `HTTPError`. You can prevent Ky from retrying the request by throwing an error. Ky will not handle it in any way and the error will be propagated to the request initiator. The rest of the `beforeRetry` hooks will not be called in this case. Alternatively, you can return the [`ky.stop`](#kystop) symbol to do the same thing but without propagating an error (this has some limitations, see `ky.stop` docs for details). **Modifying headers:** @example ``` import ky from 'ky'; const response = await ky('https://example.com', { hooks: { beforeRetry: [ async ({request, options, error, retryCount}) => { const token = await ky('https://example.com/refresh-token'); request.headers.set('Authorization', `token ${token}`); } ] } }); ``` **Modifying the request URL:** @example ``` import ky, {isHTTPError} from 'ky'; const response = await ky('https://example.com/api', { hooks: { beforeRetry: [ ({request, error}) => { // Add query parameters based on error response if ( isHTTPError(error) && typeof error.data === 'object' && error.data !== null && 'processId' in error.data ) { const url = new URL(request.url); url.searchParams.set('processId', String(error.data.processId)); return new Request(url, request); } } ] } }); ``` **Returning a cached response:** @example ``` import ky from 'ky'; const response = await ky('https://example.com/api', { hooks: { beforeRetry: [ ({error, retryCount}) => { // Use cached response instead of retrying if (retryCount > 1 && cachedResponse) { return cachedResponse; } } ] } }); ``` @default [] */ beforeRetry?: readonly BeforeRetryHook[] | undefined; /** This hook enables you to modify any error right before it is thrown. The hook function receives a state object with the current request, the normalized Ky options, the error, and retry count, and should return an `Error` instance. This hook receives errors Ky handles during its request lifecycle, including `HTTPError`, `NetworkError`, `TimeoutError`, `ResponseSizeError`, and `ForceRetryError` (when retry limit is exceeded via `ky.retry()`). Use type guards like `isHTTPError()`, `isNetworkError()`, `isTimeoutError()`, or `isResponseSizeError()` to handle specific error types. Errors raised while consuming an already returned `Response` are outside the request lifecycle. Use a body method shortcut like `await ky(url).json()` to process body-read `ResponseSizeError`, `NetworkError`, and `TimeoutError` instances through this hook. The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). If a `beforeRequest` or `beforeRetry` hook returns a new `Request`, inspect `request` for the final request state. `options` remains Ky's normalized options and may not mirror every property of a replacement `Request`. @default [] @example ``` import ky, {isHTTPError} from 'ky'; await ky('https://example.com', { hooks: { beforeError: [ ({request, options, error}) => { if (isHTTPError(error)) { if ( typeof error.data === 'object' && error.data !== null && 'message' in error.data ) { error.name = 'GitHubError'; error.message = `${String(error.data.message)} (${error.response.status})`; } } // `request` and `options` are always available console.log(`Request to ${request.url} failed`, options.context); return error; } ] } }); ``` */ beforeError?: readonly BeforeErrorHook[] | undefined; /** This hook enables you to read and optionally modify the response. The hook function receives a state object with the normalized request, options, a clone of the response, and retry count. The return value of the hook function will be used by Ky as the response object if it's an instance of [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response). You can also force a retry by returning [`ky.retry(options)`](#kyretryoptions). This is useful when you need to retry based on the response body content, even if the response has a successful status code. The retry will respect the `retry.limit` option and be observable in `beforeRetry` hooks. **Warning:** `ky.retry({request})` uses the replacement request as-is. If it targets another origin, remove any credentials you do not want forwarded. Any non-`ky.retry()` error thrown by `afterResponse` hooks is treated as fatal and will not trigger Ky's retry logic. The `retryCount` is `0` for the initial request and increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). @default [] @example ``` import ky from 'ky'; const response = await ky('https://example.com', { hooks: { afterResponse: [ ({response}) => { // You could do something with the response, for example, logging. log(response); // Or return a `Response` instance to overwrite the response. return new Response('A different response', {status: 200}); }, // Or retry with a fresh token on a 401 error async ({request, response, retryCount}) => { if (response.status === 401 && retryCount === 0) { // Only refresh on first 401, not on subsequent retries const {token} = await ky.post('https://example.com/auth/refresh').json(); const headers = new Headers(request.headers); headers.set('Authorization', `Bearer ${token}`); return ky.retry({ request: new Request(request, {headers}), code: 'TOKEN_REFRESHED' }); } }, // Or force retry based on response body content async ({response}) => { if (response.status === 200) { const data = await response.json(); if (data.error?.code === 'RATE_LIMIT') { // Retry with custom delay from API response return ky.retry({ delay: data.error.retryAfter * 1000, code: 'RATE_LIMIT' }); } } }, // Or show a notification only on the last retry for 5xx errors ({options, response, retryCount}) => { if (response.status >= 500 && response.status <= 599) { if (retryCount === options.retry.limit) { showNotification('Request failed after all retries'); } } } ] } }); ``` */ afterResponse?: readonly AfterResponseHook[] | undefined; }
- sym:42710 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
  const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
- sym:42722 NetworkError.constructor (source/errors/NetworkError.ts:15-18)
  constructor(request: Request, options?: {cause?: Error | undefined})
- sym:42692 Ky.#getRemainingTotalTimeout (source/core/Ky.ts:1469-1476)
  #getRemainingTotalTimeout(): number | undefined
- sym:42663 Ky (source/core/Ky.ts:252-1550)
  class Ky
- sym:42817 isErrorType (source/utils/type-guards.ts:9-10)
  const isErrorType = (error: unknown, cls: {name: string}): boolean =>
- sym:42723 NonError (source/errors/NonError.ts:6-28)
  class NonError extends Error
- sym:42666 Ky.constructor (source/core/Ky.ts:490-658)
  constructor(input: Input, options: Options = {})
- sym:42815 TimeoutOptions (source/utils/timeout.ts:4-7)
  type TimeoutOptions = { timeout: number; fetch: InternalOptions['fetch']; }
- sym:42702 ForceRetryOptions (source/core/constants.ts:73-152)
  type ForceRetryOptions = { /** Custom delay in milliseconds before retrying. If not provided, uses the default retry delay calculation based on `retry.delay` configuration. **Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified. Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry. */ delay?: number | undefined; /** Error code for the retry. This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries. @example ``` return ky.retry({code: 'RATE_LIMIT'}); // Resulting error message: 'Forced retry: RATE_LIMIT' ``` */ code?: string | undefined; /** Original error that caused the retry. This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal. @example ``` try { const data = await response.json(); validateBusinessLogic(data); } catch (error) { return ky.retry({ code: 'VALIDATION_FAILED', cause: error // Preserves original error in chain }); } ``` */ cause?: Error | undefined; /** Custom request to use for the retry. This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed. **Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming. **Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded. @example ``` // Fallback to a different endpoint return ky.retry({ request: new Request('https://backup-api.com/endpoint', { method: request.method, headers: request.headers, }), code: 'BACKUP_ENDPOINT' }); // Retry with refreshed authentication token const data = await response.json(); return ky.retry({ request: new Request(request, { headers: { ...Object.fromEntries(request.headers), 'Authorization': `Bearer ${data.newToken}` } }), code: 'TOKEN_REFRESHED' }); ``` */ request?: Request | undefined; }
- sym:42668 Ky.#calculateDelay (source/core/Ky.ts:677-699)
  #calculateDelay(retry: InternalOptions['retry']): number
- sym:42718 HTTPError.constructor (source/errors/HTTPError.ts:22-33)
  constructor(response: Response, request: Request, options: Readonly<NormalizedOptions>)
- sym:42740 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:42676 Ky.#raceWithTotalTimeout (source/core/Ky.ts:981-1054)
  async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation>
- sym:42708 RetryTimingHeader (source/core/retry-timing.ts:11-14)
  type RetryTimingHeader = { value: string; allowTimestamp: boolean; }
- sym:42727 SchemaValidationError (source/errors/SchemaValidationError.ts:25-33)
  class SchemaValidationError extends Error
- sym:42697 Ky.#getResponseRequest (source/core/Ky.ts:1529-1531)
  #getResponseRequest(response: Response): Request
- sym:42765 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:42766 RetryOptions (source/types/retry.ts:21-187)
- sym:42812 normalizeRetryOptions (source/utils/normalize.ts:52-109)
  const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions =>
- sym:42656 isRequestInstance (source/core/Ky.ts:130-131)
  const isRequestInstance = (value: unknown): value is Request =>
- sym:42657 isResponseInstance (source/core/Ky.ts:136-137)
  const isResponseInstance = (value: unknown): value is Response =>
- sym:42661 cloneInitHookOptions (source/core/Ky.ts:157-220)
- sym:42662 validateJsonWithSchema (source/core/Ky.ts:222-250)
- sym:42704 RetryMarker.isRetryMarker (source/core/constants.ts:159-161)
- sym:42706 isRetryMarker (source/core/constants.ts:170-171)
- sym:42707 retry (source/core/constants.ts:256-256)
- sym:42714 calculateRetryTimingDelay (source/core/retry-timing.ts:151-173)
- sym:42731 createInstance (source/index.ts:10-33)
- sym:42732 ResponsePromise (source/types/ResponsePromise.ts:7-69)
- sym:42746 KyInstance (source/types/ky.ts:5-204)
- sym:42751 Input (source/types/options.ts:19-19)
- sym:42754 KyOptions (source/types/options.ts:45-432)
- sym:42757 Options (source/types/options.ts:448-502)
- sym:42758 InitOptions (source/types/options.ts:504-511)
- sym:42760 InternalOptions (source/types/options.ts:517-528)
- sym:42844 browserTest (test/helpers/with-page.ts:39-78)
- sym:42865 withCapturedTimeouts (test/retry.ts:149-167)
- sym:42848 withHeader (test/hooks.ts:17-21)
- sym:42843 promiseWithTimeout (test/helpers/with-page.ts:19-37)
- sym:42869 createAbortDuringFetch (test/retry.ts:4040-4056)

---
tokens: 7185/8000
