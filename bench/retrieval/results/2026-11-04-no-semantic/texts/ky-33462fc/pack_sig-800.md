## Task
# ky-33462fc · ky-33462fc-2 · open
Goal: Reject a nullish hook list instead of treating it as absent
Workspace: /root/bench/ws/rebf815-ky-33462fc/ky-33462fc-2
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
- sym:41902 hasSearchParameters (source/utils/options.ts:31-60)
  const hasSearchParameters = (search: SearchParamsOption): boolean =>
- sym:41889 mergeHooks (source/utils/merge.ts:217-231)
  const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks =>
- sym:41750 Ky (source/core/Ky.ts:267-1585)
  class Ky
- sym:41751 Ky.create (source/core/Ky.ts:268-464)
  static create(input: Input, options: Options): ResponsePromise
- sym:41871 delay (source/utils/delay.ts:9-29)
  async function delay( ms: number, {signal}: DelayOptions, ): Promise<void>
- sym:41762 Ky.#raceWithTotalTimeout (source/core/Ky.ts:983-1067)
  async #raceWithTotalTimeout<T>(operation: () => Promise<T>, abortSignal?: AbortSignal): Promise<T | typeof timedOutOperation>
- sym:41753 Ky.constructor (source/core/Ky.ts:498-668)
  constructor(input: Input, options: Options = {})
- sym:41832 Hooks (source/types/hooks.ts:90-371)
- sym:41833 NormalizedHooks (source/types/hooks.ts:376-376)
  type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>}
- sym:41836 SearchParamsOption (source/types/options.ts:10-14)
  type SearchParamsOption = | Exclude<SearchParamsInit, string[][]> | Record<string, string | number | boolean | undefined> | Array<Array<string | number | boolean>> | ReadonlyArray<ReadonlyArray<string | number | boolean>>
- sym:41931 promiseWithTimeout (test/helpers/with-page.ts:19-37)
  const promiseWithTimeout = async <Value>( promise: Promise<Value>, timeoutInMilliseconds: number, errorMessage: string, ): Promise<Value> =>
- sym:41957 createAbortDuringFetch (test/retry.ts:4040-4056)
  const createAbortDuringFetch = (onRequest?: () => void): typeof fetch =>

---
tokens: 639/800
