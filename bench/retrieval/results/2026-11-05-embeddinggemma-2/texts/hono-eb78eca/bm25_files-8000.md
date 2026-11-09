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
          fallbackLanguage: 'en',
          lookupFromPathIndex: 0,
        })
      )

      app.get('/home', (c) => c.text(c.get('language')))

      const res = await app.request('/fr/home')
      expect(await res.text()).toBe('fr')
    })
  })

  describe('Detection Order', () => {
    it('should respect detection order', async () => {
      const app = createTestApp({
        order: ['cookie', 'querystring'],
        supportedLanguages: ['en', 'fr', 'es'],
        fallbackLanguage: 'en',
      })

      const res = await app.request('/?lang=fr', {
        headers: {
          cookie: 'language=es',
        },
      })

      // Since cookie is first in order, it should use 'es'
      expect(await res.text()).toBe('es')
    })

    it('should fall back to next detector if first fails', async () => {
      const app = createTestApp({
        order: ['cookie', 'querystring'],
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
      })

      const res = await app.request('/?lang=fr') // No cookie
      expect(await res.text()).toBe('fr') // Should use querystring
    })
  })

  describe('Language Conversion', () => {
    it('should apply language conversion function', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        convertDetectedLanguage: (lang: string) => lang.split('-')[0],
      })

      const res = await app.request('/?lang=fr-FR')
      expect(await res.text()).toBe('fr')
    })

    it('should handle case sensitivity according to options', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        ignoreCase: false,
      })

      const res = await app.request('/?lang=FR')
      expect(await res.text()).toBe('en') // Falls back because case doesn't match
    })
  })

  describe('Error Handling', () => {
    it('should fall back to default language on error', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
      })

      const detector = vi.spyOn(detectors, 'querystring').mockImplementation(() => {
        throw new Error('Simulated error')
      })

      const res = await app.request('/?lang=fr')
      expect(await res.text()).toBe('en')

      detector.mockRestore()
    })

    it('should handle missing cookie values gracefully', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        order: ['cookie'],
      })

      const res = await app.request('/')
      expect(await res.text()).toBe('en')
    })

    it('should handle errors in convertDetectedLanguage function', async () => {
      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        convertDetectedLanguage: (lang: string) => {
          if (lang === 'fr') {
            throw new Error('Simulated error in convertDetectedLanguage')
          }
          return lang
        },
      })

      const res = await app.request('/?lang=fr')
      expect(await res.text()).toBe('en')
    })
  })

  describe('Configuration Validation', () => {
    it('should throw if fallback language is not in supported languages', () => {
      expect(() => {
        createTestApp({
          supportedLanguages: ['fr', 'es'],
          fallbackLanguage: 'en',
        })
      }).toThrow()
    })

    it('should throw if path index is negative', () => {
      expect(() => {
        createTestApp({
          lookupFromPathIndex: -1,
        })
      }).toThrow()
    })

    it('should handle empty supported languages list', () => {
      expect(() => {
        createTestApp({
          supportedLanguages: [],
        })
      }).toThrow()
    })

    it('should handle invalid order option', async () => {
      expect(() => {
        createTestApp({
          order: ['invalidDetector' as any],
        })
      }).toThrow()
    })

    it('should handle early return for caches', async () => {
      const app = createTestApp({
        caches: ['test'],
        supportedLanguages: ['en'],
      })

      const res = await app.request('/?lang=en')

      expect(await res.text()).toBe('en')
      expect(res.headers.get('set-cookie')).toBeNull()
    })
  })

  describe('Debug Mode', () => {
    it('should log errors in debug mode', async () => {
      const consoleErrorSpy = vi.spyOn(console, 'error')

      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        debug: true,
      })

      const detector = vi.spyOn(detectors, 'querystring').mockImplementation(() => {
        throw new Error('Simulated error')
      })

      await app.request('/?lang=fr')

      expect(consoleErrorSpy).toHaveBeenCalledWith(
        'Error in querystring detector:',
        expect.any(Error)
      )

      detector.mockRestore()
      consoleErrorSpy.mockRestore()
    })

    // The log test remains unchanged
    it('should log debug information when enabled', async () => {
      const consoleSpy = vi.spyOn(console, 'log')

      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        debug: true,
      })

      await app.request('/?lang=fr')

      expect(consoleSpy).toHaveBeenCalledWith(
        expect.stringContaining('Language detected from querystring')
      )

      consoleSpy.mockRestore()
    })

    it('should handle cookie cache errors in debug mode', async () => {
      const consoleErrorSpy = vi.spyOn(console, 'error')

      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        caches: ['cookie'],
        lookupCookie: 'bad cookie',
        debug: true,
      })

      const res = await app.request('/?lang=fr')

      expect(await res.text()).toBe('fr')
      expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to cache language:', expect.any(Error))

      consoleErrorSpy.mockRestore()
    })

    it('should silently handle cookie cache errors when debug mode is disabled', async () => {
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {})

      const app = createTestApp({
        supportedLanguages: ['en', 'fr'],
        fallbackLanguage: 'en',
        caches: ['cookie'],
        lookupCookie: 'bad cookie',
        debug: false,
      })

      const res = await app.request('/?lang=fr')

      expect(await res.text()).toBe('fr')
      expect(consoleErrorSpy).not.toHaveBeenCalled()

      consoleErrorSpy.mockRestore()
    })
  })
})
```
### src/middleware/language/language.ts
```typescript
/**
 * @module
 * Language module for Hono.
 */
