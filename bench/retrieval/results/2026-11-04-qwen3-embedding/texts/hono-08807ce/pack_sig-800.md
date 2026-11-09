## Task
# hono-08807ce · hono-08807ce · open
Goal: fix(trie-router): keep first value for duplicate path params (#5422) (#5423)
Workspace: /root/bench/ws/rebf815-hono-08807ce/hono-08807ce
Changed (0): nothing yet

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:50239 TrieRouter (src/router/trie-router/router.ts:5-18)
  class TrieRouter<T> implements Router<T>
- sym:50235 Node (src/router/trie-router/node.ts:20-217)
  class Node<T>
- sym:50240 TrieRouter.add (src/router/trie-router/router.ts:9-13)
  add(method: string, path: string, handler: T)
- sym:50241 TrieRouter.match (src/router/trie-router/router.ts:15-17)
  match(method: string, path: string): Result<T>
- sym:50225 Trie (src/router/reg-exp-router/trie.ts:7-84)
  class Trie
- sym:50238 Node.search (src/router/trie-router/node.ts:86-216)
  search(method: string, path: string): [[T, Params][]]
- sym:50234 HandlerParamsSet (src/router/trie-router/node.ts:13-15)
  type HandlerParamsSet<T> = HandlerSet<T> & { params: Record<string, string> }
- sym:50226 Trie.insert (src/router/reg-exp-router/trie.ts:14-57)
  insert(path: string, isStatic: boolean): void
- sym:50237 Node.#pushHandlerSets (src/router/trie-router/node.ts:64-84)
  #pushHandlerSets( handlerSets: HandlerParamsSet<T>[], node: Node<T>, method: string, nodeParams: Record<string, string>, params?: Record<string, string> ): void
- sym:50213 buildInitParams (src/router/reg-exp-router/prepared-router.ts:95-154)
  const buildInitParams: (params: { paths: string[] }) => ConstructorParameters<typeof PreparedRegExpRouter> = ({ paths }) =>
- sym:50218 RegExpRouter (src/router/reg-exp-router/router.ts:48-174)
  class RegExpRouter<T> implements Router<T>
- sym:50203 Node (src/router/reg-exp-router/node.ts:47-170)
  class Node
- sym:49273 runBenchmark (benchmarks/http-server/benchmark.ts:180-232)
  const runBenchmark = async (appPath: string, name: string) =>
- sym:49298 main (perf-measures/type-check/scripts/process-results.ts:3-49)
  async function main()
- sym:49538 acceptsOptions (src/helper/accepts/accepts.ts:17-19)
  interface acceptsOptions extends acceptsConfig
- sym:49542 accepts (src/helper/accepts/accepts.ts:87-96)
  const accepts = (c: Context, options: acceptsOptions): string =>

---
tokens: 687/800
