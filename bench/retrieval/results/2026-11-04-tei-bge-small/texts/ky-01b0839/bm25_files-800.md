### source/types/hooks.ts
```typescript
import {type stop, type RetryMarker} from '../core/constants.js';
import type {KyRequest, KyResponse} from '../index.js';
import type {InitOptions, NormalizedOptions} from './options.js';

/**
This hook enables you to modify the options before they are used to construct the request. The hook function receives the mutable options object and can modify it in place. You could, for example, modify `searchParams`, `headers`, or `json` here. The `headers` option is always a plain object with lowercase names, where a header removed with `undefined` keeps an `undefined` value.

The `body` option is the exception: it is passed through by reference, so mutating a `FormData` or `URLSearchParams` body in place changes your own object and repeats on every request from the same instance. Replace `options.body` with a new instance instead, the way the `beforeRequest` workflow does.

Unlike other hooks, `init` hooks are synchronous. Any error thrown will propagate synchronously and will not be caught by `beforeError` hooks.

@example
```
import ky from 'ky';

const api = ky.extend({
	hooks: {
		init: [
			options => {
				options.searchParams = {apiKey: getApiKey()};
			},
		],
	},
});

const response = await api.get('https://example.com/api/users');
// URL: https://example.com/api/users?apiKey=123
```
*/
export type InitHook = (options: InitOptions) => void;

export type BeforeRequestState = {
	request: KyRequest;
	options: Readonly<NormalizedOptions>;

	/**
	The number of retries attempted. Always `0`, since `beforeRequest` hooks run once before retry handling begins.
	*/
	retryCount: 0;
};

export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;

export type BeforeRetryState = {
	request: KyRequest;
	options: Readonly<NormalizedOptions>;
	error: Error;

	/**
	The number of retries attempted. Always `>= 1`, since this hook is only called during retries, not on the initial request.
	*/
	retryCount: number;
};

export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;

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

// Returns `Error` to allow replacing Ky errors with custom non-Ky error types.
export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;

export type AfterResponseState = {
	request: KyRequest;
	options: Readonly<NormalizedOptions>;
	response: KyResponse;

	/**
	The number of retries attempted. `0` for the initial request, increments with each retry.

	This allows you to distinguish between the initial request and retries, which is useful when you need different behavior for retries (e.g., showing a notification only on the final retry).
	*/
	retryCount: number;
};

export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;

export type Hooks = {
	/**
