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
source/core/constants.ts:
  L40: const validate = <T extends Array<true>>() => undefined as unknown as T;
  L73: export type ForceRetryOptions = {
  L157: export class RetryMarker {
  L159: get isRetryMarker(): true {
  L165: constructor(options?: ForceRetryOptions) {
  L170: export const isRetryMarker = (value: unknown): value is RetryMarker =>
  L256: export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);
source/types/ResponsePromise.ts:
  L7: export type ResponsePromise<T = unknown> = {
source/errors/TimeoutError.ts:
  L7: export class TimeoutError extends KyError {
  L11: constructor(request: Request) {
source/types/retry.ts:
  L3: export type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & {
  L9: export type ShouldRetryState = {
  L21: export type RetryOptions = {
source/utils/timeout.ts:
  L4: export type TimeoutOptions = {
  L10: export default async function timeout(
test/hooks.ts:
  L42: const createResponseLike = (response: Response): any => ({
  L17: const withHeader = (request: Request, name: string, value: string) => {
  L23: const createRequestLike = (request: Request): any => ({
  L93: const createStreamBody = (text: string) => new ReadableStream<Uint8Array>({
  L100: const createStreamFetch = ({
test/http-error.ts:
  L7: function createFakeResponse({status, statusText}: {status?: number; statusText?: string}): Response {
test/http-response-bodies.ts:
  L9: const byteStream = (bytes: Uint8Array) => {
source/types/common.ts:
  L2: export type Primitive = null | undefined | string | number | boolean | symbol | bigint;
  L4: export type LiteralUnion<LiteralType extends BaseType, BaseType extends Primitive> =
source/utils/types.ts:
  L1: export type Mutable<T> = {
  L5: export type ObjectEntries<T> = T extends ArrayLike<infer U>
source/types/request.ts:
  L1: export type KyRequest<T = unknown> = {
source/types/standard-schema.ts:
  L1: export type StandardSchemaV1Issue = {
  L6: export type StandardSchemaV1SuccessResult<OutputType> = {
  L11: export type StandardSchemaV1FailureResult = {
  L16: export type StandardSchemaV1Result<OutputType> = StandardSchemaV1SuccessResult<OutputType> | StandardSchemaV1FailureResult;
  L18: export type StandardSchemaV1Types<InputType, OutputType> = {
  L23: export type StandardSchemaV1Options = {
  L27: export type StandardSchemaV1<InputType = unknown, OutputType = InputType> = {
  L39: export type StandardSchemaV1InferOutput<Schema extends StandardSchemaV1> = Schema['~standard'] extends {
source/utils/is.ts:
  L2: export const isObject = (value: unknown): value is object => value !== null && typeof value === 'object';
  L7: export const isError = (value: unknown): value is Error =>
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
  L226: const resolveArrayMarkers = (value: unknown[], resolveItem: (item: unknown) => unknown): unknown[] | undefined => {
  L239: const resolveObjectMarkers = (value: Record<string, unknown>, resolveItem: (item: unknown) => unknown): Record<string, unknown> | undefined => {
  L257: const resolveReplaceMarkers = (value: unknown, seen: Set<Record<string, unknown> | unknown[]>): unknown => {
  L270: const appendSearchParameters = (target: any, source: any): URLSearchParams => {
  L327: const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T => {
  L464: export const deepMerge = <T>(...sources: Array<Partial<T> | undefined>): T =>
source/utils/normalize.ts:
  L5: export const normalizeRequestMethod = (input: string): string =>
  L13: export const normalizeRetryMethod = (method: string): string =>
  L23: type InternalRetryOptions = InternalOptions['retry'];
  L40: const normalizeRetryLimit = (retryLimit: unknown): number => {
  L52: export const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions => {
source/errors/NetworkError.ts:
  L11: export class NetworkError extends KyError {
  L15: constructor(request: Request, options?: {cause?: Error | undefined}) {
source/errors/ForceRetryError.ts:
  L10: export class ForceRetryError extends KyError {
  L16: constructor(options?: ForceRetryOptions) {
source/errors/HTTPError.ts:
  L15: export class HTTPError<T = unknown> extends KyError {
  L22: constructor(response: Response, request: Request, options: Readonly<NormalizedOptions>) {
source/errors/NonError.ts:
  L6: export class NonError extends Error {
  L10: constructor(value: unknown) {
source/core/retry-timing.ts:
  L11: type RetryTimingHeader = {
  L16: type RetryTimingDateParts = {
  L25: export const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined => {
  L51: const createTimestamp = ({year, month, day, hours, minutes, seconds}: RetryTimingDateParts): number | undefined => {
  L86: const getCapture = (match: RegExpExecArray, index: number): string => match[index]!;
  L88: const parseDate = (value: string): number | undefined => {
  L151: export const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined => {
source/utils/is-network-error.ts:
  L15: export default function isRawNetworkError(error: unknown): error is TypeError {
source/utils/options.ts:
  L5: export const findUnknownOptions = (
  L31: export const hasSearchParameters = (search: SearchParamsOption): boolean => {
source/errors/SchemaValidationError.ts:
  L25: export class SchemaValidationError extends Error {
  L29: constructor(issues: readonly StandardSchemaV1Issue[]) {
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
test-d/normalized-options.ts:
  L22: type ExpectedNormalizedRetryOptions = {
  L34: type ExpectedProgressCallbacks = {
test-d/retry.ts:
  L17: const initHook: InitHook = options => {
test/browser.ts:
  L22: const DIST_DIR = new URL('../distribution', import.meta.url).toString();
  L23: const createEsmTestServer = async (options?: HttpServerOptions) => {
  L33: const KY_SCRIPT = {
