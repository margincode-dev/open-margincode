## Task
# ky-0b1f462 · ky-0b1f462-2 · open
Goal: Progress from a declared content-length, and a throwing callback is never retried
Workspace: /root/bench/ws/rebf815-ky-0b1f462/ky-0b1f462-2
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
- sym:43453 streamResponse (source/utils/body.ts:198-211)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:43350 Ky.#raceBodyRead (source/core/Ky.ts:883-949)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:43450 getProgressCallbackError (source/utils/body.ts:104-105)
  const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
- sym:43449 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:43451 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43448 getBodySize (source/utils/body.ts:14-51)
  const getBodySize = (body?: BodyInit | null): number =>
- sym:43454 streamRequest (source/utils/body.ts:214-232)
  const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) =>
- sym:43340 Ky.create (source/core/Ky.ts:253-452)
  static create(input: Input, options: Options): ResponsePromise
- sym:43339 Ky (source/core/Ky.ts:252-1516)
  class Ky
- sym:43347 Ky.#getResponseData (source/core/Ky.ts:807-844)
  async #getResponseData(response: Response): Promise<unknown>
- sym:43427 KyOptions (source/types/options.ts:45-430)
  type KyOptions = { /** Shortcut for sending JSON. Use this instead of the `body` option. Accepts any plain object or value, which will be stringified using `JSON.stringify()` and sent in the body with the correct header set. The `Content-Type` header is set to `application/json` unless you set a `Content-Type` in the `headers` option, which always takes precedence. A `Content-Type` on a `Request` input is replaced, because the `Request` constructor sets one automatically from its body. */ json?: unknown; /** User-defined JSON-parsing function. The function receives the response text as the first argument and a context object as the second argument containing the `request` and `response`. Use-cases: 1. Parse JSON via the [`bourne` package](https://github.com/hapijs/bourne) to protect from prototype pollution. 2. Parse JSON with [`reviver` option of `JSON.parse()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse). 3. Log or handle JSON parse errors with request context. @default JSON.parse() @example ``` import ky from 'ky'; import bourne from '@hapijs/bourne'; const json = await ky('https://example.com', { parseJson: text => bourne(text) }).json(); ``` @example ``` import ky from 'ky'; const json = await ky('https://example.com', { parseJson: (text, {request, response}) => { console.log(`Parsing JSON from ${request.url} (status: ${response.status})`); return JSON.parse(text); } }).json(); ``` */ // `options` is intentionally not included in the context to avoid exposing Ky internals through a parsing callback. `request`/`response` already provide the metadata needed for logging. parseJson?: ((text: string, context: {request: Request; response: Response}) => unknown) | undefined; /** User-defined JSON-stringifying function. Use-cases: 1. Stringify JSON with a custom `replacer` function. @default JSON.stringify() @example ``` import ky from 'ky'; import {DateTime} from 'luxon'; const json = await ky('https://example.com', { stringifyJson: data => JSON.stringify(data, (key, value) => { if (key.endsWith('_at')) { return DateTime.fromISO(value).toSeconds(); } return value; }) }).json(); ``` */ stringifyJson?: ((data: unknown) => string) | undefined; /** Search parameters to include in the request URL. Setting this will merge with any existing search parameters in the input URL. Accepts any value supported by [`URLSearchParams()`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams). When passing an object, setting a value to `undefined` deletes the parameter, including from the input URL, even when a later option layer adds the parameter again. `null` values are preserved and converted to the string `'null'`. When `input` is a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) with a body, the body is sent as a stream, which requires [request stream support](https://caniuse.com/wf-fetch-request-streams) and, in Chromium-based browsers, an HTTP/2 or HTTP/3 connection (streaming uploads over HTTP/1.1 fail with a network error, even over plain HTTP). The inherited body is dropped when the search parameters change the URL and the request has to be rebuilt in an environment that cannot reuse it, which is the case without request stream support, when `keepalive` is true, or when the effective mode is `'no-cors'`. A search parameter value that leaves the URL unchanged does not rebuild the request, so the body is kept. A compatible body passed explicitly with the `body` option is still used. */ searchParams?: SearchParamsOption; /** A base URL to [resolve](https://developer.mozilla.org/en-US/docs/Web/API/URL_API/Resolving_relative_references) the `input` against. When the `input` (after applying the `prefix` option) is only a relative URL, such as `'users'`, `'/users'`, or `'//my-site.com'`, it will be resolved against the `baseUrl` to determine the destination of the request. Otherwise, the `input` is absolute, such as `'https://my-site.com'`, and it will bypass the `baseUrl`. Useful when used with [`ky.extend()`](#kyextenddefaultoptions) to create niche-specific Ky instances. If the `baseUrl` itself is relative, it will be resolved against the environment's base URL, such as [`document.baseURI`](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI) in browsers or `location.href` in Deno (see the `--location` flag). **Tip:** When setting a `baseUrl` that has a path, we recommend that it include a trailing slash `/`, as in `'/api/'` rather than `/api`. This ensures more intuitive behavior for page-relative `input` URLs, such as `'users'` or `'./users'`, where they will _extend_ from the full path of the `baseUrl` rather than _replacing_ its last path segment. @example ``` import ky from 'ky'; // On https://example.com const response = await ky('users', {baseUrl: '/api/'}); //=> 'https://example.com/api/users' const response = await ky('/users', {baseUrl: '/api/'}); //=> 'https://example.com/users' ``` */ baseUrl?: URL | string | undefined; /** A prefix to prepend to the `input` before making the request (and before it is resolved against the `baseUrl`). It can be any valid path or URL, either relative or absolute. A trailing slash `/` is optional and will be added automatically, if needed, when it is joined with `input`. Only takes effect when `input` is a string. Useful when used with [`ky.extend()`](#kyextenddefaultoptions) to create niche-specific Ky instances. *In most cases, you should use the `baseUrl` option instead, as it is more consistent with web standards. However, `prefix` is useful if you want origin-relative `input` URLs, such as `/users`, to be treated as if they were page-relative. In other words, the leading slash of the `input` will essentially be ignored, because the `prefix` will become part of the `input` before URL resolution happens.* Notes: - The `prefix` and `input` are joined with a slash `/`, and slashes are normalized at the join boundary by trimming trailing slashes from `prefix` and leading slashes from `input`. - After `prefix` and `input` are joined, the result is resolved against the `baseUrl` option, if present. @example ``` import ky from 'ky'; // On https://example.com const response = await ky('users', {prefix: '/api/'}); //=> 'https://example.com/api/users' const response = await ky('/users', {prefix: '/api/'}); //=> 'https://example.com/api/users' ``` */ prefix?: URL | string | undefined; /** Controls retry behavior. Each field is documented in the `RetryOptions` type. If `retry` is a number, it will be used as `limit` and other defaults will remain in place. `retry.limit`, including numeric shorthand, must be a finite, non-negative integer. Network errors (e.g., DNS failures, connection refused, offline) are automatically retried for retriable methods. Only errors recognized as network errors are retried; other errors (e.g., programming bugs) are thrown immediately. Use `shouldRetry` to customize this behavior. `413 Payload Too Large` is only retried when the response includes a retry timing header, unless `shouldRetry` returns `true`. When the response status is contained in `afterStatusCodes` and the retry is allowed by `statusCodes` or `shouldRetry`, Ky uses retry timing headers to choose the retry delay. [`Retry-After`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Retry-After) may provide a delay in seconds or an HTTP-date. If `Retry-After` is missing, Ky falls back to rate-limit timing headers (`RateLimit-Reset`, `X-RateLimit-Retry-After`, `X-RateLimit-Reset`, and `X-Rate-Limit-Reset`). Numeric `Retry-After` and `X-RateLimit-Retry-After` values are interpreted as delay seconds. Numeric `RateLimit-Reset`, `X-RateLimit-Reset`, and `X-Rate-Limit-Reset` values may also be interpreted as Unix timestamps, from 2001-09-09 onwards. A value below that is read as delay seconds, and a timestamp already in the past means the retry happens immediately. If the status code is not in `afterStatusCodes`, retry timing headers will be ignored. If the retry delay from a retry timing header is greater than `maxRetryAfter`, Ky will use `maxRetryAfter`. @example ``` import ky from 'ky'; const json = await ky('https://example.com', { retry: { limit: 10, methods: ['get'], statusCodes: [500] } }).json(); ``` */ retry?: RetryOptions | number | undefined; /** Per-attempt timeout in milliseconds for getting a response, applied independently to each retry. Ky shortcut methods also use this value as a separate timeout for reading the response body. Cannot be greater than 2147483647. See also `totalTimeout`. If set to `false`, there will be no per-attempt timeout. @default 10000 */ timeout?: number | false | undefined; /** Overall timeout in milliseconds for the entire operation, including retries and delays. Throws a `TimeoutError` if exceeded. Cannot be greater than 2147483647. `beforeError` hooks run after an error is produced and are not bounded by `totalTimeout`. If set to `false` or not specified, there is no overall timeout. @default false @example ``` import ky from 'ky'; // Each attempt gets 5s, but the whole operation must complete within 30s const json = await ky('https://example.com', { timeout: 5000, totalTimeout: 30_000, retry: { limit: 3, retryOnTimeout: true, } }).json(); ``` */ totalTimeout?: number | false | undefined; /** Maximum response body size in bytes. Must be a non-negative safe integer or `Infinity`. Set to `0` to allow only empty bodies. The limit counts bytes from the response stream after decompression, independently of `Content-Length`. It applies as the body is consumed, including in `afterResponse` hooks and for responses returned by hooks. Exceeding the limit cancels the stream and throws a `ResponseSizeError`, without automatically retrying. With `await ky(url)`, the response can resolve before the limit is exceeded; the body read will reject instead. This limits body bytes, not total memory usage. Parsing, buffering, and concurrent requests can use additional memory. Responses with a [null body status](https://fetch.spec.whatwg.org/#null-body-status), such as `204` and `205`, are never wrapped, because the `Response` constructor rejects a body for those statuses. A runtime that still exposes a body for such a status, such as WebKit for `205`, is therefore not limited. @default Infinity @example ``` import ky from 'ky'; const data = await ky('https://example.com/data', { maxResponseSize: 20 * 1024 * 1024, }).json(); ``` */ maxResponseSize?: number | undefined; /** Hooks allow modifications during the request lifecycle. Hook functions may be async and are run serially, unless otherwise noted. */ hooks?: Hooks | undefined; /** Throw an `HTTPError` when, after following redirects, the response has a non-2xx status code. To also throw for redirects instead of following them, set the [`redirect`](https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/fetch#Parameters) option to `'manual'`. Setting this to `false` may be useful if you are checking for resource availability and are expecting error responses. You can also pass a function that accepts the HTTP status code and returns a boolean for selective error handling. Note that this can violate the principle of least surprise, so it's recommended to use the boolean form unless you have a specific use case like treating 404 responses differently. Note: If `false`, error responses are considered successful and the request will not be retried. Note: [Opaque responses](https://developer.mozilla.org/en-US/docs/Web/API/Response/type) from `no-cors` requests are returned as-is (without throwing `HTTPError`), since the actual status is hidden by the browser. @default true */ throwHttpErrors?: boolean | ((status: number) => boolean) | undefined; /** Download progress event handler. @param progress - Object containing download progress information. @param chunk - Data that was received. When an empty response body stream completes, the callback receives an empty chunk. `content-length` is only used as the total for a response that is not content-coded, since it counts encoded bytes while the progress stream counts the bytes after decompression. A compressed response therefore reports `totalBytes: 0` until it completes, which means the percentage cannot be calculated while downloading. Responses with a [null body status](https://fetch.spec.whatwg.org/#null-body-status) are not streamed, so no progress events are emitted for them. When the callback throws, the error is reported by Ky's body method shortcuts. A response read directly with `response.text()` or `response.json()` may instead report the runtime's own stream error, such as Chromium's generic `TypeError`. @example ``` import ky from 'ky'; const response = await ky('https://example.com', { onDownloadProgress: (progress, chunk) => { // Example output: // `100% - 1271 of 1271 bytes` console.log(`${progress.percent * 100}% - ${progress.transferredBytes} of ${progress.totalBytes} bytes`); } }); ``` */ onDownloadProgress?: ((progress: Progress, chunk: Uint8Array) => void) | undefined; /** Upload progress event handler. Note: Requires [request stream support](https://caniuse.com/wf-fetch-request-streams) and, in Chromium-based browsers, an HTTP/2 or HTTP/3 connection (streaming uploads over HTTP/1.1 fail with a network error, even over plain HTTP). This handler is silently ignored in unsupported environments and for requests with `keepalive: true` or `mode: 'no-cors'`, since they cannot use streaming request bodies. @param progress - Object containing upload progress information. @param chunk - Data that was sent. When an empty request body stream completes, the callback receives an empty chunk. @example ``` import ky from 'ky'; const response = await ky.post('https://example.com/upload', { body: largeFile, onUploadProgress: (progress, chunk) => { // Example output: // `100% - 1271 of 1271 bytes` console.log(`${progress.percent * 100}% - ${progress.transferredBytes} of ${progress.totalBytes} bytes`); } }); ``` */ onUploadProgress?: ((progress: Progress, chunk: Uint8Array) => void) | undefined; /** User-defined `fetch` function. Has to be fully compatible with the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API) standard. Use-cases: 1. Use the `fetch` wrapper function provided by some frameworks that use server-side rendering (SSR). 2. Add custom instrumentation or logging to all requests. @default fetch @example ``` import ky from 'ky'; const api = ky.create({ fetch: async (request, init) => { const start = performance.now(); const response = await fetch(request, init); const duration = performance.now() - start; console.log(`${request.method} ${request.url} - ${response.status} (${Math.round(duration)}ms)`); return response; } }); const json = await api('https://example.com').json(); ``` */ fetch?: ((input: Request, init?: RequestInit) => Promise<Response>) | undefined; /** User-defined data passed to hooks. This option allows you to pass arbitrary contextual data to hooks without polluting the request itself. The context is available in all hooks and is **guaranteed to always be an object** (never `undefined`), so you can safely access properties without optional chaining. Use cases: - Pass authentication tokens or API keys to hooks - Attach request metadata for logging or debugging - Implement conditional logic in hooks based on the request context - Pass serverless environment bindings (e.g., Cloudflare Workers) **Note:** Context is shallow merged. Top-level properties are merged, but nested objects are replaced. Only enumerable properties are copied. **Note:** Each request gets its own deep copy of the context, so a hook that mutates a nested plain object or array cannot write back to the instance defaults or to the object you passed in. Non-plain values such as class instances are shared by reference. A hook that caches data in the context, for example with `options.context.token ??= await getToken()`, therefore recomputes it once per request rather than once per instance. @example ``` import ky from 'ky'; // Pass data to hooks const api = ky.create({ hooks: { beforeRequest: [ ({request, options}) => { const {token} = options.context; if (token) { request.headers.set('Authorization', `Bearer ${token}`); } } ] } }); await api('https://example.com', { context: { token: 'secret123' } }).json(); // Shallow merge: only top-level properties are merged const instance = ky.create({ context: { a: 1, b: { nested: true } } }); const extended = instance.extend({ context: { b: { updated: true }, c: 3 } }); // Result: {a: 1, b: {updated: true}, c: 3} // Note: The original `b.nested` is gone (shallow merge) ``` @default {} */ context?: Record<string, unknown> | undefined; }
- sym:43477 deepMergeInternal (source/utils/merge.ts:320-455)
  const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T =>
- sym:43407 LiteralUnion (source/types/common.ts:4-6)
  type LiteralUnion<LiteralType extends BaseType, BaseType extends Primitive> = | LiteralType | (BaseType & {_?: never})
- sym:43548 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
  type ExpectedProgressCallbacks = { readonly onDownloadProgress?: NonNullable<Options['onDownloadProgress']>; readonly onUploadProgress?: NonNullable<Options['onUploadProgress']>; }
- sym:43353 Ky.#readResponseText (source/core/Ky.ts:1048-1116)
  async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined>
- sym:43425 Progress (source/types/options.ts:21-36)
  type Progress = { /** A number between `0` and `1` representing the progress percentage. */ percent: number; /** The number of bytes transferred so far. */ transferredBytes: number; /** The total number of bytes to be transferred. This is an estimate and may be `0` for an empty transfer or when the total size cannot be determined. */ totalBytes: number; }
- sym:43375 Ky.#wrapRequestWithUploadProgress (source/core/Ky.ts:1509-1515)
  #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request
- sym:43352 Ky.#isJsonContentType (source/core/Ky.ts:1026-1046)
  #isJsonContentType(contentType: string): boolean
- sym:43346 Ky.#throwProcessedError (source/core/Ky.ts:775-805)
  async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never>
- sym:43329 createTextDecoder (source/core/Ky.ts:70-80)
  const createTextDecoder = (contentType: string): TextDecoder =>
- sym:43485 hasSearchParameters (source/utils/options.ts:31-60)
  const hasSearchParameters = (search: SearchParamsOption): boolean =>
- sym:43434 NormalizedOptions (source/types/options.ts:531-544)
  interface NormalizedOptions extends Readonly<RequestInit>
- sym:43404 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:43473 newHookValue (source/utils/merge.ts:188-206)
  function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K]
