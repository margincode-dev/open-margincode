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
