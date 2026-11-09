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
