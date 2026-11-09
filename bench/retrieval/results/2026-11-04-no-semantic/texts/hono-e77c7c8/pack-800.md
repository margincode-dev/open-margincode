## Task
# hono-e77c7c8 · hono-e77c7c8-2 · open
Goal: fix(helper/route): match regex params in basePath() the same way as the router (#5498)

## Map
- src/helper/route/index.ts
- src/hono-base.ts
- src/client/utils.ts
- src/context.ts
- src/hono.ts
- src/jsx/context.ts
- src/jsx/dom/render.ts
- src/middleware/cache/index.test.ts
- src/middleware/cache/index.ts
- src/middleware/method-not-allowed/index.ts
- src/preset/quick.ts

## Code
- sym:51016 basePath (src/helper/route/index.ts:107-141)
  const basePath = (c: Context, index?: number): string =>
- sym:51105 Hono.#addRoute (src/hono-base.ts:390-401)
  #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void
- sym:51584 PatternRouter.match (src/router/pattern-router/router.ts:38-53)
- sym:51099 Hono (src/hono-base.ts:98-547)
- sym:51580 LinearRouter.match (src/router/linear-router/router.ts:25-151)
- sym:51015 baseRoutePath (src/helper/route/index.ts:82-83)
- sym:51103 Hono.basePath (src/hono-base.ts:250-256)
- sym:51582 PatternRouter (src/router/pattern-router/router.ts:8-54)
  class PatternRouter<T> implements Router<T>
- sym:51578 LinearRouter (src/router/linear-router/router.ts:11-152)
  class LinearRouter<T> implements Router<T>
- sym:50882 mergePath (src/client/utils.ts:11-16)
- sym:50921 Context (src/context.ts:293-797)
- sym:50923 Context.req (src/context.ts:366-369)
- sym:51102 Hono.route (src/hono-base.ts:211-235)
- sym:51111 Hono (src/hono.ts:16-34)
- sym:51169 Context (src/jsx/context.ts:8-11)
- sym:51237 Context (src/jsx/dom/render.ts:91-104)
- sym:51388 cache (src/middleware/cache/index.ts:183-324)

---
tokens: 709/800
omitted 46 lower-ranked items (expand an id for detail):
- src/preset/tiny.ts
- src/router/linear-router/router.ts
- src/router/pattern-router/router.ts
- src/router/reg-exp-router/matcher.ts
- src/router/reg-exp-router/node.ts
- src/router/smart-router/router.ts
- src/router/trie-router/router.ts
- src/utils/url.ts
- adapters/aws-lambda/src/conninfo.ts
- adapters/aws-lambda/src/handler.ts
- adapters/bun/src/conninfo.ts
- adapters/bun/src/server.ts
- adapters/cloudflare-workers/src/conninfo.ts
- adapters/lambda-edge/src/conninfo.ts
- adapters/lambda-edge/src/handler.ts
- adapters/netlify/src/conninfo.ts
- adapters/netlify/src/handler.ts
- adapters/service-worker/src/handler.ts
- … and 28 more
