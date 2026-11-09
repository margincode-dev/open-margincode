- HonoOptions (src/hono-base.ts:46-87)
```typescript
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
```
- Hono.constructor (src/hono-base.ts:127-176)
```typescript
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
```
- Hono.#clone (src/hono-base.ts:178-187)
```typescript
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
```
- GetPath (src/hono-base.ts:44-44)
```typescript
type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string
```
- Hono (src/hono-base.ts:98-547)
```typescript
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
```
- getPathNoStrict (src/utils/url.ts:154-159)
```typescript
export const getPathNoStrict = (request: Request): string => {
  const result = getPath(request)

  // if strict routing is false => `/hello/hey/` and `/hello/hey` are treated the same
  return result.length > 1 && result.at(-1) === '/' ? result.slice(0, -1) : result
}
```
- Hono (src/preset/tiny.ts:11-20)
```typescript
export class Hono<
  E extends Env = BlankEnv,
  S extends Schema = BlankSchema,
  BasePath extends string = '/',
> extends HonoBase<E, S, BasePath> {
  constructor(options: HonoOptions<E> = {}) {
    super(options)
    this.router = new PatternRouter()
  }
}
```
- Hono (src/preset/quick.ts:13-24)
```typescript
export class Hono<
  E extends Env = BlankEnv,
  S extends Schema = BlankSchema,
  BasePath extends string = '/',
> extends HonoBase<E, S, BasePath> {
  constructor(options: HonoOptions<E> = {}) {
    super(options)
    this.router = new SmartRouter({
      routers: [new LinearRouter(), new TrieRouter()],
    })
  }
}
```
- Hono (src/hono.ts:16-34)
```typescript
export class Hono<
  E extends Env = BlankEnv,
  S extends Schema = BlankSchema,
  BasePath extends string = '/',
> extends HonoBase<E, S, BasePath> {
  /**
   * Creates an instance of the Hono class.
   *
   * @param options - Optional configuration options for the Hono instance.
   */
  constructor(options: HonoOptions<E> = {}) {
    super(options)
    this.router =
      options.router ??
      new SmartRouter({
        routers: [new RegExpRouter(), new TrieRouter()],
      })
  }
}
```
- ToSSGAdaptorInterface (src/helper/ssg/ssg.ts:356-362)
```typescript
export interface ToSSGAdaptorInterface<
  E extends Env = Env,
  S extends Schema = {},
  BasePath extends string = '/',
> {
  (app: Hono<E, S, BasePath>, options?: ToSSGOptions): Promise<ToSSGResult>
}
```
- ALBProcessor.getPath (adapters/aws-lambda/src/handler.ts:530-532)
```typescript
  protected getPath(event: ALBProxyEvent): string {
    return event.path
  }
```
- ALBProcessor.getPath (src/adapter/aws-lambda/handler.ts:564-566)
```typescript
  protected getPath(event: ALBProxyEvent): string {
    return event.path
  }
```
- EventV1Processor.getPath (adapters/aws-lambda/src/handler.ts:448-450)
```typescript
  protected getPath(event: APIGatewayProxyEvent): string {
    return event.path
  }
```
- LatticeV2Processor.getPath (adapters/aws-lambda/src/handler.ts:589-591)
```typescript
  protected getPath(event: LatticeProxyEventV2): string {
    return event.path
  }
```
- EventV1Processor.getPath (src/adapter/aws-lambda/handler.ts:479-481)
```typescript
  protected getPath(event: APIGatewayProxyEvent): string {
    return event.path
  }
```
- LatticeV2Processor.getPath (src/adapter/aws-lambda/handler.ts:626-628)
```typescript
  protected getPath(event: LatticeProxyEventV2): string {
    return event.path
  }
```
- EventV2Processor.getPath (adapters/aws-lambda/src/handler.ts:409-411)
```typescript
  protected getPath(event: APIGatewayProxyEventV2): string {
    return event.rawPath
  }
```
- EventV2Processor.getPath (src/adapter/aws-lambda/handler.ts:437-439)
```typescript
  protected getPath(event: APIGatewayProxyEventV2): string {
    return event.rawPath
  }
```
- fire (adapters/service-worker/src/index.ts:28-34)
```typescript
const fire = <E extends Env, S extends Schema, BasePath extends string>(
  app: Hono<E, S, BasePath>,
  options?: HandleOptions
): void => {
  // @ts-expect-error addEventListener is not typed well in ServiceWorker-like contexts, see: https://github.com/microsoft/TypeScript/issues/14877
  addEventListener('fetch', handle(app, options))
}
```
- fire (src/adapter/service-worker/index.ts:32-38)
```typescript
const fire = <E extends Env, S extends Schema, BasePath extends string>(
  app: Hono<E, S, BasePath>,
  options?: HandleOptions
): void => {
  // @ts-expect-error addEventListener is not typed well in ServiceWorker-like contexts, see: https://github.com/microsoft/TypeScript/issues/14877
  addEventListener('fetch', handle(app, options))
}
```
- Client (src/client/types.ts:311-318)
```typescript
export type Client<T, Prefix extends string> =
  T extends HonoBase<any, infer S, any>
    ? S extends Record<infer K, Schema>
      ? K extends string
        ? PathToChain<Prefix, K, S>
        : never
      : never
    : never
```
- methodNotAllowed (src/middleware/method-not-allowed/index.ts:59-141)
```typescript
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
- mergePath (src/client/utils.ts:11-16)
```typescript
export const mergePath = (base: string, path: string) => {
  base = base.replace(/\/+$/, '')
  base = base + '/'
  path = path.replace(/^\/+/, '')
  return base + path
}
```
- handle (src/adapter/cloudflare-pages/handler.ts:38-52)
```typescript
export const handle =
  <E extends Env = Env, S extends Schema = BlankSchema, BasePath extends string = '/'>(
    app: Hono<E, S, BasePath>
  ): PagesFunction<E['Bindings']> =>
  (eventContext) => {
    return app.fetch(
      eventContext.request,
      { ...eventContext.env, eventContext },
      {
        waitUntil: eventContext.waitUntil,
        passThroughOnException: eventContext.passThroughOnException,
        props: {},
      }
    )
  }
