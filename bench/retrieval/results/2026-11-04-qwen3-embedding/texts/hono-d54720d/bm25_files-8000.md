### src/middleware/serve-static/index.test.ts
```typescript
import { serveStatic as baseServeStatic } from '.'
import { Hono } from '../../hono'

describe('Serve Static Middleware with a Blob body', () => {
  it('Should serve content returned as a Blob', async () => {
    const app = new Hono()
    app.use(
      '/static/*',
      baseServeStatic({
        getContent: async (path) => new Blob([`Hello in ${path}`]),
      })
    )
    const res = await app.request('http://localhost/static/hello.txt')
    expect(res.status).toBe(200)
    expect(await res.text()).toBe('Hello in static/hello.txt')
  })
})

describe('Serve Static Middleware', () => {
  const app = new Hono()
  const getContent = vi.fn(async (path) => {
    if (path.endsWith('not-found.txt')) {
      return null
    }
    return `Hello in ${path}`
  })

  const serveStatic = baseServeStatic({
    getContent,
    isDir: (path) => {
      if (path === 'static/sub' || path === 'static/hello.world') {
        return true
      }
    },
    onFound: (path, c) => {
      if (path.endsWith('hello.html')) {
        c.header('X-Custom', `Found the file at ${path}`)
      }
    },
  })

  app.get('/static/*', serveStatic)

  beforeEach(() => {
    getContent.mockClear()
  })

  it.each(['GET', 'HEAD'])(
    'Should serve files for %s requests registered with use()',
    async (method) => {
      const app = new Hono().use('/static/*', serveStatic)

      const res = await app.request('/static/hello.html', { method })

      expect(res.status).toBe(200)
      expect(res.headers.get('Content-Type')).toMatch(/^text\/html/)
      expect(await res.text()).toBe(method === 'HEAD' ? '' : 'Hello in static/hello.html')
      expect(getContent).toBeCalledTimes(1)
    }
  )

  it.each(['POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS', 'PROPFIND'])(
    'Should pass %s requests to the next handler without accessing files',
    async (method) => {
      const isDir = vi.fn(() => false)
      const rewriteRequestPath = vi.fn((path: string) => path)
      const onFound = vi.fn()
      const onNotFound = vi.fn()
      const app = new Hono().use(
        '/static/*',
        baseServeStatic({ getContent, isDir, rewriteRequestPath, onFound, onNotFound })
      )
      app.on(method, '/static/hello.html', (c) => c.text('Handled downstream', 202))

      const res = await app.request('/static/hello.html', { method })

      expect(res.status).toBe(202)
      expect(await res.text()).toBe('Handled downstream')
      expect(rewriteRequestPath).not.toBeCalled()
      expect(isDir).not.toBeCalled()
      expect(getContent).not.toBeCalled()
      expect(onFound).not.toBeCalled()
      expect(onNotFound).not.toBeCalled()
    }
  )

  it.each(['/static/hello.html', '/static/not-found.txt'])(
    'Should return 404 for an unhandled POST request regardless of file existence - %s',
    async (path) => {
      const app = new Hono().use('/static/*', serveStatic)

      const res = await app.request(path, { method: 'POST' })

      expect(res.status).toBe(404)
      expect(await res.text()).toBe('404 Not Found')
      expect(getContent).not.toBeCalled()
    }
  )

  it('Should return 200 response - /static/hello.html', async () => {
    const res = await app.request('/static/hello.html')
    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Encoding')).toBeNull()
    expect(res.headers.get('Content-Type')).toMatch(/^text\/html/)
    expect(await res.text()).toBe('Hello in static/hello.html')
    expect(res.headers.get('X-Custom')).toBe('Found the file at static/hello.html')
  })

  it('Should return 200 response - /static/sub', async () => {
    const res = await app.request('/static/sub')
    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Type')).toMatch(/^text\/html/)
    expect(await res.text()).toBe('Hello in static/sub/index.html')
  })

  it('Should return 200 response - /static/hello.world', async () => {
    const res = await app.request('/static/hello.world')
    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Type')).toMatch(/^text\/html/)
    expect(await res.text()).toBe('Hello in static/hello.world/index.html')
  })

  it.each([
    ['/static/%E7%82%8E.txt', 'static/炎.txt'],
    ['/static/hello%20world.txt', 'static/hello world.txt'],
  ])('Should decode URI strings - %s', async (url, path) => {
    const res = await app.request(url)
    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Type')).toMatch(/^text\/plain/)
    expect(await res.text()).toBe(`Hello in ${path}`)
  })

  it.each(['/static/100%25.txt', '/static/100%25/hello.txt', '/static/%2Fadmin/secret.txt'])(
    'Should skip paths containing percent signs by default - %s',
    async (path) => {
      const onNotFound = vi.fn()
      const app = new Hono().use('*', baseServeStatic({ getContent, onNotFound }))

      const res = await app.request(path)

      expect(res.status).toBe(404)
      expect(getContent).not.toBeCalled()
      expect(onNotFound).toHaveBeenCalledWith(path, expect.anything())
    }
  )

  it.each([
    ['/static/100%25.txt', 'static/100%.txt'],
    ['/static/100%25/hello.txt', 'static/100%/hello.txt'],
    ['/static/%2Fadmin/secret.txt', 'static/%2Fadmin/secret.txt'],
  ])('Should allow percent signs when opted in - %s', async (url, path) => {
    const app = new Hono().use('*', baseServeStatic({ getContent, allowPercentInPath: true }))

    const res = await app.request(url)

    expect(res.status).toBe(200)
    expect(await res.text()).toBe(`Hello in ${path}`)
  })

  it('Should not bypass authentication through a second decode', async () => {
    const app = new Hono()
    app.get('/static/admin/*', (c) => c.text('Unauthorized', 401))
    app.use('/static/*', baseServeStatic({ getContent }))

    const protectedRes = await app.request('/static/admin/secret.txt')
    expect(protectedRes.status).toBe(401)

    const res = await app.request('/static/%%36%31dmin/secret.txt')
    expect(res.status).toBe(404)
    expect(getContent).not.toBeCalled()
  })

  it('Should return 404 response - /static/not-found.txt', async () => {
    const res = await app.request('/static/not-found.txt')
    expect(res.status).toBe(404)
    expect(res.headers.get('Content-Encoding')).toBeNull()
    expect(res.headers.get('Content-Type')).toMatch(/^text\/plain/)
    expect(await res.text()).toBe('404 Not Found')
    expect(getContent).toBeCalledTimes(1)
  })

  it('Should return 200 response for empty string content - /static/empty.txt', async () => {
    const onNotFound = vi.fn()
    const app = new Hono().use(
      '*',
      baseServeStatic({
        getContent: async () => '',
        onNotFound,
      })
    )
    const res = await app.request('/static/empty.txt')
    expect(res.status).toBe(200)
    expect(await res.text()).toBe('')
    expect(onNotFound).not.toBeCalled()
  })

  it('Should not allow a directory traversal - /static/%2e%2e/static/hello.html', async () => {
    const res = await app.fetch({
      method: 'GET',
      url: 'http://localhost/static/%2e%2e/static/hello.html',
    } as Request)
    expect(res.status).toBe(404)
    expect(res.headers.get('Content-Type')).toMatch(/^text\/plain/)
    expect(await res.text()).toBe('404 Not Found')
  })

  it('Should not allow a backslash separator injection - /static/admin%5Csecret.txt', async () => {
    const res = await app.fetch({
      method: 'GET',
      url: 'http://localhost/static/admin%5Csecret.txt',
    } as Request)
    expect(res.status).toBe(404)
    expect(await res.text()).toBe('404 Not Found')
  })

  it('Should return a pre-compressed zstd response - /static/hello.html', async () => {
    const app = new Hono().use(
      '*',
      baseServeStatic({
        getContent,
        precompressed: true,
      })
    )

    const res = await app.request('/static/hello.html', {
      headers: { 'Accept-Encoding': 'zstd' },
    })

    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Encoding')).toBe('zstd')
    expect(res.headers.get('Vary')).toBe('Accept-Encoding')
    expect(res.headers.get('Content-Type')).toMatch(/^text\/html/)
    expect(await res.text()).toBe('Hello in static/hello.html.zst')
  })

  it('Should return a pre-compressed brotli response - /static/hello.html', async () => {
    const app = new Hono().use(
      '*',
      baseServeStatic({
        getContent,
        precompressed: true,
      })
    )

    const res = await app.request('/static/hello.html', {
      headers: { 'Accept-Encoding': 'wompwomp, gzip, br, deflate, zstd' },
    })

    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Encoding')).toBe('br')
    expect(res.headers.get('Vary')).toBe('Accept-Encoding')
    expect(res.headers.get('Content-Type')).toMatch(/^text\/html/)
    expect(await res.text()).toBe('Hello in static/hello.html.br')
  })

  it('Should return a pre-compressed brotli response - /static/hello.unknown', async () => {
    const app = new Hono().use(
      '*',
      baseServeStatic({
        getContent,
        precompressed: true,
      })
    )

    const res = await app.request('/static/hello.unknown', {
      headers: { 'Accept-Encoding': 'wompwomp, gzip, br, deflate, zstd' },
    })

    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Encoding')).toBe('br')
    expect(res.headers.get('Vary')).toBe('Accept-Encoding')
    expect(res.headers.get('Content-Type')).toBe('application/octet-stream')
    expect(await res.text()).toBe('Hello in static/hello.unknown.br')
  })

  it('Should not return a pre-compressed response - /static/not-found.txt', async () => {
    const app = new Hono().use(
      '*',
      baseServeStatic({
        getContent,
        precompressed: true,
      })
    )

    const res = await app.request('/static/not-found.txt', {
      headers: { 'Accept-Encoding': 'gzip, zstd, br' },
    })

    expect(res.status).toBe(404)
    expect(res.headers.get('Content-Encoding')).toBeNull()
    expect(res.headers.get('Vary')).toBeNull()
    expect(res.headers.get('Content-Type')).toMatch(/^text\/plain/)
    expect(await res.text()).toBe('404 Not Found')
  })

  it('Should not return a pre-compressed response - /static/hello.html', async () => {
    const app = new Hono().use(
      '*',
      baseServeStatic({
        getContent,
        precompressed: true,
      })
    )

    const res = await app.request('/static/hello.html', {
      headers: { 'Accept-Encoding': 'wompwomp, unknown' },
    })

    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Encoding')).toBeNull()
    expect(res.headers.get('Vary')).toBeNull()
    expect(res.headers.get('Content-Type')).toMatch(/^text\/html/)
    expect(await res.text()).toBe('Hello in static/hello.html')
  })

  it('Should not find pre-compressed files - /static/hello.jpg', async () => {
    const app = new Hono().use(
      '*',
      baseServeStatic({
        getContent,
        precompressed: true,
      })
    )

    const res = await app.request('/static/hello.jpg', {
      headers: { 'Accept-Encoding': 'gzip, br, deflate, zstd' },
    })

    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Encoding')).toBeNull()
    expect(res.headers.get('Vary')).toBeNull()
    expect(res.headers.get('Content-Type')).toMatch(/^image\/jpeg/)
    expect(await res.text()).toBe('Hello in static/hello.jpg')
  })

  it('Should return response object content as-is', async () => {
    const body = new ReadableStream()
    const response = new Response(body)
    const app = new Hono().use(
      '*',
      baseServeStatic({
        getContent: async () => {
          return response
        },
      })
    )

    const res = await app.fetch({
      method: 'GET',
      url: 'http://localhost',
    } as Request)
    expect(res.status).toBe(200)
    expect(res.body).toBe(body)
  })

  describe('Changing root path', () => {
    it('Should return the content with absolute root path', async () => {
      const app = new Hono()
      const serveStatic = baseServeStatic({
        getContent,
        root: '/home/hono/child',
      })
      app.get('/static/*', serveStatic)

      const res = await app.request('/static/html/hello.html')
      expect(await res.text()).toBe('Hello in /home/hono/child/static/html/hello.html')
    })

    it('Should traverse the directories with absolute root path', async () => {
      const app = new Hono()
      const serveStatic = baseServeStatic({
        getContent,
        root: '/home/hono/../parent',
      })
      app.get('/static/*', serveStatic)

      const res = await app.request('/static/html/hello.html')
      expect(await res.text()).toBe('Hello in /home/parent/static/html/hello.html')
    })

    it('Should treat the root path includes .. as relative path', async () => {
      const app = new Hono()
      const serveStatic = baseServeStatic({
        getContent,
        root: '../home/hono',
      })
      app.get('/static/*', serveStatic)

      const res = await app.request('/static/html/hello.html')
      expect(await res.text()).toBe('Hello in ../home/hono/static/html/hello.html')
    })

    it('Should not allow directory traversal with . as relative path', async () => {
      const app = new Hono()
      const serveStatic = baseServeStatic({
        getContent,
        root: '.',
      })
      app.get('*', serveStatic)

      const res = await app.request('/etc/passwd')
      expect(await res.text()).toBe('Hello in etc/passwd')
    })

    it('Should not allow bypass via path mismatch between middleware and serveStatic', async () => {
      const app = new Hono()

      app.use('/admin/*', async (c, next) => {
        c.header('X-Authorized', 'true')
        await next()
      })

      const serveStatic = baseServeStatic({
        getContent,
        root: '.',
      })
      app.use('/*', serveStatic)

      const res = await app.request('/admin/secret.txt')
      expect(res.headers.get('X-Authorized')).toBe('true')
      expect(await res.text()).toBe('Hello in admin/secret.txt')

      const res2 = await app.request('/admin%2Fsecret.txt')
      expect(res2.headers.get('X-Authorized')).toBeNull()
      expect(res2.status).toBe(404)

      const res3 = await app.request('//admin/secret.txt')
      expect(res3.status).toBe(404)
    })
  })
})
```
### src/middleware/serve-static/index.ts
```typescript
/**
 * @module
 * Serve Static Middleware for Hono.
 */

import type { Context, Data } from '../../context'
import type { Env, MiddlewareHandler } from '../../types'
import { COMPRESSIBLE_CONTENT_TYPE_REGEX } from '../../utils/compress'
import { getMimeType } from '../../utils/mime'
import { tryDecodeURI } from '../../utils/url'
import { defaultJoin } from './path'

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

const ENCODINGS = {
  br: '.br',
  zstd: '.zst',
  gzip: '.gz',
} as const
const ENCODINGS_ORDERED_KEYS = Object.keys(ENCODINGS) as (keyof typeof ENCODINGS)[]

const DEFAULT_DOCUMENT = 'index.html'

/**
 * This middleware is not directly used by the user. Create a wrapper specifying `getContent()` by the environment such as Deno or Bun.
 */
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
### adapters/cloudflare-workers/src/serve-static.ts
```typescript
import { serveStatic as baseServeStatic } from 'hono/serve-static'
import type { ServeStaticOptions as BaseServeStaticOptions } from 'hono/serve-static'
import type { Env, MiddlewareHandler } from 'hono/types'
import { getContentFromKVAsset } from './utils'

