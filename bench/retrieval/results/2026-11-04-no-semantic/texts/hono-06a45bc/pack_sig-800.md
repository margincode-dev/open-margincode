## Task
# hono-06a45bc · hono-06a45bc-2 · open

## Map
- src/middleware/basic-auth/index.ts
- src/middleware/bearer-auth/index.ts
- src/context.ts
- src/jsx/context.ts
- src/jsx/dom/render.ts
- src/middleware/cache/index.test.ts
- src/router/reg-exp-router/node.ts
- adapters/aws-lambda/src/conninfo.ts
- adapters/bun/src/conninfo.ts
- adapters/bun/src/server.ts
- adapters/lambda-edge/src/conninfo.ts
- adapters/netlify/src/conninfo.ts
- src/client/types.ts

## Code
- sym:45787 MessageFunction (src/middleware/bearer-auth/index.ts:16-16)
  type MessageFunction = (c: Context) => string | object | Promise<string | object>
- sym:45781 MessageFunction (src/middleware/basic-auth/index.ts:12-12)
  type MessageFunction = (c: Context) => string | object | Promise<string | object>
- sym:45790 bearerAuth (src/middleware/bearer-auth/index.ts:104-221)
- sym:45789 BearerAuthOptions (src/middleware/bearer-auth/index.ts:22-66)
- sym:45788 CustomizedErrorResponseOptions (src/middleware/bearer-auth/index.ts:17-20)
- sym:45782 BasicAuthOptions (src/middleware/basic-auth/index.ts:14-29)
- sym:45263 detectResponseType (src/client/fetch-result-please.ts:80-95)
- sym:45864 jwk (src/middleware/jwk/jwk.ts:49-174)
- sym:45345 Context (src/context.ts:293-797)
- sym:45593 Context (src/jsx/context.ts:8-11)
- sym:45661 Context (src/jsx/dom/render.ts:91-104)
- sym:46019 Context (src/router/reg-exp-router/node.ts:9-11)
  interface Context
- sym:44946 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
- sym:44947 getConnInfo (adapters/aws-lambda/src/conninfo.ts:45-73)
- sym:45027 getConnInfo (adapters/bun/src/conninfo.ts:10-43)
- sym:45285 Response (src/client/types.ts:232-232)

---
tokens: 724/800
omitted 41 lower-ranked items (expand an id for detail):
- src/adapter/aws-lambda/conninfo.ts
- src/adapter/bun/conninfo.ts
- src/adapter/bun/server.ts
- src/adapter/cloudflare-pages/handler.test.ts
- src/adapter/cloudflare-pages/handler.ts
- src/adapter/lambda-edge/conninfo.ts
- src/adapter/netlify/conninfo.ts
- src/compose.ts
- src/helper/accepts/accepts.ts
- src/helper/adapter/index.ts
- src/helper/conninfo/types.ts
- src/helper/cookie/index.ts
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)
- sym:45029 getBunServer (adapters/bun/src/server.ts:13-14)
- … and 27 more