- sym:43394 NetworkError (source/errors/NetworkError.ts:11-19)
  class NetworkError extends KyError
- sym:43402 TimeoutError (source/errors/TimeoutError.ts:7-15)
  class TimeoutError extends KyError
- sym:43430 Options (source/types/options.ts:446-500)
  interface Options extends KyOptions, RequestOptions
- sym:43457 isRawNetworkError (source/utils/is-network-error.ts:15-46)
  function isRawNetworkError(error: unknown): error is TypeError
- sym:43530 ReceivedBody (test/request-body-serialization.ts:6-10)
  type ReceivedBody = { bytes: number[]; contentType?: string; contentLength: string; }
- sym:43531 createBodyServer (test/request-body-serialization.ts:12-22)
  async function createBodyServer(t: ExecutionContext)
- sym:43532 assertBytes (test/request-body-serialization.ts:24-27)
  function assertBytes(t: ExecutionContext, received: ReceivedBody, expected: Uint8Array)
- sym:43541 createUploadProgressTestServer (test/stream.ts:1185-1195)
  const createUploadProgressTestServer = async (t: ExecutionContext) =>
- sym:43499 KY_SCRIPT (test/browser.ts:33-39)
  const KY_SCRIPT = {
- sym:43525 byteStream (test/http-response-bodies.ts:9-21)
  const byteStream = (bytes: Uint8Array) =>
- sym:43538 createCutConnectionServer (test/retry.ts:3888-3904)
  const createCutConnectionServer = async (t: ExecutionContext) =>
- sym:43539 createAbortDuringBodyFetch (test/retry.ts:4019-4038)
  const createAbortDuringBodyFetch = (): typeof fetch =>

---
tokens: 5786/8000
