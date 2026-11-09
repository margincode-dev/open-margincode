### source/core/Ky.ts
```typescript
import {HTTPError} from '../errors/HTTPError.js';
import {NetworkError} from '../errors/NetworkError.js';
import {NonError} from '../errors/NonError.js';
import {ForceRetryError} from '../errors/ForceRetryError.js';
import {SchemaValidationError} from '../errors/SchemaValidationError.js';
import {TimeoutError} from '../errors/TimeoutError.js';
import type {
	Input,
	InitOptions,
	InternalOptions,
	KyHeadersInit,
	NormalizedOptions,
	Options,
	SearchParamsInit,
	SearchParamsOption,
} from '../types/options.js';
import {type ResponsePromise} from '../types/ResponsePromise.js';
import type {StandardSchemaV1} from '../types/standard-schema.js';
import {
	getProgressCallbackError,
	limitResponseSize,
	streamRequest,
	streamResponse,
} from '../utils/body.js';
import {
	cloneShallow,
	cloneDeep,
	mergeHeaders,
	mergeHeaderContainers,
	mergeHooks,
	deletedParametersSymbol,
} from '../utils/merge.js';
import type {RetryOptions} from '../types/retry.js';
import {normalizeRequestMethod, normalizeRetryMethod, normalizeRetryOptions} from '../utils/normalize.js';
import timeout from '../utils/timeout.js';
import delay from '../utils/delay.js';
import {type ObjectEntries} from '../utils/types.js';
import {findUnknownOptions, hasSearchParameters} from '../utils/options.js';
import isRawNetworkError from '../utils/is-network-error.js';
import {isError} from '../utils/is.js';
import {
	isHTTPError, isNetworkError, isTimeoutError, isResponseSizeError, isForceRetryError,
} from '../utils/type-guards.js';
import {
	calculateRetryTimingDelay,
	getRetryTimingHeader,
} from './retry-timing.js';
import {
	maxSafeTimeout,
	responseTypes,
	stop,
	isRetryMarker,
	supportsAbortController,
	supportsAbortSignal,
	supportsFormData,
	supportsResponseStreams,
	supportsRequestStreams,
} from './constants.js';

const maxErrorResponseBodySize = 10 * 1024 * 1024;
const prefixUrlRenamedErrorMessage = 'The `prefixUrl` option has been renamed `prefix` in v2 and enhanced to allow slashes in input. See also the new `baseUrl` option for improved flexibility with standard URL resolution: https://github.com/sindresorhus/ky#baseurl';
const timedOutResponseData = Symbol('timedOutResponseData');
const timedOutOperation = Symbol('timedOutOperation');

type ErrorDataTimeout = {
	milliseconds: number;
	fromTotalTimeout: boolean;
};

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

const invalidSchemaMessage = 'The `schema` argument must follow the Standard Schema specification';
const missingResponseMessage = 'The request resolved without a response. Returning `ky.stop` from a `beforeRetry` hook is not compatible with body method shortcuts. Throw from the hook instead.';

// Both timeout options are milliseconds or `false`. A non-finite or negative value used to reach `setTimeout()`,
// which silently clamps it to ~1ms, or to be ignored entirely when `totalTimeout` was not a number.
const validateTimeoutOption = (value: unknown, name: 'timeout' | 'totalTimeout'): void => {
	if (value === false) {
		return;
	}

	if (typeof value !== 'number' || !Number.isFinite(value) || value < 0) {
		throw new TypeError(`The \`${name}\` option must be a non-negative number or \`false\``);
	}

	if (value > maxSafeTimeout) {
		throw new RangeError(`The \`${name}\` option cannot be greater than ${maxSafeTimeout}`);
	}
};

const cloneRetryOptions = (retry: RetryOptions | number): RetryOptions | number => {
	if (retry === null || typeof retry !== 'object' || Array.isArray(retry)) {
		return retry as RetryOptions | number;
	}

	const clonedRetry = {...retry};

	// Clone nested arrays too so init hooks can mutate retry config without leaking state across requests.
	if (Array.isArray(clonedRetry.methods)) {
		clonedRetry.methods = [...clonedRetry.methods];
	}

	if (Array.isArray(clonedRetry.statusCodes)) {
		clonedRetry.statusCodes = [...clonedRetry.statusCodes];
	}

	if (Array.isArray(clonedRetry.afterStatusCodes)) {
		clonedRetry.afterStatusCodes = [...clonedRetry.afterStatusCodes];
	}

	return clonedRetry;
};

