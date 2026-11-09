### src/middleware/language/index.test.ts
```typescript
import { languageDetector } from '.'
import { Hono } from '../../hono'
import { DEFAULT_OPTIONS, detectors, normalizeLanguage } from './language'

describe('languageDetector', () => {
  const createTestApp = (options = {}) => {
    const app = new Hono()

    app.use('/*', languageDetector(options))

    app.get('/*', (c) => c.text(c.get('language')))

    return app
  }

  describe('Query Parameter Detection', () => {
    it('should detect language from query parameter', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr', 'es'],
        fallbackLanguage: 'en',
      })

      const res = await app.request('/?lang=fr')
      expect(await res.text()).toBe('fr')
    })

    it('should ignore unsupported languages in query', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
      })

      const res = await app.request('/?lang=de')
      expect(await res.text()).toBe('en')
    })
  })

  describe('Cookie Detection', () => {
    it('should detect language from cookie', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
      })

      const res = await app.request('/', {
        headers: {
          cookie: 'language=fr',
        },
      })
      expect(await res.text()).toBe('fr')
    })

    it('should cache detected language in cookie when enabled', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        caches: ['cookie'],
      })

      const res = await app.request('/?lang=fr')
      expect(res.headers.get('set-cookie')).toContain('language=fr')
    })
  })

  describe('Header Detection', () => {
    it('should detect language from Accept-Language header', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr', 'es'],
        fallbackLanguage: 'en',
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'fr-FR,fr;q=0.9,en;q=0.8',
        },
      })
      expect(await res.text()).toBe('fr')
    })

    it('should fallback to language code when locale code is not in supportedLanguages', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'ja'],
        fallbackLanguage: 'en',
        order: ['header'],
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'ja-JP',
        },
      })
      expect(await res.text()).toBe('ja')
    })

    it('should not detect a language explicitly rejected with q=0', async () => {
      const app = createTestApp({
        supportedLanguages: ['fr', 'en'],
        fallbackLanguage: 'en',
        order: ['header'],
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'fr;q=0',
        },
      })
      expect(await res.text()).toBe('en')
    })

    it('should skip q=0 languages before falling back to the default', async () => {
      const app = createTestApp({
        supportedLanguages: ['fr', 'en'],
        fallbackLanguage: 'en',
        order: ['header'],
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'fr;q=0,de;q=0.8',
        },
      })
      expect(await res.text()).toBe('en')
    })

    it('should continue to the next detector after a zero-quality language', async () => {
      const app = createTestApp({
        supportedLanguages: ['fr', 'en', 'ja'],
        fallbackLanguage: 'en',
        order: ['header', 'querystring'],
      })

      const res = await app.request('/?lang=ja', {
        headers: { 'accept-language': 'fr;q=0' },
      })
      expect(await res.text()).toBe('ja')
      expect(res.headers.get('set-cookie')).toContain('language=ja')
    })

    it('should match after multiple truncations', async () => {
      const app = createTestApp({
        supportedLanguages: ['zh-Hant', 'en'],
        fallbackLanguage: 'en',
        order: ['header'],
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'zh-Hant-CN',
        },
      })
      expect(await res.text()).toBe('zh-Hant')
    })

    it('should select the longest truncated match regardless of supported language order', async () => {
      const app = createTestApp({
        supportedLanguages: ['zh', 'zh-Hant'],
        fallbackLanguage: 'zh',
        order: ['header'],
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'zh-Hant-CN',
        },
      })
      expect(await res.text()).toBe('zh-Hant')
    })

    it('should fallback when truncation does not match any supported language', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'ja'],
        fallbackLanguage: 'en',
        order: ['header'],
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'ko-KR',
        },
      })
      expect(await res.text()).toBe('en')
    })

    it('should prefer exact match over truncated match', async () => {
      const app = createTestApp({
        supportedLanguages: ['fr', 'fr-CA'],
        fallbackLanguage: 'fr',
        order: ['header'],
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'fr-CA',
        },
      })
      expect(await res.text()).toBe('fr-CA')
    })

    it('should handle case-insensitive truncation matching', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'ja'],
        fallbackLanguage: 'en',
        order: ['header'],
        ignoreCase: true,
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'JA-JP',
        },
      })
      expect(await res.text()).toBe('ja')
    })

    it('should handle malformed Accept-Language headers', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'invalid;header;;format',
        },
      })
      expect(await res.text()).toBe('en')
    })

    it('should handle invalid header lookup key gracefully', async () => {
      const app = createTestApp({
        order: ['header'],
        lookupFromHeaderKey: 'accept\nlanguage',
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
      })

      const res = await app.request('/', {
        headers: {
          'accept-language': 'fr',
        },
      })

      expect(await res.text()).toBe('en')
    })
  })

  describe('Language Normalization', () => {
    it('should preserve the configured value for a case-insensitive truncated match', () => {
      expect(
        normalizeLanguage('zh-HANT-CN', {
          ...DEFAULT_OPTIONS,
          supportedLanguages: ['en', 'ZH-Hant'],
        })
      ).toBe('ZH-Hant')
    })

    it('should handle a large unsupported language tag', () => {
      const unsupportedLanguage = 'x-'.repeat(30_000).slice(0, -1)

      expect(
        normalizeLanguage(unsupportedLanguage, {
          ...DEFAULT_OPTIONS,
          supportedLanguages: ['en', 'ja'],
        })
      ).toBeUndefined()
    })
  })

  describe('Path Detection', () => {
    it('should detect language from path', async () => {
      const app = createTestApp({
        order: ['path'],
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        lookupFromPathIndex: 0,
      })

      const res = await app.request('/fr/page')
      expect(await res.text()).toBe('fr')
    })

    it('should handle invalid path index gracefully', async () => {
      const app = createTestApp({
        order: ['path'],
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        lookupFromPathIndex: 99,
      })

      const res = await app.request('/fr/page')
      expect(await res.text()).toBe('en')
    })

    it('should detect language from original URL when getPath modifies the path', async () => {
      const app = new Hono({
        getPath: (req) => {
          const url = new URL(req.url)
          let pathname = url.pathname
          // Remove language prefix /fr/
          if (pathname.startsWith('/fr/')) {
            pathname = pathname.replace('/fr/', '/')
          }
          return pathname
        },
      })

      app.use(
        '*',
        languageDetector({
          order: ['path'],
          supportedLanguages: ['en', 'fr'],
