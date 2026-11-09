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
  request = (
    input: Request | string | URL,
    requestInit?: RequestInit,
    Env?: E['Bindings'] | {},
    executionCtx?: ExecutionContext
  ): Response | Promise<Response> => {
    if (input instanceof Request) {
      return this.fetch(requestInit ? new Request(input, requestInit) : input, Env, executionCtx)
    }
    input = input.toString()
    return this.fetch(
      new Request(
        /^https?:\/\//.test(input) ? input : `http://localhost${mergePath('/', input)}`,
        requestInit
      ),
      Env,
      executionCtx
    )
  }

  /**
   * `.fire()` automatically adds a global fetch event listener.
   * This can be useful for environments that adhere to the Service Worker API, such as non-ES module Cloudflare Workers.
   * @deprecated
   * Use `fire` from `hono/service-worker` instead.
   * ```ts
   * import { Hono } from 'hono'
   * import { fire } from 'hono/service-worker'
   *
   * const app = new Hono()
   * // ...
   * fire(app)
   * ```
   * @see https://hono.dev/docs/api/hono#fire
   * @see https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API
   * @see https://developers.cloudflare.com/workers/reference/migrate-to-module-workers/
   */
  fire = (): void => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    addEventListener('fetch', (event: FetchEventLike): void => {
      event.respondWith(this.#dispatch(event.request, event, undefined, event.request.method))
    })
  }
}

export { Hono as HonoBase }
```
### src/utils/url.test.ts
```typescript
import {
  checkOptionalParameter,
  getPath,
  getPathNoStrict,
  getPattern,
  getQueryParam,
  getQueryParams,
  getQueryStrings,
  mergePath,
  splitPath,
  splitRoutingPath,
} from './url'

