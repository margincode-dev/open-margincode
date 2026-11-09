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
