## Task
# ky-8a3a9cf · ky-8a3a9cf · open
Goal: Advertise the shortcut media type on the attempt that produced the response
Workspace: /root/bench/ws/rebf815-ky-8a3a9cf/ky-8a3a9cf
Changed (0): nothing yet

## Map
- repo map (106 files):
  .editorconfig
  .gitattributes
  .github/ (3 files)
  .gitignore
  .npmrc
  AGENTS.md
  license
  media/ (3 files)
  package.json
  readme.md
  source/ (31 files)
  test-d/ (20 files)
  test/ (38 files)
  tsconfig.dist.json
  tsconfig.json
  tsconfig.test.json

## Code
- sym:42959 ResponsePromise (source/types/ResponsePromise.ts:7-69)
  type ResponsePromise<T = unknown> = { arrayBuffer: () => Promise<ArrayBuffer>; blob: () => Promise<Blob>; formData: () => Promise<FormData>; /** Get the response body as raw bytes. Note: This shortcut is only available when the runtime supports `Response.prototype.bytes()`. */ bytes: () => Promise<Uint8Array<ArrayBuffer>>; // TODO: Use `json<T extends JSONValue>(): Promise<T>;` when it's fixed in TS. // See https://github.com/microsoft/TypeScript/issues/15300 and https://github.com/sindresorhus/ky/pull/80 json: { /** Get the response body as JSON. @example ``` import ky from 'ky'; const json = await ky(…).json(); ``` @example ``` import ky from 'ky'; interface Result { value: number; } const result1 = await ky(…).json<Result>(); // or const result2 = await ky<Result>(…).json(); ``` */ <JsonType = T>(schema?: undefined): Promise<JsonType>; /** Get the response body as JSON and validate it with a Standard Schema. Use a Standard Schema compatible validator (for example, Zod 3.24+). Throws a `SchemaValidationError` when validation fails. @example ``` import ky from 'ky'; import {z} from 'zod'; const userSchema = z.object({name: z.string()}); const user = await ky('/api/user').json(userSchema); ``` */ <Schema extends StandardSchemaV1>(schema: Schema): Promise<StandardSchemaV1InferOutput<Schema>>; }; text: () => Promise<string>; } & Promise<KyResponse<T>>
- sym:42969 AfterResponseState (source/types/hooks.ts:75-86)
  type AfterResponseState = { request: KyRequest; options: Readonly<NormalizedOptions>; response: KyResponse; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry). */ retryCount: number; }
- sym:42899 Ky.#getResponseData (source/core/Ky.ts:833-870)
  async #getResponseData(response: Response): Promise<unknown>
- sym:42970 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:42913 Ky.#runAfterResponseHooks (source/core/Ky.ts:1223-1289)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:42981 KyOptions (source/types/options.ts:45-432)
- sym:42967 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:43007 streamResponse (source/utils/body.ts:198-213)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:43005 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43103 ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
  type ExpectedNormalizedRetryOptions = { limit: number; methods: string[]; statusCodes: number[]; afterStatusCodes: number[]; maxRetryAfter: number; backoffLimit: number; delay: (attemptCount: number) => number; jitter: boolean | ((delay: number) => number) | undefined; retryOnTimeout: boolean; shouldRetry?: RetryOptions['shouldRetry']; }
- sym:42904 Ky.#isJsonContentType (source/core/Ky.ts:1052-1072)
  #isJsonContentType(contentType: string): boolean
- sym:43043 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:42993 RetryOptions (source/types/retry.ts:21-187)
- sym:42917 Ky.#fetch (source/core/Ky.ts:1394-1458)
  async #fetch(): Promise<Response>
- sym:42990 KyResponse (source/types/response.ts:1-4)
  type KyResponse<T = unknown> = { clone: () => KyResponse<T>; json: <J = T>() => Promise<J>; } & Response
- sym:42891 Ky.create (source/core/Ky.ts:253-453)
  static create(input: Input, options: Options): ResponsePromise
- sym:42893 Ky.constructor (source/core/Ky.ts:486-654)
  constructor(input: Input, options: Options = {})
- sym:42900 Ky.#getErrorDataTimeout (source/core/Ky.ts:872-890)
  #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout
- sym:42901 Ky.#getEffectiveTimeout (source/core/Ky.ts:892-905)
  #getEffectiveTimeout(): number | undefined
- sym:43077 createResponseLike (test/hooks.ts:42-65)
  const createResponseLike = (response: Response): any =>

---
tokens: 1462/2000
