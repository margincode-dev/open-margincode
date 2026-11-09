## Task
# ky-4cc6698 · ky-4cc6698 · open
Goal: Recognise a cross-realm `beforeRetry` error and keep the retried error as the cause

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
- sym:41986 Ky.#throwProcessedError (source/core/Ky.ts:805-835)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42019 ForceRetryOptions (source/core/constants.ts:73-152)
- sym:42032 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:42005 Ky.#retryFromError (source/core/Ky.ts:1332-1419)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:42057 BeforeErrorState (source/types/hooks.ts:57-70)
- sym:42055 BeforeRetryState (source/types/hooks.ts:44-53)
  type BeforeRetryState = { request: KyRequest; options: Readonly<NormalizedOptions>; error: Error; /** The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request. */ retryCount: number; }
- sym:41979 Ky (source/core/Ky.ts:267-1582)
  class Ky
- sym:42082 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:41984 Ky.#calculateRetryDelay (source/core/Ky.ts:694-774)
  async #calculateRetryDelay(error: unknown)
- sym:41980 Ky.create (source/core/Ky.ts:268-464)
  static create(input: Input, options: Options): ResponsePromise
- sym:42053 BeforeRequestState (source/types/hooks.ts:32-40)
- sym:42118 mergeHooks (source/utils/merge.ts:217-231)
  const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks =>
- sym:42189 beforeErrorHook (test-d/hooks.ts:5-5)
- sym:41972 isRequestInstance (source/core/Ky.ts:145-146)
- sym:41973 isResponseInstance (source/core/Ky.ts:151-152)
- sym:42040 NonError (source/errors/NonError.ts:6-28)
  class NonError extends Error
- sym:42046 TimeoutError (source/errors/TimeoutError.ts:7-15)

---
tokens: 685/800
