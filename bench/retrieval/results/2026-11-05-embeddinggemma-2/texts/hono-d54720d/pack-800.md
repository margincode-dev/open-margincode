## Task
# hono-d54720d · hono-d54720d · open
Goal: fix(serve-static): respect q-values in Accept-Encoding for precompressed files (#5436)

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:52913 ServeStaticOptions (src/middleware/serve-static/index.ts:13-23)
- sym:53050 parseQuality (src/utils/accept.ts:242-268)
  const parseQuality = (qVal?: string): number =>
- sym:52793 selectEncoding (src/middleware/compress/index.ts:24-44)
  const selectEncoding = ( header: string | undefined, candidates: readonly Encoding[] ): Encoding | undefined =>
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
  type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & { // namespace is KVNamespace namespace?: unknown manifest?: object | string }
- sym:52329 defaultMatch (src/helper/accepts/accepts.ts:52-72)
  const defaultMatch = (accepts: Accept[], config: acceptsConfig): string =>
- sym:52181 ServeStaticOptions (src/adapter/cloudflare-workers/serve-static.ts:9-13)
- sym:52324 Accept (src/helper/accepts/accepts.ts:5-9)
  interface Accept
- sym:52008 module (adapters/cloudflare-workers/src/serve-static-module.ts:6-10)
- sym:53041 Accept (src/utils/accept.ts:1-5)
  interface Accept
- sym:52180 module (src/adapter/cloudflare-workers/serve-static-module.ts:6-10)
- sym:52790 Encoding (src/middleware/compress/index.ts:13-13)
- sym:52010 serveStatic (adapters/cloudflare-workers/src/serve-static.ts:21-42)
- sym:52794 compress (src/middleware/compress/index.ts:72-136)
- sym:52182 serveStatic (src/adapter/cloudflare-workers/serve-static.ts:24-45)
- sym:52914 ENCODINGS (src/middleware/serve-static/index.ts:25-29)
  const ENCODINGS = {
- sym:51916 Env (adapters/aws-lambda/src/conninfo.ts:14-18)
- sym:51998 serveStatic (adapters/bun/src/serve-static.ts:8-32)
- sym:52014 serveStatic (adapters/deno/src/serve-static.ts:8-42)
- sym:52018 Env (adapters/lambda-edge/src/conninfo.ts:5-9)

---
tokens: 679/800
