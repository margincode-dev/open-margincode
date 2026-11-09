source/types/options.ts:
  L513: type NormalizedRetryOptions = {
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
  L517: export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
  L533: export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
source/utils/delay.ts:
  L5: export type DelayOptions = {
  L9: export default async function delay(
source/core/constants.ts:
  L157: export class RetryMarker {
  L160: constructor(options?: ForceRetryOptions) {
  L73: export type ForceRetryOptions = {
  L248: export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);
  L40: const validate = <T extends Array<true>>() => undefined as unknown as T;
source/types/retry.ts:
  L3: export type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & {
  L9: export type ShouldRetryState = {
  L21: export type RetryOptions = {
source/core/retry-timing.ts:
  L11: type RetryTimingHeader = {
  L16: type RetryTimingDateParts = {
  L25: export const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined => {
  L151: export const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined => {
  L51: const createTimestamp = ({year, month, day, hours, minutes, seconds}: RetryTimingDateParts): number | undefined => {
  L86: const getCapture = (match: RegExpExecArray, index: number): string => match[index]!;
  L88: const parseDate = (value: string): number | undefined => {
source/core/Ky.ts:
  L101: const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number => {
  L678: async #calculateRetryDelay(error: unknown) {
  L1272: async #retry(): Promise<Response | void> {
  L1280: async #retryFromError(error: unknown): Promise<Response | void> {
  L1369: #consumeReturnedResponseFromBeforeRetryHook(): boolean {
  L65: type ErrorDataTimeout = {
  L70: const createTextDecoder = (contentType: string): TextDecoder => {
  L87: const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
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
  L485: constructor(input: Input, options: Options = {}) {
  L654: #calculateDelay(retry: InternalOptions['retry']): number {
  L760: #decorateResponse(response: Response): Response {
  L782: async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never> {
  L814: async #getResponseData(response: Response): Promise<unknown> {
  L853: #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
  L873: #getEffectiveTimeout(): number | undefined {
  L890: async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
  L958: async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation> {
  L1033: #isJsonContentType(contentType: string): boolean {
  L1055: async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined> {
  L1125: async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown> {
  L1146: #cancelBody(body: ReadableStream | undefined): void {
  L1155: #cancelResponseBody(response: Response): void {
  L1160: #cancelReturnedBody(value: unknown): void {
  L1168: #createManagedSignal(): AbortSignal {
  L1174: #throwIfTotalTimeoutExhausted(request: Request = this.request): void {
  L1181: async #runBeforeRequestHooks(): Promise<Response | undefined> {
  L1204: async #runAfterResponseHooks(response: Response): Promise<Response> {
  L1375: async #fetch(): Promise<Response> {
  L1442: #throwIfAbortedByUser(): void {
  L1446: #getRemainingTotalTimeout(): number | undefined {
  L1455: #getCurrentTime(): number {
  L1459: #getNormalizedOptions(): NormalizedOptions {
  L1482: #assignRequest(request: Request, requestBodyCanBeCancelled = false): void {
  L1494: #withManagedSignal(request: Request): Request {
  L1506: #getResponseRequest(response: Response): Request {
  L1510: #setResponseRequest(response: Response, request: Request): Response {
  L1515: #limitResponseSize(response: Response): Response {
  L1520: #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request {
source/utils/normalize.ts:
  L13: export const normalizeRetryMethod = (method: string): string =>
  L23: type InternalRetryOptions = InternalOptions['retry'];
  L40: const normalizeRetryLimit = (retryLimit: unknown): number => {
  L52: export const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions => {
  L5: export const normalizeRequestMethod = (input: string): string =>
source/types/hooks.ts:
  L44: export type BeforeRetryState = {
  L55: export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
  L30: export type InitHook = (options: InitOptions) => void;
  L32: export type BeforeRequestState = {
  L42: export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;
  L57: export type BeforeErrorState = {
  L73: export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;
  L75: export type AfterResponseState = {
  L88: export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
  L90: export type Hooks = {
  L376: export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
source/errors/ForceRetryError.ts:
  L10: export class ForceRetryError extends KyError {
  L16: constructor(options?: ForceRetryOptions) {
test/retry.ts:
  L169: const createSingleRetryHeaderServer = async (t: ExecutionContext, headers: Record<string, string | number>) => {
  L130: const formatHttpTime = (date: Date) => [
  L136: const formatAsctimeDate = (date: Date) => {
  L142: const formatRfc850Date = (date: Date) => {
  L149: const withCapturedTimeouts = async (body: (scheduledDelays: number[]) => Promise<void>) => {
  L3888: const createCutConnectionServer = async (t: ExecutionContext) => {
  L4019: const createAbortDuringBodyFetch = (): typeof fetch => async input => {
  L4040: const createAbortDuringFetch = (onRequest?: () => void): typeof fetch => async input => {
test/helpers/create-http-test-server.ts:
  L7: export type HttpServerOptions = {
  L11: export type ExtendedHttpTestServer = {
  L21: export async function createHttpTestServer(
source/utils/type-guards.ts:
  L129: export function isForceRetryError(error: unknown): error is ForceRetryError {
  L9: const isErrorType = (error: unknown, cls: {name: string}): boolean =>
  L36: export function isKyError(error: unknown): error is KyError {
  L58: export function isHTTPError<T = unknown>(error: unknown): error is HTTPError<T> {
  L80: export function isNetworkError(error: unknown): error is NetworkError {
  L102: export function isTimeoutError(error: unknown): error is TimeoutError {
  L152: export function isResponseSizeError(error: unknown): error is ResponseSizeError {
source/errors/KyError.ts:
  L8: export class KyError extends Error {
  L11: get isKyError(): true {
source/utils/timeout.ts:
  L4: export type TimeoutOptions = {
  L10: export default async function timeout(
source/types/common.ts:
  L2: export type Primitive = null | undefined | string | number | boolean | symbol | bigint;
  L4: export type LiteralUnion<LiteralType extends BaseType, BaseType extends Primitive> =
source/utils/body.ts:
  L107: const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response => {
  L14: export const getBodySize = (body?: BodyInit | null): number => {
  L53: const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> => {
  L104: export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
  L173: export const limitResponseSize = (response: Response, request: Request, maxResponseSize: number): Response => {
  L198: export const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) => {
  L226: export const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) => {
test-d/normalized-options.ts:
  L22: type ExpectedNormalizedRetryOptions = {
  L34: type ExpectedProgressCallbacks = {
test-d/retry.ts:
  L17: const initHook: InitHook = options => {
source/errors/TimeoutError.ts:
  L7: export class TimeoutError extends KyError {
  L11: constructor(request: Request) {
source/types/request.ts:
  L1: export type KyRequest<T = unknown> = {
source/errors/NonError.ts:
  L6: export class NonError extends Error {
  L10: constructor(value: unknown) {
source/errors/ResponseSizeError.ts:
  L20: export class ResponseSizeError extends KyError {
  L25: constructor(request: Request, maxResponseSize: number) {
source/types/response.ts:
  L1: export type KyResponse<T = unknown> = {
source/errors/NetworkError.ts:
  L11: export class NetworkError extends KyError {
  L15: constructor(request: Request, options?: {cause?: Error | undefined}) {
source/utils/is.ts:
  L2: export const isObject = (value: unknown): value is object => value !== null && typeof value === 'object';
  L7: export const isError = (value: unknown): value is Error =>
source/errors/HTTPError.ts:
  L15: export class HTTPError<T = unknown> extends KyError {
  L22: constructor(response: Response, request: Request, options: Readonly<NormalizedOptions>) {
source/utils/merge.ts:
  L8: type ReplaceMarked<T> = {
  L13: type ReplaceState<T> = {
  L18: const getReplaceState = <T>(value: T): ReplaceState<T> =>
  L49: export const replaceOption = <T>(value: T): T => {
  L54: export const validateAndMerge = (...sources: Array<Partial<Options> | undefined>): Partial<Options> => {
  L64: export const mergeHeaders = (source1: KyHeadersInit = {}, source2: KyHeadersInit = {}) => {
  L79: const isPlainObject = (value: unknown): value is Record<string, unknown> => {
  L89: const isMergeable = (value: unknown): value is Record<string, unknown> | unknown[] => isPlainObject(value) || Array.isArray(value);
  L91: export const cloneShallow = <T>(value: T): T => {
  L116: export const cloneDeep = <T>(value: T, seen: WeakMap<Record<string, unknown> | unknown[], unknown> = new WeakMap()): T => {
  L158: const mergeHeaderObjects = (source1: Record<string, unknown>, source2: Record<string, unknown>): Record<string, string | undefined> => {
  L169: const toHeaderObject = (source: KyHeadersInit): Record<string, string | undefined> => {
  L185: export const mergeHeaderContainers = (source1: KyHeadersInit, source2: KyHeadersInit): Record<string, string | undefined> =>
  L188: function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K] {
  L208: export const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks => {
  L230: const resolveReplaceMarkers = (value: unknown, seen: Set<Record<string, unknown> | unknown[]>): unknown => {
  L263: const appendSearchParameters = (target: any, source: any): URLSearchParams => {
  L320: const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T => {
  L457: export const deepMerge = <T>(...sources: Array<Partial<T> | undefined>): T =>
source/types/standard-schema.ts:
  L1: export type StandardSchemaV1Issue = {
  L6: export type StandardSchemaV1SuccessResult<OutputType> = {
  L11: export type StandardSchemaV1FailureResult = {
  L16: export type StandardSchemaV1Result<OutputType> = StandardSchemaV1SuccessResult<OutputType> | StandardSchemaV1FailureResult;
  L18: export type StandardSchemaV1Types<InputType, OutputType> = {
  L23: export type StandardSchemaV1Options = {
  L27: export type StandardSchemaV1<InputType = unknown, OutputType = InputType> = {
  L39: export type StandardSchemaV1InferOutput<Schema extends StandardSchemaV1> = Schema['~standard'] extends {
source/utils/is-network-error.ts:
  L15: export default function isRawNetworkError(error: unknown): error is TypeError {
source/utils/options.ts:
  L5: export const findUnknownOptions = (
  L31: export const hasSearchParameters = (search: SearchParamsOption): boolean => {
source/errors/SchemaValidationError.ts:
  L25: export class SchemaValidationError extends Error {
  L29: constructor(issues: readonly StandardSchemaV1Issue[]) {
source/types/ResponsePromise.ts:
  L7: export type ResponsePromise<T = unknown> = {
source/utils/types.ts:
  L1: export type Mutable<T> = {
  L5: export type ObjectEntries<T> = T extends ArrayLike<infer U>
source/index.ts:
  L10: const createInstance = (defaults?: Partial<Options>): KyInstance => {
source/types/ky.ts:
  L5: export type KyInstance = {
test-d/hooks.ts:
  L5: const beforeErrorHook: BeforeErrorHook = ({error}) => error;
  L18: const initHook: InitHook = options => {
test-d/init-search-params.ts:
  L4: const initHook: InitHook = options => {
test-d/network-error-cause.ts:
  L8: const createNetworkError = (cause?: Error) => new NetworkError(request, {cause});
test/browser.ts:
  L22: const DIST_DIR = new URL('../distribution', import.meta.url).toString();
  L23: const createEsmTestServer = async (options?: HttpServerOptions) => {
  L33: const KY_SCRIPT = {
  L40: const addKyScriptToPage = async (page: Page) => {
test/headers.ts:
  L11: const echoHeaders: RequestHandler = (request, response) => {
