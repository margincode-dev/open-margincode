## Task
# ky-8a3a9cf · ky-8a3a9cf · open
Goal: Advertise the shortcut media type on the attempt that produced the response

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
- sym:42959 ResponsePromise (source/types/ResponsePromise.ts:7-69)
- sym:42899 Ky.#getResponseData (source/core/Ky.ts:833-870)
  async #getResponseData(response: Response): Promise<unknown>
- sym:42880 createTextDecoder (source/core/Ky.ts:70-80)
```typescript
const createTextDecoder = (contentType: string): TextDecoder => {
	const match = /;\s*charset\s*=\s*(?:"([^"]+)"|([^;,\s]+))/i.exec(contentType);
	const charset = match?.[1] ?? match?.[2];
	if (charset) {
		try {
			return new TextDecoder(charset);
		} catch {}
	}

	return new TextDecoder();
};
```
- sym:42981 KyOptions (source/types/options.ts:45-432)
- sym:42967 BeforeErrorState (source/types/hooks.ts:57-70)
- sym:42990 KyResponse (source/types/response.ts:1-4)
  type KyResponse<T = unknown> = { clone: () => KyResponse<T>; json: <J = T>() => Promise<J>; } & Response
- sym:43103 ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
- sym:43043 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:42993 RetryOptions (source/types/retry.ts:21-187)
- sym:42917 Ky.#fetch (source/core/Ky.ts:1394-1458)
  async #fetch(): Promise<Response>
- sym:43032 appendSearchParameters (source/utils/merge.ts:270-324)
- sym:42891 Ky.create (source/core/Ky.ts:253-453)
- sym:42893 Ky.constructor (source/core/Ky.ts:486-654)
- sym:42900 Ky.#getErrorDataTimeout (source/core/Ky.ts:872-890)
- sym:42901 Ky.#getEffectiveTimeout (source/core/Ky.ts:892-905)
- sym:43077 createResponseLike (test/hooks.ts:42-65)
- sym:43079 createStreamFetch (test/hooks.ts:100-114)
- sym:43080 createFakeResponse (test/http-error.ts:7-15)
- sym:43076 createRequestLike (test/hooks.ts:23-40)

---
tokens: 682/800
