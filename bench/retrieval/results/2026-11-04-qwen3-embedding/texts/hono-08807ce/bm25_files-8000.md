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
    expect((0, eval)(serializeInitParams(params))).toEqual(params)
  })

  it('should build init params with paths with params', () => {
    const params = buildInitParams({
      paths: ['/hello/:name', '/hello/:name/posts/:postId'],
    })
    expect(params).toEqual([
      {
        [METHOD_NAME_ALL]: [/^\/hello\/([^/]+)(?:$()|\/posts\/([^/]+)$())/, [0, 0, [], 0, []], {}],
      },
      {
        '/hello/:name': [[[2], { name: 1 }]],
        '/hello/:name/posts/:postId': [[[4], { name: 1, postId: 3 }]],
      },
    ])
    expect((0, eval)(serializeInitParams(params))).toEqual(params)
  })

  it('should build init params with wildcard', () => {
    const params = buildInitParams({
      paths: ['*'],
    })
    expect(params).toEqual([
      {
        [METHOD_NAME_ALL]: [/^.*$()/, [0, []], {}],
      },
      {},
    ])
    expect((0, eval)(serializeInitParams(params))).toEqual(params)
  })

  it('should build init params with complex path', () => {
    const params = buildInitParams({
      paths: ['/hello', '/hello/:name', '/hello/:name/posts/:postId', '*'],
    })
    expect(params).toEqual([
      {
        [METHOD_NAME_ALL]: [
          /^(?:\/hello\/([^/]+)(?:$()|\/posts\/([^/]+)$())|.*$())/,
          [0, 0, [], 0, [], []],
          {
            '/hello': [[], []],
          },
        ],
      },
      {
        '/hello': [[['']]],
        '/hello/:name': [[[2], { name: 1 }]],
        '/hello/:name/posts/:postId': [[[4], { name: 1, postId: 3 }]],
      },
    ])
    expect((0, eval)(serializeInitParams(params))).toEqual(params)
  })
})
```
### src/router/reg-exp-router/router.test.ts
```typescript
import type { ParamIndexMap, ParamStash } from '../../router'
import { UnsupportedPathError } from '../../router'
import { runTest } from '../common.case.test'
import { RegExpRouter } from './router'

