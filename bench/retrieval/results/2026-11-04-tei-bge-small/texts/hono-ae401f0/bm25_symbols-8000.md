- PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
```typescript
export const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
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
- cssCommon (src/helper/css/common.ts:203-240)
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
- splitRule (src/jsx/dom/css.ts:27-66)
```typescript
const splitRule = (rule: string): string[] => {
  const result: string[] = []
  let startPos = 0
  let depth = 0
  for (let i = 0, len = rule.length; i < len; i++) {
    const char = rule[i]

    // consume quote

    if (char === "'" || char === '"') {
      const quote = char
      i++
      for (; i < len; i++) {
        if (rule[i] === '\\') {
          i++
          continue
        }
        if (rule[i] === quote) {
          break
        }
      }
      continue
    }

    // comments are removed from the rule in advance
    if (char === '{') {
      depth++
      continue
    }
    if (char === '}') {
      depth--
      if (depth === 0) {
        result.push(rule.slice(startPos, i + 1))
        startPos = i + 1
      }
      continue
    }
  }
  return result
}
```
- colorStatus (src/middleware/logger/index.ts:28-46)
```typescript
const colorStatus = async (status: number) => {
  const colorEnabled = await getColorEnabledAsync()
  if (colorEnabled) {
    switch ((status / 100) | 0) {
      case 5: // red = error
        return `\x1b[31m${status}\x1b[0m`
      case 4: // yellow = warning
        return `\x1b[33m${status}\x1b[0m`
      case 3: // cyan = redirect
        return `\x1b[36m${status}\x1b[0m`
      case 2: // green = success
        return `\x1b[32m${status}\x1b[0m`
    }
  }
  // Fallback to unsupported status code.
  // E.g.) Bun and Deno supports new Response with 101, but Node.js does not.
  // And those may evolve to accept more status.
  return `${status}`
}
```
- SSGPlugin (src/helper/ssg/ssg.ts:175-179)
```typescript
export interface SSGPlugin {
  beforeRequestHook?: BeforeRequestHook | BeforeRequestHook[]
  afterResponseHook?: AfterResponseHook | AfterResponseHook[]
  afterGenerateHook?: AfterGenerateHook | AfterGenerateHook[]
}
```
- applyNodeObject (src/jsx/dom/render.ts:389-483)
```typescript
const applyNodeObject = (node: NodeObject, container: Container, isNew: boolean): void => {
  const next: Node[] = []
  const remove: Node[] = []
  const callbacks: EffectData[] = []
  getNextChildren(node, container, next, remove, callbacks)
  remove.forEach(removeNode)

  const childNodes = (isNew ? undefined : container.childNodes) as NodeListOf<ChildNode>
  let offset: number
  let insertBeforeNode: ChildNode | null = null
  if (isNew) {
    offset = -1
  } else if (!childNodes.length) {
    offset = 0
  } else {
    const offsetByNextNode = findChildNodeIndex(childNodes, findInsertBefore(node.nN))
    if (offsetByNextNode !== undefined) {
      insertBeforeNode = childNodes[offsetByNextNode]
      offset = offsetByNextNode
    } else {
      offset =
        findChildNodeIndex(childNodes, next.find((n) => n.tag !== HONO_PORTAL_ELEMENT && n.e)?.e) ??
        -1
    }

    if (offset === -1) {
      isNew = true
    }
  }

  for (let i = 0, len = next.length; i < len; i++, offset++) {
    const child = next[i]

    let el: SupportedElement | Text
    if (child.s && child.e) {
      el = child.e
      child.s = false
    } else {
      const isNewLocal = isNew || !child.e
      if (isNodeString(child)) {
        if (child.e && child.d) {
          child.e.textContent = child.t
        }
        child.d = false
        el = child.e ||= document.createTextNode(child.t)
      } else {
        el = child.e ||= child.n
          ? (document.createElementNS(child.n, child.tag as string) as SVGElement | MathMLElement)
          : document.createElement(child.tag as string)

        applyProps(el as HTMLElement, child.props, child.pP)
        applyNodeObject(child, el as HTMLElement, isNewLocal)
        if (child.tag === 'select') {
          applySelectValue(el as HTMLSelectElement, child.props)
        }
      }
    }
    if (child.tag === HONO_PORTAL_ELEMENT) {
      offset--
    } else if (isNew) {
      if (!el.parentNode) {
        container.appendChild(el)
      }
    } else if (childNodes[offset] !== el && childNodes[offset - 1] !== el) {
      if (childNodes[offset + 1] === el) {
        // Move extra elements to the back of the container. This is to be done efficiently when elements are swapped.
        container.appendChild(childNodes[offset])
      } else {
        container.insertBefore(el, insertBeforeNode || childNodes[offset] || null)
      }
    }
  }
  if (node.pP) {
    node.pP = undefined
  }
  if (callbacks.length) {
    const useLayoutEffectCbs: Array<() => void> = []
    const useEffectCbs: Array<() => void> = []
    callbacks.forEach(([, useLayoutEffectCb, , useEffectCb, useInsertionEffectCb]) => {
      if (useLayoutEffectCb) {
        useLayoutEffectCbs.push(useLayoutEffectCb)
      }
      if (useEffectCb) {
        useEffectCbs.push(useEffectCb)
      }
      useInsertionEffectCb?.() // invoke useInsertionEffect callbacks
    })
    useLayoutEffectCbs.forEach((cb) => cb()) // invoke useLayoutEffect callbacks
    if (useEffectCbs.length) {
      requestAnimationFrame(() => {
        useEffectCbs.forEach((cb) => cb()) // invoke useEffect callbacks
      })
    }
  }
}
```
- title (src/jsx/intrinsic-element/components.ts:121-135)
```typescript
export const title: FC<PropsWithChildren> = ({ children, ...props }) => {
  const nameSpaceContext = getNameSpaceContext()
  if (nameSpaceContext) {
    const context = useContext(nameSpaceContext)
    if (context === 'svg' || context === 'head') {
      return new JSXNode(
        'title',
        props,
        toArray(children ?? []) as Child[]
      ) as unknown as HtmlEscapedString
    }
  }

  return documentMetadataTag('title', children, props, false)
}
```
- ToSSGOptions (src/helper/ssg/ssg.ts:181-198)
```typescript
export interface ToSSGOptions {
  dir?: string
  /**
   * @deprecated Use plugins[].beforeRequestHook instead.
   */
  beforeRequestHook?: BeforeRequestHook | BeforeRequestHook[]
  /**
   * @deprecated Use plugins[].afterResponseHook instead.
   */
  afterResponseHook?: AfterResponseHook | AfterResponseHook[]
  /**
   * @deprecated Use plugins[].afterGenerateHook instead.
   */
  afterGenerateHook?: AfterGenerateHook | AfterGenerateHook[]
  concurrency?: number
  extensionMap?: Record<string, string>
  plugins?: SSGPlugin[]
}
```
- toSSG (src/helper/ssg/ssg.ts:369-471)
```typescript
export const toSSG: ToSSGInterface = async (app, fs, options) => {
  let result: ToSSGResult | undefined
  const getInfoPromises: Promise<unknown>[] = []
  const savePromises: Promise<string | undefined>[] = []
  const plugins = options?.plugins || [defaultPlugin()]
  const beforeRequestHooks: BeforeRequestHook[] = []
  const afterResponseHooks: AfterResponseHook[] = []
  const afterGenerateHooks: AfterGenerateHook[] = []
  if (options?.beforeRequestHook) {
    beforeRequestHooks.push(
      ...(Array.isArray(options.beforeRequestHook)
        ? options.beforeRequestHook
        : [options.beforeRequestHook])
    )
  }
  if (options?.afterResponseHook) {
    afterResponseHooks.push(
      ...(Array.isArray(options.afterResponseHook)
        ? options.afterResponseHook
        : [options.afterResponseHook])
    )
  }
  if (options?.afterGenerateHook) {
    afterGenerateHooks.push(
      ...(Array.isArray(options.afterGenerateHook)
        ? options.afterGenerateHook
        : [options.afterGenerateHook])
    )
  }
  for (const plugin of plugins) {
    if (plugin.beforeRequestHook) {
      beforeRequestHooks.push(
        ...(Array.isArray(plugin.beforeRequestHook)
          ? plugin.beforeRequestHook
          : [plugin.beforeRequestHook])
      )
    }
    if (plugin.afterResponseHook) {
      afterResponseHooks.push(
        ...(Array.isArray(plugin.afterResponseHook)
          ? plugin.afterResponseHook
          : [plugin.afterResponseHook])
      )
    }
    if (plugin.afterGenerateHook) {
      afterGenerateHooks.push(
        ...(Array.isArray(plugin.afterGenerateHook)
          ? plugin.afterGenerateHook
          : [plugin.afterGenerateHook])
      )
    }
  }
  try {
    const outputDir = options?.dir ?? DEFAULT_OUTPUT_DIR
    const concurrency = options?.concurrency ?? DEFAULT_CONCURRENCY

    const combinedBeforeRequestHook = combineBeforeRequestHooks(
      beforeRequestHooks.length > 0 ? beforeRequestHooks : [(req) => req]
    )
    const combinedAfterResponseHook = combineAfterResponseHooks(
      afterResponseHooks.length > 0 ? afterResponseHooks : [(req) => req]
    )
    const getInfoGen = fetchRoutesContent(
      app,
      combinedBeforeRequestHook,
      combinedAfterResponseHook,
      concurrency
    )
    for (const getInfo of getInfoGen) {
      getInfoPromises.push(
        getInfo.then((getContentGen) => {
          if (!getContentGen) {
            return
          }
          for (const content of getContentGen) {
            savePromises.push(
              saveContentToFile(content, fs, outputDir, options?.extensionMap).catch((e) => e)
            )
          }
        })
      )
    }
    await Promise.all(getInfoPromises)
    const files: string[] = []
    for (const savePromise of savePromises) {
      const fileOrError = await savePromise
      if (typeof fileOrError === 'string') {
        files.push(fileOrError)
      } else if (fileOrError) {
        throw fileOrError
      }
    }
    result = { success: true, files }
  } catch (error) {
    const errorObj = error instanceof Error ? error : new Error(String(error))
    result = { success: false, files: [], error: errorObj }
  }
  if (afterGenerateHooks.length > 0) {
    const combinedAfterGenerateHooks = combineAfterGenerateHooks(afterGenerateHooks, fs, options)
    await combinedAfterGenerateHooks(result, fs, options)
  }
  return result
}
```
- documentMetadataTag (src/jsx/dom/intrinsic-element/components.ts:58-256)
```typescript
const documentMetadataTag = (
  tag: string,
  props: Props,
  preserveNodeType: PreserveNodeType | undefined,
  supportSort: boolean,
  supportBlocking: boolean
) => {
  if (props?.itemProp) {
    return {
      tag,
      props,
      type: tag,
      ref: props.ref,
    }
  }

  const head = document.head

  let { onLoad, onError, precedence, blocking, ...restProps } = props
  let element: HTMLElement | null = null
  let created = false

  const deDupeKeys = deDupeKeyMap[tag]
  const deDupeByKey = shouldDeDupeByKey(tag, supportSort)
  const isDeDupeCandidateLink = (e: HTMLElement) =>
    e.getAttribute('rel') === 'stylesheet' && e.getAttribute(dataPrecedenceAttr) !== null
  let existingElements: NodeListOf<HTMLElement> | undefined = undefined
  if (deDupeByKey) {
    const tags = head.querySelectorAll<HTMLElement>(tag)
    LOOP: for (const e of tags) {
      if (tag === 'link' && !isDeDupeCandidateLink(e)) {
        continue
      }
      for (const key of deDupeKeys) {
        if (e.getAttribute(key) === props[key]) {
          element = e
          break LOOP
        }
      }
    }

    if (!element) {
      const cacheKey = deDupeKeys.reduce(
        (acc, key) => (props[key] === undefined ? acc : `${acc}-${key}-${props[key]}`),
        tag
      )
      created = !createdElements[cacheKey]
      element = createdElements[cacheKey] ||= (() => {
        const e = document.createElement(tag)
        for (const key of deDupeKeys) {
          if (props[key] !== undefined) {
            e.setAttribute(key, props[key] as string)
          }
        }
        if (props.rel) {
          e.setAttribute('rel', props.rel)
        }
        return e
      })()
    }
  } else {
    existingElements = head.querySelectorAll<HTMLElement>(tag)
  }

  precedence = supportSort ? (precedence ?? '') : undefined
  if (supportSort) {
    restProps[dataPrecedenceAttr] = precedence
  }

  const insert = useCallback(
    (e: HTMLElement) => {
      if (deDupeByKey) {
        if (tag === 'link' && precedence !== undefined) {
          let found = false
          for (const existingElement of head.querySelectorAll<HTMLElement>(tag)) {
            const existingPrecedence = existingElement.getAttribute(dataPrecedenceAttr)
            if (existingPrecedence === null) {
              head.insertBefore(e, existingElement)
              return
            }
            if (found && existingPrecedence !== precedence) {
              head.insertBefore(e, existingElement)
              return
            }
            if (existingPrecedence === precedence) {
              found = true
            }
          }

          // if sentinel is not found, append to the end
          head.appendChild(e)
          return
        }

        let found = false
        for (const existingElement of head.querySelectorAll<HTMLElement>(tag)) {
          if (found && existingElement.getAttribute(dataPrecedenceAttr) !== precedence) {
            head.insertBefore(e, existingElement)
            return
          }
          if (existingElement.getAttribute(dataPrecedenceAttr) === precedence) {
            found = true
          }
        }

        // if sentinel is not found, append to the end
        head.appendChild(e)
      } else if (tag === 'link') {
        if (!head.contains(e)) {
          head.appendChild(e)
        }
      } else if (existingElements) {
        let found = false
        for (const existingElement of existingElements!) {
          if (existingElement === e) {
            found = true
            break
          }
        }
        if (!found) {
          // newly created element
          head.insertBefore(
            e,
            head.contains(existingElements[0]) ? existingElements[0] : head.querySelector(tag)
          )
        }
        existingElements = undefined
      }
    },
