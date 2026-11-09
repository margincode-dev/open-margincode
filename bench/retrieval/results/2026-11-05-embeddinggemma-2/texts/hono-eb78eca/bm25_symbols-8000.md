- warnIfStorelessAccess (src/jsx/context.ts:73-80)
```typescript
const warnIfStorelessAccess = (): void => {
  if (fallbackRendersInFlight > 0 && !warnedFallbackDefault) {
    warnedFallbackDefault = true
    console.warn(
      'hono/jsx: AsyncLocalStorage is unavailable in this runtime, so useContext() after an await in an async component falls back to the context default value during server-side rendering. To get provided values across await boundaries, use a runtime with AsyncLocalStorage (Node.js >= 20.16, Deno, Bun, or Cloudflare Workers with the nodejs_compat flag).'
    )
  }
}
```
- DetectorOptions (src/middleware/language/language.ts:13-45)
```typescript
export interface DetectorOptions {
  /** Order of language detection strategies */
  order: DetectorType[]
  /** Query parameter name for language */
  lookupQueryString: string
  /** Cookie name for language */
  lookupCookie: string
  /** Index in URL path where language code appears */
  lookupFromPathIndex: number
  /** Header key for language detection */
  lookupFromHeaderKey: string
  /** Caching strategies */
  caches: CacheType[] | false
  /** Cookie configuration options */
  cookieOptions?: {
    domain?: string
    path?: string
    sameSite?: 'Strict' | 'Lax' | 'None'
    secure?: boolean
    maxAge?: number
    httpOnly?: boolean
  }
  /** Whether to ignore case in language codes */
  ignoreCase: boolean
  /** Default language if none detected */
  fallbackLanguage: string
  /** List of supported language codes */
  supportedLanguages: string[]
  /** Optional function to transform detected language codes */
  convertDetectedLanguage?: (lang: string) => string
  /** Enable debug logging */
  debug?: boolean
}
```
- EventV1Processor.getQueryString (adapters/aws-lambda/src/handler.ts:456-472)
```typescript
  protected getQueryString(event: APIGatewayProxyEvent): string {
    // In the case of gateway Integration either queryStringParameters or multiValueQueryStringParameters can be present not both
    // API Gateway passes decoded values, so we need to re-encode them to preserve the original URL
    if (event.multiValueQueryStringParameters) {
      return Object.entries(event.multiValueQueryStringParameters || {})
        .filter(([, value]) => value)
        .map(([key, values]) =>
          values.map((value) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`).join('&')
        )
        .join('&')
    } else {
      return Object.entries(event.queryStringParameters || {})
        .filter(([, value]) => value !== undefined)
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value || '')}`)
        .join('&')
    }
  }
```
- EventV1Processor.getQueryString (src/adapter/aws-lambda/handler.ts:487-503)
```typescript
  protected getQueryString(event: APIGatewayProxyEvent): string {
    // In the case of gateway Integration either queryStringParameters or multiValueQueryStringParameters can be present not both
    // API Gateway passes decoded values, so we need to re-encode them to preserve the original URL
    if (event.multiValueQueryStringParameters) {
      return Object.entries(event.multiValueQueryStringParameters || {})
        .filter(([, value]) => value)
        .map(([key, values]) =>
          values.map((value) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`).join('&')
        )
        .join('&')
    } else {
      return Object.entries(event.queryStringParameters || {})
        .filter(([, value]) => value !== undefined)
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value || '')}`)
        .join('&')
    }
  }
```
- ClientRequestImpl (src/client/client.ts:38-148)
```typescript
class ClientRequestImpl {
  private url: string
  private method: string
  private buildSearchParams: BuildSearchParamsFn
  private queryParams: URLSearchParams | undefined = undefined
  private pathParams: Record<string, string> = {}
  private rBody: BodyInit | undefined
  private cType: string | undefined = undefined

  constructor(
    url: string,
    method: string,
    options: {
      buildSearchParams: BuildSearchParamsFn
    }
  ) {
    this.url = url
    this.method = method
    this.buildSearchParams = options.buildSearchParams
  }
  fetch = async (
    args?: ValidationTargets<FormValue> & {
      param?: Record<string, string>
    },
    opt?: ClientRequestOptions
  ) => {
    if (args) {
      if (args.query) {
        this.queryParams = this.buildSearchParams(args.query)
      }

      if (args.form) {
        const form = new FormData()
        for (const [k, v] of Object.entries(args.form)) {
          if (v === undefined) {
            continue
          }
          if (Array.isArray(v)) {
            for (const v2 of v) {
              if (v2 === undefined) {
                continue
              }
              form.append(k, v2)
            }
          } else {
            form.append(k, v)
          }
        }
        this.rBody = form
      }

      if (args.json !== undefined) {
        this.rBody = JSON.stringify(args.json)
        this.cType = 'application/json'
      }

      if (args.param) {
        this.pathParams = args.param
      }
    }

    let methodUpperCase = this.method.toUpperCase()

    const headerValues: Record<string, string | undefined> = {
      ...args?.header,
      ...(typeof opt?.headers === 'function' ? await opt.headers() : opt?.headers),
    }

    if (args?.cookie) {
      const cookies: string[] = []
      for (const [key, value] of Object.entries(args.cookie)) {
        if (value === undefined) {
          continue
        }
        cookies.push(serialize(key, value))
      }
      if (cookies.length > 0) {
        headerValues['Cookie'] = cookies.join('; ')
      }
    }

    if (this.cType) {
      headerValues['Content-Type'] = this.cType
    }

    const headers = new Headers()
    for (const [key, value] of Object.entries(headerValues)) {
      if (value !== undefined) {
        headers.set(key, value)
      }
    }
    let url = this.url

    url = removeIndexString(url)
    url = replaceUrlParam(url, this.pathParams)

    if (this.queryParams) {
      url = appendQueryParams(url, this.queryParams)
    }
    methodUpperCase = this.method.toUpperCase()
    const setBody = !(methodUpperCase === 'GET' || methodUpperCase === 'HEAD')

    // Pass URL string to 1st arg for testing with MSW and node-fetch
    return (opt?.fetch || fetch)(url, {
      body: setBody ? this.rBody : undefined,
      method: methodUpperCase,
      headers: headers,
      ...opt?.init,
    })
  }
}
```
- createCacheKey (src/middleware/cache/index.ts:51-70)
```typescript
const createCacheKey = (
  key: string,
  requestUrl: string,
  request: CacheKeyRequest,
  varyHeaders: [string, string][]
): string => {
  const url = new URL(cacheKeyPath, requestUrl)
  url.searchParams.append(cacheKeyParameter, key)
  url.searchParams.append(cacheMethodKeyParameter, request.method)

  if (request.method === 'QUERY') {
    url.searchParams.append(queryDigestKeyParameter, request.digest)
  }

  for (const header of varyHeaders) {
    url.searchParams.append(cacheVaryKeyParameter, JSON.stringify(header))
  }

  return url.href
}
```
- createAgent (runtime-tests/node/index.test.ts:275-296)
```typescript
function createAgent(app: Hono) {
  const server = createAdaptorServer(app)
  const listening = once(server.listen(), 'listening')

  return {
    async get(path: string, init?: RequestInit) {
      await listening
      const url = new URL(path, getOrigin())
      return fetch(url, init)
    },
  }

  function getOrigin(): string {
    let address = server.address()

    if (typeof address === 'object') {
      address = address?.port ? `http://localhost:${address.port}` : 'http://localhost'
    }

    return address
  }
}
```
- DEFAULT_OPTIONS (src/middleware/language/language.ts:51-68)
```typescript
export const DEFAULT_OPTIONS: DetectorOptions = {
  order: ['querystring', 'cookie', 'header'],
  lookupQueryString: 'lang',
  lookupCookie: 'language',
  lookupFromHeaderKey: 'accept-language',
  lookupFromPathIndex: 0,
  caches: ['cookie'],
  ignoreCase: true,
  fallbackLanguage: 'en',
  supportedLanguages: ['en'],
  cookieOptions: {
    sameSite: 'Strict',
    secure: true,
    maxAge: 365 * 24 * 60 * 60,
    httpOnly: true,
  },
  debug: false,
}
```
- main (benchmarks/http-server/benchmark.ts:234-324)
```typescript
const main = async () => {
  console.log('🏁 Hono HTTP Benchmark')
  console.log('======================')
  console.log(`Baseline: ${baseline}`)
  console.log(`Target: ${target}`)
  console.log(`Runs: ${runs}`)
  console.log(`Duration: ${duration}s`)
  console.log(`Concurrency: ${concurrency}`)
  console.log(`Skip Tests: ${skipTests}`)
  console.log('')

  setupTemp()

  try {
    // Compare baseline vs target
    const baselinePath = await buildVersion(baseline, 'baseline')
    const targetPath = await buildVersion(target, 'target')

    const baselineResult = await runBenchmark(baselinePath, 'baseline')
    const targetResult = await runBenchmark(targetPath, 'target')

    // Calculate changes
    const calculateChange = (target: number, baseline: number) =>
      (((target - baseline) / baseline) * 100).toFixed(2)

    const changes = {
      average: calculateChange(targetResult.average, baselineResult.average),
      ping: calculateChange(targetResult.ping, baselineResult.ping),
      query: calculateChange(targetResult.query, baselineResult.query),
      body: calculateChange(targetResult.body, baselineResult.body),
    }

    // Format numbers
    const format = (num: number) => num.toFixed(2).replace(/\B(?=(\d{3})+(?!\d))/g, ',')
    const formatChange = (change: string) => (Number(change) >= 0 ? '+' : '') + change + '%'

    // Generate table data
    const rows = [
      {
        framework: `hono (${baseline})`,
        runtime: 'bun',
        average: format(baselineResult.average),
        ping: format(baselineResult.ping),
        query: format(baselineResult.query),
        body: format(baselineResult.body),
      },
      {
        framework: `hono (${target})`,
        runtime: 'bun',
        average: format(targetResult.average),
        ping: format(targetResult.ping),
        query: format(targetResult.query),
        body: format(targetResult.body),
      },
      {
        framework: 'Change',
        runtime: '',
        average: formatChange(changes.average),
        ping: formatChange(changes.ping),
        query: formatChange(changes.query),
        body: formatChange(changes.body),
      },
    ]

    const table = [
      '| Framework | Runtime | Average | Ping | Query | Body |',
      '| --- | --- | --- | --- | --- | --- |',
      ...rows.map(
        (row) =>
          `| ${row.framework} | ${row.runtime} | ${row.average} | ${row.ping} | ${row.query} | ${row.body} |`
      ),
    ]

    // Console output
    console.log('')
    table.forEach((line) => console.log(line))
    console.log('')

    // Markdown output
    const markdownOutput = ['## HTTP Performance Benchmark', '', ...table].join('\n')

    writeFileSync(join(SCRIPT_DIR, 'benchmark-results.md'), markdownOutput)
  } catch (error) {
    console.error('❌ Benchmark failed:', error)
    throw error
  } finally {
    if (existsSync(TEMP_DIR)) {
      rmSync(TEMP_DIR, { recursive: true })
    }
  }
}
```
- handleParsingAllValues (src/utils/body.ts:198-216)
```typescript
const handleParsingAllValues = (
  form: BodyData<{ all: true }>,
  key: string,
  value: FormDataEntryValue
): void => {
  if (form[key] !== undefined) {
    if (Array.isArray(form[key])) {
      ;(form[key] as (string | File)[]).push(value)
    } else {
      form[key] = [form[key] as string | File, value]
    }
  } else {
    if (!key.endsWith('[]')) {
      form[key] = value
    } else {
      form[key] = [value]
    }
  }
}
```
- _getQueryParam (src/utils/url.ts:232-318)
```typescript
const _getQueryParam = (
  url: string,
  key?: string,
  multiple?: boolean
): string | undefined | Record<string, string> | string[] | Record<string, string[]> => {
  const hashIndex = url.indexOf('#', 8)
  if (hashIndex !== -1) {
    url = url.slice(0, hashIndex)
  }

  let encoded

  if (!multiple && key && key.indexOf('%') === -1 && key.indexOf('+') === -1) {
    // optimized for unencoded key

    let keyIndex = url.indexOf('?', 8)
    if (keyIndex === -1) {
      return undefined
    }
    if (!url.startsWith(key, keyIndex + 1)) {
      keyIndex = url.indexOf(`&${key}`, keyIndex + 1)
    }
    while (keyIndex !== -1) {
      const trailingKeyCode = url.charCodeAt(keyIndex + key.length + 1)
      if (trailingKeyCode === 61) {
        const valueIndex = keyIndex + key.length + 2
        const endIndex = url.indexOf('&', valueIndex)
        return _decodeURI(url.slice(valueIndex, endIndex === -1 ? undefined : endIndex))
      } else if (trailingKeyCode == 38 || isNaN(trailingKeyCode)) {
        return ''
      }
      keyIndex = url.indexOf(`&${key}`, keyIndex + 1)
    }

    encoded = /[%+]/.test(url)
    if (!encoded) {
      return undefined
    }
    // fallback to default routine
  }

  const results: Record<string, string> | Record<string, string[]> = Object.create(null)
  encoded ??= /[%+]/.test(url)

  let keyIndex = url.indexOf('?', 8)
  while (keyIndex !== -1) {
    const nextKeyIndex = url.indexOf('&', keyIndex + 1)
    let valueIndex = url.indexOf('=', keyIndex)
    if (valueIndex > nextKeyIndex && nextKeyIndex !== -1) {
      valueIndex = -1
    }
    let name = url.slice(
      keyIndex + 1,
      valueIndex === -1 ? (nextKeyIndex === -1 ? undefined : nextKeyIndex) : valueIndex
    )
    if (encoded) {
      name = _decodeURI(name)
    }

    keyIndex = nextKeyIndex

    if (name === '') {
      continue
    }

    let value
    if (valueIndex === -1) {
      value = ''
    } else {
      value = url.slice(valueIndex + 1, nextKeyIndex === -1 ? undefined : nextKeyIndex)
      if (encoded) {
        value = _decodeURI(value)
      }
    }

    if (multiple) {
      if (!(results[name] && Array.isArray(results[name]))) {
        results[name] = []
      }
      ;(results[name] as string[]).push(value)
    } else {
      results[name] ??= value
    }
  }

  return key ? results[key] : results
}
```
- EventV1Processor (adapters/aws-lambda/src/handler.ts:447-503)
```typescript
export class EventV1Processor extends EventProcessor<APIGatewayProxyEvent> {
  protected getPath(event: APIGatewayProxyEvent): string {
    return event.path
  }