import type { Context } from '../../context'
import { setCookie, getCookie } from '../../helper/cookie'
import type { MiddlewareHandler } from '../../types'
import { parseAccept } from '../../utils/accept'

export type DetectorType = 'path' | 'querystring' | 'cookie' | 'header'
export type CacheType = 'cookie'

export interface DetectorOptions {
  /** Order of language detection strategies */
  order: DetectorType[]
  /** Query parameter name for language */
  lookupQueryString: string
  /** Cookie name for language */
  lookupCookie: string
  /** Index in URL path where language code appears */
  lookupFromPathIndex: number
  /** Header key for language detection */
  lookupFromHeaderKey: string
  /** Caching strategies */
  caches: CacheType[] | false
  /** Cookie configuration options */
  cookieOptions?: {
    domain?: string
    path?: string
    sameSite?: 'Strict' | 'Lax' | 'None'
    secure?: boolean
    maxAge?: number
    httpOnly?: boolean
  }
  /** Whether to ignore case in language codes */
  ignoreCase: boolean
  /** Default language if none detected */
  fallbackLanguage: string
  /** List of supported language codes */
  supportedLanguages: string[]
  /** Optional function to transform detected language codes */
  convertDetectedLanguage?: (lang: string) => string
  /** Enable debug logging */
  debug?: boolean
}

export interface LanguageVariables {
  language: string
}

export const DEFAULT_OPTIONS: DetectorOptions = {
  order: ['querystring', 'cookie', 'header'],
  lookupQueryString: 'lang',
  lookupCookie: 'language',
  lookupFromHeaderKey: 'accept-language',
  lookupFromPathIndex: 0,
  caches: ['cookie'],
  ignoreCase: true,
  fallbackLanguage: 'en',
  supportedLanguages: ['en'],
  cookieOptions: {
    sameSite: 'Strict',
    secure: true,
    maxAge: 365 * 24 * 60 * 60,
    httpOnly: true,
  },
  debug: false,
}
/**
 * Parse Accept-Language header values with quality scores
 * @param header Accept-Language header string
 * @returns Array of parsed languages with quality scores
 */
export function parseAcceptLanguage(header: string): Array<{ lang: string; q: number }> {
  return parseAccept(header).map(({ type, q }) => ({ lang: type, q }))
}

