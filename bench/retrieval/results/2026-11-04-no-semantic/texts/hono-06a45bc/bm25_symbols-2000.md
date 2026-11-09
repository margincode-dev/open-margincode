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
