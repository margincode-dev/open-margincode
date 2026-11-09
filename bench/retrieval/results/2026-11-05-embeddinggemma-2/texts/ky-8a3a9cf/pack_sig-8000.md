## Task
# ky-8a3a9cf · ky-8a3a9cf · open
Goal: Advertise the shortcut media type on the attempt that produced the response
Workspace: /root/bench/av_gemma/ws/rec728b-ky-8a3a9cf/ky-8a3a9cf
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
- sym:42990 KyResponse (source/types/response.ts:1-4)
  type KyResponse<T = unknown> = { clone: () => KyResponse<T>; json: <J = T>() => Promise<J>; } & Response
- sym:42970 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:42959 ResponsePromise (source/types/ResponsePromise.ts:7-69)
  type ResponsePromise<T = unknown> = { arrayBuffer: () => Promise<ArrayBuffer>; blob: () => Promise<Blob>; formData: () => Promise<FormData>; /** Get the response body as raw bytes. Note: This shortcut is only available when the runtime supports `Response.prototype.bytes()`. */ bytes: () => Promise<Uint8Array<ArrayBuffer>>; // TODO: Use `json<T extends JSONValue>(): Promise<T>;` when it's fixed in TS. // See https://github.com/microsoft/TypeScript/issues/15300 and https://github.com/sindresorhus/ky/pull/80 json: { /** Get the response body as JSON. @example ``` import ky from 'ky'; const json = await ky(…).json(); ``` @example ``` import ky from 'ky'; interface Result { value: number; } const result1 = await ky(…).json<Result>(); // or const result2 = await ky<Result>(…).json(); ``` */ <JsonType = T>(schema?: undefined): Promise<JsonType>; /** Get the response body as JSON and validate it with a Standard Schema. Use a Standard Schema compatible validator (for example, Zod 3.24+). Throws a `SchemaValidationError` when validation fails. @example ``` import ky from 'ky'; import {z} from 'zod'; const userSchema = z.object({name: z.string()}); const user = await ky('/api/user').json(userSchema); ``` */ <Schema extends StandardSchemaV1>(schema: Schema): Promise<StandardSchemaV1InferOutput<Schema>>; }; text: () => Promise<string>; } & Promise<KyResponse<T>>
- sym:42969 AfterResponseState (source/types/hooks.ts:75-86)
  type AfterResponseState = { request: KyRequest; options: Readonly<NormalizedOptions>; response: KyResponse; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). */ retryCount: number; }
- sym:42966 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:42964 BeforeRequestHook (source/types/hooks.ts:42-42)
  type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>
- sym:42924 Ky.#getResponseRequest (source/core/Ky.ts:1525-1527)
  #getResponseRequest(response: Response): Request
- sym:42897 Ky.#decorateResponse (source/core/Ky.ts:779-797)
  #decorateResponse(response: Response): Response
- sym:42925 Ky.#setResponseRequest (source/core/Ky.ts:1529-1532)
  #setResponseRequest(response: Response, request: Request): Response
- sym:42899 Ky.#getResponseData (source/core/Ky.ts:833-870)
  async #getResponseData(response: Response): Promise<unknown>
