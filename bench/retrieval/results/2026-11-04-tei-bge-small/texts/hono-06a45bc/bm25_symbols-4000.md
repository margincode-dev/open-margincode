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
