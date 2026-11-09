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
