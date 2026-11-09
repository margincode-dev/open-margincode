### src/helper/css/index.ts
```typescript
/**
 * @module
 * css Helper for Hono.
 */

import { raw } from '../../helper/html'
import { DOM_RENDERER } from '../../jsx/constants'
import { createCssJsxDomObjects } from '../../jsx/dom/css'
import { escapeToBuffer } from '../../utils/html'
import type { HtmlEscapedCallback, HtmlEscapedString } from '../../utils/html'
import type {
  ClassNameSlug,
  CssClassName as CssClassNameCommon,
  CssVariableType,
  OnInvalidSlug,
} from './common'
import {
  CLASS_NAME,
  DEFAULT_STYLE_ID,
  PSEUDO_GLOBAL_SELECTOR,
  SELECTOR,
  SELECTORS,
  STYLE_STRING,
  cssCommon,
  cxCommon,
  keyframesCommon,
  viewTransitionCommon,
} from './common'
export { rawCssString } from './common'
export type { ClassNameSlug, OnInvalidSlug } from './common'

type CssClassName = HtmlEscapedString & CssClassNameCommon

type usedClassNameData = [
  Record<string, string>, // class name to add
  Record<string, true>, // class name already added
]

interface CssType {
  (strings: TemplateStringsArray, ...values: CssVariableType[]): Promise<string>
}

interface CxType {
  (
    ...args: (CssClassName | Promise<string> | string | boolean | null | undefined)[]
  ): Promise<string>
}

interface KeyframesType {
  (strings: TemplateStringsArray, ...values: CssVariableType[]): CssClassNameCommon
}

interface ViewTransitionType {
  (strings: TemplateStringsArray, ...values: CssVariableType[]): Promise<string>
  (content: Promise<string>): Promise<string>
  (): Promise<string>
}

interface StyleType {
  (args?: { children?: Promise<string>; nonce?: string }): HtmlEscapedString
}

/**
 * @experimental
 * `createCssContext` is an experimental feature.
 * The API might be changed.
 *
 * @param options.id - The ID for the style element
 * @param options.classNameSlug - Optional function to customize generated CSS class names
 * @param options.onInvalidSlug - Optional callback function called when an invalid slug is returned from ClassNameSlug
 */
export const createCssContext = ({
  id,
  classNameSlug,
  onInvalidSlug,
}: {
  id: Readonly<string>
  classNameSlug?: ClassNameSlug
  onInvalidSlug?: OnInvalidSlug
}): DefaultContextType => {
  const [cssJsxDomObject, StyleRenderToDom] = createCssJsxDomObjects({ id })

  const contextMap: WeakMap<object, usedClassNameData> = new WeakMap()
  const nonceMap: WeakMap<object, string | undefined> = new WeakMap()

  const replaceStyleRe = new RegExp(`(<style id="${id}"(?: nonce="[^"]*")?>.*?)(</style>)`)

  const newCssClassNameObject = (cssClassName: CssClassNameCommon): Promise<string> => {
    const appendStyle: HtmlEscapedCallback = ({ buffer, context }): Promise<string> | undefined => {
      const [toAdd, added] = contextMap.get(context) as usedClassNameData
      const names = Object.keys(toAdd)

      if (!names.length) {
        return
      }

      let stylesStr = ''
      names.forEach((className) => {
        added[className] = true
        stylesStr += className.startsWith(PSEUDO_GLOBAL_SELECTOR)
          ? toAdd[className]
          : `${className[0] === '@' ? '' : '.'}${className}{${toAdd[className]}}`
      })
      contextMap.set(context, [{}, added])

      if (buffer && replaceStyleRe.test(buffer[0])) {
        buffer[0] = buffer[0].replace(replaceStyleRe, (_, pre, post) => `${pre}${stylesStr}${post}`)
        return
      }

      const nonce = nonceMap.get(context)
      const appendStyleScript = `<script${
        nonce ? ` nonce="${nonce}"` : ''
      }>document.querySelector('#${id}').textContent+=${JSON.stringify(stylesStr)}</script>`

      if (buffer) {
        buffer[0] = `${appendStyleScript}${buffer[0]}`
        return
      }

      return Promise.resolve(appendStyleScript)
    }

    const addClassNameToContext: HtmlEscapedCallback = ({ context }) => {
      if (!contextMap.has(context)) {
        contextMap.set(context, [{}, {}])
      }
      const [toAdd, added] = contextMap.get(context) as usedClassNameData
      let allAdded = true
      if (!added[cssClassName[SELECTOR]]) {
        allAdded = false
        toAdd[cssClassName[SELECTOR]] = cssClassName[STYLE_STRING]
      }
      cssClassName[SELECTORS].forEach(
        ({ [CLASS_NAME]: className, [STYLE_STRING]: styleString }) => {
          if (!added[className]) {
            allAdded = false
            toAdd[className] = styleString
          }
        }
      )
      if (allAdded) {
        return
      }

      return Promise.resolve(raw('', [appendStyle]))
    }

    // external class names from cx() are untrusted but the result is marked isEscaped,
    // so escape it here. skip the buffer when there is nothing to escape.
    const rawClassName = cssClassName[CLASS_NAME]
    let escapedClassName = rawClassName
    if (/[&<>'"]/.test(rawClassName)) {
      const escapedBuffer: [string] = ['']
      escapeToBuffer(rawClassName, escapedBuffer)
      escapedClassName = escapedBuffer[0]
    }
    const className = new String(escapedClassName) as CssClassName
    Object.assign(className, cssClassName)
    ;(className as HtmlEscapedString).isEscaped = true
    ;(className as HtmlEscapedString).callbacks = [addClassNameToContext]
    const promise = Promise.resolve(className)
    Object.assign(promise, cssClassName)

    promise.toString = cssJsxDomObject.toString
    return promise
  }

  const css: CssType = (strings, ...values) => {
    return newCssClassNameObject(cssCommon(strings, values, classNameSlug, onInvalidSlug))
  }

  const cx: CxType = (...args) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    args = cxCommon(args as any) as any
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return css(Array(args.length).fill('') as any, ...args)
  }

  const keyframes: KeyframesType = (strings, ...values) =>
    keyframesCommon(strings, values, classNameSlug, onInvalidSlug)

  const viewTransition: ViewTransitionType = ((
    strings: TemplateStringsArray | Promise<string> | undefined,
    ...values: CssVariableType[]
  ) => {
    return newCssClassNameObject(
      viewTransitionCommon(strings as any, values, classNameSlug, onInvalidSlug) // eslint-disable-line @typescript-eslint/no-explicit-any
    )
  }) as ViewTransitionType

  const Style: StyleType = ({ children, nonce } = {}) =>
    raw(
      `<style id="${id}"${nonce ? ` nonce="${nonce}"` : ''}>${
        children ? (children as unknown as CssClassName)[STYLE_STRING] : ''
      }</style>`,
      [
        ({ context }) => {
          nonceMap.set(context, nonce)
          return undefined
        },
      ]
    )

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  ;(Style as any)[DOM_RENDERER] = StyleRenderToDom

  return {
    css,
    cx,
    keyframes,
    viewTransition: viewTransition as ViewTransitionType,
    Style,
  }
}