export type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & {
  // namespace is KVNamespace
  namespace?: unknown
  manifest?: object | string
}

/**
 * @deprecated
 * `serveStatic` in the Cloudflare Workers adapter is deprecated.
 * You can serve static files directly using Cloudflare Static Assets.
 * @see https://developers.cloudflare.com/workers/static-assets/
 * Cloudflare Static Assets is currently in open beta. If this doesn't work for you,
 * please consider using Cloudflare Pages. You can start to create the Cloudflare Pages
 * application with the `npm create hono@latest` command.
 */
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
### src/adapter/cloudflare-workers/serve-static.ts
```typescript
import { serveStatic as baseServeStatic } from '../../middleware/serve-static'
import type { ServeStaticOptions as BaseServeStaticOptions } from '../../middleware/serve-static'
import type { Env, MiddlewareHandler } from '../../types'
import { getContentFromKVAsset } from './utils'

/**
 * @deprecated `hono/cloudflare-workers` will be removed in v5. Install `@hono/cloudflare-workers` and import from there instead.
 */
export type ServeStaticOptions<E extends Env = Env> = BaseServeStaticOptions<E> & {
  // namespace is KVNamespace
  namespace?: unknown
  manifest?: object | string
}

/**
 * @deprecated
 * `serveStatic` in the Cloudflare Workers adapter is deprecated.
 * You can serve static files directly using Cloudflare Static Assets.
 * @see https://developers.cloudflare.com/workers/static-assets/
 * Cloudflare Static Assets is currently in open beta. If this doesn't work for you,
 * please consider using Cloudflare Pages. You can start to create the Cloudflare Pages
 * application with the `npm create hono@latest` command.
 */
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
### adapters/bun/test/index.test.ts
```typescript
import { afterAll, beforeEach, describe, expect, it, mock } from 'bun:test'
import fs from 'node:fs/promises'
import path from 'node:path'
import { Hono } from 'hono'
import type { WSMessageReceive } from 'hono/ws'
import { createBunWebSocket, serveStatic, toSSG } from '../src/index'
import type { BunWebSocketData } from '../src/index'