```
- CookieOptions (src/utils/cookie.ts:17-28)
```typescript
export type CookieOptions = {
  domain?: string
  expires?: Date
  httpOnly?: boolean
  maxAge?: number
  path?: string
  secure?: boolean
  sameSite?: 'Strict' | 'Lax' | 'None' | 'strict' | 'lax' | 'none'
  partitioned?: boolean
  priority?: 'Low' | 'Medium' | 'High' | 'low' | 'medium' | 'high'
  prefix?: CookiePrefixOptions
} & PartitionedCookieConstraint
```
- basePath (src/helper/route/index.ts:107-144)
```typescript
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
        const restPath = reqPath.substring(basePathLength + 1)
        // like the router, fall back to matching only the current segment
        const segment = restPath.split('/', 1)[0]
        basePathLength += (restPath.match(re)?.[0] ?? (re.test(segment) ? segment : '')).length
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
- checkOptionalParameter (src/utils/url.ts:184-219)
```typescript
export const checkOptionalParameter = (path: string): string[] | null => {
  /*
    If path is `/api/animals/:type?` it will return:
    [`/api/animals`, `/api/animals/:type`]
    in other cases it will return null
  */

  if (path.charCodeAt(path.length - 1) !== 63 || !path.includes(':')) {
    return null
  }

  const segments = path.split('/')
  const results: string[] = []
  let basePath = ''

  segments.forEach((segment) => {
    if (segment !== '' && !/\:/.test(segment)) {
      basePath += '/' + segment
    } else if (/\:/.test(segment)) {
      if (segment.charCodeAt(segment.length - 1) === 63) {
        if (results.length === 0 && basePath === '') {
          results.push('/')
        } else {
          results.push(basePath)
        }
        const optionalSegment = segment.slice(0, -1)
        basePath += '/' + optionalSegment
        results.push(basePath)
      } else {
        basePath += '/' + segment
      }
    }
  })

  return results.filter((v, i, a) => a.indexOf(v) === i)
}
```
- baseRoutePath (src/helper/route/index.ts:82-83)
```typescript
export const baseRoutePath = (c: Context, index?: number): string =>
  matchedRoutes(c).at(index ?? c.req.routeIndex)?.basePath ?? ''
```
- mergePath (src/utils/url.ts:171-182)
```typescript
export const mergePath: (...paths: string[]) => string = (
  base: string | undefined,
  sub: string | undefined,
  ...rest: string[]
): string => {
  if (rest.length) {
    sub = mergePath(sub as string, ...rest)
  }
  return `${base?.[0] === '/' ? '' : '/'}${base}${
    sub === '/' ? '' : `${base?.at(-1) === '/' ? '' : '/'}${sub?.[0] === '/' ? sub.slice(1) : sub}`
  }`
}
```
- handle (adapters/service-worker/src/handler.ts:18-37)
```typescript
export const handle = <E extends Env, S extends Schema, BasePath extends string>(
  app: Hono<E, S, BasePath>,
  opts: HandleOptions = {
    // To use `fetch` on a Service Worker correctly, bind it to `globalThis`.
    fetch: globalThis.fetch.bind(globalThis),
  }
): Handler => {
  return (evt) => {
    evt.respondWith(
      (async () => {
        // @ts-expect-error Passing FetchEvent but app.fetch expects ExecutionContext
        const res = await app.fetch(evt.request, {}, evt)
        if (opts.fetch && res.status === 404) {
          return await opts.fetch(evt.request)
        }
        return res
      })()
    )
  }
}
```
- handle (src/adapter/service-worker/handler.ts:22-41)
```typescript
export const handle = <E extends Env, S extends Schema, BasePath extends string>(
  app: Hono<E, S, BasePath>,
  opts: HandleOptions = {
    // To use `fetch` on a Service Worker correctly, bind it to `globalThis`.
    fetch: globalThis.fetch.bind(globalThis),
  }
): Handler => {
  return (evt) => {
    evt.respondWith(
      (async () => {
        // @ts-expect-error Passing FetchEvent but app.fetch expects ExecutionContext
        const res = await app.fetch(evt.request, {}, evt)
        if (opts.fetch && res.status === 404) {
          return await opts.fetch(evt.request)
        }
        return res
      })()
    )
  }
}
```
- MountOptionHandler (src/hono-base.ts:89-89)
```typescript
type MountOptionHandler = (c: Context) => unknown
```
- MountReplaceRequest (src/hono-base.ts:90-90)
```typescript
type MountReplaceRequest = (originalRequest: Request) => Request
```
- DOMAttributes (src/jsx/base.ts:36-36)
```typescript
export type DOMAttributes = HonoJSX.HTMLAttributes
```
- getPath (src/utils/url.ts:106-134)
```typescript
export const getPath = (request: Request): string => {
  const url = request.url
  const start = url.indexOf('/', url.indexOf(':') + 4)
