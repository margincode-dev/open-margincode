- createCssContext (src/helper/css/index.ts:72-215)
```typescript
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
```
- buildStyleString (src/helper/css/common.ts:143-203)
```typescript
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
```
- renderTest (src/helper/css/common.case.test.tsx:13-506)
```typescript
export const renderTest = (
  getEnv: () => {
    css: typeof cssHelper
    keyframes: typeof keyframesHelper
    viewTransition: typeof viewTransitionHelper
    rawCssString: typeof rawCssStringHelper
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    toString: (template: any) => Promise<string>
    Style: typeof StyleComponent
    support: Support
  }
) => {
  const { support } = getEnv()

  let css: typeof cssHelper
  let keyframes: typeof keyframesHelper
  let viewTransition: typeof viewTransitionHelper
  let rawCssString: typeof rawCssStringHelper
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let toString: (template: any) => Promise<string>
  let Style: typeof StyleComponent
  beforeEach(() => {
    ;({ css, keyframes, viewTransition, rawCssString, toString, Style } = getEnv())
  })

  describe('render css', () => {
    it('Should render CSS styles with JSX', async () => {
      const headerClass = css`
        background-color: blue;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-2458908649{background-color:blue}</style><h1 class="css-2458908649">Hello!</h1>'
      )
    })

    it('Should render CSS with keyframes', async () => {
      const animation = keyframes`
        from {
          opacity: 0;
        }
        to {
          opacity: 1;
        }
      `
      const headerClass = css`
        background-color: blue;
        animation: ${animation} 1s ease-in-out;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-1580801783{background-color:blue;animation:css-9294673 1s ease-in-out}@keyframes css-9294673{from{opacity:0}to{opacity:1}}</style><h1 class="css-1580801783">Hello!</h1>'
      )
    })

    it('Should not output the same class name multiple times.', async () => {
      const headerClass = css`
        background-color: blue;
      `
      const headerClass2 = css`
        background-color: blue;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
          <h1 class={headerClass2}>Hello2!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-2458908649{background-color:blue}</style><h1 class="css-2458908649">Hello!</h1><h1 class="css-2458908649">Hello2!</h1>'
      )
    })

    it('Should render CSS with variable', async () => {
      const headerClass = css`
        background-color: blue;
        content: '${"I'm a variable!"}';
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-4027435072{background-color:blue;content:\'I\\\'m a variable!\'}</style><h1 class="css-4027435072">Hello!</h1>'
      )
    })

    it('Should escape </style>', async () => {
      const headerClass = css`
        background-color: blue;
        content: '${'</style>'}';
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-372954897{background-color:blue;content:\'<\\/style>\'}</style><h1 class="css-372954897">Hello!</h1>'
      )
    })

    it('Should not escape URL', async () => {
      const headerClass = css`
        background-color: blue;
        background: url('${'http://www.example.com/path/to/file.jpg'}');
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-1321888780{background-color:blue;background:url(\'http://www.example.com/path/to/file.jpg\')}</style><h1 class="css-1321888780">Hello!</h1>'
      )
    })

    it('Should render CSS with escaped variable', async () => {
      const headerClass = css`
        background-color: blue;
        content: '${rawCssString('say "Hello!"')}';
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-2238574885{background-color:blue;content:\'say "Hello!"\'}</style><h1 class="css-2238574885">Hello!</h1>'
      )
    })

    it('Should render CSS with number', async () => {
      const headerClass = css`
        background-color: blue;
        font-size: ${1}rem;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-1847536026{background-color:blue;font-size:1rem}</style><h1 class="css-1847536026">Hello!</h1>'
      )
    })

    it('Should render CSS with array', async () => {
      const animation = keyframes`
        from {
          opacity: 0;
        }
        to {
          opacity: 1;
        }
      `
      const headerClass = css`
        background-color: blue;
        animation: ${animation} 1s ease-in-out;
      `
      const extendedHeaderClass = css`
        ${headerClass}
        color: red;
      `
      const template = (
        <>
          <Style />
          <h1 class={extendedHeaderClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-2558359670{background-color:blue;animation:css-9294673 1s ease-in-out;color:red}@keyframes css-9294673{from{opacity:0}to{opacity:1}}</style><h1 class="css-2558359670">Hello!</h1>'
      )
    })

    it.runIf(support.nest)(
      'Should be used as a class name for syntax `${className} {`',
      async () => {
        const headerClass = css`
          font-weight: bold;
        `
        const containerClass = css`
          ${headerClass} {
            h1 {
              color: red;
            }
          }
        `
        const template = (
          <>
            <Style />
            <div class={containerClass}>
              <h1 class={headerClass}>Hello!</h1>
            </div>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css">.css-4220297002{.css-1032195302{h1{color:red}}}.css-1032195302{font-weight:bold}</style><div class="css-4220297002"><h1 class="css-1032195302">Hello!</h1></div>'
        )
      }
    )

    it('Should be inserted to global if style string starts with :-hono-root', async () => {
      const globalClass = css`
        :-hono-global {
          html {
            color: red;
          }
          body {
            display: flex;
          }
        }
      `
      const template = (
        <>
          <Style />
          <div class={globalClass}>
            <h1>Hello!</h1>
          </div>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">html{color:red}body{display:flex}</style><div class=""><h1>Hello!</h1></div>'
      )
    })

    it.runIf(support.nest)(
      'Should be inserted to global if style string starts with :-hono-root and extends class name',
      async () => {
        const headerClass = css`
          display: flex;
        `
        const specialHeaderClass = css`
          :-hono-global {
            ${headerClass} {
              h1 {
                color: red;
              }
            }
          }
        `
        const template = (
          <>
            <Style />
            <div class={specialHeaderClass}>
              <h1>Hello!</h1>
            </div>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css">.css-3980466870{h1{color:red}}.css-3980466870{display:flex}</style><div class="css-3980466870"><h1>Hello!</h1></div>'
        )
      }
    )

    it('Should be inserted as global css if passed css`` to Style component', async () => {
      const headerClass = css`
        font-size: 1rem;
      `
      const template = (
        <>
          <Style>{css`
            html {
              color: red;
            }
            body {
              display: flex;
            }
          `}</Style>
          <div>
            <h1 class={headerClass}>Hello!</h1>
          </div>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">html{color:red}body{display:flex}.css-1740067317{font-size:1rem}</style><div><h1 class="css-1740067317">Hello!</h1></div>'
      )
    })

    it('Should be ignored :-hono-root inside Style component', async () => {
      const headerClass = css`
        font-size: 1rem;
      `
      const template = (
        <>
          <Style>{css`
            :-hono-global {
              html {
                color: red;
              }
              body {
                display: flex;
              }
            }
          `}</Style>
          <div>
            <h1 class={headerClass}>Hello!</h1>
          </div>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">html{color:red}body{display:flex}.css-1740067317{font-size:1rem}</style><div><h1 class="css-1740067317">Hello!</h1></div>'
      )
    })

    describe('viewTransition', () => {
      it('Should render CSS with unique view-transition-name', async () => {
        const transition = viewTransition()
        const template = (
          <>
            <Style />
            <h1 class={transition}>Hello!</h1>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css">.css-1644952339{view-transition-name:css-1644952339}</style><h1 class="css-1644952339">Hello!</h1>'
        )
      })

      it('Should render CSS with css and keyframes', async () => {
        const kf = keyframes`
        from {
          opacity: 0;
        }
        to {
          opacity: 1;
        }
      `
        const transition = viewTransition(css`
          ::view-transition-old() {
            animation-name: ${kf};
          }
          ::view-transition-new() {
            animation-name: ${kf};
          }
        `)
        const headerClass = css`
          ${transition}
          background-color: blue;
        `
        const template = (
          <>
            <Style />
            <h1 class={headerClass}>Hello!</h1>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css">.css-1245070278{view-transition-name:css-399742870;background-color:blue}@keyframes css-9294673{from{opacity:0}to{opacity:1}}::view-transition-old(css-399742870){animation-name:css-9294673}::view-transition-new(css-399742870){animation-name:css-9294673}</style><h1 class="css-1245070278">Hello!</h1>'
        )
      })

      it('Should works as a template tag function', async () => {
        const kf = keyframes`
        from {
          opacity: 0;
        }
        to {
          opacity: 1;
        }
      `
        const transition = viewTransition`
          ::view-transition-old() {
            animation-name: ${kf};
          }
          ::view-transition-new() {
            animation-name: ${kf};
          }
        `
        const headerClass = css`
          ${transition}
          background-color: blue;
        `
        const template = (
          <>
            <Style />
            <h1 class={headerClass}>Hello!</h1>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css">.css-1245070278{view-transition-name:css-399742870;background-color:blue}@keyframes css-9294673{from{opacity:0}to{opacity:1}}::view-transition-old(css-399742870){animation-name:css-9294673}::view-transition-new(css-399742870){animation-name:css-9294673}</style><h1 class="css-1245070278">Hello!</h1>'
        )
      })
    })

    it.runIf(support.nest)('Should render sub CSS with keyframe', async () => {
      const headerClass = css`
        background-color: blue;
        ${[1, 2].map(
          (i) => css`
            :nth-child(${i}) {
              color: red;
            }
          `
        )}
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-1539881271{background-color:blue;:nth-child(1){color:red}:nth-child(2){color:red}}</style><h1 class="css-1539881271">Hello!</h1>'
      )
    })

    it('Should be generated deferent class name for deferent first line comment even if the content is the same', async () => {
      const headerClassA = css`
        /* class A */
        display: flex;
      `
      const headerClassB = css`
        /* class B */
        display: flex;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClassA}>Hello!</h1>
          <h1 class={headerClassB}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-3170754153{display:flex}.css-896513246{display:flex}</style><h1 class="css-3170754153">Hello!</h1><h1 class="css-896513246">Hello!</h1>'
      )
    })

    describe('Booleans, Null, and Undefined Are Ignored', () => {
      it.each([true, false, undefined, null])('%s', async (value) => {
        const headerClass = css`
          ${value}
          background-color: blue;
        `
        const template = (
          <>
            <Style />
            <h1 class={headerClass}>Hello!</h1>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css">.css-2458908649{background-color:blue}</style><h1 class="css-2458908649">Hello!</h1>'
        )
      })

      it('falsy value', async () => {
        const value = 0
        const headerClass = css`
          padding: ${value};
        `
        const template = (
          <>
            <Style />
            <h1 class={headerClass}>Hello!</h1>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css">.css-478287868{padding:0}</style><h1 class="css-478287868">Hello!</h1>'
        )
      })

      it('Should render CSS styles with CSP nonce', async () => {
        const headerClass = css`
          background-color: blue;
        `
        const template = (
          <>
            <Style nonce='1234' />
            <h1 class={headerClass}>Hello!</h1>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css" nonce="1234">.css-2458908649{background-color:blue}</style><h1 class="css-2458908649">Hello!</h1>'
        )
      })
    })
  })
}
```
- createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
```typescript
export const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) => {
  let styleSheet: CSSStyleSheet | null | undefined = undefined
  const findStyleSheet = (): [CSSStyleSheet, Set<string>] | [] => {
    if (!styleSheet) {
      styleSheet = document.querySelector<HTMLStyleElement>(`style#${id}`)
        ?.sheet as CSSStyleSheet | null
      if (styleSheet) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        ;(styleSheet as any).addedStyles = new Set<string>()
      }
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    return styleSheet ? [styleSheet, (styleSheet as any).addedStyles] : []
  }

  const insertRule = (className: string, styleString: string) => {
    const [sheet, addedStyles] = findStyleSheet()
    if (!sheet || !addedStyles) {
      Promise.resolve().then(() => {
        if (!findStyleSheet()[0]) {
          throw new Error('style sheet not found')
        }
        insertRule(className, styleString)
      })
      return
    }

    if (!addedStyles.has(className)) {
      addedStyles.add(className)
      ;(className.startsWith(PSEUDO_GLOBAL_SELECTOR)
        ? splitRule(styleString)
        : [`${className[0] === '@' ? '' : '.'}${className}{${styleString}}`]
      ).forEach((rule) => {
        sheet.insertRule(rule, sheet.cssRules.length)
      })
    }
  }

  const cssObject = {
    toString(this: CssClassName): string {
      const selector = this[SELECTOR]
      insertRule(selector, this[STYLE_STRING])
      this[SELECTORS].forEach(({ [CLASS_NAME]: className, [STYLE_STRING]: styleString }) => {
        insertRule(className, styleString)
      })

      return this[CLASS_NAME]
    },
  }

  const Style: FC<PropsWithChildren<{ nonce?: string }>> = ({ children, nonce }) =>
    ({
      tag: 'style',
      props: {
        id,
        nonce,
        children:
          children &&
          (Array.isArray(children) ? children : [children]).map(
            (c) => (c as unknown as CssClassName)[STYLE_STRING]
          ),
      },
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    }) as any

  return [cssObject, Style] as const
}
```
- DEFAULT_STYLE_ID (src/helper/css/common.ts:4-4)
```typescript
export const DEFAULT_STYLE_ID = 'hono-css'
```
- Trie.buildRegExp (src/router/reg-exp-router/trie.ts:59-83)
```typescript
  buildRegExp(): [RegExp, ReplacementMap, ReplacementMap] {
    let regexp = this.#root.buildRegExpStr()
    if (regexp === '') {
      return [/^$/, [], []] // never match
    }

    let captureIndex = 0
    const indexReplacementMap: ReplacementMap = []
    const paramReplacementMap: ReplacementMap = []

    regexp = regexp.replace(/#(\d+)|@(\d+)|\.\*\$/g, (_, handlerIndex, paramIndex) => {
      if (handlerIndex !== undefined) {
        indexReplacementMap[++captureIndex] = Number(handlerIndex)
        return '$()'
      }
      if (paramIndex !== undefined) {
        paramReplacementMap[Number(paramIndex)] = ++captureIndex
        return ''
      }

      return ''
    })

    return [new RegExp(`^${regexp}`), indexReplacementMap, paramReplacementMap]
  }
