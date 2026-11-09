- HandlerParamsSet (src/router/trie-router/node.ts:13-15)
```typescript
type HandlerParamsSet<T> = HandlerSet<T> & {
  params: Record<string, string>
}
```
- TrieRouter (src/router/trie-router/router.ts:5-18)
```typescript
export class TrieRouter<T> implements Router<T> {
  name: string = 'TrieRouter'
  #node: Node<T> = new Node()

  add(method: string, path: string, handler: T) {
    for (const result of checkOptionalParameter(path) || [path]) {
      this.#node.insert(method, result, handler)
    }
  }

  match(method: string, path: string): Result<T> {
    return this.#node.search(method, path)
  }
}
```
- buildInitParams (src/router/reg-exp-router/prepared-router.ts:95-154)
```typescript
export const buildInitParams: (params: {
  paths: string[]
}) => ConstructorParameters<typeof PreparedRegExpRouter> = ({ paths }) => {
  const RegExpRouterWithMatcherExport = class<T> extends RegExpRouter<T> {
    buildAndExportAllMatchers() {
      return this.buildAllMatchers()
    }
  }
  const router = new RegExpRouterWithMatcherExport<string>()
  for (const path of paths) {
    router.add(METHOD_NAME_ALL, path, path)
  }

  const matchers = router.buildAndExportAllMatchers()
  const all = matchers[METHOD_NAME_ALL] as Matcher<string>

  const relocateMap: RelocateMap = {}
  for (const path of paths) {
    if (path === '/*' || path === '*') {
      continue
    }
    all[1].forEach((list, i) => {
      list.forEach(([p, map]) => {
        if (p === path) {
          if (relocateMap[path]) {
            relocateMap[path][0][1] = {
              ...relocateMap[path][0][1],
              ...map,
            }
          } else {
            relocateMap[path] = [[[], map]]
          }
          if (relocateMap[path][0][0].findIndex((j) => j === i) === -1) {
            relocateMap[path][0][0].push(i)
          }
        }
      })
    })
    for (const path2 in all[2]) {
      all[2][path2][0].forEach(([p]) => {
        if (p === path) {
          relocateMap[path] ||= [[[]]]
          const value = path2 === path ? '' : path2
          if (relocateMap[path][0][0].findIndex((v) => v === value) === -1) {
            relocateMap[path][0][0].push(value)
          }
        }
      })
    }
  }

  for (let i = 0, len = all[1].length; i < len; i++) {
    all[1][i] = all[1][i] ? [] : (0 as unknown as HandlerData<string>)
  }
  for (const path in all[2]) {
    all[2][path][0] = []
  }

  return [matchers, relocateMap]
}
```
- RegExpRouter.#buildMatcher (src/router/reg-exp-router/router.ts:145-173)
```typescript
  #buildMatcher(method: string): Matcher<T> {
    const middleware = this.#middleware![method]
    const routes = this.#routes![method]

    const trie = this.#tries![method]
    const staticMap: StaticMap<T> = createNullObject()
    const handlerData: HandlerData<T>[] = []
    const [regexp, indexReplacementMap, paramReplacementMap] = trie.buildRegExp()

    for (const r of [middleware, routes]) {
      for (const path in r) {
        const handlers = r[path]
        const pathData = trie.paths[path]
        if (!pathData) {
          staticMap[path] = [handlers.map(([h]) => [h, createNullObject()]), emptyParam]
          continue
        }
        handlerData[pathData[0]] = handlers.map(([h, handlerPath]) => [
          h,
          trie.paths[handlerPath][1].reduceRight((map, [key], i) => {
            map[key] = paramReplacementMap[pathData[1][i][1]]
            return map
          }, createNullObject()),
        ])
      }
    }

    return [regexp, indexReplacementMap.map((i) => handlerData[i]), staticMap] as Matcher<T>
  }
```
- Node.#pushHandlerSets (src/router/trie-router/node.ts:64-84)
```typescript
  #pushHandlerSets(
    handlerSets: HandlerParamsSet<T>[],
    node: Node<T>,
    method: string,
    nodeParams: Record<string, string>,
    params?: Record<string, string>
  ): void {
    for (let i = 0, len = node.#methods.length; i < len; i++) {
      const m = node.#methods[i]
      const handlerSet = (m[method] || m[METHOD_NAME_ALL]) as HandlerParamsSet<T>
      if (handlerSet) {
        handlerSet.params = createNullObject()
        handlerSets.push(handlerSet)
        for (let i = 0, len = handlerSet.possibleKeys.length; i < len; i++) {
          const key = handlerSet.possibleKeys[i]
          handlerSet.params[key] =
            params?.[key] && !i ? params[key] : (nodeParams[key] ?? params?.[key])
        }
      }
    }
  }
```
- LinearRouter (src/router/linear-router/router.ts:11-152)
```typescript
export class LinearRouter<T> implements Router<T> {
  name: string = 'LinearRouter'
  #routes: [string, string, T][] = []

  add(method: string, path: string, handler: T) {
    for (
      let i = 0, paths = checkOptionalParameter(path) || [path], len = paths.length;
      i < len;
      i++
    ) {
      this.#routes.push([method, paths[i], handler])
    }
  }

  match(method: string, path: string): Result<T> {
    const handlers: [T, Params][] = []
    ROUTES_LOOP: for (let i = 0, len = this.#routes.length; i < len; i++) {
      const [routeMethod, routePath, handler] = this.#routes[i]
      if (routeMethod === method || routeMethod === METHOD_NAME_ALL) {
        if (routePath === '*' || routePath === '/*') {
          handlers.push([handler, emptyParams])
          continue
        }

        const hasStar = routePath.indexOf('*') !== -1
        const hasLabel = routePath.indexOf(':') !== -1
        if (!hasStar && !hasLabel) {
          if (routePath === path || routePath + '/' === path) {
            handlers.push([handler, emptyParams])
          }
        } else if (hasStar && !hasLabel) {
          const endsWithStar = routePath.charCodeAt(routePath.length - 1) === 42
          const endsWithSlashStar = routePath.endsWith('/*')
          const parts = (
            endsWithStar ? routePath.slice(0, endsWithSlashStar ? -2 : -1) : routePath
          ).split(splitByStarRe)

          const lastIndex = parts.length - 1
          for (let j = 0, pos = 0, len = parts.length; j < len; j++) {
            const part = parts[j]
            const index = path.indexOf(part, pos)
            if (index !== pos) {
              continue ROUTES_LOOP
            }
            pos += part.length
            if (j === lastIndex) {
              if (endsWithSlashStar) {
                if (pos !== path.length && path.charCodeAt(pos) !== 47) {
                  continue ROUTES_LOOP
                }
              } else if (
                !endsWithStar &&
                pos !== path.length &&
                !(pos === path.length - 1 && path.charCodeAt(pos) === 47)
              ) {
                continue ROUTES_LOOP
              }
            } else {
              const index = path.indexOf('/', pos)
              if (index === -1) {
                continue ROUTES_LOOP
              }
              pos = index
            }
          }
          handlers.push([handler, emptyParams])
        } else if (hasLabel && !hasStar) {
          const params: Record<string, string> = Object.create(null)
          const parts = routePath.match(splitPathRe) as string[]

          const lastIndex = parts.length - 1
          for (let j = 0, pos = 0, len = parts.length; j < len; j++) {
            if (pos === -1 || pos >= path.length) {
              continue ROUTES_LOOP
            }

            const part = parts[j]
            if (part.charCodeAt(1) === 58) {
              if (path.charCodeAt(pos) !== 47) {
                continue ROUTES_LOOP
              }

              // /:label
              let name = part.slice(2)
              let value

              if (name.charCodeAt(name.length - 1) === 125) {
                // :label{pattern}
                const openBracePos = name.indexOf('{')
                const next = parts[j + 1]
                const lookahead = next && next[1] !== ':' && next[1] !== '*' ? `(?=${next})` : ''
                const pattern = name.slice(openBracePos + 1, -1) + lookahead
                const restPath = path.slice(pos + 1)
                const match = new RegExp(pattern, 'd').exec(restPath) as RegExpMatchArrayWithIndices
                if (!match || match.indices[0][0] !== 0 || match.indices[0][1] === 0) {
                  continue ROUTES_LOOP
                }
                name = name.slice(0, openBracePos)
                value = restPath.slice(...match.indices[0])
                pos += match.indices[0][1] + 1
              } else {
                let endValuePos = path.indexOf('/', pos + 1)
                if (endValuePos === -1) {
                  endValuePos = path.length
                }
                if (endValuePos === pos + 1) {
                  // an empty segment, as in `/u/` or `/u//posts`
                  continue ROUTES_LOOP
                }
                value = path.slice(pos + 1, endValuePos)
                pos = endValuePos
              }

              params[name] ||= value as string
            } else {
              const index = path.indexOf(part, pos)
              if (index !== pos) {
                continue ROUTES_LOOP
              }
              pos += part.length
            }

            if (j === lastIndex) {
              if (
                pos !== path.length &&
                !(pos === path.length - 1 && path.charCodeAt(pos) === 47)
              ) {
                continue ROUTES_LOOP
              }
            }
          }

          handlers.push([handler, params])
        } else if (hasLabel && hasStar) {
          throw new UnsupportedPathError()
        }
      }
    }

    return [handlers]
  }
}
```
- serializeInitParams (src/router/reg-exp-router/prepared-router.ts:156-165)
```typescript
export const serializeInitParams: (
  params: ConstructorParameters<typeof PreparedRegExpRouter>
) => string = ([matchers, relocateMap]) => {
  // Embed the regular expression as a result of `toString()` so that it can be evaluated as JavaScript.
  const matchersStr = JSON.stringify(matchers, (_, value) =>
    value instanceof RegExp ? `##${value.toString()}##` : value
  ).replace(/"##(.+?)##"/g, (_, str) => str.replace(/\\\\/g, '\\'))
  const relocateMapStr = JSON.stringify(relocateMap)
  return `[${matchersStr},${relocateMapStr}]`
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
- except (src/middleware/combine/index.ts:147-171)
```typescript
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
- RegExpRouter (src/router/reg-exp-router/router.ts:48-174)
```typescript
export class RegExpRouter<T> implements Router<T> {
  name: string = 'RegExpRouter'
  #middleware?: Record<string, Record<string, HandlerWithMetadata<T>[]>>
  #routes?: Record<string, Record<string, HandlerWithMetadata<T>[]>>
  #tries?: Record<string, Trie>

  constructor() {
    this.#middleware = { [METHOD_NAME_ALL]: createNullObject() }
    this.#routes = { [METHOD_NAME_ALL]: createNullObject() }
    this.#tries = { [METHOD_NAME_ALL]: new Trie() }
  }

  #insertPath(method: string, path: string) {
    try {
      this.#tries![method].insert(path, !/\*|\/:/.test(path))
    } catch (e) {
      throw e === PATH_ERROR ? new UnsupportedPathError(path) : e
    }
  }

  add(method: string, path: string, handler: T) {
    const middleware = this.#middleware
    const routes = this.#routes!

    if (!middleware) {
      throw new Error(MESSAGE_MATCHER_IS_ALREADY_BUILT)
    }

    if (!middleware[method]) {
      this.#tries![method] = new Trie()
      for (const handlerMap of [middleware, routes]) {
        handlerMap[method] = createNullObject()
        for (const p in handlerMap[METHOD_NAME_ALL]) {
          handlerMap[method][p] = [...handlerMap[METHOD_NAME_ALL][p]]
          this.#insertPath(method, p)
        }
      }
    }

    if (path === '/*') {
      path = '*'
    }
    const methods = method === METHOD_NAME_ALL ? Object.keys(middleware) : [method]

    if (/\*$/.test(path)) {
      const re = buildWildcardRegExp(path)
      for (const m of methods) {
        if (!middleware[m][path]) {
          this.#insertPath(m, path)
          middleware[m][path] =
            findMiddleware(middleware[m], path) ||
            findMiddleware(middleware[METHOD_NAME_ALL], path) ||
            []
        }
      }
      for (const handlerMap of [middleware, routes]) {
        for (const m of methods) {
          for (const p in handlerMap[m]) {
            re.test(p) && handlerMap[m][p].push([handler, path])
          }
        }
      }

      return
    }

    const paths = checkOptionalParameter(path) || [path]
    for (const path of paths) {
      for (const m of methods) {
