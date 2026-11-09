source/types/hooks.ts:
  L30: export type InitHook = (options: InitOptions) => void;
  L42: export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;
  L55: export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
  L73: export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;
  L88: export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
  L32: export type BeforeRequestState = {
  L44: export type BeforeRetryState = {
  L57: export type BeforeErrorState = {
  L75: export type AfterResponseState = {
  L90: export type Hooks = {
  L376: export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
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
  L522: export type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & {
  L531: type NormalizedRetryOptions = {
  L535: export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
  L551: export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
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
source/types/request.ts:
  L1: export type KyRequest<T = unknown> = {
source/core/Ky.ts:
  L160: const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined => {
  L172: function cloneInitHookOptions(options: Options): InitOptions {
  L1424: #consumeReturnedResponseFromBeforeRetryHook(): boolean {
  L65: type ErrorDataTimeout = {
  L70: const createTextDecoder = (contentType: string): TextDecoder => {
  L87: const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
  L103: const validateCallbackOptions = (options: Record<string, unknown>): void => {
  L116: const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number => {
  L145: const isRequestInstance = (value: unknown): value is Request =>
  L151: const isResponseInstance = (value: unknown): value is Response =>
  L154: const isAbsoluteInput = (input: string): boolean =>
  L157: const normalizeInputForProtocolCheck = (input: string): string =>
  L237: const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> => {
  L267: export class Ky {
  L268: static create(input: Input, options: Options): ResponsePromise {
  L467: static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption {
  L498: constructor(input: Input, options: Options = {}) {
  L670: #calculateDelay(retry: InternalOptions['retry'], error: unknown): number {
  L696: async #calculateRetryDelay(error: unknown) {
  L778: #decorateResponse(response: Response): Response {
  L807: async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never> {
  L839: async #getResponseData(response: Response): Promise<unknown> {
  L878: #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
  L898: #getEffectiveTimeout(): number | undefined {
  L915: async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
  L983: async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation> {
  L1069: #isJsonContentType(contentType: string): boolean {
  L1091: async #readResponseText(response: Response, timeoutMs: number): Promise<string | typeof timedOutResponseData | undefined> {
  L1161: async #parseJson(text: string, response: Response, timeoutMs: number, request: Request): Promise<unknown> {
  L1182: #cancelBody(body: ReadableStream | undefined): void {
  L1191: #cancelResponseBody(response: Response): void {
  L1196: #cancelReturnedBody(value: unknown): void {
  L1204: #createManagedSignal(): AbortSignal {
  L1212: #shouldThrowHttpErrors(response: Response): boolean {
  L1224: get #sentRequest(): Request {
  L1228: #throwIfTotalTimeoutExhausted(request: Request = this.#sentRequest): void {
  L1235: async #runBeforeRequestHooks(): Promise<Response | undefined> {
  L1258: async #runAfterResponseHooks(response: Response): Promise<Response> {
  L1326: async #retry(): Promise<Response | void> {
  L1334: async #retryFromError(error: unknown): Promise<Response | void> {
  L1430: async #fetch(): Promise<Response> {
  L1495: #throwIfAbortedByUser(): void {
  L1499: #getRemainingTotalTimeout(): number | undefined {
  L1508: #getCurrentTime(): number {
  L1512: #getNormalizedOptions(): NormalizedOptions {
  L1535: #assignRequest(request: Request, requestBodyCanBeCancelled = false): void {
  L1547: #withManagedSignal(request: Request): Request {
  L1559: #getResponseRequest(response: Response): Request {
  L1563: #setResponseRequest(response: Response, request: Request): Response {
  L1573: #limitResponseSize(response: Response): Response {
  L1578: #wrapRequestWithUploadProgress(request: Request, originalBody?: BodyInit): Request {
test-d/hooks.ts:
  L5: const beforeErrorHook: BeforeErrorHook = ({error}) => error;
  L18: const initHook: InitHook = options => {
source/types/retry.ts:
  L3: export type MutableRetryOptions = Omit<RetryOptions, 'methods' | 'statusCodes' | 'afterStatusCodes'> & {
  L9: export type ShouldRetryState = {
  L21: export type RetryOptions = {
source/utils/timeout.ts:
  L4: export type TimeoutOptions = {
  L10: export default async function timeout(
source/utils/merge.ts:
  L197: function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K] {
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
  L217: export const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks => {
  L235: const resolveArrayMarkers = (value: unknown[], resolveItem: (item: unknown) => unknown): unknown[] | undefined => {
  L248: const resolveObjectMarkers = (value: Record<string, unknown>, resolveItem: (item: unknown) => unknown): Record<string, unknown> | undefined => {
  L265: const resolveReplaceMarkers = (value: unknown, seen: Set<Record<string, unknown> | unknown[]>): unknown => {
  L278: const appendSearchParameters = (target: any, source: any): URLSearchParams => {
  L335: const deepMergeInternal = <T>(isRoot: boolean, ...sources: Array<Partial<T> | undefined>): T => {
  L480: export const deepMerge = <T>(...sources: Array<Partial<T> | undefined>): T =>
source/types/common.ts:
  L2: export type Primitive = null | undefined | string | number | boolean | symbol | bigint;
  L4: export type LiteralUnion<LiteralType extends BaseType, BaseType extends Primitive> =
test-d/init-search-params.ts:
  L4: const initHook: InitHook = options => {
test-d/retry.ts:
  L17: const initHook: InitHook = options => {
source/errors/KyError.ts:
  L8: export class KyError extends Error {
  L11: get isKyError(): true {
source/errors/TimeoutError.ts:
  L7: export class TimeoutError extends KyError {
  L11: constructor(request: Request) {
source/types/response.ts:
  L1: export type KyResponse<T = unknown> = {
source/utils/is.ts:
  L2: export const isObject = (value: unknown): value is object => value !== null && typeof value === 'object';
  L7: export const isError = (value: unknown): value is Error =>
source/utils/type-guards.ts:
  L9: const isErrorType = (error: unknown, cls: {name: string}): boolean =>
  L36: export function isKyError(error: unknown): error is KyError {
  L58: export function isHTTPError<T = unknown>(error: unknown): error is HTTPError<T> {
  L80: export function isNetworkError(error: unknown): error is NetworkError {
  L102: export function isTimeoutError(error: unknown): error is TimeoutError {
  L129: export function isForceRetryError(error: unknown): error is ForceRetryError {
  L152: export function isResponseSizeError(error: unknown): error is ResponseSizeError {
source/utils/normalize.ts:
  L5: export const normalizeRequestMethod = (input: string): string =>
  L13: export const normalizeRetryMethod = (method: string): string =>
  L23: type InternalRetryOptions = InternalOptions['retry'];
  L40: const normalizeRetryLimit = (retryLimit: unknown): number => {
  L52: export const normalizeRetryOptions = (retry: number | RetryOptions = {}): InternalRetryOptions => {
source/utils/body.ts:
  L14: export const getBodySize = (body?: BodyInit | null): number => {
  L53: const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> => {
  L104: export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
  L107: const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response => {
  L172: export const limitResponseSize = (response: Response, request: Request, maxResponseSize: number): Response => {
  L197: export const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) => {
  L215: export const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) => {
source/types/standard-schema.ts:
  L1: export type StandardSchemaV1Issue = {
  L6: export type StandardSchemaV1SuccessResult<OutputType> = {
  L11: export type StandardSchemaV1FailureResult = {
  L16: export type StandardSchemaV1Result<OutputType> = StandardSchemaV1SuccessResult<OutputType> | StandardSchemaV1FailureResult;
  L18: export type StandardSchemaV1Types<InputType, OutputType> = {
  L23: export type StandardSchemaV1Options = {
  L27: export type StandardSchemaV1<InputType = unknown, OutputType = InputType> = {
  L39: export type StandardSchemaV1InferOutput<Schema extends StandardSchemaV1> = Schema['~standard'] extends {
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
  L51: const createTimestamp = ({year, month, day, hours, minutes, seconds}: RetryTimingDateParts): number | undefined => {
  L86: const getCapture = (match: RegExpExecArray, index: number): string => match[index]!;
  L88: const parseDate = (value: string): number | undefined => {
  L151: export const calculateRetryTimingDelay = ({value, allowTimestamp}: RetryTimingHeader): number | undefined => {
source/utils/is-network-error.ts:
  L15: export default function isRawNetworkError(error: unknown): error is TypeError {
source/utils/options.ts:
  L5: export const findUnknownOptions = (
  L31: export const hasSearchParameters = (search: SearchParamsOption): boolean => {
source/errors/ForceRetryError.ts:
  L10: export class ForceRetryError extends KyError {
  L16: constructor(options?: ForceRetryOptions) {
source/errors/HTTPError.ts:
  L15: export class HTTPError<T = unknown> extends KyError {
  L22: constructor(response: Response, request: Request, options: Readonly<NormalizedOptions>) {
source/errors/ResponseSizeError.ts:
  L20: export class ResponseSizeError extends KyError {
  L25: constructor(request: Request, maxResponseSize: number) {
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
test-d/network-error-cause.ts:
  L8: const createNetworkError = (cause?: Error) => new NetworkError(request, {cause});
test-d/normalized-options.ts:
  L22: type ExpectedNormalizedRetryOptions = {
  L34: type ExpectedProgressCallbacks = {
test/browser.ts:
  L22: const DIST_DIR = new URL('../distribution', import.meta.url).toString();
  L23: const createEsmTestServer = async (options?: HttpServerOptions) => {
  L33: const KY_SCRIPT = {
  L40: const addKyScriptToPage = async (page: Page) => {
test/headers.ts:
  L11: const echoHeaders: RequestHandler = (request, response) => {
test/helpers/create-http-test-server.ts:
  L7: export type HttpServerOptions = {
  L11: export type ExtendedHttpTestServer = {
  L21: export async function createHttpTestServer(
test/helpers/create-large-file.ts:
  L2: export function createLargeBlob(sizeInMB: number): Blob {