```
- styleObjectForEach (src/jsx/utils.ts:169-202)
```typescript
export const styleObjectForEach = (
  style: Record<string, unknown>,
  fn: (key: string, value: string | null) => void
): void => {
  for (const [k, v] of Object.entries(style)) {
    const key =
      k[0] === '-' || !/[A-Z]/.test(k)
        ? k // a CSS variable or a lowercase only property
        : k.replace(/[A-Z]/g, (m) => `-${m.toLowerCase()}`) // a camelCase property. convert to kebab-case
    if (!isValidStylePropertyName(key)) {
      continue
    }
    if (v == null) {
      fn(key, null)
      continue
    }
    let value: string
    if (typeof v === 'number') {
      value = !key.match(
        /^(?:a|border-im|column(?:-c|s)|flex(?:$|-[^b])|grid-(?!.*gap)(?:ar|[^a])|font-w|li|or|sca|st|ta|wido|z)|ty$/
      )
        ? `${v}px`
        : `${v}`
    } else if (typeof v === 'string') {
      if (hasUnsafeStyleValue(v)) {
        continue
      }
      value = v
    } else {
      continue
    }
    fn(key, value)
  }
}
```
- DefaultContextType (src/helper/css/index.ts:217-223)
```typescript
interface DefaultContextType {
  css: CssType
  cx: CxType
  keyframes: KeyframesType
  viewTransition: ViewTransitionType
  Style: StyleType
}
```
- handleNonDot (src/helper/ssg/utils.ts:35-40)
```typescript
const handleNonDot = (path: string, resultPaths: string[]): void => {
  path = path.replace(/^\.(?!.)/, '')
  if (path !== '') {
    resultPaths.push(path)
  }
}
```
- STYLE_STRING (src/helper/css/common.ts:8-8)
```typescript
export const STYLE_STRING: unique symbol = Symbol()
```
- Trie (src/router/reg-exp-router/trie.ts:7-84)
```typescript
export class Trie {
  #context: Context = { varIndex: 0 }
  #root: Node = new Node()
  #index: number = 0
  // dynamic path -> [handler index, param assoc]; static paths are not registered
  paths: Record<string, [number, ParamAssocArray]> = createNullObject()