interface DefaultContextType {
  css: CssType
  cx: CxType
  keyframes: KeyframesType
  viewTransition: ViewTransitionType
  Style: StyleType
}

const defaultContext: DefaultContextType = createCssContext({
  id: DEFAULT_STYLE_ID,
})

/**
 * @experimental
 * `css` is an experimental feature.
 * The API might be changed.
 */
export const css = defaultContext.css

/**
 * @experimental
 * `cx` is an experimental feature.
 * The API might be changed.
 */
export const cx = defaultContext.cx

/**
 * @experimental
 * `keyframes` is an experimental feature.
 * The API might be changed.
 */
export const keyframes = defaultContext.keyframes

/**
 * @experimental
 * `viewTransition` is an experimental feature.
 * The API might be changed.
 */
export const viewTransition = defaultContext.viewTransition

/**
 * @experimental
 * `Style` is an experimental feature.
 * The API might be changed.
 */
export const Style = defaultContext.Style
```
### src/jsx/utils.test.ts
```typescript
import {
  isValidAttributeName,
  isValidTagName,
  normalizeIntrinsicElementKey,
  styleObjectForEach,
} from './utils'

describe('normalizeIntrinsicElementKey', () => {
  test.each`
    key                | expected
    ${'className'}     | ${'class'}
    ${'htmlFor'}       | ${'for'}
    ${'crossOrigin'}   | ${'crossorigin'}
    ${'httpEquiv'}     | ${'http-equiv'}
    ${'itemProp'}      | ${'itemprop'}
    ${'fetchPriority'} | ${'fetchpriority'}
    ${'noModule'}      | ${'nomodule'}
    ${'formAction'}    | ${'formaction'}
    ${'href'}          | ${'href'}
  `('should convert $key to $expected', ({ key, expected }) => {
    expect(normalizeIntrinsicElementKey(key)).toBe(expected)
  })
})