- sym:42981 KyOptions (source/types/options.ts:45-432)
  type KyOptions = { /** Shortcut for sending JSON. Use this instead of the `body` option. Accepts any plain object or value, which will be stringified using `JSON.stringify()` and sent in the body with the correct header set. The `Content-Type` header is set to `application/json` unless you set a `Content-Type` in the `headers` option, which always takes precedence. A `Content-Type` on a `Request` input is replaced, because the `Request` constructor sets one automatically from its body. */ json?: unknown; /** User-defined JSON-parsing function. The function receives the response text as the first argument and a context object as the second argument containing the `request` and `response`. Use-cases: 1. Parse JSON via the [`bourne` package](https://github.com/hapijs/bourne) to protect from prototype pollution. 2. Parse JSON with [`reviver` option of `JSON.parse()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse). 3. Log or handle JSON parse errors with request context. @default JSON.parse() @example ``` import ky from 'ky'; import bourne from '@hapijs/bourne'; const json = await ky('https://example.com', { parseJson: text => bourne(text) }).json(); ``` @example ``` import ky from 'ky'; const json = await ky('https://example.com', { parseJson: (text, {request, response}) => { console.log(`Parsing JSON from ${request.url} (status: ${response.status})`); return JSON.parse(text); } }).json(); ``` */ // `options` is intentionally not included in the context to avoid exposing Ky internals through a parsing callback. `request`/`response` already provide the metadata needed for logging. parseJson?: ((text: string, context: {request: Request; response: Response}) => unknown) | undefined; /** User-defined JSON-stringifying function. Use-cases: 1. Stringify JSON with a custom `replacer` function. @default JSON.stringify() @example ``` import ky from 'ky'; import {DateTime} from 'luxon'; const json = await ky('https://example.com', { stringifyJson: data => JSON.stringify(data, (key, value) => { if (key.endsWith('_at')) { return DateTime.fromISO(value).toSeconds(); } return value; }) }).json(); ``` */ stringifyJson?: ((data: unknown) => string) | undefined; /** Search parameters to include in the request URL. Setting this will merge with any existing search parameters in the input URL. Accepts any value supported by [`URLSearchParams()`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams). When passing an object, setting a value to `undefined` deletes the parameter, including from the input URL, even when a later option layer adds the parameter again. `null` values are preserved and converted to the string `'null'`. When `input` is a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) with a body, the body is sent as a stream, which requires [request stream support](https://caniuse.com/wf-fetch-request-streams) and, in Chromium-based browsers, an HTTP/2 or HTTP/3 connection (streaming uploads over HTTP/1.1 fail with a network error, even over plain HTTP). The inherited body is dropped when the search parameters change the URL and the request has to be rebuilt in an environment that cannot reuse it, which is the case without request stream support, when `keepalive` is true, or when the effective mode is `'no-cors'`. A search parameter value that leaves the URL unchanged does not rebuild the request, so the body is kept. A compatible body passed explicitly with the `body` option is still used. */ searchParams?: SearchParamsOption; /** A base URL to [resolve](https://developer.mozilla.org/en-US/docs/Web/API/URL_API/Resolving_relative_references) the `input` against. When the `input` (after applying the `prefix` option) is only a relative URL, such as `'users'`, `'/users'`, or `'//my-site.com'`, it will be resolved against the `baseUrl` to determine the destination of the request. Otherwise, the `input` is absolute, such as `'https://my-site.com'`, and it will bypass the `baseUrl`. Useful when used with [`ky.extend()`](#kyextenddefaultoptions) to create niche-specific Ky instances. If the `baseUrl` itself is relative, it will be resolved against the environment's base URL, such as [`document.baseURI`](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI) in browsers or `location.href` in Deno (see the `--location` flag). **Tip:** When setting a `baseUrl` that has a path, we recommend that it include a trailing slash `/`, as in `'/api/'` rather than `/api`. This ensures more intuitive behavior for page-relative `input` URLs, such as `'users'` or `'./users'`, where they will _extend_ from the full path of the `baseUrl` rather than _replacing_ its last path segment. @example ``` import ky from 'ky'; // On https://example.com const response = await ky('users', {baseUrl: '/api/'}); //=> 'https://example.com/api/users' const response = await ky('/users', {baseUrl: '/api/'}); //=> 'https://example.com/users' ``` */ baseUrl?: URL | string | undefined; /** A prefix to prepend to the `input` before making the request (and before it is resolved against the `baseUrl`). It can be any valid path or URL, either relative or absolute. A trailing slash `/` is optional and will be added automatically, if needed, when it is joined with `input`. Only takes effect when `input` is a string. Useful when used with [`ky.extend()`](#kyextenddefaultoptions) to create niche-specific Ky instances. *In most cases, you should use the `baseUrl` option instead, as it is more consistent with web standards. However, `prefix` is useful if you want origin-relative `input` URLs, such as `/users`, to be treated as if they were page-relative. In other words, the leading slash of the `input` will essentially be ignored, because the `prefix` will become part of the `input` before URL resolution happens.* Notes: - The `prefix` and `input` are joined with a slash `/`, and slashes are normalized at the join boundary by trimming trailing slashes from `prefix` and leading slashes from `input`. - After `prefix` and `input` are joined, the result is resolved against the `baseUrl` option, if present. @example ``` import ky from 'ky'; // On https://example.com const response = await ky('users', {prefix: '/api/'}); //=> 'https://example.com/api/users' const response = await ky('/users', {prefix: '/api/'}); //=> 'https://example.com/api/users' ``` */ prefix?: URL | string | undefined; /** Controls retry behavior. Each field is documented in the `RetryOptions` type. If `retry` is a number, it will be used as `limit` and other defaults will remain in place. `retry.limit`, including numeric shorthand, must be a finite, non-negative integer. Network errors (e.g., DNS failures, connection refused, offline) are automatically retried for retriable methods. Only errors recognized as network errors are retried; other errors (e.g., programming bugs) are thrown immediately. Use `shouldRetry` to customize this behavior. `413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`. When the response status is contained in `afterStatusCodes` and the retry is allowed by `statusCodes` or `shouldRetry`, Ky uses retry timing headers to choose the retry delay. [`Retry-After`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Retry-After) may provide a delay in seconds or an HTTP-date. If `Retry-After` is missing, Ky falls back to rate-limit timing headers (`RateLimit-Reset`, `X-RateLimit-Retry-After`, `X-RateLimit-Reset`, and `X-Rate-Limit-Reset`). Numeric `Retry-After` and `X-RateLimit-Retry-After` values are interpreted as delay seconds. Numeric `RateLimit-Reset`, `X-RateLimit-Reset`, and `X-Rate-Limit-Reset` values may also be interpreted as Unix timestamps, from 2001-09-09 onwards. A value below that is read as delay seconds, and a timestamp already in the past means the retry happens immediately. If the status code is not in `afterStatusCodes`, retry timing headers will be ignored. If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`. @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 10, methods: ['get'], statusCodes: [500] } }).json(); ``` */ retry?: RetryOptions | number | undefined; /** Per-attempt timeout in milliseconds for getting a response, applied independently to each retry. Ky shortcut methods also use this value as a separate timeout for reading the response body. Cannot be greater than 2147483647. See also `totalTimeout`. If set to `false`, there will be no per-attempt timeout. @default 10000 */ timeout?: number | false | undefined; /** Overall timeout in milliseconds for the entire operation, including retries and delays. Throws a `TimeoutError` if exceeded. Cannot be greater than 2147483647. `beforeError` hooks run after an error is produced and are not bounded by `totalTimeout`. If set to `false` or not specified, there is no overall timeout. @default false @example ``` import ky from 'ky'; // Each attempt gets 5s, but the whole operation must complete within 30s const json = await ky('https://example.com', { timeout: 5000, totalTimeout: 30_000, retry: { limit: 3, retryOnTimeout: true, } }).json(); ``` */ totalTimeout?: number | false | undefined; /** Maximum response body size in bytes. Must be a non-negative safe integer or `Infinity`. Set to `0` to allow only empty bodies. The limit counts bytes from the response stream after decompression, independently of `Content-Length`. It applies as the body is consumed, including in `afterResponse` hooks and for responses returned by hooks. Exceeding the limit cancels the stream and throws a `ResponseSizeError`, without automatically retrying. With `await ky(url)`, the response can resolve before the limit is exceeded; the body read will reject instead. This limits body bytes, not total memory usage. Parsing, buffering, and concurrent requests can use additional memory. Responses with a [null body status](https://fetch.spec.whatwg.org/#null-body-status), such as `204` and `205`, are never wrapped, because the `Response` constructor rejects a body for those statuses. A runtime that still exposes a body for such a status, such as WebKit for `205`, is therefore not limited. @default Infinity @example ``` import ky from 'ky'; const data = await ky('https://example.com/data', { maxResponseSize: 20 * 1024 * 1024, }).json(); ``` */ maxResponseSize?: number | undefined; /** Hooks allow modifications during the request lifecycle. Hook functions may be async and are run serially, unless otherwise noted. */ hooks?: Hooks | undefined; /** Throw an `HTTPError` when, after following redirects, the response has a non-2xx status code. To also throw for redirects instead of following them, set the [`redirect`](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch#Parameters) option to `'manual'`. Setting this to `false` may be useful if you are checking for resource availability and are expecting error responses. You can also pass a function that accepts the HTTP status code and returns a boolean for selective error handling. Note that this can violate the principle of least surprise, so it's recommended to use the boolean form unless you have a specific use case like treating 404 responses differently. Note: If `false`, error responses are considered successful and the request will not be retried. Note: [Opaque responses](https://developer.mozilla.org/en-US/docs/Web/API/Response/type) from `no-cors` requests are returned as-is (without throwing `HTTPError`), since the actual status is hidden by the browser. @default true */ throwHttpErrors?: boolean | ((status: number) => boolean) | undefined; /** Download progress event handler. @param progress - Object containing download progress information. @param chunk - Data that was received. When an empty response body stream completes, the callback receives an empty chunk. `content-length` is only used as the total for a response that is not content-coded, since it counts encoded bytes while the progress stream counts the bytes after decompression. A compressed response therefore reports `totalBytes: 0` until it completes, which means the percentage cannot be calculated while downloading. Responses with no body at all are not streamed, so no progress events are emitted for them. That covers a [null body status](https://fetch.spec.whatwg.org/#null-body-status) such as `204`, and a `HEAD` request, which has no body even on an ordinary status. When the callback throws, the error is reported by Ky's body method shortcuts. A response read directly with `response.text()` or `response.json()` may instead report the runtime's own stream error, such as Chromium's generic `TypeError`. @example ``` import ky from 'ky'; const response = await ky('https://example.com', { onDownloadProgress: (progress, chunk) => { // Example output: // `100% - 1271 of 1271 bytes` console.log(`${progress.percent * 100}% - ${progress.transferredBytes} of ${progress.totalBytes} bytes`); } }); ``` */ onDownloadProgress?: ((progress: Progress, chunk: Uint8Array) => void) | undefined; /** Upload progress event handler. Note: Requires [request stream support](https://caniuse.com/wf-fetch-request-streams) and, in Chromium-based browsers, an HTTP/2 or HTTP/3 connection (streaming uploads over HTTP/1.1 fail with a network error, even over plain HTTP). This handler is silently ignored in unsupported environments and for requests with `keepalive: true` or `mode: 'no-cors'`, since they cannot use streaming request bodies. @param progress - Object containing upload progress information. @param chunk - Data that was sent. When an empty request body stream completes, the callback receives an empty chunk. A `ReadableStream` body cannot be measured, so `totalBytes` falls back to a `content-length` header you set. Browsers do not allow that header on a request, so there the total stays `0`. @example ``` import ky from 'ky'; const response = await ky.post('https://example.com/upload', { body: largeFile, onUploadProgress: (progress, chunk) => { // Example output: // `100% - 1271 of 1271 bytes` console.log(`${progress.percent * 100}% - ${progress.transferredBytes} of ${progress.totalBytes} bytes`); } }); ``` */ onUploadProgress?: ((progress: Progress, chunk: Uint8Array) => void) | undefined; /** User-defined `fetch` function. Has to be fully compatible with the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) standard. Use-cases: 1. Use the `fetch` wrapper function provided by some frameworks that use server-side rendering (SSR). 2. Add custom instrumentation or logging to all requests. @default fetch @example ``` import ky from 'ky'; const api = ky.create({ fetch: async (request, init) => { const start = performance.now(); const response = await fetch(request, init); const duration = performance.now() - start; console.log(`${request.method} ${request.url} - ${response.status} (${Math.round(duration)}ms)`); return response; } }); const json = await api('https://example.com').json(); ``` */ fetch?: ((input: Request, init?: RequestInit) => Promise<Response>) | undefined; /** User-defined data passed to hooks. This option allows you to pass arbitrary contextual data to hooks without polluting the request itself. The context is available in all hooks and is **guaranteed to always be an object** (never `undefined`), so you can safely access properties without optional chaining. Use cases: - Pass authentication tokens or API keys to hooks - Attach request metadata for logging or debugging - Implement conditional logic in hooks based on the request context - Pass serverless environment bindings (e.g., Cloudflare Workers) **Note:** Context is shallow merged. Top-level properties are merged, but nested objects are replaced. Only enumerable properties are copied. **Note:** Each request gets its own deep copy of the context, so a hook that mutates a nested plain object or array cannot write back to the instance defaults or to the object you passed in. Non-plain values such as class instances are shared by reference. A hook that caches data in the context, for example with `options.context.token ??= await getToken()`, therefore recomputes it once per request rather than once per instance. @example ``` import ky from 'ky'; // Pass data to hooks const api = ky.create({ hooks: { beforeRequest: [ ({request, options}) => { const {token} = options.context; if (token) { request.headers.set('Authorization', `Bearer ${token}`); } } ] } }); await api('https://example.com', { context: { token: 'secret123' } }).json(); // Shallow merge: only top-level properties are merged const instance = ky.create({ context: { a: 1, b: { nested: true } } }); const extended = instance.extend({ context: { b: { updated: true }, c: 3 } }); // Result: {a: 1, b: {updated: true}, c: 3} // Note: The original `b.nested` is gone (shallow merge) ``` @default {} */ context?: Record<string, unknown> | undefined; }
- sym:42916 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1388-1392)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:42967 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:43103 ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
  type ExpectedNormalizedRetryOptions = { limit: number; methods: string[]; statusCodes: number[]; afterStatusCodes: number[]; maxRetryAfter: number; backoffLimit: number; delay: (attemptCount: number) => number; jitter: boolean | ((delay: number) => number) | undefined; retryOnTimeout: boolean; shouldRetry?: RetryOptions['shouldRetry']; }
