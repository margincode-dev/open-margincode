## Task
# hono-d54720d · hono-d54720d-2 · open
Goal: fix(serve-static): respect q-values in Accept-Encoding for precompressed files (#5436)

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:52913 ServeStaticOptions (src/middleware/serve-static/index.ts:13-23)
```typescript
export type ServeStaticOptions<E extends Env = Env> = {
  root?: string
  path?: string
  precompressed?: boolean
  /** Allow percent signs in the routed request path. Defaults to false. */
  allowPercentInPath?: boolean
  mimes?: Record<string, string>
  rewriteRequestPath?: (path: string) => string
  onFound?: (path: string, c: Context<E>) => void | Promise<void>
  onNotFound?: (path: string, c: Context<E>) => void | Promise<void>
}
```
- sym:53126 AcceptHeader (src/utils/headers.ts:345-352)
```typescript
export type AcceptHeader =
  | 'Accept'
  | 'Accept-Charset'
  | 'Accept-Encoding'
  | 'Accept-Language'
  | 'Accept-Patch'
  | 'Accept-Post'
  | 'Accept-Ranges'
```
- sym:52009 ServeStaticOptions (adapters/cloudflare-workers/src/serve-static.ts:6-10)
```typescript
export type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & {
  // namespace is KVNamespace
  namespace?: unknown
  manifest?: object | string
}
```
- sym:52181 ServeStaticOptions (src/adapter/cloudflare-workers/serve-static.ts:9-13)
  type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & { // namespace is KVNamespace namespace?: unknown manifest?: object | string }
- sym:52008 module (adapters/cloudflare-workers/src/serve-static-module.ts:6-10)
  const module = <E extends Env = Env>( options: Omit<ServeStaticOptions<E>, 'namespace'> ): MiddlewareHandler =>
- sym:52180 module (src/adapter/cloudflare-workers/serve-static-module.ts:6-10)
  const module = <E extends Env = Env>( options: Omit<ServeStaticOptions<E>, 'namespace'> ): MiddlewareHandler =>
- sym:52010 serveStatic (adapters/cloudflare-workers/src/serve-static.ts:21-42)
- sym:52182 serveStatic (src/adapter/cloudflare-workers/serve-static.ts:24-45)
- sym:51916 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
- sym:51998 serveStatic (adapters/bun/src/serve-static.ts:8-32)
- sym:52014 serveStatic (adapters/deno/src/serve-static.ts:8-42)
- sym:52018 Env (adapters/lambda-edge/src/conninfo.ts:5-9)

---
tokens: 677/800
