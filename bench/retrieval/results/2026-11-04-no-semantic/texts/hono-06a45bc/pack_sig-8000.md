## Task
# hono-06a45bc · hono-06a45bc-2 · open
Goal: feat(bearer-auth): allow MessageFunction to return a Response (#5269)

* feat(bearer-auth): allow MessageFunction to return a Response

Lets callers use RFC 9457 application/problem+json (or any content type) for auth errors without a breaking change. WWW-Authenticate is set if the Response omitted it.

Fixes #5218

* fix(bearer-auth): use the status of the returned Response as is

* test(bearer-auth): cover a Response with its own status and WWW-Authenticate

---------
Workspace: /root/bench/ws/rebf815-hono-06a45bc/hono-06a45bc-2
Changed (0): nothing yet

## Map
- src/middleware/basic-auth/index.ts
  L12: type MessageFunction = (c: Context) => string | object | Promise<string | object>
  L14: type BasicAuthOptions = | { username: string password: string realm?: string hashFunction?: Function invalidUserMessage?: string | object | MessageFunction onAuthSuccess?: (c: Context, username: string) => void | Promise<void> } | { verifyUser: (username: string, password: string, c: Context) => boolean | Promise<boolean> realm?: string hashFunction?: Function invalidUserMessage?: string | object | MessageFunction onAuthSuccess?: (c: Context, username: string) => void | Promise<void> }
  L80: const basicAuth = ( options: BasicAuthOptions, ...users: { username: string; password: string }[] ): MiddlewareHandler =>
- src/middleware/bearer-auth/index.ts
  L12: const TOKEN_STRINGS = '[A-Za-z0-9._~+/-]+=*'
  L13: const PREFIX = 'Bearer'
  L14: const HEADER = 'Authorization'
  L16: type MessageFunction = (c: Context) => string | object | Promise<string | object>
  L17: type CustomizedErrorResponseOptions = { wwwAuthenticateHeader?: string | object | MessageFunction message?: string | object | MessageFunction }
  L22: type BearerAuthOptions<E extends Env = Env> = | { token: string | string[] realm?: string prefix?: string headerName?: string hashFunction?: Function /** * @deprecated Use noAuthenticationHeader.message instead */ noAuthenticationHeaderMessage?: string | object | MessageFunction noAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidAuthenticationHeader.message instead */ invalidAuthenticationHeaderMessage?: string | object | MessageFunction invalidAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidToken.message instead */ invalidTokenMessage?: string | object | MessageFunction invalidToken?: CustomizedErrorResponseOptions } | { realm?: string prefix?: string headerName?: string verifyToken: (token: string, c: Context<E>) => boolean | Promise<boolean> hashFunction?: Function /** * @deprecated Use noAuthenticationHeader.message instead */ noAuthenticationHeaderMessage?: string | object | MessageFunction noAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidAuthenticationHeader.message instead */ invalidAuthenticationHeaderMessage?: string | object | MessageFunction invalidAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidToken.message instead */ invalidTokenMessage?: string | object | MessageFunction invalidToken?: CustomizedErrorResponseOptions }
  L104: const bearerAuth = <E extends Env = Env>( options: BearerAuthOptions<E> ): MiddlewareHandler<E> =>
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
- src/adapter/aws-lambda/conninfo.ts
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
- sym:45790 bearerAuth (src/middleware/bearer-auth/index.ts:104-221)
  const bearerAuth = <E extends Env = Env>( options: BearerAuthOptions<E> ): MiddlewareHandler<E> =>
- sym:45789 BearerAuthOptions (src/middleware/bearer-auth/index.ts:22-66)
  type BearerAuthOptions<E extends Env = Env> = | { token: string | string[] realm?: string prefix?: string headerName?: string hashFunction?: Function /** * @deprecated Use noAuthenticationHeader.message instead */ noAuthenticationHeaderMessage?: string | object | MessageFunction noAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidAuthenticationHeader.message instead */ invalidAuthenticationHeaderMessage?: string | object | MessageFunction invalidAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidToken.message instead */ invalidTokenMessage?: string | object | MessageFunction invalidToken?: CustomizedErrorResponseOptions } | { realm?: string prefix?: string headerName?: string verifyToken: (token: string, c: Context<E>) => boolean | Promise<boolean> hashFunction?: Function /** * @deprecated Use noAuthenticationHeader.message instead */ noAuthenticationHeaderMessage?: string | object | MessageFunction noAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidAuthenticationHeader.message instead */ invalidAuthenticationHeaderMessage?: string | object | MessageFunction invalidAuthenticationHeader?: CustomizedErrorResponseOptions /** * @deprecated Use invalidToken.message instead */ invalidTokenMessage?: string | object | MessageFunction invalidToken?: CustomizedErrorResponseOptions }
- sym:45788 CustomizedErrorResponseOptions (src/middleware/bearer-auth/index.ts:17-20)
  type CustomizedErrorResponseOptions = { wwwAuthenticateHeader?: string | object | MessageFunction message?: string | object | MessageFunction }
- sym:45782 BasicAuthOptions (src/middleware/basic-auth/index.ts:14-29)
  type BasicAuthOptions = | { username: string password: string realm?: string hashFunction?: Function invalidUserMessage?: string | object | MessageFunction onAuthSuccess?: (c: Context, username: string) => void | Promise<void> } | { verifyUser: (username: string, password: string, c: Context) => boolean | Promise<boolean> realm?: string hashFunction?: Function invalidUserMessage?: string | object | MessageFunction onAuthSuccess?: (c: Context, username: string) => void | Promise<void> }
- sym:45263 detectResponseType (src/client/fetch-result-please.ts:80-95)
  function detectResponseType(response: Response): 'json' | 'text'
- sym:45864 jwk (src/middleware/jwk/jwk.ts:49-174)
  const jwk = ( options: { keys?: HonoJsonWebKey[] | ((ctx: Context) => Promise<HonoJsonWebKey[]> | HonoJsonWebKey[]) jwks_uri?: string | ((ctx: Context) => Promise<string> | string) allow_anon?: boolean cookie?: | string | { key: string; secret?: string | BufferSource; prefixOptions?: CookiePrefixOptions } headerName?: string alg: AsymmetricAlgorithm[] realm?: string verification?: VerifyOptions }, init?: RequestInit ): MiddlewareHandler =>
- sym:45463 parseResponseContent (src/helper/ssg/ssg.ts:74-88)
  const parseResponseContent = async (response: Response): Promise<string | ArrayBuffer> =>
- sym:46114 COMPRESSIBLE_CONTENT_TYPE_REGEX (src/utils/compress.ts:9-10)
  const COMPRESSIBLE_CONTENT_TYPE_REGEX =
- sym:45785 PREFIX (src/middleware/bearer-auth/index.ts:13-13)
  const PREFIX = 'Bearer'
- sym:46155 ResponseHeader (src/utils/headers.ts:266-343)
  type ResponseHeader = | 'Accept-Query' | 'Access-Control-Allow-Credentials' | 'Access-Control-Allow-Headers' | 'Access-Control-Allow-Methods' | 'Access-Control-Allow-Origin' | 'Access-Control-Expose-Headers' | 'Access-Control-Max-Age' | 'Activate-Storage-Access' | 'Age' | 'Allow' | 'Cache-Control' | 'Cache-Group-Invalidation' | 'Cache-Groups' | 'Clear-Site-Data' | 'Content-Disposition' | 'Content-Encoding' | 'Content-Language' | 'Content-Length' | 'Content-Location' | 'Content-Range' | 'Content-Security-Policy' | 'Content-Security-Policy-Report-Only' | 'Content-Type' | 'Cookie' | 'Cross-Origin-Embedder-Policy' | 'Cross-Origin-Opener-Policy' | 'Cross-Origin-Resource-Policy' | 'Date' | 'ETag' | 'Expires' | 'Incremental' | 'Last-Modified' | 'Location' | 'Permissions-Policy' | 'Pragma' | 'Retry-After' | 'Save-Data' | 'Sec-CH-Prefers-Color-Scheme' | 'Sec-CH-Prefers-Reduced-Motion' | 'Sec-CH-UA' | 'Sec-CH-UA-Arch' | 'Sec-CH-UA-Bitness' | 'Sec-CH-UA-Form-Factor' | 'Sec-CH-UA-Full-Version' | 'Sec-CH-UA-Full-Version-List' | 'Sec-CH-UA-Mobile' | 'Sec-CH-UA-Model' | 'Sec-CH-UA-Platform' | 'Sec-CH-UA-Platform-Version' | 'Sec-CH-UA-WoW64' | 'Sec-Fetch-Dest' | 'Sec-Fetch-Mode' | 'Sec-Fetch-Site' | 'Sec-Fetch-User' | 'Sec-GPC' | 'Server' | 'Server-Timing' | 'Service-Worker-Navigation-Preload' | 'Set-Cookie' | 'Set-Txn' | 'Strict-Transport-Security' | 'Timing-Allow-Origin' | 'Trailer' | 'Transfer-Encoding' | 'Unencoded-Digest' | 'Upgrade' | 'Vary' | 'Want-Unencoded-Digest' | 'Warning' | 'WWW-Authenticate' | 'X-Content-Type-Options' | 'X-DNS-Prefetch-Control' | 'X-Frame-Options' | 'X-Permitted-Cross-Domain-Policies' | 'X-Powered-By' | 'X-Robots-Tag' | 'X-XSS-Protection'
- sym:45786 HEADER (src/middleware/bearer-auth/index.ts:14-14)
  const HEADER = 'Authorization'
- sym:46154 RequestHeader (src/utils/headers.ts:8-264)
  type RequestHeader = | 'A-IM' | 'Accept' | 'Accept-Additions' | 'Accept-CH' | 'Accept-Charset' | 'Accept-Datetime' | 'Accept-Encoding' | 'Accept-Features' | 'Accept-Language' | 'Accept-Patch' | 'Accept-Post' | 'Accept-Query' | 'Accept-Ranges' | 'Accept-Signature' | 'Access-Control' | 'Access-Control-Allow-Credentials' | 'Access-Control-Allow-Headers' | 'Access-Control-Allow-Methods' | 'Access-Control-Allow-Origin' | 'Access-Control-Expose-Headers' | 'Access-Control-Max-Age' | 'Access-Control-Request-Headers' | 'Access-Control-Request-Method' | 'Activate-Storage-Access' | 'Age' | 'Allow' | 'ALPN' | 'Alt-Svc' | 'Alt-Used' | 'Alternates' | 'AMP-Cache-Transform' | 'Apply-To-Redirect-Ref' | 'Authentication-Control' | 'Authentication-Info' | 'Authorization' | 'Available-Dictionary' | 'C-Ext' | 'C-Man' | 'C-Opt' | 'C-PEP' | 'C-PEP-Info' | 'Cache-Control' | 'Cache-Group-Invalidation' | 'Cache-Groups' | 'Cache-Status' | 'Cal-Managed-ID' | 'CalDAV-Timezones' | 'Capsule-Protocol' | 'CDN-Cache-Control' | 'CDN-Loop' | 'Cert-Not-After' | 'Cert-Not-Before' | 'Clear-Site-Data' | 'Client-Cert' | 'Client-Cert-Chain' | 'Close' | 'CMCD-Object' | 'CMCD-Request' | 'CMCD-Session' | 'CMCD-Status' | 'CMSD-Dynamic' | 'CMSD-Static' | 'Concealed-Auth-Export' | 'Configuration-Context' | 'Connection' | 'Content-Base' | 'Content-Digest' | 'Content-Disposition' | 'Content-Encoding' | 'Content-ID' | 'Content-Language' | 'Content-Length' | 'Content-Location' | 'Content-MD5' | 'Content-Range' | 'Content-Script-Type' | 'Content-Security-Policy' | 'Content-Security-Policy-Report-Only' | 'Content-Style-Type' | 'Content-Type' | 'Content-Version' | 'Cookie' | 'Cookie2' | 'Cross-Origin-Embedder-Policy' | 'Cross-Origin-Embedder-Policy-Report-Only' | 'Cross-Origin-Opener-Policy' | 'Cross-Origin-Opener-Policy-Report-Only' | 'Cross-Origin-Resource-Policy' | 'CTA-Common-Access-Token' | 'DASL' | 'Date' | 'DAV' | 'Default-Style' | 'Delta-Base' | 'Deprecation' | 'Depth' | 'Derived-From' | 'Destination' | 'Detached-JWS' | 'Differential-ID' | 'Dictionary-ID' | 'Digest' | 'DPoP' | 'DPoP-Nonce' | 'Early-Data' | 'EDIINT-Features' | 'ETag' | 'Expect' | 'Expect-CT' | 'Expires' | 'Ext' | 'Forwarded' | 'From' | 'GetProfile' | 'Hobareg' | 'Host' | 'HTTP2-Settings' | 'If' | 'If-Match' | 'If-Modified-Since' | 'If-None-Match' | 'If-Range' | 'If-Schedule-Tag-Match' | 'If-Unmodified-Since' | 'IM' | 'Include-Referred-Token-Binding-ID' | 'Incremental' | 'Isolation' | 'Keep-Alive' | 'Label' | 'Last-Event-ID' | 'Last-Modified' | 'Link' | 'Link-Template' | 'Location' | 'Lock-Token' | 'Man' | 'Max-Forwards' | 'Memento-Datetime' | 'Meter' | 'Method-Check' | 'Method-Check-Expires' | 'MIME-Version' | 'Negotiate' | 'NEL' | 'OData-EntityId' | 'OData-Isolation' | 'OData-MaxVersion' | 'OData-Version' | 'Opt' | 'Optional-WWW-Authenticate' | 'Ordering-Type' | 'Origin' | 'Origin-Agent-Cluster' | 'OSCORE' | 'OSLC-Core-Version' | 'Overwrite' | 'P3P' | 'PEP' | 'PEP-Info' | 'Permissions-Policy' | 'PICS-Label' | 'Ping-From' | 'Ping-To' | 'Position' | 'Pragma' | 'Prefer' | 'Preference-Applied' | 'Priority' | 'ProfileObject' | 'Protocol' | 'Protocol-Info' | 'Protocol-Query' | 'Protocol-Request' | 'Proxy-Authenticate' | 'Proxy-Authentication-Info' | 'Proxy-Authorization' | 'Proxy-Features' | 'Proxy-Instruction' | 'Proxy-Status' | 'Public' | 'Public-Key-Pins' | 'Public-Key-Pins-Report-Only' | 'Range' | 'Redirect-Ref' | 'Referer' | 'Referer-Root' | 'Referrer-Policy' | 'Refresh' | 'Repeatability-Client-ID' | 'Repeatability-First-Sent' | 'Repeatability-Request-ID' | 'Repeatability-Result' | 'Replay-Nonce' | 'Reporting-Endpoints' | 'Repr-Digest' | 'Retry-After' | 'Safe' | 'Schedule-Reply' | 'Schedule-Tag' | 'Sec-Fetch-Dest' | 'Sec-Fetch-Mode' | 'Sec-Fetch-Site' | 'Sec-Fetch-Storage-Access' | 'Sec-Fetch-User' | 'Sec-GPC' | 'Sec-Purpose' | 'Sec-Token-Binding' | 'Sec-WebSocket-Accept' | 'Sec-WebSocket-Extensions' | 'Sec-WebSocket-Key' | 'Sec-WebSocket-Protocol' | 'Sec-WebSocket-Version' | 'Security-Scheme' | 'Server' | 'Server-Timing' | 'Set-Cookie' | 'Set-Cookie2' | 'Set-Txn' | 'SetProfile' | 'Signature' | 'Signature-Input' | 'SLUG' | 'SoapAction' | 'Status-URI' | 'Strict-Transport-Security' | 'Sunset' | 'Surrogate-Capability' | 'Surrogate-Control' | 'TCN' | 'TE' | 'Timeout' | 'Timing-Allow-Origin' | 'Topic' | 'Traceparent' | 'Tracestate' | 'Trailer' | 'Transfer-Encoding' | 'TTL' | 'Unencoded-Digest' | 'Upgrade' | 'Urgency' | 'URI' | 'Use-As-Dictionary' | 'User-Agent' | 'Variant-Vary' | 'Vary' | 'Via' | 'Want-Content-Digest' | 'Want-Digest' | 'Want-Repr-Digest' | 'Want-Unencoded-Digest' | 'Warning' | 'WWW-Authenticate' | 'X-Content-Type-Options' | 'X-Frame-Options'
- sym:45784 TOKEN_STRINGS (src/middleware/bearer-auth/index.ts:12-12)
  const TOKEN_STRINGS = '[A-Za-z0-9._~+/-]+=*'
- sym:45069 isContentTypeBinary (adapters/lambda-edge/src/handler.ts:231-239)
  const isContentTypeBinary = (contentType: string): boolean =>
- sym:45239 isContentTypeBinary (src/adapter/lambda-edge/handler.ts:255-263)
  const isContentTypeBinary = (contentType: string): boolean =>
- sym:45001 defaultIsContentTypeBinary (adapters/aws-lambda/src/handler.ts:670-678)
  const defaultIsContentTypeBinary = (contentType: string): boolean =>
- sym:45174 defaultIsContentTypeBinary (src/adapter/aws-lambda/handler.ts:711-719)
  const defaultIsContentTypeBinary = (contentType: string): boolean =>
- sym:45910 prettyJSON (src/middleware/pretty-json/index.ts:48-68)
  const prettyJSON = (options?: PrettyOptions): MiddlewareHandler =>
- sym:45331 BodyRespond (src/context.ts:121-141)
  interface BodyRespond
- sym:45865 unauthorizedResponse (src/middleware/jwk/jwk.ts:176-196)
  function unauthorizedResponse(opts: { ctx: Context error: string errDescription: string statusText?: string realm?: string })
- sym:45868 unauthorizedResponse (src/middleware/jwt/jwt.ts:165-185)
  function unauthorizedResponse(opts: { ctx: Context error: string errDescription: string statusText?: string realm?: string })
- sym:45867 jwt (src/middleware/jwt/jwt.ts:54-163)
  const jwt = (options: { secret: SignatureKey cookie?: | string | { key: string; secret?: string | BufferSource; prefixOptions?: CookiePrefixOptions } alg: SignatureAlgorithm headerName?: string realm?: string verification?: VerifyOptions }): MiddlewareHandler =>
- sym:45824 compress (src/middleware/compress/index.ts:72-136)
  const compress = (options?: CompressionOptions): MiddlewareHandler =>
- sym:45900 ApplicationHandler (src/middleware/mount/index.ts:11-11)
  type ApplicationHandler = (request: Request, ...args: any) => Response | Promise<Response>
- sym:45998 cloneRawRequest (src/request.ts:481-520)
  const cloneRawRequest = async (req: HonoRequest): Promise<Request> =>
- sym:45855 ipRestriction (src/middleware/ip-restriction/index.ts:218-279)
  const ipRestriction = ( getIP: GetIPAddr, { denyList = [], allowList = [] }: IPRestrictionRules, onError?: ( remote: { addr: string; type: AddressType }, c: Context ) => Response | Promise<Response> ): MiddlewareHandler =>
- sym:45537 HTTPExceptionOptions (src/http-exception.ts:14-18)
  type HTTPExceptionOptions = { res?: Response message?: string cause?: unknown }
- sym:45898 methodOverride (src/middleware/method-override/index.ts:60-138)
  const methodOverride = (options: MethodOverrideOptions): MiddlewareHandler =>
- sym:45795 bodyLimit (src/middleware/body-limit/index.ts:50-111)
  const bodyLimit = (options: BodyLimitOptions): MiddlewareHandler =>
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
- sym:45048 Env (adapters/lambda-edge/src/conninfo.ts:5-9)
- sym:45049 getConnInfo (adapters/lambda-edge/src/conninfo.ts:11-15)
- sym:45072 Env (adapters/netlify/src/conninfo.ts:28-32)
- sym:45073 getConnInfo (adapters/netlify/src/conninfo.ts:53-57)
- sym:45119 Env (src/adapter/aws-lambda/conninfo.ts:14-18)
- sym:45120 getConnInfo (src/adapter/aws-lambda/conninfo.ts:46-74)
- sym:45191 getConnInfo (src/adapter/bun/conninfo.ts:11-44)
- sym:45193 getBunServer (src/adapter/bun/server.ts:14-15)
- sym:45207 handleMiddleware (src/adapter/cloudflare-pages/handler.ts:58-111)
- sym:45218 Env (src/adapter/lambda-edge/conninfo.ts:5-9)
- sym:45219 getConnInfo (src/adapter/lambda-edge/conninfo.ts:14-18)
- sym:45241 Env (src/adapter/netlify/conninfo.ts:28-32)
- sym:45242 getConnInfo (src/adapter/netlify/conninfo.ts:54-58)
- sym:45285 Response (src/client/types.ts:232-232)
- sym:45317 compose (src/compose.ts:15-73)
- sym:45321 ExecutionContext (src/context.ts:31-52)
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
- sym:46332 onErrorHandler (src/validator/validator.test.ts:119-124)
- sym:45802 Context (src/middleware/cache/index.test.ts:39-48)
- sym:45202 Env (src/adapter/cloudflare-pages/handler.test.ts:7-11)

---
tokens: 7181/8000