/**
 * Validate and normalize language codes
 * @param lang Language code to normalize
 * @param options Detector options
 * @returns Normalized language code or undefined
 */
export const normalizeLanguage = (
  lang: string | null | undefined,
  options: DetectorOptions
): string | undefined => {
  if (!lang) {
    return undefined
  }

  try {
    let normalizedLang = lang.trim()
    if (options.convertDetectedLanguage) {
      normalizedLang = options.convertDetectedLanguage(normalizedLang)
    }

    const compLang = options.ignoreCase ? normalizedLang.toLowerCase() : normalizedLang
    const compSupported = options.supportedLanguages.map((l) =>
      options.ignoreCase ? l.toLowerCase() : l
    )

    // Exact match
    const exactIndex = compSupported.indexOf(compLang)
    if (exactIndex !== -1) {
      return options.supportedLanguages[exactIndex]
    }

    // Progressive truncation (RFC 4647 Lookup)
    let longestMatchIndex = -1
    let longestMatchLength = -1
    for (let i = 0; i < compSupported.length; i++) {
      const candidate = compSupported[i]
      if (
        candidate.length < compLang.length &&
        candidate.length > longestMatchLength &&
        compLang.startsWith(candidate) &&
        compLang[candidate.length] === '-'
      ) {
        longestMatchIndex = i
        longestMatchLength = candidate.length
      }
    }
    if (longestMatchIndex !== -1) {
      return options.supportedLanguages[longestMatchIndex]
    }

    return undefined
  } catch {
    return undefined
  }
}

/**
 * Detects language from query parameter
 */
export const detectFromQuery = (c: Context, options: DetectorOptions): string | undefined => {
  const query = c.req.query(options.lookupQueryString)
  return normalizeLanguage(query, options)
}

/**
 * Detects language from cookie
 */
export const detectFromCookie = (c: Context, options: DetectorOptions): string | undefined => {
  const cookie = getCookie(c, options.lookupCookie)
  return normalizeLanguage(cookie, options)
}

/**
 * Detects language from Accept-Language header
 */
export function detectFromHeader(c: Context, options: DetectorOptions): string | undefined {
  try {
    const acceptLanguage = c.req.header(options.lookupFromHeaderKey)
    if (!acceptLanguage) {
      return undefined
    }

    const languages = parseAcceptLanguage(acceptLanguage)
    for (const { lang, q } of languages) {
      // A quality value of 0 means the language is explicitly rejected
      // (RFC 9110 §12.5.4), so it must not be detected.
      if (q === 0) {
        continue
      }
      const normalizedLang = normalizeLanguage(lang, options)
      if (normalizedLang) {
        return normalizedLang
      }
    }
    return undefined
  } catch {
    return undefined
  }
}

/**
 * Detects language from URL path
 */
export function detectFromPath(c: Context, options: DetectorOptions): string | undefined {
  const url = new URL(c.req.url)
  const pathSegments = url.pathname.split('/').filter(Boolean)
  const langSegment = pathSegments[options.lookupFromPathIndex]
  return normalizeLanguage(langSegment, options)
}

/**
 * Collection of all language detection strategies
 */
export const detectors = {
  querystring: detectFromQuery,
  cookie: detectFromCookie,
  header: detectFromHeader,
  path: detectFromPath,
} as const

/** Type for detector functions */
export type DetectorFunction = (c: Context, options: DetectorOptions) => string | undefined

/** Type-safe detector map */
export type Detectors = Record<keyof typeof detectors, DetectorFunction>

/**
 * Validate detector options
 * @param options Detector options to validate
 * @throws Error if options are invalid
 */
export function validateOptions(options: DetectorOptions): void {
  if (!options.supportedLanguages.includes(options.fallbackLanguage)) {
    throw new Error('Fallback language must be included in supported languages')
  }

  if (options.lookupFromPathIndex < 0) {
    throw new Error('Path index must be non-negative')
  }

  if (!options.order.every((detector) => Object.keys(detectors).includes(detector))) {
    throw new Error('Invalid detector type in order array')
  }
}

