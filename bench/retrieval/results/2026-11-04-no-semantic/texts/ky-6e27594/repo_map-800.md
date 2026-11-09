source/types/options.ts:
  L16: export type RequestHttpMethod = 'get' | 'post' | 'put' | 'patch' | 'head' | 'delete' | 'query';
  L21: export type Progress = {
  L441: type RequestOptions = {
  L6: export type SearchParamsInit = string | string[][] | Record<string, string> | URLSearchParams | undefined;
  L10: export type SearchParamsOption =
  L17: export type HttpMethod = LiteralUnion<RequestHttpMethod | 'options' | 'trace', string>;
  L19: export type Input = string | URL | Request;
  L39: export type KyHeadersInit = NonNullable<RequestInit['headers']> | Record<string, string | undefined> | ReadonlyArray<readonly [string, string]>;
  L45: export type KyOptions = {
  L439: export type KyOptionsRegistry = {[K in keyof KyOptions]-?: true};
  L448: export interface Options extends KyOptions, RequestOptions { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
  L522: export type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & {
  L531: type NormalizedRetryOptions = {
  L535: export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
  L551: export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
source/types/hooks.ts:
  L32: export type BeforeRequestState = {
  L42: export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;
  L44: export type BeforeRetryState = {
  L55: export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
  L57: export type BeforeErrorState = {
  L73: export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;
  L30: export type InitHook = (options: InitOptions) => void;
  L75: export type AfterResponseState = {
  L88: export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
  L90: export type Hooks = {
  L376: export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
source/core/Ky.ts:
  L1200: get #sentRequest(): Request {
  L1211: async #runBeforeRequestHooks(): Promise<Response | undefined> {
  L1553: #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request {
  L145: const isRequestInstance = (value: unknown): value is Request =>
  L157: const normalizeInputForProtocolCheck = (input: string): string =>
  L1057: #isJsonContentType(contentType: string): boolean {
  L1399: #consumeReturnedResponseFromBeforeRetryHook(): boolean {
  L1510: #assignRequest(request: Request, requestBodyCanBeCancelled = false): void {
  L1534: #getResponseRequest(response: Response): Request {
  L1538: #setResponseRequest(response: Response, request: Request): Response {
  L65: type ErrorDataTimeout = {
