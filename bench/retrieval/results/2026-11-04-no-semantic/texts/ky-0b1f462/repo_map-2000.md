source/types/options.ts:
  L21: export type Progress = {
  L6: export type SearchParamsInit = string | string[][] | Record<string, string> | URLSearchParams | undefined;
  L10: export type SearchParamsOption =
  L16: export type RequestHttpMethod = 'get' | 'post' | 'put' | 'patch' | 'head' | 'delete' | 'query';
  L17: export type HttpMethod = LiteralUnion<RequestHttpMethod | 'options' | 'trace', string>;
  L19: export type Input = string | URL | Request;
  L39: export type KyHeadersInit = NonNullable<RequestInit['headers']> | Record<string, string | undefined> | ReadonlyArray<readonly [string, string]>;
  L45: export type KyOptions = {
  L437: export type KyOptionsRegistry = {[K in keyof KyOptions]-?: true};
  L439: type RequestOptions = {
  L446: export interface Options extends KyOptions, RequestOptions { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
  L502: export type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & {
  L511: type NormalizedRetryOptions = {
  L515: export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
  L531: export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
source/utils/body.ts:
  L104: export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
  L53: const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> => {
  L14: export const getBodySize = (body?: BodyInit | null): number => {
  L107: const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response => {
  L173: export const limitResponseSize = (response: Response, request: Request, maxResponseSize: number): Response => {
  L198: export const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) => {
  L214: export const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) => {
source/utils/delay.ts:
  L5: export type DelayOptions = {
  L9: export default async function delay(
source/core/constants.ts:
  L40: const validate = <T extends Array<true>>() => undefined as unknown as T;
  L73: export type ForceRetryOptions = {
  L157: export class RetryMarker {
  L160: constructor(options?: ForceRetryOptions) {
  L248: export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);
source/types/hooks.ts:
  L30: export type InitHook = (options: InitOptions) => void;
  L32: export type BeforeRequestState = {
  L42: export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;
  L44: export type BeforeRetryState = {
  L55: export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
  L57: export type BeforeErrorState = {
  L73: export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;
  L75: export type AfterResponseState = {
  L88: export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
  L90: export type Hooks = {
  L376: export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
source/core/Ky.ts:
  L1026: #isJsonContentType(contentType: string): boolean {
  L1509: #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request {
  L65: type ErrorDataTimeout = {
  L70: const createTextDecoder = (contentType: string): TextDecoder => {
  L87: const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
  L101: const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number => {
  L130: const isRequestInstance = (value: unknown): value is Request =>
  L136: const isResponseInstance = (value: unknown): value is Response =>
  L139: const isAbsoluteInput = (input: string): boolean =>
  L142: const normalizeInputForProtocolCheck = (input: string): string =>
  L145: const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined => {
  L157: function cloneInitHookOptions(options: Options): InitOptions {
  L222: const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> => {
  L252: export class Ky {
  L253: static create(input: Input, options: Options): ResponsePromise {
  L455: static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption {
  L482: constructor(input: Input, options: Options = {}) {
  L651: #calculateDelay(retry: InternalOptions['retry']): number {
  L675: async #calculateRetryDelay(error: unknown) {
  L753: #decorateResponse(response: Response): Response {
  L775: async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never> {
  L807: async #getResponseData(response: Response): Promise<unknown> {
  L846: #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
  L866: #getEffectiveTimeout(): number | undefined {
  L883: async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
  L951: async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation> {
  L1048: async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined> {
  L1118: async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown> {
  L1139: #cancelBody(body: ReadableStream | undefined): void {
  L1148: #cancelResponseBody(response: Response): void {
  L1153: #cancelReturnedBody(value: unknown): void {
  L1161: #createManagedSignal(): AbortSignal {
  L1167: #throwIfTotalTimeoutExhausted(request: Request = this.request): void {
  L1174: async #runBeforeRequestHooks(): Promise<Response | undefined> {
  L1197: async #runAfterResponseHooks(response: Response): Promise<Response> {
  L1265: async #retry(): Promise<Response | void> {
  L1273: async #retryFromError(error: unknown): Promise<Response | void> {
  L1362: #consumeReturnedResponseFromBeforeRetryHook(): boolean {
  L1368: async #fetch(): Promise<Response> {
  L1431: #throwIfAbortedByUser(): void {
  L1435: #getRemainingTotalTimeout(): number | undefined {
  L1444: #getCurrentTime(): number {
  L1448: #getNormalizedOptions(): NormalizedOptions {
  L1471: #assignRequest(request: Request, requestBodyCanBeCancelled = false): void {
  L1483: #withManagedSignal(request: Request): Request {
  L1495: #getResponseRequest(response: Response): Request {
  L1499: #setResponseRequest(response: Response, request: Request): Response {
  L1504: #limitResponseSize(response: Response): Response {
source/types/retry.ts:
  L3: export type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & {
  L9: export type ShouldRetryState = {
  L21: export type RetryOptions = {
source/utils/timeout.ts:
  L4: export type TimeoutOptions = {
  L10: export default async function timeout(
source/types/common.ts:
  L2: export type Primitive = null | undefined | string | number | boolean | symbol | bigint;
  L4: export type LiteralUnion<LiteralType extends BaseType, BaseType extends Primitive> =
test-d/normalized-options.ts:
  L34: type ExpectedProgressCallbacks = {
  L22: type ExpectedNormalizedRetryOptions = {
