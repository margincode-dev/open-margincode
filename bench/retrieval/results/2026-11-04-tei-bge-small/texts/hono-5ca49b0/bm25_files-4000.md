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
