## Task
# hono-08807ce · hono-08807ce-2 · open
Goal: fix(trie-router): keep first value for duplicate path params (#5422) (#5423)
Workspace: /root/bench/ws/rebf815-hono-08807ce/hono-08807ce-2
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
- sym:50235 Node (src/router/trie-router/node.ts:20-217)
  class Node<T>
- sym:50237 Node.#pushHandlerSets (src/router/trie-router/node.ts:64-84)
  #pushHandlerSets( handlerSets: HandlerParamsSet<T>[], node: Node<T>, method: string, nodeParams: Record<string, string>, params?: Record<string, string> ): void
- sym:50213 buildInitParams (src/router/reg-exp-router/prepared-router.ts:95-154)
  const buildInitParams: (params: { paths: string[] }) => ConstructorParameters<typeof PreparedRegExpRouter> = ({ paths }) =>
- sym:50239 TrieRouter (src/router/trie-router/router.ts:5-18)
  class TrieRouter<T> implements Router<T>
- sym:50241 TrieRouter.match (src/router/trie-router/router.ts:15-17)
  match(method: string, path: string): Result<T>
- sym:50240 TrieRouter.add (src/router/trie-router/router.ts:9-13)
  add(method: string, path: string, handler: T)
- sym:50223 RegExpRouter.#buildMatcher (src/router/reg-exp-router/router.ts:145-173)
  #buildMatcher(method: string): Matcher<T>
- sym:50218 RegExpRouter (src/router/reg-exp-router/router.ts:48-174)
  class RegExpRouter<T> implements Router<T>
- sym:50226 Trie.insert (src/router/reg-exp-router/trie.ts:14-57)
  insert(path: string, isStatic: boolean): void
- sym:50214 serializeInitParams (src/router/reg-exp-router/prepared-router.ts:156-165)
  const serializeInitParams: ( params: ConstructorParameters<typeof PreparedRegExpRouter> ) => string = ([matchers, relocateMap]) =>
- sym:50225 Trie (src/router/reg-exp-router/trie.ts:7-84)
  class Trie
- sym:50190 PatternRouter.match (src/router/pattern-router/router.ts:38-53)
  match(method: string, path: string): Result<T>
- sym:50186 LinearRouter.match (src/router/linear-router/router.ts:25-151)
  match(method: string, path: string): Result<T>
- sym:50184 LinearRouter (src/router/linear-router/router.ts:11-152)
  class LinearRouter<T> implements Router<T>
- sym:50236 Node.insert (src/router/trie-router/node.ts:28-62)
  insert(method: string, path: string, handler: T): void
- sym:50221 RegExpRouter.add (src/router/reg-exp-router/router.ts:68-127)
  add(method: string, path: string, handler: T)
- sym:50188 PatternRouter (src/router/pattern-router/router.ts:8-54)
  class PatternRouter<T> implements Router<T>
- sym:50224 ReplacementMap (src/router/reg-exp-router/trie.ts:5-5)
  type ReplacementMap = number[]
- sym:50233 HandlerSet (src/router/trie-router/node.ts:7-11)
  type HandlerSet<T> = { handler: T possibleKeys: string[] score: number }
- sym:50227 Trie.buildRegExp (src/router/reg-exp-router/trie.ts:59-83)
  buildRegExp(): [RegExp, ReplacementMap, ReplacementMap]
- sym:49999 except (src/middleware/combine/index.ts:147-171)
  const except = ( condition: string | Condition | (string | Condition)[], ...middleware: MiddlewareHandler[] ): MiddlewareHandler =>
- sym:49701 HonoOptions (src/hono-base.ts:46-87)
  type HonoOptions<E extends Env> = { /** * `strict` option specifies whether to distinguish whether the last path is a directory or not. * * @see {@link https://hono.dev/docs/api/hono#strict-mode} * * @default true */ strict?: boolean /** * `router` option specifies which router to use. * * @see {@link https://hono.dev/docs/api/hono#router-option} * * @example * ```ts * const app = new Hono({ router: new RegExpRouter() }) * ``` */ router?: Router<[H, RouterRoute]> /** * `getPath` can handle the host header value. * * @see {@link https://hono.dev/docs/api/routing#routing-with-host-header-value} * * @example * ```ts * const app = new Hono({ * getPath: (req) => * '/' + req.headers.get('host') + req.url.replace(/^https?:\/\/[^/]+(\/[^?]*)/, '$1'), * }) * * app.get('/www1.example.com/hello', () => c.text('hello www1')) * * // A following request will match the route: * // new Request('http://www1.example.com/hello', { * // headers: { host: 'www1.example.com' }, * // }) * ``` */ getPath?: GetPath<E> }
- sym:50250 Params (src/router.ts:65-65)
  type Params = Record<string, string>
- sym:50147 Hono (src/preset/quick.ts:13-24)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:49441 hc (src/client/client.ts:151-249)
  const hc = <T extends Hono<any, any, any>, Prefix extends string = string>( baseUrl: Prefix, options?: ClientRequestOptions ) =>
- sym:50219 RegExpRouter.constructor (src/router/reg-exp-router/router.ts:54-58)
  constructor()
- sym:49717 Hono (src/hono.ts:16-34)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:50251 Result (src/router.ts:98-98)
  type Result<T> = [[T, ParamIndexMap][], ParamStash] | [[T, Params][]]
- sym:50077 methodNotAllowed (src/middleware/method-not-allowed/index.ts:59-141)
  const methodNotAllowed = <E extends Env = Env>( options: MethodNotAllowedOptions<E> ): MiddlewareHandler<E> =>
- sym:49386 Params (src/adapter/cloudflare-pages/handler.ts:9-9)
  type Params<P extends string = any> = Record<P, string | string[]>
- sym:49466 TypedURL (src/client/types.ts:215-230)
  interface TypedURL< Protocol extends string, Hostname extends string, Port extends string, Pathname extends string, Search extends string, > extends URL
- sym:49840 Node (src/jsx/dom/render.ts:80-80)
  type Node = NodeString | NodeObject
- sym:50176 HonoRequest.method (src/request.ts:390-392)
  get method(): string
- sym:50243 METHOD_NAME_ALL (src/router.ts:9-9)
  const METHOD_NAME_ALL = 'ALL' as const
- sym:50195 match (src/router/reg-exp-router/matcher.ts:10-33)
  function match<R extends Router<T>, T>(this: R, method: string, path: string): Result<T>
- sym:50203 Node (src/router/reg-exp-router/node.ts:47-170)
  class Node
- sym:50231 SmartRouter.match (src/router/smart-router/router.ts:21-61)
  match(method: string, path: string): Result<T>
- sym:50242 createNullObject (src/router/utils.ts:1-1)
  const createNullObject = () =>
- sym:50348 escapeToBuffer (src/utils/html.ts:90-127)
  const escapeToBuffer = (str: string, buffer: StringBuffer): void =>
- sym:50490 splitPath (src/utils/url.ts:8-14)
  const splitPath = (path: string): string[] =>
- sym:50182 runTest (src/router/common.case.test.ts:14-1118)
  const runTest = ({ skip = [], newRouter, }: { skip?: { reason: string tests: string[] }[] newRouter: <T>() => Router<T> }) =>
- sym:49981 MockCache.match (src/middleware/cache/index.test.ts:18-20)
  async match(key: Request | string): Promise<Response | null>

---
tokens: 2622/8000