/**
 * Cache detected language
 */
function cacheLanguage(c: Context, language: string, options: DetectorOptions): void {
  if (!Array.isArray(options.caches) || !options.caches.includes('cookie')) {
    return
  }

  try {
    setCookie(c, options.lookupCookie, language, options.cookieOptions)
  } catch (error) {
    if (options.debug) {
      console.error('Failed to cache language:', error)
    }
  }
}

/**
 * Detect language from request
 */
const detectLanguage = (c: Context, options: DetectorOptions): string => {
  let detectedLang: string | undefined

  for (const detectorName of options.order) {
    const detector = detectors[detectorName]

    try {
      detectedLang = detector(c, options)
      if (detectedLang) {
        if (options.debug) {
          console.log(`Language detected from ${detectorName}: ${detectedLang}`)
        }
        break
      }
    } catch (error) {
      if (options.debug) {
        console.error(`Error in ${detectorName} detector:`, error)
      }
      continue
    }
  }

  const finalLang = detectedLang || options.fallbackLanguage

  if (detectedLang && options.caches) {
    cacheLanguage(c, finalLang, options)
  }

  return finalLang
}

/**
 * Language detector middleware factory
 * @param userOptions Configuration options for the language detector
 * @returns Hono middleware function
 *
 * @example
 * ```ts
 * type Variables = LanguageVariables
 * const app = new Hono<{ Variables: Variables }>()
 *
 * app.use(
 *   languageDetector({
 *     supportedLanguages: ['en', 'ja'], // Must include fallback
 *     fallbackLanguage: 'en', // Required
 *   })
 * )
 *
 * app.get('/', (c) => {
 *   const lang = c.get('language')
 *   return c.text(`Current language: ${lang}`)
 * })
 * ```
 */
export const languageDetector = (userOptions: Partial<DetectorOptions>): MiddlewareHandler => {
  const options: DetectorOptions = {
    ...DEFAULT_OPTIONS,
    ...userOptions,
    cookieOptions: {
      ...DEFAULT_OPTIONS.cookieOptions,
      ...userOptions.cookieOptions,
    },
  }

  validateOptions(options)

  return async function languageDetector(ctx, next) {
    const lang = detectLanguage(ctx, options)
    ctx.set('language', lang)
    await next()
  }
}
```
### src/utils/accept.test.ts
```typescript
import { parseAccept } from './accept'

