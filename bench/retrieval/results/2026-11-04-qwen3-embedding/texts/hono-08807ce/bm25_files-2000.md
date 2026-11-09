### src/middleware/combine/index.ts
```typescript
/**
 * @module
 * Combine Middleware for Hono.
 */

import { compose } from '../../compose'
import type { Context } from '../../context'
import { METHOD_NAME_ALL } from '../../router'
import { TrieRouter } from '../../router/trie-router'
import type { MiddlewareHandler, Next } from '../../types'

type Condition = (c: Context) => boolean

/**
 * Create a composed middleware that runs the first middleware that returns true.
 *
 * @param middleware - An array of MiddlewareHandler or Condition functions.
 * Middleware is applied in the order it is passed, and if any middleware exits without returning
 * an exception first, subsequent middleware will not be executed.
 * You can also pass a condition function that returns a boolean value. If returns true
 * the evaluation will be halted, and rest of the middleware will not be executed.
 * @returns A composed middleware.
 *
 * @example
 * ```ts
 * import { some } from 'hono/combine'
 * import { bearerAuth } from 'hono/bearer-auth'
 * import { myRateLimit } from '@/rate-limit'
 *
 * // If client has a valid token, then skip rate limiting.
 * // Otherwise, apply rate limiting.
 * app.use('/api/*', some(
 *   bearerAuth({ token }),
 *   myRateLimit({ limit: 100 }),
 * ));
 * ```
 */
export const some = (...middleware: (MiddlewareHandler | Condition)[]): MiddlewareHandler => {
  return async function some(c, next) {
    let isNextCalled = false
    const wrappedNext = () => {
      isNextCalled = true
      return next()
    }

    let lastError: unknown
    let response: Response | undefined
    for (const handler of middleware) {
      try {
        const result = await handler(c, wrappedNext)
        if (result === true && !c.finalized) {
          await wrappedNext()
        } else if (result === false) {
          lastError = new Error('No successful middleware found')
          continue
        } else if (result && result !== true) {
          response = result
        }
        lastError = undefined
        break
      } catch (error) {
        lastError = error
        if (isNextCalled) {
          break
        }
      }
    }
    if (lastError) {
      throw lastError
    }
    return response
  }
}

/**
 * Create a composed middleware that runs all middleware and throws an error if any of them fail.
 *
 * @param middleware - An array of MiddlewareHandler or Condition functions.
 * Middleware is applied in the order it is passed, and if any middleware throws an error,
 * subsequent middleware will not be executed.
 * You can also pass a condition function that returns a boolean value. If returns false
 * the evaluation will be halted, and rest of the middleware will not be executed.
 * @returns A composed middleware.
 *
 * @example
 * ```ts
 * import { some, every } from 'hono/combine'
 * import { bearerAuth } from 'hono/bearer-auth'
 * import { myCheckLocalNetwork } from '@/check-local-network'
 * import { myRateLimit } from '@/rate-limit'
 *
 * // If client is in local network, then skip authentication and rate limiting.
 * // Otherwise, apply authentication and rate limiting.
 * app.use('/api/*', some(
 *   myCheckLocalNetwork(),
 *   every(
 *     bearerAuth({ token }),
 *     myRateLimit({ limit: 100 }),
 *   ),
 * ));
 * ```
 */
export const every = (...middleware: (MiddlewareHandler | Condition)[]): MiddlewareHandler => {
  return async function every(c, next) {
    const currentRouteIndex = c.req.routeIndex
    await compose(
      middleware.map((m) => [
        [
          async (c: Context, next: Next) => {
            c.req.routeIndex = currentRouteIndex // should be unchanged in this context
            const res = await m(c, next)
            if (res === false) {
              throw new Error('Unmet condition')
            }
            return res
          },
        ],
      ])
    )(c, next)
  }
}

/**
 * Create a composed middleware that runs all middleware except when the condition is met.
 *
 * @param condition - A string or Condition function.
 * If there are multiple targets to match any of them, they can be passed as an array.
 * If a string is passed, it will be treated as a path pattern to match.
 * If a Condition function is passed, it will be evaluated against the request context.
 * @param middleware - Middleware to run when the condition is not met.
 * Multiple middleware can be passed, and they are applied in the order they are passed.
 * @returns A composed middleware.
 *
 * @example
 * ```ts
 * import { except } from 'hono/combine'
 * import { bearerAuth } from 'hono/bearer-auth'
 *
 * // If client is accessing public API, then skip authentication.
 * // Otherwise, require a valid token.
 * app.use('/api/*', except(
 *   '/api/public/*',
 *   bearerAuth({ token }),
 * ));
 * ```
 */
export const except = (
  condition: string | Condition | (string | Condition)[],
  ...middleware: MiddlewareHandler[]
): MiddlewareHandler => {
  let router: TrieRouter<true> | undefined = undefined
  const conditions = (Array.isArray(condition) ? condition : [condition])
    .map((condition) => {
      if (typeof condition === 'string') {
        router ||= new TrieRouter()
        router.add(METHOD_NAME_ALL, condition, true)
      } else {
        return condition
      }
    })
    .filter(Boolean) as Condition[]

  if (router) {
    conditions.unshift((c: Context) => !!router?.match(METHOD_NAME_ALL, c.req.path)?.[0]?.[0]?.[0])
  }

  const handler = some((c: Context) => conditions.some((cond) => cond(c)), every(...middleware))
  return async function except(c, next) {
    await handler(c, next)
  }
}
```
### src/router/reg-exp-router/prepared-router.test.ts
```typescript
import { METHOD_NAME_ALL } from '../../router'
import { runTest } from '../common.case.test'
import { buildInitParams, serializeInitParams, PreparedRegExpRouter } from './prepared-router'

describe('PreparedRegExpRouter', async () => {
  runTest({
    skip: [
      {
        reason: 'UnsupportedPath',
        tests: [
          'Duplicate param name > parent',
          'Duplicate param name > child',
          'Capture Group > Complex capturing group > GET request',
          'Capture complex multiple directories > GET /part1/middle-b/latest',
          'Capture complex multiple directories > GET /part1/middle-b/end-c/latest',
          'Complex > Parameter with {.*} regexp',
        ],
      },
      {
        reason:
          'This route can not be added with `:label` to PreparedRegExpRouter. This is ambiguous',
        tests: ['Including slashes > GET /js/main.js'],
      },
    ],
    newRouter: <T>() => {
      let router: PreparedRegExpRouter<T>
      const routes: [string, string, T][] = []
      return {
        name: 'PreparedRegExpRouterBuilder',
        add: (method: string, path: string, handler: T) => {
          routes.push([method, path, handler])
        },
        match: (method: string, path: string) => {
          if (!router) {
            const serialized = serializeInitParams(
              buildInitParams({
                paths: routes.map((r) => r[1]),
              })
            )
            const params = eval(serialized) as ConstructorParameters<typeof PreparedRegExpRouter<T>>
            router = new PreparedRegExpRouter<T>(...params)

            for (const route of routes) {
              router.add(...route)
            }
          }
          return router.match(method, path)
        },
      }
    },
  })

  describe('add()', () => {
    it('should add a route', () => {
      const params = buildInitParams({
        paths: ['/hello'],
      })
      const router = new PreparedRegExpRouter(...params)
      router.add('GET', '/hello', 'get hello')
      expect(router.match('GET', '/hello')).toEqual([[['get hello', {}]], []])
    })

    it('should throw an error if the path is not pre-registered', () => {
      const params = buildInitParams({
        paths: ['/hello'],
      })
      const router = new PreparedRegExpRouter(...params)
      expect(() => router.add('GET', '/unknown', 'get hello')).toThrowError()
    })
  })
})

describe('buildInitParams() and serializeInitParams()', () => {
  it('should build empty init params', () => {
    const params = buildInitParams({
      paths: [],
    })
    expect(params).toEqual([{ [METHOD_NAME_ALL]: [/^$/, [], {}] }, {}])
    expect((0, eval)(serializeInitParams(params))).toEqual(params)
  })

  it('should build init params with one static path', () => {
    const params = buildInitParams({
      paths: ['/hello'],
    })
    expect(params).toEqual([
      {
        [METHOD_NAME_ALL]: [
          /^$/,
          [],
          {
            '/hello': [[], []],
          },
        ],
      },
      {
        '/hello': [[['']]],
      },
    ])
