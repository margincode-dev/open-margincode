- serveStatic (src/middleware/serve-static/index.ts:37-135)
```typescript
export const serveStatic = <E extends Env = Env>(
  options: ServeStaticOptions<E> & {
    getContent: (path: string, c: Context<E>) => Promise<Data | Response | null>
    /**
     *
     * `join` option according to the runtime. Example `import { join } from 'node:path`. If not specified, it will fall back to the default join function.`
     */
    join?: (...paths: string[]) => string
    /**
     * @deprecated Currently, `pathResolve` is no longer used.
     */
    pathResolve?: (path: string) => string
    isDir?: (path: string) => boolean | undefined | Promise<boolean | undefined>
  }
): MiddlewareHandler => {
  const root = options.root ?? './'
  const optionPath = options.path
  const join = options.join ?? defaultJoin

  return async (c, next) => {
    // Do nothing if Response is already set
    if (c.finalized) {
      return next()
    }

    if (c.req.method !== 'GET' && c.req.method !== 'HEAD') {
      return next()
    }

    let filename: string

    if (options.path) {
      filename = options.path
    } else {
      try {
        if (!options.allowPercentInPath && c.req.path.includes('%')) {
          throw new Error()
        }
        filename = tryDecodeURI(c.req.path)
        if (/(?:^|[\/\\])\.{1,2}(?:$|[\/\\])|[\/\\]{2,}|\\/.test(filename)) {
          throw new Error()
        }
      } catch {
        await options.onNotFound?.(c.req.path, c)
        return next()
      }
    }

    let path = join(
      root,
      !optionPath && options.rewriteRequestPath ? options.rewriteRequestPath(filename) : filename
    )

    if (options.isDir && (await options.isDir(path))) {
      path = join(path, DEFAULT_DOCUMENT)
    }

    const getContent = options.getContent
    let content = await getContent(path, c)

    if (content instanceof Response) {
      return c.newResponse(content.body, content)
    }

    if (content != null) {
      const mimeType = (options.mimes && getMimeType(path, options.mimes)) || getMimeType(path)
      c.header('Content-Type', mimeType || 'application/octet-stream')

      if (options.precompressed && (!mimeType || COMPRESSIBLE_CONTENT_TYPE_REGEX.test(mimeType))) {
        const acceptEncodingSet = new Set(
          c.req
            .header('Accept-Encoding')
            ?.split(',')
            .map((encoding) => encoding.trim())
        )

        for (const encoding of ENCODINGS_ORDERED_KEYS) {
          if (!acceptEncodingSet.has(encoding)) {
            continue
          }
          const compressedContent = (await getContent(path + ENCODINGS[encoding], c)) as Data | null

          if (compressedContent) {
            content = compressedContent
            c.header('Content-Encoding', encoding)
            c.header('Vary', 'Accept-Encoding', { append: true })
            break
          }
        }
      }
      await options.onFound?.(path, c)
      return c.body(content)
    }

    await options.onNotFound?.(path, c)
    await next()
    return
  }
}
```
- ServeStaticOptions (src/middleware/serve-static/index.ts:13-23)
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
- ServeStaticOptions (adapters/cloudflare-workers/src/serve-static.ts:6-10)
```typescript
export type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & {
  // namespace is KVNamespace
  namespace?: unknown
  manifest?: object | string
}
```
- ServeStaticOptions (src/adapter/cloudflare-workers/serve-static.ts:9-13)
```typescript
export type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & {
  // namespace is KVNamespace
  namespace?: unknown
  manifest?: object | string
}
```
- serveStatic (adapters/cloudflare-workers/src/serve-static.ts:21-42)
```typescript
export const serveStatic = <E extends Env = Env>(
  options: ServeStaticOptions<E> = {}
): MiddlewareHandler => {
  return async function serveStatic(c, next) {
    const getContent = async (path: string) => {
      return getContentFromKVAsset(path, {
        manifest: options.manifest,
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        namespace: options.namespace
          ? options.namespace
          : c.env
            ? c.env.__STATIC_CONTENT
            : undefined,
      })
    }
    return baseServeStatic({
      ...options,
      getContent,
    })(c, next)
  }
}
```
- serveStatic (src/adapter/cloudflare-workers/serve-static.ts:24-45)
```typescript
export const serveStatic = <E extends Env = Env>(
  options: ServeStaticOptions<E> = {}
): MiddlewareHandler => {
  return async function serveStatic(c, next) {
    const getContent = async (path: string) => {
      return getContentFromKVAsset(path, {
        manifest: options.manifest,
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        namespace: options.namespace
          ? options.namespace
          : c.env
            ? c.env.__STATIC_CONTENT
            : undefined,
      })
    }
    return baseServeStatic({
      ...options,
      getContent,
    })(c, next)
  }
}
```
- module (adapters/cloudflare-workers/src/serve-static-module.ts:6-10)
```typescript
const module = <E extends Env = Env>(
  options: Omit<ServeStaticOptions<E>, 'namespace'>
): MiddlewareHandler => {
  return serveStatic<E>(options)
}
```
- module (src/adapter/cloudflare-workers/serve-static-module.ts:6-10)
```typescript
const module = <E extends Env = Env>(
  options: Omit<ServeStaticOptions<E>, 'namespace'>
): MiddlewareHandler => {
  return serveStatic<E>(options)
}
```
- serveStatic (adapters/bun/src/serve-static.ts:8-32)
```typescript
export const serveStatic = <E extends Env = Env>(
  options: ServeStaticOptions<E> = {}
): MiddlewareHandler => {
  return async function serveStatic(c, next) {
    const getContent = async (path: string) => {
      // @ts-ignore
      const file = Bun.file(path)
      return (await file.exists()) ? file : null
    }
    const isDir = async (path: string) => {
      let isDir
      try {
        const stats = await stat(path)
        isDir = stats.isDirectory()
      } catch {}
      return isDir
    }
    return baseServeStatic({
      ...options,
      getContent,
      join,
      isDir,
    })(c, next)
  }
}
```
- serveStatic (src/adapter/bun/serve-static.ts:11-35)
```typescript
export const serveStatic = <E extends Env = Env>(
  options: ServeStaticOptions<E> = {}
): MiddlewareHandler => {
  return async function serveStatic(c, next) {
    const getContent = async (path: string) => {
      // @ts-ignore
      const file = Bun.file(path)
      return (await file.exists()) ? file : null
    }
    const isDir = async (path: string) => {
      let isDir
      try {
        const stats = await stat(path)
        isDir = stats.isDirectory()
      } catch {}
      return isDir
    }
    return baseServeStatic({
      ...options,
      getContent,
      join,
      isDir,
    })(c, next)
  }
}
```
- serveStatic (adapters/deno/src/serve-static.ts:8-42)
```typescript
export const serveStatic = <E extends Env = Env>(
  options: ServeStaticOptions<E> = {}
): MiddlewareHandler => {
  return async function serveStatic(c, next) {
    const getContent = async (path: string) => {
      try {
        if (isDir(path)) {
          return null
        }

        const file = await open(path)
        return file.readable
      } catch (e) {
        if (!(e instanceof errors.NotFound)) {
          console.warn(`${e}`)
        }
        return null
      }
    }
    const isDir = (path: string) => {
      let isDir
      try {
        const stat = lstatSync(path)
        isDir = stat.isDirectory
      } catch {}
      return isDir
    }
    return baseServeStatic({
      ...options,
      getContent,
      join,
      isDir,
    })(c, next)
  }
}
```
- serveStatic (src/adapter/deno/serve-static.ts:11-45)
```typescript
export const serveStatic = <E extends Env = Env>(
  options: ServeStaticOptions<E> = {}
): MiddlewareHandler => {
  return async function serveStatic(c, next) {
    const getContent = async (path: string) => {
      try {
        if (isDir(path)) {
          return null
        }

        const file = await open(path)
        return file.readable
      } catch (e) {
        if (!(e instanceof errors.NotFound)) {
          console.warn(`${e}`)
        }
        return null
      }
    }
    const isDir = (path: string) => {
      let isDir
      try {
        const stat = lstatSync(path)
        isDir = stat.isDirectory
      } catch {}
      return isDir
    }
    return baseServeStatic({
      ...options,
      getContent,
      join,
      isDir,
    })(c, next)
  }
}
```
- AcceptHeader (src/utils/headers.ts:345-352)
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
- parseAccept (src/utils/accept.ts:211-240)
```typescript
export const parseAccept = (acceptHeader: string): Accept[] => {
  if (!acceptHeader) {
    return []
  }

  const values: Accept[] = []
  let i = 0
  let accept: Accept | undefined
  let requiresSort = false // in many cases, accept values are already sorted by quality (e.g. "text/html, application/json;q=0.9, */*;q=0.8")
  let lastAccept: Accept | undefined
  while (i < acceptHeader.length) {
    ;[i, accept] = getNextAcceptValue(acceptHeader, i)
    if (accept) {
      // The "q" parameter is case-insensitive (RFC 9110 §12.4.2). `params` keeps
      // the name as it was sent, so both spellings are checked here.
      accept.q = parseQuality(accept.params.q ?? accept.params.Q)
      values.push(accept)
      if (lastAccept && lastAccept.q < accept.q) {
        // find higher quality accept value, so we need to sort
        requiresSort = true
      }
      lastAccept = accept
    }
  }
  if (requiresSort) {
    values.sort((a, b) => b.q - a.q)
  }

  return values
}
```
- serveStatic (src/adapter/cloudflare-pages/handler.ts:124-133)
```typescript
export const serveStatic = (): MiddlewareHandler => {
  return async (c) => {
    const env = c.env as { ASSETS: FetcherLike }
    const res = await env.ASSETS.fetch(c.req.raw)
    if (res.status === 404) {
      return c.notFound()
    }
    return res
  }
}
```
- compress (src/middleware/compress/index.ts:72-136)
```typescript
export const compress = (options?: CompressionOptions): MiddlewareHandler => {
  const threshold = options?.threshold ?? 1024
  const candidates: readonly Encoding[] = options?.encoding ? [options.encoding] : ENCODING_TYPES

  const contentTypeFilter = options?.contentTypeFilter ?? COMPRESSIBLE_CONTENT_TYPE_REGEX
  const shouldCompress =
    typeof contentTypeFilter === 'function'
      ? (res: Response) => {
          const type = res.headers.get('Content-Type')
          return type && contentTypeFilter(type)
        }
      : (res: Response) => {
          const type = res.headers.get('Content-Type')
          return type && contentTypeFilter.test(type)
        }

  return async function compress(ctx, next) {
    await next()

    const contentLength = ctx.res.headers.get('Content-Length')

    // Check if response should be compressed
    if (
      ctx.res.status === 206 || // partial content, Content-Range refers to the uncompressed bytes
      ctx.res.headers.has('Content-Encoding') || // already encoded
      ctx.res.headers.has('Transfer-Encoding') || // already encoded or chunked
      ctx.req.method === 'HEAD' || // HEAD request
      (contentLength && Number(contentLength) < threshold) || // content-length below threshold
      !shouldCompress(ctx.res) || // not compressible type
      !shouldTransform(ctx.res) // cache-control: no-transform
    ) {
      return
    }

    // From here on the representation is selected from the request's Accept-Encoding,
    // so caches must key on that header. This is done before the negotiation result is
    // known, because an identity response served for a missing or unsupported
    // Accept-Encoding is just as much a negotiated representation: RFC 9110 lists
    // Accept-Encoding "or lack thereof" as a determining factor. Without this, a shared
    // cache could reuse the identity response for a client that does accept gzip.
    // https://www.rfc-editor.org/rfc/rfc9110#field.vary
    const current = ctx.res.headers.get('Vary')
    if (current !== '*' && !(current && varyAcceptEncodingRegExp.test(current))) {
      ctx.header('Vary', current ? `${current}, Accept-Encoding` : 'Accept-Encoding')
    }

    const accepted = ctx.req.header('Accept-Encoding')
    const encoding = selectEncoding(accepted, candidates)
    if (!encoding || !ctx.res.body) {
      return
    }

    // Compress the response
    const stream = new CompressionStream(encoding)
    ctx.res = new Response(ctx.res.body.pipeThrough(stream), ctx.res)
    ctx.res.headers.delete('Content-Length')
    ctx.res.headers.set('Content-Encoding', encoding)

    // Convert strong ETag to weak ETag since compressed content is not byte-identical
    const etag = ctx.res.headers.get('ETag')
    if (etag && !etag.startsWith('W/')) {
      ctx.res.headers.set('ETag', `W/${etag}`)
    }
  }
}
```
- DEFAULT_DOCUMENT (src/middleware/serve-static/index.ts:32-32)
```typescript
const DEFAULT_DOCUMENT = 'index.html'
```
- ENCODINGS (src/middleware/serve-static/index.ts:25-29)
```typescript
const ENCODINGS = {
  br: '.br',
  zstd: '.zst',
  gzip: '.gz',
} as const
```
- ENCODINGS_ORDERED_KEYS (src/middleware/serve-static/index.ts:30-30)
```typescript
const ENCODINGS_ORDERED_KEYS = Object.keys(ENCODINGS) as (keyof typeof ENCODINGS)[]
```
- selectEncoding (src/middleware/compress/index.ts:24-44)
```typescript
const selectEncoding = (
  header: string | undefined,
  candidates: readonly Encoding[]
): Encoding | undefined => {
  if (header === undefined) {
    return undefined
  }
  const accepts = parseAccept(header)
  const wildcardQ = accepts.find((a) => a.type === '*')?.q
  let best: { encoding: Encoding; q: number } | undefined
  for (const enc of candidates) {
    const explicit = accepts.find((a) => a.type.toLowerCase() === enc)
    const q = explicit ? explicit.q : (wildcardQ ?? 0)
    if (q === 1) {
      return enc
    } else if (q > 0 && (!best || q > best.q)) {
      best = { encoding: enc, q }
    }
  }
  return best?.encoding
}
```
- RequestHeader (src/utils/headers.ts:8-264)
```typescript
export type RequestHeader =
  | 'A-IM'
  | 'Accept'
  | 'Accept-Additions'
  | 'Accept-CH'
  | 'Accept-Charset'
  | 'Accept-Datetime'
  | 'Accept-Encoding'
  | 'Accept-Features'
  | 'Accept-Language'
  | 'Accept-Patch'
  | 'Accept-Post'
  | 'Accept-Query'
  | 'Accept-Ranges'
  | 'Accept-Signature'
  | 'Access-Control'
  | 'Access-Control-Allow-Credentials'
  | 'Access-Control-Allow-Headers'
  | 'Access-Control-Allow-Methods'
  | 'Access-Control-Allow-Origin'
  | 'Access-Control-Expose-Headers'
  | 'Access-Control-Max-Age'
  | 'Access-Control-Request-Headers'
  | 'Access-Control-Request-Method'
  | 'Activate-Storage-Access'
  | 'Age'
  | 'Allow'
  | 'ALPN'
  | 'Alt-Svc'
  | 'Alt-Used'
  | 'Alternates'
  | 'AMP-Cache-Transform'
  | 'Apply-To-Redirect-Ref'
  | 'Authentication-Control'
  | 'Authentication-Info'
  | 'Authorization'
  | 'Available-Dictionary'
  | 'C-Ext'
  | 'C-Man'
  | 'C-Opt'
  | 'C-PEP'
  | 'C-PEP-Info'
  | 'Cache-Control'
