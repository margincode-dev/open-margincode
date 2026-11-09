## Task
# hono-06a45bc · hono-06a45bc · open
Goal: feat(bearer-auth): allow MessageFunction to return a Response (#5269)

* feat(bearer-auth): allow MessageFunction to return a Response

Lets callers use RFC 9457 application/problem+json (or any content type) for auth errors without a breaking change. WWW-Authenticate is set if the Response omitted it.

Fixes #5218

* fix(bearer-auth): use the status of the returned Response as is

* test(bearer-auth): cover a Response with its own status and WWW-Authenticate

---------
Workspace: /root/bench/ws/rebf815-hono-06a45bc/hono-06a45bc
Changed (0): nothing yet

## Map
- src/middleware/basic-auth/index.ts
  L12: type MessageFunction = (c: Context) => string | object | Promise<string | object>
  L14: type BasicAuthOptions = | { username: string password: string realm?: string hashFunction?: Function invalidUserMessage?: string | object | MessageFunction onAuthSuccess?: (c: Context, username: string) => void | Promise<void> } | { verifyUser: (username: string, password: string, c: Context) => boolean | Promise<boolean> realm?: string hashFunction?: Function invalidUserMessage?: string | object | MessageFunction onAuthSuccess?: (c: Context, username: string) => void | Promise<void> }
  L80: const basicAuth = ( options: BasicAuthOptions, ...users: { username: string; password: string }[] ): MiddlewareHandler =>
- src/middleware/bearer-auth/index.ts
- src/context.ts
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
- src/router/reg-exp-router/node.ts
  L3: const LABEL_REG_EXP_STR = '[^/]+'
  L4: const ONLY_WILDCARD_REG_EXP_STR = '.*'
  L5: const TAIL_WILDCARD_REG_EXP_STR = '(?:|/.*)'
  L6: const PATH_ERROR = Symbol()
  L8: type ParamAssocArray = [string, number][]
  L9: interface Context
  L22: function compareKey(a: string, b: string): number
  L47: class Node
  L53: insert( tokens: readonly string[], index: number, paramMap: ParamAssocArray, context: Context, isStatic: boolean ): void
  L139: buildRegExpStr(): string
- adapters/aws-lambda/src/conninfo.ts
  L9: type LambdaRequestContext = | ApiGatewayRequestContext | ApiGatewayRequestContextV2 | ALBRequestContext
  L14: type Env = { Bindings: { requestContext: LambdaRequestContext } }
  L45: const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- adapters/bun/src/conninfo.ts
  L10: const getConnInfo: GetConnInfo = (c: Context) =>
- adapters/bun/src/server.ts
  L13: const getBunServer = <T>(c: Context): T | undefined =>
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
- src/adapter/bun/conninfo.ts
  L11: const getConnInfo: GetConnInfo = (c: Context) =>
- src/adapter/bun/server.ts
  L14: const getBunServer = <T>(c: Context): T | undefined =>
- src/adapter/cloudflare-pages/handler.test.ts
- src/adapter/cloudflare-pages/handler.ts
- src/adapter/lambda-edge/conninfo.ts
- src/adapter/netlify/conninfo.ts
- src/client/types.ts
- src/compose.ts
- src/helper/accepts/accepts.ts
- src/helper/adapter/index.ts
- src/helper/conninfo/types.ts
- src/helper/cookie/index.ts
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:45787 MessageFunction (src/middleware/bearer-auth/index.ts:16-16)
  type MessageFunction = (c: Context) => string | object | Promise<string | object>
- sym:45781 MessageFunction (src/middleware/basic-auth/index.ts:12-12)
  type MessageFunction = (c: Context) => string | object | Promise<string | object>
- sym:45788 CustomizedErrorResponseOptions (src/middleware/bearer-auth/index.ts:17-20)
  type CustomizedErrorResponseOptions = { wwwAuthenticateHeader?: string | object | MessageFunction message?: string | object | MessageFunction }
- sym:45790 bearerAuth (src/middleware/bearer-auth/index.ts:104-221)
  const bearerAuth = <E extends Env = Env>( options: BearerAuthOptions<E> ): MiddlewareHandler<E> =>
- sym:45789 BearerAuthOptions (src/middleware/bearer-auth/index.ts:22-66)
  type BearerAuthOptions<E extends Env = Env> = | { token: string | string[] realm?: string prefix?: string headerName?: string hashFunction?: Function /** * @deprecated Use noAuthenticationHeader.message instead */ noAuthenticationHeaderMessage?: string | object | MessageFunction noAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidAuthenticationHeader.message instead */ invalidAuthenticationHeaderMessage?: string | object | MessageFunction invalidAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidToken.message instead */ invalidTokenMessage?: string | object | MessageFunction invalidToken?: CustomizedErrorResponseOptions } | { realm?: string prefix?: string headerName?: string verifyToken: (token: string, c: Context<E>) => boolean | Promise<boolean> hashFunction?: Function /** * @deprecated Use noAuthenticationHeader.message instead */ noAuthenticationHeaderMessage?: string | object | MessageFunction noAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidAuthenticationHeader.message instead */ invalidAuthenticationHeaderMessage?: string | object | MessageFunction invalidAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidToken.message instead */ invalidTokenMessage?: string | object | MessageFunction invalidToken?: CustomizedErrorResponseOptions }
- sym:45786 HEADER (src/middleware/bearer-auth/index.ts:14-14)
  const HEADER = 'Authorization'
- sym:45785 PREFIX (src/middleware/bearer-auth/index.ts:13-13)
  const PREFIX = 'Bearer'
- sym:45784 TOKEN_STRINGS (src/middleware/bearer-auth/index.ts:12-12)
  const TOKEN_STRINGS = '[A-Za-z0-9._~+/-]+=*'
- sym:45782 BasicAuthOptions (src/middleware/basic-auth/index.ts:14-29)
  type BasicAuthOptions = | { username: string password: string realm?: string hashFunction?: Function invalidUserMessage?: string | object | MessageFunction onAuthSuccess?: (c: Context, username: string) => void | Promise<void> } | { verifyUser: (username: string, password: string, c: Context) => boolean | Promise<boolean> realm?: string hashFunction?: Function invalidUserMessage?: string | object | MessageFunction onAuthSuccess?: (c: Context, username: string) => void | Promise<void> }
- sym:45868 unauthorizedResponse (src/middleware/jwt/jwt.ts:165-185)
  function unauthorizedResponse(opts: { ctx: Context error: string errDescription: string statusText?: string realm?: string })
- sym:45865 unauthorizedResponse (src/middleware/jwk/jwk.ts:176-196)
  function unauthorizedResponse(opts: { ctx: Context error: string errDescription: string statusText?: string realm?: string })
- sym:45263 detectResponseType (src/client/fetch-result-please.ts:80-95)
  function detectResponseType(response: Response): 'json' | 'text'
- sym:45331 BodyRespond (src/context.ts:121-141)
  interface BodyRespond
- sym:45864 jwk (src/middleware/jwk/jwk.ts:49-174)
  const jwk = ( options: { keys?: HonoJsonWebKey[] | ((ctx: Context) => Promise<HonoJsonWebKey[]> | HonoJsonWebKey[]) jwks_uri?: string | ((ctx: Context) => Promise<string> | string) allow_anon?: boolean cookie?: | string | { key: string; secret?: string | BufferSource; prefixOptions?: CookiePrefixOptions } headerName?: string alg: AsymmetricAlgorithm[] realm?: string verification?: VerifyOptions }, init?: RequestInit ): MiddlewareHandler =>
- sym:45463 parseResponseContent (src/helper/ssg/ssg.ts:74-88)
  const parseResponseContent = async (response: Response): Promise<string | ArrayBuffer> =>
- sym:45333 JSONRespond (src/context.ts:181-189)
  interface JSONRespond
- sym:46114 COMPRESSIBLE_CONTENT_TYPE_REGEX (src/utils/compress.ts:9-10)
  const COMPRESSIBLE_CONTENT_TYPE_REGEX =
- sym:45334 JSONRespondReturn (src/context.ts:205-208)
  type JSONRespondReturn< T extends JSONValue | {} | InvalidJSONValue, U extends ContentfulStatusCode, > = Response & TypedResponse<JSONParsed<T>, U, 'json'>
- sym:46155 ResponseHeader (src/utils/headers.ts:266-343)
- sym:45330 NewResponse (src/context.ts:113-116)
  interface NewResponse
- sym:46154 RequestHeader (src/utils/headers.ts:8-264)
- sym:45466 AfterResponseHook (src/helper/ssg/ssg.ts:111-111)
  type AfterResponseHook = (res: Response) => Response | false | Promise<Response | false>
- sym:45069 isContentTypeBinary (adapters/lambda-edge/src/handler.ts:231-239)
  const isContentTypeBinary = (contentType: string): boolean =>
- sym:45345 Context (src/context.ts:293-797)
  class Context< // eslint-disable-next-line @typescript-eslint/no-explicit-any E extends Env = any, // eslint-disable-next-line @typescript-eslint/no-explicit-any P extends string = any, I extends Input = {}, >
- sym:45593 Context (src/jsx/context.ts:8-11)
  interface Context<T> extends FC<PropsWithChildren<{ value: T }>>
- sym:45661 Context (src/jsx/dom/render.ts:91-104)
  type Context = | [ PendingType, // PendingType boolean, // got an error UpdateHook, // update hook boolean, // is in view transition boolean, // is in top level render [Context, Function, NodeObject][], // [context, error handler, node] stack for this context ] | [PendingType, boolean, UpdateHook, boolean] | [PendingType, boolean, UpdateHook] | [PendingType, boolean] | [PendingType] | []
- sym:46019 Context (src/router/reg-exp-router/node.ts:9-11)
  interface Context
- sym:44946 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:44947 getConnInfo (adapters/aws-lambda/src/conninfo.ts:45-73)
  const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- sym:45027 getConnInfo (adapters/bun/src/conninfo.ts:10-43)
  const getConnInfo: GetConnInfo = (c: Context) =>
- sym:45029 getBunServer (adapters/bun/src/server.ts:13-14)
  const getBunServer = <T>(c: Context): T | undefined =>
- sym:45048 Env (adapters/lambda-edge/src/conninfo.ts:5-9)
  type Env = { Bindings: { event: CloudFrontEdgeEvent } }
- sym:45049 getConnInfo (adapters/lambda-edge/src/conninfo.ts:11-15)
  const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- sym:45072 Env (adapters/netlify/src/conninfo.ts:28-32)
  type Env = { Bindings: { context: NetlifyContext } }
- sym:45073 getConnInfo (adapters/netlify/src/conninfo.ts:53-57)
  const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- sym:45119 Env (src/adapter/aws-lambda/conninfo.ts:14-18)
  type Env = { Bindings: { requestContext: LambdaRequestContext } }
- sym:45120 getConnInfo (src/adapter/aws-lambda/conninfo.ts:46-74)
  const getConnInfo: GetConnInfo = (c: Context<Env>) =>
- sym:45191 getConnInfo (src/adapter/bun/conninfo.ts:11-44)
- sym:45193 getBunServer (src/adapter/bun/server.ts:14-15)
- sym:45207 handleMiddleware (src/adapter/cloudflare-pages/handler.ts:58-111)
- sym:45218 Env (src/adapter/lambda-edge/conninfo.ts:5-9)
- sym:45219 getConnInfo (src/adapter/lambda-edge/conninfo.ts:14-18)
- sym:45241 Env (src/adapter/netlify/conninfo.ts:28-32)
- sym:45242 getConnInfo (src/adapter/netlify/conninfo.ts:54-58)
- sym:45285 Response (src/client/types.ts:232-232)
  interface Response extends ClientResponse<unknown>
- sym:45317 compose (src/compose.ts:15-73)
- sym:45321 ExecutionContext (src/context.ts:31-52)
  interface ExecutionContext
- sym:45347 Context.req (src/context.ts:366-369)
- sym:45350 Context.res (src/context.ts:403-407)
- sym:45360 accepts (src/helper/accepts/accepts.ts:87-96)
- sym:45362 env (src/helper/adapter/index.ts:10-41)
- sym:45368 GetConnInfo (src/helper/conninfo/types.ts:45-45)
- sym:45369 GetCookie (src/helper/cookie/index.ts:10-14)
- sym:45370 GetSignedCookie (src/helper/cookie/index.ts:16-25)
- sym:45374 setCookie (src/helper/cookie/index.ts:99-102)
- sym:45376 setSignedCookie (src/helper/cookie/index.ts:130-139)
- sym:45377 deleteCookie (src/helper/cookie/index.ts:141-145)
- sym:45534 createResponseProxy (src/hono.test.ts:28-47)
- sym:45802 Context (src/middleware/cache/index.test.ts:39-48)
- sym:45202 Env (src/adapter/cloudflare-pages/handler.test.ts:7-11)

---
tokens: 3584/4000
