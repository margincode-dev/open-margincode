## Task
# hono-e9c0107 · hono-e9c0107-2 · open
Goal: fix(hono-base): disallow `getPath` when `strict` is `false` (#5563)

## Map
- adapters/aws-lambda/src/handler.ts
- src/hono-base.ts
- src/utils/url.ts
- adapters/aws-lambda/src/conninfo.ts
- adapters/lambda-edge/src/conninfo.ts
- adapters/netlify/src/conninfo.ts
- src/adapter/aws-lambda/conninfo.ts
- src/adapter/aws-lambda/handler.ts
- src/context.ts

## Code
- sym:48306 GetPath (src/hono-base.ts:44-44)
  type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string
- sym:48307 HonoOptions (src/hono-base.ts:46-87)
- sym:48312 Hono.constructor (src/hono-base.ts:127-176)
  constructor(options: HonoOptions<E> = {})
- sym:48313 Hono.#clone (src/hono-base.ts:178-187)
  #clone(): Hono<E, S, BasePath, CurrentPath>
- sym:49106 getPathNoStrict (src/utils/url.ts:154-159)
- sym:48316 Hono.mount (src/hono-base.ts:333-388)
- sym:47758 EventV2Processor.getPath (adapters/aws-lambda/src/handler.ts:409-411)
- sym:47765 EventV1Processor.getPath (adapters/aws-lambda/src/handler.ts:448-450)
- sym:49104 getPath (src/utils/url.ts:106-134)
- sym:48311 Hono (src/hono-base.ts:98-547)
- sym:48315 Hono.basePath (src/hono-base.ts:250-256)
- sym:47734 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
- sym:47739 APIGatewayProxyEventV2 (adapters/aws-lambda/src/handler.ts:42-62)
  interface APIGatewayProxyEventV2
- sym:47740 APIGatewayProxyEvent (adapters/aws-lambda/src/handler.ts:65-83)
- sym:47754 EventProcessor.createRequest (adapters/aws-lambda/src/handler.ts:322-346)

---
tokens: 703/800
omitted 38 lower-ranked items (expand an id for detail):
- src/adapter/cloudflare-pages/handler.test.ts
- src/adapter/lambda-edge/conninfo.ts
- src/adapter/netlify/conninfo.ts
- src/helper/adapter/index.ts
- src/hono.test.ts
- src/request.ts
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)
- sym:47836 Env (adapters/lambda-edge/src/conninfo.ts:5-9)
- sym:47860 Env (adapters/netlify/src/conninfo.ts:28-32)
- sym:47907 Env (src/adapter/aws-lambda/conninfo.ts:14-18)
- … and 28 more
