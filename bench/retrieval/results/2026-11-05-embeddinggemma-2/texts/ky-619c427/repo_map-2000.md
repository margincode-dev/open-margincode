source/errors/KyError.ts:
  L11: get isKyError(): true {
  L8: export class KyError extends Error {
source/errors/TimeoutError.ts:
  L7: export class TimeoutError extends KyError {
  L11: constructor(request: Request) {
source/core/Ky.ts:
  L65: type ErrorDataTimeout = {
  L876: #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
  L87: const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
  L130: const isRequestInstance = (value: unknown): value is Request =>
  L805: async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never> {
  L896: #getEffectiveTimeout(): number | undefined {
  L981: async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation> {
  L1197: #throwIfTotalTimeoutExhausted(request: Request = this.request): void {
  L1204: async #runBeforeRequestHooks(): Promise<Response | undefined> {
  L1303: async #retryFromError(error: unknown): Promise<Response | void> {
  L1469: #getRemainingTotalTimeout(): number | undefined {
  L1505: #assignRequest(request: Request, requestBodyCanBeCancelled = false): void {
  L1529: #getResponseRequest(response: Response): Request {
  L1533: #setResponseRequest(response: Response, request: Request): Response {
  L1543: #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request {
  L70: const createTextDecoder = (contentType: string): TextDecoder => {
  L101: const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number => {
  L136: const isResponseInstance = (value: unknown): value is Response =>
  L139: const isAbsoluteInput = (input: string): boolean =>
  L142: const normalizeInputForProtocolCheck = (input: string): string =>
  L145: const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined => {
  L157: function cloneInitHookOptions(options: Options): InitOptions {
  L222: const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> => {
  L252: export class Ky {
  L253: static create(input: Input, options: Options): ResponsePromise {
  L460: static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption {
  L490: constructor(input: Input, options: Options = {}) {
  L662: #validateCallbackOptions(): void {
  L677: #calculateDelay(retry: InternalOptions['retry']): number {
  L701: async #calculateRetryDelay(error: unknown) {
  L783: #decorateResponse(response: Response): Response {
  L837: async #getResponseData(response: Response): Promise<unknown> {
  L913: async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
  L1056: #isJsonContentType(contentType: string): boolean {
  L1078: async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined> {
  L1148: async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown> {
  L1169: #cancelBody(body: ReadableStream | undefined): void {
  L1178: #cancelResponseBody(response: Response): void {
  L1183: #cancelReturnedBody(value: unknown): void {
  L1191: #createManagedSignal(): AbortSignal {
  L1227: async #runAfterResponseHooks(response: Response): Promise<Response> {
  L1295: async #retry(): Promise<Response | void> {
  L1392: #consumeReturnedResponseFromBeforeRetryHook(): boolean {
  L1398: async #fetch(): Promise<Response> {
  L1465: #throwIfAbortedByUser(): void {
  L1478: #getCurrentTime(): number {
  L1482: #getNormalizedOptions(): NormalizedOptions {
  L1517: #withManagedSignal(request: Request): Request {
  L1538: #limitResponseSize(response: Response): Response {
source/utils/timeout.ts:
  L4: export type TimeoutOptions = {
  L10: export default async function timeout(
source/types/options.ts:
  L16: export type RequestHttpMethod = 'get' | 'post' | 'put' | 'patch' | 'head' | 'delete' | 'query';
  L441: type RequestOptions = {
  L6: export type SearchParamsInit = string | string[][] | Record<string, string> | URLSearchParams | undefined;
  L10: export type SearchParamsOption =
  L17: export type HttpMethod = LiteralUnion<RequestHttpMethod | 'options' | 'trace', string>;
  L19: export type Input = string | URL | Request;
  L21: export type Progress = {
  L39: export type KyHeadersInit = NonNullable<RequestInit['headers']> | Record<string, string | undefined> | ReadonlyArray<readonly [string, string]>;
  L45: export type KyOptions = {
  L439: export type KyOptionsRegistry = {[K in keyof KyOptions]-?: true};
  L448: export interface Options extends KyOptions, RequestOptions { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
  L504: export type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & {
  L513: type NormalizedRetryOptions = {
  L517: export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
  L533: export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
source/utils/type-guards.ts:
  L102: export function isTimeoutError(error: unknown): error is TimeoutError {
  L9: const isErrorType = (error: unknown, cls: {name: string}): boolean =>
  L36: export function isKyError(error: unknown): error is KyError {
  L58: export function isHTTPError<T = unknown>(error: unknown): error is HTTPError<T> {
  L80: export function isNetworkError(error: unknown): error is NetworkError {
  L129: export function isForceRetryError(error: unknown): error is ForceRetryError {
  L152: export function isResponseSizeError(error: unknown): error is ResponseSizeError {
source/errors/NetworkError.ts:
  L11: export class NetworkError extends KyError {
  L15: constructor(request: Request, options?: {cause?: Error | undefined}) {
source/types/request.ts:
  L1: export type KyRequest<T = unknown> = {
source/errors/ResponseSizeError.ts:
  L20: export class ResponseSizeError extends KyError {
  L25: constructor(request: Request, maxResponseSize: number) {
source/types/hooks.ts:
  L32: export type BeforeRequestState = {
  L42: export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;
  L57: export type BeforeErrorState = {
  L73: export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;
  L30: export type InitHook = (options: InitOptions) => void;
  L44: export type BeforeRetryState = {
  L55: export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
  L75: export type AfterResponseState = {
  L88: export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
  L90: export type Hooks = {
  L376: export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
source/errors/NonError.ts:
  L6: export class NonError extends Error {
  L10: constructor(value: unknown) {
source/errors/ForceRetryError.ts:
  L10: export class ForceRetryError extends KyError {
  L16: constructor(options?: ForceRetryOptions) {
source/errors/HTTPError.ts:
  L15: export class HTTPError<T = unknown> extends KyError {
  L22: constructor(response: Response, request: Request, options: Readonly<NormalizedOptions>) {
source/utils/is.ts:
  L7: export const isError = (value: unknown): value is Error =>
  L2: export const isObject = (value: unknown): value is object => value !== null && typeof value === 'object';
