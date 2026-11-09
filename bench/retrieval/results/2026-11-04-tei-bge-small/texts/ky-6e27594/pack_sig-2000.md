## Task
# ky-6e27594 · ky-6e27594 · open
Goal: Type-check `onDownloadProgress` before the request is sent
Workspace: /root/bench/av_tei/ws/rec09fc-ky-6e27594/ky-6e27594
Changed (0): nothing yet

## Map
- repo map (107 files):
  .editorconfig
  .gitattributes
  .github/ (3 files)
  .gitignore
  .npmrc
  AGENTS.md
  license
  media/ (3 files)
  package.json
  readme.md
  source/ (31 files)
  test-d/ (21 files)
  test/ (38 files)
  tsconfig.dist.json
  tsconfig.json
  tsconfig.test.json

## Code
- sym:42422 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
  type ExpectedProgressCallbacks = { readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>; readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>; }
- sym:42321 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:42325 streamResponse (source/utils/body.ts:197-212)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:42245 Ky.#wrapRequestWithUploadProgress (source/core/Ky.ts:1553-1559)
  #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request
- sym:42306 NormalizedOptions (source/types/options.ts:551-564)
  interface NormalizedOptions extends Readonly<RequestInit>
- sym:42326 streamRequest (source/utils/body.ts:215-234)
  const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) =>
- sym:42323 copyResponseMetadata (source/utils/body.ts:107-170)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42247 ForceRetryOptions (source/core/constants.ts:73-152)
  type ForceRetryOptions = { /** Custom delay in milliseconds before retrying. If not provided, uses the default retry delay calculation based on `retry.delay` configuration. **Note:** Custom delays bypass jitter and `backoffLimit`. This is intentional, as custom delays often come from server responses (e.g., `Retry-After` headers) and should be respected exactly as specified. Must be a non-negative number, or `undefined`; anything else throws a `TypeError` rather than collapsing to a 1ms retry. */ delay?: number | undefined; /** Error code for the retry. This machine-readable identifier will be included in the error message passed to `beforeRetry` hooks, allowing you to distinguish between different types of forced retries. @example ``` return ky.retry({code: 'RATE_LIMIT'}); // Resulting error message: 'Forced retry: RATE_LIMIT' ``` */ code?: string | undefined; /** Original error that caused the retry. This allows you to preserve the error chain when forcing a retry based on caught exceptions. The error will be set as the `cause` of the `ForceRetryError`, enabling proper error chain traversal. @example ``` try { const data = await response.json(); validateBusinessLogic(data); } catch (error) { return ky.retry({ code: 'VALIDATION_FAILED', cause: error // Preserves original error in chain }); } ``` */ cause?: Error | undefined; /** Custom request to use for the retry. This allows you to modify or completely replace the request during a forced retry. The custom request becomes the starting point for the retry - `beforeRetry` hooks can still further modify it if needed. **Note:** The custom request's `signal` will be replaced with Ky's managed signal to handle timeouts and user-provided abort signals correctly. If the original request body has been consumed, you must provide a new body or clone the request before consuming. **Warning:** Custom retry requests are not sanitized. If you reuse headers across origins, remove any credentials you do not want forwarded. @example ``` // Fallback to a different endpoint return ky.retry({ request: new Request('https://backup-api.com/endpoint', { method: request.method, headers: request.headers, }), code: 'BACKUP_ENDPOINT' }); // Retry with refreshed authentication token const data = await response.json(); return ky.retry({ request: new Request(request, { headers: { ...Object.fromEntries(request.headers), 'Authorization': `Bearer ${data.newToken}` } }), code: 'TOKEN_REFRESHED' }); ``` */ request?: Request | undefined; }
- sym:42289 Hooks (source/types/hooks.ts:90-371)
- sym:42215 Ky.#throwProcessedError (source/core/Ky.ts:806-836)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42281 BeforeRequestState (source/types/hooks.ts:32-40)
  type BeforeRequestState = { request: KyRequest; options: Readonly<NormalizedOptions>; /** The number of retries attempted. Always `0`, since `beforeRequest` hooks run once before retry handling begins. */ retryCount: 0; }
- sym:42282 BeforeRequestHook (source/types/hooks.ts:42-42)
  type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>
- sym:42322 getProgressCallbackError (source/utils/body.ts:104-105)
  const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
- sym:42209 Ky.create (source/core/Ky.ts:268-472)
  static create(input: Input, options: Options): ResponsePromise
- sym:42297 Progress (source/types/options.ts:21-36)
  type Progress = { /** A number between `0` and `1` representing the progress percentage. */ percent: number; /** The number of bytes transferred so far. */ transferredBytes: number; /** The total number of bytes to be transferred. This is an estimate and may be `0` for an empty transfer or when the total size cannot be determined. */ totalBytes: number; }
- sym:42302 Options (source/types/options.ts:448-520)
  interface Options extends KyOptions, RequestOptions
- sym:42415 createUploadProgressTestServer (test/stream.ts:1185-1195)
  const createUploadProgressTestServer = async (t: ExecutionContext) =>

---
tokens: 1460/2000
