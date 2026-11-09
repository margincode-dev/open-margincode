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
