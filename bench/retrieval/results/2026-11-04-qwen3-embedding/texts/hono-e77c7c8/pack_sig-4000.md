## Task
# hono-e77c7c8 · hono-e77c7c8 · open
Goal: fix(helper/route): match regex params in basePath() the same way as the router (#5498)
Workspace: /root/bench/ws/rebf815-hono-e77c7c8/hono-e77c7c8
Changed (0): nothing yet

## Map
- src/helper/route/index.ts
  L32: const matchedRoutes = (c: Context): RouterRoute[] => // @ts-expect-error c.req[GET_MATCH_RESULT] is not typed
  L58: const routePath = (c: Context, index?: number): string =>
  L82: const baseRoutePath = (c: Context, index?: number): string =>
  L107: const basePath = (c: Context, index?: number): string =>
- src/hono-base.ts
- src/client/utils.ts
  L11: const mergePath = (base: string, path: string) =>
  L18: const replaceUrlParam = (urlString: string, params: Record<string, string | undefined>) =>
  L26: const buildSearchParams = (query: Record<string, string | string[]>) =>
  L49: const replaceUrlProtocol = (urlString: string, protocol: 'ws' | 'http') =>
  L58: const removeIndexString = (urlString: string) =>
  L65: function isObject(item: unknown): item is ObjectType
  L69: function deepMerge<T>(target: T, source: Record<string, unknown>): T
  L95: async function parseResponse<T extends ClientResponse<any>>( fetchRes: T | Promise<T> ): Promise< FilterClientResponseByStatusCode< T, Exclude<ContentfulStatusCode, ClientErrorStatusCode | ServerErrorStatusCode> // Filter out the error responses > extends never ? // Filtered responses does not include any contentful responses, exit with undefined undefined : // Filtered responses includes contentful responses, proceed to infer the type FilterClientResponseByStatusCode< T, Exclude<ContentfulStatusCode, ClientErrorStatusCode | ServerErrorStatusCode> > extends ClientResponse<infer RT, infer _, infer RF> ? RF extends 'json' ? RT : RT extends string ? RT : string : undefined >
- src/context.ts
- src/hono.ts
  L16: class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
  L26: constructor(options: HonoOptions<E> = {})
- src/jsx/context.ts
- src/jsx/dom/render.ts
- src/middleware/cache/index.test.ts
  L7: type StoreMap = Map<string | Request, Response>
  L9: class MockCache
  L13: constructor(name: string, store: StoreMap)
  L18: async match(key: Request | string): Promise<Response | null>
  L22: async keys()
  L26: async put(key: Request | string, response: Response): Promise<void>
  L39: class Context implements ExecutionContext
  L40: passThroughOnException(): void
  L43: async waitUntil(promise: Promise<unknown>): Promise<void>
- src/middleware/cache/index.ts
- src/middleware/method-not-allowed/index.ts
  L13: type MethodNotAllowedHandler<E extends Env> = ( c: Context<E>, allowedMethods: string[] ) => Response | Promise<Response>
  L18: type MethodNotAllowedOptions<E extends Env> = { // eslint-disable-next-line @typescript-eslint/no-explicit-any app: Hono<E, any, any> onMethodNotAllowed?: MethodNotAllowedHandler<E> }
  L59: const methodNotAllowed = <E extends Env = Env>( options: MethodNotAllowedOptions<E> ): MiddlewareHandler<E> =>
- src/preset/quick.ts
  L13: class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
  L18: constructor(options: HonoOptions<E> = {})
- src/preset/tiny.ts
  L11: class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
  L16: constructor(options: HonoOptions<E> = {})
- src/router/linear-router/router.ts
  L5: type RegExpMatchArrayWithIndices = RegExpMatchArray & { indices: [number, number][] }
  L11: class LinearRouter<T> implements Router<T>
  L15: add(method: string, path: string, handler: T)
  L25: match(method: string, path: string): Result<T>
- src/router/pattern-router/router.ts
- src/router/reg-exp-router/matcher.ts
- src/router/reg-exp-router/node.ts
- src/router/smart-router/router.ts
- src/router/trie-router/router.ts
  L5: class TrieRouter<T> implements Router<T>
  L9: add(method: string, path: string, handler: T)
  L15: match(method: string, path: string): Result<T>
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
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:51016 basePath (src/helper/route/index.ts:107-141)
  const basePath = (c: Context, index?: number): string =>
- sym:51015 baseRoutePath (src/helper/route/index.ts:82-83)
  const baseRoutePath = (c: Context, index?: number): string =>
- sym:51580 LinearRouter.match (src/router/linear-router/router.ts:25-151)
  match(method: string, path: string): Result<T>
- sym:51103 Hono.basePath (src/hono-base.ts:250-256)
  basePath<SubPath extends string>( path: SubPath ): Hono<E, S, MergePath<BasePath, SubPath>, MergePath<BasePath, SubPath>>
- sym:51578 LinearRouter (src/router/linear-router/router.ts:11-152)
  class LinearRouter<T> implements Router<T>
- sym:51105 Hono.#addRoute (src/hono-base.ts:390-401)
  #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void
- sym:51584 PatternRouter.match (src/router/pattern-router/router.ts:38-53)
  match(method: string, path: string): Result<T>
- sym:51589 match (src/router/reg-exp-router/matcher.ts:10-33)
  function match<R extends Router<T>, T>(this: R, method: string, path: string): Result<T>
- sym:51099 Hono (src/hono-base.ts:98-547)
  class Hono< E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', CurrentPath extends string = BasePath, >
- sym:51471 methodNotAllowed (src/middleware/method-not-allowed/index.ts:59-141)
  const methodNotAllowed = <E extends Env = Env>( options: MethodNotAllowedOptions<E> ): MiddlewareHandler<E> =>
- sym:51102 Hono.route (src/hono-base.ts:211-235)
  route< SubPath extends string, SubEnv extends Env, SubSchema extends Schema, SubBasePath extends string, SubCurrentPath extends string, >( path: SubPath, app: Hono<SubEnv, SubSchema, SubBasePath, SubCurrentPath> ): Hono<E, MergeSchemaPath<SubSchema, MergePath<BasePath, SubPath>> | S, BasePath, CurrentPath>
- sym:51625 SmartRouter.match (src/router/smart-router/router.ts:21-61)
  match(method: string, path: string): Result<T>
- sym:51541 Hono (src/preset/quick.ts:13-24)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:51612 RegExpRouter (src/router/reg-exp-router/router.ts:48-174)
  class RegExpRouter<T> implements Router<T>
- sym:51607 buildInitParams (src/router/reg-exp-router/prepared-router.ts:95-154)
  const buildInitParams: (params: { paths: string[] }) => ConstructorParameters<typeof PreparedRegExpRouter> = ({ paths }) =>
- sym:51582 PatternRouter (src/router/pattern-router/router.ts:8-54)
  class PatternRouter<T> implements Router<T>
- sym:51480 defaultReplaceRequest (src/middleware/mount/index.ts:21-27)
  const defaultReplaceRequest = (c: Context): Request =>
- sym:51593 PATH_ERROR (src/router/reg-exp-router/node.ts:6-6)
  const PATH_ERROR = Symbol()
- sym:51641 Router (src/router.ts:29-52)
  interface Router<T>
- sym:51610 buildWildcardRegExp (src/router/reg-exp-router/router.ts:22-36)
  function buildWildcardRegExp(path: string): RegExp
- sym:51601 PreparedRegExpRouter (src/router/reg-exp-router/prepared-router.ts:9-93)
  class PreparedRegExpRouter<T> implements Router<T>
- sym:51101 Hono.#clone (src/hono-base.ts:178-187)
  #clone(): Hono<E, S, BasePath, CurrentPath>
- sym:51640 MESSAGE_MATCHER_IS_ALREADY_BUILT (src/router.ts:21-22)
  const MESSAGE_MATCHER_IS_ALREADY_BUILT =
- sym:51095 HonoOptions (src/hono-base.ts:46-87)
  type HonoOptions<E extends Env> = { /** * `strict` option specifies whether to distinguish whether the last path is a directory or not. * * @see {@link https://hono.dev/docs/api/hono#strict-mode} * * @default true */ strict?: boolean /** * `router` option specifies which router to use. * * @see {@link https://hono.dev/docs/api/hono#router-option} * * @example * ```ts * const app = new Hono({ router: new RegExpRouter() }) * ``` */ router?: Router<[H, RouterRoute]> /** * `getPath` can handle the host header value. * * @see {@link https://hono.dev/docs/api/routing#routing-with-host-header-value} * * @example * ```ts * const app = new Hono({ * getPath: (req) => * '/' + req.headers.get('host') + req.url.replace(/^https?:\/\/[^/]+(\/[^?]*)/, '$1'), * }) * * app.get('/www1.example.com/hello', () => c.text('hello www1')) * * // A following request will match the route: * // new Request('http://www1.example.com/hello', { * // headers: { host: 'www1.example.com' }, * // }) * ``` */ getPath?: GetPath<E> }
- sym:51013 matchedRoutes (src/helper/route/index.ts:32-34)
  const matchedRoutes = (c: Context): RouterRoute[] => // @ts-expect-error c.req[GET_MATCH_RESULT] is not typed
- sym:51894 getPathNoStrict (src/utils/url.ts:154-159)
  const getPathNoStrict = (request: Request): string =>
- sym:51597 Node (src/router/reg-exp-router/node.ts:47-170)
  class Node
- sym:50882 mergePath (src/client/utils.ts:11-16)
  const mergePath = (base: string, path: string) =>
- sym:50921 Context (src/context.ts:293-797)
  class Context< // eslint-disable-next-line @typescript-eslint/no-explicit-any E extends Env = any, // eslint-disable-next-line @typescript-eslint/no-explicit-any P extends string = any, I extends Input = {}, >
- sym:50923 Context.req (src/context.ts:366-369)
  get req(): HonoRequest<P, I['out']>
- sym:51111 Hono (src/hono.ts:16-34)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:51169 Context (src/jsx/context.ts:8-11)
  interface Context<T> extends FC<PropsWithChildren<{ value: T }>>
- sym:51237 Context (src/jsx/dom/render.ts:91-104)
  type Context = | [ PendingType, // PendingType boolean, // got an error UpdateHook, // update hook boolean, // is in view transition boolean, // is in top level render [Context, Function, NodeObject][], // [context, error handler, node] stack for this context ] | [PendingType, boolean, UpdateHook, boolean] | [PendingType, boolean, UpdateHook] | [PendingType, boolean] | [PendingType] | []
- sym:51388 cache (src/middleware/cache/index.ts:183-324)
  const cache = (options: { cacheName: string | ((c: Context) => Promise<string> | string) wait?: boolean cacheControl?: string vary?: string | string[] keyGenerator?: (c: Context) => Promise<string> | string maxQueryBodySize?: number cacheableStatusCodes?: StatusCode[] onCacheNotAvailable?: ((reason: string) => void) | false }): MiddlewareHandler =>
- sym:51543 Hono (src/preset/tiny.ts:11-20)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:51595 Context (src/router/reg-exp-router/node.ts:9-11)
  interface Context
- sym:51635 TrieRouter.match (src/router/trie-router/router.ts:15-17)
  match(method: string, path: string): Result<T>
- sym:51885 splitRoutingPath (src/utils/url.ts:16-21)
- sym:51888 getPattern (src/utils/url.ts:51-78)
- sym:51895 mergePath (src/utils/url.ts:171-182)
- sym:51896 checkOptionalParameter (src/utils/url.ts:184-219)
- sym:50522 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
- sym:50523 getConnInfo (adapters/aws-lambda/src/conninfo.ts:45-73)
- sym:50536 streamHandle (adapters/aws-lambda/src/handler.ts:142-197)
- sym:50538 handle (adapters/aws-lambda/src/handler.ts:243-280)
- sym:50603 getConnInfo (adapters/bun/src/conninfo.ts:10-43)
- sym:50605 getBunServer (adapters/bun/src/server.ts:13-14)
- sym:50613 getConnInfo (adapters/cloudflare-workers/src/conninfo.ts:3-7)
- sym:50624 Env (adapters/lambda-edge/src/conninfo.ts:5-9)
- sym:50625 getConnInfo (adapters/lambda-edge/src/conninfo.ts:11-15)
- sym:50640 handle (adapters/lambda-edge/src/handler.ts:115-147)
- sym:50648 Env (adapters/netlify/src/conninfo.ts:28-32)
- sym:50649 getConnInfo (adapters/netlify/src/conninfo.ts:53-57)
- sym:50650 handle (adapters/netlify/src/handler.ts:4-10)
- sym:50653 handle (adapters/service-worker/src/handler.ts:18-37)
- sym:51576 runTest (src/router/common.case.test.ts:14-1118)
- sym:51375 MockCache.match (src/middleware/cache/index.test.ts:18-20)
- sym:51378 Context (src/middleware/cache/index.test.ts:39-48)
  class Context implements ExecutionContext

---
tokens: 3582/4000
