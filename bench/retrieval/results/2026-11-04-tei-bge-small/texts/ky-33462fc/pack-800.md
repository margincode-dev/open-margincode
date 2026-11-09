## Task
# ky-33462fc · ky-33462fc · open
Goal: Reject a nullish hook list instead of treating it as absent
Workspace: /root/bench/av_tei/ws/rec09fc-ky-33462fc/ky-33462fc
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
- sym:41889 mergeHooks (source/utils/merge.ts:217-231)
  const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks =>
- sym:41888 newHookValue (source/utils/merge.ts:197-215)
  function newHookValue<K extends keyof Hooks>(original: Hooks, incoming: Hooks, property: K): NormalizedHooks[K]
- sym:41902 hasSearchParameters (source/utils/options.ts:31-60)
  const hasSearchParameters = (search: SearchParamsOption): boolean =>
- sym:41750 Ky (source/core/Ky.ts:267-1585)
  class Ky
- sym:41961 initHook (test-d/hooks.ts:18-29)
  const initHook: InitHook = options =>
- sym:41751 Ky.create (source/core/Ky.ts:268-464)
  static create(input: Input, options: Options): ResponsePromise
- sym:41768 Ky.#cancelReturnedBody (source/core/Ky.ts:1196-1202)
  #cancelReturnedBody(value: unknown): void
- sym:41960 beforeErrorHook (test-d/hooks.ts:5-5)
  const beforeErrorHook: BeforeErrorHook = ({error}) =>
- sym:41901 findUnknownOptions (source/utils/options.ts:5-29)
  const findUnknownOptions = ( options: Record<string, unknown>, ): Record<string, unknown> =>
- sym:41871 delay (source/utils/delay.ts:9-29)
- sym:41762 Ky.#raceWithTotalTimeout (source/core/Ky.ts:983-1067)
- sym:41832 Hooks (source/types/hooks.ts:90-371)
- sym:41753 Ky.constructor (source/core/Ky.ts:498-668)
  constructor(input: Input, options: Options = {})
- sym:41833 NormalizedHooks (source/types/hooks.ts:376-376)
- sym:41836 SearchParamsOption (source/types/options.ts:10-14)
- sym:41873 isObject (source/utils/is.ts:2-2)
- sym:41931 promiseWithTimeout (test/helpers/with-page.ts:19-37)
- sym:41957 createAbortDuringFetch (test/retry.ts:4040-4056)
- sym:41956 createAbortDuringBodyFetch (test/retry.ts:4019-4038)

---
tokens: 683/800
