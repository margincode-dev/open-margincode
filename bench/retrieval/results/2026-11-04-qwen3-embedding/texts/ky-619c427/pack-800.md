## Task
# ky-619c427 · ky-619c427 · open
Goal: Name the request that was sent on every `TimeoutError`

## Map
- source/errors/TimeoutError.ts
- source/core/Ky.ts
- source/errors/KyError.ts
- source/utils/timeout.ts
- source/utils/type-guards.ts
- source/core/constants.ts
- source/core/retry-timing.ts
- source/errors/ForceRetryError.ts
- source/errors/HTTPError.ts
- source/errors/NetworkError.ts
- source/errors/NonError.ts
- source/errors/ResponseSizeError.ts
- source/index.ts
- source/types/ky.ts

## Code
- sym:42729 TimeoutError (source/errors/TimeoutError.ts:7-15)
  class TimeoutError extends KyError
- sym:42673 Ky.#getErrorDataTimeout (source/core/Ky.ts:876-894)
  #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout
- sym:42684 Ky.#throwIfTotalTimeoutExhausted (source/core/Ky.ts:1197-1202)
  #throwIfTotalTimeoutExhausted(request: Request = this.request): void
- sym:42730 TimeoutError.constructor (source/errors/TimeoutError.ts:11-14)
  constructor(request: Request)
- sym:42821 isTimeoutError (source/utils/type-guards.ts:102-104)
- sym:42816 timeout (source/utils/timeout.ts:10-47)
- sym:42672 Ky.#getResponseData (source/core/Ky.ts:837-874)
- sym:42674 Ky.#getEffectiveTimeout (source/core/Ky.ts:896-909)
- sym:42688 Ky.#retryFromError (source/core/Ky.ts:1303-1390)
- sym:42652 ErrorDataTimeout (source/core/Ky.ts:65-68)
- sym:42654 validateTimeoutOption (source/core/Ky.ts:87-99)
- sym:42744 Hooks (source/types/hooks.ts:90-371)
- sym:42664 Ky.create (source/core/Ky.ts:253-457)
- sym:42669 Ky.#calculateRetryDelay (source/core/Ky.ts:701-781)
- sym:42675 Ky.#raceBodyRead (source/core/Ky.ts:913-979)
- sym:42690 Ky.#fetch (source/core/Ky.ts:1398-1462)

---
tokens: 722/800
omitted 30 lower-ranked items (expand an id for detail):
- source/types/ResponsePromise.ts
- source/types/options.ts
- repo map: .editorconfig .gitattributes .github/ .gitignore .npmrc AGENTS.md license media/ package.json readme.md source/ test-d/ (106 files)
- sym:42685 Ky.#runBeforeRequestHooks (source/core/Ky.ts:1204-1225)
- sym:42686 Ky.#runAfterResponseHooks (source/core/Ky.ts:1227-1293)
- sym:42719 KyError (source/errors/KyError.ts:8-14)
- sym:42656 isRequestInstance (source/core/Ky.ts:130-131)
- sym:42657 isResponseInstance (source/core/Ky.ts:136-137)
- sym:42661 cloneInitHookOptions (source/core/Ky.ts:157-220)
- … and 21 more
