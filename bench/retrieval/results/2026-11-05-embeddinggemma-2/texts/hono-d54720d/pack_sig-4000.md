## Task
# hono-d54720d · hono-d54720d · open
Goal: fix(serve-static): respect q-values in Accept-Encoding for precompressed files (#5436)
Workspace: /root/bench/av_gemma/ws/rec728b-hono-d54720d/hono-d54720d
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
- sym:52010 serveStatic (adapters/cloudflare-workers/src/serve-static.ts:21-42)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52914 ENCODINGS (src/middleware/serve-static/index.ts:25-29)
  const ENCODINGS = {
- sym:52915 ENCODINGS_ORDERED_KEYS (src/middleware/serve-static/index.ts:30-30)
  const ENCODINGS_ORDERED_KEYS = Object.keys(ENCODINGS) as (keyof typeof ENCODINGS)[]
- sym:53050 parseQuality (src/utils/accept.ts:242-268)
  const parseQuality = (qVal?: string): number =>
- sym:52793 selectEncoding (src/middleware/compress/index.ts:24-44)
  const selectEncoding = ( header: string | undefined, candidates: readonly Encoding[] ): Encoding | undefined =>
- sym:53126 AcceptHeader (src/utils/headers.ts:345-352)
  type AcceptHeader = | 'Accept' | 'Accept-Charset' | 'Accept-Encoding' | 'Accept-Language' | 'Accept-Patch' | 'Accept-Post' | 'Accept-Ranges'
- sym:52009 ServeStaticOptions (adapters/cloudflare-workers/src/serve-static.ts:6-10)
  type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & { // namespace is KVNamespace namespace?: unknown manifest?: object | string }
- sym:52329 defaultMatch (src/helper/accepts/accepts.ts:52-72)
  const defaultMatch = (accepts: Accept[], config: acceptsConfig): string =>
- sym:52181 ServeStaticOptions (src/adapter/cloudflare-workers/serve-static.ts:9-13)
  type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & { // namespace is KVNamespace namespace?: unknown manifest?: object | string }
- sym:52324 Accept (src/helper/accepts/accepts.ts:5-9)
  interface Accept
- sym:52008 module (adapters/cloudflare-workers/src/serve-static-module.ts:6-10)
  const module = <E extends Env = Env>( options: Omit<ServeStaticOptions<E>, 'namespace'> ): MiddlewareHandler =>
- sym:53041 Accept (src/utils/accept.ts:1-5)
  interface Accept
- sym:52180 module (src/adapter/cloudflare-workers/serve-static-module.ts:6-10)
  const module = <E extends Env = Env>( options: Omit<ServeStaticOptions<E>, 'namespace'> ): MiddlewareHandler =>
- sym:52790 Encoding (src/middleware/compress/index.ts:13-13)
  type Encoding = (typeof ENCODING_TYPES)[number]
- sym:52794 compress (src/middleware/compress/index.ts:72-136)
  const compress = (options?: CompressionOptions): MiddlewareHandler =>
- sym:52182 serveStatic (src/adapter/cloudflare-workers/serve-static.ts:24-45)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:51998 serveStatic (adapters/bun/src/serve-static.ts:8-32)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52789 ENCODING_TYPES (src/middleware/compress/index.ts:12-12)
  const ENCODING_TYPES = ['gzip', 'deflate'] as const
- sym:52162 serveStatic (src/adapter/bun/serve-static.ts:11-35)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52792 CompressionOptions (src/middleware/compress/index.ts:18-22)
  interface CompressionOptions
- sym:52916 DEFAULT_DOCUMENT (src/middleware/serve-static/index.ts:32-32)
  const DEFAULT_DOCUMENT = 'index.html'
- sym:52014 serveStatic (adapters/deno/src/serve-static.ts:8-42)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:53049 parseAccept (src/utils/accept.ts:211-240)
  const parseAccept = (acceptHeader: string): Accept[] =>
- sym:52186 serveStatic (src/adapter/deno/serve-static.ts:11-45)
  const serveStatic = <E extends Env = Env>( options: ServeStaticOptions<E> = {} ): MiddlewareHandler =>
- sym:52795 shouldTransform (src/middleware/compress/index.ts:138-143)
  const shouldTransform = (res: Response) =>
- sym:52406 proxy (src/helper/proxy/index.ts:160-202)
  const proxy: ProxyFetch = async (input, proxyInit) =>
- sym:53124 RequestHeader (src/utils/headers.ts:8-264)
- sym:51916 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:52012 getContentFromKVAsset (adapters/cloudflare-workers/src/utils.ts:11-50)
  const getContentFromKVAsset = async ( path: string, options?: KVAssetOptions ): Promise<ReadableStream | null> =>
- sym:52018 Env (adapters/lambda-edge/src/conninfo.ts:5-9)
  type Env = { Bindings: { event: CloudFrontEdgeEvent } }
- sym:52042 Env (adapters/netlify/src/conninfo.ts:28-32)
  type Env = { Bindings: { context: NetlifyContext } }
- sym:52089 Env (src/adapter/aws-lambda/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:52178 serveStatic (src/adapter/cloudflare-pages/handler.ts:124-133)
  const serveStatic = (): MiddlewareHandler =>
- sym:52184 getContentFromKVAsset (src/adapter/cloudflare-workers/utils.ts:17-56)
  const getContentFromKVAsset = async ( path: string, options?: KVAssetOptions ): Promise<ReadableStream | null> =>
- sym:52172 Env (src/adapter/cloudflare-pages/handler.test.ts:7-11)
  type Env = { Bindings: { TOKEN: string } }

---
tokens: 2251/4000
