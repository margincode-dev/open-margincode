## Task
# ky-01b0839 · ky-01b0839 · open
Goal: Recognise another copy of Ky: the stop symbol and the retry marker
Workspace: /root/bench/ws/rebf815-ky-01b0839/ky-01b0839
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
- sym:43156 RetryMarker (source/core/constants.ts:157-163)
  class RetryMarker
- sym:43182 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:43189 BeforeRetryState (source/types/hooks.ts:44-53)
  type BeforeRetryState = { request: KyRequest; options: Readonly<NormalizedOptions>; error: Error; /** The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request. */ retryCount: number; }
- sym:43190 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:43191 BeforeErrorState (source/types/hooks.ts:57-70)
- sym:43216 ShouldRetryState (source/types/retry.ts:9-19)
- sym:43246 cloneShallow (source/utils/merge.ts:91-113)
  const cloneShallow = <T>(value: T): T =>
- sym:43194 AfterResponseHook (source/types/hooks.ts:88-88)
- sym:43193 AfterResponseState (source/types/hooks.ts:75-86)
- sym:43122 Ky.#calculateRetryDelay (source/core/Ky.ts:678-758)
  async #calculateRetryDelay(error: unknown)
- sym:43158 retry (source/core/constants.ts:248-248)
- sym:43157 RetryMarker.constructor (source/core/constants.ts:160-162)
  constructor(options?: ForceRetryOptions)
- sym:43267 isKyError (source/utils/type-guards.ts:36-38)
- sym:43124 Ky.#throwProcessedError (source/core/Ky.ts:782-812)
- sym:43171 KyError.isKyError (source/errors/KyError.ts:11-13)
- sym:43114 cloneSearchParametersForInitHook (source/core/Ky.ts:145-151)
- sym:43117 Ky (source/core/Ky.ts:252-1527)
  class Ky
- sym:43118 Ky.create (source/core/Ky.ts:253-452)
- sym:43197 KyInstance (source/types/ky.ts:5-204)

---
tokens: 684/800