describe('isValidAttributeName', () => {
  test.each`
    name
    ${'class'}
    ${'id'}
    ${'href'}
    ${'data-foo'}
    ${'aria-label'}
    ${'onclick'}
    ${'viewBox'}
    ${'xml:lang'}
  `('should return true for valid name "$name"', ({ name }) => {
    expect(isValidAttributeName(name)).toBe(true)
  })

  test.each`
    name                             | description
    ${''}                            | ${'empty string'}
    ${'" onfocus="alert(1)'}         | ${'double quote'}
    ${"' onfocus='alert(1)"}         | ${'single quote'}
    ${'foo<bar'}                     | ${'less than'}
    ${'"><script>alert(1)</script>'} | ${'greater than'}
    ${'foo bar'}                     | ${'space'}
    ${'foo=bar'}                     | ${'equals sign'}
    ${'foo/bar'}                     | ${'slash'}
    ${'foo\\bar'}                    | ${'backslash'}
    ${'foo`bar'}                     | ${'backtick'}
    ${'a\x00b'}                      | ${'null byte'}
    ${'a\x1fb'}                      | ${'control character'}
    ${'a\x7fb'}                      | ${'DEL character'}
  `('should return false for "$description"', ({ name }) => {
    expect(isValidAttributeName(name)).toBe(false)
  })

  test.each`
    name            | description
    ${'xlink:href'} | ${'namespace separator'}
    ${'data.foo'}   | ${'dot'}
    ${'data_foo'}   | ${'underscore'}
    ${'é'}          | ${'non-ascii'}
  `('should return true for valid "$description" names', ({ name }) => {
    expect(isValidAttributeName(name)).toBe(true)
  })

  it('should keep validating names after the valid attribute name cache is reset', () => {
    for (let i = 0; i < 1025; i++) {
      expect(isValidAttributeName(`data-k${i}`)).toBe(true)
    }

    expect(isValidAttributeName('class')).toBe(true)
    expect(isValidAttributeName('" onfocus="alert(1)')).toBe(false)
  })
})

describe('isValidTagName', () => {
  test.each`
    name
    ${''}
    ${'div'}
    ${'h1'}
    ${'custom-element'}
    ${'clipPath'}
    ${'foo:bar'}
    ${'x.foo'}
    ${'x_bar'}
    ${'é'}
  `('should return true for valid tag name "$name"', ({ name }) => {
    expect(isValidTagName(name)).toBe(true)
  })

  test.each`
    name                             | description
    ${'foo bar'}                     | ${'space'}
    ${'foo\nbar'}                    | ${'newline'}
    ${'" onfocus="alert(1)'}         | ${'double quote'}
    ${"' onfocus='alert(1)"}         | ${'single quote'}
    ${'foo<bar'}                     | ${'less than'}
    ${'"><script>alert(1)</script>'} | ${'greater than'}
    ${'foo=bar'}                     | ${'equals sign'}
    ${'foo/bar'}                     | ${'slash'}
    ${'foo\\bar'}                    | ${'backslash'}
    ${'foo`bar'}                     | ${'backtick'}
    ${'a\x00b'}                      | ${'null byte'}
    ${'a\x1fb'}                      | ${'control character'}
    ${'a\x7fb'}                      | ${'DEL character'}
    ${'!'}                           | ${'single bang'}
    ${'!--'}                         | ${'parser-control bang prefix'}
    ${'!DOCTYPE'}                    | ${'doctype-like name'}
    ${'?'}                           | ${'single question mark'}
    ${'?xml'}                        | ${'processing instruction'}
  `('should return false for "$description"', ({ name }) => {
    expect(isValidTagName(name)).toBe(false)
  })

  it('should keep validating names after the valid tag name cache is reset', () => {
    for (let i = 0; i < 257; i++) {
      expect(isValidTagName(`custom-${i}`)).toBe(true)
    }

    expect(isValidTagName('div')).toBe(true)
    expect(isValidTagName('div onmouseover="alert(1)"')).toBe(false)
  })
})

describe('styleObjectForEach', () => {
  describe('Should output the number as it is, when a number type is passed', () => {
    test.each`
      property
      ${'animationIterationCount'}
      ${'aspectRatio'}
      ${'borderImageOutset'}
      ${'borderImageSlice'}
      ${'borderImageWidth'}
      ${'columnCount'}
      ${'columns'}
      ${'flex'}
      ${'flexGrow'}
      ${'flexPositive'}
      ${'flexShrink'}
      ${'flexNegative'}
      ${'flexOrder'}
      ${'gridArea'}
      ${'gridRow'}
      ${'gridRowEnd'}
      ${'gridRowSpan'}
      ${'gridRowStart'}
      ${'gridColumn'}
      ${'gridColumnEnd'}
      ${'gridColumnSpan'}
      ${'gridColumnStart'}
      ${'fontWeight'}
      ${'lineClamp'}
      ${'lineHeight'}
      ${'opacity'}
      ${'order'}
      ${'orphans'}
      ${'scale'}
      ${'tabSize'}
      ${'widows'}
      ${'zIndex'}
      ${'zoom'}
      ${'fillOpacity'}
      ${'floodOpacity'}
      ${'stopOpacity'}
      ${'strokeDasharray'}
      ${'strokeDashoffset'}
      ${'strokeMiterlimit'}
      ${'strokeOpacity'}
      ${'strokeWidth'}
    `('$property', ({ property }) => {
      const fn = vi.fn()
      styleObjectForEach({ [property]: 1 }, fn)
      expect(fn).toBeCalledWith(
        property.replace(/[A-Z]/g, (m: string) => `-${m.toLowerCase()}`),
        '1'
      )
    })
  })
  describe('Should output with px suffix, when a number type is passed', () => {
    test.each`
      property
      ${'borderBottomWidth'}
      ${'borderLeftWidth'}
      ${'borderRightWidth'}
      ${'borderTopWidth'}
      ${'borderWidth'}
      ${'bottom'}
      ${'fontSize'}
      ${'height'}
      ${'left'}
      ${'margin'}
      ${'marginBottom'}
      ${'marginLeft'}
      ${'marginRight'}
      ${'marginTop'}
      ${'padding'}
      ${'paddingBottom'}
      ${'paddingLeft'}
      ${'paddingRight'}
      ${'paddingTop'}
      ${'right'}
      ${'top'}
      ${'width'}
      ${'gridGap'}
      ${'gridRowGap'}
      ${'gridColumnGap'}
    `('$property', ({ property }) => {
      const fn = vi.fn()
      styleObjectForEach({ [property]: 1 }, fn)
      expect(fn).toBeCalledWith(
        property.replace(/[A-Z]/g, (m: string) => `-${m.toLowerCase()}`),
        '1px'
      )
    })
  })

  describe('Should skip values containing ";" to prevent CSS injection', () => {
    it('skips a value with ";"', () => {
      const fn = vi.fn()
      styleObjectForEach({ color: 'red;background:blue' }, fn)
      expect(fn).not.toBeCalled()
    })

    it('keeps safe siblings while skipping unsafe ones', () => {
      const fn = vi.fn()
      styleObjectForEach({ color: 'red;background:blue', backgroundColor: 'white' }, fn)
      expect(fn).toBeCalledTimes(1)
      expect(fn).toBeCalledWith('background-color', 'white')
    })

    it('does not throw for unsafe values', () => {
      expect(() => styleObjectForEach({ color: 'a;b' }, () => {})).not.toThrow()
    })

    it('keeps semicolons inside quoted strings', () => {
      const fn = vi.fn()
      styleObjectForEach({ fontFamily: '"a;b", sans-serif' }, fn)
      expect(fn).toBeCalledWith('font-family', '"a;b", sans-serif')
    })

    test.each([
      ['LF', '\n'],
      ['CR', '\r'],
      ['FF', '\f'],
    ])('skips quoted strings that contain %s before a declaration separator', (_, newline) => {
      const fn = vi.fn()
      styleObjectForEach(
        { fontFamily: `"${newline};background:url(https://example.com/a.png)` },
        fn
      )
      expect(fn).not.toBeCalled()
    })

    it('keeps semicolons inside CSS functions', () => {
      const fn = vi.fn()
      styleObjectForEach({ backgroundImage: 'url("data:image/svg+xml;utf8,<svg></svg>")' }, fn)
      expect(fn).toBeCalledWith('background-image', 'url("data:image/svg+xml;utf8,<svg></svg>")')
    })

    test.each([['square brackets', 'red[;background:blue]']])(
      'keeps semicolons inside CSS simple blocks with %s',
      (_, value) => {
        const fn = vi.fn()
        styleObjectForEach({ color: value }, fn)
        expect(fn).toBeCalledWith('color', value)
      }
    )

    test.each([
      ['opening curly block', 'red{;background:blue}'],
      ['closing curly block', 'red};background:blue'],
    ])('skips CSS values containing %s', (_, value) => {
      const fn = vi.fn()
      styleObjectForEach({ color: value, backgroundColor: 'white' }, fn)
      expect(fn).toBeCalledTimes(1)
      expect(fn).toBeCalledWith('background-color', 'white')
    })

    test.each([
      ['square brackets', 'red[;background:blue];position:fixed'],
      ['curly braces', 'red{;background:blue};position:fixed'],
    ])('skips top-level semicolons after CSS simple blocks with %s', (_, value) => {
      const fn = vi.fn()
      styleObjectForEach({ color: value }, fn)
      expect(fn).not.toBeCalled()
    })

    it('skips top-level semicolons after CSS comments', () => {
      const fn = vi.fn()
      styleObjectForEach({ color: 'red/*(*/;background:blue;position:fixed;top:0' }, fn)
      expect(fn).not.toBeCalled()
    })

    test.each([
      ['comment', 'red/*'],
      ['double-quoted string', '"abc'],
      ['single-quoted string', "'abc"],
      ['function block', 'red('],
      ['square block', 'red['],
      ['curly block', 'red{'],
      ['unmatched function closer', 'red)'],
      ['unmatched square closer', 'red]'],
      ['unmatched curly closer', 'red}'],
      ['mismatched simple block closer', 'red[)'],
      ['escape', 'red\\'],
    ])('skips unterminated CSS %s that can swallow following declarations', (_, value) => {
      const fn = vi.fn()
      styleObjectForEach({ color: value, display: 'none' }, fn)
      expect(fn).toBeCalledTimes(1)
      expect(fn).toBeCalledWith('display', 'none')
    })

    test.each([
      ['comment that closes exactly at end of value', 'red/* hi */'],
      ['CSS hex escape that decodes to a delimiter byte', 'red\\3b '],
      ['vertical tab inside a quoted string', '"\vfoo"'],
      ['trailing solidus that is not a comment start', 'a/'],
    ])('keeps CSS-safe edge case — %s', (_, value) => {
      const fn = vi.fn()
      styleObjectForEach({ color: value }, fn)
      expect(fn).toBeCalledWith('color', value)
    })

    it('skips style property names that can break declaration boundaries', () => {
      const fn = vi.fn()
      styleObjectForEach({ 'color;background-image': 'url(https://example.com/a.png)' }, fn)
      styleObjectForEach({ 'color:background': 'red' }, fn)
      expect(fn).not.toBeCalled()
    })

    it('keeps safe property names that use digits', () => {
      const fn = vi.fn()
      styleObjectForEach({ '--my-var-1': '15px', '--myVar-2': '20px' }, fn)
      expect(fn).toHaveBeenNthCalledWith(1, '--my-var-1', '15px')
      expect(fn).toHaveBeenNthCalledWith(2, '--myVar-2', '20px')
    })

    it('keeps vendor-prefixed property names', () => {
      const fn = vi.fn()
      styleObjectForEach({ WebkitLineClamp: '2' }, fn)
      expect(fn).toBeCalledWith('-webkit-line-clamp', '2')
    })

    it('keeps validating style property names after the valid style property name cache is reset', () => {
      for (let i = 0; i < 1025; i++) {
        styleObjectForEach({ [`--cache-${i}`]: '1px' }, () => {})
      }

      const fn = vi.fn()
      styleObjectForEach({ '--after-reset-1': '2px', 'color:background': 'red' }, fn)
      expect(fn).toBeCalledTimes(1)
      expect(fn).toBeCalledWith('--after-reset-1', '2px')
    })

    it('skips unsupported runtime values without throwing', () => {
      const fn = vi.fn()
      expect(() => styleObjectForEach({ color: false, backgroundColor: 'white' }, fn)).not.toThrow()
      expect(fn).toBeCalledTimes(1)
      expect(fn).toBeCalledWith('background-color', 'white')
    })
  })
})
```
### src/helper/css/common.ts
```typescript
// provide utility functions for css helper both on server and client
export const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
export const isPseudoGlobalSelectorRe = new RegExp(`^${PSEUDO_GLOBAL_SELECTOR}{(.*)}$`)
export const DEFAULT_STYLE_ID = 'hono-css'

