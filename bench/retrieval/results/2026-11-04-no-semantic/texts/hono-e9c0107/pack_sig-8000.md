## Task
# hono-e9c0107 · hono-e9c0107-2 · open
Goal: fix(hono-base): disallow `getPath` when `strict` is `false` (#5563)
Workspace: /root/bench/ws/rebf815-hono-e9c0107/hono-e9c0107-2
Changed (0): nothing yet

## Map
- adapters/aws-lambda/src/handler.ts
  L14: function sanitizeHeaderValue(value: string): string
  L24: type LambdaEvent = | APIGatewayProxyEvent | APIGatewayProxyEventV2 | ALBProxyEvent | LatticeProxyEventV2
  L30: interface LatticeProxyEventV2
  L42: interface APIGatewayProxyEventV2
  L65: interface APIGatewayProxyEvent
  L86: interface ALBProxyEvent
  L100: type WithHeaders = { headers: Record<string, string | string[]> multiValueHeaders?: undefined }
  L104: type WithMultiValueHeaders = { headers?: undefined multiValueHeaders: Record<string, string[]> }
  L109: type APIGatewayProxyResult = { statusCode: number statusDescription?: string body: string cookies?: string[] isBase64Encoded: boolean } & (WithHeaders | WithMultiValueHeaders)
  L117: const getRequestContext = ( event: LambdaEvent ): | ApiGatewayRequestContext | ApiGatewayRequestContextV2 | ALBRequestContext | LatticeRequestContextV2 =>
  L127: async function* readWebStream( reader: ReadableStreamDefaultReader<Uint8Array> ): AsyncGenerator<Uint8Array>
  L137: const streamToNodeStream = ( reader: ReadableStreamDefaultReader<Uint8Array>, writer: NodeJS.WritableStream ): Promise<void> =>
  L142: const streamHandle = < E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', >( app: Hono<E, S, BasePath> ): Handler =>
  L199: type HandleOptions = { isContentTypeBinary: ((contentType: string) => boolean) | undefined }
  L243: const handle = <E extends Env = Env, S extends Schema = {}, BasePath extends string = '/'>( app: Hono<E, S, BasePath>, { isContentTypeBinary }: HandleOptions = { isContentTypeBinary: undefined } ): (<L extends LambdaEvent>( event: L, lambdaContext?: LambdaContext ) => Promise< APIGatewayProxyResult & (L extends { multiValueHeaders: Record<string, string[]> } ? WithMultiValueHeaders : WithHeaders) >) =>
  L282: abstract class EventProcessor<E extends LambdaEvent>
  L295: protected getHeaderValue(headers: E['headers'], key: string): string | undefined
  L305: protected getDomainName(event: E): string | undefined
  L322: createRequest(event: E): Request
  L348: async createResult( event: E, res: Response, options: Pick<HandleOptions, 'isContentTypeBinary'> ): Promise<APIGatewayProxyResult>
  L392: setCookies(_event: E, res: Response, result: APIGatewayProxyResult)
  L408: class EventV2Processor extends EventProcessor<APIGatewayProxyEventV2>
  L409: protected getPath(event: APIGatewayProxyEventV2): string
  L413: protected getMethod(event: APIGatewayProxyEventV2): string
  L417: protected getQueryString(event: APIGatewayProxyEventV2): string
  L421: protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void
  L427: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void
  L431: protected getHeaders(event: APIGatewayProxyEventV2): Headers
  L447: class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
  L448: protected getPath(event: APIGatewayProxyEvent): string
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
- src/utils/url.ts
  L6: type Pattern = readonly [string, string, RegExp | true] | '*'
  L8: const splitPath = (path: string): string[] =>
  L16: const splitRoutingPath = (routePath: string): string[] =>
  L23: const extractGroupsFromPath = (path: string): { groups: [string, string][]; path: string } =>
  L35: const replaceGroupMarks = (paths: string[], groups: [string, string][]): string[] =>
  L51: const getPattern = (label: string, next?: string): Pattern | null =>
  L80: type Decoder = (str: string) => string
  L81: const tryDecode = (str: string, decoder: Decoder): string =>
  L104: const tryDecodeURI = (str: string): string =>
  L106: const getPath = (request: Request): string =>
  L140: const getQueryStrings = (url: string): string =>
  L154: const getPathNoStrict = (request: Request): string =>
  L171: const mergePath: (...paths: string[]) => string = ( base: string | undefined, sub: string | undefined, ...rest: string[] ): string =>
  L184: const checkOptionalParameter = (path: string): string[] | null =>
  L221: const tryDecodeURIComponent = (str: string): string =>
  L225: const _decodeURI = (value: string): string =>
  L232: const _getQueryParam = ( url: string, key?: string, multiple?: boolean ): string | undefined | Record<string, string> | string[] | Record<string, string[]> =>
  L328: const getQueryParams = ( url: string, key?: string ): string[] | undefined | Record<string, string[]> =>
- adapters/aws-lambda/src/conninfo.ts
  L9: type LambdaRequestContext = | ApiGatewayRequestContext | ApiGatewayRequestContextV2 | ALBRequestContext
  L14: type Env = { Bindings: { requestContext: LambdaRequestContext } }
  L45: const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- adapters/lambda-edge/src/conninfo.ts
  L5: type Env = { Bindings: { event: CloudFrontEdgeEvent } }
  L11: const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- adapters/netlify/src/conninfo.ts
  L8: type NetlifyContext = { ip?: string geo?: { city?: string country?: { code?: string name?: string } subdivision?: { code?: string name?: string } latitude?: number longitude?: number timezone?: string postalCode?: string } requestId?: string }
  L28: type Env = { Bindings: { context: NetlifyContext } }
  L53: const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- src/adapter/aws-lambda/conninfo.ts
  L9: type LambdaRequestContext = | ApiGatewayRequestContext | ApiGatewayRequestContextV2 | ALBRequestContext
  L14: type Env = { Bindings: { requestContext: LambdaRequestContext } }
  L46: const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- src/adapter/aws-lambda/handler.ts
  L14: function sanitizeHeaderValue(value: string): string
  L27: type LambdaEvent = | APIGatewayProxyEvent | APIGatewayProxyEventV2 | ALBProxyEvent | LatticeProxyEventV2
  L36: interface LatticeProxyEventV2
  L51: interface APIGatewayProxyEventV2
  L77: interface APIGatewayProxyEvent
  L101: interface ALBProxyEvent
  L115: type WithHeaders = { headers: Record<string, string | string[]> multiValueHeaders?: undefined }
  L119: type WithMultiValueHeaders = { headers?: undefined multiValueHeaders: Record<string, string[]> }
  L127: type APIGatewayProxyResult = { statusCode: number statusDescription?: string body: string cookies?: string[] isBase64Encoded: boolean } & (WithHeaders | WithMultiValueHeaders)
  L135: const getRequestContext = ( event: LambdaEvent ): | ApiGatewayRequestContext | ApiGatewayRequestContextV2 | ALBRequestContext | LatticeRequestContextV2 =>
  L145: async function* readWebStream( reader: ReadableStreamDefaultReader<Uint8Array> ): AsyncGenerator<Uint8Array>
  L155: const streamToNodeStream = ( reader: ReadableStreamDefaultReader<Uint8Array>, writer: NodeJS.WritableStream ): Promise<void> =>
  L163: const streamHandle = < E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', >( app: Hono<E, S, BasePath> ): Handler =>
  L220: type HandleOptions = { isContentTypeBinary: ((contentType: string) => boolean) | undefined }
  L265: const handle = <E extends Env = Env, S extends Schema = {}, BasePath extends string = '/'>( app: Hono<E, S, BasePath>, { isContentTypeBinary }: HandleOptions = { isContentTypeBinary: undefined } ): (<L extends LambdaEvent>( event: L, lambdaContext?: LambdaContext ) => Promise< APIGatewayProxyResult & (L extends { multiValueHeaders: Record<string, string[]> } ? WithMultiValueHeaders : WithHeaders) >) =>
  L307: abstract class EventProcessor<E extends LambdaEvent>
  L320: protected getHeaderValue(headers: E['headers'], key: string): string | undefined
  L330: protected getDomainName(event: E): string | undefined
  L347: createRequest(event: E): Request
  L373: async createResult( event: E, res: Response, options: Pick<HandleOptions, 'isContentTypeBinary'> ): Promise<APIGatewayProxyResult>
  L417: setCookies(_event: E, res: Response, result: APIGatewayProxyResult)
  L436: class EventV2Processor extends EventProcessor<APIGatewayProxyEventV2>
  L437: protected getPath(event: APIGatewayProxyEventV2): string
  L441: protected getMethod(event: APIGatewayProxyEventV2): string
  L445: protected getQueryString(event: APIGatewayProxyEventV2): string
  L449: protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void
  L455: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void
  L459: protected getHeaders(event: APIGatewayProxyEventV2): Headers
  L478: class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
  L479: protected getPath(event: APIGatewayProxyEvent): string
- src/adapter/cloudflare-pages/handler.test.ts
  L7: type Env = { Bindings: { TOKEN: string } }
  L13: function createEventContext( context: Partial<EventContext<Env['Bindings']>> ): EventContext<Env['Bindings']>
- src/adapter/lambda-edge/conninfo.ts
  L5: type Env = { Bindings: { event: CloudFrontEdgeEvent } }
  L14: const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- src/adapter/netlify/conninfo.ts
  L8: type NetlifyContext = { ip?: string geo?: { city?: string country?: { code?: string name?: string } subdivision?: { code?: string name?: string } latitude?: number longitude?: number timezone?: string postalCode?: string } requestId?: string }
  L28: type Env = { Bindings: { context: NetlifyContext } }
  L54: const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- src/context.ts
- src/helper/adapter/index.ts
  L8: type Runtime = 'node' | 'deno' | 'bun' | 'workerd' | 'fastly' | 'edge-light' | 'other'
  L10: const env = < T extends Record<string, unknown>, C extends Context = Context<{ Bindings: T }>, >( c: T extends Record<string, unknown> ? Context : C, runtime?: Runtime ): T & C['env'] =>
  L50: const getRuntimeKey = (): Runtime =>
  L86: const checkUserAgentEquals = (platform: string): boolean =>
- src/hono.test.ts
  L18: function throwExpression(errorMessage: string): never
  L22: type Env = { Bindings: { _: string } }
  L28: const createResponseProxy = (response: Response) =>
- src/request.ts
- repo map (583 files):
  .editorconfig
  .github/ (12 files)
  .gitignore
  .tool-versions
  .vscode/ (2 files)
  LICENSE
  README.md
  adapters/ (113 files)
  benchmarks/ (72 files)
  build/ (8 files)
  bunfig.toml
  codecov.yml
  docs/ (10 files)
  jsr.json
  package.cjs.json
  package.json
  perf-measures/ (12 files)
  pnpm-lock.yaml
  pnpm-workspace.yaml
  runtime-tests/ (22 files)
  src/ (314 files)
  tsconfig.base.json
  tsconfig.build.json
  tsconfig.json
  tsconfig.spec.json
  vite.config.ts
  vitest.config.ts

## Code
- sym:48306 GetPath (src/hono-base.ts:44-44)
  type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string
- sym:47765 EventV1Processor.getPath (adapters/aws-lambda/src/handler.ts:448-450)
  protected getPath(event: APIGatewayProxyEvent): string
- sym:47758 EventV2Processor.getPath (adapters/aws-lambda/src/handler.ts:409-411)
  protected getPath(event: APIGatewayProxyEventV2): string
- sym:48307 HonoOptions (src/hono-base.ts:46-87)
  type HonoOptions<E extends Env> = { /** * `strict` option specifies whether to distinguish whether the last path is a directory or not. * * @see {@link https://hono.dev/docs/api/hono#strict-mode} * * @default true */ strict?: boolean /** * `router` option specifies which router to use. * * @see {@link https://hono.dev/docs/api/hono#router-option} * * @example * ```ts * const app = new Hono({ router: new RegExpRouter() }) * ``` */ router?: Router<[H, RouterRoute]> /** * `getPath` can handle the host header value. * * @see {@link https://hono.dev/docs/api/routing#routing-with-host-header-value} * * @example * ```ts * const app = new Hono({ * getPath: (req) => * '/' + req.headers.get('host') + req.url.replace(/^https?:\/\/[^/]+(\/[^?]*)/, '$1'), * }) * * app.get('/www1.example.com/hello', () => c.text('hello www1')) * * // A following request will match the route: * // new Request('http://www1.example.com/hello', { * // headers: { host: 'www1.example.com' }, * // }) * ``` */ getPath?: GetPath<E> }
- sym:48312 Hono.constructor (src/hono-base.ts:127-176)
  constructor(options: HonoOptions<E> = {})
- sym:48313 Hono.#clone (src/hono-base.ts:178-187)
  #clone(): Hono<E, S, BasePath, CurrentPath>
- sym:49106 getPathNoStrict (src/utils/url.ts:154-159)
  const getPathNoStrict = (request: Request): string =>
- sym:48316 Hono.mount (src/hono-base.ts:333-388)
  mount( path: string, applicationHandler: (request: Request, ...args: any) => Response | Promise<Response>, options?: MountOptions ): Hono<E, S, BasePath, CurrentPath>
- sym:48319 Hono.#dispatch (src/hono-base.ts:410-470)
  #dispatch( request: Request, executionCtx: ExecutionContext | FetchEventLike | undefined, env: E['Bindings'], method: string ): Response | Promise<Response>
- sym:49104 getPath (src/utils/url.ts:106-134)
  const getPath = (request: Request): string =>
- sym:48311 Hono (src/hono-base.ts:98-547)
  class Hono< E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', CurrentPath extends string = BasePath, >
- sym:48315 Hono.basePath (src/hono-base.ts:250-256)
  basePath<SubPath extends string>( path: SubPath ): Hono<E, S, MergePath<BasePath, SubPath>, MergePath<BasePath, SubPath>>
- sym:48317 Hono.#addRoute (src/hono-base.ts:390-401)
  #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void
- sym:48314 Hono.route (src/hono-base.ts:211-235)
  route< SubPath extends string, SubEnv extends Env, SubSchema extends Schema, SubBasePath extends string, SubCurrentPath extends string, >( path: SubPath, app: Hono<SubEnv, SubSchema, SubBasePath, SubCurrentPath> ): Hono<E, MergeSchemaPath<SubSchema, MergePath<BasePath, SubPath>> | S, BasePath, CurrentPath>
- sym:48755 Hono (src/preset/tiny.ts:11-20)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:48753 Hono (src/preset/quick.ts:13-24)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:48323 Hono (src/hono.ts:16-34)
  class Hono< E extends Env = BlankEnv, S extends Schema = BlankSchema, BasePath extends string = '/', > extends HonoBase<E, S, BasePath>
- sym:48912 CookieOptions (src/utils/cookie.ts:17-28)
  type CookieOptions = { domain?: string expires?: Date httpOnly?: boolean maxAge?: number path?: string secure?: boolean sameSite?: 'Strict' | 'Lax' | 'None' | 'strict' | 'lax' | 'none' partitioned?: boolean priority?: 'Low' | 'Medium' | 'High' | 'low' | 'medium' | 'high' prefix?: CookiePrefixOptions } & PartitionedCookieConstraint
- sym:47994 handle (src/adapter/cloudflare-pages/handler.ts:38-52)
  const handle = <E extends Env = Env, S extends Schema = BlankSchema, BasePath extends string = '/'>( app: Hono<E, S, BasePath> ): PagesFunction<E['Bindings']> =>
- sym:48683 methodNotAllowed (src/middleware/method-not-allowed/index.ts:59-141)
  const methodNotAllowed = <E extends Env = Env>( options: MethodNotAllowedOptions<E> ): MiddlewareHandler<E> =>
- sym:48263 ToSSGAdaptorInterface (src/helper/ssg/ssg.ts:356-362)
  interface ToSSGAdaptorInterface< E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', >
- sym:47866 fire (adapters/service-worker/src/index.ts:28-34)
  const fire = <E extends Env, S extends Schema, BasePath extends string>( app: Hono<E, S, BasePath>, options?: HandleOptions ): void =>
- sym:48035 fire (src/adapter/service-worker/index.ts:32-38)
  const fire = <E extends Env, S extends Schema, BasePath extends string>( app: Hono<E, S, BasePath>, options?: HandleOptions ): void =>
- sym:47773 ALBProcessor.getPath (adapters/aws-lambda/src/handler.ts:530-532)
  protected getPath(event: ALBProxyEvent): string
- sym:47946 ALBProcessor.getPath (src/adapter/aws-lambda/handler.ts:564-566)
  protected getPath(event: ALBProxyEvent): string
- sym:47865 handle (adapters/service-worker/src/handler.ts:18-37)
  const handle = <E extends Env, S extends Schema, BasePath extends string>( app: Hono<E, S, BasePath>, opts: HandleOptions = { // To use `fetch` on a Service Worker correctly, bind it to `globalThis`. fetch: globalThis.fetch.bind(globalThis), } ): Handler =>
- sym:48034 handle (src/adapter/service-worker/handler.ts:22-41)
  const handle = <E extends Env, S extends Schema, BasePath extends string>( app: Hono<E, S, BasePath>, opts: HandleOptions = { // To use `fetch` on a Service Worker correctly, bind it to `globalThis`. fetch: globalThis.fetch.bind(globalThis), } ): Handler =>
- sym:48082 Client (src/client/types.ts:311-318)
  type Client<T, Prefix extends string> = T extends HonoBase<any, infer S, any> ? S extends Record<infer K, Schema> ? K extends string ? PathToChain<Prefix, K, S> : never : never : never
- sym:47779 LatticeV2Processor.getPath (adapters/aws-lambda/src/handler.ts:589-591)
  protected getPath(event: LatticeProxyEventV2): string
- sym:47938 EventV1Processor.getPath (src/adapter/aws-lambda/handler.ts:479-481)
  protected getPath(event: APIGatewayProxyEvent): string
- sym:47952 LatticeV2Processor.getPath (src/adapter/aws-lambda/handler.ts:626-628)
  protected getPath(event: LatticeProxyEventV2): string
- sym:47931 EventV2Processor.getPath (src/adapter/aws-lambda/handler.ts:437-439)
  protected getPath(event: APIGatewayProxyEventV2): string
- sym:48047 hc (src/client/client.ts:151-249)
  const hc = <T extends Hono<any, any, any>, Prefix extends string = string>( baseUrl: Prefix, options?: ClientRequestOptions ) =>
- sym:47734 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:47739 APIGatewayProxyEventV2 (adapters/aws-lambda/src/handler.ts:42-62)
  interface APIGatewayProxyEventV2
- sym:47740 APIGatewayProxyEvent (adapters/aws-lambda/src/handler.ts:65-83)
  interface APIGatewayProxyEvent
- sym:47754 EventProcessor.createRequest (adapters/aws-lambda/src/handler.ts:322-346)
  createRequest(event: E): Request
- sym:47836 Env (adapters/lambda-edge/src/conninfo.ts:5-9)
  type Env = { Bindings: { event: CloudFrontEdgeEvent } }
- sym:47860 Env (adapters/netlify/src/conninfo.ts:28-32)
  type Env = { Bindings: { context: NetlifyContext } }
- sym:47907 Env (src/adapter/aws-lambda/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:47912 APIGatewayProxyEventV2 (src/adapter/aws-lambda/handler.ts:51-71)
  interface APIGatewayProxyEventV2
- sym:47913 APIGatewayProxyEvent (src/adapter/aws-lambda/handler.ts:77-95)
  interface APIGatewayProxyEvent
- sym:47927 EventProcessor.createRequest (src/adapter/aws-lambda/handler.ts:347-371)
  createRequest(event: E): Request
- sym:48006 Env (src/adapter/lambda-edge/conninfo.ts:5-9)
  type Env = { Bindings: { event: CloudFrontEdgeEvent } }
- sym:48029 Env (src/adapter/netlify/conninfo.ts:28-32)
  type Env = { Bindings: { context: NetlifyContext } }
- sym:48136 Context.event (src/context.ts:377-383)
  get event(): FetchEventLike
- sym:48150 env (src/helper/adapter/index.ts:10-41)
  const env = < T extends Record<string, unknown>, C extends Context = Context<{ Bindings: T }>, >( c: T extends Record<string, unknown> ? Context : C, runtime?: Runtime ): T & C['env'] =>
- sym:48781 HonoRequest.url (src/request.ts:374-376)
  get url(): string
- sym:49103 tryDecodeURI (src/utils/url.ts:104-104)
  const tryDecodeURI = (str: string): string =>
- sym:47733 LambdaRequestContext (adapters/aws-lambda/src/conninfo.ts:9-12)
  type LambdaRequestContext = | ApiGatewayRequestContext | ApiGatewayRequestContextV2 | ALBRequestContext
- sym:47735 getConnInfo (adapters/aws-lambda/src/conninfo.ts:45-73)
  const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- sym:47737 LambdaEvent (adapters/aws-lambda/src/handler.ts:24-28)
  type LambdaEvent = | APIGatewayProxyEvent | APIGatewayProxyEventV2 | ALBProxyEvent | LatticeProxyEventV2
- sym:47745 getRequestContext (adapters/aws-lambda/src/handler.ts:117-125)
  const getRequestContext = ( event: LambdaEvent ): | ApiGatewayRequestContext | ApiGatewayRequestContextV2 | ALBRequestContext | LatticeRequestContextV2 =>
- sym:47748 streamHandle (adapters/aws-lambda/src/handler.ts:142-197)
  const streamHandle = < E extends Env = Env, S extends Schema = {}, BasePath extends string = '/', >( app: Hono<E, S, BasePath> ): Handler =>
- sym:47750 handle (adapters/aws-lambda/src/handler.ts:243-280)
  const handle = <E extends Env = Env, S extends Schema = {}, BasePath extends string = '/'>( app: Hono<E, S, BasePath>, { isContentTypeBinary }: HandleOptions = { isContentTypeBinary: undefined } ): (<L extends LambdaEvent>( event: L, lambdaContext?: LambdaContext ) => Promise< APIGatewayProxyResult & (L extends { multiValueHeaders: Record<string, string[]> } ? WithMultiValueHeaders : WithHeaders) >) =>
- sym:47753 EventProcessor.getDomainName (adapters/aws-lambda/src/handler.ts:305-320)
  protected getDomainName(event: E): string | undefined
- sym:47755 EventProcessor.createResult (adapters/aws-lambda/src/handler.ts:348-390)
  async createResult( event: E, res: Response, options: Pick<HandleOptions, 'isContentTypeBinary'> ): Promise<APIGatewayProxyResult>
- sym:47757 EventV2Processor (adapters/aws-lambda/src/handler.ts:408-443)
  class EventV2Processor extends EventProcessor<APIGatewayProxyEventV2>
- sym:47759 EventV2Processor.getMethod (adapters/aws-lambda/src/handler.ts:413-415)
  protected getMethod(event: APIGatewayProxyEventV2): string
- sym:47760 EventV2Processor.getQueryString (adapters/aws-lambda/src/handler.ts:417-419)
  protected getQueryString(event: APIGatewayProxyEventV2): string
- sym:47761 EventV2Processor.getCookies (adapters/aws-lambda/src/handler.ts:421-425)
  protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void
- sym:47763 EventV2Processor.getHeaders (adapters/aws-lambda/src/handler.ts:431-442)
  protected getHeaders(event: APIGatewayProxyEventV2): Headers
- sym:47764 EventV1Processor (adapters/aws-lambda/src/handler.ts:447-503)
  class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
- sym:47766 EventV1Processor.getMethod (adapters/aws-lambda/src/handler.ts:452-454)
  protected getMethod(event: APIGatewayProxyEvent): string
- sym:47767 EventV1Processor.getQueryString (adapters/aws-lambda/src/handler.ts:456-472)
  protected getQueryString(event: APIGatewayProxyEvent): string
- sym:47990 Env (src/adapter/cloudflare-pages/handler.test.ts:7-11)
  type Env = { Bindings: { TOKEN: string } }
- sym:48321 Env (src/hono.test.ts:22-26)
  type Env = { Bindings: { _: string } }

---
tokens: 7061/8000
