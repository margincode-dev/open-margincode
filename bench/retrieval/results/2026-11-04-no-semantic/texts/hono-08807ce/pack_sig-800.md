## Task
# hono-08807ce · hono-08807ce-2 · open
Goal: fix(trie-router): keep first value for duplicate path params (#5422) (#5423)
Workspace: /root/bench/ws/rebf815-hono-08807ce/hono-08807ce-2
Changed (0): nothing yet

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:50238 Node.search (src/router/trie-router/node.ts:86-216)
  search(method: string, path: string): [[T, Params][]]
- sym:50234 HandlerParamsSet (src/router/trie-router/node.ts:13-15)
  type HandlerParamsSet<T> = HandlerSet<T> & { params: Record<string, string> }
- sym:50235 Node (src/router/trie-router/node.ts:20-217)
  class Node<T>
- sym:50237 Node.#pushHandlerSets (src/router/trie-router/node.ts:64-84)
  #pushHandlerSets( handlerSets: HandlerParamsSet<T>[], node: Node<T>, method: string, nodeParams: Record<string, string>, params?: Record<string, string> ): void
- sym:50213 buildInitParams (src/router/reg-exp-router/prepared-router.ts:95-154)
  const buildInitParams: (params: { paths: string[] }) => ConstructorParameters<typeof PreparedRegExpRouter> = ({ paths }) =>
- sym:50239 TrieRouter (src/router/trie-router/router.ts:5-18)
  class TrieRouter<T> implements Router<T>
- sym:50241 TrieRouter.match (src/router/trie-router/router.ts:15-17)
  match(method: string, path: string): Result<T>
- sym:50240 TrieRouter.add (src/router/trie-router/router.ts:9-13)
  add(method: string, path: string, handler: T)
- sym:49386 Params (src/adapter/cloudflare-pages/handler.ts:9-9)
  type Params<P extends string = any> = Record<P, string | string[]>
- sym:49466 TypedURL (src/client/types.ts:215-230)
  interface TypedURL< Protocol extends string, Hostname extends string, Port extends string, Pathname extends string, Search extends string, > extends URL
- sym:49840 Node (src/jsx/dom/render.ts:80-80)
  type Node = NodeString | NodeObject
- sym:49981 MockCache.match (src/middleware/cache/index.test.ts:18-20)
  async match(key: Request | string): Promise<Response | null>

---
tokens: 598/800
