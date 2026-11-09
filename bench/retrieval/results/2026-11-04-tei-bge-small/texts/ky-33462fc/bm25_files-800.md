### source/utils/options.ts
```typescript
import {kyOptionKeys, requestOptionsRegistry} from '../core/constants.js';
import type {SearchParamsOption} from '../types/options.js';
import {deletedParametersSymbol} from './merge.js';

export const findUnknownOptions = (
	options: Record<string, unknown>,
): Record<string, unknown> => {
	const unknownOptions: Record<string, unknown> = {};

	for (const key in options) {
		// Skip inherited properties
		if (!Object.hasOwn(options, key)) {
			continue;
		}

		// Forward every non-standard, non-Ky option to fetch().
		// We intentionally do not check whether the key also exists on `Request`, because some runtimes
		// patch `Request.prototype` with fetch-only extensions. For example, Next.js adds `next`, and the
		// old `key in request` heuristic dropped it unless Ky kept a special-case allowlist.
		// Passing all non-standard keys makes that allowlist unnecessary and preserves future fetch extensions too.
		// The registries are plain objects, so `Object.hasOwn()` is required: `in` would also match
		// `Object.prototype` members such as `constructor` and `toString`, silently dropping those options.
		if (!Object.hasOwn(requestOptionsRegistry, key) && !Object.hasOwn(kyOptionKeys, key)) {
			unknownOptions[key] = options[key];
		}
	}

	return unknownOptions;
};

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
### test/validation.ts
```typescript
import test from 'ava';
import ky from '../source/index.js';

const okFetch = async () => new Response('ok');

// `timeout` and `totalTimeout` are documented as milliseconds or `false`. Anything else used to be passed
// straight to `setTimeout()`, which clamps it to ~1ms, or was silently ignored, so the request never got the
// timeout the caller asked for. `null` is left out: it is nullish, so it selects the default.
test('rejects non-numeric and negative `timeout` values', async t => {
	for (const timeout of ['1000', Number.NaN, -1, -0.5, {}] as unknown[]) {
		// eslint-disable-next-line no-await-in-loop
		await t.throwsAsync(
			ky('https://example.com', {timeout: timeout as never, fetch: okFetch}).text(),
			{
				name: 'TypeError',
				message: 'The `timeout` option must be a non-negative number or `false`',
			},
			`timeout: ${JSON.stringify(timeout)}`,
		);
	}
});

test('rejects non-numeric and negative `totalTimeout` values', async t => {
