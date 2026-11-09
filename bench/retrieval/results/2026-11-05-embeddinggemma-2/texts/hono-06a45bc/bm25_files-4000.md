### src/middleware/bearer-auth/index.ts
```typescript
/**
 * @module
 * Bearer Auth Middleware for Hono.
 */

import type { Context } from '../../context'
import { HTTPException } from '../../http-exception'
import type { Env, MiddlewareHandler } from '../../types'
import { timingSafeEqual } from '../../utils/buffer'
import type { ContentfulStatusCode } from '../../utils/http-status'

const TOKEN_STRINGS = '[A-Za-z0-9._~+/-]+=*'
const PREFIX = 'Bearer'
const HEADER = 'Authorization'

type MessageFunction = (c: Context) => string | object | Promise<string | object>
type CustomizedErrorResponseOptions = {
  wwwAuthenticateHeader?: string | object | MessageFunction
  message?: string | object | MessageFunction
}

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

/**
 * Bearer Auth Middleware for Hono.
 *
 * @see {@link https://hono.dev/docs/middleware/builtin/bearer-auth}
 *
 * @template E - The environment type.
 * @param {BearerAuthOptions} options - The options for the bearer authentication middleware.
 * @param {string | string[]} [options.token] - The string or array of strings to validate the incoming bearer token against.
 * @param {Function} [options.verifyToken] - The function to verify the token.
 * @param {string} [options.realm=""] - The domain name of the realm, as part of the returned WWW-Authenticate challenge header.
 * @param {string} [options.prefix="Bearer"] - The prefix (or known as `schema`) for the Authorization header value. If set to the empty string, no prefix is expected.
 * @param {string} [options.headerName=Authorization] - The header name.
 * @param {Function} [options.hashFunction] - A function to handle hashing for safe comparison of authentication tokens.
 * @param {string | object | MessageFunction} [options.noAuthenticationHeader.message="Unauthorized"] - The no authentication header message.
 * @param {string | object | MessageFunction} [options.noAuthenticationHeader.wwwAuthenticateHeader="Bearer realm=\"\""] - The response header value for the WWW-Authenticate header when no authentication header is provided.
 * @param {string | object | MessageFunction} [options.invalidAuthenticationHeader.message="Bad Request"] - The invalid authentication header message.
 * @param {string | object | MessageFunction} [options.invalidAuthenticationHeader.wwwAuthenticateHeader="Bearer error=\"invalid_request\""] - The response header value for the WWW-Authenticate header when authentication header is invalid.
 * @param {string | object | MessageFunction} [options.invalidToken.message="Unauthorized"] - The invalid token message.
 * @param {string | object | MessageFunction} [options.invalidToken.wwwAuthenticateHeader="Bearer error=\"invalid_token\""] - The response header value for the WWW-Authenticate header when token is invalid.
 * @returns {MiddlewareHandler<E>} The middleware handler function.
 * @throws {Error} If neither "token" nor "verifyToken" options are provided.
 * @throws {HTTPException} If authentication fails, with 401 status code for missing or invalid token, or 400 status code for invalid request.
 *
 * @example
 * ```ts
 * const app = new Hono()
 *
 * const token = 'honoishot'
 *
 * app.use('/api/*', bearerAuth({ token }))
 *
 * app.get('/api/page', (c) => {
 *   return c.json({ message: 'You are authorized' })
 * })
 * ```
 */
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
### src/middleware/jwk/index.test.ts
```typescript
import { HttpResponse, http } from 'msw'
import { setupServer } from 'msw/node'
import { jwk } from '.'
import { setSignedCookie } from '../../helper/cookie'
import { Hono } from '../../hono'
import { HTTPException } from '../../http-exception'
import { encodeBase64Url } from '../../utils/encode'
import { Jwt } from '../../utils/jwt'
import type { HonoJsonWebKey } from '../../utils/jwt/jws'
import { signing } from '../../utils/jwt/jws'
import { verifyWithJwks } from '../../utils/jwt/jwt'
import type { JWTPayload } from '../../utils/jwt/types'
import { utf8Encoder } from '../../utils/jwt/utf8'
import * as test_keys from './keys.test.json'

const verify_keys = test_keys.public_keys

