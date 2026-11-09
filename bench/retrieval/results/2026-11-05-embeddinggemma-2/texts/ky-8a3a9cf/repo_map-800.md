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
