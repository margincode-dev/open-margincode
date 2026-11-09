## Task
# ky-74224d8 · ky-74224d8 · open
Goal: Make a non-Error progress-callback throw non-retriable too
Workspace: /root/bench/ws/rebf815-ky-74224d8/ky-74224d8
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
- sym:42441 Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
  async #calculateRetryDelay(error: unknown)
- sym:42550 getProgressCallbackError (source/utils/body.ts:104-105)
```typescript
export const getProgressCallbackError = (stream: ReadableStream | undefined): unknown =>
	stream ? progressCallbackErrors.get(stream)?.() : undefined;
```
- sym:42447 Ky.#raceBodyRead (source/core/Ky.ts:913-979)
  async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown>
- sym:42538 ShouldRetryState (source/types/retry.ts:9-19)
  type ShouldRetryState = { /** The error that caused the request to fail. */ error: Error; /** The number of retries attempted. Starts at 1 for the first retry. */ retryCount: number; }
- sym:42443 Ky.#throwProcessedError (source/core/Ky.ts:805-835)
  async #throwProcessedError(error: unknown, request: Request = this.#sentRequest): Promise<never>
- sym:42551 copyResponseMetadata (source/utils/body.ts:107-171)
- sym:42513 BeforeErrorState (source/types/hooks.ts:57-70)
- sym:42549 withProgress (source/utils/body.ts:53-99)
- sym:42488 ForceRetryError (source/errors/ForceRetryError.ts:10-39)
  class ForceRetryError extends KyError
- sym:42650 ExpectedProgressCallbacks (test-d/normalized-options.ts:34-37)
- sym:42439 Ky.#validateCallbackOptions (source/core/Ky.ts:662-675)
- sym:42496 NonError (source/errors/NonError.ts:6-28)
- sym:42539 RetryOptions (source/types/retry.ts:21-187)
- sym:42595 isForceRetryError (source/utils/type-guards.ts:129-131)
- sym:42463 Ky.#fetch (source/core/Ky.ts:1404-1468)
- sym:42480 retry (source/core/constants.ts:256-256)
- sym:42483 getRetryTimingHeader (source/core/retry-timing.ts:25-49)
- sym:42487 calculateRetryTimingDelay (source/core/retry-timing.ts:151-173)

---
tokens: 681/800