describe('serveStatic', () => {
  const app = new Hono()
  const onNotFound = mock(() => {})
  app.all('/favicon.ico', serveStatic({ path: './test/favicon.ico' }))
  app.all('/favicon-notfound.ico', serveStatic({ path: './test/favicon-notfound.ico', onNotFound }))
  app.use('/favicon-notfound.ico', async (c, next) => {
    await next()
    c.header('X-Custom', 'Bun')
  })
  app.get(
    '/static/*',
    serveStatic({
      root: './test/',
      onNotFound,
    })
  )
  app.get(
    '/dot-static/*',
    serveStatic({
      root: './test/',
      rewriteRequestPath: (path) => path.replace(/^\/dot-static/, './.static'),
    })
  )
  app.all('/static-absolute-root/*', serveStatic({ root: path.dirname(__filename) }))

  beforeEach(() => onNotFound.mockClear())

  it('Should return static file correctly', async () => {
    const res = await app.request(new Request('http://localhost/favicon.ico'))
    await res.arrayBuffer()
    expect(res.status).toBe(200)
    expect(res.headers.get('Content-Type')).toBe('image/x-icon')
  })

  it('Should return 404 response', async () => {
    const res = await app.request(new Request('http://localhost/favicon-notfound.ico'))
    expect(res.status).toBe(404)
    expect(res.headers.get('X-Custom')).toBe('Bun')
    expect(onNotFound).toHaveBeenCalledWith(
      process.platform === 'win32' ? 'test\\favicon-notfound.ico' : 'test/favicon-notfound.ico',
      expect.anything()
    )
  })

  it('Should return 200 response - /static/plain.txt', async () => {
    const res = await app.request(new Request('http://localhost/static/plain.txt'))
    expect(res.status).toBe(200)
    expect(await res.text()).toMatch(/^Bun!(\r?\n)?$/)
    expect(onNotFound).not.toHaveBeenCalled()
  })

  it('Should return 200 response - /static/download', async () => {
    const res = await app.request(new Request('http://localhost/static/download'))
    expect(res.status).toBe(200)
    expect(await res.text()).toMatch(/^download(\r?\n)?$/)
    expect(onNotFound).not.toHaveBeenCalled()
  })

  it('Should return 200 response - /dot-static/plain.txt', async () => {
    const res = await app.request(new Request('http://localhost/dot-static/plain.txt'))
    expect(res.status).toBe(200)
    expect(await res.text()).toMatch(/^Bun!!(\r?\n)?$/)
  })

  it('Should return 200 response - /static/helloworld', async () => {
    const res = await app.request('http://localhost/static/helloworld')
    expect(res.status).toBe(200)
    expect(await res.text()).toMatch(/Hi\r?\n/)
  })

  it('Should return 200 response - /static/hello.world', async () => {
    const res = await app.request('http://localhost/static/hello.world')
    expect(res.status).toBe(200)
    expect(await res.text()).toMatch(/Hi\r?\n/)
  })

  it('Should return 200 response - /static-absolute-root/plain.txt', async () => {
    const res = await app.request('http://localhost/static-absolute-root/plain.txt')
    expect(res.status).toBe(200)
    expect(await res.text()).toMatch(/^Bun!(\r?\n)?$/)
    expect(onNotFound).not.toHaveBeenCalled()
  })
})