  protected getMethod(event: APIGatewayProxyEvent): string {
    return event.httpMethod
  }

  protected getQueryString(event: APIGatewayProxyEvent): string {
    // In the case of gateway Integration either queryStringParameters or multiValueQueryStringParameters can be present not both
    // API Gateway passes decoded values, so we need to re-encode them to preserve the original URL
    if (event.multiValueQueryStringParameters) {
      return Object.entries(event.multiValueQueryStringParameters || {})
        .filter(([, value]) => value)
        .map(([key, values]) =>
          values.map((value) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`).join('&')
        )
        .join('&')
    } else {
      return Object.entries(event.queryStringParameters || {})
        .filter(([, value]) => value !== undefined)
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value || '')}`)
        .join('&')
    }
  }

  protected getCookies(_event: APIGatewayProxyEvent, _headers: Headers): void {
    // nop
  }

  protected getHeaders(event: APIGatewayProxyEvent): Headers {
    const headers = new Headers()
    this.getCookies(event, headers)
    if (event.multiValueHeaders) {
      for (const [k, values] of Object.entries(event.multiValueHeaders)) {
        if (values) {
          values.forEach((v) => headers.append(k, sanitizeHeaderValue(v)))
        }
      }
    }
    if (event.headers) {
      for (const [k, v] of Object.entries(event.headers)) {
        if (v && !headers.has(k)) {
          headers.set(k, sanitizeHeaderValue(v))
        }
      }
    }
    return headers
  }

  protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
    result.multiValueHeaders = {
      'set-cookie': cookies,
    }
  }
}
```
- EventV1Processor (src/adapter/aws-lambda/handler.ts:478-534)
```typescript
export class EventV1Processor extends EventProcessor<APIGatewayProxyEvent> {
  protected getPath(event: APIGatewayProxyEvent): string {
    return event.path
  }

