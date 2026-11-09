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