describe('RegExpRouter', () => {
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
        reason: 'This route can not be added with `:label` to RegExpRouter. This is ambiguous',
        tests: ['Including slashes > GET /js/main.js'],
      },
    ],
    newRouter: () => new RegExpRouter(),
  })

  describe('Return value type', () => {
    it('Should return [[T, ParamIndexMap][], ParamStash]', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/posts/:id', 'get post')

      const [res, stash] = router.match('GET', '/posts/1')
      expect(res.length).toBe(1)
      expect(res).toEqual([['get post', { id: 1 }]])
      expect((stash as ParamStash)[1]).toBe('1')
    })
  })

  describe('UnsupportedPathError', () => {
    describe('Ambiguous', () => {
      it('GET /entry/:name', () => {
        const router = new RegExpRouter<string>()
        router.add('GET', '/:user/entries', 'get user entries')
        expect(() => {
          router.add('GET', '/entry/:name', 'get entry')
        }).toThrowError(UnsupportedPathError)
      })
    })

    describe('Multiple handlers with different label', () => {
      it('GET /:class/:id', () => {
        const router = new RegExpRouter<string>()
        router.add('GET', '/:type/:id', ':type')
        expect(() => {
          router.add('GET', '/:class/:id', ':class')
        }).toThrowError(UnsupportedPathError)
      })
    })

    it('parent', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/:id/:action', 'foo')
      expect(() => {
        router.add('GET', '/posts/:id', 'bar')
      }).toThrowError(UnsupportedPathError)
    })

    it('child', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/posts/:id', 'foo')
      expect(() => {
        router.add('GET', '/:id/:action', 'bar')
      }).toThrowError(UnsupportedPathError)
    })

    describe('static and dynamic', () => {
      it('static first', () => {
        const router = new RegExpRouter<string>()
        router.add('GET', '/reg-exp/router', 'foo')
        expect(() => {
          router.add('GET', '/reg-exp/:id', 'bar')
        }).toThrowError(UnsupportedPathError)
      })

      it('long label', () => {
        const router = new RegExpRouter<string>()
        router.add('GET', '/reg-exp/router', 'foo')
        expect(() => {
          router.add('GET', '/reg-exp/:service', 'bar')
        }).toThrowError(UnsupportedPathError)
      })

      it('dynamic first', () => {
        const router = new RegExpRouter<string>()
        router.add('GET', '/reg-exp/:id', 'bar')
        expect(() => {
          router.add('GET', '/reg-exp/router', 'foo')
        }).toThrowError(UnsupportedPathError)
      })
    })

    it('different regular expression', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/:id/:action{create|update}', 'foo')
      expect(() => {
        router.add('GET', '/:id/:action{delete}', 'bar')
      }).toThrowError(UnsupportedPathError)
    })

    it('ALL route added after a specific-method route', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/foo/:a', 'foo')
      expect(() => {
        router.add('ALL', '/foo/:b', 'bar')
      }).toThrowError(UnsupportedPathError)
    })

    it('specific-method route added after an ALL route', () => {
      const router = new RegExpRouter<string>()
      router.add('ALL', '/reg-exp/router', 'foo')

      expect(() => {
        router.add('GET', '/reg-exp/:id', 'bar')
      }).toThrowError(UnsupportedPathError)
    })

    it('different methods do not conflict', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/reg-exp/router', 'foo')
      router.add('POST', '/reg-exp/:id', 'bar')

      expect(router.match('GET', '/reg-exp/router')[0][0][0]).toBe('foo')
      expect(router.match('POST', '/reg-exp/router')[0][0][0]).toBe('bar')
    })

    describe('Capture Group', () => {
      describe('Complex capturing group', () => {
        it('GET request', () => {
          const router = new RegExpRouter<string>()
          expect(() => {
            router.add('GET', '/foo/:capture{ba(r|z)}', 'ok')
          }).toThrowError(UnsupportedPathError)
        })
      })
    })
  })

  describe('Wildcard after label', () => {
    it('Should be able to add a tail wildcard after a label', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/api/:x', 'label')
      router.add('GET', '/api/*', 'wildcard')

      const [res] = router.match('GET', '/api/foo')
      expect(res.length).toBe(2)
      expect(res[0][0]).toBe('label')
      expect(res[1][0]).toBe('wildcard')
    })

    it('Should prioritize the only wildcard over the tail wildcard regardless of the order', () => {
      for (const paths of [
        ['/a*', '/a/*'],
        ['/a/*', '/a*'],
      ]) {
        const router = new RegExpRouter<string>()
        for (const path of paths) {
          router.add('GET', path, path)
        }
        const [res] = router.match('GET', '/a')
        expect(res[0][0]).toBe('/a*')
      }
    })
  })

  describe('Single character regexp pattern', () => {
    it('Should capture a param even if a static path created the node first', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/a/c', 'static')
      router.add('GET', '/:x{a}/b', 'pattern')

      const [res, stash] = router.match('GET', '/a/b')
      expect(res.length).toBe(1)
      expect(res[0][0]).toBe('pattern')
      expect((stash as ParamStash)[(res[0][1] as ParamIndexMap)['x']]).toBe('a')
    })

    it('Should throw an error when a pattern terminal conflicts with a static terminal', () => {
      for (const paths of [
        ['/a', '/:x{a}'],
        ['/:x{a}', '/a'],
      ]) {
        const router = new RegExpRouter<string>()
        router.add('GET', paths[0], paths[0])
        expect(() => {
          router.add('GET', paths[1], paths[1])
        }).toThrowError(UnsupportedPathError)
      }
    })

    it('Should coexist with static paths regardless of the order', () => {
      for (const paths of [
        ['/foo/bar', '/:y{b}'],
        ['/:y{b}', '/foo/bar'],
      ]) {
        const router = new RegExpRouter<string>()
        for (const path of paths) {
          router.add('GET', path, path)
        }
        expect(router.match('GET', '/foo/bar')[0][0][0]).toBe('/foo/bar')
        expect(router.match('GET', '/b')[0][0][0]).toBe('/:y{b}')
      }
    })

    it('Should coexist with dynamic paths regardless of the order', () => {
      for (const paths of [
        ['/foo/:p', '/:x{a}'],
        ['/:x{a}', '/foo/:p'],
      ]) {
        const router = new RegExpRouter<string>()
        for (const path of paths) {
          router.add('GET', path, path)
        }
        expect(router.match('GET', '/a')[0][0][0]).toBe('/:x{a}')
        expect(router.match('GET', '/foo/v')[0][0][0]).toBe('/foo/:p')
      }
    })

    it('Should throw an error for a single meta character pattern', () => {
      const router = new RegExpRouter<string>()
      expect(() => {
        router.add('GET', '/:x{.}', 'meta')
      }).toThrowError(UnsupportedPathError)
    })
  })

  describe('Capture a param of a label node created by a middle wildcard', () => {
    it('Should capture the param', () => {
      const router = new RegExpRouter<string>()
      router.add('GET', '/w/*/x', 'wildcard')
      router.add('GET', '/w/:id/y', 'label')

      const [res, stash] = router.match('GET', '/w/123/y')
      expect(res.length).toBe(1)
      expect(res[0][0]).toBe('label')
      expect((stash as ParamStash)[(res[0][1] as ParamIndexMap)['id']]).toBe('123')
    })
  })

  describe('Static path including a colon in the middle', () => {
    it('Should be treated as a static path', () => {
      for (const paths of [
        ['/v1/name:activate', '/v1/name2'],
        ['/v1/name2', '/v1/name:activate'],
      ]) {
        const router = new RegExpRouter<string>()
        for (const path of paths) {
          router.add('GET', path, path)
        }
        expect(router.match('GET', '/v1/name:activate')[0][0][0]).toBe('/v1/name:activate')
        expect(router.match('GET', '/v1/name2')[0][0][0]).toBe('/v1/name2')
      }
    })
  })
})
```
### src/router/common.case.test.ts
```typescript
import type { RunnerTestSuite } from 'vitest'
import type { ParamIndexMap, Params, Router } from '../router'

