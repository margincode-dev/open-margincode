src/request.ts:
  L147: query(key?: string) {
  L374: get url(): string {
  L34: export class HonoRequest<P extends string = '/', I extends Input['out'] = {}> {
  L69: constructor(
  L99: param(key?: string): unknown {
  L103: #getDecodedParam(key: string): string | undefined {
  L109: #getAllDecodedParams(): Record<string, string> {
  L123: #getParamValue(paramKey: any): string | undefined {
  L166: queries(key?: string) {
  L185: header(name?: string) {
  L213: async parseBody(options?: Partial<ParseBodyOptions>) {
  L258: json<T = any>(): Promise<T> {
  L274: text(): Promise<string> {
  L290: arrayBuffer(): Promise<ArrayBuffer> {
  L306: bytes(): Promise<Uint8Array> {
  L320: blob(): Promise<Blob> {
  L334: formData(): Promise<FormData> {
  L344: addValidatedData(target: keyof ValidationTargets, data: {}) {
  L357: valid(target: keyof ValidationTargets) {
  L390: get method(): string {
  L394: get [GET_MATCH_RESULT](): Result<[unknown, RouterRoute]> {
  L425: get matchedRoutes(): RouterRoute[] {
  L445: get routePath(): string {
  L20: type Body = {
  L27: type BodyCache = Partial<Body>
  L29: type OptionalRequestInitProperties = 'window' | 'priority'
  L30: type RequiredRequestInit = Required<Omit<RequestInit, OptionalRequestInitProperties>> & {
  L481: export const cloneRawRequest = async (req: HonoRequest): Promise<Request> => {
src/client/types.ts:
  L176: type HonoURL<Prefix extends string, Path extends string, Arg> =
  L28: type HonoRequest = (typeof Hono.prototype)['request']
  L168: type BuildTypedURL<
  L213: type IsEmptyObject<T> = keyof T extends never ? true : false
  L215: export interface TypedURL<
  L11: type MethodNameAll = `$${typeof METHOD_NAME_ALL_LOWERCASE}`
  L17: type StandardMethods = `$${(typeof METHODS)[number]}`
  L24: type ExpandAllMethod<S> = MethodNameAll extends keyof S
  L30: export type BuildSearchParamsFn = (query: Record<string, string | string[]>) => URLSearchParams
  L32: export type ClientRequestOptions<T = unknown> = {
  L67: export type ClientRequest<Prefix extends string, Path extends string, S extends Schema> = {
  L120: type ClientResponseOfEndpoint<T extends Endpoint = Endpoint> = T extends {
  L128: export interface ClientResponse<
  L156: type BuildSearch<Arg, Key extends 'query'> = Arg extends { [K in Key]: infer Query }
  L162: type BuildPathname<P extends string, Arg> = Arg extends { param: infer Param }
  L166: type BuildPath<P extends string, Arg> = `${BuildPathname<P, Arg>}${BuildSearch<Arg, 'query'>}`
  L188: type ParseHostName<T extends string> = T extends `${infer Host}:${infer Port}`
  L191: type TrimStartSlash<T extends string> = T extends `/${infer R}` ? TrimStartSlash<R> : T
  L192: type TrimEndSlash<T extends string> = T extends `${infer R}/` ? TrimEndSlash<R> : T
  L193: type IsLiteral<T extends string> = [T] extends [never] ? false : string extends T ? false : true
  L194: type ApplyParam<
  L232: export interface Response extends ClientResponse<unknown> {}
  L234: export type Fetch<T> = (
  L239: type InferEndpointType<T> = T extends (
  L251: export type InferResponseType<T, U extends StatusCode = StatusCode> = InferResponseTypeFromEndpoint<
  L256: type InferResponseTypeFromEndpoint<T extends Endpoint, U extends StatusCode> = T extends {
  L265: export type InferRequestType<T> = T extends (
  L273: export type InferRequestOptionsType<T> = T extends (
  L284: export type FilterClientResponseByStatusCode<
  L294: type PathToChain<
  L311: export type Client<T, Prefix extends string> =
  L320: export type Callback = (opts: CallbackOptions) => unknown
  L322: interface CallbackOptions {
  L328: export type ObjectType<T = unknown> = {
  L332: type GlobalResponseDefinition = {
  L338: type ToEndpoints<Def extends GlobalResponseDefinition, R> = {
  L348: type ModRoute<R, Def extends GlobalResponseDefinition> = R extends Endpoint
  L352: type ModSchema<D, Def extends GlobalResponseDefinition> = {
  L358: export type ApplyGlobalResponse<App, Def extends GlobalResponseDefinition> =
  L365: type PickRoute<R, U extends StatusCode> = R extends Endpoint
  L371: type PickSchema<D, U extends StatusCode> = {
  L388: export type PickResponseByStatusCode<App, U extends StatusCode> =
src/utils/url.ts:
  L140: export const getQueryStrings = (url: string): string => {
  L184: export const checkOptionalParameter = (path: string): string[] | null => {
  L232: const _getQueryParam = (
  L328: export const getQueryParams = (
  L6: export type Pattern = readonly [string, string, RegExp | true] | '*'
  L8: export const splitPath = (path: string): string[] => {
  L16: export const splitRoutingPath = (routePath: string): string[] => {
  L23: const extractGroupsFromPath = (path: string): { groups: [string, string][]; path: string } => {
  L35: const replaceGroupMarks = (paths: string[], groups: [string, string][]): string[] => {
  L51: export const getPattern = (label: string, next?: string): Pattern | null => {
  L80: type Decoder = (str: string) => string
  L81: export const tryDecode = (str: string, decoder: Decoder): string => {
  L104: export const tryDecodeURI = (str: string): string => tryDecode(str, decodeURI)
  L106: export const getPath = (request: Request): string => {
  L154: export const getPathNoStrict = (request: Request): string => {
  L171: export const mergePath: (...paths: string[]) => string = (
  L221: export const tryDecodeURIComponent = (str: string): string =>
  L225: const _decodeURI = (value: string): string => {
src/hono-base.ts:
  L46: export type HonoOptions<E extends Env> = {
  L113: class Hono<
  L142: constructor(options: HonoOptions<E> = {}) {
  L193: #clone(): Hono<E, S, BasePath, CurrentPath> {
  L226: route<
  L265: basePath<SubPath extends string>(
  L348: mount(
  L405: #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void {
  L418: #handleError(err: unknown, c: Context<E>): Response | Promise<Response> {
  L425: #dispatch(
  L31: const notFoundHandler: NotFoundHandler = (c) => {
  L35: const errorHandler: ErrorHandler = (err, c) => {
  L44: type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string
  L104: type MountOptionHandler = (c: Context) => unknown
  L105: type MountReplaceRequest = (originalRequest: Request) => Request
  L106: type MountOptions =
src/utils/html.ts:
  L11: type HtmlEscapedCallbackOpts = {
  L16: export type HtmlEscapedCallback = (opts: HtmlEscapedCallbackOpts) => Promise<string> | undefined
  L17: export type HtmlEscaped = {
  L21: export type HtmlEscapedString = string & HtmlEscaped
  L37: export type StringBuffer = (string | Promise<string>)[]
  L38: export type StringBufferWithCallbacks = StringBuffer & { callbacks: HtmlEscapedCallback[] }
  L40: export const raw = (value: unknown, callbacks?: HtmlEscapedCallback[]): HtmlEscapedString => {
  L53: export const stringBufferToString = async (
  L90: export const escapeToBuffer = (str: string, buffer: StringBuffer): void => {
  L129: export const resolveCallbackSync = (str: string | HtmlEscapedString): string => {
  L142: export const resolveCallback = async (
src/context.ts:
  L18: type HeaderRecord =
  L26: export type Data = string | ArrayBuffer | Blob | ReadableStream | Uint8Array<ArrayBuffer>
  L31: export interface ExecutionContext {
  L57: export interface ContextVariableMap {}
  L62: export interface ContextRenderer {}
  L71: interface DefaultRenderer {
  L78: export type Renderer = ContextRenderer extends Function ? ContextRenderer : DefaultRenderer
  L83: export type PropsForRenderer = [...Required<Parameters<Renderer>>] extends [unknown, infer Props]
  L88: export type Layout<T = Record<string, any>> = (props: T) => any
  L95: interface Get<E extends Env> {
  L105: interface Set<E extends Env> {
  L113: interface NewResponse {
  L121: interface BodyRespond {
  L156: interface TextRespond {
  L181: interface JSONRespond {
  L205: type JSONRespondReturn<
  L220: interface HTMLRespond {
  L237: type ContextOptions<E extends Env> = {
  L254: interface SetHeadersOptions {
  L258: interface SetHeaders {
  L264: type ResponseHeadersInit =
  L271: interface ResponseInit<T extends StatusCode = StatusCode> {
  L277: type ResponseOrInit<T extends StatusCode = StatusCode> = ResponseInit<T> | Response
  L279: export const TEXT_PLAIN = 'text/plain; charset=UTF-8'
  L281: const setDefaultContentType = (contentType: string, headers?: HeaderRecord): HeaderRecord => {
  L288: const createResponseInstance = (
  L293: export class Context<
  L352: constructor(req: Request, options?: ContextOptions<E>) {
  L366: get req(): HonoRequest<P, I['out']> {
  L377: get event(): FetchEventLike {
  L391: get executionCtx(): ExecutionContext {
  L403: get res(): Response {
  L414: set res(_res: Response | undefined) {
  L597: get var(): Readonly<
  L608: #newResponse(
src/jsx/base.ts:
  L24: export type Props = Record<string, any>
  L25: type FunctionComponentResult =
  L31: export type FC<P = Props> = {
  L36: export type DOMAttributes = HonoJSX.HTMLAttributes
  L56: export const getNameSpaceContext = () => nameSpaceContext
  L58: const toSVGAttributeName = (key: string): string =>
  L114: type SuspendedContext = <T>(callback: () => T) => T
  L116: const resolveFunctionComponentResult = (
  L139: const childrenToStringToBuffer = (children: Child[], buffer: StringBufferWithCallbacks): void => {
  L166: export type Child =
  L176: export const renderChildren = (children: Child[]): HtmlEscapedString | Promise<HtmlEscapedString> =>
  L185: export const isUntrustedObject = (value: unknown): boolean =>
  L194: export const renderUntrustedObject = (
  L202: export class JSXNode implements HtmlEscaped {
  L208: constructor(tag: string | Function, props: Props, children: Child[]) {
  L217: get type(): string | Function {
  L223: get ref(): any {
  L227: toString(): string | Promise<string> {
  L240: toStringToBuffer(buffer: StringBufferWithCallbacks): void {
  L315: class JSXFunctionNode extends JSXNode {
  L316: override toStringToBuffer(buffer: StringBufferWithCallbacks): void {
  L352: export class JSXFragmentNode extends JSXNode {
  L353: override toStringToBuffer(buffer: StringBufferWithCallbacks): void {
  L358: export const jsx = (tag: string | Function, props: Props | null, ...children: Child[]): JSXNode => {
  L373: export const jsxFn = (tag: string | Function, props: Props, children: Child[]): JSXNode => {
  L407: export const shallowEqual = (a: Props, b: Props): boolean => {
  L434: export type MemorableFC<T> = FC<T> & {
  L437: export const memo = <T>(
  L452: export const Fragment = ({
  L467: export const isValidElement = (element: unknown): element is JSXNode => {
  L471: export const cloneElement = <T extends JSXNode | JSX.Element>(
src/router.ts:
  L9: export const METHOD_NAME_ALL = 'ALL' as const
  L13: export const METHOD_NAME_ALL_LOWERCASE = 'all' as const
  L17: export const METHODS = ['get', 'post', 'put', 'delete', 'options', 'patch', 'query'] as const
  L21: export const MESSAGE_MATCHER_IS_ALREADY_BUILT =
  L29: export interface Router<T> {
  L57: export type ParamIndexMap = Record<string, number>
  L61: export type ParamStash = string[]
  L65: export type Params = Record<string, string>
  L98: export type Result<T> = [[T, ParamIndexMap][], ParamStash] | [[T, Params][]]
  L103: export class UnsupportedPathError extends Error {}
src/middleware/cache/index.test.ts:
  L7: type StoreMap = Map<string | Request, Response>
  L9: class MockCache {
  L13: constructor(name: string, store: StoreMap) {
  L18: async match(key: Request | string): Promise<Response | null> {
  L22: async keys() {
  L26: async put(key: Request | string, response: Response): Promise<void> {
  L39: class Context implements ExecutionContext {
  L40: passThroughOnException(): void {
  L43: async waitUntil(promise: Promise<unknown>): Promise<void> {
src/hono.ts:
  L16: export class Hono<
  L26: constructor(options: HonoOptions<E> = {}) {
src/preset/quick.ts:
  L13: export class Hono<
  L18: constructor(options: HonoOptions<E> = {}) {
src/preset/tiny.ts:
  L11: export class Hono<
  L16: constructor(options: HonoOptions<E> = {}) {
src/helper/css/index.test.tsx:
  L12: async function toString(
  L24: async function toCSS(
src/jsx/context.ts:
  L84: const getContextValuesIn = <T>(store: RenderStore | undefined, context: Context<T>): T[] => {
  L111: const captureContextValues = (store: RenderStore | undefined): LocalContexts =>
  L117: const resumeWithContextValues = <T>(
  L8: export interface Context<T> extends FC<PropsWithChildren<{ value: T }>> {
  L16: type RenderStore = WeakMap<Context<unknown>, unknown[]>
  L18: type AsyncLocalStorageLike<T> = {
  L22: type AsyncLocalStorageClass = new <T>() => AsyncLocalStorageLike<T>
  L23: type AsyncHooksModule = { AsyncLocalStorage?: AsyncLocalStorageClass }
  L34: const loadAsyncLocalStorage = (): AsyncLocalStorageLike<RenderStore> | undefined => {
  L69: const getCurrentStore = (): RenderStore | undefined => {
  L73: const warnIfStorelessAccess = (): void => {
  L100: const readContextValueIn = <T>(store: RenderStore | undefined, context: Context<T>): T => {
  L109: type LocalContexts = [Context<unknown>, unknown][]
  L161: export const runWithRenderContext = <T>(callback: () => T, resumeStore?: RenderStore): T => {
  L196: export const captureRenderContext = (): (<T>(callback: () => T) => T) => {
  L213: export const createContext = <T>(defaultValue: T): Context<T> => {
  L264: export const useContext = <T>(context: Context<T>): T => {
src/jsx/dom/render.ts:
  L17: const HONO_PORTAL_ELEMENT = '_hp'
  L37: export type PreserveNodeType =
  L30: export type HasRenderToDom = FC<any> & { [DOM_RENDERER]: FC<any> }
  L32: export type ErrorHandler = (error: any, retry: () => void) => Child | undefined
  L34: type Container = HTMLElement | DocumentFragment
  L35: type LocalJSXContexts = [JSXContext<unknown>, unknown][] | undefined
  L36: type SupportedElement = HTMLElement | SVGElement | MathMLElement
  L41: export type NodeObject = {
  L66: type NodeString = {
  L80: export type Node = NodeString | NodeObject
  L82: export type PendingType =
  L86: export type UpdateHook = (
  L91: export type Context =
  L111: export const getNameSpaceContext = () => nameSpaceContext
  L113: const isNodeString = (node: Node): node is NodeString => 't' in (node as NodeString)
  L119: const getEventSpec = (key: string): [string, boolean] | undefined => {
  L135: const toAttributeName = (element: SupportedElement, key: string): string =>
  L144: const normalizeFormValue = (value: Props['value']) =>
  L147: const applySelectValue = (select: HTMLSelectElement, props: Props): void => {
  L161: const isIgnorableAttributeError = (error: unknown): boolean =>
  L164: const applyProps = (
  L276: const invokeTag = (context: Context, node: NodeObject): Child[] => {
  L295: const getNextChildren = (
  L336: const findInsertBefore = (node: Node | undefined): SupportedElement | Text | undefined => {
  L343: const removeNode = (node: Node): void => {
  L366: const apply = (node: NodeObject, container: Container, isNew: boolean): void => {
  L371: const findChildNodeIndex = (
  L389: const applyNodeObject = (node: NodeObject, container: Container, isNew: boolean): void => {
  L485: const isSameContext = (
  L495: const indexChildrenByKey = (children: Node[]): Map<unknown, Node> | undefined => {
  L512: export const build = (context: Context, node: NodeObject, children?: Child[]): void => {
  L721: export const buildNode = (node: Child): Node | undefined => {
  L760: const replaceContainer = (node: NodeObject, from: DocumentFragment, to: Container): void => {
  L767: const updateSync = (context: Context, node: NodeObject): void => {
  L788: type UpdateMapResolve = (node: NodeObject | undefined) => void
  L794: export const update = async (
  L837: export const renderNode = (node: NodeObject, container: Container): void => {
  L850: export const render = (jsxNode: Child, container: Container): void => {
  L855: export const flushSync = (callback: () => void): void => {
  L869: export const createPortal = (children: Child, container: HTMLElement, key?: string): Child =>
src/router/trie-router/router.ts:
  L5: export class TrieRouter<T> implements Router<T> {
  L9: add(method: string, path: string, handler: T) {
  L15: match(method: string, path: string): Result<T> {
src/utils/http-status.ts:
  L6: export type InfoStatusCode = 100 | 101 | 102 | 103
  L7: export type SuccessStatusCode = 200 | 201 | 202 | 203 | 204 | 205 | 206 | 207 | 208 | 226
  L8: export type DeprecatedStatusCode = 305 | 306
  L9: export type RedirectStatusCode = 300 | 301 | 302 | 303 | 304 | DeprecatedStatusCode | 307 | 308
  L10: export type ClientErrorStatusCode =
  L40: export type ServerErrorStatusCode = 500 | 501 | 502 | 503 | 504 | 505 | 506 | 507 | 508 | 510 | 511
  L52: export type UnofficialStatusCode = -1
  L58: export type UnOfficalStatusCode = UnofficialStatusCode
  L63: export type StatusCode =
  L71: export type ContentlessStatusCode = 101 | 204 | 205 | 304
  L72: export type ContentfulStatusCode = Exclude<StatusCode, ContentlessStatusCode>
src/router/smart-router/router.ts:
  L4: export class SmartRouter<T> implements Router<T> {
  L9: constructor(init: { routers: Router<T>[] }) {
  L13: add(method: string, path: string, handler: T) {
  L21: match(method: string, path: string): Result<T> {
  L63: get activeRouter(): Router<T> {
src/router/linear-router/router.ts:
  L5: type RegExpMatchArrayWithIndices = RegExpMatchArray & { indices: [number, number][] }
  L11: export class LinearRouter<T> implements Router<T> {
  L15: add(method: string, path: string, handler: T) {
  L25: match(method: string, path: string): Result<T> {
src/router/pattern-router/router.ts:
  L4: type Route<T> = [RegExp, string, T] // [pattern, method, handler]
  L8: export class PatternRouter<T> implements Router<T> {
  L12: add(method: string, path: string, handler: T) {
  L38: match(method: string, path: string): Result<T> {
src/helper/adapter/index.ts:
  L8: export type Runtime = 'node' | 'deno' | 'bun' | 'workerd' | 'fastly' | 'edge-light' | 'other'
  L50: export const getRuntimeKey = (): Runtime => {
  L10: export const env = <
  L86: export const checkUserAgentEquals = (platform: string): boolean => {
adapters/bun/src/websocket.ts:
  L8: export interface BunServerWebSocket<T> {
  L15: export interface BunWebSocketHandler<T> {
  L24: export interface BunWebSocketData {
  L110: export const createBunWebSocket = <T>(): CreateWebSocket<T> => ({
  L20: interface CreateWebSocket<T> {
  L33: export const createWSContext = (ws: BunServerWebSocket<BunWebSocketData>): WSContext => {
src/adapter/bun/websocket.ts:
  L9: export interface BunServerWebSocket<T> {
  L19: export interface BunWebSocketHandler<T> {
  L31: export interface BunWebSocketData {
  L124: export const createBunWebSocket = <T>(): CreateWebSocket<T> => ({
  L24: interface CreateWebSocket<T> {
  L41: export const createWSContext = (ws: BunServerWebSocket<BunWebSocketData>): WSContext => {
adapters/aws-lambda/src/handler.ts:
  L417: protected getQueryString(event: APIGatewayProxyEventV2): string {
  L456: protected getQueryString(event: APIGatewayProxyEvent): string {
  L538: protected getQueryString(event: ALBProxyEvent): string {
  L597: protected getQueryString(): string {
  L14: function sanitizeHeaderValue(value: string): string {
  L24: export type LambdaEvent =
  L30: export interface LatticeProxyEventV2 {
  L42: export interface APIGatewayProxyEventV2 {
  L65: export interface APIGatewayProxyEvent {
  L86: export interface ALBProxyEvent {
  L100: type WithHeaders = {
  L104: type WithMultiValueHeaders = {
  L109: export type APIGatewayProxyResult = {
  L117: const getRequestContext = (
  L127: async function* readWebStream(
  L137: const streamToNodeStream = (
  L142: export const streamHandle = <
  L199: type HandleOptions = {
  L243: export const handle = <E extends Env = Env, S extends Schema = {}, BasePath extends string = '/'>(
  L282: export abstract class EventProcessor<E extends LambdaEvent> {
  L295: protected getHeaderValue(headers: E['headers'], key: string): string | undefined {
  L305: protected getDomainName(event: E): string | undefined {
  L322: createRequest(event: E): Request {
  L348: async createResult(
  L392: setCookies(_event: E, res: Response, result: APIGatewayProxyResult) {
  L408: export class EventV2Processor extends EventProcessor<APIGatewayProxyEventV2> {
  L409: protected getPath(event: APIGatewayProxyEventV2): string {
  L413: protected getMethod(event: APIGatewayProxyEventV2): string {
  L421: protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void {
  L427: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L431: protected getHeaders(event: APIGatewayProxyEventV2): Headers {
  L447: export class EventV1Processor extends EventProcessor<APIGatewayProxyEvent> {
  L448: protected getPath(event: APIGatewayProxyEvent): string {
  L452: protected getMethod(event: APIGatewayProxyEvent): string {
  L474: protected getCookies(_event: APIGatewayProxyEvent, _headers: Headers): void {
  L478: protected getHeaders(event: APIGatewayProxyEvent): Headers {
  L498: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L507: export class ALBProcessor extends EventProcessor<ALBProxyEvent> {
  L508: protected getHeaders(event: ALBProxyEvent): Headers {
  L530: protected getPath(event: ALBProxyEvent): string {
  L534: protected getMethod(event: ALBProxyEvent): string {
  L564: protected getCookies(event: ALBProxyEvent, headers: Headers): void {
  L576: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L588: export class LatticeV2Processor extends EventProcessor<LatticeProxyEventV2> {
  L589: protected getPath(event: LatticeProxyEventV2): string {
  L593: protected getMethod(event: LatticeProxyEventV2): string {
  L601: protected getHeaders(event: LatticeProxyEventV2): Headers {
  L615: protected getCookies(): void {
  L619: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L629: export const getProcessor = (event: LambdaEvent): EventProcessor<LambdaEvent> => {
  L643: const isProxyEventALB = (event: LambdaEvent): event is ALBProxyEvent => {
  L650: const isProxyEventV2 = (event: LambdaEvent): event is APIGatewayProxyEventV2 => {
  L657: const isLatticeEventV2 = (event: LambdaEvent): event is LatticeProxyEventV2 => {
  L670: export const defaultIsContentTypeBinary = (contentType: string): boolean => {
  L680: export const isContentEncodingBinary = (contentEncoding: string | null) => {
src/adapter/aws-lambda/handler.ts:
  L445: protected getQueryString(event: APIGatewayProxyEventV2): string {
  L487: protected getQueryString(event: APIGatewayProxyEvent): string {
  L572: protected getQueryString(event: ALBProxyEvent): string {
  L634: protected getQueryString(): string {
  L14: function sanitizeHeaderValue(value: string): string {
  L27: export type LambdaEvent =
  L36: export interface LatticeProxyEventV2 {
  L51: export interface APIGatewayProxyEventV2 {
  L77: export interface APIGatewayProxyEvent {
  L101: export interface ALBProxyEvent {
  L115: type WithHeaders = {
  L119: type WithMultiValueHeaders = {
  L127: export type APIGatewayProxyResult = {
  L135: const getRequestContext = (
  L145: async function* readWebStream(
  L155: const streamToNodeStream = (
  L163: export const streamHandle = <
  L220: type HandleOptions = {
  L265: export const handle = <E extends Env = Env, S extends Schema = {}, BasePath extends string = '/'>(
  L307: export abstract class EventProcessor<E extends LambdaEvent> {
  L320: protected getHeaderValue(headers: E['headers'], key: string): string | undefined {
  L330: protected getDomainName(event: E): string | undefined {
  L347: createRequest(event: E): Request {
  L373: async createResult(
  L417: setCookies(_event: E, res: Response, result: APIGatewayProxyResult) {
  L436: export class EventV2Processor extends EventProcessor<APIGatewayProxyEventV2> {
  L437: protected getPath(event: APIGatewayProxyEventV2): string {
  L441: protected getMethod(event: APIGatewayProxyEventV2): string {
  L449: protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void {
  L455: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L459: protected getHeaders(event: APIGatewayProxyEventV2): Headers {
  L478: export class EventV1Processor extends EventProcessor<APIGatewayProxyEvent> {
  L479: protected getPath(event: APIGatewayProxyEvent): string {
  L483: protected getMethod(event: APIGatewayProxyEvent): string {
  L505: protected getCookies(_event: APIGatewayProxyEvent, _headers: Headers): void {
  L509: protected getHeaders(event: APIGatewayProxyEvent): Headers {
  L529: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L541: export class ALBProcessor extends EventProcessor<ALBProxyEvent> {
  L542: protected getHeaders(event: ALBProxyEvent): Headers {
  L564: protected getPath(event: ALBProxyEvent): string {
  L568: protected getMethod(event: ALBProxyEvent): string {
  L598: protected getCookies(event: ALBProxyEvent, headers: Headers): void {
  L610: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L625: export class LatticeV2Processor extends EventProcessor<LatticeProxyEventV2> {
  L626: protected getPath(event: LatticeProxyEventV2): string {
  L630: protected getMethod(event: LatticeProxyEventV2): string {
  L638: protected getHeaders(event: LatticeProxyEventV2): Headers {
  L652: protected getCookies(): void {
  L656: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L669: export const getProcessor = (event: LambdaEvent): EventProcessor<LambdaEvent> => {
  L683: const isProxyEventALB = (event: LambdaEvent): event is ALBProxyEvent => {
  L690: const isProxyEventV2 = (event: LambdaEvent): event is APIGatewayProxyEventV2 => {
  L697: const isLatticeEventV2 = (event: LambdaEvent): event is LatticeProxyEventV2 => {
  L711: export const defaultIsContentTypeBinary = (contentType: string): boolean => {
  L724: export const isContentEncodingBinary = (contentEncoding: string | null) => {
src/router/reg-exp-router/node.ts:
  L3: export const LABEL_REG_EXP_STR = '[^/]+'
  L4: export const ONLY_WILDCARD_REG_EXP_STR = '.*'
  L5: export const TAIL_WILDCARD_REG_EXP_STR = '(?:|/.*)'
  L6: export const PATH_ERROR = Symbol()
  L8: export type ParamAssocArray = [string, number][]
  L9: export interface Context {
  L22: function compareKey(a: string, b: string): number {
  L47: export class Node {
  L53: insert(
  L139: buildRegExpStr(): string {
src/utils/body.ts:
  L198: const handleParsingAllValues = (
  L225: const handleParsingNestedValues = (
  L9: const MAX_NESTING_DEPTH = 32
  L10: const MAX_NESTED_OBJECTS = 10_000
  L12: type BodyDataValueDot = { [x: string]: string | File | BodyDataValueDot }
  L13: type BodyDataValueDotAll = {
  L16: type SimplifyBodyData<T> = {
  L26: type BodyDataValueComponent<T> =
  L34: type BodyDataValueObject<T> = { [key: string]: BodyDataValueComponent<T> | BodyDataValueObject<T> }
  L35: type BodyDataValue<T> =
  L42: export type BodyData<T extends Partial<ParseBodyOptions> = {}> = SimplifyBodyData<
  L46: export type ParseBodyOptions = {
  L80: const isRawRequest = (request: HonoRequest | Request): request is Request => 'headers' in request // 'headers' method exists only on Request, not on HonoRequest.
  L90: interface ParseBody {
  L100: export const parseBody: ParseBody = async (
  L126: async function parseFormData<T extends BodyData>(
  L160: function convertFormDataToBodyData<T extends BodyData = BodyData>(
  L261: const throwNestingLimitExceeded = (): never => {
src/router/reg-exp-router/matcher.ts:
  L4: export type HandlerData<T> = [T, ParamIndexMap][]
  L5: export type StaticMap<T> = Record<string, Result<T>>
  L6: export type Matcher<T> = [RegExp, HandlerData<T>[], StaticMap<T>]
  L7: export type MatcherMap<T> = Record<string, Matcher<T> | null>
  L10: export function match<R extends Router<T>, T>(this: R, method: string, path: string): Result<T> {
src/router/trie-router/node.ts:
  L7: type HandlerSet<T> = {
  L13: type HandlerParamsSet<T> = HandlerSet<T> & {
  L20: export class Node<T> {
  L28: insert(method: string, path: string, handler: T): void {
  L64: #pushHandlerSets(
  L86: search(method: string, path: string): [[T, Params][]] {
src/utils/types.ts:
  L21: export type IfAnyThenEmptyObject<T> = 0 extends 1 & T ? {} : T
  L7: export type Expect<T extends true> = T
  L8: export type Equal<X, Y> =
  L10: export type NotEqual<X, Y> = true extends Equal<X, Y> ? false : true
  L12: export type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends (
  L18: export type RemoveBlankRecord<T> =
  L23: export type JSONPrimitive = string | boolean | number | null
  L24: export type JSONArray = (JSONPrimitive | JSONObject | JSONArray)[]
  L25: export type JSONObject = {
  L28: export type InvalidJSONValue = undefined | symbol | ((...args: unknown[]) => unknown)
  L30: type InvalidToNull<T> = T extends InvalidJSONValue ? null : T
  L32: type IsInvalid<T> = T extends InvalidJSONValue ? true : false
  L37: type OmitSymbolKeys<T> = { [K in keyof T as K extends symbol ? never : K]: T[K] }
  L39: export type JSONValue = JSONObject | JSONArray | JSONPrimitive
