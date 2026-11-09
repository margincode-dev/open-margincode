## Task
# hono-08807ce · hono-08807ce · open
Goal: fix(trie-router): keep first value for duplicate path params (#5422) (#5423)
Workspace: /root/bench/av_tei/ws/rec09fc-hono-08807ce/hono-08807ce
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
- sym:50238 Node.search (src/router/trie-router/node.ts:86-216)
  search(method: string, path: string): [[T, Params][]]
- sym:50234 HandlerParamsSet (src/router/trie-router/node.ts:13-15)
  type HandlerParamsSet<T> = HandlerSet<T> & { params: Record<string, string> }
- sym:50225 Trie (src/router/reg-exp-router/trie.ts:7-84)
  class Trie
- sym:50226 Trie.insert (src/router/reg-exp-router/trie.ts:14-57)
  insert(path: string, isStatic: boolean): void
- sym:50224 ReplacementMap (src/router/reg-exp-router/trie.ts:5-5)
  type ReplacementMap = number[]
- sym:50235 Node (src/router/trie-router/node.ts:20-217)
  class Node<T>
- sym:50210 PreparedRegExpRouter.#addPath (src/router/reg-exp-router/prepared-router.ts:25-45)
  #addPath( method: string, path: string, handler: T, indexes: (number | string)[], map: ParamIndexMap | undefined )
- sym:50237 Node.#pushHandlerSets (src/router/trie-router/node.ts:64-84)
  #pushHandlerSets( handlerSets: HandlerParamsSet<T>[], node: Node<T>, method: string, nodeParams: Record<string, string>, params?: Record<string, string> ): void
- sym:50213 buildInitParams (src/router/reg-exp-router/prepared-router.ts:95-154)
  const buildInitParams: (params: { paths: string[] }) => ConstructorParameters<typeof PreparedRegExpRouter> = ({ paths }) =>
- sym:50189 PatternRouter.add (src/router/pattern-router/router.ts:12-36)
  add(method: string, path: string, handler: T)
- sym:50239 TrieRouter (src/router/trie-router/router.ts:5-18)
  class TrieRouter<T> implements Router<T>
- sym:50241 TrieRouter.match (src/router/trie-router/router.ts:15-17)
  match(method: string, path: string): Result<T>
- sym:50227 Trie.buildRegExp (src/router/reg-exp-router/trie.ts:59-83)
  buildRegExp(): [RegExp, ReplacementMap, ReplacementMap]
- sym:50240 TrieRouter.add (src/router/trie-router/router.ts:9-13)
  add(method: string, path: string, handler: T)
- sym:49386 Params (src/adapter/cloudflare-pages/handler.ts:9-9)
  type Params<P extends string = any> = Record<P, string | string[]>
- sym:49466 TypedURL (src/client/types.ts:215-230)
  interface TypedURL< Protocol extends string, Hostname extends string, Port extends string, Pathname extends string, Search extends string, > extends URL
- sym:49840 Node (src/jsx/dom/render.ts:80-80)
  type Node = NodeString | NodeObject
- sym:49981 MockCache.match (src/middleware/cache/index.test.ts:18-20)
  async match(key: Request | string): Promise<Response | null>

---
tokens: 1415/2000