export const SELECTOR: unique symbol = Symbol()
export const CLASS_NAME: unique symbol = Symbol()
export const STYLE_STRING: unique symbol = Symbol()
export const SELECTORS: unique symbol = Symbol()
export const EXTERNAL_CLASS_NAMES: unique symbol = Symbol()
const CSS_ESCAPED: unique symbol = Symbol()

export interface CssClassName {
  [SELECTOR]: string
  [CLASS_NAME]: string
  [STYLE_STRING]: string
  [SELECTORS]: CssClassName[]
  [EXTERNAL_CLASS_NAMES]: string[]
}

export const IS_CSS_ESCAPED = Symbol()

interface CssEscapedString {
  [CSS_ESCAPED]: string
}

/**
 * @experimental
 * `rawCssString` is an experimental feature.
 * The API might be changed.
 */
export const rawCssString = (value: string): CssEscapedString => {
  return {
    [CSS_ESCAPED]: value,
  }
}

/**
 * Used the goober'code as a reference:
 * https://github.com/cristianbote/goober/blob/master/src/core/to-hash.js
 * MIT License, Copyright (c) 2019 Cristian Bote
 */
const toHash = (str: string): string => {
  let i = 0,
    out = 11
  while (i < str.length) {
    out = (101 * out + str.charCodeAt(i++)) >>> 0
  }
  return 'css-' + out
}

