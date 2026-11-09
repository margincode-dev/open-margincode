## Task
# hono-08807ce · hono-08807ce · open
Goal: fix(trie-router): keep first value for duplicate path params (#5422) (#5423)

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:50234 HandlerParamsSet (src/router/trie-router/node.ts:13-15)
```typescript
type HandlerParamsSet<T> = HandlerSet<T> & {
  params: Record<string, string>
}
```
- sym:50237 Node.#pushHandlerSets (src/router/trie-router/node.ts:64-84)
  #pushHandlerSets( handlerSets: HandlerParamsSet<T>[], node: Node<T>, method: string, nodeParams: Record<string, string>, params?: Record<string, string> ): void
- sym:50240 TrieRouter.add (src/router/trie-router/router.ts:9-13)
```typescript
  add(method: string, path: string, handler: T) {
    for (const result of checkOptionalParameter(path) || [path]) {
      this.#node.insert(method, result, handler)
    }
  }
```
- sym:50241 TrieRouter.match (src/router/trie-router/router.ts:15-17)
```typescript
  match(method: string, path: string): Result<T> {
    return this.#node.search(method, path)
  }
```
- sym:50238 Node.search (src/router/trie-router/node.ts:86-216)
  search(method: string, path: string): [[T, Params][]]
- sym:50235 Node (src/router/trie-router/node.ts:20-217)
  class Node<T>
- sym:50213 buildInitParams (src/router/reg-exp-router/prepared-router.ts:95-154)
  const buildInitParams: (params: { paths: string[] }) => ConstructorParameters<typeof PreparedRegExpRouter> = ({ paths }) =>
- sym:50226 Trie.insert (src/router/reg-exp-router/trie.ts:14-57)
  insert(path: string, isStatic: boolean): void
- sym:50086 defaultReplaceRequest (src/middleware/mount/index.ts:21-27)
  const defaultReplaceRequest = (c: Context): Request =>
- sym:50239 TrieRouter (src/router/trie-router/router.ts:5-18)
  class TrieRouter<T> implements Router<T>
- sym:50250 Params (src/router.ts:65-65)
  type Params = Record<string, string>
- sym:49655 saveContentToFile (src/helper/ssg/ssg.ts:310-335)
- sym:49711 Hono.#addRoute (src/hono-base.ts:390-401)
- sym:49802 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
- sym:49810 registerAction (src/jsx/dom/hooks/index.ts:32-35)
  const registerAction = (action: Promise<unknown>) =>
- sym:49436 pathname (src/client/client.test.ts:1239-1240)

---
tokens: 684/800
