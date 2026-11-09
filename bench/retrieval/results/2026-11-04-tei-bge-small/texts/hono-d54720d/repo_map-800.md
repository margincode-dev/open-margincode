src/middleware/serve-static/index.ts:
  L13: export type ServeStaticOptions<E extends Env = Env> = {
  L37: export const serveStatic = <E extends Env = Env>(
  L25: const ENCODINGS = {
  L30: const ENCODINGS_ORDERED_KEYS = Object.keys(ENCODINGS) as (keyof typeof ENCODINGS)[]
  L32: const DEFAULT_DOCUMENT = 'index.html'
src/adapter/cloudflare-workers/serve-static.ts:
  L9: export type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & {
  L24: export const serveStatic = <E extends Env = Env>(
adapters/cloudflare-workers/src/serve-static.ts:
  L6: export type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & {
  L21: export const serveStatic = <E extends Env = Env>(
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
