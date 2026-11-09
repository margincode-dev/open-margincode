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

