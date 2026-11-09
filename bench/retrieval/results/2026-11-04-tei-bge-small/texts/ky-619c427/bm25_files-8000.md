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
				const acceptRequest = ky.#originalRequest ?? ky.request;
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
	readonly #requestInput: Request | undefined;
	#originalRequest?: Request;
	#requestBodyCanBeCancelled = false;
	readonly #userProvidedAbortSignal: AbortSignal | undefined;
	readonly #beforeRetryHookErrors = new WeakSet<Error>();
	// Errors thrown by a user callback that runs while the request is in flight. Retrying would re-send the body and
	// run the callback again, so they are never retriable, whatever `shouldRetry` says.
	readonly #inFlightCallbackErrors = new WeakSet<Error>();
	#cachedNormalizedOptions: NormalizedOptions | undefined;
	readonly #startTime: number | undefined;
	#returnedResponseFromBeforeRetryHook = false;
	readonly #responseRequests = new WeakMap<Response, Request>();
	readonly #decoratedResponses = new WeakSet<Response>();

	// eslint-disable-next-line complexity
	constructor(input: Input, options: Options = {}) {
		// Checked before the options are built, since building them reads `input.headers`.
		if (typeof input !== 'string' && !(input instanceof URL || input instanceof globalThis.Request)) {
			throw new TypeError('`input` must be a string, URL, or Request');
		}

		const {maxResponseSize = Number.POSITIVE_INFINITY} = options;
		if (Object.hasOwn(options, 'prefixUrl')) {
			throw new Error(prefixUrlRenamedErrorMessage);
		}

		this.#options = {
			...options,
			headers: mergeHeaders((input as Request).headers, options.headers),
			hooks: mergeHooks({}, options.hooks),
			method: normalizeRequestMethod(options.method ?? (input as Request).method ?? 'GET'),
			referrer: options.referrer ?? (input as Request).referrer,
			referrerPolicy: options.referrerPolicy ?? (input as Request).referrerPolicy,
			// eslint-disable-next-line @typescript-eslint/prefer-nullish-coalescing
			prefix: String(options.prefix || ''),
			retry: normalizeRetryOptions(options.retry),
			throwHttpErrors: options.throwHttpErrors ?? true,
			timeout: options.timeout ?? 10_000,
			totalTimeout: options.totalTimeout ?? false,
			maxResponseSize,
			fetch: options.fetch ?? globalThis.fetch.bind(globalThis),
			// Deep-cloned so a hook mutating a nested plain object or array cannot write back to the instance defaults or the caller's object. Non-plain values such as class instances are kept by reference, matching how `json` and the shallow `context` merge treat them.
			context: cloneDeep(options.context) ?? {},
		};
		this.#retryLimit = this.#options.retry.limit;

		if (maxResponseSize !== Number.POSITIVE_INFINITY && (!Number.isSafeInteger(maxResponseSize) || maxResponseSize < 0)) {
			throw new TypeError('The `maxResponseSize` option must be a non-negative safe integer or Infinity');
		}

		this.#requestInput = input instanceof globalThis.Request ? input : undefined;

		if (typeof input === 'string') {
			if (this.#options.prefix) {
				const normalizedPrefix = this.#options.prefix.replace(/\/+$/, '');
				const normalizedInput = input.replace(/^\/+/, '');
				input = `${normalizedPrefix}/${normalizedInput}`;
			}

			if (this.#options.baseUrl) {
				const normalizedInput = normalizeInputForProtocolCheck(input);

				if (malformedHttpProtocolPattern.test(normalizedInput)) {
					throw new TypeError('`input` url protocol must be followed by `//` when using `baseUrl`');
				}

				if (!isAbsoluteInput(normalizedInput)) {
					input = new URL(input, (new Request(this.#options.baseUrl)).url);
				}
			}
		}

		if (supportsAbortController && supportsAbortSignal) {
			this.#userProvidedAbortSignal = this.#options.signal ?? this.#requestInput?.signal;
			this.#abortController = new globalThis.AbortController();
			this.#options.signal = this.#createManagedSignal();
		}

		if (supportsRequestStreams) {
			// @ts-expect-error - Types are outdated.
			this.#options.duplex = 'half';
		}

		// A `Request` input's headers are already merged in, so only a content-type from `options.headers` counts as user-provided.
		const userProvidedContentType = options.headers !== undefined && mergeHeaders({}, options.headers).has('content-type');

		if (this.#options.json !== undefined) {
			this.#options.body = this.#options.stringifyJson?.(this.#options.json) ?? JSON.stringify(this.#options.json);
			if (!userProvidedContentType) {
				this.#options.headers.set('content-type', 'application/json');
			}
		}

		// To provide correct form boundary, Content-Type header should be deleted when creating Request from another Request with FormData/URLSearchParams body
		// Only delete if user didn't explicitly provide a custom content-type
		if (
			input instanceof globalThis.Request
			&& ((supportsFormData && this.#options.body instanceof globalThis.FormData) || this.#options.body instanceof URLSearchParams)
			&& !userProvidedContentType
		) {
			this.#options.headers.delete('content-type');
		}

		this.request = new globalThis.Request(input, this.#options as RequestInit);
		this.#requestBodyCanBeCancelled = typeof globalThis.ReadableStream === 'function' && this.#options.body instanceof globalThis.ReadableStream;

		if (hasSearchParameters(this.#options.searchParams)) {
			const url = new URL(this.request.url);
			const deleted = (this.#options.searchParams as any)?.[deletedParametersSymbol] as Set<string> | undefined;

			if (deleted) {
				// Remove keys from the input URL first so later searchParams entries can intentionally re-add them.
				for (const key of deleted) {
					url.searchParams.delete(key);
				}
			}

			if (typeof this.#options.searchParams === 'string') {
				const stringSearchParameters = this.#options.searchParams.replace(/^\?/, '');
				if (stringSearchParameters !== '') {
					url.search = url.search ? `${url.search}&${stringSearchParameters}` : `?${stringSearchParameters}`;
				}
			} else {
				const optionsSearchParameters = new URLSearchParams(Ky.#normalizeSearchParams(this.#options.searchParams) as unknown as SearchParamsInit);

				for (const [key, value] of optionsSearchParameters.entries()) {
					url.searchParams.append(key, value);
				}
			}

			if (
				this.#options.searchParams
				&& typeof this.#options.searchParams === 'object'
				&& !Array.isArray(this.#options.searchParams)
				&& !(this.#options.searchParams instanceof URLSearchParams)
			) {
				for (const [key, value] of Object.entries(this.#options.searchParams)) {
					if (value === undefined) {
						url.searchParams.delete(key);
					}
				}
			}

			// Recreate request with the updated URL. We already have all options in this.#options, including duplex.
			// Rebuilding is also what drops an inherited `Request` body, so it is skipped when the search parameters leave the URL unchanged.
			if (url.href !== this.request.url) {
				// Request options are read back from the current request so values inherited from a `Request` input (like `credentials`) survive, while explicit options already won when that request was built.
				// A `Request` input's body is only available as a stream, so keeping it requires request stream support.
				const {
					body: inheritedBody,
					cache,
					credentials,
					integrity,
					keepalive,
					mode,
					redirect,
					referrer,
					referrerPolicy,
				} = this.request;
				const canUseInheritedBody = supportsRequestStreams && !keepalive && mode !== 'no-cors';

				this.request = new globalThis.Request(url, {
					...this.#options,
					cache,
					credentials,
					integrity,
					keepalive,
					mode,
					redirect,
					referrer,
					referrerPolicy,
					body: this.#options.body ?? (canUseInheritedBody ? inheritedBody : undefined),
				} as RequestInit);
			}
		}

		if (this.#options.onUploadProgress && typeof this.#options.onUploadProgress !== 'function') {
			throw new TypeError('The `onUploadProgress` option must be a function');
		}

		// `totalTimeout` starts when the request pipeline is created, so it also includes
		// Ky's internal scheduling and user hook time before the first fetch attempt.
		this.#startTime = typeof this.#options.totalTimeout === 'number' ? this.#getCurrentTime() : undefined;
	}

	// These callbacks are called directly, so a non-function value otherwise surfaces as a runtime error naming Ky's
	// own internals rather than the option the caller set.
	#validateCallbackOptions(): void {
		for (const key of ['parseJson', 'stringifyJson', 'fetch'] as const) {
			if (this.#options[key] !== undefined && typeof this.#options[key] !== 'function') {
				throw new TypeError(`The \`${key}\` option must be a function`);
			}
		}

		if (
			typeof this.#options.throwHttpErrors !== 'boolean'
			&& typeof this.#options.throwHttpErrors !== 'function'
		) {
			throw new TypeError('The `throwHttpErrors` option must be a boolean or a function');
		}
	}

	#calculateDelay(retry: InternalOptions['retry']): number {
		const retryDelay = retry.delay(this.#retryCount + 1);

		// A `retry.delay` that does not return a usable number would turn every delay into `NaN`, which
		// `setTimeout()` clamps to 1ms, silently disabling the configured backoff. The `jitter` function form
		// already guards its own result, so the input is checked here for every jitter form.
		if (typeof retryDelay !== 'number' || Number.isNaN(retryDelay) || retryDelay < 0) {
			throw new TypeError('`retry.delay` must return a non-negative number or `Infinity`');
		}

		let jitteredDelay = retryDelay;
		if (retry.jitter === true) {
			jitteredDelay = Math.random() * retryDelay;
		} else if (typeof retry.jitter === 'function') {
			jitteredDelay = retry.jitter(retryDelay);

			if (!Number.isFinite(jitteredDelay) || jitteredDelay < 0) {
				jitteredDelay = retryDelay;
			}
		}

		return Math.min(retry.backoffLimit, jitteredDelay);
	}

	async #calculateRetryDelay(error: unknown) {
		if (isError(error) && this.#inFlightCallbackErrors.has(error)) {
			throw error;
		}

		const retry = normalizeRetryOptions(this.#options.retry);
		if (this.#retryCount >= Math.min(retry.limit, this.#retryLimit)) {
			throw error;
		}

		// Wrap non-Error throws to ensure consistent error handling
		const errorObject = error instanceof Error ? error : new NonError(error);

		// Handle forced retry from afterResponse hook - skip method check and shouldRetry
		if (isForceRetryError(errorObject)) {
			return errorObject.customDelay ?? this.#calculateDelay(retry);
		}

		// Check if method is retriable for non-forced retries
		if (!retry.methods.includes(normalizeRetryMethod(this.request.method))) {
			throw error;
		}

		let shouldRetryOverride = false;
		const {shouldRetry} = retry;
		if (shouldRetry !== undefined) {
			const result = await this.#raceWithTotalTimeout(async () => shouldRetry({error: errorObject, retryCount: this.#retryCount + 1}), this.#userProvidedAbortSignal);
			this.#throwIfAbortedByUser();
			if (result === timedOutOperation) {
				throw new TimeoutError(this.request);
			}

			// Only exact booleans override the default retry checks.
			if (result === false) {
				throw error;
			}

			shouldRetryOverride = result === true;
		}

		// Default timeout behavior
		if (isTimeoutError(error)) {
			if (!shouldRetryOverride && !retry.retryOnTimeout) {
				throw error;
			}

			return this.#calculateDelay(retry);
		}

		// The `isHTTPError()` brand only checks `name`, so a cross-realm error may not carry a `response`.
		if (isHTTPError(error) && error.response) {
			if (!shouldRetryOverride && !retry.statusCodes.includes(error.response.status)) {
				throw error;
			}

			const retryTimingHeader = getRetryTimingHeader(error.response.headers);
			if (retryTimingHeader && retry.afterStatusCodes.includes(error.response.status)) {
				const after = calculateRetryTimingDelay(retryTimingHeader);
				if (after === undefined) {
					// Malformed retry timing headers should not disable retries; they only lose their server-provided timing.
					return this.#calculateDelay(retry);
				}

				// Don't apply jitter when server provides explicit retry timing
				return Math.min(retry.maxRetryAfter, after);
			}

			if (!shouldRetryOverride && error.response.status === 413) {
				throw error;
			}

			return this.#calculateDelay(retry);
		}

		// Only retry known retriable error types. Unknown errors (e.g., programming bugs) are not retried.
		if (!shouldRetryOverride && !isNetworkError(error)) {
			throw error;
		}

		return this.#calculateDelay(retry);
	}

	#decorateResponse(response: Response): Response {
		if (!this.#options.parseJson || this.#decoratedResponses.has(response)) {
			return response;
		}

		this.#decoratedResponses.add(response);
		const request = this.#getResponseRequest(response);

		response.json = async () => {
			const text = await response.text();
			return this.#options.parseJson!(text, {request, response});
		};

		// `clone()` returns a fresh `Response` that would otherwise fall back to the native `json()`.
		const nativeClone = response.clone.bind(response);
		response.clone = () => this.#decorateResponse(this.#setResponseRequest(nativeClone(), request));

		return response;
	}

	// Defaults to the last request that was actually sent, because `this.request` is the clone `#fetch()` prepares
	// for a possible retry and may never be sent at all.
	async #throwProcessedError(error: unknown, request: Request = this.#originalRequest ?? this.request): Promise<never> {
		// Non-Error throws (e.g., thrown strings) pass through unchanged. `isError` reads the internal brand, so an
		// error from another realm still runs the hooks, the same way the Ky type guards accept branded errors.
		if (!isError(error)) {
			throw error;
		}

		// Errors thrown by beforeRetry hooks must propagate unchanged.
		if (this.#beforeRetryHookErrors.has(error)) {
			throw error;
		}

		let processedError: Error = error;
		for (const hook of this.#options.hooks.beforeError) {
			// `options` intentionally remains the stable normalized Ky options snapshot for the same reason as `HTTPError` above.
			// eslint-disable-next-line no-await-in-loop
			const hookResult: unknown = await hook({
				request,
				options: this.#getNormalizedOptions(),
				error: processedError,
				retryCount: this.#retryCount,
			});

			// Only overwrite if the hook returns a valid Error, including one from another realm.
			if (isError(hookResult)) {
				processedError = hookResult;
			}
		}

		throw processedError;
	}

	async #getResponseData(response: Response): Promise<unknown> {
		// `request` is the request that actually produced this response, which is not `this.request` once a retry clone has been prepared.
		const request = this.#getResponseRequest(response);

		// Even with request timeouts disabled, bound error-body reads so retries and error propagation
		// cannot be stalled indefinitely by never-ending response streams.
		const readTimeout = this.#getErrorDataTimeout(request);
		const text = await this.#readResponseText(response, readTimeout.milliseconds);
		if (text === timedOutResponseData) {
			if (readTimeout.fromTotalTimeout) {
				throw new TimeoutError(request);
			}

			this.#throwIfTotalTimeoutExhausted(request);
			return undefined;
		}

		if (!text) {
			return undefined;
		}

		if (!this.#isJsonContentType(response.headers.get('content-type') ?? '')) {
			return text;
		}

		const parseTimeout = this.#getErrorDataTimeout(request);
		const data = await this.#parseJson(text, response, parseTimeout.milliseconds, request);
		if (data === timedOutResponseData) {
			if (parseTimeout.fromTotalTimeout) {
				throw new TimeoutError(request);
			}

			this.#throwIfTotalTimeoutExhausted(request);
			return undefined;
		}

		return data;
	}

	#getErrorDataTimeout(request: Request = this.request): ErrorDataTimeout {
		const errorDataTimeout = this.#options.timeout === false ? 10_000 : this.#options.timeout;
		const remainingTotal = this.#getRemainingTotalTimeout();
		if (remainingTotal === undefined) {
			return {
				milliseconds: errorDataTimeout,
				fromTotalTimeout: false,
			};
		}

		if (remainingTotal <= 0) {
			throw new TimeoutError(request);
		}

		return {
			milliseconds: Math.min(errorDataTimeout, remainingTotal),
			fromTotalTimeout: remainingTotal <= errorDataTimeout,
		};
	}

	#getEffectiveTimeout(): number | undefined {
		const remainingTotal = this.#getRemainingTotalTimeout();
		if (remainingTotal !== undefined) {
			if (remainingTotal <= 0) {
				throw new TimeoutError(this.request);
			}

			return this.#options.timeout === false
				? remainingTotal
				: Math.min(this.#options.timeout, remainingTotal);
		}

		return this.#options.timeout === false ? undefined : this.#options.timeout;
	}

	// Unlike error bodies (`#getResponseData`), a successful body read has no fallback value to return -
	// the caller's `.json()`/`.text()`/etc. promise must settle, so a timeout here always rejects.
	async #raceBodyRead(createBodyPromise: () => Promise<unknown>, response: Response): Promise<unknown> {