  protected getMethod(event: APIGatewayProxyEvent): string {
    return event.httpMethod
  }

  protected getQueryString(event: APIGatewayProxyEvent): string {
    // In the case of gateway Integration either queryStringParameters or multiValueQueryStringParameters can be present not both
    // API Gateway passes decoded values, so we need to re-encode them to preserve the original URL
    if (event.multiValueQueryStringParameters) {
      return Object.entries(event.multiValueQueryStringParameters || {})
        .filter(([, value]) => value)
        .map(([key, values]) =>
          values.map((value) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`).join('&')
        )
        .join('&')
    } else {
      return Object.entries(event.queryStringParameters || {})
        .filter(([, value]) => value !== undefined)
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value || '')}`)
        .join('&')
    }
  }

  protected getCookies(_event: APIGatewayProxyEvent, _headers: Headers): void {
    // nop
  }

  protected getHeaders(event: APIGatewayProxyEvent): Headers {
    const headers = new Headers()
    this.getCookies(event, headers)
    if (event.multiValueHeaders) {
      for (const [k, values] of Object.entries(event.multiValueHeaders)) {
        if (values) {
          values.forEach((v) => headers.append(k, sanitizeHeaderValue(v)))
        }
      }
    }
    if (event.headers) {
      for (const [k, v] of Object.entries(event.headers)) {
        if (v && !headers.has(k)) {
          headers.set(k, sanitizeHeaderValue(v))
        }
      }
    }
    return headers
  }

  protected setCookiesToResult(result: APIGatewayProxyResult, cookies: string[]): void {
    result.multiValueHeaders = {
      'set-cookie': cookies,
    }
  }
}
```
- Runtime (src/helper/adapter/index.ts:8-8)
```typescript
export type Runtime = 'node' | 'deno' | 'bun' | 'workerd' | 'fastly' | 'edge-light' | 'other'
```
- convertFormDataToBodyData (src/utils/body.ts:160-189)
```typescript
function convertFormDataToBodyData<T extends BodyData = BodyData>(
  formData: FormData,
  options: ParseBodyOptions
): T {
  const form: BodyData = Object.create(null)
  const nestingState = { count: 0 }

  formData.forEach((value, key) => {
    const shouldParseAllValues = options.all || key.endsWith('[]')

    if (!shouldParseAllValues) {
      form[key] = value
    } else {
      handleParsingAllValues(form, key, value)
    }
  })

  if (options.dot) {
    Object.entries(form).forEach(([key, value]) => {
      const shouldParseDotValues = key.includes('.')

      if (shouldParseDotValues) {
        handleParsingNestedValues(form, key, value, nestingState)
        delete form[key]
      }
    })
  }

  return form as T
}
```
- ALBProxyEvent (adapters/aws-lambda/src/handler.ts:86-98)
```typescript
export interface ALBProxyEvent {
  httpMethod: string
  headers?: Record<string, string | undefined>
  multiValueHeaders?: Record<string, string[] | undefined>
  path: string
  body: string | null
  isBase64Encoded: boolean
  queryStringParameters?: Record<string, string | undefined>
  multiValueQueryStringParameters?: {
    [parameterKey: string]: string[]
  }
  requestContext: ALBRequestContext
}
```
- ALBProxyEvent (src/adapter/aws-lambda/handler.ts:101-113)
```typescript
export interface ALBProxyEvent {
  httpMethod: string
  headers?: Record<string, string | undefined>
  multiValueHeaders?: Record<string, string[] | undefined>
  path: string
  body: string | null
  isBase64Encoded: boolean
  queryStringParameters?: Record<string, string | undefined>
  multiValueQueryStringParameters?: {
    [parameterKey: string]: string[]
  }
  requestContext: ALBRequestContext
}
```
- HonoRequest.query (src/request.ts:147-149)
```typescript
  query(key?: string) {
    return getQueryParam(this.url, key)
  }
```
- HonoRequest (src/request.ts:34-448)
```typescript
export class HonoRequest<P extends string = '/', I extends Input['out'] = {}> {
  /**
   * `.raw` can get the raw Request object.
   *
   * @see {@link https://hono.dev/docs/api/request#raw}
   *
   * @example
   * ```ts
   * // For Cloudflare Workers
   * app.post('/', async (c) => {
   *   const metadata = c.req.raw.cf?.hostMetadata?
   *   ...
   * })
   * ```
   */
  raw: Request

