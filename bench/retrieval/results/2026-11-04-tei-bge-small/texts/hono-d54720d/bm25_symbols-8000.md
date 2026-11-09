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
  | 'Cache-Group-Invalidation'
  | 'Cache-Groups'
  | 'Cache-Status'
  | 'Cal-Managed-ID'
  | 'CalDAV-Timezones'
  | 'Capsule-Protocol'
  | 'CDN-Cache-Control'
  | 'CDN-Loop'
  | 'Cert-Not-After'
  | 'Cert-Not-Before'
  | 'Clear-Site-Data'
  | 'Client-Cert'
  | 'Client-Cert-Chain'
  | 'Close'
  | 'CMCD-Object'
  | 'CMCD-Request'
  | 'CMCD-Session'
  | 'CMCD-Status'
  | 'CMSD-Dynamic'
  | 'CMSD-Static'
  | 'Concealed-Auth-Export'
  | 'Configuration-Context'
  | 'Connection'
  | 'Content-Base'
  | 'Content-Digest'
  | 'Content-Disposition'
  | 'Content-Encoding'
  | 'Content-ID'
  | 'Content-Language'
  | 'Content-Length'
  | 'Content-Location'
  | 'Content-MD5'
  | 'Content-Range'
  | 'Content-Script-Type'
  | 'Content-Security-Policy'
  | 'Content-Security-Policy-Report-Only'
  | 'Content-Style-Type'
  | 'Content-Type'
  | 'Content-Version'
  | 'Cookie'
  | 'Cookie2'
  | 'Cross-Origin-Embedder-Policy'
  | 'Cross-Origin-Embedder-Policy-Report-Only'
  | 'Cross-Origin-Opener-Policy'
  | 'Cross-Origin-Opener-Policy-Report-Only'
  | 'Cross-Origin-Resource-Policy'
  | 'CTA-Common-Access-Token'
  | 'DASL'
  | 'Date'
  | 'DAV'
  | 'Default-Style'
  | 'Delta-Base'
  | 'Deprecation'
  | 'Depth'
  | 'Derived-From'
  | 'Destination'
  | 'Detached-JWS'
  | 'Differential-ID'
  | 'Dictionary-ID'
  | 'Digest'
  | 'DPoP'
  | 'DPoP-Nonce'
  | 'Early-Data'
  | 'EDIINT-Features'
  | 'ETag'
  | 'Expect'
  | 'Expect-CT'
  | 'Expires'
  | 'Ext'
  | 'Forwarded'
  | 'From'
  | 'GetProfile'
  | 'Hobareg'
  | 'Host'
  | 'HTTP2-Settings'
  | 'If'
  | 'If-Match'
  | 'If-Modified-Since'
  | 'If-None-Match'
  | 'If-Range'
  | 'If-Schedule-Tag-Match'
  | 'If-Unmodified-Since'
  | 'IM'
  | 'Include-Referred-Token-Binding-ID'
  | 'Incremental'
  | 'Isolation'
  | 'Keep-Alive'
  | 'Label'
  | 'Last-Event-ID'
  | 'Last-Modified'
  | 'Link'
  | 'Link-Template'
  | 'Location'
  | 'Lock-Token'
  | 'Man'
  | 'Max-Forwards'
  | 'Memento-Datetime'
  | 'Meter'
  | 'Method-Check'
  | 'Method-Check-Expires'
  | 'MIME-Version'
  | 'Negotiate'
  | 'NEL'
  | 'OData-EntityId'
  | 'OData-Isolation'
  | 'OData-MaxVersion'
  | 'OData-Version'
  | 'Opt'
  | 'Optional-WWW-Authenticate'
  | 'Ordering-Type'
  | 'Origin'
  | 'Origin-Agent-Cluster'
  | 'OSCORE'
  | 'OSLC-Core-Version'
  | 'Overwrite'
  | 'P3P'
  | 'PEP'
  | 'PEP-Info'
  | 'Permissions-Policy'
  | 'PICS-Label'
  | 'Ping-From'
  | 'Ping-To'
  | 'Position'
  | 'Pragma'
  | 'Prefer'
  | 'Preference-Applied'
  | 'Priority'
  | 'ProfileObject'
  | 'Protocol'
  | 'Protocol-Info'
  | 'Protocol-Query'
  | 'Protocol-Request'
  | 'Proxy-Authenticate'
  | 'Proxy-Authentication-Info'
  | 'Proxy-Authorization'
  | 'Proxy-Features'
  | 'Proxy-Instruction'
  | 'Proxy-Status'
  | 'Public'
  | 'Public-Key-Pins'
  | 'Public-Key-Pins-Report-Only'
  | 'Range'
  | 'Redirect-Ref'
  | 'Referer'
  | 'Referer-Root'
  | 'Referrer-Policy'
  | 'Refresh'
  | 'Repeatability-Client-ID'
  | 'Repeatability-First-Sent'
  | 'Repeatability-Request-ID'
  | 'Repeatability-Result'
  | 'Replay-Nonce'
  | 'Reporting-Endpoints'
  | 'Repr-Digest'
  | 'Retry-After'
  | 'Safe'
  | 'Schedule-Reply'
  | 'Schedule-Tag'
  | 'Sec-Fetch-Dest'
  | 'Sec-Fetch-Mode'
  | 'Sec-Fetch-Site'
  | 'Sec-Fetch-Storage-Access'
  | 'Sec-Fetch-User'
  | 'Sec-GPC'
  | 'Sec-Purpose'
  | 'Sec-Token-Binding'
  | 'Sec-WebSocket-Accept'
  | 'Sec-WebSocket-Extensions'
  | 'Sec-WebSocket-Key'
  | 'Sec-WebSocket-Protocol'
  | 'Sec-WebSocket-Version'
  | 'Security-Scheme'
  | 'Server'
  | 'Server-Timing'
  | 'Set-Cookie'
  | 'Set-Cookie2'
  | 'Set-Txn'
  | 'SetProfile'
  | 'Signature'
  | 'Signature-Input'
  | 'SLUG'
  | 'SoapAction'
  | 'Status-URI'
  | 'Strict-Transport-Security'
  | 'Sunset'
  | 'Surrogate-Capability'
  | 'Surrogate-Control'
  | 'TCN'
  | 'TE'
  | 'Timeout'
  | 'Timing-Allow-Origin'
  | 'Topic'
  | 'Traceparent'
  | 'Tracestate'
  | 'Trailer'
  | 'Transfer-Encoding'
  | 'TTL'
  | 'Unencoded-Digest'
  | 'Upgrade'
  | 'Urgency'
  | 'URI'
  | 'Use-As-Dictionary'
  | 'User-Agent'
  | 'Variant-Vary'
  | 'Vary'
  | 'Via'
  | 'Want-Content-Digest'
  | 'Want-Digest'
  | 'Want-Repr-Digest'
  | 'Want-Unencoded-Digest'
  | 'Warning'
  | 'WWW-Authenticate'
  | 'X-Content-Type-Options'
  | 'X-Frame-Options'