describe('JWK', () => {
  const server = setupServer(
    http.get('http://localhost/.well-known/jwks.json', () => {
      return HttpResponse.json({ keys: verify_keys })
    }),
    http.get('http://localhost/.well-known/missing-jwks.json', () => {
      return HttpResponse.json({})
    }),
    http.get('http://localhost/.well-known/bad-jwks.json', () => {
      return HttpResponse.json({ keys: 'bad-keys' })
    }),
    http.get('http://localhost/.well-known/404-jwks.json', () => {
      return HttpResponse.text('Not Found', { status: 404 })
    })
  )
  beforeAll(() => server.listen())
  afterEach(() => server.resetHandlers())
  afterAll(() => server.close())

  describe('verifyWithJwks', () => {
    it('Should throw error on missing keys/jwks_uri options', async () => {
      const credential = await Jwt.sign({ message: 'hello world' }, test_keys.private_keys[0])
      await expect(verifyWithJwks(credential, { allowedAlgorithms: ['RS256'] })).rejects.toThrow(
        'verifyWithJwks requires options for either "keys" or "jwks_uri" or both'
      )
    })
  })

  describe('options.allow_anon = true', () => {
    let handlerExecuted: boolean

    beforeEach(() => {
      handlerExecuted = false
    })

    const app = new Hono()

    app.use('/backend-auth-or-anon/*', jwk({ keys: verify_keys, allow_anon: true, alg: ['RS256'] }))

    app.get('/backend-auth-or-anon/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload ?? { message: 'hello anon' })
    })

    it('Should skip JWK if no token is present', async () => {
      const req = new Request('http://localhost/backend-auth-or-anon/a')
      const res = await app.request(req)
      expect(res).not.toBeNull()
      expect(res.status).toBe(200)
      expect(await res.json()).toEqual({ message: 'hello anon' })
      expect(handlerExecuted).toBeTruthy()
    })

    it('Should authorize if token is present', async () => {
      const credential = await Jwt.sign({ message: 'hello world' }, test_keys.private_keys[0])
      const req = new Request('http://localhost/backend-auth-or-anon/a')
      req.headers.set('Authorization', `Bearer ${credential}`)
      const res = await app.request(req)
      expect(res).not.toBeNull()
      expect(res.status).toBe(200)
      expect(await res.json()).toEqual({ message: 'hello world' })
      expect(handlerExecuted).toBeTruthy()
    })

    it('Should not authorize if a non-Bearer scheme is present', async () => {
      const credential = await Jwt.sign({ message: 'hello world' }, test_keys.private_keys[0])
      const url = 'http://localhost/backend-auth-or-anon/a'
      const req = new Request(url)
      req.headers.set('Authorization', `Basic ${credential}`)
      const res = await app.request(req)
      expect(res).not.toBeNull()
      expect(res.status).toBe(401)
      expect(res.headers.get('www-authenticate')).toEqual(
        `Bearer realm="${url}",error="invalid_request",error_description="invalid credentials structure"`
      )
      expect(handlerExecuted).toBeFalsy()
    })

    it('Should not authorize if an invalid Bearer token is present', async () => {
      const invalidToken =
        'ssyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJtZXNzYWdlIjoiaGVsbG8gd29ybGQifQ.B54pAqIiLbu170tGQ1rY06Twv__0qSHTA0ioQPIOvFE'
      const url = 'http://localhost/backend-auth-or-anon/a'
      const req = new Request(url)
      req.headers.set('Authorization', `Bearer ${invalidToken}`)
      const res = await app.request(req)
      expect(res).not.toBeNull()
      expect(res.status).toBe(401)
      expect(res.headers.get('www-authenticate')).toEqual(
        `Bearer realm="${url}",error="invalid_token",error_description="token verification failure"`
      )
      expect(handlerExecuted).toBeFalsy()
    })
  })

  describe('Credentials in header', () => {
    let handlerExecuted: boolean

    beforeEach(() => {
      handlerExecuted = false
    })

    const app = new Hono()

    app.use('/auth-with-keys/*', jwk({ keys: verify_keys, alg: ['RS256'] }))
    app.use('/auth-with-keys-unicode/*', jwk({ keys: verify_keys, alg: ['RS256'] }))
    app.use('/auth-with-keys-nested/*', async (c, next) => {
      const auth = jwk({ keys: verify_keys, alg: ['RS256'] })
      return auth(c, next)
    })
    app.use(
      '/auth-with-keys-fn/*',
      jwk({
        keys: async () => {
          const response = await fetch('http://localhost/.well-known/jwks.json')
          const data = await response.json()
          return data.keys
        },
        alg: ['RS256'],
      })
    )
    app.use(
      '/auth-with-jwks_uri/*',
      jwk({
        jwks_uri: 'http://localhost/.well-known/jwks.json',
        alg: ['RS256'],
      })
    )
    app.use(
      '/auth-with-keys-and-jwks_uri/*',
      jwk({
        keys: verify_keys,
        jwks_uri: () => 'http://localhost/.well-known/jwks.json',
        alg: ['RS256'],
      })
    )
    app.use(
      '/auth-with-missing-jwks_uri/*',
      jwk({
        jwks_uri: 'http://localhost/.well-known/missing-jwks.json',
        alg: ['RS256'],
      })
    )
    app.use(
      '/auth-with-404-jwks_uri/*',
      jwk({
        jwks_uri: 'http://localhost/.well-known/404-jwks.json',
        alg: ['RS256'],
      })
    )
    app.use(
      '/auth-with-bad-jwks_uri/*',
      jwk({
        jwks_uri: 'http://localhost/.well-known/bad-jwks.json',
        alg: ['RS256'],
      })
    )

    app.get('/auth-with-keys/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })
    app.get('/auth-with-keys-unicode/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })
    app.get('/auth-with-keys-nested/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })
    app.get('/auth-with-keys-fn/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })
    app.get('/auth-with-jwks_uri/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })
    app.get('/auth-with-keys-and-jwks_uri/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })
    app.get('/auth-with-missing-jwks_uri/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })
    app.get('/auth-with-404-jwks_uri/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })
    app.get('/auth-with-bad-jwks_uri/*', (c) => {
      handlerExecuted = true
      const payload = c.get('jwtPayload')
      return c.json(payload)
    })

    it('Should throw an error if the middleware is missing both keys and jwks_uri (empty)', async () => {
      // @ts-expect-error - Testing runtime error with missing required alg option
      expect(() => app.use('/auth-with-empty-middleware/*', jwk({}))).toThrow(
        'JWK auth middleware requires options for either "keys" or "jwks_uri"'
      )
    })

    it('Should throw an error when crypto.subtle is missing', async () => {
