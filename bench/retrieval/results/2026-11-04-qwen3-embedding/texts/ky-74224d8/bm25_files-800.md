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