- sym:43043 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:42993 RetryOptions (source/types/retry.ts:21-187)
- sym:42917 Ky.#fetch (source/core/Ky.ts:1394-1458)
  async #fetch(): Promise<Response>
- sym:43105 initHook (test-d/retry.ts:17-26)
  const initHook: InitHook = options =>
- sym:42935 RetryTimingHeader (source/core/retry-timing.ts:11-14)
  type RetryTimingHeader = { value: string; allowTimestamp: boolean; }
- sym:42914 Ky.#retry (source/core/Ky.ts:1291-1297)
  async #retry(): Promise<Response | void>
- sym:43005 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:42913 Ky.#runAfterResponseHooks (source/core/Ky.ts:1223-1289)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:43050 isResponseSizeError (source/utils/type-guards.ts:152-154)
  function isResponseSizeError(error: unknown): error is ResponseSizeError
- sym:42905 Ky.#readResponseText (source/core/Ky.ts:1074-1142)
  async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined>
- sym:42918 Ky.#throwIfAbortedByUser (source/core/Ky.ts:1461-1463)
  #throwIfAbortedByUser(): void
- sym:42908 Ky.#cancelResponseBody (source/core/Ky.ts:1174-1177)
  #cancelResponseBody(response: Response): void
- sym:42973 KyInstance (source/types/ky.ts:5-204)
- sym:42906 Ky.#parseJson (source/core/Ky.ts:1144-1163)
  async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown>
