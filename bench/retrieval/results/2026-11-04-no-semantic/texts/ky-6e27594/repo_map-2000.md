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
  L70: const createTextDecoder = (contentType: string): TextDecoder => {
  L87: const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
  L103: const validateCallbackOptions = (options: Record<string, unknown>): void => {
  L116: const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number => {
  L151: const isResponseInstance = (value: unknown): value is Response =>
  L154: const isAbsoluteInput = (input: string): boolean =>
  L160: const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined => {
  L172: function cloneInitHookOptions(options: Options): InitOptions {
  L237: const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> => {
  L267: export class Ky {
  L268: static create(input: Input, options: Options): ResponsePromise {
  L475: static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption {
  L506: constructor(input: Input, options: Options = {}) {
  L678: #calculateDelay(retry: InternalOptions['retry']): number {
  L702: async #calculateRetryDelay(error: unknown) {
  L784: #decorateResponse(response: Response): Response {
  L806: async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never> {
  L838: async #getResponseData(response: Response): Promise<unknown> {
  L877: #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
  L897: #getEffectiveTimeout(): number | undefined {
  L914: async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
  L982: async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation> {
  L1079: async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined> {
  L1149: async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown> {
  L1170: #cancelBody(body: ReadableStream | undefined): void {
  L1179: #cancelResponseBody(response: Response): void {
  L1184: #cancelReturnedBody(value: unknown): void {
  L1192: #createManagedSignal(): AbortSignal {
  L1204: #throwIfTotalTimeoutExhausted(request: Request = this.#sentRequest): void {
  L1234: async #runAfterResponseHooks(response: Response): Promise<Response> {
  L1302: async #retry(): Promise<Response | void> {
  L1310: async #retryFromError(error: unknown): Promise<Response | void> {
  L1405: async #fetch(): Promise<Response> {
  L1470: #throwIfAbortedByUser(): void {
  L1474: #getRemainingTotalTimeout(): number | undefined {
  L1483: #getCurrentTime(): number {
  L1487: #getNormalizedOptions(): NormalizedOptions {
  L1522: #withManagedSignal(request: Request): Request {
  L1548: #limitResponseSize(response: Response): Response {
source/types/request.ts:
  L1: export type KyRequest<T = unknown> = {
source/utils/delay.ts:
  L5: export type DelayOptions = {
  L9: export default async function delay(
source/core/constants.ts:
  L40: const validate = <T extends Array<true>>() => undefined as unknown as T;
  L73: export type ForceRetryOptions = {
  L157: export class RetryMarker {
  L159: get isRetryMarker(): true {
  L165: constructor(options?: ForceRetryOptions) {
  L170: export const isRetryMarker = (value: unknown): value is RetryMarker =>
  L256: export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);
source/utils/body.ts:
  L53: const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> => {
  L104: export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
  L215: export const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) => {
  L14: export const getBodySize = (body?: BodyInit | null): number => {
  L107: const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response => {
  L172: export const limitResponseSize = (response: Response, request: Request, maxResponseSize: number): Response => {
  L197: export const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) => {
source/types/retry.ts:
  L3: export type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & {
  L9: export type ShouldRetryState = {
  L21: export type RetryOptions = {
source/errors/KyError.ts:
  L8: export class KyError extends Error {
  L11: get isKyError(): true {
