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

    if (allowedMethods.size === 0 || allowedMethods.has(c.req.method)) {
      return
    }

    // Prevent stale headers from being carried over from the 404 response.
    c.res.headers.delete('Allow')
    c.res.headers.delete('Content-Length')

    const methods = [...allowedMethods]
    const allow = methods.join(', ')
    c.res = options.onMethodNotAllowed
      ? await options.onMethodNotAllowed(c, methods)
      : c.text('Method Not Allowed', 405, { Allow: allow })
  }
}
```
### src/hono-base.ts
```typescript
/**
 * @module
 * This module is the base module for the Hono object.
 */

/* eslint-disable @typescript-eslint/no-explicit-any */
import { compose } from './compose'
import { Context } from './context'
import type { ExecutionContext } from './context'
import type { Router } from './router'
import { METHODS, METHOD_NAME_ALL, METHOD_NAME_ALL_LOWERCASE } from './router'
import type {
  Env,
  ErrorHandler,
  FetchEventLike,
  H,
  HandlerInterface,
  MergePath,
  MergeSchemaPath,
  MiddlewareHandler,
  MiddlewareHandlerInterface,
  Next,
  NotFoundHandler,
  OnHandlerInterface,
  RouterRoute,
  Schema,
} from './types'
import { COMPOSED_HANDLER } from './utils/constants'
import { getPath, getPathNoStrict, mergePath } from './utils/url'

const notFoundHandler: NotFoundHandler = (c) => {
  return c.text('404 Not Found', 404)
}

const errorHandler: ErrorHandler = (err, c) => {
  if ('getResponse' in err) {
    const res = err.getResponse()
    return c.newResponse(res.body, res)
  }
  console.error(err)
  return c.text('Internal Server Error', 500)
}

type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string

export type HonoOptions<E extends Env> = {
  /**
   * `strict` option specifies whether to distinguish whether the last path is a directory or not.
   *
   * @see {@link https://hono.dev/docs/api/hono#strict-mode}
   *
   * @default true
   */
  strict?: boolean
  /**
   * `router` option specifies which router to use.
   *
   * @see {@link https://hono.dev/docs/api/hono#router-option}
   *
   * @example
   * ```ts
   * const app = new Hono({ router: new RegExpRouter() })
   * ```
   */
  router?: Router<[H, RouterRoute]>
  /**
   * `getPath` can handle the host header value.
   *
   * @see {@link https://hono.dev/docs/api/routing#routing-with-host-header-value}
   *
   * @example
   * ```ts
   * const app = new Hono({
   *  getPath: (req) =>
   *   '/' + req.headers.get('host') + req.url.replace(/^https?:\/\/[^/]+(\/[^?]*)/, '$1'),
   * })
   *
   * app.get('/www1.example.com/hello', () => c.text('hello www1'))
   *
   * // A following request will match the route:
   * // new Request('http://www1.example.com/hello', {
   * //  headers: { host: 'www1.example.com' },
   * // })
   * ```
   */
  getPath?: GetPath<E>
}

type MountOptionHandler = (c: Context) => unknown
type MountReplaceRequest = (originalRequest: Request) => Request
type MountOptions =
  | MountOptionHandler
  | {
      optionHandler?: MountOptionHandler
      replaceRequest?: MountReplaceRequest | false
    }

class Hono<
  E extends Env = Env,
  S extends Schema = {},
  BasePath extends string = '/',
  CurrentPath extends string = BasePath,