describe('parseAccept Comprehensive Tests', () => {
  describe('Basic Functionality', () => {
    test('parses simple accept header', () => {
      const header = 'text/html,application/json;q=0.9'
      expect(parseAccept(header)).toEqual([
        { type: 'text/html', params: {}, q: 1 },
        { type: 'application/json', params: { q: '0.9' }, q: 0.9 },
      ])
    })

    test('handles missing header', () => {
      expect(parseAccept('')).toEqual([])
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      expect(parseAccept(undefined as any)).toEqual([])
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      expect(parseAccept(null as any)).toEqual([])
    })

    test('handles whitespace-only header', () => {
      expect(parseAccept('   ')).toEqual([])
      expect(parseAccept(' \t\n ')).toEqual([])
    })
  })

  describe('Quality Values', () => {
    test('handles extreme q values', () => {
      const header = 'a;q=999999,b;q=-99999,c;q=Infinity,d;q=-Infinity,e;q=NaN'
      const result = parseAccept(header)
      // Each side clamps to the bound it is past, so a negative q sorts below a real one
      // rather than above it -- `-99999` lands where `-Infinity` already did. Asserted as
      // pairs because the clamp makes the sequence non-monotonic, which sorts the result.
      expect(result.map((x) => [x.type, x.q])).toEqual([
        ['a', 1],
        ['c', 1],
        ['b', 0],
        ['d', 0],
        ['e', 0],
      ])
    })

    test('a negative q does not outrank a valid one', () => {
      const result = parseAccept('application/json;q=0.9,text/html;q=-1')
      expect(result.map((x) => [x.type, x.q])).toEqual([
        ['application/json', 0.9],
        ['text/html', 0],
      ])
      // The sort is by descending q, so the refused type must not come first.
      expect(result[0].type).toBe('application/json')
    })

    test('handles malformed q values', () => {
      const header = 'a;q=,b;q=invalid,c;q=1.2.3,d;q=true,e;q="0.5"'
      const result = parseAccept(header)
      expect(result.every((x) => x.q >= 0 && x.q <= 1)).toBe(true)
    })

    test('preserves original q string in params', () => {
      const header = 'type;q=invalid'
      const result = parseAccept(header)
      expect(result[0].params.q).toBe('invalid')
      expect(result[0].q).toBe(1) // Normalized q value
    })

    test('treats the q parameter name as case-insensitive', () => {
      // RFC 9110 §12.4.2: the "q" parameter is case-insensitive.
      expect(parseAccept('text/html;Q=0.5')).toEqual([
        { type: 'text/html', params: { Q: '0.5' }, q: 0.5 },
      ])
      expect(parseAccept('text/html;Q=0')[0].q).toBe(0)
    })

    test('sorts by quality when the q parameter is uppercase', () => {
      const result = parseAccept('text/html;Q=0.5,application/json;q=0.9')
      expect(result.map((x) => x.type)).toEqual(['application/json', 'text/html'])
    })
  })

  describe('Parameter Handling', () => {
    test('handles complex parameters', () => {
      const header = 'type;a=1;b="2";c=\'3\';d="semi;colon";e="nested"quoted""'
      const result = parseAccept(header)
      expect(result[0].params).toEqual({
        a: '1',
        b: '2',
        c: "'3'",
        d: 'semi;colon',
      })
    })

    test('handles malformed parameters', () => {
      const header = 'type;=value;;key=;=;====;key====value'
      const result = parseAccept(header)
      expect(result[0].type).toBe('type')
      expect(Object.keys(result[0].params).length).toBe(0)
    })

    test('handles duplicate parameters', () => {
      const header = 'type;key=1;key=2;KEY=3'
      const result = parseAccept(header)
      expect(result[0].params.key).toBe('2')
      expect(result[0].params.KEY).toBe('3')
    })

    test('treats `__proto__` as a normal parameter without changing the prototype', () => {
      const result = parseAccept('text/html;__proto__=x')
      expect(Object.getPrototypeOf(result[0].params)).toBeNull()
      expect(result[0].params['__proto__']).toBe('x')
    })
  })

  describe('Media Type Edge Cases', () => {
    test('handles malformed media types', () => {
      const headers = [
        '*/html',
        'text/*mal/formed',
        '/partial',
        'missing/',
        'inv@lid/type',
        'text/(html)',
        'text/html?invalid',
      ]
      headers.forEach((header) => {
        const result = parseAccept(header)
        expect(result[0].type).toBe(header)
      })
    })

    test('handles extremely long types', () => {
      const longType = 'a'.repeat(10000) + '/' + 'b'.repeat(10000)
      const result = parseAccept(longType)
      expect(result[0].type).toBe(longType)
    })
  })

  describe('Delimiter Edge Cases', () => {
    test('handles multiple consecutive delimiters', () => {
      const header = 'a,,,,b;q=0.9,,,,c;q=0.8,,,,'
      const result = parseAccept(header)
      expect(result.map((x) => x.type)).toEqual(['a', 'b', 'c'])
    })

    test('handles unusual whitespace', () => {
      const header = '\n\t a \t\n ; \n\t q=0.9 \t\n , \n\t b \t\n'
      const result = parseAccept(header)
      expect(result.map((x) => x.type)).toEqual(['b', 'a'])
    })

    test('handles comma inside quoted parameter value', () => {
      const header = 'text/plain;meta="a,b";q=0.8,application/json;q=0.7'
      const result = parseAccept(header)
      expect(result).toEqual([
        {
          type: 'text/plain',
          params: {
            meta: 'a,b',
            q: '0.8',
          },
          q: 0.8,
        },
        {
          type: 'application/json',
          params: {
            q: '0.7',
          },
          q: 0.7,
        },
      ])
    })

    test('handles escaped quote and semicolon inside quoted parameter', () => {
      const header = 'text/plain;meta="a\\\";b";q=0.5'
      const result = parseAccept(header)
      expect(result).toEqual([
        {
          type: 'text/plain',
          params: {
            meta: 'a";b',
            q: '0.5',
          },
          q: 0.5,
        },
      ])
    })

    test('handles escaped character inside quoted parameter', () => {
      const header = 'text/plain;meta="a\\\\z;c";q=0.3'
      const result = parseAccept(header)
      expect(result).toEqual([
        {
          type: 'text/plain',
          params: {
            meta: 'a\\z;c',
            q: '0.3',
          },
          q: 0.3,
        },
      ])
    })

    test('skips invalid param without swallowing next media type', () => {
      const header = 'a;foo, b;q=0.5'
      const result = parseAccept(header)
      expect(result).toEqual([
        { type: 'a', params: {}, q: 1 },
        { type: 'b', params: { q: '0.5' }, q: 0.5 },
      ])
    })

    test('skips malformed quoted param tail without creating bogus media type', () => {
      const header = 'a;foo="x"bar,b'
      const result = parseAccept(header)
      expect(result).toEqual([
        { type: 'a', params: {}, q: 1 },
        { type: 'b', params: {}, q: 1 },
      ])
    })

    test('parses params after quoted value with trailing whitespace', () => {
      const header = 'a;foo="x" ;q=0.5,b;q=0.4'
      const result = parseAccept(header)
      expect(result).toEqual([
        { type: 'a', params: { foo: 'x', q: '0.5' }, q: 0.5 },
        { type: 'b', params: { q: '0.4' }, q: 0.4 },
      ])
    })

    test('handles quoted param followed immediately by comma', () => {
      const header = 'a;foo="x",b'
      const result = parseAccept(header)
      expect(result).toEqual([
        { type: 'a', params: { foo: 'x' }, q: 1 },
        { type: 'b', params: {}, q: 1 },
      ])
    })

    test('skips empty media type that starts with semicolon', () => {
      const header = ';q=0.5,b;q=0.4'
      const result = parseAccept(header)
      expect(result).toEqual([{ type: 'b', params: { q: '0.4' }, q: 0.4 }])
    })
  })

  describe('Security Cases', () => {
    test('handles potential injection patterns', () => {
      const headers = [
        'type;q=0.9--',
        'type;q=0.9;drop table users',
        'type;__|;q=0.9',
        'text/html"><script>alert(1)</script>',
        'application/json${process.env}',
      ]
      headers.forEach((header) => {
        expect(() => parseAccept(header)).not.toThrow()
      })
    })

    test('handles many semicolons with an unbalanced quote', () => {
      const header = `text/plain;${'a;'.repeat(8000)}"`
      const result = parseAccept(header)
      expect(result[0].type).toBe('text/plain')
    })

    test('handles extremely large input', () => {
      const header = 'a;q=0.9,'.repeat(100000)
      expect(() => parseAccept(header)).not.toThrow()
    })
  })

  describe('Unicode and Special Characters', () => {
    test('handles unicode in types and parameters', () => {
      const header = '🌐/😊;param=🔥;q=0.9'
      const result = parseAccept(header)
      expect(result[0].type).toBe('🌐/😊')
      expect(result[0].params.param).toBe('🔥')
    })

    test('handles special characters', () => {