describe('url', () => {
  it('splitPath', () => {
    let ps = splitPath('/')
    expect(ps).toStrictEqual([''])

    ps = splitPath('/hello')
    expect(ps).toStrictEqual(['hello'])
  })

  it('splitRoutingPath', () => {
    let ps = splitRoutingPath('/')
    expect(ps).toStrictEqual([''])

    ps = splitRoutingPath('/hello')
    expect(ps).toStrictEqual(['hello'])

    ps = splitRoutingPath('*')
    expect(ps).toStrictEqual(['*'])

    ps = splitRoutingPath('/wildcard-abc/*/wildcard-efg')
    expect(ps).toStrictEqual(['wildcard-abc', '*', 'wildcard-efg'])

    ps = splitRoutingPath('/map/:location/events')
    expect(ps).toStrictEqual(['map', ':location', 'events'])

    ps = splitRoutingPath('/js/:location{[a-z/]+.js}')
    expect(ps).toStrictEqual(['js', ':location{[a-z/]+.js}'])

    ps = splitRoutingPath('/users/:name{[0-9a-zA-Z_-]{3,10}}')
    expect(ps).toStrictEqual(['users', ':name{[0-9a-zA-Z_-]{3,10}}'])

    ps = splitRoutingPath('/users/:@name{[0-9a-zA-Z_-]{3,10}}')
    expect(ps).toStrictEqual(['users', ':@name{[0-9a-zA-Z_-]{3,10}}'])

    ps = splitRoutingPath('/users/:dept{\\d+}/:@name{[0-9a-zA-Z_-]{3,10}}')
    expect(ps).toStrictEqual(['users', ':dept{\\d+}', ':@name{[0-9a-zA-Z_-]{3,10}}'])
  })

  describe('getPattern', () => {
    it('no pattern', () => {
      const res = getPattern('id')
      expect(res).toBeNull()
    })

    it('no pattern with next', () => {
      const res = getPattern('id', 'next')
      expect(res).toBeNull()
    })

    it('default pattern', () => {
      const res = getPattern(':id')
      expect(res).toEqual([':id', 'id', true])
    })

    it('default pattern with next', () => {
      const res = getPattern(':id', 'next')
      expect(res).toEqual([':id', 'id', true])
    })

    it('regex pattern', () => {
      const res = getPattern(':id{[0-9]+}')
      expect(res).toEqual([':id{[0-9]+}', 'id', /^[0-9]+$/])
    })

    it('regex pattern with next', () => {
      const res = getPattern(':id{[0-9]+}', 'next')
      expect(res).toEqual([':id{[0-9]+}#next', 'id', /^[0-9]+(?=\/next)/])
    })

    it('wildcard', () => {
      const res = getPattern('*')
      expect(res).toBe('*')
    })

    it('wildcard with next', () => {
      const res = getPattern('*', 'next')
      expect(res).toBe('*')
    })
  })

  describe('getPath', () => {
    it('getPath - no trailing slash', () => {
      let path = getPath(new Request('https://example.com/'))
      expect(path).toBe('/')
      path = getPath(new Request('https://example.com/hello'))
      expect(path).toBe('/hello')
      path = getPath(new Request('https://example.com/hello/hey'))
      expect(path).toBe('/hello/hey')
      path = getPath(new Request('https://example.com/hello?name=foo'))
      expect(path).toBe('/hello')
      path = getPath(new Request('https://example.com/hello/hey?name=foo&name=bar'))
      expect(path).toBe('/hello/hey')
    })

    it('getPath - with trailing slash', () => {
      let path = getPath(new Request('https://example.com/hello/'))
      expect(path).toBe('/hello/')
      path = getPath(new Request('https://example.com/hello/hey/'))
      expect(path).toBe('/hello/hey/')
    })

    it('getPath - http+unix', () => {
      const path = getPath(new Request('http+unix://%2Ftmp%2Fsocket%2Esock/hello/'))
      expect(path).toBe('/hello/')
    })

    it.each([
      'http:/example.com/hello', // invalid HTTP URL
      'http:///hello', // invalid HTTP URL
      'http://a/:/hello', // starts with `/:/`
      'x://a/:/hello', // unknown schema
    ])('getPath - %s', (url) => {
      expect(getPath(new Request(url))).toBe(new URL(url).pathname)
    })

    it('getPath - with fragment', () => {
      let path = getPath(new Request('https://example.com/users/#user-list'))
      expect(path).toBe('/users/')
      path = getPath(new Request('https://example.com/users/1#profile-section'))
      expect(path).toBe('/users/1')
      path = getPath(new Request('https://example.com/hello#section'))
      expect(path).toBe('/hello')
      path = getPath(new Request('https://example.com/#top'))
      expect(path).toBe('/')
    })

    it('getPath - with query and fragment', () => {
      let path = getPath(new Request('https://example.com/hello?name=foo#section'))
      expect(path).toBe('/hello')
      path = getPath(new Request('https://example.com/search?q=test#results'))
      expect(path).toBe('/search')
    })

    it('getPath - with percent encoding only (no query or fragment)', () => {
      const path = getPath(new Request('https://example.com/hello%20world'))
      expect(path).toBe('/hello world')
    })

    it('getPath - with percent encoding and fragment', () => {
      let path = getPath(new Request('https://example.com/hello%20world#section'))
      expect(path).toBe('/hello world')
      path = getPath(new Request('https://example.com/%E7%82%8E#top'))
      expect(path).toBe('/炎')
    })

    it('getPath - with percent encoding and fragment containing query-like chars', () => {
      const path = getPath(new Request('https://example.com/hello%20world#section?foo=bar'))
      expect(path).toBe('/hello world')
    })

    it('getPath - with encoded hash (%23) in path and real fragment', () => {
      // %23 is encoded '#' - decodeURI preserves reserved characters, so %23 stays as %23
      let path = getPath(new Request('https://example.com/path%23test#real-fragment'))
      expect(path).toBe('/path%23test')
      path = getPath(new Request('https://example.com/foo%23bar%23baz#section'))
      expect(path).toBe('/foo%23bar%23baz')
      // Only encoded hash, no real fragment
      path = getPath(new Request('https://example.com/issue%23123'))
      expect(path).toBe('/issue%23123')
    })
  })

  describe('getQueryStrings', () => {
    it('getQueryStrings', () => {
      let qs = getQueryStrings('https://example.com/hello?name=foo&name=bar&age=20')
      expect(qs).toBe('?name=foo&name=bar&age=20')
      qs = getQueryStrings('https://example.com/hello?')
      expect(qs).toBe('?')
      qs = getQueryStrings('https://example.com/hello')
      expect(qs).toBe('')
      qs = getQueryStrings('https://example.com/hello?name=foo&name=bar&age=20#hash')
      expect(qs).toBe('?name=foo&name=bar&age=20')
      qs = getQueryStrings('https://example.com/hello#?name=probe')
      expect(qs).toBe('')
      qs = getQueryStrings('https://example.com/hello?name=foo%23bar#hash')
      expect(qs).toBe('?name=foo%23bar')
    })
  })

  describe('getPathNoStrict', () => {
    it('getPathNoStrict - no strict is false', () => {
      let path = getPathNoStrict(new Request('https://example.com/hello/'))
      expect(path).toBe('/hello')
      path = getPathNoStrict(new Request('https://example.com/hello/hey/'))
      expect(path).toBe('/hello/hey')
    })

    it('getPathNoStrict - return `/` even if strict is false', () => {
      const path = getPathNoStrict(new Request('https://example.com/'))
      expect(path).toBe('/')
    })
  })

  describe('mergePath', () => {
    it('mergePath', () => {
      expect(mergePath('/book', '/')).toBe('/book')
      expect(mergePath('/book/', '/')).toBe('/book/')
      expect(mergePath('/book', '/hey')).toBe('/book/hey')
      expect(mergePath('/book/', '/hey')).toBe('/book/hey')
      expect(mergePath('/book', '/hey/')).toBe('/book/hey/')
      expect(mergePath('/book/', '/hey/')).toBe('/book/hey/')
      expect(mergePath('/book', 'hey', 'say')).toBe('/book/hey/say')
      expect(mergePath('/book', '/hey/', '/say/')).toBe('/book/hey/say/')
      expect(mergePath('/book', '/hey/', '/say/', '/')).toBe('/book/hey/say/')
      expect(mergePath('/book', '/hey', '/say', '/')).toBe('/book/hey/say')
      expect(mergePath('/', '/book', '/hey', '/say', '/')).toBe('/book/hey/say')

      expect(mergePath('book', '/')).toBe('/book')
      expect(mergePath('book/', '/')).toBe('/book/')
      expect(mergePath('book', '/hey')).toBe('/book/hey')
      expect(mergePath('book', 'hey')).toBe('/book/hey')
      expect(mergePath('book', 'hey/')).toBe('/book/hey/')
    })
    it('Should be `/book`', () => {
      expect(mergePath('/', 'book')).toBe('/book')
    })
    it('Should be `/book`', () => {
      expect(mergePath('/', '/book')).toBe('/book')
    })
    it('Should be `/`', () => {
      expect(mergePath('/', '/')).toBe('/')
    })
  })

  describe('checkOptionalParameter', () => {
    it('checkOptionalParameter', () => {
      expect(checkOptionalParameter('/api/animals/:type?')).toEqual([
        '/api/animals',
        '/api/animals/:type',
      ])
      expect(checkOptionalParameter('/api/animals/type?')).toBeNull()
      expect(checkOptionalParameter('/api/animals/:type')).toBeNull()
      expect(checkOptionalParameter('/api/animals')).toBeNull()
      expect(checkOptionalParameter('/api/:animals?/type')).toBeNull()
      expect(checkOptionalParameter('/api/animals/:type?/')).toBeNull()
      expect(checkOptionalParameter('/:optional?')).toEqual(['/', '/:optional'])
      expect(checkOptionalParameter('/v1/leaderboard/:version?/:platform?')).toEqual([
        '/v1/leaderboard',
        '/v1/leaderboard/:version',
        '/v1/leaderboard/:version/:platform',
      ])
      expect(checkOptionalParameter('/api/:version/animal/:type?')).toEqual([
        '/api/:version/animal',
        '/api/:version/animal/:type',
      ])
      expect(checkOptionalParameter('/api/:id{[0-9]?}?')).toEqual(['/api', '/api/:id{[0-9]?}'])
    })
  })

  describe('getQueryParam', () => {
    it('Parse URL query strings', () => {
      expect(getQueryParam('http://example.com/?name=hey', 'name')).toBe('hey')
      expect(getQueryParam('http://example.com/?name=hey#fragment', 'name')).toBe('hey')
      expect(getQueryParam('http://example.com/#?name=probe', 'name')).toBe(undefined)
      expect(getQueryParam('http://example.com/?name=hey%23there#fragment', 'name')).toBe(
        'hey#there'
      )
      expect(getQueryParam('http://example.com/?safe=1#&admin=true')).toEqual({ safe: '1' })
      expect(getQueryParam('http://example.com/?name=hey&age=20&tall=170', 'age')).toBe('20')
      expect(getQueryParam('http://example.com/?Hono+is=a+web+framework', 'Hono is')).toBe(
        'a web framework'
      )
      expect(getQueryParam('http://example.com/?name=%E0%A4%A', 'name')).toBe('%E0%A4%A')

      expect(getQueryParam('http://example.com/?name0=sam&name1=tom', 'name0')).toBe('sam')
      expect(getQueryParam('http://example.com/?name0=sam&name1=tom', 'name1')).toBe('tom')
      expect(getQueryParam('http://example.com/?name0=sam&name1=tom', 'name')).toBe(undefined)

      let searchParams = new URLSearchParams({ name: '炎' })
      expect(getQueryParam(`http://example.com/?${searchParams.toString()}`, 'name')).toBe('炎')
      searchParams = new URLSearchParams({ '炎 is': 'a web framework' })
      expect(
        getQueryParam(
          `http://example.com/?${searchParams.toString()}`,
          searchParams.keys().next().value
        )
      ).toBe('a web framework')
      expect(getQueryParam('http://example.com/?name=hey&age=20&tall=170', 'weight')).toBe(
        undefined
      )
      expect(getQueryParam('http://example.com/?name=hey&age=20&tall=170')).toEqual({
        name: 'hey',
        age: '20',
        tall: '170',
      })
      expect(getQueryParam('http://example.com/?pretty&&&&q=1%2b1=2')).toEqual({
        pretty: '',
        q: '1+1=2',
      })
      expect(getQueryParam('http://example.com/?pretty', 'pretty')).toBe('')
      expect(getQueryParam('http://example.com/?pretty', 'prtt')).toBe(undefined)
      expect(getQueryParam('http://example.com/?name=sam&name=tom', 'name')).toBe('sam')
      expect(getQueryParam('http://example.com/&name=sam?name=tom', 'name')).toBe('tom')
      expect(getQueryParam('http://example.com/&name=sam', 'name')).toBe(undefined)
      expect(getQueryParam('http://example.com/?name=sam&name=tom')).toEqual({
        name: 'sam',
      })
      searchParams = new URLSearchParams('?name=sam=tom')
      expect(getQueryParam('name', searchParams.get('name')?.toString()))
    })
  })

  describe('getQueryParams', () => {
    it('Parse URL query strings', () => {
      expect(getQueryParams('http://example.com/?name=hey', 'name')).toEqual(['hey'])
      expect(getQueryParams('http://example.com/?name=hey#fragment', 'name')).toEqual(['hey'])
      expect(getQueryParams('http://example.com/#?name=probe', 'name')).toBe(undefined)
      expect(getQueryParams('http://example.com/?name=hey%23there#fragment', 'name')).toEqual([
        'hey#there',
      ])
      expect(getQueryParams('http://example.com/?safe=1#&admin=true')).toEqual({ safe: ['1'] })
      expect(getQueryParams('http://example.com/?name=hey&name=foo', 'name')).toEqual([
        'hey',
        'foo',
      ])
      expect(getQueryParams('http://example.com/?name=hey&age=20&tall=170', 'age')).toEqual(['20'])
      expect(
        getQueryParams('http://example.com/?name=hey&age=20&tall=170&name=foo&age=30', 'age')
      ).toEqual(['20', '30'])
      expect(getQueryParams('http://example.com/?Hono+is=a+web+framework', 'Hono is')).toEqual([
        'a web framework',
      ])
      expect(getQueryParams('http://example.com/?name=%E0%A4%A', 'name')).toEqual(['%E0%A4%A'])

      let searchParams = new URLSearchParams()
      searchParams.append('tag', '炎')
      searchParams.append('tag', 'ほのお')
      expect(getQueryParams(`http://example.com/?${searchParams.toString()}`, 'tag')).toEqual([
