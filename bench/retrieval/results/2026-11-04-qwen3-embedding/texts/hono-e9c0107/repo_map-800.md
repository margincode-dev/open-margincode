src/hono-base.ts:
  L250: basePath<SubPath extends string>(
  L44: type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string
  L46: export type HonoOptions<E extends Env> = {
  L98: class Hono<
  L127: constructor(options: HonoOptions<E> = {}) {
  L178: #clone(): Hono<E, S, BasePath, CurrentPath> {
  L211: route<
  L333: mount(
  L390: #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void {
  L403: #handleError(err: unknown, c: Context<E>): Response | Promise<Response> {
  L410: #dispatch(
  L31: const notFoundHandler: NotFoundHandler = (c) => {
  L35: const errorHandler: ErrorHandler = (err, c) => {
  L89: type MountOptionHandler = (c: Context) => unknown
  L90: type MountReplaceRequest = (originalRequest: Request) => Request
  L91: type MountOptions =
src/request.ts:
  L445: get routePath(): string {
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
  L20: type Body = {
  L27: type BodyCache = Partial<Body>
  L29: type OptionalRequestInitProperties = 'window' | 'priority'
  L30: type RequiredRequestInit = Required<Omit<RequestInit, OptionalRequestInitProperties>> & {
  L481: export const cloneRawRequest = async (req: HonoRequest): Promise<Request> => {
src/client/types.ts:
  L28: type HonoRequest = (typeof Hono.prototype)['request']
  L166: type BuildPath<P extends string, Arg> = `${BuildPathname<P, Arg>}${BuildSearch<Arg, 'query'>}`
  L176: type HonoURL<Prefix extends string, Path extends string, Arg> =
  L294: type PathToChain<
  L11: type MethodNameAll = `$${typeof METHOD_NAME_ALL_LOWERCASE}`
  L17: type StandardMethods = `$${(typeof METHODS)[number]}`
  L24: type ExpandAllMethod<S> = MethodNameAll extends keyof S
  L30: export type BuildSearchParamsFn = (query: Record<string, string | string[]>) => URLSearchParams