  #validatedData: { [K in keyof ValidationTargets]?: {} } | undefined // Short name of validatedData
  #matchResult: Result<[unknown, RouterRoute]>
  routeIndex: number = 0
  /**
   * `.path` can get the pathname of the request.
   *
   * @see {@link https://hono.dev/docs/api/request#path}
   *
   * @example
   * ```ts
   * app.get('/about/me', (c) => {
   *   const pathname = c.req.path // `/about/me`
   * })
   * ```
   */
  path: string
  bodyCache: BodyCache = {}

  constructor(
    request: Request,
    path: string = '/',
    matchResult: Result<[unknown, RouterRoute]> = [[]]
  ) {
    this.raw = request
    this.path = path
    this.#matchResult = matchResult
  }

  /**
   * `.req.param()` gets the path parameters.
   *
   * @see {@link https://hono.dev/docs/api/routing#path-parameter}
   *
   * @example
   * ```ts
   * const name = c.req.param('name')
   * // or all parameters at once
   * const { id, comment_id } = c.req.param()
   * ```
   */
  param<P2 extends ParamKeys<P> = ParamKeys<P>>(
    key: string extends P ? never : P2 extends `${infer _}?` ? never : P2
  ): string
  param<P2 extends RemoveQuestion<ParamKeys<P>> = RemoveQuestion<ParamKeys<P>>>(
    key: P2
  ): string | undefined
  param(key: string): string | undefined
  param<P2 extends string = P>(): Simplify<UnionToIntersection<ParamKeyToRecord<ParamKeys<P2>>>>
  param(key?: string): unknown {
    return key ? this.#getDecodedParam(key) : this.#getAllDecodedParams()
  }

