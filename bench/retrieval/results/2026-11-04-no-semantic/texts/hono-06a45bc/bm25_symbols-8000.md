- CustomizedErrorResponseOptions (src/middleware/bearer-auth/index.ts:17-20)
```typescript
type CustomizedErrorResponseOptions = {
  wwwAuthenticateHeader?: string | object | MessageFunction
  message?: string | object | MessageFunction
}
```
- bearerAuth (src/middleware/bearer-auth/index.ts:104-221)
```typescript
export const bearerAuth = <E extends Env = Env>(
  options: BearerAuthOptions<E>
): MiddlewareHandler<E> => {
  if (!('token' in options || 'verifyToken' in options)) {
    throw new Error('bearer auth middleware requires options for "token" or "verifyToken"')
  }
  if (!options.realm) {
    options.realm = ''
  }
  if (options.prefix === undefined) {
    options.prefix = PREFIX
  }

  const realm = options.realm?.replace(/"/g, '\\"')
  const prefix = options.prefix
  const tokenRegexp = new RegExp(`^${TOKEN_STRINGS}$`)
  const wwwAuthenticatePrefix = prefix === '' ? '' : `${prefix} `

  const throwHTTPException = async (
    c: Context,
    status: ContentfulStatusCode,
    wwwAuthenticateHeader: string | object | MessageFunction,
    messageOption: string | object | MessageFunction
  ): Promise<Response> => {
    const wwwAuthenticateHeaderValue: string | object =
      typeof wwwAuthenticateHeader === 'function'
        ? await wwwAuthenticateHeader(c)
        : wwwAuthenticateHeader

    const headers = {
      'WWW-Authenticate':
        typeof wwwAuthenticateHeaderValue === 'string'
          ? wwwAuthenticateHeaderValue
          : `${wwwAuthenticatePrefix}${Object.entries(wwwAuthenticateHeaderValue)
              .map(([key, value]) => `${key}="${value}"`)
              .join(',')}`,
    }
    const responseMessage =
      typeof messageOption === 'function' ? await messageOption(c) : messageOption
    const res =
      typeof responseMessage === 'string'
        ? new Response(responseMessage, { status, headers })
        : new Response(JSON.stringify(responseMessage), {
            status,
            headers: {
              ...headers,
              'content-type': 'application/json',
            },
          })
    throw new HTTPException(status, { res })
  }

  return async function bearerAuth(c, next) {
    const headerToken = c.req.header(options.headerName || HEADER)
    if (!headerToken) {
      // No Authorization header
      await throwHTTPException(
        c,
        401,
        options.noAuthenticationHeader?.wwwAuthenticateHeader ||
          `${wwwAuthenticatePrefix}realm="${realm}"`,
        options.noAuthenticationHeader?.message ||
          options.noAuthenticationHeaderMessage ||
          'Unauthorized'
      )
    } else {
      let tokenValue: string | undefined

      if (prefix === '') {
        tokenValue = headerToken
      } else {
        const headerLower = headerToken.toLowerCase()
        const prefixLower = prefix.toLowerCase()
        if (headerLower.startsWith(prefixLower) && headerToken[prefix.length] === ' ') {
          tokenValue = headerToken.slice(prefix.length).trimStart()
        }
      }

      if (!tokenValue || !tokenRegexp.test(tokenValue)) {
        // Invalid Request
        await throwHTTPException(
          c,
          400,
          options.invalidAuthenticationHeader?.wwwAuthenticateHeader ||
            `${wwwAuthenticatePrefix}error="invalid_request"`,
          options.invalidAuthenticationHeader?.message ||
            options.invalidAuthenticationHeaderMessage ||
            'Bad Request'
        )
      } else {
        let equal = false
        if ('verifyToken' in options) {
          equal = await options.verifyToken(tokenValue, c)
        } else if (typeof options.token === 'string') {
          equal = await timingSafeEqual(options.token, tokenValue, options.hashFunction)
        } else if (Array.isArray(options.token) && options.token.length > 0) {
          for (const token of options.token) {
            if (await timingSafeEqual(token, tokenValue, options.hashFunction)) {
              equal = true
              break
            }
          }
        }
        if (!equal) {
          // Invalid Token
          await throwHTTPException(
            c,
            401,
            options.invalidToken?.wwwAuthenticateHeader ||
              `${wwwAuthenticatePrefix}error="invalid_token"`,
            options.invalidToken?.message || options.invalidTokenMessage || 'Unauthorized'
          )
        }
      }
    }
    await next()
  }
}
```
- MessageFunction (src/middleware/bearer-auth/index.ts:16-16)
```typescript
type MessageFunction = (c: Context) => string | object | Promise<string | object>
```
- BearerAuthOptions (src/middleware/bearer-auth/index.ts:22-66)
```typescript
type BearerAuthOptions<E extends Env = Env> =
  | {
      token: string | string[]
      realm?: string
      prefix?: string
      headerName?: string
      hashFunction?: Function
      /**
       * @deprecated Use noAuthenticationHeader.message instead
       */
      noAuthenticationHeaderMessage?: string | object | MessageFunction
      noAuthenticationHeader?: CustomizedErrorResponseOptions
      /**
       * @deprecated Use invalidAuthenticationHeader.message instead
       */
      invalidAuthenticationHeaderMessage?: string | object | MessageFunction
      invalidAuthenticationHeader?: CustomizedErrorResponseOptions
      /**
       * @deprecated Use invalidToken.message instead
       */
      invalidTokenMessage?: string | object | MessageFunction
      invalidToken?: CustomizedErrorResponseOptions
    }
  | {
      realm?: string
      prefix?: string
      headerName?: string
      verifyToken: (token: string, c: Context<E>) => boolean | Promise<boolean>
      hashFunction?: Function
      /**
       * @deprecated Use noAuthenticationHeader.message instead
       */
      noAuthenticationHeaderMessage?: string | object | MessageFunction
      noAuthenticationHeader?: CustomizedErrorResponseOptions
      /**
       * @deprecated Use invalidAuthenticationHeader.message instead
       */
      invalidAuthenticationHeaderMessage?: string | object | MessageFunction
      invalidAuthenticationHeader?: CustomizedErrorResponseOptions
      /**
       * @deprecated Use invalidToken.message instead
       */
      invalidTokenMessage?: string | object | MessageFunction
      invalidToken?: CustomizedErrorResponseOptions
    }
```
- basicAuth (src/middleware/basic-auth/index.ts:80-153)
```typescript
export const basicAuth = (
  options: BasicAuthOptions,
  ...users: { username: string; password: string }[]
): MiddlewareHandler => {
  const usernamePasswordInOptions = 'username' in options && 'password' in options
  const verifyUserInOptions = 'verifyUser' in options

  if (!(usernamePasswordInOptions || verifyUserInOptions)) {
    throw new Error(
      'basic auth middleware requires options for "username and password" or "verifyUser"'
    )
  }

  if (!options.realm) {
    options.realm = 'Secure Area'
  }

  if (!options.invalidUserMessage) {
    options.invalidUserMessage = 'Unauthorized'
  }

  if (usernamePasswordInOptions) {
    users.unshift({ username: options.username, password: options.password })
  }

  return async function basicAuth(ctx, next) {
    const requestUser = auth(ctx.req.raw)
    if (requestUser) {
      if (verifyUserInOptions) {
        if (await options.verifyUser(requestUser.username, requestUser.password, ctx)) {
          if (options.onAuthSuccess) {
            await options.onAuthSuccess(ctx, requestUser.username)
          }
          await next()
          return
        }
      } else {
        for (const user of users) {
          const [usernameEqual, passwordEqual] = await Promise.all([
            timingSafeEqual(user.username, requestUser.username, options.hashFunction),
            timingSafeEqual(user.password, requestUser.password, options.hashFunction),
          ])
          if (usernameEqual && passwordEqual) {
            if (options.onAuthSuccess) {
              await options.onAuthSuccess(ctx, requestUser.username)
            }
            await next()
            return
          }
        }
      }
    }
    // Invalid user.
    const status = 401
    const headers = {
      'WWW-Authenticate': 'Basic realm="' + options.realm?.replace(/"/g, '\\"') + '"',
    }
    const responseMessage =
      typeof options.invalidUserMessage === 'function'
        ? await options.invalidUserMessage(ctx)
        : options.invalidUserMessage
    const res =
      typeof responseMessage === 'string'
        ? new Response(responseMessage, { status, headers })
        : new Response(JSON.stringify(responseMessage), {
            status,
            headers: {
              ...headers,
              'content-type': 'application/json',
            },
          })
    throw new HTTPException(status, { res })
  }
}
```
- unauthorizedResponse (src/middleware/jwk/jwk.ts:176-196)
```typescript
function unauthorizedResponse(opts: {
  ctx: Context
  error: string
  errDescription: string
  statusText?: string
  realm?: string
}) {
  // `realm` and `error_description` end up inside a quoted-string, so any `"`
  // they contain has to be escaped or the header is cut short. Same handling as
  // the bearer-auth middleware.
  const realm = (opts.realm ?? opts.ctx.req.url).replace(/"/g, '\\"')
  const errDescription = opts.errDescription.replace(/"/g, '\\"')

  return new Response('Unauthorized', {
    status: 401,
    statusText: opts.statusText,
    headers: {
      'WWW-Authenticate': `Bearer realm="${realm}",error="${opts.error}",error_description="${errDescription}"`,
    },
  })
}
```
- unauthorizedResponse (src/middleware/jwt/jwt.ts:165-185)
```typescript
function unauthorizedResponse(opts: {
  ctx: Context
  error: string
  errDescription: string
  statusText?: string
  realm?: string
}) {
  // `realm` and `error_description` end up inside a quoted-string, so any `"`
  // they contain has to be escaped or the header is cut short. Same handling as
  // the bearer-auth middleware.
  const realm = (opts.realm ?? opts.ctx.req.url).replace(/"/g, '\\"')
  const errDescription = opts.errDescription.replace(/"/g, '\\"')

  return new Response('Unauthorized', {
    status: 401,
    statusText: opts.statusText,
    headers: {
      'WWW-Authenticate': `Bearer realm="${realm}",error="${opts.error}",error_description="${errDescription}"`,
    },
  })
}
```
- MessageFunction (src/middleware/basic-auth/index.ts:12-12)
```typescript
type MessageFunction = (c: Context) => string | object | Promise<string | object>
```
- BasicAuthOptions (src/middleware/basic-auth/index.ts:14-29)
```typescript
type BasicAuthOptions =
  | {
      username: string
      password: string
      realm?: string
      hashFunction?: Function
      invalidUserMessage?: string | object | MessageFunction
      onAuthSuccess?: (c: Context, username: string) => void | Promise<void>
    }
  | {
      verifyUser: (username: string, password: string, c: Context) => boolean | Promise<boolean>
      realm?: string
      hashFunction?: Function
      invalidUserMessage?: string | object | MessageFunction
      onAuthSuccess?: (c: Context, username: string) => void | Promise<void>
    }
```
- detectResponseType (src/client/fetch-result-please.ts:80-95)
```typescript
function detectResponseType(response: Response): 'json' | 'text' {
  const _contentType = response.headers.get('content-type')

  if (!_contentType) {
    return 'text'
  }

  // `_contentType` might look like: `application/json; charset=utf-8`, `text/plain`, so we get the first part before `;`
  const contentType = _contentType.split(';').shift()!

  if (jsonRegex.test(contentType)) {
    return 'json'
  }

  return 'text'
}
```
- ResponseHeader (src/utils/headers.ts:266-343)
```typescript
export type ResponseHeader =
  | 'Accept-Query'
  | 'Access-Control-Allow-Credentials'
  | 'Access-Control-Allow-Headers'
  | 'Access-Control-Allow-Methods'
  | 'Access-Control-Allow-Origin'
  | 'Access-Control-Expose-Headers'
  | 'Access-Control-Max-Age'
  | 'Activate-Storage-Access'
  | 'Age'
  | 'Allow'
  | 'Cache-Control'
  | 'Cache-Group-Invalidation'
  | 'Cache-Groups'
  | 'Clear-Site-Data'
  | 'Content-Disposition'
  | 'Content-Encoding'
  | 'Content-Language'
  | 'Content-Length'
  | 'Content-Location'
  | 'Content-Range'
  | 'Content-Security-Policy'
  | 'Content-Security-Policy-Report-Only'
  | 'Content-Type'
  | 'Cookie'
  | 'Cross-Origin-Embedder-Policy'
  | 'Cross-Origin-Opener-Policy'
  | 'Cross-Origin-Resource-Policy'
  | 'Date'
  | 'ETag'
  | 'Expires'
  | 'Incremental'
  | 'Last-Modified'
  | 'Location'
  | 'Permissions-Policy'
  | 'Pragma'
  | 'Retry-After'
  | 'Save-Data'
  | 'Sec-CH-Prefers-Color-Scheme'
  | 'Sec-CH-Prefers-Reduced-Motion'
  | 'Sec-CH-UA'
  | 'Sec-CH-UA-Arch'
  | 'Sec-CH-UA-Bitness'
  | 'Sec-CH-UA-Form-Factor'
  | 'Sec-CH-UA-Full-Version'
  | 'Sec-CH-UA-Full-Version-List'
  | 'Sec-CH-UA-Mobile'
  | 'Sec-CH-UA-Model'
  | 'Sec-CH-UA-Platform'
  | 'Sec-CH-UA-Platform-Version'
  | 'Sec-CH-UA-WoW64'
  | 'Sec-Fetch-Dest'
  | 'Sec-Fetch-Mode'
  | 'Sec-Fetch-Site'
  | 'Sec-Fetch-User'
  | 'Sec-GPC'
  | 'Server'
  | 'Server-Timing'
  | 'Service-Worker-Navigation-Preload'
  | 'Set-Cookie'
  | 'Set-Txn'
  | 'Strict-Transport-Security'
  | 'Timing-Allow-Origin'
  | 'Trailer'
  | 'Transfer-Encoding'
  | 'Unencoded-Digest'
  | 'Upgrade'
  | 'Vary'
  | 'Want-Unencoded-Digest'
  | 'Warning'
  | 'WWW-Authenticate'
  | 'X-Content-Type-Options'
  | 'X-DNS-Prefetch-Control'
  | 'X-Frame-Options'
  | 'X-Permitted-Cross-Domain-Policies'
  | 'X-Powered-By'
  | 'X-Robots-Tag'
  | 'X-XSS-Protection'
```
- COMPRESSIBLE_CONTENT_TYPE_REGEX (src/utils/compress.ts:9-10)
```typescript
export const COMPRESSIBLE_CONTENT_TYPE_REGEX =
  /^\s*(?:text\/(?!event-stream(?:[;\s]|$))[^;\s]+|application\/(?:javascript|json|xml|xml-dtd|ecmascript|dart|msgpack|postscript|rtf|tar|toml|vnd\.dart|vnd\.ms-fontobject|vnd\.ms-opentype|vnd\.msgpack|wasm|x-httpd-php|x-javascript|x-msgpack|x-ns-proxy-autoconfig|x-sh|x-tar|x-virtualbox-hdd|x-virtualbox-ova|x-virtualbox-ovf|x-virtualbox-vbox|x-virtualbox-vdi|x-virtualbox-vhd|x-virtualbox-vmdk|x-www-form-urlencoded)|font\/(?:otf|ttf)|image\/(?:bmp|vnd\.adobe\.photoshop|vnd\.microsoft\.icon|vnd\.ms-dds|x-icon|x-ms-bmp)|message\/rfc822|model\/gltf-binary|x-shader\/x-fragment|x-shader\/x-vertex|[^;\s]+?\+(?:json|text|xml|yaml|msgpack))(?:[;\s]|$)/i
```
- RequestHeader (src/utils/headers.ts:8-264)
```typescript
export type RequestHeader =
  | 'A-IM'
  | 'Accept'
  | 'Accept-Additions'
  | 'Accept-CH'
  | 'Accept-Charset'
  | 'Accept-Datetime'
  | 'Accept-Encoding'
  | 'Accept-Features'
  | 'Accept-Language'
  | 'Accept-Patch'
  | 'Accept-Post'
  | 'Accept-Query'
  | 'Accept-Ranges'
  | 'Accept-Signature'
  | 'Access-Control'
  | 'Access-Control-Allow-Credentials'
  | 'Access-Control-Allow-Headers'
  | 'Access-Control-Allow-Methods'
  | 'Access-Control-Allow-Origin'
  | 'Access-Control-Expose-Headers'
  | 'Access-Control-Max-Age'
  | 'Access-Control-Request-Headers'
  | 'Access-Control-Request-Method'
  | 'Activate-Storage-Access'
  | 'Age'
  | 'Allow'
  | 'ALPN'
  | 'Alt-Svc'
  | 'Alt-Used'
  | 'Alternates'
  | 'AMP-Cache-Transform'
  | 'Apply-To-Redirect-Ref'
  | 'Authentication-Control'
  | 'Authentication-Info'
  | 'Authorization'
  | 'Available-Dictionary'
  | 'C-Ext'
  | 'C-Man'
  | 'C-Opt'
  | 'C-PEP'
  | 'C-PEP-Info'
  | 'Cache-Control'
  | 'Cache-Group-Invalidation'
  | 'Cache-Groups'
  | 'Cache-Status'
  | 'Cal-Managed-ID'
  | 'CalDAV-Timezones'
  | 'Capsule-Protocol'
  | 'CDN-Cache-Control'
  | 'CDN-Loop'
  | 'Cert-Not-After'
  | 'Cert-Not-Before'
  | 'Clear-Site-Data'
  | 'Client-Cert'
  | 'Client-Cert-Chain'
  | 'Close'
  | 'CMCD-Object'
  | 'CMCD-Request'
  | 'CMCD-Session'
  | 'CMCD-Status'
  | 'CMSD-Dynamic'
  | 'CMSD-Static'
  | 'Concealed-Auth-Export'
  | 'Configuration-Context'
  | 'Connection'
  | 'Content-Base'
  | 'Content-Digest'
  | 'Content-Disposition'
  | 'Content-Encoding'
  | 'Content-ID'
  | 'Content-Language'
  | 'Content-Length'
  | 'Content-Location'
  | 'Content-MD5'
  | 'Content-Range'
  | 'Content-Script-Type'
  | 'Content-Security-Policy'
  | 'Content-Security-Policy-Report-Only'
  | 'Content-Style-Type'
  | 'Content-Type'
  | 'Content-Version'
  | 'Cookie'
  | 'Cookie2'
  | 'Cross-Origin-Embedder-Policy'
  | 'Cross-Origin-Embedder-Policy-Report-Only'
  | 'Cross-Origin-Opener-Policy'
  | 'Cross-Origin-Opener-Policy-Report-Only'
  | 'Cross-Origin-Resource-Policy'
  | 'CTA-Common-Access-Token'
  | 'DASL'
  | 'Date'
  | 'DAV'
  | 'Default-Style'
  | 'Delta-Base'
  | 'Deprecation'
  | 'Depth'
  | 'Derived-From'
  | 'Destination'
  | 'Detached-JWS'
  | 'Differential-ID'
  | 'Dictionary-ID'
  | 'Digest'
  | 'DPoP'
  | 'DPoP-Nonce'
  | 'Early-Data'
  | 'EDIINT-Features'
  | 'ETag'
  | 'Expect'
  | 'Expect-CT'
  | 'Expires'
  | 'Ext'
  | 'Forwarded'
  | 'From'
  | 'GetProfile'
  | 'Hobareg'
  | 'Host'
  | 'HTTP2-Settings'
  | 'If'
  | 'If-Match'
  | 'If-Modified-Since'
  | 'If-None-Match'
  | 'If-Range'
  | 'If-Schedule-Tag-Match'
  | 'If-Unmodified-Since'
  | 'IM'
  | 'Include-Referred-Token-Binding-ID'
  | 'Incremental'
  | 'Isolation'
  | 'Keep-Alive'
  | 'Label'
  | 'Last-Event-ID'
  | 'Last-Modified'
  | 'Link'
  | 'Link-Template'
  | 'Location'
  | 'Lock-Token'
  | 'Man'
  | 'Max-Forwards'
  | 'Memento-Datetime'
  | 'Meter'
  | 'Method-Check'
  | 'Method-Check-Expires'
  | 'MIME-Version'
  | 'Negotiate'
  | 'NEL'
  | 'OData-EntityId'
  | 'OData-Isolation'
  | 'OData-MaxVersion'
  | 'OData-Version'
  | 'Opt'
  | 'Optional-WWW-Authenticate'
  | 'Ordering-Type'
  | 'Origin'
  | 'Origin-Agent-Cluster'
  | 'OSCORE'
  | 'OSLC-Core-Version'
  | 'Overwrite'
  | 'P3P'
  | 'PEP'
  | 'PEP-Info'
  | 'Permissions-Policy'
  | 'PICS-Label'
  | 'Ping-From'
  | 'Ping-To'
  | 'Position'
  | 'Pragma'
  | 'Prefer'
  | 'Preference-Applied'
  | 'Priority'
  | 'ProfileObject'
  | 'Protocol'
  | 'Protocol-Info'
  | 'Protocol-Query'
  | 'Protocol-Request'
  | 'Proxy-Authenticate'
  | 'Proxy-Authentication-Info'
  | 'Proxy-Authorization'
  | 'Proxy-Features'
  | 'Proxy-Instruction'
  | 'Proxy-Status'
  | 'Public'
  | 'Public-Key-Pins'
  | 'Public-Key-Pins-Report-Only'
  | 'Range'
  | 'Redirect-Ref'
  | 'Referer'
  | 'Referer-Root'
  | 'Referrer-Policy'
  | 'Refresh'
  | 'Repeatability-Client-ID'
  | 'Repeatability-First-Sent'
  | 'Repeatability-Request-ID'
  | 'Repeatability-Result'
  | 'Replay-Nonce'
  | 'Reporting-Endpoints'
  | 'Repr-Digest'
  | 'Retry-After'
  | 'Safe'
  | 'Schedule-Reply'
  | 'Schedule-Tag'
  | 'Sec-Fetch-Dest'
  | 'Sec-Fetch-Mode'
  | 'Sec-Fetch-Site'
  | 'Sec-Fetch-Storage-Access'
  | 'Sec-Fetch-User'
  | 'Sec-GPC'
  | 'Sec-Purpose'
  | 'Sec-Token-Binding'
  | 'Sec-WebSocket-Accept'
  | 'Sec-WebSocket-Extensions'
  | 'Sec-WebSocket-Key'
  | 'Sec-WebSocket-Protocol'
  | 'Sec-WebSocket-Version'
  | 'Security-Scheme'
  | 'Server'
  | 'Server-Timing'
  | 'Set-Cookie'
  | 'Set-Cookie2'
  | 'Set-Txn'
  | 'SetProfile'
  | 'Signature'
  | 'Signature-Input'
  | 'SLUG'
  | 'SoapAction'
  | 'Status-URI'
  | 'Strict-Transport-Security'
  | 'Sunset'
  | 'Surrogate-Capability'
  | 'Surrogate-Control'
  | 'TCN'
  | 'TE'
  | 'Timeout'
  | 'Timing-Allow-Origin'
  | 'Topic'
  | 'Traceparent'
  | 'Tracestate'
  | 'Trailer'
  | 'Transfer-Encoding'
  | 'TTL'
  | 'Unencoded-Digest'
  | 'Upgrade'
  | 'Urgency'
  | 'URI'
  | 'Use-As-Dictionary'
  | 'User-Agent'
  | 'Variant-Vary'
  | 'Vary'
  | 'Via'
  | 'Want-Content-Digest'
  | 'Want-Digest'
  | 'Want-Repr-Digest'
  | 'Want-Unencoded-Digest'
  | 'Warning'
  | 'WWW-Authenticate'
  | 'X-Content-Type-Options'
  | 'X-Frame-Options'
```
- parseResponseContent (src/helper/ssg/ssg.ts:74-88)
```typescript
const parseResponseContent = async (response: Response): Promise<string | ArrayBuffer> => {
  const contentType = response.headers.get('Content-Type')

  try {
    if (contentType?.includes('text') || contentType?.includes('json')) {
      return await response.text()
    } else {
      return await response.arrayBuffer()
    }
  } catch (error) {
    throw new Error(
      `Error processing response: ${error instanceof Error ? error.message : 'Unknown error'}`
    )
  }
}
```
- BodyRespond (src/context.ts:121-141)
```typescript
interface BodyRespond {
  // if we return content, only allow the status codes that allow for returning the body
  <T extends Data, U extends ContentfulStatusCode>(
    data: T,
    status?: U,
    headers?: HeaderRecord
  ): Response & TypedResponse<T, U, 'body'>
  <T extends Data, U extends ContentfulStatusCode>(
    data: T,
    init?: ResponseOrInit<U>
  ): Response & TypedResponse<T, U, 'body'>
  <T extends null, U extends StatusCode>(
    data: T,
    status?: U,
    headers?: HeaderRecord
  ): Response & TypedResponse<null, U, 'body'>
  <T extends null, U extends StatusCode>(
    data: T,
    init?: ResponseOrInit<U>
  ): Response & TypedResponse<null, U, 'body'>
}
```
- DetailedError (src/client/fetch-result-please.ts:46-75)
```typescript
export class DetailedError extends Error {
  /**
   * Additional `message` that will be logged AND returned to client
   */
  public detail?: any
  /**
   * Additional `code` that will be logged AND returned to client
   */
  public code?: any
  /**
   * Additional value that will be logged AND NOT returned to client
   */
  public log?: any
  /**
   * Optionally set the status code to return, in a web server context
   */
  public statusCode?: any

  constructor(
    message: string,
    options: { detail?: any; code?: any; statusCode?: number; log?: any } = {}
  ) {
    super(message)
    this.name = 'DetailedError'
    this.log = options.log
    this.detail = options.detail
    this.code = options.code
    this.statusCode = options.statusCode
  }
}
```
- compress (src/middleware/compress/index.ts:72-136)
```typescript
export const compress = (options?: CompressionOptions): MiddlewareHandler => {
  const threshold = options?.threshold ?? 1024
  const candidates: readonly Encoding[] = options?.encoding ? [options.encoding] : ENCODING_TYPES

  const contentTypeFilter = options?.contentTypeFilter ?? COMPRESSIBLE_CONTENT_TYPE_REGEX
  const shouldCompress =
    typeof contentTypeFilter === 'function'
      ? (res: Response) => {
          const type = res.headers.get('Content-Type')
          return type && contentTypeFilter(type)
        }
      : (res: Response) => {
          const type = res.headers.get('Content-Type')
          return type && contentTypeFilter.test(type)
        }

  return async function compress(ctx, next) {
    await next()

    const contentLength = ctx.res.headers.get('Content-Length')

    // Check if response should be compressed
    if (
      ctx.res.status === 206 || // partial content, Content-Range refers to the uncompressed bytes
      ctx.res.headers.has('Content-Encoding') || // already encoded
      ctx.res.headers.has('Transfer-Encoding') || // already encoded or chunked
      ctx.req.method === 'HEAD' || // HEAD request
      (contentLength && Number(contentLength) < threshold) || // content-length below threshold
      !shouldCompress(ctx.res) || // not compressible type
      !shouldTransform(ctx.res) // cache-control: no-transform
    ) {
      return
    }

    // From here on the representation is selected from the request's Accept-Encoding,
    // so caches must key on that header. This is done before the negotiation result is
    // known, because an identity response served for a missing or unsupported
    // Accept-Encoding is just as much a negotiated representation: RFC 9110 lists
    // Accept-Encoding "or lack thereof" as a determining factor. Without this, a shared
    // cache could reuse the identity response for a client that does accept gzip.
    // https://www.rfc-editor.org/rfc/rfc9110#field.vary
    const current = ctx.res.headers.get('Vary')
    if (current !== '*' && !(current && varyAcceptEncodingRegExp.test(current))) {
      ctx.header('Vary', current ? `${current}, Accept-Encoding` : 'Accept-Encoding')
    }

    const accepted = ctx.req.header('Accept-Encoding')
    const encoding = selectEncoding(accepted, candidates)
    if (!encoding || !ctx.res.body) {
      return
    }

    // Compress the response
    const stream = new CompressionStream(encoding)
    ctx.res = new Response(ctx.res.body.pipeThrough(stream), ctx.res)
    ctx.res.headers.delete('Content-Length')
    ctx.res.headers.set('Content-Encoding', encoding)

    // Convert strong ETag to weak ETag since compressed content is not byte-identical
    const etag = ctx.res.headers.get('ETag')
    if (etag && !etag.startsWith('W/')) {
      ctx.res.headers.set('ETag', `W/${etag}`)
    }
  }
}
```
- Context (src/context.ts:293-797)
```typescript
export class Context<
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  E extends Env = any,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  P extends string = any,
  I extends Input = {},
> {
  #rawRequest: Request
  #req: HonoRequest<P, I['out']> | undefined
  /**
   * `.env` can get bindings (environment variables, secrets, KV namespaces, D1 database, R2 bucket etc.) in Cloudflare Workers.
   *
   * @see {@link https://hono.dev/docs/api/context#env}
   *
   * @example
   * ```ts
   * // Environment object for Cloudflare Workers
   * app.get('*', async c => {
   *   const counter = c.env.COUNTER
   * })
   * ```
   */
  env: E['Bindings'] = {}
  #var: Map<unknown, unknown> | undefined
  finalized: boolean = false
  /**
   * `.error` can get the error object from the middleware if the Handler throws an error.
   *
   * @see {@link https://hono.dev/docs/api/context#error}
   *
   * @example
   * ```ts
   * app.use('*', async (c, next) => {
   *   await next()
   *   if (c.error) {
   *     // do something...
   *   }
   * })
   * ```
   */
  error: Error | undefined

  #status: StatusCode | undefined
  #executionCtx: FetchEventLike | ExecutionContext | undefined
  #res: Response | undefined
  #layout: Layout<PropsForRenderer & { Layout: Layout }> | undefined
  #renderer: Renderer | undefined
  #notFoundHandler: NotFoundHandler<E> | undefined
  #preparedHeaders: Headers | undefined

  #matchResult: Result<[H, RouterRoute]> | undefined
  #path: string | undefined

  /**
   * Creates an instance of the Context class.
   *
   * @param req - The Request object.
   * @param options - Optional configuration options for the context.
   */
  constructor(req: Request, options?: ContextOptions<E>) {
    this.#rawRequest = req
    if (options) {
      this.#executionCtx = options.executionCtx
      this.env = options.env
      this.#notFoundHandler = options.notFoundHandler
      this.#path = options.path
      this.#matchResult = options.matchResult
    }
  }

  /**
   * `.req` is the instance of {@link HonoRequest}.
   */
  get req(): HonoRequest<P, I['out']> {
    this.#req ??= new HonoRequest(this.#rawRequest, this.#path, this.#matchResult)
    return this.#req
  }

  /**
   * @see {@link https://hono.dev/docs/api/context#event}
   * The FetchEvent associated with the current request.
   *
   * @throws Will throw an error if the context does not have a FetchEvent.
   */
  get event(): FetchEventLike {
    if (this.#executionCtx && 'respondWith' in this.#executionCtx) {
      return this.#executionCtx
    } else {
      throw Error('This context has no FetchEvent')
    }
  }

  /**
   * @see {@link https://hono.dev/docs/api/context#executionctx}
   * The ExecutionContext associated with the current request.
   *
   * @throws Will throw an error if the context does not have an ExecutionContext.
   */
  get executionCtx(): ExecutionContext {
    if (this.#executionCtx) {
      return this.#executionCtx as ExecutionContext
    } else {
      throw Error('This context has no ExecutionContext')
    }
  }

  /**
   * @see {@link https://hono.dev/docs/api/context#res}
   * The Response object for the current request.
   */
  get res(): Response {
    return (this.#res ||= createResponseInstance(null, {
      headers: (this.#preparedHeaders ??= new Headers()),
    }))
  }

  /**
   * Sets the Response object for the current request.
   *
   * @param _res - The Response object to set.
   */
  set res(_res: Response | undefined) {
    if (this.#res && _res) {
      _res = createResponseInstance(_res.body, _res)
      for (const [k, v] of this.#res.headers.entries()) {
        if (k === 'content-type') {
          continue
        }
        if (k === 'set-cookie') {
          const cookies = this.#res.headers.getSetCookie()
          _res.headers.delete('set-cookie')
          for (const cookie of cookies) {
            _res.headers.append('set-cookie', cookie)
          }
        } else {
          _res.headers.set(k, v)
        }
      }
    }
    this.#res = _res
    this.finalized = true
  }

  /**
   * `.render()` can create a response within a layout.
   *
   * @see {@link https://hono.dev/docs/api/context#render-setrenderer}
   *
   * @example
   * ```ts
   * app.get('/', (c) => {
   *   return c.render('Hello!')
   * })
   * ```
   */
  render: Renderer = (...args) => {
    this.#renderer ??= (content: string | Promise<string>) => this.html(content)
    return this.#renderer(...args)
  }

  /**
   * Sets the layout for the response.
   *
   * @param layout - The layout to set.
   * @returns The layout function.
   */
  setLayout = (
    layout: Layout<PropsForRenderer & { Layout: Layout }>
  ): Layout<
    PropsForRenderer & {
      Layout: Layout
    }
  > => (this.#layout = layout)

  /**
   * Gets the current layout for the response.
   *
   * @returns The current layout function.
   */
  getLayout = (): Layout<PropsForRenderer & { Layout: Layout }> | undefined => this.#layout

  /**
   * `.setRenderer()` can set the layout in the custom middleware.
   *
   * @see {@link https://hono.dev/docs/api/context#render-setrenderer}
   *
   * @example
   * ```tsx
   * app.use('*', async (c, next) => {
   *   c.setRenderer((content) => {
   *     return c.html(
   *       <html>
   *         <body>
   *           <p>{content}</p>
   *         </body>
   *       </html>
   *     )
   *   })
   *   await next()
   * })
   * ```
   */