  insert(path: string, isStatic: boolean): void {
    if (isStatic) {
      // a static path has no pattern; every character is a literal token
      this.#root.insert(path.split(''), 0, [], this.#context, true)
      return
    }

    const paramAssoc: ParamAssocArray = []

    const groups: [string, string][] = [] // [mark, original string]
    let markedPath = path
    for (let i = 0; ;) {
      let replaced = false
      markedPath = markedPath.replace(/\{[^}]+\}/g, (m) => {
        const mark = `@\\${i}`
        groups[i] = [mark, m]
        i++
        replaced = true
        return mark
      })
      if (!replaced) {
        break
      }
    }

    /**
     *  - pattern (:label, :label{0-9]+}, ...)
     *  - /* wildcard
     *  - character
     */
    const tokens = markedPath.match(/(?::[^\/]+)|(?:\/\*$)|./g) || []
    for (let i = groups.length - 1; i >= 0; i--) {
      const [mark] = groups[i]
      for (let j = tokens.length - 1; j >= 0; j--) {
        if (tokens[j].indexOf(mark) !== -1) {
          tokens[j] = tokens[j].replace(mark, groups[i][1])
          break
        }
      }
    }

    this.#root.insert(tokens, this.#index, paramAssoc, this.#context, false)
    this.paths[path] = [this.#index++, paramAssoc]
  }