describe('toSSG', () => {
  const dir = './test/.ssg-output'

  afterAll(async () => {
    await fs.rm(dir, { recursive: true, force: true })
  })

  it('Should generate static files for the routes', async () => {
    const app = new Hono()
    app.get('/', (c) => c.text('Hello, World!'))
    app.get('/about', (c) => c.text('About Page'))
    app.get('/about/some', (c) => c.text('About Page 2tier'))
    app.post('/about/some/thing', (c) => c.text('About Page 3tier'))
    app.get('/bravo', (c) => c.html('Bravo Page'))

    const result = await toSSG(app, { dir })
    expect(result.success).toBeTruthy()
    expect(result.error).toBeUndefined()
    expect(result.files).toBeDefined()
    expect(await fs.readFile(path.join(dir, 'index.txt'), 'utf8')).toBe('Hello, World!')
  })
})

describe('createBunWebSocket', () => {
  const app = new Hono()
  const { websocket, upgradeWebSocket } = createBunWebSocket()

  it('Should deliver messages to the handler', async () => {
    const receivedMessagePromise = new Promise<WSMessageReceive>((resolve) =>
      app.get(
        '/ws',
        upgradeWebSocket(() => ({
          onMessage(evt) {
            resolve(evt.data)
          },
        }))
      )
    )
    const upgradedData = await new Promise<BunWebSocketData>((resolve) =>
      app.fetch(new Request('http://localhost/ws'), {
        upgrade: (_req: Request, data: { data: BunWebSocketData }) => {
          resolve(data.data)
        },
      })
    )
    const message = Math.random().toString()
    websocket.message(
      {
        close: () => undefined,
        readyState: 3,
        data: upgradedData,
        send: () => undefined,
      },
      message
    )
    expect(await receivedMessagePromise).toBe(message)
  })
})
```
### src/helper/ssg/plugins.test.tsx
```typescript
import { Hono } from '../../hono'
import type { RedirectStatusCode, StatusCode } from '../../utils/http-status'
import * as plugins from './plugins'
import { toSSG } from './ssg'
import type { FileSystemModule } from './ssg'