- sym:42909 Ky.#cancelReturnedBody (source/core/Ky.ts:1179-1185)
  #cancelReturnedBody(value: unknown): void
- sym:42987 InternalOptions (source/types/options.ts:517-528)
  type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & { headers: Headers; hooks: NormalizedHooks; retry: NormalizedRetryOptions; fetch: NonNullable<Options['fetch']>; prefix: string; timeout: number | false; totalTimeout: number | false; maxResponseSize: number; context: Record<string, unknown>; throwHttpErrors: boolean | ((status: number) => boolean); }
- sym:42902 Ky.#raceBodyRead (source/core/Ky.ts:909-975)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42904 Ky.#isJsonContentType (source/core/Ky.ts:1052-1072)
  #isJsonContentType(contentType: string): boolean
- sym:42884 isResponseInstance (source/core/Ky.ts:136-137)
  const isResponseInstance = (value: unknown): value is Response =>
- sym:42926 Ky.#limitResponseSize (source/core/Ky.ts:1534-1537)
  #limitResponseSize(response: Response): Response
- sym:42934 retry (source/core/constants.ts:256-256)
  const retry = (options?: ForceRetryOptions) =>
- sym:42991 MutableRetryOptions (source/types/retry.ts:3-7)
  type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & { methods?: HttpMethod[] | undefined; statusCodes?: number[] | undefined; afterStatusCodes?: number[] | undefined; }