const objectToString = Object.prototype.toString;
const leadingC0ControlOrSpacePattern = /^[\0-\u0020]+/g;
const asciiTabOrNewLinePattern = /[\t\n\r]/g;
const schemePattern = /^[a-z][\d+.a-z-]*:/i;
const malformedHttpProtocolPattern = /^https?:(?!\/\/)/i;

const isRequestInstance = (value: unknown): value is Request =>
	value instanceof globalThis.Request || objectToString.call(value) === '[object Request]';

// Accepted custom responses are treated as full Responses throughout Ky.
// If a custom fetch returns one, it must behave like a Response for cloning,
// body consumption, `json()` decoration, and any enabled stream features.
const isResponseInstance = (value: unknown): value is Response =>
	value instanceof globalThis.Response || objectToString.call(value) === '[object Response]';

const isAbsoluteInput = (input: string): boolean =>
	schemePattern.test(input);

const normalizeInputForProtocolCheck = (input: string): string =>
	input.replaceAll(leadingC0ControlOrSpacePattern, '').replaceAll(asciiTabOrNewLinePattern, '');

const cloneSearchParametersForInitHook = (searchParameters: SearchParamsOption | undefined): SearchParamsOption | undefined => {
	if (Array.isArray(searchParameters)) {
		return searchParameters.map(parameter => [...parameter]) as SearchParamsOption;
	}

	return cloneShallow(searchParameters) as SearchParamsOption | undefined;
};

// Shallow-clone mutable option properties so init hook mutations don't leak across requests.
// Every one of them also copies on assignment, so a hook that replaces the value with an object of its own cannot
// accumulate mutations on that object across requests. Non-plain values (functions, class instances) are kept by
// reference, matching how option merging treats them as whole values.
function cloneInitHookOptions(options: Options): InitOptions {
	let headers = mergeHeaderContainers({}, options.headers ?? {});
	let context = cloneDeep(options.context) ?? {};
	let json = cloneDeep(options.json);
	let searchParameters = cloneSearchParametersForInitHook(options.searchParams);
	let retry = options.retry === undefined ? undefined : cloneRetryOptions(options.retry);

	const clonedOptions: Options = {...options};

	Object.defineProperties(clonedOptions, {
		headers: {
			enumerable: true,
			configurable: true,
			get() {
				return headers;
			},
			set(value: KyHeadersInit | undefined) {
				headers = mergeHeaderContainers({}, value ?? {});
			},
		},
		context: {
			enumerable: true,
			configurable: true,
			get() {
				return context;
			},
			set(value: Record<string, unknown> | undefined) {
				context = cloneDeep(value) ?? {};
			},
		},
		json: {
			enumerable: true,
			configurable: true,
			get() {
				return json;
			},
			set(value: unknown) {
				json = cloneDeep(value);
			},
		},
		searchParams: {
			enumerable: true,
			configurable: true,
			get() {
				return searchParameters;
			},
			set(value: SearchParamsOption | undefined) {
				searchParameters = cloneSearchParametersForInitHook(value);
			},
		},
		retry: {
			enumerable: true,
			configurable: true,
			get() {
				return retry;
			},
			set(value: Options['retry']) {
				retry = value === undefined ? undefined : cloneRetryOptions(value);
			},
		},
	});

	return clonedOptions as InitOptions;
}

const validateJsonWithSchema = async (jsonValue: unknown, schema: StandardSchemaV1): Promise<unknown> => {
	if (
		(
			typeof schema !== 'object'
			&& typeof schema !== 'function'
		)
		|| schema === null
	) {
		throw new TypeError(invalidSchemaMessage);
	}

	const standardSchema = schema['~standard'];

	if (
		typeof standardSchema !== 'object'
		|| standardSchema === null
		|| typeof standardSchema.validate !== 'function'
	) {
		throw new TypeError(invalidSchemaMessage);
	}

	const validationResult = await standardSchema.validate(jsonValue);

	if (validationResult.issues) {
		throw new SchemaValidationError(validationResult.issues);
	}

	return validationResult.value;
};