const normalizeLabel = (label: string): string => {
  return label.trim().replace(/\s+/g, '-')
}

const isValidClassName = (name: string): boolean => /^-?[_a-zA-Z][_a-zA-Z0-9-]*$/.test(name)

// `<`, `{`, `}` never appear in valid class names but can break out of a <style> element
// or inject a CSS rule when an external cx() class name is used as a selector.
const hasUnsafeSelectorChar = (name: string): boolean => /[<{}]/.test(name)

// CSS-wide keywords that are invalid as @keyframes names per the spec
const RESERVED_KEYFRAME_NAMES = new Set([
  'default',
  'inherit',
  'initial',
  'none',
  'revert',
  'revert-layer',
  'unset',
])
const isValidKeyframeName = (name: string): boolean =>
  isValidClassName(name) && !RESERVED_KEYFRAME_NAMES.has(name.toLowerCase())

const defaultOnInvalidSlug = (slug: string) => {
  console.warn(`Invalid slug: ${slug}`)
}

const cssStringReStr: string = [
  '"(?:(?:\\\\[\\s\\S]|[^"\\\\])*)"', // double quoted string

  "'(?:(?:\\\\[\\s\\S]|[^'\\\\])*)'", // single quoted string

  'url\\(\\s*[^)"\'\\s]*\\s*\\)', // unquoted url, which may contain "//"
].join('|')
const minifyCssRe: RegExp = new RegExp(
  [
    '(' + cssStringReStr + ')', // $1: quoted string or unquoted url

    '(?:' +
      [
        '^\\s+', // head whitespace
        '\\/\\*.*?\\*\\/\\s*', // multi-line comment
        '\\/\\/.*\\n\\s*', // single-line comment
        '\\s+$', // tail whitespace
      ].join('|') +
      ')',

    '\\s*;\\s*(}|$)\\s*', // $2: trailing semicolon
    '\\s*([{};,])\\s*', // $3: whitespace around { } , ;
    // $4: whitespace after : (the whitespace before it is a descendant combinator in a selector)
    '(:)\\s*',
    '(\\s)\\s+', // $5: 2+ spaces
  ].join('|'),
  'g'
)

