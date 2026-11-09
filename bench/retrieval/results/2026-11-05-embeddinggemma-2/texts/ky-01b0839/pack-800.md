## Task
# ky-01b0839 · ky-01b0839 · open
Goal: Recognise another copy of Ky: the stop symbol and the retry marker

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
- sym:43182 createInstance (source/index.ts:10-33)
  const createInstance = (defaults?: Partial<Options>): KyInstance =>
- sym:43158 retry (source/core/constants.ts:248-248)
```typescript
export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);
```
- sym:43140 Ky.#retry (source/core/Ky.ts:1272-1278)
```typescript
	async #retry(): Promise<Response | void> {
		try {
			return await this.#fetch();
		} catch (error) {
			return this.#retryFromError(error);
		}
	}
```
- sym:43142 Ky.#consumeReturnedResponseFromBeforeRetryHook (source/core/Ky.ts:1369-1373)
  #consumeReturnedResponseFromBeforeRetryHook(): boolean
- sym:43190 BeforeRetryHook (source/types/hooks.ts:55-55)
  type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>
- sym:43246 cloneShallow (source/utils/merge.ts:91-113)
  const cloneShallow = <T>(value: T): T =>
- sym:43144 Ky.#throwIfAbortedByUser (source/core/Ky.ts:1442-1444)
  #throwIfAbortedByUser(): void
- sym:43194 AfterResponseHook (source/types/hooks.ts:88-88)
- sym:43156 RetryMarker (source/core/constants.ts:157-163)
  class RetryMarker
- sym:43139 Ky.#runAfterResponseHooks (source/core/Ky.ts:1204-1270)
  async #runAfterResponseHooks(response: Response): Promise<Response>
- sym:43157 RetryMarker.constructor (source/core/constants.ts:160-162)
  constructor(options?: ForceRetryOptions)
- sym:43170 KyError (source/errors/KyError.ts:8-14)
- sym:43117 Ky (source/core/Ky.ts:252-1527)
  class Ky
- sym:43124 Ky.#throwProcessedError (source/core/Ky.ts:782-812)
- sym:43109 cloneRetryOptions (source/core/Ky.ts:101-122)
- sym:43115 cloneInitHookOptions (source/core/Ky.ts:157-220)
- sym:43118 Ky.create (source/core/Ky.ts:253-452)
- sym:43120 Ky.constructor (source/core/Ky.ts:485-652)

---
tokens: 688/800
