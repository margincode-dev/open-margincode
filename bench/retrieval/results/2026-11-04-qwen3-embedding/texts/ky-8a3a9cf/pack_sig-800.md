## Task
# ky-8a3a9cf · ky-8a3a9cf · open
Goal: Advertise the shortcut media type on the attempt that produced the response

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
- sym:42969 AfterResponseState (source/types/hooks.ts:75-86)
- sym:42899 Ky.#getResponseData (source/core/Ky.ts:833-870)
  async #getResponseData(response: Response): Promise<unknown>
- sym:42970 AfterResponseHook (source/types/hooks.ts:88-88)
  type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>
- sym:42913 Ky.#runAfterResponseHooks (source/core/Ky.ts:1223-1289)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:42981 KyOptions (source/types/options.ts:45-432)
- sym:42967 BeforeErrorState (source/types/hooks.ts:57-70)
- sym:43007 streamResponse (source/utils/body.ts:198-213)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:43005 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43103 ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
- sym:42904 Ky.#isJsonContentType (source/core/Ky.ts:1052-1072)
  #isJsonContentType(contentType: string): boolean
- sym:43043 timeout (source/utils/timeout.ts:10-47)
- sym:42993 RetryOptions (source/types/retry.ts:21-187)
- sym:42917 Ky.#fetch (source/core/Ky.ts:1394-1458)
  async #fetch(): Promise<Response>
- sym:42990 KyResponse (source/types/response.ts:1-4)
- sym:42891 Ky.create (source/core/Ky.ts:253-453)
  static create(input: Input, options: Options): ResponsePromise
- sym:42893 Ky.constructor (source/core/Ky.ts:486-654)
- sym:42900 Ky.#getErrorDataTimeout (source/core/Ky.ts:872-890)
- sym:42901 Ky.#getEffectiveTimeout (source/core/Ky.ts:892-905)
- sym:43077 createResponseLike (test/hooks.ts:42-65)

---
tokens: 679/800
