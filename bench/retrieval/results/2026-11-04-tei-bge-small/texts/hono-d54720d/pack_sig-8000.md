## Task
# hono-d54720d · hono-d54720d · open
Goal: fix(serve-static): respect q-values in Accept-Encoding for precompressed files (#5436)
Workspace: /root/bench/av_tei/ws/rec09fc-hono-d54720d/hono-d54720d
Changed (0): nothing yet

## Map
- repo map (583 files):
  .editorconfig
  .github/ (12 files)
    FUNDING.yml
    ISSUE_TEMPLATE/ (3 files)
    actions/ (1 files)
    pull_request_template.md
    workflows/ (6 files)
  .gitignore
  .tool-versions
  .vscode/ (2 files)
    extensions.json
    settings.json
  LICENSE
  README.md
  adapters/ (113 files)
    aws-lambda/ (15 files)
    bun/ (21 files)
    cloudflare-workers/ (16 files)
    deno/ (24 files)
    lambda-edge/ (10 files)
    netlify/ (10 files)
    service-worker/ (8 files)
    vercel/ (9 files)
  benchmarks/ (72 files)
    fetch/ (8 files)
    http-server/ (3 files)
    jsx-dom/ (1 files)
    jsx/ (20 files)
    query-param/ (7 files)
    routers-deno/ (12 files)
    routers/ (17 files)
    utils/ (4 files)
  build/ (8 files)
    declaration-emit.test.ts
    dts-plugins.ts
    fixtures/ (4 files)
    jsx-types.test.ts
    release-adapter.ts
  bunfig.toml
  codecov.yml
  docs/ (10 files)
    CODE_OF_CONDUCT.md
    CONTRIBUTING.md
    MAINTAINING.md
    MIGRATION.md
    images/ (6 files)
  jsr.json
  package.cjs.json
  package.json
  perf-measures/ (12 files)
    .octocov.consolidated.perf-measures.main.yml
    .octocov.consolidated.perf-measures.yml
    bundle-check/ (3 files)
    type-check/ (7 files)
  pnpm-lock.yaml
  pnpm-workspace.yaml
  runtime-tests/ (22 files)
    bun/ (3 files)
    deno-jsx/ (3 files)
    deno/ (6 files)
    fastly/ (3 files)
    node/ (3 files)
    workerd/ (4 files)
  src/ (314 files)
    adapter/ (57 files)
    client/ (8 files)
    compose.test.ts
    compose.ts
    context.test.ts
    context.ts
    helper/ (43 files)
    hono-base.ts
    hono.test.ts
    hono.ts
    http-exception.test.ts
    http-exception.ts
    index.ts
    jsx/ (46 files)
    middleware/ (66 files)
    preset/ (4 files)
    request.test.ts
    request.ts
    request/ (1 files)
    router.ts
    router/ (24 files)
    types.test.ts
    types.ts
    utils/ (45 files)
    validator/ (5 files)
  tsconfig.base.json
  tsconfig.build.json
  tsconfig.json
  tsconfig.spec.json
  vite.config.ts
  vitest.config.ts

## Code
- sym:53126 AcceptHeader (src/utils/headers.ts:345-352)
  type AcceptHeader = | 'Accept' | 'Accept-Charset' | 'Accept-Encoding' | 'Accept-Language' | 'Accept-Patch' | 'Accept-Post' | 'Accept-Ranges'
- sym:52914 ENCODINGS (src/middleware/serve-static/index.ts:25-29)
  const ENCODINGS = {
- sym:53049 parseAccept (src/utils/accept.ts:211-240)
  const parseAccept = (acceptHeader: string): Accept[] =>
- sym:52794 compress (src/middleware/compress/index.ts:72-136)
  const compress = (options?: CompressionOptions): MiddlewareHandler =>
- sym:53124 RequestHeader (src/utils/headers.ts:8-264)
  type RequestHeader = | 'A-IM' | 'Accept' | 'Accept-Additions' | 'Accept-CH' | 'Accept-Charset' | 'Accept-Datetime' | 'Accept-Encoding' | 'Accept-Features' | 'Accept-Language' | 'Accept-Patch' | 'Accept-Post' | 'Accept-Query' | 'Accept-Ranges' | 'Accept-Signature' | 'Access-Control' | 'Access-Control-Allow-Credentials' | 'Access-Control-Allow-Headers' | 'Access-Control-Allow-Methods' | 'Access-Control-Allow-Origin' | 'Access-Control-Expose-Headers' | 'Access-Control-Max-Age' | 'Access-Control-Request-Headers' | 'Access-Control-Request-Method' | 'Activate-Storage-Access' | 'Age' | 'Allow' | 'ALPN' | 'Alt-Svc' | 'Alt-Used' | 'Alternates' | 'AMP-Cache-Transform' | 'Apply-To-Redirect-Ref' | 'Authentication-Control' | 'Authentication-Info' | 'Authorization' | 'Available-Dictionary' | 'C-Ext' | 'C-Man' | 'C-Opt' | 'C-PEP' | 'C-PEP-Info' | 'Cache-Control' | 'Cache-Group-Invalidation' | 'Cache-Groups' | 'Cache-Status' | 'Cal-Managed-ID' | 'CalDAV-Timezones' | 'Capsule-Protocol' | 'CDN-Cache-Control' | 'CDN-Loop' | 'Cert-Not-After' | 'Cert-Not-Before' | 'Clear-Site-Data' | 'Client-Cert' | 'Client-Cert-Chain' | 'Close' | 'CMCD-Object' | 'CMCD-Request' | 'CMCD-Session' | 'CMCD-Status' | 'CMSD-Dynamic' | 'CMSD-Static' | 'Concealed-Auth-Export' | 'Configuration-Context' | 'Connection' | 'Content-Base' | 'Content-Digest' | 'Content-Disposition' | 'Content-Encoding' | 'Content-ID' | 'Content-Language' | 'Content-Length' | 'Content-Location' | 'Content-MD5' | 'Content-Range' | 'Content-Script-Type' | 'Content-Security-Policy' | 'Content-Security-Policy-Report-Only' | 'Content-Style-Type' | 'Content-Type' | 'Content-Version' | 'Cookie' | 'Cookie2' | 'Cross-Origin-Embedder-Policy' | 'Cross-Origin-Embedder-Policy-Report-Only' | 'Cross-Origin-Opener-Policy' | 'Cross-Origin-Opener-Policy-Report-Only' | 'Cross-Origin-Resource-Policy' | 'CTA-Common-Access-Token' | 'DASL' | 'Date' | 'DAV' | 'Default-Style' | 'Delta-Base' | 'Deprecation' | 'Depth' | 'Derived-From' | 'Destination' | 'Detached-JWS' | 'Differential-ID' | 'Dictionary-ID' | 'Digest' | 'DPoP' | 'DPoP-Nonce' | 'Early-Data' | 'EDIINT-Features' | 'ETag' | 'Expect' | 'Expect-CT' | 'Expires' | 'Ext' | 'Forwarded' | 'From' | 'GetProfile' | 'Hobareg' | 'Host' | 'HTTP2-Settings' | 'If' | 'If-Match' | 'If-Modified-Since' | 'If-None-Match' | 'If-Range' | 'If-Schedule-Tag-Match' | 'If-Unmodified-Since' | 'IM' | 'Include-Referred-Token-Binding-ID' | 'Incremental' | 'Isolation' | 'Keep-Alive' | 'Label' | 'Last-Event-ID' | 'Last-Modified' | 'Link' | 'Link-Template' | 'Location' | 'Lock-Token' | 'Man' | 'Max-Forwards' | 'Memento-Datetime' | 'Meter' | 'Method-Check' | 'Method-Check-Expires' | 'MIME-Version' | 'Negotiate' | 'NEL' | 'OData-EntityId' | 'OData-Isolation' | 'OData-MaxVersion' | 'OData-Version' | 'Opt' | 'Optional-WWW-Authenticate' | 'Ordering-Type' | 'Origin' | 'Origin-Agent-Cluster' | 'OSCORE' | 'OSLC-Core-Version' | 'Overwrite' | 'P3P' | 'PEP' | 'PEP-Info' | 'Permissions-Policy' | 'PICS-Label' | 'Ping-From' | 'Ping-To' | 'Position' | 'Pragma' | 'Prefer' | 'Preference-Applied' | 'Priority' | 'ProfileObject' | 'Protocol' | 'Protocol-Info' | 'Protocol-Query' | 'Protocol-Request' | 'Proxy-Authenticate' | 'Proxy-Authentication-Info' | 'Proxy-Authorization' | 'Proxy-Features' | 'Proxy-Instruction' | 'Proxy-Status' | 'Public' | 'Public-Key-Pins' | 'Public-Key-Pins-Report-Only' | 'Range' | 'Redirect-Ref' | 'Referer' | 'Referer-Root' | 'Referrer-Policy' | 'Refresh' | 'Repeatability-Client-ID' | 'Repeatability-First-Sent' | 'Repeatability-Request-ID' | 'Repeatability-Result' | 'Replay-Nonce' | 'Reporting-Endpoints' | 'Repr-Digest' | 'Retry-After' | 'Safe' | 'Schedule-Reply' | 'Schedule-Tag' | 'Sec-Fetch-Dest' | 'Sec-Fetch-Mode' | 'Sec-Fetch-Site' | 'Sec-Fetch-Storage-Access' | 'Sec-Fetch-User' | 'Sec-GPC' | 'Sec-Purpose' | 'Sec-Token-Binding' | 'Sec-WebSocket-Accept' | 'Sec-WebSocket-Extensions' | 'Sec-WebSocket-Key' | 'Sec-WebSocket-Protocol' | 'Sec-WebSocket-Version' | 'Security-Scheme' | 'Server' | 'Server-Timing' | 'Set-Cookie' | 'Set-Cookie2' | 'Set-Txn' | 'SetProfile' | 'Signature' | 'Signature-Input' | 'SLUG' | 'SoapAction' | 'Status-URI' | 'Strict-Transport-Security' | 'Sunset' | 'Surrogate-Capability' | 'Surrogate-Control' | 'TCN' | 'TE' | 'Timeout' | 'Timing-Allow-Origin' | 'Topic' | 'Traceparent' | 'Tracestate' | 'Trailer' | 'Transfer-Encoding' | 'TTL' | 'Unencoded-Digest' | 'Upgrade' | 'Urgency' | 'URI' | 'Use-As-Dictionary' | 'User-Agent' | 'Variant-Vary' | 'Vary' | 'Via' | 'Want-Content-Digest' | 'Want-Digest' | 'Want-Repr-Digest' | 'Want-Unencoded-Digest' | 'Warning' | 'WWW-Authenticate' | 'X-Content-Type-Options' | 'X-Frame-Options'
- sym:52790 Encoding (src/middleware/compress/index.ts:13-13)
  type Encoding = (typeof ENCODING_TYPES)[number]
- sym:53041 Accept (src/utils/accept.ts:1-5)
  interface Accept
- sym:52324 Accept (src/helper/accepts/accepts.ts:5-9)
  interface Accept
- sym:52793 selectEncoding (src/middleware/compress/index.ts:24-44)
  const selectEncoding = ( header: string | undefined, candidates: readonly Encoding[] ): Encoding | undefined =>
- sym:53125 ResponseHeader (src/utils/headers.ts:266-343)
  type ResponseHeader = | 'Accept-Query' | 'Access-Control-Allow-Credentials' | 'Access-Control-Allow-Headers' | 'Access-Control-Allow-Methods' | 'Access-Control-Allow-Origin' | 'Access-Control-Expose-Headers' | 'Access-Control-Max-Age' | 'Activate-Storage-Access' | 'Age' | 'Allow' | 'Cache-Control' | 'Cache-Group-Invalidation' | 'Cache-Groups' | 'Clear-Site-Data' | 'Content-Disposition' | 'Content-Encoding' | 'Content-Language' | 'Content-Length' | 'Content-Location' | 'Content-Range' | 'Content-Security-Policy' | 'Content-Security-Policy-Report-Only' | 'Content-Type' | 'Cookie' | 'Cross-Origin-Embedder-Policy' | 'Cross-Origin-Opener-Policy' | 'Cross-Origin-Resource-Policy' | 'Date' | 'ETag' | 'Expires' | 'Incremental' | 'Last-Modified' | 'Location' | 'Permissions-Policy' | 'Pragma' | 'Retry-After' | 'Save-Data' | 'Sec-CH-Prefers-Color-Scheme' | 'Sec-CH-Prefers-Reduced-Motion' | 'Sec-CH-UA' | 'Sec-CH-UA-Arch' | 'Sec-CH-UA-Bitness' | 'Sec-CH-UA-Form-Factor' | 'Sec-CH-UA-Full-Version' | 'Sec-CH-UA-Full-Version-List' | 'Sec-CH-UA-Mobile' | 'Sec-CH-UA-Model' | 'Sec-CH-UA-Platform' | 'Sec-CH-UA-Platform-Version' | 'Sec-CH-UA-WoW64' | 'Sec-Fetch-Dest' | 'Sec-Fetch-Mode' | 'Sec-Fetch-Site' | 'Sec-Fetch-User' | 'Sec-GPC' | 'Server' | 'Server-Timing' | 'Service-Worker-Navigation-Preload' | 'Set-Cookie' | 'Set-Txn' | 'Strict-Transport-Security' | 'Timing-Allow-Origin' | 'Trailer' | 'Transfer-Encoding' | 'Unencoded-Digest' | 'Upgrade' | 'Vary' | 'Want-Unencoded-Digest' | 'Warning' | 'WWW-Authenticate' | 'X-Content-Type-Options' | 'X-DNS-Prefetch-Control' | 'X-Frame-Options' | 'X-Permitted-Cross-Domain-Policies' | 'X-Powered-By' | 'X-Robots-Tag' | 'X-XSS-Protection'
- sym:52915 ENCODINGS_ORDERED_KEYS (src/middleware/serve-static/index.ts:30-30)
  const ENCODINGS_ORDERED_KEYS = Object.keys(ENCODINGS) as (keyof typeof ENCODINGS)[]
- sym:53048 getNextAcceptValue (src/utils/accept.ts:159-209)
  const getNextAcceptValue = ( acceptHeader: string, startIndex: number ): [number, Accept | undefined] =>
- sym:53047 getNextParam (src/utils/accept.ts:66-157)
  const getNextParam = ( acceptHeader: string, startIndex: number ): [number, string | undefined, string | undefined, boolean] =>
- sym:53046 skipInvalidAcceptValue (src/utils/accept.ts:46-64)
  const skipInvalidAcceptValue = (acceptHeader: string, startIndex: number): number =>
- sym:52913 ServeStaticOptions (src/middleware/serve-static/index.ts:13-23)
  type ServeStaticOptions<E extends Env = Env> = { root?: string path?: string precompressed?: boolean /** Allow percent signs in the routed request path. Defaults to false. */ allowPercentInPath?: boolean mimes?: Record<string, string> rewriteRequestPath?: (path: string) => string onFound?: (path: string, c: Context<E>) => void | Promise<void> onNotFound?: (path: string, c: Context<E>) => void | Promise<void> }
- sym:52009 ServeStaticOptions (adapters/cloudflare-workers/src/serve-static.ts:6-10)
  type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & { // namespace is KVNamespace namespace?: unknown manifest?: object | string }
- sym:52789 ENCODING_TYPES (src/middleware/compress/index.ts:12-12)
  const ENCODING_TYPES = ['gzip', 'deflate'] as const
- sym:52181 ServeStaticOptions (src/adapter/cloudflare-workers/serve-static.ts:9-13)
  type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & { // namespace is KVNamespace namespace?: unknown manifest?: object | string }
- sym:52008 module (adapters/cloudflare-workers/src/serve-static-module.ts:6-10)
  const module = <E extends Env = Env>( options: Omit<ServeStaticOptions<E>, 'namespace'> ): MiddlewareHandler =>
- sym:52180 module (src/adapter/cloudflare-workers/serve-static-module.ts:6-10)
  const module = <E extends Env = Env>( options: Omit<ServeStaticOptions<E>, 'namespace'> ): MiddlewareHandler =>
- sym:52010 serveStatic (adapters/cloudflare-workers/src/serve-static.ts:21-42)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52182 serveStatic (src/adapter/cloudflare-workers/serve-static.ts:24-45)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52792 CompressionOptions (src/middleware/compress/index.ts:18-22)
  interface CompressionOptions
- sym:51998 serveStatic (adapters/bun/src/serve-static.ts:8-32)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52162 serveStatic (src/adapter/bun/serve-static.ts:11-35)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:53084 COMPRESSIBLE_CONTENT_TYPE_REGEX (src/utils/compress.ts:9-10)
  const COMPRESSIBLE_CONTENT_TYPE_REGEX =
- sym:52916 DEFAULT_DOCUMENT (src/middleware/serve-static/index.ts:32-32)
  const DEFAULT_DOCUMENT = 'index.html'
- sym:52014 serveStatic (adapters/deno/src/serve-static.ts:8-42)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52329 defaultMatch (src/helper/accepts/accepts.ts:52-72)
  const defaultMatch = (accepts: Accept[], config: acceptsConfig): string =>
- sym:52186 serveStatic (src/adapter/deno/serve-static.ts:11-45)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:53045 skipInvalidParam (src/utils/accept.ts:30-44)
  const skipInvalidParam = (acceptHeader: string, startIndex: number): [number, boolean] =>
- sym:53050 parseQuality (src/utils/accept.ts:242-268)
  const parseQuality = (qVal?: string): number =>
- sym:52917 serveStatic (src/middleware/serve-static/index.ts:37-135)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> & { getContent: (path: string, c: Context<E>) => Promise<Data | Response | null> /** * * `join` option according to the runtime. Example `import { join } from 'node:path`. If not specified, it will fall back to the default join function.` */ join?: (...paths: string[]) => string /** * @deprecated Currently, `pathResolve` is no longer used. */ pathResolve?: (path: string) => string isDir?: (path: string) => boolean | undefined | Promise<boolean | undefined> } ): MiddlewareHandler =>
- sym:52918 defaultJoin (src/middleware/serve-static/path.ts:5-25)
  const defaultJoin = (...paths: string[]): string =>
- sym:53175 convertIPv6BinaryToString (src/utils/ipaddr.ts:328-374)
  const convertIPv6BinaryToString = (ipV6: bigint): string =>
- sym:52178 serveStatic (src/adapter/cloudflare-pages/handler.ts:124-133)
  const serveStatic = (): MiddlewareHandler =>
- sym:52325 acceptsConfig (src/helper/accepts/accepts.ts:11-15)
  interface acceptsConfig
- sym:53161 CHAR_CODE_COLON (src/utils/ipaddr.ts:53-53)
  const CHAR_CODE_COLON = 58
- sym:52431 ToSSGResult (src/helper/ssg/ssg.ts:44-48)
  interface ToSSGResult
- sym:53044 ignoreTrailingWhitespace (src/utils/accept.ts:20-28)
  const ignoreTrailingWhitespace = (acceptHeader: string, startIndex: number): number =>
- sym:52995 PreparedRegExpRouter (src/router/reg-exp-router/prepared-router.ts:9-93)
  class PreparedRegExpRouter<T> implements Router<T>
- sym:53158 CHAR_CODE_A (src/utils/ipaddr.ts:48-48)
  const CHAR_CODE_A = 65
- sym:51971 defaultIsContentTypeBinary (adapters/aws-lambda/src/handler.ts:670-678)
  const defaultIsContentTypeBinary = (contentType: string): boolean =>
- sym:52406 proxy (src/helper/proxy/index.ts:160-202)
  const proxy: ProxyFetch = async (input, proxyInit) =>
- sym:53115 encodeBase64Url (src/utils/encode.ts:10-11)
  const encodeBase64Url = (buf: ArrayBufferLike): string =>
- sym:53200 encodeSignaturePart (src/utils/jwt/jwt.ts:33-33)
  const encodeSignaturePart = (buf: ArrayBufferLike): string =>
- sym:53159 CHAR_CODE_F (src/utils/ipaddr.ts:49-49)
  const CHAR_CODE_F = 70
- sym:52144 defaultIsContentTypeBinary (src/adapter/aws-lambda/handler.ts:711-719)
  const defaultIsContentTypeBinary = (contentType: string): boolean =>
- sym:52848 detectFromHeader (src/middleware/language/language.ts:153-176)
  function detectFromHeader(c: Context, options: DetectorOptions): string | undefined
- sym:52312 TEXT_PLAIN (src/context.ts:279-279)
  const TEXT_PLAIN = 'text/plain; charset=UTF-8'
- sym:52001 BunServerWebSocket (adapters/bun/src/websocket.ts:8-13)
  interface BunServerWebSocket<T>
- sym:52005 createWSContext (adapters/bun/src/websocket.ts:33-46)
  const createWSContext = (ws: BunServerWebSocket<BunWebSocketData>): WSContext =>
- sym:52165 BunServerWebSocket (src/adapter/bun/websocket.ts:9-14)
  interface BunServerWebSocket<T>
- sym:52169 createWSContext (src/adapter/bun/websocket.ts:41-54)
  const createWSContext = (ws: BunServerWebSocket<BunWebSocketData>): WSContext =>
- sym:52255 Response (src/client/types.ts:232-232)
  interface Response extends ClientResponse<unknown>
- sym:52317 Context.req (src/context.ts:366-369)
  get req(): HonoRequest<P, I['out']>
- sym:52320 Context.res (src/context.ts:403-407)
  get res(): Response
- sym:52330 accepts (src/helper/accepts/accepts.ts:87-96)
  const accepts = (c: Context, options: acceptsOptions): string =>
- sym:52466 stream (src/helper/streaming/stream.ts:7-45)
  const stream = ( c: Context, cb: (stream: StreamingApi) => Promise<void>, onError?: (e: Error, stream: StreamingApi) => Promise<void> ): Response =>
- sym:52475 SendOptions (src/helper/websocket/index.ts:63-65)
  interface SendOptions
- sym:52526 JSXNode.type (src/jsx/base.ts:217-219)
  get type(): string | Function
- sym:52795 shouldTransform (src/middleware/compress/index.ts:138-143)
  const shouldTransform = (res: Response) =>
- sym:52817 etag (src/middleware/etag/index.ts:80-136)
  const etag = (options?: ETagOptions): MiddlewareHandler =>
- sym:52844 parseAcceptLanguage (src/middleware/language/language.ts:74-76)
  function parseAcceptLanguage(header: string): Array<{ lang: string; q: number }>
- sym:52953 HonoRequest.header (src/request.ts:185-195)
  header(name?: string)
- sym:52964 HonoRequest.method (src/request.ts:390-392)
  get method(): string

---
tokens: 5512/8000
