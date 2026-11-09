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
- sym:42019 ForceRetryOptions (source/core/constants.ts:73-152)
- sym:41986 Ky.#throwProcessedError (source/core/Ky.ts:805-835)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42005 Ky.#retryFromError (source/core/Ky.ts:1332-1419)
  async #retryFromError(error: unknown): Promise<Response | void>
- sym:42032 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:41984 Ky.#calculateRetryDelay (source/core/Ky.ts:694-774)
  async #calculateRetryDelay(error: unknown)
- sym:41979 Ky (source/core/Ky.ts:267-1582)
  class Ky
- sym:42057 BeforeErrorState (source/types/hooks.ts:57-70)
  type BeforeErrorState = { request: KyRequest; options: Readonly<NormalizedOptions>; // `Error` (not `KyError`) because this receives all errors, including non-Ky ones. error: Error; /** The number of retries attempted. `0` for the initial request, increments with each retry. This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt). */ retryCount: number; }
- sym:42082 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:41980 Ky.create (source/core/Ky.ts:268-464)
  static create(input: Input, options: Options): ResponsePromise
- sym:42118 mergeHooks (source/utils/merge.ts:217-231)
- sym:42033 ForceRetryError.constructor (source/errors/ForceRetryError.ts:16-38)
  constructor(options?: ForceRetryOptions)
- sym:41972 isRequestInstance (source/core/Ky.ts:145-146)
- sym:41973 isResponseInstance (source/core/Ky.ts:151-152)
- sym:42040 NonError (source/errors/NonError.ts:6-28)
- sym:42046 TimeoutError (source/errors/TimeoutError.ts:7-15)

---
tokens: 685/800
