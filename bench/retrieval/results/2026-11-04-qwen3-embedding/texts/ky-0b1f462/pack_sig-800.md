## Task
# ky-0b1f462 · ky-0b1f462 · open
Goal: Progress from a declared content-length, and a throwing callback is never retried

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
- sym:43450 getProgressCallbackError (source/utils/body.ts:104-105)
  const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
- sym:43453 streamResponse (source/utils/body.ts:198-211)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:43350 Ky.#raceBodyRead (source/core/Ky.ts:883-949)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:43449 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:43425 Progress (source/types/options.ts:21-36)
  type Progress = { /** A number between `0` and `1` representing the progress percentage. */ percent: number; /** The number of bytes transferred so far. */ transferredBytes: number; /** The total number of bytes to be transferred. This is an estimate and may be `0` for an empty transfer or when the total size cannot be determined. */ totalBytes: number; }
- sym:43451 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43365 Ky.#fetch (source/core/Ky.ts:1368-1428)
  async #fetch(): Promise<Response>
- sym:43448 getBodySize (source/utils/body.ts:14-51)
  const getBodySize = (body?: BodyInit | null): number =>
- sym:43340 Ky.create (source/core/Ky.ts:253-452)
  static create(input: Input, options: Options): ResponsePromise
- sym:43430 Options (source/types/options.ts:446-500)
- sym:43530 ReceivedBody (test/request-body-serialization.ts:6-10)
- sym:43539 createAbortDuringBodyFetch (test/retry.ts:4019-4038)
- sym:43531 createBodyServer (test/request-body-serialization.ts:12-22)
- sym:43541 createUploadProgressTestServer (test/stream.ts:1185-1195)

---
tokens: 678/800
