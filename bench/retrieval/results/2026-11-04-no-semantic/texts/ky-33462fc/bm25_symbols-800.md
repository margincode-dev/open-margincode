- hasSearchParameters (source/utils/options.ts:31-60)
```typescript
export const hasSearchParameters = (search: SearchParamsOption): boolean => {
	if (search === undefined) {
		return false;
	}

	// Option merging turns `null` into an absent value, but an `init` hook assigns straight onto the options object. `null` is not accepted, so report it instead of treating it as absent.
	if (search === null) {
		throw new TypeError('The `searchParams` option must not be `null`. Use `undefined` to clear it.');
	}

	// The `typeof array` still gives "object", so we need different checking for array.
	if (Array.isArray(search)) {
		return search.length > 0;
	}

	if (search instanceof URLSearchParams) {
		return search.size > 0 || Boolean((search as any)[deletedParametersSymbol]?.size);
	}

	// Record
	if (typeof search === 'object') {
		return Object.keys(search).length > 0;
	}

	if (typeof search === 'string') {
		return search.trim().length > 0;
	}

	return Boolean(search);
};
```
- mergeHooks (source/utils/merge.ts:217-231)
```typescript
export const mergeHooks = (original: Hooks = {}, incoming: Hooks = {}): NormalizedHooks => {
	// `null` selects the default, the same way option merging treats a nullish value as absent.
	const incomingHooks = incoming ?? {};
	if (!isObject(incomingHooks) || Array.isArray(incomingHooks)) {
		throw new TypeError('The `hooks` option must be an object');
	}

	return {
		init: newHookValue(original, incomingHooks, 'init'),
		beforeRequest: newHookValue(original, incomingHooks, 'beforeRequest'),
		beforeRetry: newHookValue(original, incomingHooks, 'beforeRetry'),
		beforeError: newHookValue(original, incomingHooks, 'beforeError'),
		afterResponse: newHookValue(original, incomingHooks, 'afterResponse'),
	};
};
```
- promiseWithTimeout (test/helpers/with-page.ts:19-37)
```typescript
export const promiseWithTimeout = async <Value>(
	promise: Promise<Value>,
	timeoutInMilliseconds: number,
	errorMessage: string,
): Promise<Value> => new Promise((resolve, reject) => {
	const timeoutIdentifier = setTimeout(() => {
		reject(new Error(errorMessage));
	}, timeoutInMilliseconds);

	promise
		.then(value => {
			clearTimeout(timeoutIdentifier);
			resolve(value);
		})
		.catch((error: unknown) => {
			clearTimeout(timeoutIdentifier);
			reject(error instanceof Error ? error : new Error(String(error)));
		});
});
```
- createAbortDuringFetch (test/retry.ts:4040-4056)
```typescript
const createAbortDuringFetch = (onRequest?: () => void): typeof fetch => async input => {
	onRequest?.();
	const {signal} = input as Request;

	return new Promise<Response>((_resolve, reject) => {
		const fail = () => {
			reject(new TypeError('Failed to fetch'));
		};

		if (signal.aborted) {
			fail();
			return;
		}

		signal.addEventListener('abort', fail, {once: true});
	});
};
```
- delay (source/utils/delay.ts:9-29)
```typescript
export default async function delay(
	ms: number,
	{signal}: DelayOptions,
): Promise<void> {
	return new Promise((resolve, reject) => {
		if (signal) {
			signal.throwIfAborted();
			signal.addEventListener('abort', abortHandler, {once: true});
		}

		function abortHandler() {
			clearTimeout(timeoutId);
			reject(signal!.reason as Error);
		}