const getSuiteHierarchy = (suite?: RunnerTestSuite) => {
  const res: RunnerTestSuite[] = []
  let s: RunnerTestSuite | undefined = suite
  while (s) {
    res.unshift(s)
    s = s.suite
  }
  return res
}

export const runTest = ({
  skip = [],
  newRouter,
}: {
  skip?: {
    reason: string
    tests: string[]
  }[]
  newRouter: <T>() => Router<T>
}) => {
  describe('Common', () => {
    type Match = (method: string, path: string) => { handler: string; params: Params }[]
    let router: Router<string>
    let match: Match

    beforeEach(({ task, skip: skipTask }) => {
      const suites = getSuiteHierarchy(task.suite)
      const name = [...suites.slice(2).map((s) => s.name), task.name].join(' > ')
      const isSkip = skip.find((s) => s.tests.includes(name))
      if (isSkip) {
        console.log(`Skip: ${isSkip.reason}`)
        skipTask()
        return
      }

      router = newRouter()
      match = (method: string, path: string) => {
        const [matchRes, stash] = router.match(method, path)
        const res = matchRes.map((r) =>
          stash
            ? {
                handler: r[0],
                params: Object.keys(r[1]).reduce(
                  (acc, key) => {
                    acc[key] = stash[(r[1] as ParamIndexMap)[key]]
                    return acc
                  },
                  Object.create(null) as Params
                ),
              }
            : { handler: r[0], params: r[1] as Params }
        )
        return res
      }
    })

    describe('Basic Usage', () => {
      beforeEach(() => {
        router.add('GET', '/hello', 'get hello')
        router.add('POST', '/hello', 'post hello')
        router.add('PURGE', '/hello', 'purge hello')
      })

      it('GET, post hello', async () => {
        let res = match('GET', '/hello')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('get hello')
        res = match('POST', '/hello')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('post hello')
        res = match('PURGE', '/hello')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('purge hello')
        res = match('PUT', '/hello')
        expect(res.length).toBe(0)
        res = match('GET', '/')
        expect(res.length).toBe(0)
      })
    })

    describe('Reserved words', () => {
      it('Reserved words and named parameter', async () => {
        router.add('GET', '/entry/:constructor', 'get entry')
        const res = match('GET', '/entry/123')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('get entry')
        expect(res[0].params['constructor']).toBe('123')
      })

      it('Reserved words and wildcard', async () => {
        router.add('GET', '/wild/*/card', 'get wildcard')
        const res = match('GET', '/wild/constructor/card')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('get wildcard')
      })

      it('Reserved words and optional named parameter', async () => {
        router.add('GET', '/api/animals/:constructor?', 'animals')
        const res = match('GET', '/api/animals')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('animals')
        expect(res[0].params['constructor']).toBeUndefined()
      })
    })

    describe('Complex', () => {
      it('Named Param', async () => {
        router.add('GET', '/entry/:id', 'get entry')
        let res = match('GET', '/entry/123')

        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('get entry')
        expect(res[0].params['id']).toBe('123')

        res = match('GET', '/entry-123')
        expect(res.length).toBe(0)
      })

      it('Wildcard', async () => {
        router.add('GET', '/wild/*/card', 'get wildcard')
        const res = match('GET', '/wild/xxx/card')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('get wildcard')
      })

      it('Default', async () => {
        router.add('GET', '/api/abc', 'get api')
        router.add('GET', '/api/*', 'fallback')
        let res = match('GET', '/api/abc')
        expect(res.length).toBe(2)
        expect(res[0].handler).toEqual('get api')
        expect(res[1].handler).toEqual('fallback')
        res = match('GET', '/api/def')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('fallback')
      })

      it('Regexp', async () => {
        router.add('GET', '/post/:date{[0-9]+}/:title{[a-z]+}', 'get post')
        let res = match('GET', '/post/20210101/hello')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('get post')
        expect(res[0].params['date']).toBe('20210101')
        expect(res[0].params['title']).toBe('hello')
        res = match('GET', '/post/onetwothree')
        expect(res.length).toBe(0)
        res = match('GET', '/post/123/123')
        expect(res.length).toBe(0)
      })

      it('Parameter with {.*} regexp', () => {
        router.add('GET', '/files/:name{.*}', 'file')
        let res = match('GET', '/files')
        expect(res.length).toBe(0)

        res = match('GET', '/files/a')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('file')
        expect(res[0].params['name']).toEqual('a')

        res = match('GET', '/files/a/b')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('file')
        expect(res[0].params['name']).toEqual('a/b')

        res = match('GET', '/files/')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('file')
      })

      it('/*', async () => {
        router.add('GET', '/api/*', 'auth middleware')
        router.add('GET', '/api', 'top')
        router.add('GET', '/api/posts', 'posts')
        router.add('GET', '/api/*', 'fallback')

        let res = match('GET', '/api')
        expect(res.length).toBe(3)
        expect(res[0].handler).toEqual('auth middleware')
        expect(res[1].handler).toEqual('top')
        expect(res[2].handler).toEqual('fallback')
        res = match('GET', '/api/posts')
        expect(res.length).toBe(3)
        expect(res[0].handler).toEqual('auth middleware')
        expect(res[1].handler).toEqual('posts')
        expect(res[2].handler).toEqual('fallback')
      })
    })

    describe('Registration order', () => {
      it('middleware -> handler', async () => {
        router.add('GET', '*', 'bar')
        router.add('GET', '/:type/:action', 'foo')
        const res = match('GET', '/posts/123')
        expect(res.length).toBe(2)
        expect(res[0].handler).toEqual('bar')
        expect(res[1].handler).toEqual('foo')
      })

      it('handler -> fallback', async () => {
        router.add('GET', '/:type/:action', 'foo')
        router.add('GET', '*', 'fallback')
        const res = match('GET', '/posts/123')
        expect(res.length).toBe(2)
        expect(res[0].handler).toEqual('foo')
        expect(res[1].handler).toEqual('fallback')
      })
    })

    describe('Multi match', () => {
      describe('Blog', () => {
        beforeEach(() => {
          router.add('ALL', '*', 'middleware a')
          router.add('GET', '*', 'middleware b')
          router.add('GET', '/entry', 'get entries')
          router.add('POST', '/entry/*', 'middleware c')
          router.add('POST', '/entry', 'post entry')
          router.add('GET', '/entry/:id', 'get entry')
          router.add('GET', '/entry/:id/comment/:comment_id', 'get comment')
        })

        it('GET /', async () => {
          const res = match('GET', '/')
          expect(res.length).toBe(2)
          expect(res[0].handler).toEqual('middleware a')
          expect(res[1].handler).toEqual('middleware b')
        })
        it('GET /entry/123', async () => {
          const res = match('GET', '/entry/123')
          expect(res.length).toBe(3)
          expect(res[0].handler).toEqual('middleware a')
          expect(res[0].params['id']).toBe(undefined)
          expect(res[1].handler).toEqual('middleware b')
          expect(res[1].params['id']).toBe(undefined)
          expect(res[2].handler).toEqual('get entry')
          expect(res[2].params['id']).toBe('123')
        })
        it('GET /entry/123/comment/456', async () => {
          const res = match('GET', '/entry/123/comment/456')
          expect(res.length).toBe(3)
          expect(res[0].handler).toEqual('middleware a')
          expect(res[0].params['id']).toBe(undefined)
          expect(res[0].params['comment_id']).toBe(undefined)
          expect(res[1].handler).toEqual('middleware b')
          expect(res[1].params['id']).toBe(undefined)
          expect(res[1].params['comment_id']).toBe(undefined)
          expect(res[2].handler).toEqual('get comment')
          expect(res[2].params['id']).toBe('123')
          expect(res[2].params['comment_id']).toBe('456')
        })
        it('POST /entry', async () => {
          const res = match('POST', '/entry')
          expect(res.length).toBe(3)
          expect(res[0].handler).toEqual('middleware a')
          expect(res[1].handler).toEqual('middleware c')
          expect(res[2].handler).toEqual('post entry')
        })
        it('DELETE /entry', async () => {
          const res = match('DELETE', '/entry')
          expect(res.length).toBe(1)
          expect(res[0].handler).toEqual('middleware a')
        })
      })

      describe('`params` per a handler', () => {
        beforeEach(() => {
          router.add('ALL', '*', 'middleware a')
          router.add('GET', '/entry/:id/*', 'middleware b')
          router.add('GET', '/entry/:id/:action', 'action')
        })

        it('GET /entry/123/show', async () => {
          const res = match('GET', '/entry/123/show')
          expect(res.length).toBe(3)
          expect(res[0].handler).toEqual('middleware a')
          expect(res[0].params['id']).toBe(undefined)
          expect(res[0].params['action']).toBe(undefined)
          expect(res[1].handler).toEqual('middleware b')
          expect(res[1].params['id']).toBe('123')
          expect(res[1].params['comment_id']).toBe(undefined)
          expect(res[2].handler).toEqual('action')
          expect(res[2].params['id']).toBe('123')
          expect(res[2].params['action']).toBe('show')
        })
      })

      it('hierarchy', () => {
        router.add('GET', '/posts/:id/comments/:comment_id', 'foo')
        router.add('GET', '/posts/:id', 'bar')
        expect(() => {
          router.match('GET', '/')
        }).not.toThrow()
      })
    })

    describe('Duplicate param name', () => {
      it('self', () => {
        router.add('GET', '/:id/:id', 'foo')
        const res = match('GET', '/123/456')
        expect(res.length).toBe(1)
        expect(res[0].handler).toEqual('foo')
        expect(res[0].params['id']).toBe('123')
      })

      it('parent', () => {
        router.add('GET', '/:id/:action', 'foo')
        router.add('GET', '/posts/:id', 'bar')
        const res = match('GET', '/posts/get')
        expect(res.length).toBe(2)
        expect(res[0].handler).toEqual('foo')
        expect(res[0].params['id']).toBe('posts')
        expect(res[0].params['action']).toBe('get')
        expect(res[1].handler).toEqual('bar')
        expect(res[1].params['id']).toBe('get')
      })

      it('child', () => {
        router.add('GET', '/posts/:id', 'foo')
        router.add('GET', '/:id/:action', 'bar')
        const res = match('GET', '/posts/get')
        expect(res.length).toBe(2)
        expect(res[0].handler).toEqual('foo')
        expect(res[0].params['id']).toBe('get')
        expect(res[1].handler).toEqual('bar')
        expect(res[1].params['id']).toBe('posts')
        expect(res[1].params['action']).toBe('get')
      })
    })

    describe('page', () => {
      it('GET /page', async () => {
        router.add('GET', '/page', 'page')
        router.add('ALL', '*', 'fallback') // or '*'

        const res = match('GET', '/page')
        expect(res.length).toBe(2)
        expect(res[0].handler).toEqual('page')
        expect(res[1].handler).toEqual('fallback')
      })
    })

    describe('star', () => {
      beforeEach(() => {
        router.add('GET', '/', '/')
        router.add('GET', '/*', '/*')
        router.add('GET', '*', '*')

        router.add('GET', '/x', '/x')
        router.add('GET', '/x/*', '/x/*')
      })

      it('top', async () => {
        const res = match('GET', '/')
        expect(res.length).toBe(3)
        expect(res[0].handler).toEqual('/')
        expect(res[1].handler).toEqual('/*')