export class Ky {
	static create(input: Input, options: Options): ResponsePromise {
		const initHooks = options.hooks?.init ?? [];
		const initHookOptions = initHooks.length > 0 ? cloneInitHookOptions(options) : options;

		for (const hook of initHooks) {
			hook(initHookOptions as InitOptions);
		}

		const ky = new Ky(input, initHookOptions);

		const function_ = async (): Promise<Response | void> => {
			validateTimeoutOption(ky.#options.totalTimeout, 'totalTimeout');
			validateTimeoutOption(ky.#options.timeout, 'timeout');
			ky.#validateCallbackOptions();

			// Delay the fetch so that body method shortcuts can set the Accept header
			await Promise.resolve();
			const beforeRequestResponse = await ky.#runBeforeRequestHooks();
			if (beforeRequestResponse !== undefined) {
				ky.#retryLimit = normalizeRetryOptions(ky.#options.retry).limit;
			}

			let response = beforeRequestResponse ?? await ky.#retry();
			let responseFromHook = beforeRequestResponse !== undefined
				|| ky.#consumeReturnedResponseFromBeforeRetryHook();

			for (;;) {
				// `undefined` means a hook stopped the flow without providing a response.
				// Non-native Responses still continue through Ky if they pass `isResponseInstance()`.
				if (response === undefined) {
					return response;
				}

				if (isResponseInstance(response)) {
					try {
						// eslint-disable-next-line no-await-in-loop
						response = await ky.#runAfterResponseHooks(response);
					} catch (error) {
						if (!isForceRetryError(error)) {
							throw error;
						}

						// eslint-disable-next-line no-await-in-loop
						const retriedResponse: Response | void = await ky.#retryFromError(error);
						if (retriedResponse === undefined) {
							return retriedResponse;
						}

						response = retriedResponse;
						responseFromHook = ky.#consumeReturnedResponseFromBeforeRetryHook();
						continue;
					}
				}

				const currentResponse: Response = response;

				// Opaque responses (`response.type === 'opaque'`) from `no-cors` requests always have `status: 0` and `ok: false`, but this is not a failure - the actual status is hidden by the browser.
				if (!currentResponse.ok && currentResponse.type !== 'opaque' && (
					typeof ky.#options.throwHttpErrors === 'function'
						? ky.#options.throwHttpErrors(currentResponse.status)
						: ky.#options.throwHttpErrors
				)) {
					// `request` must reflect the request that actually failed, but `options` stays as Ky's
					// normalized options snapshot. Replacement `Request` instances do not preserve the
					// original `BodyInit`, so trying to make `options` mirror arbitrary requests would be lossy.
					const httpError: HTTPError = new HTTPError(currentResponse, ky.#getResponseRequest(currentResponse), ky.#getNormalizedOptions());
					// eslint-disable-next-line no-await-in-loop
					httpError.data = await ky.#getResponseData(currentResponse);
					ky.#throwIfAbortedByUser();
					ky.#throwIfTotalTimeoutExhausted();

					if (responseFromHook) {
						throw httpError;
					}

					// eslint-disable-next-line no-await-in-loop
					const retriedResponse: Response | void = await ky.#retryFromError(httpError);
					if (retriedResponse === undefined) {
						return retriedResponse;
					}

					response = retriedResponse;
					responseFromHook = ky.#consumeReturnedResponseFromBeforeRetryHook();
					continue;
				}

				break;
			}

			if (!isResponseInstance(response)) {
				return response;
			}

			ky.#decorateResponse(response);

			// If `onDownloadProgress` is passed, it uses the stream API internally
			if (ky.#options.onDownloadProgress) {
				if (typeof ky.#options.onDownloadProgress !== 'function') {
					throw new TypeError('The `onDownloadProgress` option must be a function');
				}

				if (!supportsResponseStreams) {
					throw new Error('Streams are not supported in your environment. `ReadableStream` is missing.');
				}

				const progressResponse = streamResponse(response, ky.#options.onDownloadProgress);
				ky.#setResponseRequest(progressResponse, ky.#getResponseRequest(response));
				return ky.#decorateResponse(progressResponse);
			}

			return response;
		};

		const result = (async () => {
			let response: Response | undefined;
			try {
				response = (await function_()) ?? undefined;
				return response;
			} catch (error: unknown) {
				return await ky.#throwProcessedError(error);
			} finally {
				const originalRequest = ky.#originalRequest;

				// Ignore cancellation errors from already-locked or already-consumed streams.
				// A custom fetch or hook can return the request body as its response body; ownership then belongs to the caller.
				if (originalRequest?.body !== response?.body) {
					ky.#cancelBody(originalRequest?.body ?? undefined);
				}

				// Only cancel the current request body if it's distinct from the original (i.e. it was cloned for
				// retries). `#originalRequest` is only set once a request is handed to fetch, so a pipeline that
				// never started, for example one that failed validation, leaves the caller's own body alone.
				if (originalRequest && ky.request !== originalRequest && ky.request.body !== response?.body) {
					ky.#cancelBody(ky.request.body ?? undefined);
				}
			}
		})() as ResponsePromise;

		for (const [type, mimeType] of Object.entries(responseTypes) as ObjectEntries<typeof responseTypes>) {
			// Only expose `.bytes()` when the environment implements it.
			if (
				type === 'bytes'
				&& typeof (globalThis.Response?.prototype as unknown as {bytes?: unknown})?.bytes !== 'function'
			) {
				continue;
			}

			result[type] = async (schema?: StandardSchemaV1) => {
				// `#fetch()` replaces `ky.request` with the clone it prepares for a possible retry, so a shortcut called
				// after the request was dispatched would otherwise advertise its media type on that clone, which is only
				// sent if a retry happens, instead of on the attempt that produced the response.
				const acceptRequest = ky.#sentRequest;
				// eslint-disable-next-line @typescript-eslint/prefer-nullish-coalescing
				acceptRequest.headers.set('accept', acceptRequest.headers.get('accept') || mimeType);

				const response = await result;

				// `ky.stop` from a `beforeRetry` hook resolves the request with no response at all. Reading a body
				// method would otherwise crash deep inside Ky with a TypeError that names Ky internals.
				if (response === undefined) {
					throw new TypeError(missingResponseMessage);
				}

				if (type !== 'json') {
					return ky.#raceBodyRead(async () => response[type](), response);
				}

				const text = await ky.#raceBodyRead(async () => response.text(), response) as string;
				const request = ky.#getResponseRequest(response);
				// This implementation is shared by every body method, so its return type has to satisfy all of them. The parsed value is whatever `parseJson` or `JSON.parse` produced, which is `any` at this boundary.
				let parsedResult: any;
				try {
					parsedResult = await ky.#raceWithTotalTimeout(async () => {
						const jsonValue = initHookOptions.parseJson
							? await initHookOptions.parseJson(text, {request, response})
							: (text === '' && schema !== undefined
								? undefined
								: JSON.parse(text));

						if (schema === undefined) {
							return jsonValue;
						}

						// eslint-disable-next-line no-return-await, @typescript-eslint/return-await -- Awaiting here preserves the caller's async stack when schema validation fails.
						return await validateJsonWithSchema(jsonValue, schema);
					}, ky.#userProvidedAbortSignal);
				} catch (error: unknown) {
					// A cancellation is part of the request lifecycle, so it reaches `beforeError` like every other abort. Other failures, such as invalid JSON, stay as they are.
					if (ky.#userProvidedAbortSignal?.aborted) {
						await ky.#throwProcessedError(ky.#userProvidedAbortSignal.reason, request);
					}

					throw error;
				}

				if (parsedResult === timedOutOperation) {
					await ky.#throwProcessedError(new TimeoutError(request), request);
				}

				return parsedResult;
			};
		}

		return result;
	}

	// eslint-disable-next-line unicorn/prevent-abbreviations
	static #normalizeSearchParams(searchParams: SearchParamsOption): SearchParamsOption {
		// Filter out undefined values from plain objects
		if (searchParams && typeof searchParams === 'object' && !Array.isArray(searchParams) && !(searchParams instanceof URLSearchParams)) {
			return Object.fromEntries(Object.entries(searchParams).filter(([, value]) => value !== undefined));
		}

		return searchParams;
	}

	public request: Request;
	#abortController?: AbortController;
	#retryCount = 0;
	#retryLimit: number;
	readonly #options: InternalOptions;
	// Keep the input Request alive because Node.js stops forwarding its signal when the Request is garbage-collected.
