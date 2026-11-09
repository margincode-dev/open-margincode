- Hono.#addRoute (src/hono-base.ts:390-401)
```typescript
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
```
- PatternRouter.match (src/router/pattern-router/router.ts:38-53)
```typescript
  match(method: string, path: string): Result<T> {
    const handlers: [T, Params][] = []

    for (let i = 0, len = this.#routes.length; i < len; i++) {
      const [pattern, routeMethod, handler] = this.#routes[i]

      if (routeMethod === method || routeMethod === METHOD_NAME_ALL) {
        const match = pattern.exec(path)
        if (match) {
          handlers.push([handler, match.groups || emptyParams])
        }
      }
    }

    return [handlers]
  }
```
- baseRoutePath (src/helper/route/index.ts:82-83)
```typescript
export const baseRoutePath = (c: Context, index?: number): string =>
  matchedRoutes(c).at(index ?? c.req.routeIndex)?.basePath ?? ''
```
- PatternRouter (src/router/pattern-router/router.ts:8-54)
```typescript
export class PatternRouter<T> implements Router<T> {
  name: string = 'PatternRouter'
  #routes: Route<T>[] = []

  add(method: string, path: string, handler: T) {
    const suffix = path.endsWith('/*') ? '(?:$|/)' : path.endsWith('*') ? '' : '/?$'
    path = path.replace(/\*$/, '')
    if (path.at(-1) === '?') {
      path = path.slice(0, -1)
      this.add(method, path.replace(/\/[^/]+$/, ''), handler)
    }

    const parts = (path.match(/\/?(:\w+(?:{(?:(?:{[\d,]+})|[^}])+})?)|\/?[^\/\?]+/g) || []).map(
      (part) => {
        const match = part.match(/^\/:([^{]+)(?:{(.*)})?/)
        return match
          ? `/(?<${match[1]}>${match[2] || '[^/]+'})`
          : part === '/*'
            ? '/[^/]+'
            : part.replace(/[.\\+*[^\]$()]/g, '\\$&')
      }
    )

    try {
      this.#routes.push([new RegExp(`^${parts.join('')}${suffix}`), method, handler])
    } catch {
      throw new UnsupportedPathError()
    }
  }

  match(method: string, path: string): Result<T> {
    const handlers: [T, Params][] = []

    for (let i = 0, len = this.#routes.length; i < len; i++) {
      const [pattern, routeMethod, handler] = this.#routes[i]

      if (routeMethod === method || routeMethod === METHOD_NAME_ALL) {
        const match = pattern.exec(path)
        if (match) {
          handlers.push([handler, match.groups || emptyParams])
        }
      }
    }

    return [handlers]
  }
}
```
- basePath (src/helper/route/index.ts:107-141)
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

