## Task
# hono-e77c7c8 · hono-e77c7c8-2 · open
Goal: fix(helper/route): match regex params in basePath() the same way as the router (#5498)
Workspace: /root/bench/ws/rebf815-hono-e77c7c8/hono-e77c7c8-2
Changed (0): nothing yet

## Map
- src/helper/route/index.ts
  L32: const matchedRoutes = (c: Context): RouterRoute[] => // @ts-expect-error c.req[GET_MATCH_RESULT] is not typed
  L58: const routePath = (c: Context, index?: number): string =>
  L82: const baseRoutePath = (c: Context, index?: number): string =>
  L107: const basePath = (c: Context, index?: number): string =>
- src/hono-base.ts
  L31: const notFoundHandler: NotFoundHandler = (c) =>
  L35: const errorHandler: ErrorHandler = (err, c) =>
  L44: type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string
  L46: type HonoOptions<E extends Env> = { /** * `strict` option specifies whether to distinguish whether the last path is a directory or not. * * @see {@link https://hono.dev/docs/api/hono#strict-mode} * * @default true */ strict?: boolean /** * `router` option specifies which router to use. * * @see {@link https://hono.dev/docs/api/hono#router-option} * * @example * ```ts * const app = new Hono({ router: new RegExpRouter() }) * ``` */ router?: Router<[H, RouterRoute]> /** * `getPath` can handle the host header value. * * @see {@link https://hono.dev/docs/api/routing#routing-with-host-header-value} * * @example * ```ts * const app = new Hono({ * getPath: (req) => * '/' + req.headers.get('host') + req.url.replace(/^https?:\/\/[^/]+(\/[^?]*)/, '$1'), * }) * * app.get('/www1.example.com/hello', () => c.text('hello www1')) * * // A following request will match the route: * // new Request('http://www1.example.com/hello', { * // headers: { host: 'www1.example.com' }, * // }) * ``` */ getPath?: GetPath<E> }
  L89: type MountOptionHandler = (c: Context) => unknown
  L90: type MountReplaceRequest = (originalRequest: Request) => Request
  L91: type MountOptions = | MountOptionHandler | { optionHandler?: MountOptionHandler replaceRequest?: MountReplaceRequest | false }
  L98: class Hono< E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', CurrentPath extends string = BasePath, >
  L127: constructor(options: HonoOptions<E> = {})
  L178: #clone(): Hono<E, S, BasePath, CurrentPath>
  L211: route< SubPath extends string, SubEnv extends Env, SubSchema extends Schema, SubBasePath extends string, SubCurrentPath extends string, >( path: SubPath, app: Hono<SubEnv, SubSchema, SubBasePath, SubCurrentPath> ): Hono<E, MergeSchemaPath<SubSchema, MergePath<BasePath, SubPath>> | S, BasePath, CurrentPath>
  L250: basePath<SubPath extends string>( path: SubPath ): Hono<E, S, MergePath<BasePath, SubPath>, MergePath<BasePath, SubPath>>
  L333: mount( path: string, applicationHandler: (request: Request, ...args: any) => Response | Promise<Response>, options?: MountOptions ): Hono<E, S, BasePath, CurrentPath>
  L390: #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void
  L403: #handleError(err: unknown, c: Context<E>): Response | Promise<Response>
  L410: #dispatch( request: Request, executionCtx: ExecutionContext | FetchEventLike | undefined, env: E['Bindings'], method: string ): Response | Promise<Response>
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
  L18: type HeaderRecord = | Record<'Content-Type', BaseMime> | Record<ResponseHeader, string | string[]> | Record<string, string | string[]>
  L26: type Data = string | ArrayBuffer | Blob | ReadableStream | Uint8Array<ArrayBuffer>
  L31: interface ExecutionContext
  L57: interface ContextVariableMap
  L62: interface ContextRenderer
  L71: interface DefaultRenderer
  L78: type Renderer = ContextRenderer extends Function ? ContextRenderer : DefaultRenderer
  L83: type PropsForRenderer = [...Required<Parameters<Renderer>>] extends [unknown, infer Props] ? Props : unknown
  L88: type Layout<T = Record<string, any>> = (props: T) => any
  L95: interface Get<E extends Env>
  L105: interface Set<E extends Env>
  L113: interface NewResponse
  L121: interface BodyRespond
  L156: interface TextRespond
  L181: interface JSONRespond
  L205: type JSONRespondReturn< T extends JSONValue | {} | InvalidJSONValue, U extends ContentfulStatusCode, > = Response & TypedResponse<JSONParsed<T>, U, 'json'>
  L220: interface HTMLRespond
  L237: type ContextOptions<E extends Env> = { /** * Bindings for the environment. */ env: E['Bindings'] /** * Execution context for the request. */ executionCtx?: FetchEventLike | ExecutionContext | undefined /** * Handler for not found responses. */ notFoundHandler?: NotFoundHandler<E> matchResult?: Result<[H, RouterRoute]> path?: string }
  L254: interface SetHeadersOptions
  L258: interface SetHeaders
  L264: type ResponseHeadersInit = | [string, string][] | Record<'Content-Type', BaseMime> | Record<ResponseHeader, string> | Record<string, string> | Headers
  L271: interface ResponseInit<T extends StatusCode = StatusCode>
  L277: type ResponseOrInit<T extends StatusCode = StatusCode> = ResponseInit<T> | Response
  L279: const TEXT_PLAIN = 'text/plain; charset=UTF-8'
  L281: const setDefaultContentType = (contentType: string, headers?: HeaderRecord): HeaderRecord =>
  L288: const createResponseInstance = ( body?: BodyInit | null | undefined, init?: globalThis.ResponseInit ): Response =>
  L293: class Context< // eslint-disable-next-line @typescript-eslint/no-explicit-any E extends Env = any, // eslint-disable-next-line @typescript-eslint/no-explicit-any P extends string = any, I extends Input = {}, >
  L352: constructor(req: Request, options?: ContextOptions<E>)
  L366: get req(): HonoRequest<P, I['out']>
  L377: get event(): FetchEventLike
- src/hono.ts
  L16: class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
  L26: constructor(options: HonoOptions<E> = {})
- src/jsx/context.ts
  L8: interface Context<T> extends FC<PropsWithChildren<{ value: T }>>
  L16: type RenderStore = WeakMap<Context<unknown>, unknown[]>
  L18: type AsyncLocalStorageLike<T> = { getStore(): T | undefined run<R>(store: T, callback: () => R): R }
  L22: type AsyncLocalStorageClass = new <T>() => AsyncLocalStorageLike<T>
  L23: type AsyncHooksModule = { AsyncLocalStorage?: AsyncLocalStorageClass }
  L34: const loadAsyncLocalStorage = (): AsyncLocalStorageLike<RenderStore> | undefined =>
  L69: const getCurrentStore = (): RenderStore | undefined =>
  L73: const warnIfStorelessAccess = (): void =>
  L84: const getContextValuesIn = <T>(store: RenderStore | undefined, context: Context<T>): T[] =>
  L100: const readContextValueIn = <T>(store: RenderStore | undefined, context: Context<T>): T =>
  L109: type LocalContexts = [Context<unknown>, unknown][]
  L111: const captureContextValues = (store: RenderStore | undefined): LocalContexts =>
  L117: const resumeWithContextValues = <T>( callback: () => T, store: RenderStore | undefined, contexts: LocalContexts ): T =>
  L161: const runWithRenderContext = <T>(callback: () => T, resumeStore?: RenderStore): T =>
  L196: const captureRenderContext = (): (<T>(callback: () => T) => T) =>
  L213: const createContext = <T>(defaultValue: T): Context<T> =>
  L264: const useContext = <T>(context: Context<T>): T =>
- src/jsx/dom/render.ts
  L17: const HONO_PORTAL_ELEMENT = '_hp'
  L30: type HasRenderToDom = FC<any> & { [DOM_RENDERER]: FC<any> }
  L32: type ErrorHandler = (error: any, retry: () => void) => Child | undefined
  L34: type Container = HTMLElement | DocumentFragment
  L35: type LocalJSXContexts = [JSXContext<unknown>, unknown][] | undefined
  L36: type SupportedElement = HTMLElement | SVGElement | MathMLElement
  L37: type PreserveNodeType = | 1 // preserve only self | 2 // preserve self and children
  L41: type NodeObject = { pP: Props | undefined // previous props nN: Node | undefined // next node vC: Node[] // virtual dom children pC?: Node[] // previous virtual dom children vR: Node[] // virtual dom children to remove n?: string // namespace f?: boolean // force build s?: boolean // skip build and apply c: Container | undefined // container e: SupportedElement | Text | undefined // rendered element p?: PreserveNodeType // preserve HTMLElement if it will be unmounted a?: boolean // cancel apply() if true o?: NodeObject // original node [DOM_STASH]: | [ number, // current hook index // eslint-disable-next-line @typescript-eslint/no-explicit-any any[][], // stash for hooks LocalJSXContexts, // context [Context, Function, NodeObject], // [context, error handler, node] for closest error boundary or suspense ] // eslint-disable-next-line @typescript-eslint/no-explicit-any | [number, any[][]] } & JSXNode
  L66: type NodeString = { t: string // text content d: boolean // is dirty s?: boolean // skip build and apply } & { e?: Text // like a NodeObject vC: undefined nN: undefined p?: true // from JSXNode key: undefined tag: undefined }
  L80: type Node = NodeString | NodeObject
  L82: type PendingType = | 0 // no pending | 1 // global | 2 // hook
  L86: type UpdateHook = ( context: Context, node: Node, cb: (context: Context) => void ) => Promise<void>
  L91: type Context = | [ PendingType, // PendingType boolean, // got an error UpdateHook, // update hook boolean, // is in view transition boolean, // is in top level render [Context, Function, NodeObject][], // [context, error handler, node] stack for this context ] | [PendingType, boolean, UpdateHook, boolean] | [PendingType, boolean, UpdateHook] | [PendingType, boolean] | [PendingType] | []
  L111: const getNameSpaceContext = () =>
  L113: const isNodeString = (node: Node): node is NodeString =>
  L119: const getEventSpec = (key: string): [string, boolean] | undefined =>
  L135: const toAttributeName = (element: SupportedElement, key: string): string =>
  L144: const normalizeFormValue = (value: Props['value']) =>
  L147: const applySelectValue = (select: HTMLSelectElement, props: Props): void =>
  L161: const isIgnorableAttributeError = (error: unknown): boolean =>
  L164: const applyProps = ( container: SupportedElement, attributes: Props, oldAttributes?: Props ): void =>
  L276: const invokeTag = (context: Context, node: NodeObject): Child[] =>
  L295: const getNextChildren = ( node: NodeObject, container: Container, nextChildren: Node[], childrenToRemove: Node[], callbacks: EffectData[] ): void =>
  L336: const findInsertBefore = (node: Node | undefined): SupportedElement | Text | undefined =>
  L343: const removeNode = (node: Node): void =>
  L366: const apply = (node: NodeObject, container: Container, isNew: boolean): void =>
  L371: const findChildNodeIndex = ( childNodes: NodeListOf<ChildNode>, child: ChildNode | undefined ): number | undefined =>
  L389: const applyNodeObject = (node: NodeObject, container: Container, isNew: boolean): void =>
  L485: const isSameContext = ( oldContexts: LocalJSXContexts, newContexts: NonNullable<LocalJSXContexts> ): boolean =>
  L495: const indexChildrenByKey = (children: Node[]): Map<unknown, Node> | undefined =>
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
  L25: type CacheKeyRequest = | { method: 'GET' } | { method: 'QUERY' digest: string }
  L39: const shouldSkipCacheControl = (cacheControl: string | null): boolean =>
  L42: const parseVaryDirectives = (vary: string | string[] | null | undefined): string[] =>
  L51: const createCacheKey = ( key: string, requestUrl: string, request: CacheKeyRequest, varyHeaders: [string, string][] ): string =>
  L72: const shouldSkipCache = ( res: Response, optionsVaryDirectives: Set<string> | undefined, responseVary: string[] ): boolean =>
  L82: const reportCacheNotAvailable = ( onCacheNotAvailable: ((reason: string) => void) | false | undefined, reason: string ): void =>
  L95: const createQueryDigest = async ( c: Context, maxQueryBodySize: number ): Promise<string | undefined> =>
  L183: const cache = (options: { cacheName: string | ((c: Context) => Promise<string> | string) wait?: boolean cacheControl?: string vary?: string | string[] keyGenerator?: (c: Context) => Promise<string> | string maxQueryBodySize?: number cacheableStatusCodes?: StatusCode[] onCacheNotAvailable?: ((reason: string) => void) | false }): MiddlewareHandler =>
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
  L10: const getConnInfo: GetConnInfo = (c: Context) =>
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
- sym:51103 Hono.basePath (src/hono-base.ts:250-256)
  basePath<SubPath extends string>( path: SubPath ): Hono<E, S, MergePath<BasePath, SubPath>, MergePath<BasePath, SubPath>>
- sym:51105 Hono.#addRoute (src/hono-base.ts:390-401)
  #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void
- sym:51584 PatternRouter.match (src/router/pattern-router/router.ts:38-53)
  match(method: string, path: string): Result<T>
- sym:51099 Hono (src/hono-base.ts:98-547)
  class Hono< E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', CurrentPath extends string = BasePath, >
- sym:51580 LinearRouter.match (src/router/linear-router/router.ts:25-151)
  match(method: string, path: string): Result<T>
- sym:51015 baseRoutePath (src/helper/route/index.ts:82-83)
  const baseRoutePath = (c: Context, index?: number): string =>
- sym:51471 methodNotAllowed (src/middleware/method-not-allowed/index.ts:59-141)
  const methodNotAllowed = <E extends Env = Env>( options: MethodNotAllowedOptions<E> ): MiddlewareHandler<E> =>
- sym:51102 Hono.route (src/hono-base.ts:211-235)
  route< SubPath extends string, SubEnv extends Env, SubSchema extends Schema, SubBasePath extends string, SubCurrentPath extends string, >( path: SubPath, app: Hono<SubEnv, SubSchema, SubBasePath, SubCurrentPath> ): Hono<E, MergeSchemaPath<SubSchema, MergePath<BasePath, SubPath>> | S, BasePath, CurrentPath>
- sym:51541 Hono (src/preset/quick.ts:13-24)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:51543 Hono (src/preset/tiny.ts:11-20)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:51111 Hono (src/hono.ts:16-34)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:51896 checkOptionalParameter (src/utils/url.ts:184-219)
  const checkOptionalParameter = (path: string): string[] | null =>
- sym:51635 TrieRouter.match (src/router/trie-router/router.ts:15-17)
  match(method: string, path: string): Result<T>
- sym:51589 match (src/router/reg-exp-router/matcher.ts:10-33)
  function match<R extends Router<T>, T>(this: R, method: string, path: string): Result<T>
- sym:51625 SmartRouter.match (src/router/smart-router/router.ts:21-61)
  match(method: string, path: string): Result<T>
- sym:51582 PatternRouter (src/router/pattern-router/router.ts:8-54)
  class PatternRouter<T> implements Router<T>
- sym:51578 LinearRouter (src/router/linear-router/router.ts:11-152)
  class LinearRouter<T> implements Router<T>
- sym:51641 Router (src/router.ts:29-52)
  interface Router<T>
- sym:51101 Hono.#clone (src/hono-base.ts:178-187)
  #clone(): Hono<E, S, BasePath, CurrentPath>
- sym:51095 HonoOptions (src/hono-base.ts:46-87)
  type HonoOptions<E extends Env> = { /** * `strict` option specifies whether to distinguish whether the last path is a directory or not. * * @see {@link https://hono.dev/docs/api/hono#strict-mode} * * @default true */ strict?: boolean /** * `router` option specifies which router to use. * * @see {@link https://hono.dev/docs/api/hono#router-option} * * @example * ```ts * const app = new Hono({ router: new RegExpRouter() }) * ``` */ router?: Router<[H, RouterRoute]> /** * `getPath` can handle the host header value. * * @see {@link https://hono.dev/docs/api/routing#routing-with-host-header-value} * * @example * ```ts * const app = new Hono({ * getPath: (req) => * '/' + req.headers.get('host') + req.url.replace(/^https?:\/\/[^/]+(\/[^?]*)/, '$1'), * }) * * app.get('/www1.example.com/hello', () => c.text('hello www1')) * * // A following request will match the route: * // new Request('http://www1.example.com/hello', { * // headers: { host: 'www1.example.com' }, * // }) * ``` */ getPath?: GetPath<E> }
- sym:51013 matchedRoutes (src/helper/route/index.ts:32-34)
  const matchedRoutes = (c: Context): RouterRoute[] => // @ts-expect-error c.req[GET_MATCH_RESULT] is not typed
- sym:51552 HonoRequest.constructor (src/request.ts:69-77)
  constructor( request: Request, path: string = '/', matchResult: Result<[unknown, RouterRoute]> = [[]] )
- sym:51632 Node.search (src/router/trie-router/node.ts:86-216)
  search(method: string, path: string): [[T, Params][]]
- sym:51051 ToSSGAdaptorInterface (src/helper/ssg/ssg.ts:356-362)
  interface ToSSGAdaptorInterface< E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', >
- sym:51024 ssgParams (src/helper/ssg/middleware.ts:43-49)
  const ssgParams: SSGParamsMiddleware = (params) =>
- sym:51571 HonoRequest.[GET_MATCH_RESULT] (src/request.ts:394-396)
  get [GET_MATCH_RESULT](): Result<[unknown, RouterRoute]>
- sym:51583 PatternRouter.add (src/router/pattern-router/router.ts:12-36)
  add(method: string, path: string, handler: T)
- sym:51607 buildInitParams (src/router/reg-exp-router/prepared-router.ts:95-154)
  const buildInitParams: (params: { paths: string[] }) => ConstructorParameters<typeof PreparedRegExpRouter> = ({ paths }) =>
- sym:51610 buildWildcardRegExp (src/router/reg-exp-router/router.ts:22-36)
  function buildWildcardRegExp(path: string): RegExp
- sym:50912 ContextOptions (src/context.ts:237-252)
  type ContextOptions<E extends Env> = { /** * Bindings for the environment. */ env: E['Bindings'] /** * Execution context for the request. */ executionCtx?: FetchEventLike | ExecutionContext | undefined /** * Handler for not found responses. */ notFoundHandler?: NotFoundHandler<E> matchResult?: Result<[H, RouterRoute]> path?: string }
- sym:50882 mergePath (src/client/utils.ts:11-16)
  const mergePath = (base: string, path: string) =>
- sym:50921 Context (src/context.ts:293-797)
  class Context< // eslint-disable-next-line @typescript-eslint/no-explicit-any E extends Env = any, // eslint-disable-next-line @typescript-eslint/no-explicit-any P extends string = any, I extends Input = {}, >
- sym:50923 Context.req (src/context.ts:366-369)
  get req(): HonoRequest<P, I['out']>
- sym:51169 Context (src/jsx/context.ts:8-11)
  interface Context<T> extends FC<PropsWithChildren<{ value: T }>>
- sym:51237 Context (src/jsx/dom/render.ts:91-104)
  type Context = | [ PendingType, // PendingType boolean, // got an error UpdateHook, // update hook boolean, // is in view transition boolean, // is in top level render [Context, Function, NodeObject][], // [context, error handler, node] stack for this context ] | [PendingType, boolean, UpdateHook, boolean] | [PendingType, boolean, UpdateHook] | [PendingType, boolean] | [PendingType] | []
- sym:51388 cache (src/middleware/cache/index.ts:183-324)
  const cache = (options: { cacheName: string | ((c: Context) => Promise<string> | string) wait?: boolean cacheControl?: string vary?: string | string[] keyGenerator?: (c: Context) => Promise<string> | string maxQueryBodySize?: number cacheableStatusCodes?: StatusCode[] onCacheNotAvailable?: ((reason: string) => void) | false }): MiddlewareHandler =>
- sym:51595 Context (src/router/reg-exp-router/node.ts:9-11)
  interface Context
- sym:51885 splitRoutingPath (src/utils/url.ts:16-21)
  const splitRoutingPath = (routePath: string): string[] =>
- sym:51888 getPattern (src/utils/url.ts:51-78)
  const getPattern = (label: string, next?: string): Pattern | null =>
- sym:51895 mergePath (src/utils/url.ts:171-182)
  const mergePath: (...paths: string[]) => string = ( base: string | undefined, sub: string | undefined, ...rest: string[] ): string =>
- sym:50522 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:50523 getConnInfo (adapters/aws-lambda/src/conninfo.ts:45-73)
  const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- sym:50536 streamHandle (adapters/aws-lambda/src/handler.ts:142-197)
  const streamHandle = < E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', >( app: Hono<E, S, BasePath> ): Handler =>
- sym:50538 handle (adapters/aws-lambda/src/handler.ts:243-280)
  const handle = <E extends Env = Env, S extends Schema = {}, BasePath extends string = '/'>( app: Hono<E, S, BasePath>, { isContentTypeBinary }: HandleOptions = { isContentTypeBinary: undefined } ): (<L extends LambdaEvent>( event: L, lambdaContext?: LambdaContext ) => Promise< APIGatewayProxyResult & (L extends { multiValueHeaders: Record<string, string[]> } ? WithMultiValueHeaders : WithHeaders) >) =>
- sym:50603 getConnInfo (adapters/bun/src/conninfo.ts:10-43)
  const getConnInfo: GetConnInfo = (c: Context) =>
- sym:50605 getBunServer (adapters/bun/src/server.ts:13-14)
  const getBunServer = <T>(c: Context): T | undefined =>
- sym:50613 getConnInfo (adapters/cloudflare-workers/src/conninfo.ts:3-7)
  const getConnInfo: GetConnInfo = (c) =>
- sym:50624 Env (adapters/lambda-edge/src/conninfo.ts:5-9)
  type Env = { Bindings: { event: CloudFrontEdgeEvent } }
- sym:50625 getConnInfo (adapters/lambda-edge/src/conninfo.ts:11-15)
  const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- sym:50640 handle (adapters/lambda-edge/src/handler.ts:115-147)
  const handle = ( // eslint-disable-next-line @typescript-eslint/no-explicit-any app: Hono<any> ): (( event: CloudFrontEdgeEvent, context?: CloudFrontContext, callback?: Callback ) => Promise<CloudFrontResult | CloudFrontRequest>) =>
- sym:50648 Env (adapters/netlify/src/conninfo.ts:28-32)
  type Env = { Bindings: { context: NetlifyContext } }
- sym:50649 getConnInfo (adapters/netlify/src/conninfo.ts:53-57)
  const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- sym:50650 handle (adapters/netlify/src/handler.ts:4-10)
  const handle = ( app: Hono<any, any> ): ((req: Request, context: any) => Response | Promise<Response>) =>
- sym:50653 handle (adapters/service-worker/src/handler.ts:18-37)
  const handle = <E extends Env, S extends Schema, BasePath extends string>( app: Hono<E, S, BasePath>, opts: HandleOptions = { // To use `fetch` on a Service Worker correctly, bind it to `globalThis`. fetch: globalThis.fetch.bind(globalThis), } ): Handler =>
- sym:51576 runTest (src/router/common.case.test.ts:14-1118)
  const runTest = ({ skip = [], newRouter, }: { skip?: { reason: string tests: string[] }[] newRouter: <T>() => Router<T> }) =>
- sym:51375 MockCache.match (src/middleware/cache/index.test.ts:18-20)
  async match(key: Request | string): Promise<Response | null>
- sym:51378 Context (src/middleware/cache/index.test.ts:39-48)
  class Context implements ExecutionContext

---
tokens: 7181/8000
