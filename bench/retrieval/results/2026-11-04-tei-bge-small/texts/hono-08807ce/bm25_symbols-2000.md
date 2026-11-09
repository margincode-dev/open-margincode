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
