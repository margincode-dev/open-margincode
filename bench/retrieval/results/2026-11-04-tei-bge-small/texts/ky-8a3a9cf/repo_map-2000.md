source/core/Ky.ts:
  L136: const isResponseInstance = (value: unknown): value is Response =>
  L779: #decorateResponse(response: Response): Response {
  L833: async #getResponseData(response: Response): Promise<unknown> {
  L1052: #isJsonContentType(contentType: string): boolean {
  L1074: async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined> {
  L1174: #cancelResponseBody(response: Response): void {
  L1223: async #runAfterResponseHooks(response: Response): Promise<Response> {
  L1388: #consumeReturnedResponseFromBeforeRetryHook(): boolean {
  L1525: #getResponseRequest(response: Response): Request {
  L1529: #setResponseRequest(response: Response, request: Request): Response {
  L1534: #limitResponseSize(response: Response): Response {
  L65: type ErrorDataTimeout = {
  L70: const createTextDecoder = (contentType: string): TextDecoder => {
  L87: const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
  L101: const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number => {
  L130: const isRequestInstance = (value: unknown): value is Request =>
  L139: const isAbsoluteInput = (input: string): boolean =>
  L142: const normalizeInputForProtocolCheck = (input: string): string =>
  L145: const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined => {
  L157: function cloneInitHookOptions(options: Options): InitOptions {
  L222: const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> => {
  L252: export class Ky {
  L253: static create(input: Input, options: Options): ResponsePromise {
  L456: static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption {
  L486: constructor(input: Input, options: Options = {}) {
  L658: #validateCallbackOptions(): void {
  L673: #calculateDelay(retry: InternalOptions['retry']): number {
  L697: async #calculateRetryDelay(error: unknown) {
  L801: async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never> {
  L872: #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
  L892: #getEffectiveTimeout(): number | undefined {
  L909: async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
  L977: async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation> {
  L1144: async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown> {
  L1165: #cancelBody(body: ReadableStream | undefined): void {
  L1179: #cancelReturnedBody(value: unknown): void {
  L1187: #createManagedSignal(): AbortSignal {
  L1193: #throwIfTotalTimeoutExhausted(request: Request = this.request): void {
  L1200: async #runBeforeRequestHooks(): Promise<Response | undefined> {
  L1291: async #retry(): Promise<Response | void> {
  L1299: async #retryFromError(error: unknown): Promise<Response | void> {
  L1394: async #fetch(): Promise<Response> {
  L1461: #throwIfAbortedByUser(): void {
  L1465: #getRemainingTotalTimeout(): number | undefined {
  L1474: #getCurrentTime(): number {
  L1478: #getNormalizedOptions(): NormalizedOptions {
  L1501: #assignRequest(request: Request, requestBodyCanBeCancelled = false): void {
  L1513: #withManagedSignal(request: Request): Request {
  L1539: #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request {
source/errors/KyError.ts:
  L8: export class KyError extends Error {
  L11: get isKyError(): true {
source/types/options.ts:
  L6: export type SearchParamsInit = string | string[][] | Record<string, string> | URLSearchParams | undefined;
  L10: export type SearchParamsOption =
  L16: export type RequestHttpMethod = 'get' | 'post' | 'put' | 'patch' | 'head' | 'delete' | 'query';
  L17: export type HttpMethod = LiteralUnion<RequestHttpMethod | 'options' | 'trace', string>;
  L19: export type Input = string | URL | Request;
  L21: export type Progress = {
  L39: export type KyHeadersInit = NonNullable<RequestInit['headers']> | Record<string, string | undefined> | ReadonlyArray<readonly [string, string]>;
  L45: export type KyOptions = {
  L439: export type KyOptionsRegistry = {[K in keyof KyOptions]-?: true};
  L441: type RequestOptions = {
  L448: export interface Options extends KyOptions, RequestOptions { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
  L504: export type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & {
  L513: type NormalizedRetryOptions = {
  L517: export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
  L533: export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
source/errors/ResponseSizeError.ts:
  L20: export class ResponseSizeError extends KyError {
  L25: constructor(request: Request, maxResponseSize: number) {
source/types/response.ts:
  L1: export type KyResponse<T = unknown> = {
source/utils/body.ts:
  L107: const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response => {
  L173: export const limitResponseSize = (response: Response, request: Request, maxResponseSize: number): Response => {
  L198: export const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) => {
  L14: export const getBodySize = (body?: BodyInit | null): number => {
  L53: const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> => {
  L104: export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
  L216: export const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) => {
source/utils/type-guards.ts:
  L9: const isErrorType = (error: unknown, cls: {name: string}): boolean =>
  L152: export function isResponseSizeError(error: unknown): error is ResponseSizeError {
  L36: export function isKyError(error: unknown): error is KyError {
  L58: export function isHTTPError<T = unknown>(error: unknown): error is HTTPError<T> {
  L80: export function isNetworkError(error: unknown): error is NetworkError {
  L102: export function isTimeoutError(error: unknown): error is TimeoutError {
  L129: export function isForceRetryError(error: unknown): error is ForceRetryError {
source/types/hooks.ts:
  L75: export type AfterResponseState = {
  L88: export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
  L30: export type InitHook = (options: InitOptions) => void;
  L32: export type BeforeRequestState = {
  L42: export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;
  L44: export type BeforeRetryState = {
  L55: export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
  L57: export type BeforeErrorState = {
  L73: export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;
  L90: export type Hooks = {
  L376: export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
source/utils/delay.ts:
  L5: export type DelayOptions = {
  L9: export default async function delay(