  #getDecodedParam(key: string): string | undefined {
    const paramKey = this.#matchResult[0][this.routeIndex]?.[1][key]
    const param = this.#getParamValue(paramKey)
    return param && tryDecodeURIComponent(param)
  }

  #getAllDecodedParams(): Record<string, string> {
    const decoded: Record<string, string> = {}

    const keys = Object.keys(this.#matchResult[0][this.routeIndex]?.[1] ?? {})
    for (const key of keys) {
      const value = this.#getParamValue(this.#matchResult[0][this.routeIndex][1][key])
      if (value !== undefined) {
        decoded[key] = tryDecodeURIComponent(value)
      }
    }

    return decoded
  }

  #getParamValue(paramKey: any): string | undefined {
    return this.#matchResult[1] ? this.#matchResult[1][paramKey as any] : paramKey
  }

  /**
   * `.query()` can get querystring parameters.
   *
   * @see {@link https://hono.dev/docs/api/request#query}
   *
   * @example
   * ```ts
   * // Query params
   * app.get('/search', (c) => {
   *   const query = c.req.query('q')
   * })
   *
   * // Get all params at once
   * app.get('/search', (c) => {
   *   const { q, limit, offset } = c.req.query()
   * })
   * ```
   */
  query(key: string): string | undefined
  query(): Record<string, string>
  query(key?: string) {
    return getQueryParam(this.url, key)
  }

  /**
   * `.queries()` can get multiple querystring parameter values, e.g. /search?tags=A&tags=B
   *
   * @see {@link https://hono.dev/docs/api/request#queries}
   *
   * @example
   * ```ts
   * app.get('/search', (c) => {
   *   // tags will be string[]
   *   const tags = c.req.queries('tags')
   * })
   * ```
   */
  queries(key: string): string[] | undefined
  queries(): Record<string, string[]>
  queries(key?: string) {
    return getQueryParams(this.url, key)
  }

  /**
   * `.header()` can get the request header value.
   *
   * @see {@link https://hono.dev/docs/api/request#header}
   *
   * @example
   * ```ts
   * app.get('/', (c) => {
   *   const userAgent = c.req.header('User-Agent')
   * })
   * ```
   */
  header(name: RequestHeader): string | undefined
  header(name: string): string | undefined
  header(): Record<RequestHeader | (string & CustomHeader), string>
  header(name?: string) {
    if (name) {
      return this.raw.headers.get(name) ?? undefined
    }

    const headerData: Record<string, string | undefined> = Object.create(null)
    this.raw.headers.forEach((value, key) => {
      headerData[key] = value
    })
    return headerData
  }

  /**
   * `.parseBody()` can parse Request body of type `multipart/form-data` or `application/x-www-form-urlencoded`
   *
   * @see {@link https://hono.dev/docs/api/request#parsebody}
   *
   * @example
   * ```ts
   * app.post('/entry', async (c) => {
   *   const body = await c.req.parseBody()
   * })
   * ```
   */
  async parseBody<Options extends Partial<ParseBodyOptions>, T extends BodyData<Options>>(
    options?: Options
  ): Promise<T>
  async parseBody<T extends BodyData>(options?: Partial<ParseBodyOptions>): Promise<T>
  async parseBody(options?: Partial<ParseBodyOptions>) {
    return parseBody(this, options)
  }

  #cachedBody = (key: keyof Body) => {
    const { bodyCache, raw } = this
    const cachedBody = bodyCache[key]

    if (cachedBody) {
      return cachedBody
    }

    for (const anyCachedKey in bodyCache) {
      return (bodyCache[anyCachedKey as keyof Body] as Promise<BodyInit>).then((body) => {
        if (anyCachedKey === 'json') {
          body = JSON.stringify(body)
        }
        // Rebuilding the body through a bare `Response` loses the request's media
        // type, so a representation that needs it (e.g. `formData()`) can no longer
        // be produced even though the bytes are still available. Carry the original
        // `Content-Type` over, except for `FormData`, where `Response` must generate
        // a fresh multipart boundary of its own.
        const contentType =
          anyCachedKey === 'formData' ? undefined : raw.headers.get('content-type')
        return new Response(body, {
          headers: contentType ? { 'Content-Type': contentType } : undefined,
        })[key]()
      })
    }

    return (bodyCache[key] = raw[key]())
  }

  /**
   * `.json()` can parse Request body of type `application/json`
   *
   * @see {@link https://hono.dev/docs/api/request#json}
   *
   * @example
   * ```ts
   * app.post('/entry', async (c) => {
   *   const body = await c.req.json()
   * })
   * ```
   */
  json<T = any>(): Promise<T> {
    return this.#cachedBody('text').then((text: string) => JSON.parse(text))
  }

  /**
   * `.text()` can parse Request body of type `text/plain`
   *
   * @see {@link https://hono.dev/docs/api/request#text}
   *
   * @example
   * ```ts
   * app.post('/entry', async (c) => {
   *   const body = await c.req.text()
   * })
   * ```
   */
  text(): Promise<string> {
    return this.#cachedBody('text')
  }

  /**
   * `.arrayBuffer()` parse Request body as an `ArrayBuffer`
   *
   * @see {@link https://hono.dev/docs/api/request#arraybuffer}
   *
   * @example
   * ```ts
   * app.post('/entry', async (c) => {
   *   const body = await c.req.arrayBuffer()
   * })
   * ```
   */
  arrayBuffer(): Promise<ArrayBuffer> {
    return this.#cachedBody('arrayBuffer')
  }

  /**
   * `.bytes()` parses the request body as a `Uint8Array`.
   *
   * @see {@link https://hono.dev/docs/api/request#bytes}
   *
   * @example
   * ```ts
   * app.post('/entry', async (c) => {
   *   const body = await c.req.bytes()
   * })
   * ```
   */
  bytes(): Promise<Uint8Array> {
    return this.#cachedBody('arrayBuffer').then((buffer: ArrayBuffer) => new Uint8Array(buffer))
  }

  /**
   * Parses the request body as a `Blob`.
   * @example
   * ```ts
   * app.post('/entry', async (c) => {
   *   const body = await c.req.blob();
   * });
   * ```
   * @see https://hono.dev/docs/api/request#blob
   */
  blob(): Promise<Blob> {
    return this.#cachedBody('blob')
  }

  /**
   * Parses the request body as `FormData`.
   * @example
   * ```ts
   * app.post('/entry', async (c) => {
   *   const body = await c.req.formData();
   * });
   * ```
   * @see https://hono.dev/docs/api/request#formdata
   */
  formData(): Promise<FormData> {
    return this.#cachedBody('formData')
  }

  /**
   * Adds validated data to the request.
   *
   * @param target - The target of the validation.
   * @param data - The validated data to add.
   */
  addValidatedData(target: keyof ValidationTargets, data: {}) {
    ;(this.#validatedData ??= {})[target] = data
  }

  /**
   * Gets validated data from the request.
   *
   * @param target - The target of the validation.
   * @returns The validated data.
   *
   * @see https://hono.dev/docs/api/request#valid
   */
  valid<T extends keyof I & keyof ValidationTargets>(target: T): InputToDataByTarget<I, T>
  valid(target: keyof ValidationTargets) {
    return this.#validatedData?.[target] as unknown
  }

  /**
   * `.url` can get the request url strings.
   *
   * @see {@link https://hono.dev/docs/api/request#url}
   *
   * @example
   * ```ts
   * app.get('/about/me', (c) => {
   *   const url = c.req.url // `http://localhost:8787/about/me`
   *   ...
   * })
   * ```
   */
  get url(): string {
    return this.raw.url
  }

  /**
   * `.method` can get the method name of the request.
   *
   * @see {@link https://hono.dev/docs/api/request#method}
   *
   * @example
   * ```ts
   * app.get('/about/me', (c) => {
   *   const method = c.req.method // `GET`
   * })
   * ```
   */
  get method(): string {
    return this.raw.method
  }

  get [GET_MATCH_RESULT](): Result<[unknown, RouterRoute]> {
    return this.#matchResult
  }

  /**
   * `.matchedRoutes` can return a matched route in the handler
   *
   * @deprecated
   *
   * Use matchedRoutes helper defined in "hono/route" instead.
   *
   * @see {@link https://hono.dev/docs/api/request#matchedroutes}
   *
   * @example
   * ```ts
   * app.use('*', async function logger(c, next) {
   *   await next()
   *   c.req.matchedRoutes.forEach(({ handler, method, path }, i) => {
   *     const name = handler.name || (handler.length < 2 ? '[handler]' : '[middleware]')
   *     console.log(
   *       method,
   *       ' ',
   *       path,
   *       ' '.repeat(Math.max(10 - path.length, 0)),
   *       name,
   *       i === c.req.routeIndex ? '<- respond from here' : ''
   *     )
   *   })
   * })
   * ```
   */
  get matchedRoutes(): RouterRoute[] {
    return this.#matchResult[0].map(([[, route]]) => route)
  }

  /**
   * `.routePath` can retrieve the path registered within the handler
   *
   * @deprecated
   *
   * Use routePath helper defined in "hono/route" instead.
   *
   * @see {@link https://hono.dev/docs/api/request#routepath}
   *
   * @example
   * ```ts
   * app.get('/posts/:id', (c) => {
   *   return c.json({ path: c.req.routePath })
   * })
   * ```
   */
  get routePath(): string {
    return this.#matchResult[0].map(([[, route]]) => route)[this.routeIndex].path
  }
}
```
