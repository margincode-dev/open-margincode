src/client/types.ts:
  L251: export type InferResponseType<T, U extends StatusCode = StatusCode> = InferResponseTypeFromEndpoint<
  L256: type InferResponseTypeFromEndpoint<T extends Endpoint, U extends StatusCode> = T extends {
  L284: export type FilterClientResponseByStatusCode<
  L388: export type PickResponseByStatusCode<App, U extends StatusCode> =
  L120: type ClientResponseOfEndpoint<T extends Endpoint = Endpoint> = T extends {
  L128: export interface ClientResponse<
  L232: export interface Response extends ClientResponse<unknown> {}
  L239: type InferEndpointType<T> = T extends (
  L265: export type InferRequestType<T> = T extends (
  L273: export type InferRequestOptionsType<T> = T extends (
  L328: export type ObjectType<T = unknown> = {
  L332: type GlobalResponseDefinition = {
  L358: export type ApplyGlobalResponse<App, Def extends GlobalResponseDefinition> =
  L11: type MethodNameAll = `$${typeof METHOD_NAME_ALL_LOWERCASE}`
  L17: type StandardMethods = `$${(typeof METHODS)[number]}`
  L24: type ExpandAllMethod<S> = MethodNameAll extends keyof S
  L28: type HonoRequest = (typeof Hono.prototype)['request']
  L30: export type BuildSearchParamsFn = (query: Record<string, string | string[]>) => URLSearchParams
  L32: export type ClientRequestOptions<T = unknown> = {
  L67: export type ClientRequest<Prefix extends string, Path extends string, S extends Schema> = {
  L156: type BuildSearch<Arg, Key extends 'query'> = Arg extends { [K in Key]: infer Query }
  L162: type BuildPathname<P extends string, Arg> = Arg extends { param: infer Param }
  L166: type BuildPath<P extends string, Arg> = `${BuildPathname<P, Arg>}${BuildSearch<Arg, 'query'>}`
  L168: type BuildTypedURL<
  L176: type HonoURL<Prefix extends string, Path extends string, Arg> =
  L188: type ParseHostName<T extends string> = T extends `${infer Host}:${infer Port}`
  L191: type TrimStartSlash<T extends string> = T extends `/${infer R}` ? TrimStartSlash<R> : T
  L192: type TrimEndSlash<T extends string> = T extends `${infer R}/` ? TrimEndSlash<R> : T
  L193: type IsLiteral<T extends string> = [T] extends [never] ? false : string extends T ? false : true
  L194: type ApplyParam<
  L213: type IsEmptyObject<T> = keyof T extends never ? true : false
  L215: export interface TypedURL<
  L234: export type Fetch<T> = (
  L294: type PathToChain<
  L311: export type Client<T, Prefix extends string> =
  L320: export type Callback = (opts: CallbackOptions) => unknown
  L322: interface CallbackOptions {
  L338: type ToEndpoints<Def extends GlobalResponseDefinition, R> = {
  L348: type ModRoute<R, Def extends GlobalResponseDefinition> = R extends Endpoint
  L352: type ModSchema<D, Def extends GlobalResponseDefinition> = {
  L365: type PickRoute<R, U extends StatusCode> = R extends Endpoint
  L371: type PickSchema<D, U extends StatusCode> = {
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
src/context.ts:
  L281: const setDefaultContentType = (contentType: string, headers?: HeaderRecord): HeaderRecord => {
  L113: interface NewResponse {
  L181: interface JSONRespond {
  L205: type JSONRespondReturn<
  L264: type ResponseHeadersInit =
  L271: interface ResponseInit<T extends StatusCode = StatusCode> {
  L277: type ResponseOrInit<T extends StatusCode = StatusCode> = ResponseInit<T> | Response
  L288: const createResponseInstance = (
  L608: #newResponse(
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
  L121: interface BodyRespond {
  L156: interface TextRespond {
  L220: interface HTMLRespond {
  L237: type ContextOptions<E extends Env> = {
  L254: interface SetHeadersOptions {
  L258: interface SetHeaders {
  L279: export const TEXT_PLAIN = 'text/plain; charset=UTF-8'
  L293: export class Context<
  L352: constructor(req: Request, options?: ContextOptions<E>) {
  L366: get req(): HonoRequest<P, I['out']> {
  L377: get event(): FetchEventLike {
  L391: get executionCtx(): ExecutionContext {
  L403: get res(): Response {
  L414: set res(_res: Response | undefined) {
  L597: get var(): Readonly<
src/request.ts:
  L258: json<T = any>(): Promise<T> {
  L20: type Body = {
  L27: type BodyCache = Partial<Body>
  L29: type OptionalRequestInitProperties = 'window' | 'priority'
  L30: type RequiredRequestInit = Required<Omit<RequestInit, OptionalRequestInitProperties>> & {
  L34: export class HonoRequest<P extends string = '/', I extends Input['out'] = {}> {
  L69: constructor(
  L99: param(key?: string): unknown {
  L103: #getDecodedParam(key: string): string | undefined {
  L109: #getAllDecodedParams(): Record<string, string> {
  L123: #getParamValue(paramKey: any): string | undefined {
  L147: query(key?: string) {
  L166: queries(key?: string) {
  L185: header(name?: string) {
  L213: async parseBody(options?: Partial<ParseBodyOptions>) {
  L274: text(): Promise<string> {
  L290: arrayBuffer(): Promise<ArrayBuffer> {
  L306: bytes(): Promise<Uint8Array> {
  L320: blob(): Promise<Blob> {
  L334: formData(): Promise<FormData> {
  L344: addValidatedData(target: keyof ValidationTargets, data: {}) {
  L357: valid(target: keyof ValidationTargets) {
  L374: get url(): string {
  L390: get method(): string {
  L394: get [GET_MATCH_RESULT](): Result<[unknown, RouterRoute]> {
  L425: get matchedRoutes(): RouterRoute[] {
  L445: get routePath(): string {
  L481: export const cloneRawRequest = async (req: HonoRequest): Promise<Request> => {
src/jsx/base.ts:
  L217: get type(): string | Function {
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
src/helper/css/index.ts:
  L39: interface CssType {
  L43: interface CxType {
  L49: interface KeyframesType {
  L53: interface ViewTransitionType {
  L59: interface StyleType {
  L217: interface DefaultContextType {
  L32: type CssClassName = HtmlEscapedString & CssClassNameCommon
  L34: type usedClassNameData = [
  L72: export const createCssContext = ({
src/router.ts:
  L21: export const MESSAGE_MATCHER_IS_ALREADY_BUILT =
  L9: export const METHOD_NAME_ALL = 'ALL' as const
  L13: export const METHOD_NAME_ALL_LOWERCASE = 'all' as const
  L17: export const METHODS = ['get', 'post', 'put', 'delete', 'options', 'patch', 'query'] as const
  L29: export interface Router<T> {
  L57: export type ParamIndexMap = Record<string, number>
  L61: export type ParamStash = string[]
  L65: export type Params = Record<string, string>
  L98: export type Result<T> = [[T, ParamIndexMap][], ParamStash] | [[T, Params][]]
  L103: export class UnsupportedPathError extends Error {}
src/helper/css/common.ts:
  L112: type CssVariableBasicType =
  L120: type CssVariableAsyncType = Promise<CssVariableBasicType>
  L121: type CssVariableArrayType = (CssVariableBasicType | CssVariableAsyncType)[]
  L122: export type CssVariableType = CssVariableBasicType | CssVariableAsyncType | CssVariableArrayType
  L294: type ViewTransitionType = {
  L2: export const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
  L4: export const DEFAULT_STYLE_ID = 'hono-css'
  L6: export const SELECTOR: unique symbol = Symbol()
  L7: export const CLASS_NAME: unique symbol = Symbol()
  L8: export const STYLE_STRING: unique symbol = Symbol()
  L9: export const SELECTORS: unique symbol = Symbol()
  L10: export const EXTERNAL_CLASS_NAMES: unique symbol = Symbol()
  L11: const CSS_ESCAPED: unique symbol = Symbol()
  L13: export interface CssClassName {
  L21: export const IS_CSS_ESCAPED = Symbol()
  L23: interface CssEscapedString {
  L32: export const rawCssString = (value: string): CssEscapedString => {
  L43: const toHash = (str: string): string => {
  L52: const normalizeLabel = (label: string): string => {
  L56: const isValidClassName = (name: string): boolean => /^-?[_a-zA-Z][_a-zA-Z0-9-]*$/.test(name)
  L60: const hasUnsafeSelectorChar = (name: string): boolean => /[<{}]/.test(name)
  L63: const RESERVED_KEYFRAME_NAMES = new Set([
  L72: const isValidKeyframeName = (name: string): boolean =>
  L75: const defaultOnInvalidSlug = (slug: string) => {
  L108: export const minify = (css: string): string => {
  L134: export type ClassNameSlug = (hash: string, label: string, styleString: string) => string
  L141: export type OnInvalidSlug = (slug: string) => void
  L143: export const buildStyleString = (
  L205: export const cssCommon = (
  L244: export const cxCommon = (
  L264: export const keyframesCommon = (
src/jsx/dom/css.ts:
  L68: interface CreateCssJsxDomObjectsType {
  L145: interface CssType {
  L149: interface CxType {
  L153: interface KeyframesType {
  L157: interface ViewTransitionType {
  L163: interface DefaultContextType {
  L27: const splitRule = (rule: string): string[] => {
  L77: export const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) => {
  L180: export const createCssContext = ({
src/utils/types.ts:
  L23: export type JSONPrimitive = string | boolean | number | null
  L24: export type JSONArray = (JSONPrimitive | JSONObject | JSONArray)[]
  L25: export type JSONObject = {
  L28: export type InvalidJSONValue = undefined | symbol | ((...args: unknown[]) => unknown)
  L39: export type JSONValue = JSONObject | JSONArray | JSONPrimitive
  L53: export type JSONParsed<T, TError = bigint | ReadonlyArray<bigint>> = T extends {
  L102: export type InterfaceToType<T> = T extends Function ? T : { [K in keyof T]: InterfaceToType<T[K]> }
  L7: export type Expect<T extends true> = T
  L8: export type Equal<X, Y> =
  L10: export type NotEqual<X, Y> = true extends Equal<X, Y> ? false : true
  L12: export type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends (
  L18: export type RemoveBlankRecord<T> =
  L21: export type IfAnyThenEmptyObject<T> = 0 extends 1 & T ? {} : T
  L30: type InvalidToNull<T> = T extends InvalidJSONValue ? null : T
  L32: type IsInvalid<T> = T extends InvalidJSONValue ? true : false
  L37: type OmitSymbolKeys<T> = { [K in keyof T as K extends symbol ? never : K]: T[K] }
  L93: export type Simplify<T> = { [KeyType in keyof T]: T[KeyType] } & {}
  L98: export type SimplifyDeepArray<T> = T extends any[]
  L104: export type RequiredKeysOf<BaseType extends object> = Exclude<
  L111: export type HasRequiredKeys<BaseType extends object> =
  L114: export type IsAny<T> = boolean extends (T extends never ? true : false) ? true : false
  L120: export type StringLiteralUnion<T> = T | (string & Record<never, never>)
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
src/client/client.test.ts:
  L24: toJSON() {
  L21: class SafeBigInt {
  L1239: const pathname = <T extends URL | string>(value: T): string =>
src/middleware/bearer-auth/index.ts:
  L16: type MessageFunction = (c: Context) => string | object | Promise<string | object>
  L22: type BearerAuthOptions<E extends Env = Env> =
  L104: export const bearerAuth = <E extends Env = Env>(
  L17: type CustomizedErrorResponseOptions = {
  L12: const TOKEN_STRINGS = '[A-Za-z0-9._~+/-]+=*'
  L13: const PREFIX = 'Bearer'
  L14: const HEADER = 'Authorization'
src/middleware/basic-auth/index.ts:
  L12: type MessageFunction = (c: Context) => string | object | Promise<string | object>
  L14: type BasicAuthOptions =
  L80: export const basicAuth = (
src/adapter/lambda-edge/handler.ts:
  L255: export const isContentTypeBinary = (contentType: string): boolean => {
  L61: export interface CloudFrontResponse {
  L9: interface CloudFrontHeader {
  L14: interface CloudFrontHeaders {
  L18: interface CloudFrontCustomOrigin {
  L29: interface CloudFrontS3Origin {
  L36: type CloudFrontOrigin =
  L43: export interface CloudFrontRequest {
  L70: export interface CloudFrontConfig {
  L77: interface CloudFrontEvent {
  L88: export interface CloudFrontEdgeEvent {
  L92: type CloudFrontContext = {}
  L97: export interface Callback {
  L102: interface CloudFrontResult {
  L119: const convertHeaders = (headers: Headers): CloudFrontHeaders => {
  L133: export const handle = (
  L167: const createResult = async (res: Response): Promise<CloudFrontResult> => {
  L190: const getCloudFrontRecord = (event: CloudFrontEdgeEvent): CloudFrontEvent['cf'] => {
  L200: const createRequest = (cf: CloudFrontEvent['cf']): Request => {
  L236: export const createBody = (
adapters/lambda-edge/src/handler.ts:
  L231: export const isContentTypeBinary = (contentType: string): boolean => {
  L55: export interface CloudFrontResponse {
  L9: interface CloudFrontHeader {
  L14: interface CloudFrontHeaders {
  L18: interface CloudFrontCustomOrigin {
  L29: interface CloudFrontS3Origin {
  L36: type CloudFrontOrigin =
  L40: export interface CloudFrontRequest {
  L61: export interface CloudFrontConfig {
  L68: interface CloudFrontEvent {
  L76: export interface CloudFrontEdgeEvent {
  L80: type CloudFrontContext = {}
  L82: export interface Callback {
  L87: interface CloudFrontResult {
  L104: const convertHeaders = (headers: Headers): CloudFrontHeaders => {
  L115: export const handle = (
  L149: const createResult = async (res: Response): Promise<CloudFrontResult> => {
  L172: const getCloudFrontRecord = (event: CloudFrontEdgeEvent): CloudFrontEvent['cf'] => {
  L182: const createRequest = (cf: CloudFrontEvent['cf']): Request => {
  L215: export const createBody = (
src/helper/ssg/ssg.ts:
  L25: const DEFAULT_CONTENT_TYPE = 'text/plain'
  L74: const parseResponseContent = async (response: Response): Promise<string | ArrayBuffer> => {
  L111: export type AfterResponseHook = (res: Response) => Response | false | Promise<Response | false>
  L139: export const combineAfterResponseHooks = (
  L310: export const saveContentToFile = async (
  L17: const DEFAULT_CONCURRENCY = 2 // default concurrency for ssg
  L27: export const DEFAULT_OUTPUT_DIR = './static'
  L34: export interface FileSystemModule {
  L44: export interface ToSSGResult {
  L50: const generateFilePath = (
  L99: const determineExtension = (
  L110: export type BeforeRequestHook = (req: Request) => Request | false | Promise<Request | false>
  L112: export type AfterGenerateHook = (
  L118: export const combineBeforeRequestHooks = (
  L160: export const combineAfterGenerateHooks = (
  L175: export interface SSGPlugin {
  L181: export interface ToSSGOptions {
  L342: export interface ToSSGInterface {
  L356: export interface ToSSGAdaptorInterface<
  L369: export const toSSG: ToSSGInterface = async (app, fs, options) => {
src/jsx/dom/render.ts:
  L37: export type PreserveNodeType =
  L82: export type PendingType =
  L17: const HONO_PORTAL_ELEMENT = '_hp'
  L30: export type HasRenderToDom = FC<any> & { [DOM_RENDERER]: FC<any> }
  L32: export type ErrorHandler = (error: any, retry: () => void) => Child | undefined
  L34: type Container = HTMLElement | DocumentFragment
  L35: type LocalJSXContexts = [JSXContext<unknown>, unknown][] | undefined
  L36: type SupportedElement = HTMLElement | SVGElement | MathMLElement
  L41: export type NodeObject = {
  L66: type NodeString = {
  L80: export type Node = NodeString | NodeObject
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
src/helper/css/index.test.tsx:
  L12: async function toString(
  L24: async function toCSS(
src/utils/url.ts:
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
  L140: export const getQueryStrings = (url: string): string => {
  L154: export const getPathNoStrict = (request: Request): string => {
  L171: export const mergePath: (...paths: string[]) => string = (
  L184: export const checkOptionalParameter = (path: string): string[] | null => {
  L221: export const tryDecodeURIComponent = (str: string): string =>
  L225: const _decodeURI = (value: string): string => {
  L232: const _getQueryParam = (
  L328: export const getQueryParams = (
adapters/aws-lambda/src/handler.ts:
  L670: export const defaultIsContentTypeBinary = (contentType: string): boolean => {
  L680: export const isContentEncodingBinary = (contentEncoding: string | null) => {
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
  L417: protected getQueryString(event: APIGatewayProxyEventV2): string {
  L421: protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void {
  L427: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L431: protected getHeaders(event: APIGatewayProxyEventV2): Headers {
  L447: export class EventV1Processor extends EventProcessor<APIGatewayProxyEvent> {
  L448: protected getPath(event: APIGatewayProxyEvent): string {
  L452: protected getMethod(event: APIGatewayProxyEvent): string {
  L456: protected getQueryString(event: APIGatewayProxyEvent): string {
  L474: protected getCookies(_event: APIGatewayProxyEvent, _headers: Headers): void {
  L478: protected getHeaders(event: APIGatewayProxyEvent): Headers {
  L498: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L507: export class ALBProcessor extends EventProcessor<ALBProxyEvent> {
  L508: protected getHeaders(event: ALBProxyEvent): Headers {
  L530: protected getPath(event: ALBProxyEvent): string {
  L534: protected getMethod(event: ALBProxyEvent): string {
  L538: protected getQueryString(event: ALBProxyEvent): string {
  L564: protected getCookies(event: ALBProxyEvent, headers: Headers): void {
  L576: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L588: export class LatticeV2Processor extends EventProcessor<LatticeProxyEventV2> {
  L589: protected getPath(event: LatticeProxyEventV2): string {
  L593: protected getMethod(event: LatticeProxyEventV2): string {
  L597: protected getQueryString(): string {
  L601: protected getHeaders(event: LatticeProxyEventV2): Headers {
  L615: protected getCookies(): void {
  L619: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L629: export const getProcessor = (event: LambdaEvent): EventProcessor<LambdaEvent> => {
  L643: const isProxyEventALB = (event: LambdaEvent): event is ALBProxyEvent => {
  L650: const isProxyEventV2 = (event: LambdaEvent): event is APIGatewayProxyEventV2 => {
  L657: const isLatticeEventV2 = (event: LambdaEvent): event is LatticeProxyEventV2 => {
src/adapter/aws-lambda/handler.ts:
  L711: export const defaultIsContentTypeBinary = (contentType: string): boolean => {
  L724: export const isContentEncodingBinary = (contentEncoding: string | null) => {
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
  L445: protected getQueryString(event: APIGatewayProxyEventV2): string {
  L449: protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void {
  L455: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L459: protected getHeaders(event: APIGatewayProxyEventV2): Headers {
  L478: export class EventV1Processor extends EventProcessor<APIGatewayProxyEvent> {
  L479: protected getPath(event: APIGatewayProxyEvent): string {
  L483: protected getMethod(event: APIGatewayProxyEvent): string {
  L487: protected getQueryString(event: APIGatewayProxyEvent): string {
  L505: protected getCookies(_event: APIGatewayProxyEvent, _headers: Headers): void {
  L509: protected getHeaders(event: APIGatewayProxyEvent): Headers {
  L529: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L541: export class ALBProcessor extends EventProcessor<ALBProxyEvent> {
  L542: protected getHeaders(event: ALBProxyEvent): Headers {
  L564: protected getPath(event: ALBProxyEvent): string {
  L568: protected getMethod(event: ALBProxyEvent): string {
