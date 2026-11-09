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
source/errors/NetworkError.ts:
  L11: export class NetworkError extends KyError {
  L15: constructor(request: Request, options?: {cause?: Error | undefined}) {
source/errors/NonError.ts:
  L6: export class NonError extends Error {
  L10: constructor(value: unknown) {
source/core/retry-timing.ts:
  L11: type RetryTimingHeader = {
  L16: type RetryTimingDateParts = {
  L25: export const getRetryTimingHeader = (headers: Headers): RetryTimingHeader | undefined => {
  L151: export const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined => {
  L51: const createTimestamp = ({year, month, day, hours, minutes, seconds}: RetryTimingDateParts): number | undefined => {
  L86: const getCapture = (match: RegExpExecArray, index: number): string => match[index]!;
  L88: const parseDate = (value: string): number | undefined => {
source/errors/ResponseSizeError.ts:
  L20: export class ResponseSizeError extends KyError {
  L25: constructor(request: Request, maxResponseSize: number) {
source/utils/normalize.ts:
  L13: export const normalizeRetryMethod = (method: string): string =>
  L23: type InternalRetryOptions = InternalOptions['retry'];
  L40: const normalizeRetryLimit = (retryLimit: unknown): number => {
  L52: export const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions => {
  L5: export const normalizeRequestMethod = (input: string): string =>
source/errors/HTTPError.ts:
  L15: export class HTTPError<T = unknown> extends KyError {
  L22: constructor(response: Response, request: Request, options: Readonly<NormalizedOptions>) {
source/utils/is.ts:
  L7: export const isError = (value: unknown): value is Error =>
  L2: export const isObject = (value: unknown): value is object => value !== null && typeof value === 'object';
source/errors/SchemaValidationError.ts:
  L25: export class SchemaValidationError extends Error {
  L29: constructor(issues: readonly StandardSchemaV1Issue[]) {
test-d/network-error-cause.ts:
  L8: const createNetworkError = (cause?: Error) => new NetworkError(request, {cause});
source/types/standard-schema.ts:
  L1: export type StandardSchemaV1Issue = {
  L6: export type StandardSchemaV1SuccessResult<OutputType> = {
  L11: export type StandardSchemaV1FailureResult = {
  L16: export type StandardSchemaV1Result<OutputType> = StandardSchemaV1SuccessResult<OutputType> | StandardSchemaV1FailureResult;
  L18: export type StandardSchemaV1Types<InputType, OutputType> = {
  L23: export type StandardSchemaV1Options = {
  L27: export type StandardSchemaV1<InputType = unknown, OutputType = InputType> = {
  L39: export type StandardSchemaV1InferOutput<Schema extends StandardSchemaV1> = Schema['~standard'] extends {
source/types/request.ts:
  L1: export type KyRequest<T = unknown> = {
source/utils/is-network-error.ts:
  L15: export default function isRawNetworkError(error: unknown): error is TypeError {
test-d/hooks.ts:
  L5: const beforeErrorHook: BeforeErrorHook = ({error}) => error;
  L18: const initHook: InitHook = options => {
test/retry.ts:
  L169: const createSingleRetryHeaderServer = async (t: ExecutionContext, headers: Record<string, string | number>) => {
  L130: const formatHttpTime = (date: Date) => [
  L136: const formatAsctimeDate = (date: Date) => {
  L142: const formatRfc850Date = (date: Date) => {
  L149: const withCapturedTimeouts = async (body: (scheduledDelays: number[]) => Promise<void>) => {
  L3888: const createCutConnectionServer = async (t: ExecutionContext) => {
  L4019: const createAbortDuringBodyFetch = (): typeof fetch => async input => {
  L4040: const createAbortDuringFetch = (onRequest?: () => void): typeof fetch => async input => {
source/utils/timeout.ts:
  L4: export type TimeoutOptions = {
  L10: export default async function timeout(
test/helpers/create-http-test-server.ts:
  L7: export type HttpServerOptions = {
  L11: export type ExtendedHttpTestServer = {
  L21: export async function createHttpTestServer(
source/types/common.ts:
  L2: export type Primitive = null | undefined | string | number | boolean | symbol | bigint;
  L4: export type LiteralUnion<LiteralType extends BaseType, BaseType extends Primitive> =
source/utils/body.ts:
  L104: export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
  L14: export const getBodySize = (body?: BodyInit | null): number => {
  L53: const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> => {
  L107: const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response => {
  L172: export const limitResponseSize = (response: Response, request: Request, maxResponseSize: number): Response => {
  L197: export const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) => {
  L215: export const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) => {
test-d/normalized-options.ts:
  L22: type ExpectedNormalizedRetryOptions = {
  L34: type ExpectedProgressCallbacks = {
test-d/retry.ts:
  L17: const initHook: InitHook = options => {
test/http-error.ts:
  L7: function createFakeResponse({status, statusText}: {status?: number; statusText?: string}): Response {
source/utils/types.ts:
  L1: export type Mutable<T> = {
  L5: export type ObjectEntries<T> = T extends ArrayLike<infer U>
source/types/response.ts:
  L1: export type KyResponse<T = unknown> = {
source/utils/merge.ts:
  L9: type ReplaceMarked<T> = {
  L15: type ReplaceState<T> = {
  L20: const getReplaceState = <T>(value: T): ReplaceState<T> =>
  L53: export const replaceOption = <T>(value: T): T => {
  L63: export const validateAndMerge = (...sources: Array<Partial<Options> | undefined>): Partial<Options> => {
  L73: export const mergeHeaders = (source1: KyHeadersInit = {}, source2: KyHeadersInit = {}) => {
  L88: const isPlainObject = (value: unknown): value is Record<string, unknown> => {
  L98: const isMergeable = (value: unknown): value is Record<string, unknown> | unknown[] => isPlainObject(value) || Array.isArray(value);
  L100: export const cloneShallow = <T>(value: T): T => {
  L125: export const cloneDeep = <T>(value: T, seen: WeakMap<Record<string, unknown> | unknown[], unknown> = new WeakMap()): T => {
  L167: const mergeHeaderObjects = (source1: Record<string, unknown>, source2: Record<string, unknown>): Record<string, string | undefined> => {
  L178: const toHeaderObject = (source: KyHeadersInit): Record<string, string | undefined> => {
  L194: export const mergeHeaderContainers = (source1: KyHeadersInit, source2: KyHeadersInit): Record<string, string | undefined> =>
  L197: function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K] {
  L217: export const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks => {
  L235: const resolveArrayMarkers = (value: unknown[], resolveItem: (item: unknown) => unknown): unknown[] | undefined => {
  L248: const resolveObjectMarkers = (value: Record<string, unknown>, resolveItem: (item: unknown) => unknown): Record<string, unknown> | undefined => {
  L265: const resolveReplaceMarkers = (value: unknown, seen: Set<Record<string, unknown> | unknown[]>): unknown => {
  L278: const appendSearchParameters = (target: any, source: any): URLSearchParams => {
  L335: const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T => {
  L480: export const deepMerge = <T>(...sources: Array<Partial<T> | undefined>): T =>
source/utils/options.ts:
  L5: export const findUnknownOptions = (
  L31: export const hasSearchParameters = (search: SearchParamsOption): boolean => {
source/types/ResponsePromise.ts:
  L7: export type ResponsePromise<T = unknown> = {
source/index.ts:
  L10: const createInstance = (defaults?: Partial<Options>): KyInstance => {
source/types/ky.ts:
  L5: export type KyInstance = {
test-d/init-search-params.ts:
  L4: const initHook: InitHook = options => {
test/browser.ts:
  L22: const DIST_DIR = new URL('../distribution', import.meta.url).toString();
  L23: const createEsmTestServer = async (options?: HttpServerOptions) => {
  L33: const KY_SCRIPT = {
  L40: const addKyScriptToPage = async (page: Page) => {
test/headers.ts:
  L11: const echoHeaders: RequestHandler = (request, response) => {
test/helpers/create-large-file.ts:
  L2: export function createLargeBlob(sizeInMB: number): Blob {
test/helpers/parse-body.ts:
  L4: export const parseJsonBody = async <T = any>(request: Request): Promise<T> => {
  L9: export const parseRawBody = async (request: Request): Promise<string> => text(request);
test/helpers/with-page.ts:
  L11: type Run = (t: ExecutionContext, page: Page) => Promise<void>;
  L13: const PWDEBUG = Boolean(process.env.PWDEBUG);
  L14: const DEFAULT_BROWSERS = [chromium, firefox, webkit];
  L15: const BROWSER_TEST_TIMEOUT_MS = 20_000;
  L16: const BROWSER_CLEANUP_TIMEOUT_MS = process.env.CI ? 20_000 : 5000;
  L17: const BROWSER_TOTAL_TIMEOUT_MS = BROWSER_TEST_TIMEOUT_MS + BROWSER_CLEANUP_TIMEOUT_MS;
  L19: export const promiseWithTimeout = async <Value>(
  L39: export const browserTest = (title: string, browserTypes: BrowserType[], run: Run) => {
  L80: export const defaultBrowsersTest = (title: string, run: Run) => {
test/helpers/with-performance.ts:
  L5: type Argument = {
  L16: export async function withPerformance({
test/hooks.ts:
  L17: const withHeader = (request: Request, name: string, value: string) => {
  L23: const createRequestLike = (request: Request): any => ({
  L42: const createResponseLike = (response: Response): any => ({
  L93: const createStreamBody = (text: string) => new ReadableStream<Uint8Array>({
  L100: const createStreamFetch = ({
test/http-response-bodies.ts:
  L9: const byteStream = (bytes: Uint8Array) => {
test/main.ts:
  L45: type TestSchemaResult<Output> = {value: Output} | {issues: Array<{message: string}>};
  L47: const createSchema = <Output>(
  L57: const isObjectWithValue = (value: unknown): value is {value: unknown} => (
  L63: const createSchemaCallTracker = () => {
test/request-body-serialization.ts:
  L6: type ReceivedBody = {
  L12: async function createBodyServer(t: ExecutionContext) {
  L24: function assertBytes(t: ExecutionContext, received: ReceivedBody, expected: Uint8Array) {
test/stream.ts:
  L1185: const createUploadProgressTestServer = async (t: ExecutionContext) => {
test/validation.ts:
  L4: const okFetch = async () => new Response('ok');
