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
- sym:43453 streamResponse (source/utils/body.ts:198-211)
  const streamResponse = (response: Response, onDownloadProgress: Options['onDownloadProgress']) =>
- sym:43450 getProgressCallbackError (source/utils/body.ts:104-105)
```typescript
export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
	stream ? progressCallbackErrors.get(stream)?.() : undefined;
```
- sym:43350 Ky.#raceBodyRead (source/core/Ky.ts:883-949)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:43451 copyResponseMetadata (source/utils/body.ts:107-171)
  const copyResponseMetadata = (response: Response, originalResponse: Response, getError = originalResponse.body ? responseSizeErrors.get(originalResponse.body) : undefined): Response =>
- sym:43454 streamRequest (source/utils/body.ts:214-232)
  const streamRequest = (request: Request, onUploadProgress: Options['onUploadProgress'], originalBody?: BodyInit | null) =>
- sym:43347 Ky.#getResponseData (source/core/Ky.ts:807-844)
  async #getResponseData(response: Response): Promise<unknown>
- sym:43449 withProgress (source/utils/body.ts:53-99)
  const withProgress = (stream: ReadableStream<Uint8Array>, totalBytes: number, onProgress: Options['onDownloadProgress'] | Options['onUploadProgress']): ReadableStream<Uint8Array> =>
- sym:43386 parseDate (source/core/retry-timing.ts:88-149)
  const parseDate = (value: string): number | undefined =>
- sym:43448 getBodySize (source/utils/body.ts:14-51)
  const getBodySize = (body?: BodyInit | null): number =>
- sym:43340 Ky.create (source/core/Ky.ts:253-452)
- sym:43365 Ky.#fetch (source/core/Ky.ts:1368-1428)
  async #fetch(): Promise<Response>
- sym:43430 Options (source/types/options.ts:446-500)
- sym:43530 ReceivedBody (test/request-body-serialization.ts:6-10)
- sym:43531 createBodyServer (test/request-body-serialization.ts:12-22)
- sym:43539 createAbortDuringBodyFetch (test/retry.ts:4019-4038)

---
tokens: 676/800