export const minify = (css: string): string => {
  return css.replace(minifyCssRe, (_, $1, $2, $3, $4, $5) => $1 || $2 || $3 || $4 || $5 || '')
}

type CssVariableBasicType =
  | CssClassName
  | CssEscapedString
  | string
  | number
  | boolean
  | null
  | undefined
type CssVariableAsyncType = Promise<CssVariableBasicType>
type CssVariableArrayType = (CssVariableBasicType | CssVariableAsyncType)[]
export type CssVariableType = CssVariableBasicType | CssVariableAsyncType | CssVariableArrayType

/**
 * A function that customizes generated CSS class names.
 *
 * @param hash - The default hash-based class name (e.g. `css-1234567890`)
 * @param label - The comment label extracted from the CSS template, may be empty.
 *   Whitespace is trimmed and inner spaces are replaced with hyphens.
 * @param styleString - The minified CSS style string
 * @returns The custom class name to use. Must be a safe CSS identifier;
 *   otherwise, the default hash is used as a fallback.
 */
export type ClassNameSlug = (hash: string, label: string, styleString: string) => string

/**
 * A callback function called when an invalid slug is returned from ClassNameSlug.
 *
 * @param slug - The invalid slug
 */
export type OnInvalidSlug = (slug: string) => void

export const buildStyleString = (
  strings: TemplateStringsArray,
  values: CssVariableType[]
): [string, string, CssClassName[], string[]] => {
  const selectors: CssClassName[] = []
  const externalClassNames: string[] = []

  const label = strings[0].match(/^\s*\/\*(.*?)\*\//)?.[1] || ''
  let styleString = ''
  for (let i = 0, len = strings.length; i < len; i++) {
    styleString += strings[i]
    let vArray = values[i]
    if (typeof vArray === 'boolean' || vArray === null || vArray === undefined) {
      continue
    }

    if (!Array.isArray(vArray)) {
      vArray = [vArray]
    }
    for (let j = 0, len = vArray.length; j < len; j++) {
      let value = vArray[j]
      if (typeof value === 'boolean' || value === null || value === undefined) {
        continue
      }
      if (typeof value === 'string') {
        if (/([\\"'\/])/.test(value)) {
          styleString += value.replace(/([\\"']|(?<=<)\/)/g, '\\$1')
        } else {
          styleString += value
        }
      } else if (typeof value === 'number') {
        styleString += value
      } else if ((value as CssEscapedString)[CSS_ESCAPED]) {
        styleString += (value as CssEscapedString)[CSS_ESCAPED]
      } else if ((value as CssClassName)[CLASS_NAME].startsWith('@keyframes ')) {
        selectors.push(value as CssClassName)
        styleString += ` ${(value as CssClassName)[CLASS_NAME].substring(11)} `
      } else {
        if (strings[i + 1]?.match(/^\s*{/)) {
          // assume this value is a class name
          selectors.push(value as CssClassName)
          value = `.${(value as CssClassName)[CLASS_NAME]}`
        } else {
          selectors.push(...(value as CssClassName)[SELECTORS])
          externalClassNames.push(...(value as CssClassName)[EXTERNAL_CLASS_NAMES])
          value = (value as CssClassName)[STYLE_STRING]
          const valueLen = value.length
          if (valueLen > 0) {
            const lastChar = value[valueLen - 1]
            if (lastChar !== ';' && lastChar !== '}') {
              value += ';'
            }
          }
        }
        styleString += `${value || ''}`
      }
    }
  }

  return [label, minify(styleString), selectors, externalClassNames]
}

export const cssCommon = (
  strings: TemplateStringsArray,
  values: CssVariableType[],
  classNameSlug?: ClassNameSlug,
  onInvalidSlug?: OnInvalidSlug
): CssClassName => {
  let [label, thisStyleString, selectors, externalClassNames] = buildStyleString(strings, values)
  const isPseudoGlobal = isPseudoGlobalSelectorRe.exec(thisStyleString)
  if (isPseudoGlobal) {
    thisStyleString = isPseudoGlobal[1]
  }
  const hash = toHash(label + thisStyleString)

  let customSlug: string | undefined
  if (classNameSlug) {
    const slug = classNameSlug(hash, normalizeLabel(label), thisStyleString)
    if (slug) {
      if (isValidClassName(slug)) {
        customSlug = slug
      } else {
        ;(onInvalidSlug || defaultOnInvalidSlug)(slug)
      }
    }
  }

  const selector = (isPseudoGlobal ? PSEUDO_GLOBAL_SELECTOR : '') + (customSlug || hash)
  const className = (
    isPseudoGlobal ? selectors.map((s) => s[CLASS_NAME]) : [selector, ...externalClassNames]
  ).join(' ')

  return {
    [SELECTOR]: selector,
    [CLASS_NAME]: className,
    [STYLE_STRING]: thisStyleString,
    [SELECTORS]: selectors,
    [EXTERNAL_CLASS_NAMES]: externalClassNames,
  }
}

export const cxCommon = (
  args: (string | boolean | null | undefined | CssClassName)[]
): (string | boolean | null | undefined | CssClassName)[] => {
  for (let i = 0, len = args.length; i < len; i++) {
    const arg = args[i]
    if (typeof arg === 'string') {
      args[i] = {
        [SELECTOR]: '',
        [CLASS_NAME]: '',
        [STYLE_STRING]: '',
        [SELECTORS]: [],
        // drop names that are unsafe to emit as a selector
        [EXTERNAL_CLASS_NAMES]: hasUnsafeSelectorChar(arg) ? [] : [arg],
      }
    }
  }

  return args
}

export const keyframesCommon = (
  strings: TemplateStringsArray,
  values: CssVariableType[],
  classNameSlug?: ClassNameSlug,
  onInvalidSlug?: OnInvalidSlug
): CssClassName => {
  const [label, styleString] = buildStyleString(strings, values)
  const hash = toHash(label + styleString)

  let customSlug: string | undefined
  if (classNameSlug) {
    const slug = classNameSlug(hash, normalizeLabel(label), styleString)
    if (slug) {
      if (isValidKeyframeName(slug)) {
        customSlug = slug
      } else {
        ;(onInvalidSlug || defaultOnInvalidSlug)(slug)
      }
    }
  }

  return {
    [SELECTOR]: '',
    [CLASS_NAME]: `@keyframes ${customSlug || hash}`,
    [STYLE_STRING]: styleString,
    [SELECTORS]: [],
    [EXTERNAL_CLASS_NAMES]: [],
  }
}

type ViewTransitionType = {
  (
    strings: TemplateStringsArray,
    values: CssVariableType[],
    classNameSlug?: ClassNameSlug,
    onInvalidSlug?: OnInvalidSlug
  ): CssClassName
  (content: CssClassName): CssClassName
  (): CssClassName
}

let viewTransitionNameIndex = 0
export const viewTransitionCommon: ViewTransitionType = ((
  strings: TemplateStringsArray | CssClassName | undefined,
  values: CssVariableType[],
  classNameSlug?: ClassNameSlug,
  onInvalidSlug?: OnInvalidSlug
): CssClassName => {
  if (!strings) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    strings = [`/* h-v-t ${viewTransitionNameIndex++} */`] as any
  }
  const content = Array.isArray(strings)
    ? cssCommon(strings as TemplateStringsArray, values, classNameSlug, onInvalidSlug)
    : (strings as CssClassName)

  const transitionName = content[CLASS_NAME]
  const res = cssCommon(
    ['view-transition-name:', ''] as any, // eslint-disable-line @typescript-eslint/no-explicit-any
    [transitionName],
    classNameSlug,
    onInvalidSlug
  )

  content[CLASS_NAME] = PSEUDO_GLOBAL_SELECTOR + content[CLASS_NAME]
  content[STYLE_STRING] = content[STYLE_STRING].replace(
    /(?<=::view-transition(?:[a-z-]*)\()(?=\))/g,
    transitionName
  )
  res[CLASS_NAME] = res[SELECTOR] = transitionName
  res[SELECTORS] = [...content[SELECTORS], content]

  return res
}) as ViewTransitionType
```
### src/helper/css/index.test.tsx
```typescript
/** @jsxImportSource ../../jsx */
import { Hono } from '../../'
import { html } from '../../helper/html'
import type { JSXNode } from '../../jsx'
import { isValidElement } from '../../jsx'
import { Suspense, renderToReadableStream } from '../../jsx/streaming'
import type { HtmlEscapedString } from '../../utils/html'
import { HtmlEscapedCallbackPhase, resolveCallback } from '../../utils/html'
import { renderTest } from './common.case.test'
import { Style, createCssContext, css, cx, keyframes, rawCssString, viewTransition } from './index'

async function toString(
