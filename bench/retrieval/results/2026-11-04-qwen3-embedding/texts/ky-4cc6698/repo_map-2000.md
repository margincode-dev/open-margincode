source/types/options.ts:
  L531: type NormalizedRetryOptions = {
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
  L522: export type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & {
  L535: export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
  L551: export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
source/utils/delay.ts:
  L5: export type DelayOptions = {
  L9: export default async function delay(
source/core/constants.ts:
  L159: get isRetryMarker(): true {
  L73: export type ForceRetryOptions = {
  L157: export class RetryMarker {
  L165: constructor(options?: ForceRetryOptions) {
  L170: export const isRetryMarker = (value: unknown): value is RetryMarker =>
  L256: export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);
  L40: const validate = <T extends Array<true>>() => undefined as unknown as T;
source/types/retry.ts:
  L3: export type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & {
  L9: export type ShouldRetryState = {
  L21: export type RetryOptions = {
source/errors/KyError.ts:
  L11: get isKyError(): true {
  L8: export class KyError extends Error {
source/types/hooks.ts:
  L44: export type BeforeRetryState = {
  L55: export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
  L57: export type BeforeErrorState = {
  L73: export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;
  L32: export type BeforeRequestState = {
  L42: export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;
  L30: export type InitHook = (options: InitOptions) => void;
  L75: export type AfterResponseState = {
  L88: export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
  L90: export type Hooks = {
  L376: export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
source/core/Ky.ts:
  L1332: async #retryFromError(error: unknown): Promise<Response | void> {
  L1421: #consumeReturnedResponseFromBeforeRetryHook(): boolean {
  L65: type ErrorDataTimeout = {
  L116: const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number => {
  L694: async #calculateRetryDelay(error: unknown) {
  L805: async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never> {
  L876: #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
  L1233: async #runBeforeRequestHooks(): Promise<Response | undefined> {
  L1324: async #retry(): Promise<Response | void> {
  L70: const createTextDecoder = (contentType: string): TextDecoder => {
  L87: const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
  L103: const validateCallbackOptions = (options: Record<string, unknown>): void => {
  L145: const isRequestInstance = (value: unknown): value is Request =>
  L151: const isResponseInstance = (value: unknown): value is Response =>
  L154: const isAbsoluteInput = (input: string): boolean =>
  L157: const normalizeInputForProtocolCheck = (input: string): string =>
  L160: const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined => {
  L172: function cloneInitHookOptions(options: Options): InitOptions {
  L237: const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> => {
  L267: export class Ky {
  L268: static create(input: Input, options: Options): ResponsePromise {
  L467: static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption {
  L498: constructor(input: Input, options: Options = {}) {
  L670: #calculateDelay(retry: InternalOptions['retry']): number {
  L776: #decorateResponse(response: Response): Response {
  L837: async #getResponseData(response: Response): Promise<unknown> {
  L896: #getEffectiveTimeout(): number | undefined {
  L913: async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
  L981: async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation> {
  L1067: #isJsonContentType(contentType: string): boolean {
  L1089: async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined> {
  L1159: async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown> {
  L1180: #cancelBody(body: ReadableStream | undefined): void {
  L1189: #cancelResponseBody(response: Response): void {
  L1194: #cancelReturnedBody(value: unknown): void {
  L1202: #createManagedSignal(): AbortSignal {
  L1210: #shouldThrowHttpErrors(response: Response): boolean {
  L1222: get #sentRequest(): Request {
  L1226: #throwIfTotalTimeoutExhausted(request: Request = this.#sentRequest): void {
  L1256: async #runAfterResponseHooks(response: Response): Promise<Response> {
  L1427: async #fetch(): Promise<Response> {
  L1492: #throwIfAbortedByUser(): void {
  L1496: #getRemainingTotalTimeout(): number | undefined {
  L1505: #getCurrentTime(): number {
  L1509: #getNormalizedOptions(): NormalizedOptions {
  L1532: #assignRequest(request: Request, requestBodyCanBeCancelled = false): void {
  L1544: #withManagedSignal(request: Request): Request {
  L1556: #getResponseRequest(response: Response): Request {
  L1560: #setResponseRequest(response: Response, request: Request): Response {
  L1570: #limitResponseSize(response: Response): Response {
  L1575: #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request {
source/utils/type-guards.ts:
  L129: export function isForceRetryError(error: unknown): error is ForceRetryError {
  L9: const isErrorType = (error: unknown, cls: {name: string}): boolean =>
  L36: export function isKyError(error: unknown): error is KyError {
  L58: export function isHTTPError<T = unknown>(error: unknown): error is HTTPError<T> {
  L80: export function isNetworkError(error: unknown): error is NetworkError {
  L102: export function isTimeoutError(error: unknown): error is TimeoutError {
  L152: export function isResponseSizeError(error: unknown): error is ResponseSizeError {
source/errors/TimeoutError.ts:
  L7: export class TimeoutError extends KyError {
  L11: constructor(request: Request) {
source/errors/ForceRetryError.ts:
  L10: export class ForceRetryError extends KyError {
  L16: constructor(options?: ForceRetryOptions) {
