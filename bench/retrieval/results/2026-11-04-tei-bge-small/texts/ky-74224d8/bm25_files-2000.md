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

