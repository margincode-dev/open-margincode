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
	This hook enables you to modify the options before they are used to construct the request. The hook function receives the mutable options object and can modify it in place. You could, for example, modify `searchParams`, `headers`, or `json` here. The `headers` option is always a plain object with lowercase names, where a header removed with `undefined` keeps an `undefined` value.

	Unlike other hooks, `init` hooks are synchronous. Any error thrown will propagate synchronously and will not be caught by `beforeError` hooks.

	The `body` option is passed through by reference, so mutating a `FormData` or `URLSearchParams` body in place changes your own object and repeats on every request from the same instance. Replace `options.body` with a new instance instead, the way the `beforeRequest` workflow does.

	A common use case is to add a search parameter to every request:

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

	@default []
	*/
	init?: readonly InitHook[] | undefined;

	/**
	This hook enables you to modify the request right before it is sent. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, and retry count. You could, for example, modify `request.headers` here.

	The `retryCount` is always `0`, since `beforeRequest` hooks run once before retry handling begins.

	The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing request (remaining hooks will still run with the updated request). It can also return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to completely avoid making an HTTP request, in which case remaining `beforeRequest` hooks are skipped. This can be used to mock a request, check an internal cache, etc. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work.

	Any error thrown by `beforeRequest` hooks is treated as fatal and will not trigger Ky's retry logic.

	@example
	```
	import ky from 'ky';

	const api = ky.extend({
		hooks: {
			beforeRequest: [
				({request}) => {
					request.headers.set('Authorization', 'token initial-token');
				}
			]
		}
	});

	const response = await api.get('https://example.com/api/users');
	```

	**Modifying the request URL:**

	@example
	```
	import ky from 'ky';

	const api = ky.extend({
		hooks: {
			beforeRequest: [
				({request}) => {
					const url = new URL(request.url);
					url.searchParams.set('token', 'secret-token');
					return new Request(url, request);
				}
			]
		}
	});

	const response = await api.get('https://example.com/api/users');
	```

	@default []
	*/
	beforeRequest?: readonly BeforeRequestHook[] | undefined;

	/**
	This hook enables you to modify the request right before retry. Ky will make no further changes to the request after this. The hook function receives a state object with the normalized request, options, an error instance, and retry count. You could, for example, modify `request.headers` here.

	The hook can return a [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request) to replace the outgoing retry request, or return a [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response) to skip the retry and use that response instead. A returned `Request` instance's `signal` is replaced with Ky's managed signal so that timeouts and user-provided abort signals still work. **Note:** Returning a request or response skips remaining `beforeRetry` hooks.

	**Warning:** Returned `Request` objects are used as-is. If you point one at another origin, remove any credentials you do not want forwarded.

	The `retryCount` is always `>= 1`, since this hook is only called during retries, not on the initial request.

	If the request received a response, the error will be of type `HTTPError`. The `Response` object will be available at `error.response`, and the pre-parsed response body may be available at `error.data`. Be aware that some types of errors, such as network errors, inherently mean that a response was not received. In that case, the error will be an instance of `NetworkError` instead of `HTTPError`.

	You can prevent Ky from retrying the request by throwing an error. Ky will not handle it in any way and the error will be propagated to the request initiator. The rest of the `beforeRetry` hooks will not be called in this case. Alternatively, you can return the [`ky.stop`](#kystop) symbol to do the same thing but without propagating an error (this has some limitations, see `ky.stop` docs for details).

	**Modifying headers:**

	@example
	```
	import ky from 'ky';

	const response = await ky('https://example.com', {
		hooks: {
			beforeRetry: [
				async ({request, options, error, retryCount}) => {
					const token = await ky('https://example.com/refresh-token');
					request.headers.set('Authorization', `token ${token}`);
				}
			]
		}
	});
	```

	**Modifying the request URL:**

	@example
	```
