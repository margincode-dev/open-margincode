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
