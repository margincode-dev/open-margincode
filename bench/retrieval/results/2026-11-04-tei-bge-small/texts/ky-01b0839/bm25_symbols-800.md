- cloneShallow (source/utils/merge.ts:91-113)
```typescript
export const cloneShallow = <T>(value: T): T => {
	if (value instanceof URLSearchParams) {
		const copy = new URLSearchParams(value) as URLSearchParams & {[deletedParametersSymbol]?: Set<string>};
		const deleted = (value as URLSearchParams & {[deletedParametersSymbol]?: Set<string>})[deletedParametersSymbol];
		if (deleted) {
			// Preserve internal deletion markers so init-hook cloning does not resurrect params removed during option merging.
			copy[deletedParametersSymbol] = new Set(deleted);
		}

		return copy as T;
	}

	if (Array.isArray(value)) {
		return [...value] as T;
	}

	if (isPlainObject(value)) {
		const copy = {...value};
		return copy as T;
	}

	return value;
};
```
- createInstance (source/index.ts:10-33)
```typescript
const createInstance = (defaults?: Partial<Options>): KyInstance => {
	// eslint-disable-next-line @typescript-eslint/promise-function-async
	const ky: Partial<Mutable<KyInstance>> = (input: Input, options?: Options) => Ky.create(input, validateAndMerge(defaults, options));

	for (const method of requestMethods) {
		// eslint-disable-next-line @typescript-eslint/promise-function-async
		ky[method] = (input: Input, options?: Options) => Ky.create(input, validateAndMerge(defaults, options, {method}));
	}

	ky.create = (newDefaults?: Partial<Options>) => createInstance(validateAndMerge(newDefaults));
	ky.extend = (newDefaults?: Partial<Options> | ((parentDefaults: Partial<Options>) => Partial<Options>)) => {
		if (typeof newDefaults === 'function') {
			// Pass a deep copy so mutations inside the callback cannot leak into this instance's defaults. Deep, because options like `context` are merged shallowly, so a nested value would still point at the parent's object.
			newDefaults = newDefaults(validateAndMerge(cloneDeep(defaults)));
		}

		return createInstance(validateAndMerge(defaults, newDefaults));
	};

	ky.stop = stop;
	ky.retry = retry;

	return ky as KyInstance;
};
```
- RetryMarker (source/core/constants.ts:157-163)
```typescript
export class RetryMarker {
	options: ForceRetryOptions | undefined;

	constructor(options?: ForceRetryOptions) {
		this.options = options;
	}
}
```
- KyInstance (source/types/ky.ts:5-204)
```typescript
export type KyInstance = {
	/**
	Fetch the given `url`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` method added.

	@example
	```
	import ky from 'ky';

	const json = await ky('https://example.com', {json: {foo: true}}).json();

	console.log(json);
	//=> `{data: '🦄'}`
	```
	*/
	<T>(url: Input, options?: Options): ResponsePromise<T>;

	/**
	Fetch the given `url` using the option `{method: 'get'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
	get: <T>(url: Input, options?: Options) => ResponsePromise<T>;

	/**
	Fetch the given `url` using the option `{method: 'post'}`.

	@param url - `Request` object, `URL` object, or URL string.
	@returns A promise with `Body` methods added.
	*/
