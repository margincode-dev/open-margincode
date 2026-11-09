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
- sym:43754 EventV1Processor (src/adapter/aws-lambda/handler.ts:478-534)
  class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
- sym:43581 EventV1Processor (adapters/aws-lambda/src/handler.ts:447-503)
  class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
- sym:44417 createQueryDigest (src/middleware/cache/index.ts:95-153)
  const createQueryDigest = async ( c: Context, maxQueryBodySize: number ): Promise<string | undefined> =>
- sym:44477 DetectorOptions (src/middleware/language/language.ts:13-45)
  interface DetectorOptions
- sym:43584 EventV1Processor.getQueryString (adapters/aws-lambda/src/handler.ts:456-472)
  protected getQueryString(event: APIGatewayProxyEvent): string
- sym:43757 EventV1Processor.getQueryString (src/adapter/aws-lambda/handler.ts:487-503)
  protected getQueryString(event: APIGatewayProxyEvent): string
- sym:44430 compress (src/middleware/compress/index.ts:72-136)
  const compress = (options?: CompressionOptions): MiddlewareHandler =>
- sym:44414 createCacheKey (src/middleware/cache/index.ts:51-70)
  const createCacheKey = ( key: string, requestUrl: string, request: CacheKeyRequest, varyHeaders: [string, string][] ): string =>
- sym:44551 ENCODINGS_ORDERED_KEYS (src/middleware/serve-static/index.ts:31-31)
  const ENCODINGS_ORDERED_KEYS = Object.keys(ENCODINGS) as (keyof typeof ENCODINGS)[]
- sym:44587 HonoRequest.query (src/request.ts:147-149)
  query(key?: string)
- sym:44599 HonoRequest.url (src/request.ts:374-376)
  get url(): string
- sym:44928 _decodeURI (src/utils/url.ts:225-230)
  const _decodeURI = (value: string): string =>
- sym:44930 getQueryParams (src/utils/url.ts:328-333)
  const getQueryParams = ( url: string, key?: string ): string[] | undefined | Record<string, string[]> =>

---
tokens: 1352/2000
