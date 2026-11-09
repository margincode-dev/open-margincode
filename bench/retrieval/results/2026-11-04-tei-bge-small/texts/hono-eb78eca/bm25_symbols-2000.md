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