const { defaultPlugin, redirectPlugin } = plugins

describe('Built-in SSG plugins', () => {
  let app: Hono
  let fsMock: FileSystemModule

  beforeEach(() => {
    app = new Hono()
    app.get('/', (c) => c.html('<h1>Home</h1>'))
    app.get('/about', (c) => c.html('<h1>About</h1>'))
    app.get('/blog', (c) => c.html('<h1>Blog</h1>'))
    app.get('/created', (c) => c.text('201 Created', 201))
    app.get('/redirect', (c) => c.redirect('/'))
    app.get('/notfound', (c) => c.notFound())
    app.get('/error', (c) => c.text('500 Error', 500))

    fsMock = {
      writeFile: vi.fn(() => Promise.resolve()),
      mkdir: vi.fn(() => Promise.resolve()),
    }
  })

  describe('default plugin', () => {
    it('uses defaultPlugin when plugins option is omitted', async () => {
      const defaultPluginSpy = vi.spyOn(plugins, 'defaultPlugin')
      await toSSG(app, fsMock, { dir: './static' })
      expect(defaultPluginSpy).toHaveBeenCalled()
      defaultPluginSpy.mockRestore()
    })

    it('skips non-200 responses with defaultPlugin', async () => {
      const result = await toSSG(app, fsMock, { plugins: [defaultPlugin()], dir: './static' })
      expect(fsMock.writeFile).toHaveBeenCalledWith('static/index.html', '<h1>Home</h1>')
      expect(fsMock.writeFile).toHaveBeenCalledWith('static/about.html', '<h1>About</h1>')
      expect(fsMock.writeFile).toHaveBeenCalledWith('static/blog.html', '<h1>Blog</h1>')
      expect(fsMock.writeFile).not.toHaveBeenCalledWith('static/created.txt', expect.any(String))
      expect(fsMock.writeFile).not.toHaveBeenCalledWith('static/redirect.txt', expect.any(String))
      expect(fsMock.writeFile).not.toHaveBeenCalledWith('static/notfound.txt', expect.any(String))
      expect(fsMock.writeFile).not.toHaveBeenCalledWith('static/error.txt', expect.any(String))
      expect(result.files.some((f) => f.includes('created'))).toBe(false)
      expect(result.files.some((f) => f.includes('redirect'))).toBe(false)
      expect(result.files.some((f) => f.includes('notfound'))).toBe(false)
      expect(result.files.some((f) => f.includes('error'))).toBe(false)
      expect(result.success).toBe(true)
    })
  })

  describe('redirect plugin', () => {
    it('generates redirect HTML for status codes requiring Location per HTTP Semantics specification', async () => {
      const statusCodes = [301, 302, 303, 307, 308] satisfies RedirectStatusCode[]
      for (const statusCode of statusCodes) {
        const writtenFiles: Record<string, string> = {}
        const fsMockLocal: FileSystemModule = {
          writeFile: (path, data) => {
            writtenFiles[path] = typeof data === 'string' ? data : data.toString()
            return Promise.resolve()
          },
          mkdir: vi.fn(() => Promise.resolve()),
        }
        const redirectApp = new Hono()
        redirectApp.get('/old', (c) => c.redirect('/new', statusCode)) // Default is 302
        redirectApp.get('/new', (c) => c.html('New Page'))

        await toSSG(redirectApp, fsMockLocal, { dir: './static', plugins: [redirectPlugin()] })

        expect(writtenFiles['static/old.html']).toBeDefined()
        const content = writtenFiles['static/old.html']
        // Should contain meta refresh
        expect(content).toContain('meta http-equiv="refresh" content="0;url=/new"')
        // Should contain canonical
        expect(content).toContain('rel="canonical" href="/new"')
        // Should contain robots noindex
        expect(content).toContain('<meta name="robots" content="noindex" />')
        // Should contain link anchor
        expect(content).toContain('<a href="/new">Redirecting to <code>/new</code></a>')
        // Should contain a body element that includes the anchor
        expect(content).toMatch(/<body[^>]*>[\s\S]*<a href=\"\/new\">[\s\S]*<\/body>/)
      }
    })

    it('skips generating redirect HTML for status codes requiring Location when Location header is missing', async () => {
      const statusCodes = [301, 302, 303, 307, 308] satisfies RedirectStatusCode[]
      for (const statusCode of statusCodes) {
        const writtenFiles: Record<string, string> = {}
        const fsMockLocal: FileSystemModule = {
          writeFile: (path, data) => {
            writtenFiles[path] = typeof data === 'string' ? data : data.toString()
            return Promise.resolve()
          },
          mkdir: vi.fn(() => Promise.resolve()),
        }
        const redirectApp = new Hono()
        redirectApp.get('/bad', () => new Response(null, { status: statusCode }))

        await toSSG(redirectApp, fsMockLocal, { dir: './static', plugins: [redirectPlugin()] })

        expect(writtenFiles['static/bad.html']).toBeUndefined()
      }
    })

    it('skips generating redirect HTML for status codes not requiring Location per HTTP Semantics specification', async () => {
      const statusCodes = [300, 304, 305, 306] satisfies RedirectStatusCode[]
      for (const statusCode of statusCodes) {
        const writtenFiles: Record<string, string> = {}
        const fsMockLocal: FileSystemModule = {
          writeFile: (path, data) => {
            writtenFiles[path] = typeof data === 'string' ? data : data.toString()
            return Promise.resolve()
          },
          mkdir: vi.fn(() => Promise.resolve()),
        }
        const redirectApp = new Hono()

        redirectApp.get(
          '/response',
          () => new Response(null, { status: statusCode, headers: { Location: '/' } })
        )

