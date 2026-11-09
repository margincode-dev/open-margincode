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

