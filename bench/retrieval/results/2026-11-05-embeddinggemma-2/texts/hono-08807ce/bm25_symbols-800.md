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

