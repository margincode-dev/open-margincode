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
