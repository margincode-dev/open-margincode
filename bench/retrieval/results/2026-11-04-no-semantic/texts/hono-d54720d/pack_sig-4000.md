## Task
# hono-d54720d · hono-d54720d-2 · open
Goal: fix(serve-static): respect q-values in Accept-Encoding for precompressed files (#5436)
Workspace: /root/bench/ws/rebf815-hono-d54720d/hono-d54720d-2
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
- sym:52913 ServeStaticOptions (src/middleware/serve-static/index.ts:13-23)
  type ServeStaticOptions<E extends Env = Env> = { root?: string path?: string precompressed?: boolean /** Allow percent signs in the routed request path. Defaults to false. */ allowPercentInPath?: boolean mimes?: Record<string, string> rewriteRequestPath?: (path: string) => string onFound?: (path: string, c: Context<E>) => void | Promise<void> onNotFound?: (path: string, c: Context<E>) => void | Promise<void> }
- sym:53126 AcceptHeader (src/utils/headers.ts:345-352)
  type AcceptHeader = | 'Accept' | 'Accept-Charset' | 'Accept-Encoding' | 'Accept-Language' | 'Accept-Patch' | 'Accept-Post' | 'Accept-Ranges'
- sym:52009 ServeStaticOptions (adapters/cloudflare-workers/src/serve-static.ts:6-10)
  type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & { // namespace is KVNamespace namespace?: unknown manifest?: object | string }
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
- sym:51998 serveStatic (adapters/bun/src/serve-static.ts:8-32)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52162 serveStatic (src/adapter/bun/serve-static.ts:11-35)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52916 DEFAULT_DOCUMENT (src/middleware/serve-static/index.ts:32-32)
  const DEFAULT_DOCUMENT = 'index.html'
- sym:52014 serveStatic (adapters/deno/src/serve-static.ts:8-42)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52186 serveStatic (src/adapter/deno/serve-static.ts:11-45)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52914 ENCODINGS (src/middleware/serve-static/index.ts:25-29)
  const ENCODINGS = {
- sym:52915 ENCODINGS_ORDERED_KEYS (src/middleware/serve-static/index.ts:30-30)
  const ENCODINGS_ORDERED_KEYS = Object.keys(ENCODINGS) as (keyof typeof ENCODINGS)[]
- sym:53124 RequestHeader (src/utils/headers.ts:8-264)
  type RequestHeader = | 'A-IM' | 'Accept' | 'Accept-Additions' | 'Accept-CH' | 'Accept-Charset' | 'Accept-Datetime' | 'Accept-Encoding' | 'Accept-Features' | 'Accept-Language' | 'Accept-Patch' | 'Accept-Post' | 'Accept-Query' | 'Accept-Ranges' | 'Accept-Signature' | 'Access-Control' | 'Access-Control-Allow-Credentials' | 'Access-Control-Allow-Headers' | 'Access-Control-Allow-Methods' | 'Access-Control-Allow-Origin' | 'Access-Control-Expose-Headers' | 'Access-Control-Max-Age' | 'Access-Control-Request-Headers' | 'Access-Control-Request-Method' | 'Activate-Storage-Access' | 'Age' | 'Allow' | 'ALPN' | 'Alt-Svc' | 'Alt-Used' | 'Alternates' | 'AMP-Cache-Transform' | 'Apply-To-Redirect-Ref' | 'Authentication-Control' | 'Authentication-Info' | 'Authorization' | 'Available-Dictionary' | 'C-Ext' | 'C-Man' | 'C-Opt' | 'C-PEP' | 'C-PEP-Info' | 'Cache-Control' | 'Cache-Group-Invalidation' | 'Cache-Groups' | 'Cache-Status' | 'Cal-Managed-ID' | 'CalDAV-Timezones' | 'Capsule-Protocol' | 'CDN-Cache-Control' | 'CDN-Loop' | 'Cert-Not-After' | 'Cert-Not-Before' | 'Clear-Site-Data' | 'Client-Cert' | 'Client-Cert-Chain' | 'Close' | 'CMCD-Object' | 'CMCD-Request' | 'CMCD-Session' | 'CMCD-Status' | 'CMSD-Dynamic' | 'CMSD-Static' | 'Concealed-Auth-Export' | 'Configuration-Context' | 'Connection' | 'Content-Base' | 'Content-Digest' | 'Content-Disposition' | 'Content-Encoding' | 'Content-ID' | 'Content-Language' | 'Content-Length' | 'Content-Location' | 'Content-MD5' | 'Content-Range' | 'Content-Script-Type' | 'Content-Security-Policy' | 'Content-Security-Policy-Report-Only' | 'Content-Style-Type' | 'Content-Type' | 'Content-Version' | 'Cookie' | 'Cookie2' | 'Cross-Origin-Embedder-Policy' | 'Cross-Origin-Embedder-Policy-Report-Only' | 'Cross-Origin-Opener-Policy' | 'Cross-Origin-Opener-Policy-Report-Only' | 'Cross-Origin-Resource-Policy' | 'CTA-Common-Access-Token' | 'DASL' | 'Date' | 'DAV' | 'Default-Style' | 'Delta-Base' | 'Deprecation' | 'Depth' | 'Derived-From' | 'Destination' | 'Detached-JWS' | 'Differential-ID' | 'Dictionary-ID' | 'Digest' | 'DPoP' | 'DPoP-Nonce' | 'Early-Data' | 'EDIINT-Features' | 'ETag' | 'Expect' | 'Expect-CT' | 'Expires' | 'Ext' | 'Forwarded' | 'From' | 'GetProfile' | 'Hobareg' | 'Host' | 'HTTP2-Settings' | 'If' | 'If-Match' | 'If-Modified-Since' | 'If-None-Match' | 'If-Range' | 'If-Schedule-Tag-Match' | 'If-Unmodified-Since' | 'IM' | 'Include-Referred-Token-Binding-ID' | 'Incremental' | 'Isolation' | 'Keep-Alive' | 'Label' | 'Last-Event-ID' | 'Last-Modified' | 'Link' | 'Link-Template' | 'Location' | 'Lock-Token' | 'Man' | 'Max-Forwards' | 'Memento-Datetime' | 'Meter' | 'Method-Check' | 'Method-Check-Expires' | 'MIME-Version' | 'Negotiate' | 'NEL' | 'OData-EntityId' | 'OData-Isolation' | 'OData-MaxVersion' | 'OData-Version' | 'Opt' | 'Optional-WWW-Authenticate' | 'Ordering-Type' | 'Origin' | 'Origin-Agent-Cluster' | 'OSCORE' | 'OSLC-Core-Version' | 'Overwrite' | 'P3P' | 'PEP' | 'PEP-Info' | 'Permissions-Policy' | 'PICS-Label' | 'Ping-From' | 'Ping-To' | 'Position' | 'Pragma' | 'Prefer' | 'Preference-Applied' | 'Priority' | 'ProfileObject' | 'Protocol' | 'Protocol-Info' | 'Protocol-Query' | 'Protocol-Request' | 'Proxy-Authenticate' | 'Proxy-Authentication-Info' | 'Proxy-Authorization' | 'Proxy-Features' | 'Proxy-Instruction' | 'Proxy-Status' | 'Public' | 'Public-Key-Pins' | 'Public-Key-Pins-Report-Only' | 'Range' | 'Redirect-Ref' | 'Referer' | 'Referer-Root' | 'Referrer-Policy' | 'Refresh' | 'Repeatability-Client-ID' | 'Repeatability-First-Sent' | 'Repeatability-Request-ID' | 'Repeatability-Result' | 'Replay-Nonce' | 'Reporting-Endpoints' | 'Repr-Digest' | 'Retry-After' | 'Safe' | 'Schedule-Reply' | 'Schedule-Tag' | 'Sec-Fetch-Dest' | 'Sec-Fetch-Mode' | 'Sec-Fetch-Site' | 'Sec-Fetch-Storage-Access' | 'Sec-Fetch-User' | 'Sec-GPC' | 'Sec-Purpose' | 'Sec-Token-Binding' | 'Sec-WebSocket-Accept' | 'Sec-WebSocket-Extensions' | 'Sec-WebSocket-Key' | 'Sec-WebSocket-Protocol' | 'Sec-WebSocket-Version' | 'Security-Scheme' | 'Server' | 'Server-Timing' | 'Set-Cookie' | 'Set-Cookie2' | 'Set-Txn' | 'SetProfile' | 'Signature' | 'Signature-Input' | 'SLUG' | 'SoapAction' | 'Status-URI' | 'Strict-Transport-Security' | 'Sunset' | 'Surrogate-Capability' | 'Surrogate-Control' | 'TCN' | 'TE' | 'Timeout' | 'Timing-Allow-Origin' | 'Topic' | 'Traceparent' | 'Tracestate' | 'Trailer' | 'Transfer-Encoding' | 'TTL' | 'Unencoded-Digest' | 'Upgrade' | 'Urgency' | 'URI' | 'Use-As-Dictionary' | 'User-Agent' | 'Variant-Vary' | 'Vary' | 'Via' | 'Want-Content-Digest' | 'Want-Digest' | 'Want-Repr-Digest' | 'Want-Unencoded-Digest' | 'Warning' | 'WWW-Authenticate' | 'X-Content-Type-Options' | 'X-Frame-Options'
- sym:51916 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:52018 Env (adapters/lambda-edge/src/conninfo.ts:5-9)
  type Env = { Bindings: { event: CloudFrontEdgeEvent } }
- sym:52042 Env (adapters/netlify/src/conninfo.ts:28-32)
  type Env = { Bindings: { context: NetlifyContext } }
- sym:52089 Env (src/adapter/aws-lambda/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:52178 serveStatic (src/adapter/cloudflare-pages/handler.ts:124-133)
  const serveStatic = (): MiddlewareHandler =>
- sym:52188 Env (src/adapter/lambda-edge/conninfo.ts:5-9)
  type Env = { Bindings: { event: CloudFrontEdgeEvent } }
- sym:52211 Env (src/adapter/netlify/conninfo.ts:28-32)
  type Env = { Bindings: { context: NetlifyContext } }
- sym:52172 Env (src/adapter/cloudflare-pages/handler.test.ts:7-11)
  type Env = { Bindings: { TOKEN: string } }

---
tokens: 3462/4000