  buildRegExp(): [RegExp, ReplacementMap, ReplacementMap] {
    let regexp = this.#root.buildRegExpStr()
    if (regexp === '') {
      return [/^$/, [], []] // never match
    }

    let captureIndex = 0
    const indexReplacementMap: ReplacementMap = []
    const paramReplacementMap: ReplacementMap = []

    regexp = regexp.replace(/#(\d+)|@(\d+)|\.\*\$/g, (_, handlerIndex, paramIndex) => {
      if (handlerIndex !== undefined) {
        indexReplacementMap[++captureIndex] = Number(handlerIndex)
        return '$()'
      }
      if (paramIndex !== undefined) {
        paramReplacementMap[Number(paramIndex)] = ++captureIndex
        return ''
      }

      return ''
    })

    return [new RegExp(`^${regexp}`), indexReplacementMap, paramReplacementMap]
  }
}
```
- StyleType (src/helper/css/index.ts:59-61)
```typescript
interface StyleType {
  (args?: { children?: Promise<string>; nonce?: string }): HtmlEscapedString
}
```
- CssClassName (src/helper/css/common.ts:13-19)
```typescript
export interface CssClassName {
  [SELECTOR]: string
  [CLASS_NAME]: string
  [STYLE_STRING]: string
  [SELECTORS]: CssClassName[]
  [EXTERNAL_CLASS_NAMES]: string[]
}
```
- cssCommon (src/helper/css/common.ts:205-242)
```typescript
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
```
- keyframesCommon (src/helper/css/common.ts:264-292)
```typescript
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
