## Task
# hono-eb78eca · hono-eb78eca · open
Goal: fix(utils/url): preserve encoded duplicate query order (#5495)

Fixes #5580

* fix(utils/url): preserve encoded duplicate query order

* test(bun): cover encoded query fallback and empty values

Added tests for query parameter handling in Hono app.

* test: drop the bun runtime case for query parsing

---------
Workspace: /root/bench/av_tei/ws/rec09fc-hono-eb78eca/hono-eb78eca
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
- sym:44929 _getQueryParam (src/utils/url.ts:232-318)
  const _getQueryParam = ( url: string, key?: string, multiple?: boolean ): string | undefined | Record<string, string> | string[] | Record<string, string[]> =>
- sym:44923 getQueryStrings (src/utils/url.ts:140-152)
  const getQueryStrings = (url: string): string =>
- sym:43584 EventV1Processor.getQueryString (adapters/aws-lambda/src/handler.ts:456-472)
  protected getQueryString(event: APIGatewayProxyEvent): string
- sym:43754 EventV1Processor (src/adapter/aws-lambda/handler.ts:478-534)
  class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
- sym:43581 EventV1Processor (adapters/aws-lambda/src/handler.ts:447-503)
  class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
- sym:43757 EventV1Processor.getQueryString (src/adapter/aws-lambda/handler.ts:487-503)
  protected getQueryString(event: APIGatewayProxyEvent): string
- sym:44930 getQueryParams (src/utils/url.ts:328-333)
  const getQueryParams = ( url: string, key?: string ): string[] | undefined | Record<string, string[]> =>
- sym:44205 warnIfStorelessAccess (src/jsx/context.ts:73-80)
  const warnIfStorelessAccess = (): void =>
- sym:43861 appendQueryParams (src/client/client.ts:33-36)
  const appendQueryParams = (url: string, searchParams: URLSearchParams): string =>
- sym:44417 createQueryDigest (src/middleware/cache/index.ts:95-153)
  const createQueryDigest = async ( c: Context, maxQueryBodySize: number ): Promise<string | undefined> =>
- sym:44477 DetectorOptions (src/middleware/language/language.ts:13-45)
  interface DetectorOptions
- sym:44430 compress (src/middleware/compress/index.ts:72-136)
  const compress = (options?: CompressionOptions): MiddlewareHandler =>
- sym:44414 createCacheKey (src/middleware/cache/index.ts:51-70)
  const createCacheKey = ( key: string, requestUrl: string, request: CacheKeyRequest, varyHeaders: [string, string][] ): string =>
- sym:44551 ENCODINGS_ORDERED_KEYS (src/middleware/serve-static/index.ts:31-31)
  const ENCODINGS_ORDERED_KEYS = Object.keys(ENCODINGS) as (keyof typeof ENCODINGS)[]
- sym:44587 HonoRequest.query (src/request.ts:147-149)
  query(key?: string)
- sym:44488 validateOptions (src/middleware/language/language.ts:209-221)
  function validateOptions(options: DetectorOptions): void
- sym:44429 selectEncoding (src/middleware/compress/index.ts:24-44)
  const selectEncoding = ( header: string | undefined, candidates: readonly Encoding[] ): Encoding | undefined =>
- sym:44926 checkOptionalParameter (src/utils/url.ts:184-219)
  const checkOptionalParameter = (path: string): string[] | null =>
- sym:43639 BunWebSocketData (adapters/bun/src/websocket.ts:24-28)
  interface BunWebSocketData
- sym:43803 BunWebSocketData (src/adapter/bun/websocket.ts:31-35)
  interface BunWebSocketData
- sym:43913 buildSearchParams (src/client/utils.ts:26-47)
  const buildSearchParams = (query: Record<string, string | string[]>) =>
- sym:43571 EventProcessor.createRequest (adapters/aws-lambda/src/handler.ts:322-346)
  createRequest(event: E): Request
- sym:44588 HonoRequest.queries (src/request.ts:166-168)
  queries(key?: string)
- sym:43744 EventProcessor.createRequest (src/adapter/aws-lambda/handler.ts:347-371)
  createRequest(event: E): Request
- sym:44482 detectFromQuery (src/middleware/language/language.ts:137-140)
  const detectFromQuery = (c: Context, options: DetectorOptions): string | undefined =>
- sym:43874 ClientRequestOptions (src/client/types.ts:32-65)
  type ClientRequestOptions<T = unknown> = { fetch?: typeof fetch | HonoRequest webSocket?: (...args: ConstructorParameters<typeof WebSocket>) => WebSocket /** * Standard `RequestInit`, caution that this take highest priority * and could be used to overwrite things that Hono sets for you, like `body | method | headers`. * * If you want to add some headers, use in `headers` instead of `init` */ init?: RequestInit /** * Custom function to serialize query parameters into URLSearchParams. * By default, arrays are serialized as multiple parameters with the same key (e.g., `key=a&key=b`). * You can provide a custom function to change this behavior, for example to use bracket notation (e.g., `key[]=a&key[]=b`). * * @example * ```ts * const client = hc('http://localhost', { * buildSearchParams: (query) => { * return new URLSearchParams(qs.stringify(query)) * } * }) * ``` */ buildSearchParams?: BuildSearchParamsFn } & (keyof T extends never ? { headers?: | Record<string, string> | (() => Record<string, string> | Promise<Record<string, string>>) } : { headers: T | (() => T | Promise<T>) })
- sym:43878 BuildSearch (src/client/types.ts:156-160)
  type BuildSearch<Arg, Key extends 'query'> = Arg extends { [K in Key]: infer Query } ? IsEmptyObject<Query> extends true ? '' : `?${string}` : ''
- sym:44516 prettyJSON (src/middleware/pretty-json/index.ts:48-68)
  const prettyJSON = (options?: PrettyOptions): MiddlewareHandler =>
- sym:44685 parseAccept (src/utils/accept.ts:211-240)
  const parseAccept = (acceptHeader: string): Accept[] =>
- sym:43640 createWSContext (adapters/bun/src/websocket.ts:33-46)
  const createWSContext = (ws: BunServerWebSocket<BunWebSocketData>): WSContext =>
- sym:44426 Encoding (src/middleware/compress/index.ts:13-13)
  type Encoding = (typeof ENCODING_TYPES)[number]
- sym:43598 LatticeV2Processor.getQueryString (adapters/aws-lambda/src/handler.ts:597-599)
  protected getQueryString(): string
- sym:43804 createWSContext (src/adapter/bun/websocket.ts:41-54)
  const createWSContext = (ws: BunServerWebSocket<BunWebSocketData>): WSContext =>
- sym:43691 getAppTemplate (benchmarks/http-server/benchmark.ts:37-52)
  const getAppTemplate = () =>
- sym:44922 getPath (src/utils/url.ts:106-134)
  const getPath = (request: Request): string =>
- sym:43558 ALBProxyEvent (adapters/aws-lambda/src/handler.ts:86-98)
  interface ALBProxyEvent
- sym:43771 LatticeV2Processor.getQueryString (src/adapter/aws-lambda/handler.ts:634-636)
  protected getQueryString(): string
- sym:43731 ALBProxyEvent (src/adapter/aws-lambda/handler.ts:101-113)
  interface ALBProxyEvent
- sym:44581 HonoRequest (src/request.ts:34-448)
  class HonoRequest<P extends string = '/', I extends Input['out'] = {}>
- sym:43588 ALBProcessor (adapters/aws-lambda/src/handler.ts:507-584)
  class ALBProcessor extends EventProcessor<ALBProxyEvent>
- sym:43761 ALBProcessor (src/adapter/aws-lambda/handler.ts:541-618)
  class ALBProcessor extends EventProcessor<ALBProxyEvent>
- sym:44124 HonoOptions (src/hono-base.ts:46-102)
  type HonoOptions<E extends Env> = { /** * `router` option specifies which router to use. * * @see {@link https://hono.dev/docs/api/hono#router-option} * * @example * ```ts * const app = new Hono({ router: new RegExpRouter() }) * ``` */ router?: Router<[H, RouterRoute]> } & ( | { /** * `strict` option specifies whether to distinguish whether the last path is a directory or not. * * @see {@link https://hono.dev/docs/api/hono#strict-mode} * * @default true */ strict?: true /** * `getPath` can handle the host header value. * * @see {@link https://hono.dev/docs/api/routing#routing-with-host-header-value} * * @example * ```ts * const app = new Hono({ * getPath: (req) => * '/' + req.headers.get('host') + req.url.replace(/^https?:\/\/[^/]+(\/[^?]*)/, '$1'), * }) * * app.get('/www1.example.com/hello', () => c.text('hello www1')) * * // A following request will match the route: * // new Request('http://www1.example.com/hello', { * // headers: { host: 'www1.example.com' }, * // }) * ``` */ getPath?: GetPath<E> } | { /** * `strict` option specifies whether to distinguish whether the last path is a directory or not. * When `strict` is `false`, `getPath` is not used. * * @see {@link https://hono.dev/docs/api/hono#strict-mode} * * @default true */ strict: false getPath?: never } )
- sym:43862 ClientRequestImpl (src/client/client.ts:38-148)
  class ClientRequestImpl
- sym:44418 cache (src/middleware/cache/index.ts:183-324)
  const cache = (options: { cacheName: string | ((c: Context) => Promise<string> | string) wait?: boolean cacheControl?: string vary?: string | string[] keyGenerator?: (c: Context) => Promise<string> | string maxQueryBodySize?: number cacheableStatusCodes?: StatusCode[] onCacheNotAvailable?: ((reason: string) => void) | false }): MiddlewareHandler =>
- sym:43697 main (benchmarks/http-server/benchmark.ts:234-324)
  const main = async () =>
- sym:43873 BuildSearchParamsFn (src/client/types.ts:30-30)
  type BuildSearchParamsFn = (query: Record<string, string | string[]>) => URLSearchParams
- sym:44126 MountReplaceRequest (src/hono-base.ts:105-105)
  type MountReplaceRequest = (originalRequest: Request) => Request
- sym:44479 DEFAULT_OPTIONS (src/middleware/language/language.ts:51-68)
  const DEFAULT_OPTIONS: DetectorOptions = {
- sym:43568 EventProcessor (adapters/aws-lambda/src/handler.ts:282-406)
  abstract class EventProcessor<E extends LambdaEvent>
- sym:44604 cloneRawRequest (src/request.ts:481-520)
  const cloneRawRequest = async (req: HonoRequest): Promise<Request> =>
- sym:43592 ALBProcessor.getQueryString (adapters/aws-lambda/src/handler.ts:538-562)
  protected getQueryString(event: ALBProxyEvent): string
- sym:43741 EventProcessor (src/adapter/aws-lambda/handler.ts:307-431)
  abstract class EventProcessor<E extends LambdaEvent>
- sym:43917 deepMerge (src/client/utils.ts:69-85)
  function deepMerge<T>(target: T, source: Record<string, unknown>): T
- sym:44260 PreserveNodeType (src/jsx/dom/render.ts:37-39)
  type PreserveNodeType = | 1 // preserve only self | 2 // preserve self and children
- sym:44599 HonoRequest.url (src/request.ts:374-376)
  get url(): string
- sym:44928 _decodeURI (src/utils/url.ts:225-230)
  const _decodeURI = (value: string): string =>
- sym:44437 buildSimplePostRequestData (src/middleware/csrf/index.test.ts:14-24)
  const buildSimplePostRequestData = (options: { origin?: string; secFetchSite?: string } = {}) =>

---
tokens: 3459/8000