- sym:43045 isKyError (source/utils/type-guards.ts:36-38)
  function isKyError(error: unknown): error is KyError
- sym:42911 Ky.#throwIfTotalTimeoutExhausted (source/core/Ky.ts:1193-1198)
  #throwIfTotalTimeoutExhausted(request: Request = this.request): void
- sym:42937 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
  const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined =>
- sym:42945 HTTPError.constructor (source/errors/HTTPError.ts:22-33)
  constructor(response: Response, request: Request, options: Readonly<NormalizedOptions>)
- sym:43094 createCutConnectionServer (test/retry.ts:3888-3904)
  const createCutConnectionServer = async (t: ExecutionContext) =>
- sym:43095 createAbortDuringBodyFetch (test/retry.ts:4019-4038)
  const createAbortDuringBodyFetch = (): typeof fetch =>
- sym:43077 createResponseLike (test/hooks.ts:42-65)
  const createResponseLike = (response: Response): any =>
- sym:43079 createStreamFetch (test/hooks.ts:100-114)
  const createStreamFetch = ({ text = 'ok', onResponse, }: { text?: string; onResponse?: (response: Response) => void; } = {}): typeof fetch =>
- sym:43057 echoHeaders (test/headers.ts:11-14)
  const echoHeaders: RequestHandler = (request, response) =>
- sym:43096 createAbortDuringFetch (test/retry.ts:4040-4056)
  const createAbortDuringFetch = (onRequest?: () => void): typeof fetch =>
- sym:43087 createBodyServer (test/request-body-serialization.ts:12-22)
  async function createBodyServer(t: ExecutionContext)
- sym:43093 createSingleRetryHeaderServer (test/retry.ts:169-190)
  const createSingleRetryHeaderServer = async (t: ExecutionContext, headers: Record<string, string | number>) =>
- sym:43081 byteStream (test/http-response-bodies.ts:9-21)
  const byteStream = (bytes: Uint8Array) =>
- sym:43098 okFetch (test/validation.ts:4-4)
  const okFetch = async () =>
- sym:43080 createFakeResponse (test/http-error.ts:7-15)
  function createFakeResponse({status, statusText}: {status?: number; statusText?: string}): Response

---
tokens: 7012/8000
