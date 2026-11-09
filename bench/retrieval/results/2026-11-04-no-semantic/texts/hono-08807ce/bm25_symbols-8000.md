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
        if (!routes[m][path]) {
          this.#insertPath(m, path)
          routes[m][path] =
            findMiddleware(middleware[m], path) ||
            findMiddleware(middleware[METHOD_NAME_ALL], path) ||
            []
        }
        routes[m][path].push([handler, path])
      }
    }
  }

  match: typeof match<Router<T>, T> = match

  protected buildAllMatchers(): MatcherMap<T> {
    const matchers: MatcherMap<T> = createNullObject()

    for (const method of Object.keys(this.#routes!)) {
      matchers[method] = this.#buildMatcher(method)
    }

    // Release cache
    this.#middleware = this.#routes = this.#tries = undefined
    wildcardRegExpCache = createNullObject()

    return matchers
  }

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
}
```
- Node.search (src/router/trie-router/node.ts:86-216)
```typescript
  search(method: string, path: string): [[T, Params][]] {
    const handlerSets: HandlerParamsSet<T>[] = []
    this.#params = emptyParams

    // eslint-disable-next-line @typescript-eslint/no-this-alias
    const curNode: Node<T> = this
    let curNodes = [curNode]
    const parts = splitPath(path)
    const curNodesQueue: Node<T>[][] = []

    const len = parts.length
    let partOffsets: number[] | null = null

    for (let i = 0; i < len; i++) {
      const part: string = parts[i]
      const isLast = i === len - 1
      const tempNodes: Node<T>[] = []

      for (let j = 0, len2 = curNodes.length; j < len2; j++) {
        const node = curNodes[j]
        const nextNode = node.#children[part]

        if (nextNode) {
          nextNode.#params = node.#params
          if (isLast) {
            // '/hello/*' => match '/hello'
            if (nextNode.#children['*']) {
              this.#pushHandlerSets(handlerSets, nextNode.#children['*'], method, node.#params)
            }
            this.#pushHandlerSets(handlerSets, nextNode, method, node.#params)
          } else {
            tempNodes.push(nextNode)
          }
        }

        for (const child of node.#patterns) {
          const pattern = child.#pattern!
          const params = node.#params === emptyParams ? {} : { ...node.#params }

          // Wildcard
          // '/hello/*/foo' => match /hello/bar/foo
          if (typeof pattern === 'string') {
            if (pattern === '*' || part.startsWith(pattern.slice(0, -1))) {
              this.#pushHandlerSets(handlerSets, child, method, node.#params)
              if (pattern === '*') {
                child.#params = params
                tempNodes.push(child)
              }
            }
            continue
          }

          const [, name, matcher] = pattern

          if (!part && matcher === true) {
            continue
          }

          // `/js/:filename{[a-z]+.js}` => match /js/chunk/123.js
          if (matcher !== true) {
            if (!partOffsets) {
              partOffsets = []
              let offset = path[0] === '/' ? 1 : 0
              for (let p = 0; p < len; p++) {
                partOffsets[p] = offset
                offset += parts[p].length + 1
              }
            }
            const restPathString = path.slice(partOffsets[i])

            const m = matcher.exec(restPathString)
            if (m) {
              params[name] = m[0]
              this.#pushHandlerSets(handlerSets, child, method, node.#params, params)

              // '/:id{[0-9]+}/*' => match '/123'
              if (m[0].length === restPathString.length && child.#children['*']) {
                this.#pushHandlerSets(
                  handlerSets,
                  child.#children['*'],
                  method,
                  node.#params,
                  params
                )
              }

              for (const _ in child.#children) {
                child.#params = params
                const componentCount = m[0].match(/\//g)?.length ?? 0
                const targetCurNodes = (curNodesQueue[componentCount] ||= [])
                targetCurNodes.push(child)
                break
              }

              continue
            }
          }

          if (matcher === true || matcher.test(part)) {
            params[name] = part
            if (isLast) {
              this.#pushHandlerSets(handlerSets, child, method, params, node.#params)
              if (child.#children['*']) {
                this.#pushHandlerSets(
                  handlerSets,
                  child.#children['*'],
                  method,
                  params,
                  node.#params
                )
              }
            } else {
              child.#params = params
              tempNodes.push(child)
            }
          }
        }
      }

      const shifted = curNodesQueue.shift()
      curNodes = shifted ? tempNodes.concat(shifted) : tempNodes
    }

    if (handlerSets[1]) {
      handlerSets.sort((a, b) => {
        return a.score - b.score
      })
    }

    return [handlerSets.map(({ handler, params }) => [handler, params] as [T, Params])]
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
- runTest (src/router/common.case.test.ts:14-1118)
```typescript
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