```
- ToSSGResult (src/helper/ssg/ssg.ts:44-48)
```typescript
export interface ToSSGResult {
  success: boolean
  files: string[]
  error?: Error
}
```
- defaultJoin (src/middleware/serve-static/path.ts:5-25)
```typescript
export const defaultJoin = (...paths: string[]): string => {
  // Join non-empty paths with '/'
  let result = paths.filter((p) => p !== '').join('/')

  // Normalize multiple slashes to single slash
  result = result.replace(/(?<=\/)\/+/g, '')

  // Handle path resolution (. and ..)
  const segments = result.split('/')
  const resolved = []

  for (const segment of segments) {
    if (segment === '..' && resolved.length > 0 && resolved.at(-1) !== '..') {
      resolved.pop()
    } else if (segment !== '.') {
      resolved.push(segment)
    }
  }

  return resolved.join('/') || '.'
}
```
- getNextAcceptValue (src/utils/accept.ts:159-209)
```typescript
const getNextAcceptValue = (
  acceptHeader: string,
  startIndex: number
): [number, Accept | undefined] => {
  const accept: Accept = {
    type: '',
    params: Object.create(null),
    q: 1,
  }
  startIndex = consumeWhitespace(acceptHeader, startIndex)
  let i = startIndex
  while (i < acceptHeader.length) {
    const char = acceptHeader.charCodeAt(i)
    if (char === 59 || char === 44) {
      // ';' or ',' => end of type
      accept.type = acceptHeader.slice(startIndex, ignoreTrailingWhitespace(acceptHeader, i))
      i++
      if (char === 44) {
        // ',' => end of value
        return [i, accept.type ? accept : undefined]
      }
      if (!accept.type) {
        return [skipInvalidAcceptValue(acceptHeader, i), undefined]
      }
      break // parse params
    }
    i++
  }
  if (!accept.type) {
    accept.type = acceptHeader.slice(
      startIndex,
      ignoreTrailingWhitespace(acceptHeader, acceptHeader.length)
    )
    return [acceptHeader.length, accept.type ? accept : undefined]
  }

  let param: string | undefined
  let value: string | undefined
  let hasNext: boolean
  while (i < acceptHeader.length) {
    ;[i, param, value, hasNext] = getNextParam(acceptHeader, i)
    if (param && value) {
      accept.params[param] = value
    }
    if (!hasNext) {
      break
    }
  }

  return [i, accept] as [number, Accept]
}
```
- matchType (src/helper/accepts/accepts.ts:21-40)
```typescript
const matchType = (acceptType: string, supportedType: string): boolean => {
  // Media types and language tags are both case-insensitive (RFC 9110 §8.3.1,
  // RFC 4647 §2.1), so compare them case-folded. `defaultMatch` returns the
  // caller's own `supports` entry, so its casing is preserved.
  const accept = acceptType.toLowerCase()
  const supported = supportedType.toLowerCase()

  if (accept === supported) {
    return true
  }
  if (accept === '*/*' || accept === '*') {
    return false
  }
  if (accept.endsWith('/*')) {
    const [acceptMain] = accept.split('/')
    const [supportedMain] = supported.split('/')
    return acceptMain === supportedMain
  }
  return false
}
```
- Accept (src/utils/accept.ts:1-5)
```typescript
export interface Accept {
  type: string
  params: Record<string, string>
  q: number
}
```
- skipInvalidAcceptValue (src/utils/accept.ts:46-64)
```typescript
const skipInvalidAcceptValue = (acceptHeader: string, startIndex: number): number => {
  let i = startIndex
  let inQuotes = false
  while (i < acceptHeader.length) {
    const char = acceptHeader.charCodeAt(i)
    if (inQuotes && char === 92) {
      // '\' => escape
      i++
    } else if (char === 34) {
      // '"' => toggle quotes
      inQuotes = !inQuotes
    } else if (!inQuotes && char === 44) {
      // ',' => next accept value
      return i + 1
    }
    i++
  }
  return i
}
```
- isContentEncodingBinary (adapters/aws-lambda/src/handler.ts:680-682)
```typescript
export const isContentEncodingBinary = (contentEncoding: string | null) => {
  return !!contentEncoding && !/^identity$/i.test(contentEncoding)
}
```
- isContentEncodingBinary (src/adapter/aws-lambda/handler.ts:724-726)
```typescript
export const isContentEncodingBinary = (contentEncoding: string | null) => {
  return !!contentEncoding && !/^identity$/i.test(contentEncoding)
}
```
- Encoding (src/middleware/compress/index.ts:13-13)
```typescript
type Encoding = (typeof ENCODING_TYPES)[number]
```
- getNextParam (src/utils/accept.ts:66-157)
```typescript
const getNextParam = (
  acceptHeader: string,
  startIndex: number
): [number, string | undefined, string | undefined, boolean] => {
  startIndex = consumeWhitespace(acceptHeader, startIndex)
  let i = startIndex
  let key: string | undefined
  let value: string | undefined
  let hasNext = false
  while (i < acceptHeader.length) {
    const char = acceptHeader.charCodeAt(i)
    if (char === 61) {
      // '=' => end of key
      key = acceptHeader.slice(startIndex, ignoreTrailingWhitespace(acceptHeader, i))
      i++
      break
    }
    if (char === 59) {
      // ';' => invalid empty param, continue parsing params
      return [i + 1, undefined, undefined, true]
    }
    if (char === 44) {
      // ',' => invalid empty param, move to next accept value
      return [i + 1, undefined, undefined, false]
    }
    i++
  }
  if (key === undefined) {
    return [i, undefined, undefined, false]
  }

  i = consumeWhitespace(acceptHeader, i)
  if (acceptHeader.charCodeAt(i) === 61) {
    // '=' is invalid as a value, so return undefined
    const skipResult = skipInvalidParam(acceptHeader, i + 1)
    return [skipResult[0], key, undefined, skipResult[1]]
  }

  let inQuotes = false
  const paramStartIndex = i
  while (i < acceptHeader.length) {
    const char = acceptHeader.charCodeAt(i)

    if (inQuotes && char === 92) {
      // '\' => escape
      i++
    } else if (char === 34) {
      // '"' => start of quotes
      if (inQuotes) {
        let nextIndex = consumeWhitespace(acceptHeader, i + 1)
        const nextChar = acceptHeader.charCodeAt(nextIndex)
        if (nextIndex < acceptHeader.length && !(nextChar === 59 || nextChar === 44)) {
          // not ';' or ',' => invalid trailing chars
          const skipResult = skipInvalidParam(acceptHeader, nextIndex)
          return [skipResult[0], key, undefined, skipResult[1]]
        }
        value = acceptHeader.slice(paramStartIndex + 1, i)
        if (value.includes('\\')) {
          value = value.replace(/\\(.)/g, '$1')
        }
        if (nextChar === 44) {
          // ',' => end of accept value
          return [nextIndex + 1, key, value, false]
        }
        if (nextChar === 59) {
          // ';' => has next param
          hasNext = true
          nextIndex++
        }
        i = nextIndex
        break
      }
      inQuotes = true
    } else if (!inQuotes && (char === 59 || char === 44)) {
      // ';' or ',' => end of value
      value = acceptHeader.slice(paramStartIndex, ignoreTrailingWhitespace(acceptHeader, i))
      if (char === 59) {
        // ';' => has next param
        hasNext = true
      }
      i++
      break
    }
    i++
  }
  return [
    i,
    key,
    value ?? acceptHeader.slice(paramStartIndex, ignoreTrailingWhitespace(acceptHeader, i)),
    hasNext,
  ]
}
```
- parseAcceptLanguage (src/middleware/language/language.ts:74-76)
```typescript
export function parseAcceptLanguage(header: string): Array<{ lang: string; q: number }> {
  return parseAccept(header).map(({ type, q }) => ({ lang: type, q }))
}
```
- accepts (src/helper/accepts/accepts.ts:87-96)
```typescript
export const accepts = (c: Context, options: acceptsOptions): string => {
  const acceptHeader = c.req.header(options.header)
  if (!acceptHeader) {
    return options.default
  }
  const accepts = parseAccept(acceptHeader)
  const match = options.match || defaultMatch

  return match(accepts, options)
}
```
- consumeWhitespace (src/utils/accept.ts:10-18)
```typescript
const consumeWhitespace = (acceptHeader: string, startIndex: number): number => {
  while (startIndex < acceptHeader.length) {
    if (!isWhitespace(acceptHeader.charCodeAt(startIndex))) {
      break
    }
    startIndex++
  }
  return startIndex
}
```
- skipInvalidParam (src/utils/accept.ts:30-44)
```typescript
const skipInvalidParam = (acceptHeader: string, startIndex: number): [number, boolean] => {
  while (startIndex < acceptHeader.length) {
    const char = acceptHeader.charCodeAt(startIndex)
    if (char === 59) {
      // ';' => next param
      return [startIndex + 1, true]
    }
    if (char === 44) {
      // ',' => next accept value
      return [startIndex + 1, false]
    }
    startIndex++
  }
  return [startIndex, false]
}
```
- Accept (src/helper/accepts/accepts.ts:5-9)
```typescript
export interface Accept {
  type: string
  params: Record<string, string>
  q: number
}
```
- ENCODING_TYPES (src/middleware/compress/index.ts:12-12)
```typescript
const ENCODING_TYPES = ['gzip', 'deflate'] as const
```
- defaultMatch (src/helper/accepts/accepts.ts:52-72)
```typescript
export const defaultMatch = (accepts: Accept[], config: acceptsConfig): string => {
  const { supports, default: defaultSupport } = config
  // A quality value of 0 means "not acceptable" (RFC 9110 §12.5.1), so such
  // entries must never match, the same rule the compress middleware applies.
  const sortedAccepts = accepts
    .filter((accept) => accept.q > 0)
