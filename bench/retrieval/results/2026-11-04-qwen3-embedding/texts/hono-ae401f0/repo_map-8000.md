src/helper/css/common.ts:
  L2: export const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
  L6: export const SELECTOR: unique symbol = Symbol()
  L11: const CSS_ESCAPED: unique symbol = Symbol()
  L13: export interface CssClassName {
  L21: export const IS_CSS_ESCAPED = Symbol()
  L23: interface CssEscapedString {
  L32: export const rawCssString = (value: string): CssEscapedString => {
  L60: const hasUnsafeSelectorChar = (name: string): boolean => /[<{}]/.test(name)
  L110: type CssVariableBasicType =
  L118: type CssVariableAsyncType = Promise<CssVariableBasicType>
  L119: type CssVariableArrayType = (CssVariableBasicType | CssVariableAsyncType)[]
  L120: export type CssVariableType = CssVariableBasicType | CssVariableAsyncType | CssVariableArrayType
  L203: export const cssCommon = (
  L4: export const DEFAULT_STYLE_ID = 'hono-css'
  L7: export const CLASS_NAME: unique symbol = Symbol()
  L8: export const STYLE_STRING: unique symbol = Symbol()
  L9: export const SELECTORS: unique symbol = Symbol()
  L10: export const EXTERNAL_CLASS_NAMES: unique symbol = Symbol()
  L43: const toHash = (str: string): string => {
  L52: const normalizeLabel = (label: string): string => {
  L56: const isValidClassName = (name: string): boolean => /^-?[_a-zA-Z][_a-zA-Z0-9-]*$/.test(name)
  L63: const RESERVED_KEYFRAME_NAMES = new Set([
  L72: const isValidKeyframeName = (name: string): boolean =>
  L75: const defaultOnInvalidSlug = (slug: string) => {
  L106: export const minify = (css: string): string => {
  L132: export type ClassNameSlug = (hash: string, label: string, styleString: string) => string
  L139: export type OnInvalidSlug = (slug: string) => void
  L141: export const buildStyleString = (
  L242: export const cxCommon = (
  L262: export const keyframesCommon = (
  L292: type ViewTransitionType = {
src/helper/css/index.ts:
  L32: type CssClassName = HtmlEscapedString & CssClassNameCommon
  L39: interface CssType {
  L72: export const createCssContext = ({
  L34: type usedClassNameData = [
  L43: interface CxType {
  L49: interface KeyframesType {
  L53: interface ViewTransitionType {
  L59: interface StyleType {
  L217: interface DefaultContextType {
src/jsx/base.ts:
  L56: export const getNameSpaceContext = () => nameSpaceContext
  L166: export type Child =
  L467: export const isValidElement = (element: unknown): element is JSXNode => {
  L471: export const cloneElement = <T extends JSXNode | JSX.Element>(
  L24: export type Props = Record<string, any>
  L25: type FunctionComponentResult =
  L31: export type FC<P = Props> = {
  L36: export type DOMAttributes = HonoJSX.HTMLAttributes
  L58: const toSVGAttributeName = (key: string): string =>
  L114: type SuspendedContext = <T>(callback: () => T) => T
  L116: const resolveFunctionComponentResult = (
  L139: const childrenToStringToBuffer = (children: Child[], buffer: StringBufferWithCallbacks): void => {
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
src/request.ts:
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
  L258: json<T = any>(): Promise<T> {
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
src/jsx/dom/css.ts:
  L27: const splitRule = (rule: string): string[] => {
  L68: interface CreateCssJsxDomObjectsType {
  L77: export const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) => {
  L145: interface CssType {
  L180: export const createCssContext = ({
  L149: interface CxType {
  L153: interface KeyframesType {
  L157: interface ViewTransitionType {
  L163: interface DefaultContextType {
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
src/client/types.ts:
  L11: type MethodNameAll = `$${typeof METHOD_NAME_ALL_LOWERCASE}`
  L17: type StandardMethods = `$${(typeof METHODS)[number]}`
  L24: type ExpandAllMethod<S> = MethodNameAll extends keyof S
  L28: type HonoRequest = (typeof Hono.prototype)['request']
  L30: export type BuildSearchParamsFn = (query: Record<string, string | string[]>) => URLSearchParams
  L32: export type ClientRequestOptions<T = unknown> = {
  L67: export type ClientRequest<Prefix extends string, Path extends string, S extends Schema> = {
  L120: type ClientResponseOfEndpoint<T extends Endpoint = Endpoint> = T extends {
  L128: export interface ClientResponse<
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
src/jsx/dom/render.ts:
  L17: const HONO_PORTAL_ELEMENT = '_hp'
  L36: type SupportedElement = HTMLElement | SVGElement | MathMLElement
  L111: export const getNameSpaceContext = () => nameSpaceContext
  L336: const findInsertBefore = (node: Node | undefined): SupportedElement | Text | undefined => {
  L371: const findChildNodeIndex = (
  L30: export type HasRenderToDom = FC<any> & { [DOM_RENDERER]: FC<any> }
  L32: export type ErrorHandler = (error: any, retry: () => void) => Child | undefined
  L34: type Container = HTMLElement | DocumentFragment
  L35: type LocalJSXContexts = [JSXContext<unknown>, unknown][] | undefined
  L37: export type PreserveNodeType =
  L41: export type NodeObject = {
  L66: type NodeString = {
  L80: export type Node = NodeString | NodeObject
  L82: export type PendingType =
  L86: export type UpdateHook = (
  L91: export type Context =
  L113: const isNodeString = (node: Node): node is NodeString => 't' in (node as NodeString)
  L119: const getEventSpec = (key: string): [string, boolean] | undefined => {
  L135: const toAttributeName = (element: SupportedElement, key: string): string =>
  L144: const normalizeFormValue = (value: Props['value']) =>
  L147: const applySelectValue = (select: HTMLSelectElement, props: Props): void => {
  L161: const isIgnorableAttributeError = (error: unknown): boolean =>
  L164: const applyProps = (
  L276: const invokeTag = (context: Context, node: NodeObject): Child[] => {
  L295: const getNextChildren = (
  L343: const removeNode = (node: Node): void => {
  L366: const apply = (node: NodeObject, container: Container, isNew: boolean): void => {
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
src/helper/css/index.test.tsx:
  L24: async function toCSS(
  L12: async function toString(
src/jsx/utils.ts:
  L11: export const normalizeIntrinsicElementKey = (key: string): string =>
  L25: const cacheValidName = (cache: Set<unknown>, max: number, name: string): void => {
  L32: export const isValidTagName = (name: unknown): name is string => {
  L49: export const isValidAttributeName = (name: string): boolean => {
  L88: const isValidStylePropertyName = (name: string): boolean => {
  L122: const hasUnsafeStyleValue = (value: string): boolean => {
  L169: export const styleObjectForEach = (
src/helper/ssg/ssg.ts:
  L110: export type BeforeRequestHook = (req: Request) => Request | false | Promise<Request | false>
  L111: export type AfterResponseHook = (res: Response) => Response | false | Promise<Response | false>
  L112: export type AfterGenerateHook = (
  L118: export const combineBeforeRequestHooks = (
  L139: export const combineAfterResponseHooks = (
  L160: export const combineAfterGenerateHooks = (
  L17: const DEFAULT_CONCURRENCY = 2 // default concurrency for ssg
  L25: const DEFAULT_CONTENT_TYPE = 'text/plain'
  L27: export const DEFAULT_OUTPUT_DIR = './static'
  L34: export interface FileSystemModule {
  L44: export interface ToSSGResult {
  L50: const generateFilePath = (
  L74: const parseResponseContent = async (response: Response): Promise<string | ArrayBuffer> => {
  L99: const determineExtension = (
  L175: export interface SSGPlugin {
  L181: export interface ToSSGOptions {
  L310: export const saveContentToFile = async (
  L342: export interface ToSSGInterface {
  L356: export interface ToSSGAdaptorInterface<
  L369: export const toSSG: ToSSGInterface = async (app, fs, options) => {
src/router/trie-router/router.ts:
  L5: export class TrieRouter<T> implements Router<T> {
  L9: add(method: string, path: string, handler: T) {
  L15: match(method: string, path: string): Result<T> {
src/router/linear-router/router.ts:
  L5: type RegExpMatchArrayWithIndices = RegExpMatchArray & { indices: [number, number][] }
  L11: export class LinearRouter<T> implements Router<T> {
  L15: add(method: string, path: string, handler: T) {
  L25: match(method: string, path: string): Result<T> {
src/router/smart-router/router.ts:
  L4: export class SmartRouter<T> implements Router<T> {
  L9: constructor(init: { routers: Router<T>[] }) {
  L13: add(method: string, path: string, handler: T) {
  L21: match(method: string, path: string): Result<T> {
  L63: get activeRouter(): Router<T> {
src/router/pattern-router/router.ts:
  L4: type Route<T> = [RegExp, string, T] // [pattern, method, handler]
  L8: export class PatternRouter<T> implements Router<T> {
  L12: add(method: string, path: string, handler: T) {
  L38: match(method: string, path: string): Result<T> {
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
src/helper/adapter/index.ts:
  L8: export type Runtime = 'node' | 'deno' | 'bun' | 'workerd' | 'fastly' | 'edge-light' | 'other'
  L10: export const env = <
  L50: export const getRuntimeKey = (): Runtime => {
  L86: export const checkUserAgentEquals = (platform: string): boolean => {
src/router/reg-exp-router/matcher.ts:
  L4: export type HandlerData<T> = [T, ParamIndexMap][]
  L5: export type StaticMap<T> = Record<string, Result<T>>
  L6: export type Matcher<T> = [RegExp, HandlerData<T>[], StaticMap<T>]
  L7: export type MatcherMap<T> = Record<string, Matcher<T> | null>
  L10: export function match<R extends Router<T>, T>(this: R, method: string, path: string): Result<T> {
src/jsx/intrinsic-element/components.ts:
  L194: const formActionableElement = (
  L21: const insertIntoHead: (
  L93: const returnWithoutSpecialBehavior = (tag: string, children: Child, props: Props) =>
  L96: const documentMetadataTag = (tag: string, children: Child, props: Props, sort: boolean) => {
  L121: export const title: FC<PropsWithChildren> = ({ children, ...props }) => {
  L136: export const script: FC<PropsWithChildren<IntrinsicElements['script']>> = ({
  L151: export const style: FC<PropsWithChildren<IntrinsicElements['style']>> = ({
  L162: export const link: FC<PropsWithChildren<IntrinsicElements['link']>> = ({ children, ...props }) => {
  L171: export const meta: FC<PropsWithChildren> = ({ children, ...props }) => {
  L179: const newJSXNode = (tag: string, { children, ...props }: PropsWithChildren<unknown>) =>
  L182: export const form: FC<
  L207: export const input: (props: PropsWithChildren) => unknown = (props) =>
  L209: export const button: (props: PropsWithChildren) => unknown = (props) =>
src/jsx/context.ts:
  L8: export interface Context<T> extends FC<PropsWithChildren<{ value: T }>> {
  L16: type RenderStore = WeakMap<Context<unknown>, unknown[]>
  L18: type AsyncLocalStorageLike<T> = {
  L22: type AsyncLocalStorageClass = new <T>() => AsyncLocalStorageLike<T>
  L23: type AsyncHooksModule = { AsyncLocalStorage?: AsyncLocalStorageClass }
  L34: const loadAsyncLocalStorage = (): AsyncLocalStorageLike<RenderStore> | undefined => {
  L69: const getCurrentStore = (): RenderStore | undefined => {
  L73: const warnIfStorelessAccess = (): void => {
  L84: const getContextValuesIn = <T>(store: RenderStore | undefined, context: Context<T>): T[] => {
  L100: const readContextValueIn = <T>(store: RenderStore | undefined, context: Context<T>): T => {
  L109: type LocalContexts = [Context<unknown>, unknown][]
  L111: const captureContextValues = (store: RenderStore | undefined): LocalContexts =>
  L117: const resumeWithContextValues = <T>(
  L161: export const runWithRenderContext = <T>(callback: () => T, resumeStore?: RenderStore): T => {
  L196: export const captureRenderContext = (): (<T>(callback: () => T) => T) => {
  L213: export const createContext = <T>(defaultValue: T): Context<T> => {
  L264: export const useContext = <T>(context: Context<T>): T => {
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
src/router/trie-router/node.ts:
  L7: type HandlerSet<T> = {
  L13: type HandlerParamsSet<T> = HandlerSet<T> & {
  L20: export class Node<T> {
  L28: insert(method: string, path: string, handler: T): void {
  L64: #pushHandlerSets(
  L86: search(method: string, path: string): [[T, Params][]] {
src/jsx/dom/intrinsic-element/components.ts:
  L388: const formActionableElement = (
  L18: export const clearCache = () => {
  L24: export const composeRef = <T>(
  L58: const documentMetadataTag = (
  L257: export const title: FC<PropsWithChildren> = (props) => {
  L272: export const script: FC<PropsWithChildren<IntrinsicElements['script']>> = (props) => {
  L284: export const style: FC<PropsWithChildren<IntrinsicElements['style']>> = (props) => {
  L298: export const link: FC<PropsWithChildren<IntrinsicElements['link']>> = (props) => {
  L314: export const meta: FC<PropsWithChildren> = (props) => {
  L319: export const form: FC<
  L423: export const input: FC<PropsWithChildren<IntrinsicElements['input']>> = (props) =>
  L426: export const button: FC<PropsWithChildren<IntrinsicElements['button']>> = (props) =>
src/utils/color.ts:
  L14: export function getColorEnabled(): boolean {
  L36: export async function getColorEnabledAsync(): Promise<boolean> {
src/utils/accept.ts:
  L7: const isWhitespace = (char: number): boolean =>
  L10: const consumeWhitespace = (acceptHeader: string, startIndex: number): number => {
  L20: const ignoreTrailingWhitespace = (acceptHeader: string, startIndex: number): number => {
  L1: export interface Accept {
  L30: const skipInvalidParam = (acceptHeader: string, startIndex: number): [number, boolean] => {
  L46: const skipInvalidAcceptValue = (acceptHeader: string, startIndex: number): number => {
  L66: const getNextParam = (
  L159: const getNextAcceptValue = (
  L211: export const parseAccept = (acceptHeader: string): Accept[] => {
  L242: const parseQuality = (qVal?: string): number => {
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
src/jsx/constants.ts:
  L1: export const DOM_RENDERER = Symbol('RENDERER')
  L2: export const DOM_ERROR_HANDLER = Symbol('ERROR_HANDLER')
  L3: export const DOM_STASH = Symbol('STASH')
  L4: export const DOM_INTERNAL_TAG = Symbol('INTERNAL')
  L5: export const DOM_MEMO = Symbol('MEMO')
  L6: export const PERMALINK = Symbol('PERMALINK')
src/jsx/types.ts:
  L12: export type CSSProperties = JSX.CSSProperties
  L19: type ReactElement<P = any, T = string | Function> = JSXNode & {
  L11: export type PropsWithChildren<P = unknown> = P & { children?: Child | undefined }
  L24: type ReactNode = ReactElement | string | number | boolean | null | undefined
  L25: type ComponentClass<_P = {}, _S = {}> = unknown
  L29: export type Event = globalThis.Event
  L30: export type MouseEvent = globalThis.MouseEvent
  L31: export type KeyboardEvent = globalThis.KeyboardEvent
  L32: export type FocusEvent = globalThis.FocusEvent
  L33: export type ClipboardEvent = globalThis.ClipboardEvent
  L34: export type InputEvent = globalThis.InputEvent
  L35: export type PointerEvent = globalThis.PointerEvent
  L36: export type TouchEvent = globalThis.TouchEvent
  L37: export type WheelEvent = globalThis.WheelEvent
  L38: export type AnimationEvent = globalThis.AnimationEvent
  L39: export type TransitionEvent = globalThis.TransitionEvent
  L40: export type DragEvent = globalThis.DragEvent
src/jsx/dom/index.ts:
  L41: const createElement = (
  L60: const cloneElement = <T extends JSXNode | JSX.Element>(
  L76: const memo = <T>(
src/hono-base.ts:
  L31: const notFoundHandler: NotFoundHandler = (c) => {
  L35: const errorHandler: ErrorHandler = (err, c) => {
  L44: type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string
  L46: export type HonoOptions<E extends Env> = {
  L89: type MountOptionHandler = (c: Context) => unknown
  L90: type MountReplaceRequest = (originalRequest: Request) => Request
  L91: type MountOptions =
  L98: class Hono<
  L127: constructor(options: HonoOptions<E> = {}) {
  L178: #clone(): Hono<E, S, BasePath, CurrentPath> {
  L211: route<
  L250: basePath<SubPath extends string>(
  L333: mount(
  L390: #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void {
  L403: #handleError(err: unknown, c: Context<E>): Response | Promise<Response> {
  L410: #dispatch(
src/utils/jwt/types.ts:
  L34: export class JwtTokenNotBefore extends Error {
  L35: constructor(token: string) {
  L6: export class JwtAlgorithmNotImplemented extends Error {
  L7: constructor(alg: string) {
  L13: export class JwtAlgorithmRequired extends Error {
  L14: constructor() {
  L20: export class JwtAlgorithmMismatch extends Error {
  L21: constructor(expected: string, actual: string) {
  L27: export class JwtTokenInvalid extends Error {
  L28: constructor(token: string) {
  L41: export class JwtTokenExpired extends Error {
  L42: constructor(token: string) {
  L48: export class JwtTokenIssuedAt extends Error {
  L49: constructor(currentTimestamp: number, iat: number) {
  L57: export class JwtTokenIssuer extends Error {
  L58: constructor(expected: string | RegExp, iss: string | null) {
  L64: export class JwtHeaderInvalid extends Error {
  L65: constructor(header: object) {
  L71: export class JwtHeaderRequiresKid extends Error {
  L72: constructor(header: object) {
  L78: export class JwtSymmetricAlgorithmNotAllowed extends Error {
  L79: constructor(alg: string) {
  L85: export class JwtAlgorithmNotAllowed extends Error {
  L86: constructor(alg: string, allowedAlgorithms: readonly string[]) {
  L92: export class JwtTokenSignatureMismatched extends Error {
  L93: constructor(token: string) {
  L99: export class JwtPayloadRequiresAud extends Error {
  L100: constructor(payload: object) {
  L106: export class JwtTokenAudience extends Error {
  L107: constructor(expected: string | string[] | RegExp, aud: string | string[]) {
  L117: export enum CryptoKeyUsage {
  L131: export type JWTPayload = {
src/utils/body.ts:
  L10: const MAX_NESTED_OBJECTS = 10_000
  L225: const handleParsingNestedValues = (
  L9: const MAX_NESTING_DEPTH = 32
