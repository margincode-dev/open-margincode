### src/helper/route/index.ts
```typescript
import type { Context } from '../../context'
import { GET_MATCH_RESULT } from '../../request/constants'
import type { RouterRoute } from '../../types'
import { getPattern, splitRoutingPath } from '../../utils/url'

/**
 * Get matched routes in the handler
 *
 * @param {Context} c - The context object
 * @returns An array of matched routes
 *
 * @example
 * ```ts
 * import { matchedRoutes } from 'hono/route'
 *
 * app.use('*', async function logger(c, next) {
 *   await next()
 *   matchedRoutes(c).forEach(({ handler, method, path }, i) => {
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
export const matchedRoutes = (c: Context): RouterRoute[] =>
  // @ts-expect-error c.req[GET_MATCH_RESULT] is not typed
  (c.req as unknown)[GET_MATCH_RESULT][0].map(([[, route]]) => route)

/**
 * Get the route path registered within the handler
 *
 * @param {Context} c - The context object
 * @param {number} index - The index of the route from which to retrieve the path, similar to Array.prototype.at(), where a negative number is the index counted from the end of the matching route. Defaults to the current route index.
 * @returns The route path registered within the handler
 *
 * @example
 * ```ts
 * import { routePath } from 'hono/route'
 *
 * app.use('*', (c, next) => {
 *   console.log(routePath(c)) // '*'
 *   console.log(routePath(c, -1)) // '/posts/:id'
 *   return next()
 * })
 *
 * app.get('/posts/:id', (c) => {
 *   return c.text(routePath(c)) // '/posts/:id'
 * })
 * ```
 */
export const routePath = (c: Context, index?: number): string =>
  matchedRoutes(c).at(index ?? c.req.routeIndex)?.path ?? ''

/**
 * Get the basePath of the as-is route specified by routing.
 *
 * @param {Context} c - The context object
 * @param {number} index - The index of the route from which to retrieve the path, similar to Array.prototype.at(), where a negative number is the index counted from the end of the matching route. Defaults to the current route index.
 * @returns The basePath of the as-is route specified by routing.
 *
 * @example
 * ```ts
 * import { baseRoutePath } from 'hono/route'
 *
 * const app = new Hono()
 *
 * const subApp = new Hono()
 * subApp.get('/posts/:id', (c) => {
 *   return c.text(baseRoutePath(c)) // '/:sub'
 * })
 *
 * app.route('/:sub', subApp)
 * ```
 */
export const baseRoutePath = (c: Context, index?: number): string =>
  matchedRoutes(c).at(index ?? c.req.routeIndex)?.basePath ?? ''

/**
 * Get the basePath with embedded parameters
 *
 * @param {Context} c - The context object
 * @param {number} index - The index of the route from which to retrieve the path, similar to Array.prototype.at(), where a negative number is the index counted from the end of the matching route. Defaults to the current route index.
 * @returns The basePath with embedded parameters.
 *
 * @example
 * ```ts
 * import { basePath } from 'hono/route'
 *
 * const app = new Hono()
 *
 * const subApp = new Hono()
 * subApp.get('/posts/:id', (c) => {
 *   return c.text(basePath(c)) // '/requested-sub-app-path'
 * })
 *
 * app.route('/:sub', subApp)
 * ```
 */
const basePathCacheMap: WeakMap<Context, Record<number, string>> = new WeakMap()
export const basePath = (c: Context, index?: number): string => {
  index ??= c.req.routeIndex

  const cache = basePathCacheMap.get(c) || []
  if (typeof cache[index] === 'string') {
    return cache[index]
  }

  let result: string
  const rp = baseRoutePath(c, index)
  if (!/[:*]/.test(rp)) {
    result = rp
  } else {
    const paths = splitRoutingPath(rp)

    const reqPath = c.req.path
    let basePathLength = 0
    for (let i = 0, len = paths.length; i < len; i++) {
      const pattern = getPattern(paths[i], paths[i + 1])
      if (pattern) {
        const re = pattern[2] === true || pattern === '*' ? /[^\/]+/ : pattern[2]
        basePathLength += reqPath.substring(basePathLength + 1).match(re)?.[0].length || 0
      } else {
        basePathLength += paths[i].length
      }
      basePathLength += 1 // for '/'
    }
    result = reqPath.substring(0, basePathLength)
  }

  cache[index] = result
  basePathCacheMap.set(c, cache)

  return result
}
```
### src/helper/route/index.test.ts
```typescript
import { matchedRoutes, routePath, baseRoutePath, basePath } from '.'
import { Context } from '../../context'
import { Hono } from '../../hono'

const defaultContextOptions = {
  executionCtx: {
    waitUntil: () => {},
    passThroughOnException: () => {},
    props: {},
  },
  env: {},
}

describe('matchedRoutes', () => {
  it('should return matched routes', () => {
    const handlerA = () => {}
    const handlerB = () => {}
    const rawRequest = new Request('http://localhost?page=2&tag=A&tag=B')
    const c = new Context(rawRequest, {
      path: '/123/key',
      matchResult: [
        [
          [
            [handlerA, { basePath: '/', handler: handlerA, method: 'GET', path: '/:id' }],
            { id: '123' },
          ],
          [
            [handlerA, { basePath: '/', handler: handlerB, method: 'GET', path: '/:id/:name' }],
            { id: '456', name: 'key' },
          ],
        ],
      ],
      ...defaultContextOptions,
    })

    expect(matchedRoutes(c)).toEqual([
      { basePath: '/', handler: handlerA, method: 'GET', path: '/:id' },
      { basePath: '/', handler: handlerB, method: 'GET', path: '/:id/:name' },
    ])
  })
})

describe('routePath', () => {
  it('should return route path', () => {
    const handlerA = () => {}
    const handlerB = () => {}
    const rawRequest = new Request('http://localhost?page=2&tag=A&tag=B')
    const c = new Context(rawRequest, {
      path: '/123/key',
      matchResult: [
        [
          [
            [handlerA, { basePath: '/', handler: handlerA, method: 'GET', path: '/:id' }],
            { id: '123' },
          ],
          [
            [handlerA, { basePath: '/', handler: handlerB, method: 'GET', path: '/:id/:name' }],
            { id: '456', name: 'key' },
          ],
        ],
      ],
      ...defaultContextOptions,
    })

    expect(routePath(c)).toBe('/:id')

    expect(routePath(c, 0)).toBe('/:id')
    expect(routePath(c, 1)).toBe('/:id/:name')
    expect(routePath(c, -1)).toBe('/:id/:name')
    expect(routePath(c, 2)).toBe('')

    c.req.routeIndex = 1
    expect(routePath(c)).toBe('/:id/:name')
  })
})

describe('baseRoutePath', () => {
  it('should return raw basePath', () => {
    const handlerA = () => {}
    const handlerB = () => {}
    const rawRequest = new Request('http://localhost?page=2&tag=A&tag=B')
    const c = new Context(rawRequest, {
      path: '/123/key',
      matchResult: [
        [
          [
            [handlerA, { basePath: '/', handler: handlerA, method: 'GET', path: '/:id' }],
            { id: '123' },
          ],
          [
            [handlerA, { basePath: '/sub', handler: handlerB, method: 'GET', path: '/:id/:name' }],
            { id: '456', name: 'key' },
          ],
          [
            [handlerA, { basePath: '/:sub', handler: handlerB, method: 'GET', path: '/:id/:name' }],
            { id: '456', name: 'key' },
          ],
        ],
      ],
      ...defaultContextOptions,
    })

    expect(baseRoutePath(c)).toBe('/')

    expect(baseRoutePath(c, 0)).toBe('/')
    expect(baseRoutePath(c, 1)).toBe('/sub')
    expect(baseRoutePath(c, 2)).toBe('/:sub')
    expect(baseRoutePath(c, -1)).toBe('/:sub')
    expect(baseRoutePath(c, 3)).toBe('')

    c.req.routeIndex = 1
    expect(baseRoutePath(c)).toBe('/sub')

    c.req.routeIndex = 2
    expect(baseRoutePath(c)).toBe('/:sub')
  })
})

describe('basePath', () => {
  it('should return basePath without parameters', () => {
    const handlerA = () => {}
    const handlerB = () => {}
    const rawRequest = new Request('http://localhost?page=2&tag=A&tag=B')
    const c = new Context(rawRequest, {
      path: '/123/key',
      matchResult: [
        [
          [
            [handlerA, { basePath: '/', handler: handlerA, method: 'GET', path: '/:id' }],
            { id: '123' },
          ],
          [
            [handlerA, { basePath: '/sub', handler: handlerB, method: 'GET', path: '/:id/:name' }],
            { id: '456', name: 'key' },
          ],
        ],
      ],
      ...defaultContextOptions,
    })

    expect(basePath(c)).toBe('/')
    expect(basePath(c)).toBe('/') // cached value

    expect(basePath(c, 0)).toBe('/')
    expect(basePath(c, 1)).toBe('/sub')
    expect(basePath(c, -1)).toBe('/sub')
    expect(basePath(c, 2)).toBe('')

    c.req.routeIndex = 1
    expect(basePath(c)).toBe('/sub')
  })

  it('should return basePath with embedded parameters', () => {
    const handlerA = () => {}
    const rawRequest = new Request('http://localhost?page=2&tag=A&tag=B')
    const c = new Context(rawRequest, {
      path: '/sub-app-path/123/key',
      matchResult: [
        [
          [
            [handlerA, { basePath: '/:sub', handler: handlerA, method: 'GET', path: '/:id' }],
            { id: '123' },
          ],
        ],
      ],
      ...defaultContextOptions,
    })

    expect(basePath(c)).toBe('/sub-app-path')
  })

  it('should return basePath with wildcard', () => {
    const handlerA = () => {}
    const rawRequest = new Request('http://localhost?page=2&tag=A&tag=B')
    const c = new Context(rawRequest, {
      path: '/sub-app-path/foo/app/123/key',
      matchResult: [
        [
          [
            [
              handlerA,
              { basePath: '/sub-app-path/*/app', handler: handlerA, method: 'GET', path: '/:id' },
            ],
            { id: '123' },
          ],
        ],
      ],
      ...defaultContextOptions,
    })

    expect(basePath(c)).toBe('/sub-app-path/foo/app')
  })

  it('should return the fully expanded basePath for nested route() apps', async () => {
    const app = new Hono()
    const sub1 = new Hono()
    const sub2 = new Hono()
    const sub3 = new Hono()

    sub3.get('/assets/*', (c) => c.json({ baseRoutePath: baseRoutePath(c), basePath: basePath(c) }))
    sub2.route('/:app/sub3/assets', sub3)
    sub1.route('/:app/sub2/assets', sub2)
    app.route('/:app/sub1/assets', sub1)

    const res = await app.request(
      '/s1/sub1/assets/ss2/sub2/assets/sss3/sub3/assets/assets/file-hoge/assets-kage.js'
    )
    expect(await res.json()).toEqual({
      baseRoutePath: '/:app/sub1/assets/:app/sub2/assets/:app/sub3/assets',
      basePath: '/s1/sub1/assets/ss2/sub2/assets/sss3/sub3/assets',
    })
  })

  it('should return basePath with custom regex pattern', () => {
    const handlerA = () => {}
    const rawRequest = new Request('http://localhost?page=2&tag=A&tag=B')
    const c = new Context(rawRequest, {
      path: '/sub-app-path/foo/123/key',
      matchResult: [
        [
          [
            [
              handlerA,
              {
                basePath: '/sub-app-path/:sub{foo|bar}',
                handler: handlerA,
                method: 'GET',
                path: '/:id',
              },
            ],
            { sub: 'foo', id: '123' },
          ],
        ],
      ],
      ...defaultContextOptions,
    })

    expect(basePath(c)).toBe('/sub-app-path/foo')
  })
})
```
### src/middleware/method-not-allowed/index.ts
```typescript
/**
 * @module
 * Method Not Allowed Middleware for Hono.
 */

import type { Context } from '../../context'
import { basePath, matchedRoutes } from '../../helper/route'
import type { Hono } from '../../hono'
import { METHOD_NAME_ALL } from '../../router'
import { TrieRouter } from '../../router/trie-router'
import type { Env, MiddlewareHandler } from '../../types'

type MethodNotAllowedHandler<E extends Env> = (
  c: Context<E>,
  allowedMethods: string[]
) => Response | Promise<Response>

type MethodNotAllowedOptions<E extends Env> = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  app: Hono<E, any, any>
  onMethodNotAllowed?: MethodNotAllowedHandler<E>
}

/**
 * Method Not Allowed Middleware for Hono.
 *
 * Returns a `405 Method Not Allowed` response with an `Allow` header when the request path
 * matches a registered route but the request method is not supported.
 *
 * @param {MethodNotAllowedOptions} options - The options for the middleware.
 * @param {Hono} options.app - The Hono instance used by the application.
 * @param {MethodNotAllowedHandler} [options.onMethodNotAllowed] - Generates the response, including its `Allow` header.
 * @returns {MiddlewareHandler} The middleware handler function.
 *
 * @example
 * ```ts
 * const app = new Hono()
 *
 * app.use(methodNotAllowed({ app }))
 *
 * app.get('/hello', (c) => c.text('Hello!'))
 * app.post('/hello', (c) => c.text('Posted!'))
 *
 * // PUT /hello -> 405 Method Not Allowed
 * // Allow: GET, HEAD, POST
 * ```
 *
 * @example
 * ```ts
 * app.use(
 *   methodNotAllowed({
 *     app,
 *     onMethodNotAllowed: (c, methods) =>
 *       c.json({ error: 'Method Not Allowed' }, 405, { Allow: methods.join(', ') }),
 *   })
 * )
 * ```
 */
export const methodNotAllowed = <E extends Env = Env>(
  options: MethodNotAllowedOptions<E>
): MiddlewareHandler<E> => {
  let methodRouter: TrieRouter<string[]> | undefined

  return async function methodNotAllowed(c, next) {
    const routeIndex = c.req.routeIndex
    await next()

    if (c.res.status !== 404) {
      return
    }

    if (c.error) {
      return
    }

    if (!methodRouter) {
      const methodsByPath = new Map<string, Set<string>>()

      for (const route of options.app.routes) {
        // Hono does not distinguish middleware registered with `app.use()` from handlers
        // registered with `app.all()`, so `ALL` routes cannot contribute to the Allow header.
        if (route.method === METHOD_NAME_ALL || route.method === 'HEAD') {
          continue
        }

        const methods = methodsByPath.get(route.path) ?? new Set<string>()
        methods.add(route.method)

        // Hono dispatches HEAD requests to GET routes.
        if (route.method === 'GET') {
          methods.add('HEAD')
        }

        methodsByPath.set(route.path, methods)
      }

      methodRouter = new TrieRouter()
      for (const [path, methods] of methodsByPath) {
        methodRouter.add(METHOD_NAME_ALL, path, [...methods])
      }
    }

    let requestPath = c.req.path
    const routes = matchedRoutes(c)
    const currentRoute = routes[routeIndex]
    const sourceRoute = options.app.routes.find((route) => route.handler === methodNotAllowed)
    if (currentRoute && sourceRoute) {
      const currentBasePathParts = basePath(c, routeIndex).split('/').filter(Boolean)
      const sourceBasePathLength = sourceRoute.basePath.split('/').filter(Boolean).length
      const mountBasePathParts = currentBasePathParts.slice(
        0,
        currentBasePathParts.length - sourceBasePathLength
      )
      if (mountBasePathParts.length > 0) {
        const mountBasePath = `/${mountBasePathParts.join('/')}`
        requestPath = c.req.path.slice(mountBasePath.length) || '/'
      }
    }

    const allowedMethods = new Set<string>()
    for (const [methods] of methodRouter.match(METHOD_NAME_ALL, requestPath)[0]) {
      for (const method of methods) {
        allowedMethods.add(method)
      }
    }

