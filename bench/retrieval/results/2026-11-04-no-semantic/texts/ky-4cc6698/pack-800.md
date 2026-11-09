## Task
# ky-4cc6698 · ky-4cc6698-2 · open
Goal: Recognise a cross-realm `beforeRetry` error and keep the retried error as the cause
Workspace: /root/bench/ws/rebf815-ky-4cc6698/ky-4cc6698-2
Changed (0): nothing yet

## Map
- repo map (107 files):
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
  test-d/ (21 files)
  test/ (38 files)
  tsconfig.dist.json
  tsconfig.json
  tsconfig.test.json

## Code
- sym:42005 Ky.#retryFromError (source/core/Ky.ts:1332-1419)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:41986 Ky.#throwProcessedError (source/core/Ky.ts:805-835)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42019 ForceRetryOptions (source/core/constants.ts:73-152)
- sym:41979 Ky (source/core/Ky.ts:267-1582)
  class Ky
- sym:41984 Ky.#calculateRetryDelay (source/core/Ky.ts:694-774)
  async #calculateRetryDelay(error: unknown)
- sym:41980 Ky.create (source/core/Ky.ts:268-464)
  static create(input: Input, options: Options): ResponsePromise
- sym:42118 mergeHooks (source/utils/merge.ts:217-231)
  const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks =>
- sym:42032 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:41972 isRequestInstance (source/core/Ky.ts:145-146)
  const isRequestInstance = (value: unknown): value is Request =>
- sym:41973 isResponseInstance (source/core/Ky.ts:151-152)
  const isResponseInstance = (value: unknown): value is Response =>
- sym:42040 NonError (source/errors/NonError.ts:6-28)
  class NonError extends Error
- sym:42046 TimeoutError (source/errors/TimeoutError.ts:7-15)
  class TimeoutError extends KyError

---
tokens: 565/800
