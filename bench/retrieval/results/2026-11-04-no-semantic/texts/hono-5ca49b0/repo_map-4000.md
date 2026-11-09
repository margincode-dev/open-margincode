src/helper/css/common.ts:
  L4: export const DEFAULT_STYLE_ID = 'hono-css'
  L8: export const STYLE_STRING: unique symbol = Symbol()
  L11: const CSS_ESCAPED: unique symbol = Symbol()
  L13: export interface CssClassName {
  L21: export const IS_CSS_ESCAPED = Symbol()
  L23: interface CssEscapedString {
  L32: export const rawCssString = (value: string): CssEscapedString => {
  L112: type CssVariableBasicType =
  L120: type CssVariableAsyncType = Promise<CssVariableBasicType>
  L121: type CssVariableArrayType = (CssVariableBasicType | CssVariableAsyncType)[]
  L122: export type CssVariableType = CssVariableBasicType | CssVariableAsyncType | CssVariableArrayType
  L143: export const buildStyleString = (
  L205: export const cssCommon = (
  L2: export const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
  L6: export const SELECTOR: unique symbol = Symbol()
  L7: export const CLASS_NAME: unique symbol = Symbol()
  L9: export const SELECTORS: unique symbol = Symbol()
  L10: export const EXTERNAL_CLASS_NAMES: unique symbol = Symbol()
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
  L244: export const cxCommon = (
  L264: export const keyframesCommon = (
  L294: type ViewTransitionType = {
src/helper/css/index.ts:
  L32: type CssClassName = HtmlEscapedString & CssClassNameCommon
  L39: interface CssType {
  L59: interface StyleType {
  L72: export const createCssContext = ({
  L34: type usedClassNameData = [
  L43: interface CxType {
  L49: interface KeyframesType {
  L53: interface ViewTransitionType {
  L217: interface DefaultContextType {
src/jsx/dom/css.ts:
  L68: interface CreateCssJsxDomObjectsType {
  L77: export const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) => {
  L145: interface CssType {
  L180: export const createCssContext = ({
  L27: const splitRule = (rule: string): string[] => {
  L149: interface CxType {
  L153: interface KeyframesType {
  L157: interface ViewTransitionType {
  L163: interface DefaultContextType {
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
src/helper/css/index.test.tsx:
  L24: async function toCSS(
  L12: async function toString(
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
src/jsx/utils.ts:
  L88: const isValidStylePropertyName = (name: string): boolean => {
  L122: const hasUnsafeStyleValue = (value: string): boolean => {
  L169: export const styleObjectForEach = (
  L11: export const normalizeIntrinsicElementKey = (key: string): string =>
  L25: const cacheValidName = (cache: Set<unknown>, max: number, name: string): void => {
  L32: export const isValidTagName = (name: unknown): name is string => {
  L49: export const isValidAttributeName = (name: string): boolean => {
src/jsx/dom/intrinsic-element/components.ts:
  L284: export const style: FC<PropsWithChildren<IntrinsicElements['style']>> = (props) => {
  L18: export const clearCache = () => {
  L24: export const composeRef = <T>(
  L58: const documentMetadataTag = (
  L257: export const title: FC<PropsWithChildren> = (props) => {
  L272: export const script: FC<PropsWithChildren<IntrinsicElements['script']>> = (props) => {
  L298: export const link: FC<PropsWithChildren<IntrinsicElements['link']>> = (props) => {
  L314: export const meta: FC<PropsWithChildren> = (props) => {
  L319: export const form: FC<
  L388: const formActionableElement = (
  L423: export const input: FC<PropsWithChildren<IntrinsicElements['input']>> = (props) =>
  L426: export const button: FC<PropsWithChildren<IntrinsicElements['button']>> = (props) =>
src/jsx/intrinsic-element/components.ts:
  L151: export const style: FC<PropsWithChildren<IntrinsicElements['style']>> = ({
  L21: const insertIntoHead: (
  L93: const returnWithoutSpecialBehavior = (tag: string, children: Child, props: Props) =>
  L96: const documentMetadataTag = (tag: string, children: Child, props: Props, sort: boolean) => {
  L121: export const title: FC<PropsWithChildren> = ({ children, ...props }) => {
  L136: export const script: FC<PropsWithChildren<IntrinsicElements['script']>> = ({
  L162: export const link: FC<PropsWithChildren<IntrinsicElements['link']>> = ({ children, ...props }) => {
  L171: export const meta: FC<PropsWithChildren> = ({ children, ...props }) => {
  L179: const newJSXNode = (tag: string, { children, ...props }: PropsWithChildren<unknown>) =>
  L182: export const form: FC<
  L194: const formActionableElement = (
  L207: export const input: (props: PropsWithChildren) => unknown = (props) =>
  L209: export const button: (props: PropsWithChildren) => unknown = (props) =>
