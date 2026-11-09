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
