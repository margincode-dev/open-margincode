## Task
# ky-619c427 · ky-619c427 · open
Goal: Name the request that was sent on every `TimeoutError`
Workspace: /root/bench/av_tei/ws/rec09fc-ky-619c427/ky-619c427
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
- source/core/retry-timing.ts
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
- sym:42672 Ky.#getResponseData (source/core/Ky.ts:837-874)
  async #getResponseData(response: Response): Promise<unknown>
- sym:42674 Ky.#getEffectiveTimeout (source/core/Ky.ts:896-909)
  #getEffectiveTimeout(): number | undefined
- sym:42688 Ky.#retryFromError (source/core/Ky.ts:1303-1390)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:42816 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:42730 TimeoutError.constructor (source/errors/TimeoutError.ts:11-14)
  constructor(request: Request)
- sym:42821 isTimeoutError (source/utils/type-guards.ts:102-104)
  function isTimeoutError(error: unknown): error is TimeoutError
- sym:42685 Ky.#runBeforeRequestHooks (source/core/Ky.ts:1204-1225)
  async #runBeforeRequestHooks(): Promise<Response | undefined>
- sym:42669 Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
  async #calculateRetryDelay(error: unknown)
- sym:42654 validateTimeoutOption (source/core/Ky.ts:87-99)
  const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void =>
- sym:42715 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:42652 ErrorDataTimeout (source/core/Ky.ts:65-68)
  type ErrorDataTimeout = { milliseconds: number; fromTotalTimeout: boolean; }
- sym:42721 NetworkError (source/errors/NetworkError.ts:11-19)
  class NetworkError extends KyError
- sym:42725 ResponseSizeError (source/errors/ResponseSizeError.ts:20-30)
  class ResponseSizeError extends KyError
- sym:42744 Hooks (source/types/hooks.ts:90-371)
- sym:42815 TimeoutOptions (source/utils/timeout.ts:4-7)
  type TimeoutOptions = { timeout: number; fetch: InternalOptions['fetch']; }
- sym:42702 ForceRetryOptions (source/core/constants.ts:73-152)
  type ForceRetryOptions = { /** Custom delay in milliseconds before retrying. If not provided, uses the default retry delay calculation based on `retry.delay` configuration. **Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified. Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry. */ delay?: number | undefined; /** Error code for the retry. This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries. @example ``` return ky.retry({code: 'RATE_LIMIT'}); // Resulting error message: 'Forced retry: RATE_LIMIT' ``` */ code?: string | undefined; /** Original error that caused the retry. This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal. @example ``` try { const data = await response.json(); validateBusinessLogic(data); } catch (error) { return ky.retry({ code: 'VALIDATION_FAILED', cause: error // Preserves original error in chain }); } ``` */ cause?: Error | undefined; /** Custom request to use for the retry. This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed. **Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming. **Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded. @example ``` // Fallback to a different endpoint return ky.retry({ request: new Request('https://backup-api.com/endpoint', { method: request.method, headers: request.headers, }), code: 'BACKUP_ENDPOINT' }); // Retry with refreshed authentication token const data = await response.json(); return ky.retry({ request: new Request(request, { headers: { ...Object.fromEntries(request.headers), 'Authorization': `Bearer ${data.newToken}` } }), code: 'TOKEN_REFRESHED' }); ``` */ request?: Request | undefined; }
- sym:42679 Ky.#parseJson (source/core/Ky.ts:1148-1167)
  async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown>
- sym:42692 Ky.#getRemainingTotalTimeout (source/core/Ky.ts:1469-1476)
  #getRemainingTotalTimeout(): number | undefined
- sym:42676 Ky.#raceWithTotalTimeout (source/core/Ky.ts:981-1054)
  async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation>
- sym:42817 isErrorType (source/utils/type-guards.ts:9-10)
  const isErrorType = (error: unknown, cls: {name: string}): boolean =>
- sym:42664 Ky.create (source/core/Ky.ts:253-457)
  static create(input: Input, options: Options): ResponsePromise
- sym:42675 Ky.#raceBodyRead (source/core/Ky.ts:913-979)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42686 Ky.#runAfterResponseHooks (source/core/Ky.ts:1227-1293)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:42690 Ky.#fetch (source/core/Ky.ts:1398-1462)
  async #fetch(): Promise<Response>
- sym:42719 KyError (source/errors/KyError.ts:8-14)
  class KyError extends Error
- sym:42656 isRequestInstance (source/core/Ky.ts:130-131)
  const isRequestInstance = (value: unknown): value is Request =>
- sym:42657 isResponseInstance (source/core/Ky.ts:136-137)
  const isResponseInstance = (value: unknown): value is Response =>
- sym:42661 cloneInitHookOptions (source/core/Ky.ts:157-220)
  function cloneInitHookOptions(options: Options): InitOptions
- sym:42662 validateJsonWithSchema (source/core/Ky.ts:222-250)
  const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> =>
- sym:42663 Ky (source/core/Ky.ts:252-1550)
  class Ky
- sym:42666 Ky.constructor (source/core/Ky.ts:490-658)
  constructor(input: Input, options: Options = {})
- sym:42704 RetryMarker.isRetryMarker (source/core/constants.ts:159-161)
  get isRetryMarker(): true
- sym:42706 isRetryMarker (source/core/constants.ts:170-171)
  const isRetryMarker = (value: unknown): value is RetryMarker =>
- sym:42707 retry (source/core/constants.ts:256-256)
  const retry = (options?: ForceRetryOptions) =>
- sym:42710 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
  const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
- sym:42714 calculateRetryTimingDelay (source/core/retry-timing.ts:151-173)
  const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined =>
- sym:42717 HTTPError (source/errors/HTTPError.ts:15-34)
  class HTTPError<T = unknown> extends KyError
- sym:42723 NonError (source/errors/NonError.ts:6-28)
  class NonError extends Error
- sym:42731 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:42732 ResponsePromise (source/types/ResponsePromise.ts:7-69)
- sym:42746 KyInstance (source/types/ky.ts:5-204)
- sym:42751 Input (source/types/options.ts:19-19)
  type Input = string | URL | Request
- sym:42754 KyOptions (source/types/options.ts:45-432)
- sym:42757 Options (source/types/options.ts:448-502)
  interface Options extends KyOptions, RequestOptions
- sym:42758 InitOptions (source/types/options.ts:504-511)
  type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & { get headers(): Record<string, string | undefined>; set headers(value: KyHeadersInit | undefined); get context(): Record<string, unknown>; set context(value: Record<string, unknown> | undefined); retry?: MutableRetryOptions | number | undefined; hooks?: {[Key in keyof Hooks]?: NormalizedHooks[Key] | undefined} | undefined; }
- sym:42760 InternalOptions (source/types/options.ts:517-528)
  type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & { headers: Headers; hooks: NormalizedHooks; retry: NormalizedRetryOptions; fetch: NonNullable<Options['fetch']>; prefix: string; timeout: number | false; totalTimeout: number | false; maxResponseSize: number; context: Record<string, unknown>; throwHttpErrors: boolean | ((status: number) => boolean); }
- sym:42844 browserTest (test/helpers/with-page.ts:39-78)
  const browserTest = (title: string, browserTypes: BrowserType[], run: Run) =>

---
tokens: 3535/4000
