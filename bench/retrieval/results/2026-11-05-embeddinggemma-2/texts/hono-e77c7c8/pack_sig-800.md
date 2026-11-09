## Task
# hono-e77c7c8 · hono-e77c7c8 · open
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
- src/preset/tiny.ts

## Code
- sym:51015 baseRoutePath (src/helper/route/index.ts:82-83)
  const baseRoutePath = (c: Context, index?: number): string =>
- sym:51016 basePath (src/helper/route/index.ts:107-141)
  const basePath = (c: Context, index?: number): string =>
- sym:51105 Hono.#addRoute (src/hono-base.ts:390-401)
- sym:51584 PatternRouter.match (src/router/pattern-router/router.ts:38-53)
  match(method: string, path: string): Result<T>
- sym:51589 match (src/router/reg-exp-router/matcher.ts:10-33)
- sym:51099 Hono (src/hono-base.ts:98-547)
- sym:51580 LinearRouter.match (src/router/linear-router/router.ts:25-151)
- sym:51103 Hono.basePath (src/hono-base.ts:250-256)
- sym:51617 RegExpRouter.#buildMatcher (src/router/reg-exp-router/router.ts:145-173)
- sym:51604 PreparedRegExpRouter.#addPath (src/router/reg-exp-router/prepared-router.ts:25-45)
- sym:51603 PreparedRegExpRouter.#addWildcard (src/router/reg-exp-router/prepared-router.ts:19-23)
- sym:51582 PatternRouter (src/router/pattern-router/router.ts:8-54)
- sym:51602 PreparedRegExpRouter.constructor (src/router/reg-exp-router/prepared-router.ts:14-17)
- sym:51427 IS_CIDR_NOTATION_REGEX (src/middleware/ip-restriction/index.ts:40-40)
- sym:51578 LinearRouter (src/router/linear-router/router.ts:11-152)

---
tokens: 700/800
omitted 53 lower-ranked items (expand an id for detail):
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
- … and 36 more
