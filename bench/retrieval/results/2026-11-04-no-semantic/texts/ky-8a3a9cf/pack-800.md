## Task
# ky-8a3a9cf · ky-8a3a9cf-2 · open
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
- sym:42981 KyOptions (source/types/options.ts:45-432)
- sym:42967 BeforeErrorState (source/types/hooks.ts:57-70)
```typescript
export type BeforeErrorState = {
	request: KyRequest;
	options: Readonly<NormalizedOptions>;

	// `Error` (not `KyError`) because this receives all errors, including non-Ky ones.
	error: Error;

	/**
	The number of retries attempted. `0` for the initial request, increments with each retry.

	This allows you to distinguish between the initial request and retries, which is useful when you need different error handling based on retry attempts (e.g., showing different error messages on the final attempt).
	*/
	retryCount: number;
};
```
- sym:43103 ExpectedNormalizedRetryOptions (test-d/normalized-options.ts:22-33)
```typescript
type ExpectedNormalizedRetryOptions = {
	limit: number;
	methods: string[];
	statusCodes: number[];
	afterStatusCodes: number[];
	maxRetryAfter: number;
	backoffLimit: number;
	delay: (attemptCount: number) => number;
	jitter: boolean | ((delay: number) => number) | undefined;
	retryOnTimeout: boolean;
	shouldRetry?: RetryOptions['shouldRetry'];
};
```
- sym:43043 timeout (source/utils/timeout.ts:10-47)
  async function timeout( request: Request, init: RequestInit, abortController: AbortController | undefined, options: TimeoutOptions, ): Promise<Response>
- sym:42993 RetryOptions (source/types/retry.ts:21-187)
- sym:42917 Ky.#fetch (source/core/Ky.ts:1394-1458)
  async #fetch(): Promise<Response>
- sym:42891 Ky.create (source/core/Ky.ts:253-453)
  static create(input: Input, options: Options): ResponsePromise
- sym:42893 Ky.constructor (source/core/Ky.ts:486-654)
  constructor(input: Input, options: Options = {})
- sym:42900 Ky.#getErrorDataTimeout (source/core/Ky.ts:872-890)
  #getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout
- sym:42901 Ky.#getEffectiveTimeout (source/core/Ky.ts:892-905)

---
tokens: 685/800
