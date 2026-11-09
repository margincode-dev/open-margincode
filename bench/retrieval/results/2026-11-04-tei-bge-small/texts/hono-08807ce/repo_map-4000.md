src/router.ts:
  L29: export interface Router<T> {
  L65: export type Params = Record<string, string>
  L103: export class UnsupportedPathError extends Error {}
  L9: export const METHOD_NAME_ALL = 'ALL' as const
  L13: export const METHOD_NAME_ALL_LOWERCASE = 'all' as const
  L17: export const METHODS = ['get', 'post', 'put', 'delete', 'options', 'patch', 'query'] as const
  L21: export const MESSAGE_MATCHER_IS_ALREADY_BUILT =
  L57: export type ParamIndexMap = Record<string, number>
  L61: export type ParamStash = string[]
  L98: export type Result<T> = [[T, ParamIndexMap][], ParamStash] | [[T, Params][]]
src/adapter/cloudflare-pages/handler.ts:
  L9: type Params<P extends string = any> = Record<P, string | string[]>
  L15: export type EventContext<Env = {}, P extends string = any, Data = Record<string, unknown>> = {
  L38: export const handle =
  L58: export function handleMiddleware<E extends Env = {}, P extends string = any, I extends Input = {}>(
  L124: export const serveStatic = (): MiddlewareHandler => {
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
src/request.ts:
  L109: #getAllDecodedParams(): Record<string, string> {
  L123: #getParamValue(paramKey: any): string | undefined {
  L445: get routePath(): string {
  L20: type Body = {
  L27: type BodyCache = Partial<Body>
  L29: type OptionalRequestInitProperties = 'window' | 'priority'
  L30: type RequiredRequestInit = Required<Omit<RequestInit, OptionalRequestInitProperties>> & {
  L34: export class HonoRequest<P extends string = '/', I extends Input['out'] = {}> {
  L69: constructor(
  L99: param(key?: string): unknown {
  L103: #getDecodedParam(key: string): string | undefined {
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
  L481: export const cloneRawRequest = async (req: HonoRequest): Promise<Request> => {
src/utils/url.ts:
  L8: export const splitPath = (path: string): string[] => {
  L16: export const splitRoutingPath = (routePath: string): string[] => {
  L23: const extractGroupsFromPath = (path: string): { groups: [string, string][]; path: string } => {
  L106: export const getPath = (request: Request): string => {
  L154: export const getPathNoStrict = (request: Request): string => {
  L171: export const mergePath: (...paths: string[]) => string = (
  L328: export const getQueryParams = (
  L6: export type Pattern = readonly [string, string, RegExp | true] | '*'
  L35: const replaceGroupMarks = (paths: string[], groups: [string, string][]): string[] => {
  L51: export const getPattern = (label: string, next?: string): Pattern | null => {
  L80: type Decoder = (str: string) => string
  L81: export const tryDecode = (str: string, decoder: Decoder): string => {
  L104: export const tryDecodeURI = (str: string): string => tryDecode(str, decodeURI)
  L140: export const getQueryStrings = (url: string): string => {
  L184: export const checkOptionalParameter = (path: string): string[] | null => {
  L221: export const tryDecodeURIComponent = (str: string): string =>
  L225: const _decodeURI = (value: string): string => {
  L232: const _getQueryParam = (
src/client/types.ts:
  L30: export type BuildSearchParamsFn = (query: Record<string, string | string[]>) => URLSearchParams
  L166: type BuildPath<P extends string, Arg> = `${BuildPathname<P, Arg>}${BuildSearch<Arg, 'query'>}`
  L294: type PathToChain<
  L11: type MethodNameAll = `$${typeof METHOD_NAME_ALL_LOWERCASE}`
  L17: type StandardMethods = `$${(typeof METHODS)[number]}`
  L24: type ExpandAllMethod<S> = MethodNameAll extends keyof S
  L28: type HonoRequest = (typeof Hono.prototype)['request']
  L32: export type ClientRequestOptions<T = unknown> = {
  L67: export type ClientRequest<Prefix extends string, Path extends string, S extends Schema> = {
  L120: type ClientResponseOfEndpoint<T extends Endpoint = Endpoint> = T extends {
  L128: export interface ClientResponse<
  L156: type BuildSearch<Arg, Key extends 'query'> = Arg extends { [K in Key]: infer Query }
  L162: type BuildPathname<P extends string, Arg> = Arg extends { param: infer Param }
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
adapters/aws-lambda/src/handler.ts:
  L14: function sanitizeHeaderValue(value: string): string {
  L104: type WithMultiValueHeaders = {
  L295: protected getHeaderValue(headers: E['headers'], key: string): string | undefined {
  L409: protected getPath(event: APIGatewayProxyEventV2): string {
  L448: protected getPath(event: APIGatewayProxyEvent): string {
  L530: protected getPath(event: ALBProxyEvent): string {
  L589: protected getPath(event: LatticeProxyEventV2): string {
  L24: export type LambdaEvent =
  L30: export interface LatticeProxyEventV2 {
  L42: export interface APIGatewayProxyEventV2 {
  L65: export interface APIGatewayProxyEvent {
  L86: export interface ALBProxyEvent {
  L100: type WithHeaders = {
  L109: export type APIGatewayProxyResult = {
  L117: const getRequestContext = (
  L127: async function* readWebStream(
  L137: const streamToNodeStream = (
  L142: export const streamHandle = <
  L199: type HandleOptions = {
  L243: export const handle = <E extends Env = Env, S extends Schema = {}, BasePath extends string = '/'>(
  L282: export abstract class EventProcessor<E extends LambdaEvent> {
  L305: protected getDomainName(event: E): string | undefined {
  L322: createRequest(event: E): Request {
  L348: async createResult(
  L392: setCookies(_event: E, res: Response, result: APIGatewayProxyResult) {
  L408: export class EventV2Processor extends EventProcessor<APIGatewayProxyEventV2> {
  L413: protected getMethod(event: APIGatewayProxyEventV2): string {
  L417: protected getQueryString(event: APIGatewayProxyEventV2): string {
  L421: protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void {
  L427: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L431: protected getHeaders(event: APIGatewayProxyEventV2): Headers {
  L447: export class EventV1Processor extends EventProcessor<APIGatewayProxyEvent> {
  L452: protected getMethod(event: APIGatewayProxyEvent): string {
  L456: protected getQueryString(event: APIGatewayProxyEvent): string {
  L474: protected getCookies(_event: APIGatewayProxyEvent, _headers: Headers): void {
  L478: protected getHeaders(event: APIGatewayProxyEvent): Headers {
  L498: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L507: export class ALBProcessor extends EventProcessor<ALBProxyEvent> {
  L508: protected getHeaders(event: ALBProxyEvent): Headers {
  L534: protected getMethod(event: ALBProxyEvent): string {
  L538: protected getQueryString(event: ALBProxyEvent): string {
  L564: protected getCookies(event: ALBProxyEvent, headers: Headers): void {
  L576: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L588: export class LatticeV2Processor extends EventProcessor<LatticeProxyEventV2> {
  L593: protected getMethod(event: LatticeProxyEventV2): string {
  L597: protected getQueryString(): string {
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
  L14: function sanitizeHeaderValue(value: string): string {
  L119: type WithMultiValueHeaders = {
  L320: protected getHeaderValue(headers: E['headers'], key: string): string | undefined {
  L437: protected getPath(event: APIGatewayProxyEventV2): string {
  L479: protected getPath(event: APIGatewayProxyEvent): string {
  L564: protected getPath(event: ALBProxyEvent): string {
  L626: protected getPath(event: LatticeProxyEventV2): string {
  L27: export type LambdaEvent =
  L36: export interface LatticeProxyEventV2 {
  L51: export interface APIGatewayProxyEventV2 {
  L77: export interface APIGatewayProxyEvent {
  L101: export interface ALBProxyEvent {
  L115: type WithHeaders = {
  L127: export type APIGatewayProxyResult = {
  L135: const getRequestContext = (
  L145: async function* readWebStream(
  L155: const streamToNodeStream = (
  L163: export const streamHandle = <
  L220: type HandleOptions = {
  L265: export const handle = <E extends Env = Env, S extends Schema = {}, BasePath extends string = '/'>(
  L307: export abstract class EventProcessor<E extends LambdaEvent> {
  L330: protected getDomainName(event: E): string | undefined {
  L347: createRequest(event: E): Request {
  L373: async createResult(
  L417: setCookies(_event: E, res: Response, result: APIGatewayProxyResult) {
  L436: export class EventV2Processor extends EventProcessor<APIGatewayProxyEventV2> {
  L441: protected getMethod(event: APIGatewayProxyEventV2): string {
  L445: protected getQueryString(event: APIGatewayProxyEventV2): string {
  L449: protected getCookies(event: APIGatewayProxyEventV2, headers: Headers): void {
  L455: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L459: protected getHeaders(event: APIGatewayProxyEventV2): Headers {
  L478: export class EventV1Processor extends EventProcessor<APIGatewayProxyEvent> {
  L483: protected getMethod(event: APIGatewayProxyEvent): string {
  L487: protected getQueryString(event: APIGatewayProxyEvent): string {
  L505: protected getCookies(_event: APIGatewayProxyEvent, _headers: Headers): void {
  L509: protected getHeaders(event: APIGatewayProxyEvent): Headers {
  L529: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L541: export class ALBProcessor extends EventProcessor<ALBProxyEvent> {
  L542: protected getHeaders(event: ALBProxyEvent): Headers {
  L568: protected getMethod(event: ALBProxyEvent): string {
  L572: protected getQueryString(event: ALBProxyEvent): string {
  L598: protected getCookies(event: ALBProxyEvent, headers: Headers): void {
  L610: protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
  L625: export class LatticeV2Processor extends EventProcessor<LatticeProxyEventV2> {
  L630: protected getMethod(event: LatticeProxyEventV2): string {
  L634: protected getQueryString(): string {