> {
  get!: HandlerInterface<E, 'get', S, BasePath, CurrentPath>
  post!: HandlerInterface<E, 'post', S, BasePath, CurrentPath>
  put!: HandlerInterface<E, 'put', S, BasePath, CurrentPath>
  delete!: HandlerInterface<E, 'delete', S, BasePath, CurrentPath>
  options!: HandlerInterface<E, 'options', S, BasePath, CurrentPath>
  patch!: HandlerInterface<E, 'patch', S, BasePath, CurrentPath>
  query!: HandlerInterface<E, 'query', S, BasePath, CurrentPath>
  all!: HandlerInterface<E, 'all', S, BasePath, CurrentPath>
  on: OnHandlerInterface<E, S, BasePath>
  use: MiddlewareHandlerInterface<E, S, BasePath>

  /*
    This class is like an abstract class and does not have a router.
    To use it, inherit the class and implement router in the constructor.
  */
  router!: Router<[H, RouterRoute]>
  readonly getPath: GetPath<E>
  // Cannot use `#` because it requires visibility at JavaScript runtime.
  private _basePath: string = '/'
  #path: string = '/'

  routes: RouterRoute[] = []

  constructor(options: HonoOptions<E> = {}) {
    // Implementation of app.get(...handlers[]) or app.get(path, ...handlers[])
    const allMethods = [...METHODS, METHOD_NAME_ALL_LOWERCASE]
    allMethods.forEach((method) => {
      this[method] = (args1: string | H, ...args: H[]) => {
        const methodName = method.toUpperCase()
        if (typeof args1 === 'string') {
          this.#path = args1
        } else {
          this.#addRoute(methodName, this.#path, args1)
        }
        args.forEach((handler) => {
          this.#addRoute(methodName, this.#path, handler)
        })
        return this as any
      }
    })

    // Implementation of app.on(method, path, ...handlers[])
    this.on = (method: string | string[], path: string | string[], ...handlers: H[]) => {
      for (const p of [path].flat()) {
        this.#path = p
        for (const m of [method].flat()) {
          const methodName = m.toUpperCase()
          for (const handler of handlers) {
            this.#addRoute(methodName, this.#path, handler)
          }
        }
      }
      return this as any
    }

    // Implementation of app.use(...handlers[]) or app.use(path, ...handlers[])
    this.use = (arg1: string | MiddlewareHandler<any>, ...handlers: MiddlewareHandler<any>[]) => {
      if (typeof arg1 === 'string') {
        this.#path = arg1
      } else {
        this.#path = '*'
        handlers.unshift(arg1)
      }
      handlers.forEach((handler) => {
        this.#addRoute(METHOD_NAME_ALL, this.#path, handler)
      })
      return this as any
    }

    const { strict, ...optionsWithoutStrict } = options
    Object.assign(this, optionsWithoutStrict)
    this.getPath = (strict ?? true) ? (options.getPath ?? getPath) : getPathNoStrict
  }

  #clone(): Hono<E, S, BasePath, CurrentPath> {
    const clone = new Hono<E, S, BasePath, CurrentPath>({
      router: this.router,
      getPath: this.getPath,
    })
    clone.errorHandler = this.errorHandler
    clone.#notFoundHandler = this.#notFoundHandler
    clone.routes = this.routes
    return clone
  }

  #notFoundHandler: NotFoundHandler = notFoundHandler
  // Cannot use `#` because it requires visibility at JavaScript runtime.
  private errorHandler: ErrorHandler = errorHandler

  /**
   * `.route()` allows grouping other Hono instance in routes.
   *
   * @see {@link https://hono.dev/docs/api/routing#grouping}
   *
   * @param {string} path - base Path
   * @param {Hono} app - other Hono instance
   * @returns {Hono} routed Hono instance
   *
   * @example
   * ```ts
   * const app = new Hono()
   * const app2 = new Hono()
   *
   * app2.get("/user", (c) => c.text("user"))
   * app.route("/api", app2) // GET /api/user
   * ```
   */
  route<
    SubPath extends string,
    SubEnv extends Env,
    SubSchema extends Schema,
    SubBasePath extends string,
    SubCurrentPath extends string,
  >(
    path: SubPath,
    app: Hono<SubEnv, SubSchema, SubBasePath, SubCurrentPath>
  ): Hono<E, MergeSchemaPath<SubSchema, MergePath<BasePath, SubPath>> | S, BasePath, CurrentPath> {
    const subApp = this.basePath(path)
    app.routes.map((r) => {
      let handler
      if (app.errorHandler === errorHandler) {
        handler = r.handler
      } else {
        handler = async (c: Context, next: Next) =>
          (await compose([], app.errorHandler)(c, () => r.handler(c, next))).res
        ;(handler as any)[COMPOSED_HANDLER] = r.handler
      }

      subApp.#addRoute(r.method, r.path, handler, r.basePath)
    })
    return this
  }

  /**
   * `.basePath()` allows base paths to be specified.
   *
   * @see {@link https://hono.dev/docs/api/routing#base-path}
   *
   * @param {string} path - base Path
   * @returns {Hono} changed Hono instance
   *
   * @example
   * ```ts
   * const api = new Hono().basePath('/api')
   * ```
   */
  basePath<SubPath extends string>(
    path: SubPath
  ): Hono<E, S, MergePath<BasePath, SubPath>, MergePath<BasePath, SubPath>> {
    const subApp = this.#clone()
    subApp._basePath = mergePath(this._basePath, path)
    return subApp
  }

  /**
   * `.onError()` handles an error and returns a customized Response.
   *
   * @see {@link https://hono.dev/docs/api/hono#error-handling}
   *
   * @param {ErrorHandler} handler - request Handler for error
   * @returns {Hono} changed Hono instance
   *
   * @example
   * ```ts
   * app.onError((err, c) => {
   *   console.error(`${err}`)
   *   return c.text('Custom Error Message', 500)
   * })
   * ```
   */
  onError = (handler: ErrorHandler<E>): Hono<E, S, BasePath, CurrentPath> => {
    this.errorHandler = handler
    return this
  }

  /**
   * `.notFound()` allows you to customize a Not Found Response.
   *
   * @see {@link https://hono.dev/docs/api/hono#not-found}
   *
   * @param {NotFoundHandler} handler - request handler for not-found
   * @returns {Hono} changed Hono instance
   *
   * @example
   * ```ts
   * app.notFound((c) => {
   *   return c.text('Custom 404 Message', 404)
   * })
   * ```
   */
  notFound = (handler: NotFoundHandler<E>): Hono<E, S, BasePath, CurrentPath> => {
    this.#notFoundHandler = handler
    return this
  }

  /**
   * `.mount()` allows you to mount applications built with other frameworks into your Hono application.
   *
   * @deprecated Use `mount()` from `hono/mount` instead. `.mount()` will be removed in v5.
   *
   * @see {@link https://hono.dev/docs/api/hono#mount}
   *
   * @param {string} path - base Path
   * @param {Function} applicationHandler - other Request Handler
   * @param {MountOptions} [options] - options of `.mount()`
   * @returns {Hono} mounted Hono instance
   *
   * @example
   * ```ts
   * import { Router as IttyRouter } from 'itty-router'
   * import { Hono } from 'hono'
   * // Create itty-router application
   * const ittyRouter = IttyRouter()
   * // GET /itty-router/hello
   * ittyRouter.get('/hello', () => new Response('Hello from itty-router'))
   *
   * const app = new Hono()
   * app.mount('/itty-router', ittyRouter.handle)
   * ```
   *
   * @example
   * ```ts
   * const app = new Hono()
   * // Send the request to another application without modification.
   * app.mount('/app', anotherApp, {
   *   replaceRequest: (req) => req,
   * })
   * ```
   */
  mount(
    path: string,
    applicationHandler: (request: Request, ...args: any) => Response | Promise<Response>,
    options?: MountOptions
  ): Hono<E, S, BasePath, CurrentPath> {
    // handle options
    let replaceRequest: MountReplaceRequest | undefined
    let optionHandler: MountOptionHandler | undefined
    if (options) {
      if (typeof options === 'function') {
        optionHandler = options
      } else {
        optionHandler = options.optionHandler
        if (options.replaceRequest === false) {
          replaceRequest = (request) => request
        } else {
          replaceRequest = options.replaceRequest
        }
      }
    }

    // prepare handlers for request
    const getOptions: (c: Context) => unknown[] = optionHandler
      ? (c) => {
          const options = optionHandler!(c)
          return Array.isArray(options) ? options : [options]
        }
      : (c) => {
          let executionContext: ExecutionContext | undefined = undefined
          try {
            executionContext = c.executionCtx
          } catch {} // Do nothing
          return [c.env, executionContext]
        }
    replaceRequest ||= (() => {
      const mergedPath = mergePath(this._basePath, path)
      const pathPrefixLength = mergedPath === '/' ? 0 : mergedPath.length
      return (request) => {
        const url = new URL(request.url)
        url.pathname = this.getPath(request).slice(pathPrefixLength) || '/'
        return new Request(url, request)
      }
    })()

    const handler: MiddlewareHandler = async (c, next) => {
      const res = await applicationHandler(replaceRequest(c.req.raw), ...getOptions(c))

      if (res) {
        return res
      }

      await next()
    }
    this.#addRoute(METHOD_NAME_ALL, mergePath(path, '*'), handler)
    return this
  }

  #addRoute(method: string, path: string, handler: H, baseRoutePath?: string): void {
    path = mergePath(this._basePath, path)
    const r: RouterRoute = {
      basePath:
        baseRoutePath !== undefined ? mergePath(this._basePath, baseRoutePath) : this._basePath,
      path,
      method,
      handler,
    }
    this.router.add(method, path, [handler, r])
    this.routes.push(r)
  }

  #handleError(err: unknown, c: Context<E>): Response | Promise<Response> {
    if (err instanceof Error) {
      return this.errorHandler(err, c)
    }
    throw err
  }

  #dispatch(
    request: Request,
    executionCtx: ExecutionContext | FetchEventLike | undefined,
    env: E['Bindings'],
    method: string
  ): Response | Promise<Response> {
    // Handle HEAD method
    if (method === 'HEAD') {
      return (async () =>
        new Response(null, await this.#dispatch(request, executionCtx, env, 'GET')))()
    }

    const path = this.getPath(request, { env })
    const matchResult = this.router.match(method, path)

    const c = new Context(request, {
      path,
      matchResult,
      env,
      executionCtx,
      notFoundHandler: this.#notFoundHandler,
    })

    // Do not `compose` if it has only one handler
    if (matchResult[0].length === 1) {
      let res: ReturnType<H>
      try {
        res = matchResult[0][0][0][0](c, async () => {
          c.res = await this.#notFoundHandler(c)
        })
      } catch (err) {
        return this.#handleError(err, c)
      }

      return res instanceof Promise
        ? res
            .then(
              (resolved: Response | undefined) =>
                resolved || (c.finalized ? c.res : this.#notFoundHandler(c))
            )
            .catch((err: Error) => this.#handleError(err, c))
        : (res ?? this.#notFoundHandler(c))
    }

    const composed = compose(matchResult[0], this.errorHandler, this.#notFoundHandler)

    return (async () => {
      try {
        const context = await composed(c)
        if (!context.finalized) {
          throw new Error(
            'Context is not finalized. Did you forget to return a Response object or `await next()`?'
          )
        }

        return context.res
      } catch (err) {
        return this.#handleError(err, c)
      }
    })()
  }

  /**
   * `.fetch()` will be entry point of your app.
   *
   * @see {@link https://hono.dev/docs/api/hono#fetch}
   *
   * @param {Request} request - request Object of request
   * @param {Env} env - env Object
   * @param {ExecutionContext} executionCtx - context of execution
   * @returns {Response | Promise<Response>} response of request
   *
   */
  fetch: (
    request: Request,
    env?: E['Bindings'] | {},
    executionCtx?: ExecutionContext
  ) => Response | Promise<Response> = (request, ...rest) => {
    return this.#dispatch(request, rest[1], rest[0], request.method)
  }

  /**
   * `.request()` is a useful method for testing.
   * You can pass a URL or pathname to send a GET request.
   * app will return a Response object.
   * ```ts
   * test('GET /hello is ok', async () => {
   *   const res = await app.request('/hello')
   *   expect(res.status).toBe(200)
   * })
   * ```
   * @see https://hono.dev/docs/api/hono#request
   */
