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
### src/jsx/dom/index.test.tsx
```typescript
/** @jsxImportSource ../ */
import { JSDOM } from 'jsdom'
import DefaultExport, {
  cloneElement,
  cloneElement as cloneElementForDom,
  createElement as createElementForDom,
  createContext,
  useContext,
  createPortal,
  flushSync,
  isValidElement,
  memo,
  render,
  version,
} from '.'
import type { Child, FC } from '..'
// run tests by old style jsx default
// hono/jsx/jsx-runtime and hono/jsx/dom/jsx-runtime are tested in their respective settings
import { createElement, jsx } from '..'
import type { RefObject } from '../hooks'
import {
  createRef,
  useCallback,
  useEffect,
  useInsertionEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
} from '../hooks'
import type { NodeObject } from './render'
import { build, buildNode } from './render'

describe('Common', () => {
  ;[createElement, createElementForDom].forEach((createElement) => {
    describe('createElement', () => {
      it('simple', () => {
        const element = createElement('div', { id: 'app' })
        expect(element).toEqual(expect.objectContaining({ tag: 'div', props: { id: 'app' } }))
      })

      it('children', () => {
        const element = createElement('div', { id: 'app' }, 'Hello')
        expect(element).toEqual(
          expect.objectContaining({ tag: 'div', props: { id: 'app', children: 'Hello' } })
        )
      })

      it('multiple children', () => {
        const element = createElement('div', { id: 'app' }, 'Hello', 'World')
        expect(element).toEqual(
          expect.objectContaining({
            tag: 'div',
            props: { id: 'app', children: ['Hello', 'World'] },
          })
        )
      })

      it('key', () => {
        const element = createElement('div', { id: 'app', key: 'key' })
        expect(element).toEqual(
          expect.objectContaining({ tag: 'div', props: { id: 'app' }, key: 'key' })
        )
      })

      it('ref', () => {
        const ref = { current: null }
        const element = createElement('div', { id: 'app', ref })
        expect(element).toEqual(expect.objectContaining({ tag: 'div', props: { id: 'app', ref } }))
        expect(element.ref).toBe(ref)
      })

      it('type', () => {
        const element = createElement('div', { id: 'app' })
        expect(element.type).toBe('div')
      })

      it('null props', () => {
        const element = createElement('div', null)
        expect(element).toEqual(expect.objectContaining({ tag: 'div', props: {} }))
      })
    })
  })
})

describe('DOM', () => {
  beforeAll(() => {
    global.requestAnimationFrame = (cb) => setTimeout(cb)
  })

  let dom: JSDOM
  let root: HTMLElement
  beforeEach(() => {
    dom = new JSDOM('<html><body><div id="root"></div></body></html>', {
      runScripts: 'dangerously',
    })
    global.document = dom.window.document
    global.HTMLElement = dom.window.HTMLElement
    global.SVGElement = dom.window.SVGElement
    global.Text = dom.window.Text
    global.DOMException = dom.window.DOMException
    root = document.getElementById('root') as HTMLElement
  })

  it('simple App', () => {
    const App = <h1>Hello</h1>
    render(App, root)
    expect(root.innerHTML).toBe('<h1>Hello</h1>')
  })

  it('replace', () => {
    dom.window.document.body.innerHTML = '<div id="root">Existing content</div>'
    root = document.getElementById('root') as HTMLElement
    const App = <h1>Hello</h1>
    render(App, root)
    expect(root.innerHTML).toBe('<h1>Hello</h1>')
  })

  it('render text directly', () => {
    const App = () => <>{'Hello'}</>
    render(<App />, root)
    expect(root.innerHTML).toBe('Hello')
  })

  describe('performance', () => {
    it('should be O(N) for each additional element', () => {
      const App = () => (
        <>
          {Array.from({ length: 1000 }, (_, i) => (
            <div>
              <span>{i}</span>
            </div>
          ))}
        </>
      )
      render(<App />, root)
      expect(root.innerHTML).toBe(
        Array.from({ length: 1000 }, (_, i) => `<div><span>${i}</span></div>`).join('')
      )
    })

    it.each(['same-order', 'remove-second', 'adjacent-swap'])(
      'preserves keyed children during common updates: %s',
      (kind) => {
        const size = 100
        const ids = Array.from({ length: size }, (_, i) => i)
        const createRow = (id: number) => <div key={id}>{id}</div>
        const parent = buildNode(<section>{ids.map(createRow)}</section>) as NodeObject
        build([], parent)
        const initialChildren = parent.vC
        const nextIds = [...ids]
        if (kind === 'remove-second') {
          nextIds.splice(1, 1)
        } else if (kind === 'adjacent-swap') {
          ;[nextIds[1], nextIds[2]] = [nextIds[2], nextIds[1]]
        }
        parent.props.children = [[], ...nextIds.map(createRow)]

        build([], parent)

        expect(parent.vC).toHaveLength(nextIds.length)
        nextIds.forEach((id, i) => expect(parent.vC[i]).toBe(initialChildren[id]))
        expect(parent.vR).toHaveLength(kind === 'remove-second' ? 1 : 0)
        if (kind === 'remove-second') {
          expect(parent.vR[0]).toBe(initialChildren[1])
        }
      }
    )
  })

  describe('large keyed updates', () => {
    it.each([
      'reverse',
      'rotate-half',
      'replace-all',
      'footer-unkeyed',
      'footer-text',
      'insert-text',
      'move-text',
    ])('matches keyed list updates in linear work: %s', (kind) => {
      const measure = (size: number) => {
        let oldKeyReads = 0
        let newKeyReads = 0
        const createRow = (id: number, old = false) => {
          const row = <div>{id}</div>
          // Count both sides so node normalization cannot silently disable the check.
          Object.defineProperty(row, 'key', {
            configurable: true,
            get: () => {
              if (old) {
                oldKeyReads++
              } else {
                newKeyReads++
              }
              return id
            },
          })
          return row
        }
        const ids = Array.from({ length: size }, (_, i) => i)
        const before: Child[] = ids.map((id) => createRow(id, true))
        if (kind === 'footer-unkeyed') {
          before.push(<div>footer</div>)
        } else if (kind === 'footer-text' || kind === 'move-text') {
          before.push('original')
        }
        const parent = buildNode(<section>{before}</section>) as NodeObject
        build([], parent)
        const initial = parent.vC
        const next = [...ids]
        if (kind === 'rotate-half') {
          next.push(...next.splice(0, size / 2))
        } else if (kind === 'replace-all') {
          next.splice(0, size, ...ids.map((id) => id + size))
        } else {
          next.reverse()
        }
        const after: Child[] = next.map((id) => createRow(id))
        const expected = next.map((id) => (id < size ? initial[id] : undefined))
        if (kind === 'footer-unkeyed' || kind === 'footer-text') {
          after.push(kind === 'footer-unkeyed' ? <div>footer</div> : 'updated')
          expected.push(initial[size])
        } else if (kind === 'insert-text' || kind === 'move-text') {
          after.splice(4, 0, 'updated')
          expected.splice(4, 0, kind === 'move-text' ? initial[size] : undefined)
          if (kind === 'move-text') {
            after.splice(8, 0, 'extra')
            expected.splice(8, 0, undefined)
          }
        }
        parent.props.children = after
        oldKeyReads = newKeyReads = 0
        build([], parent)
        const reads = { old: oldKeyReads, new: newKeyReads }

        expect(parent.vC).toHaveLength(expected.length)
        expected.forEach((child, i) => {
          if (child) {
            expect(parent.vC[i]).toBe(child)
          } else {
            expect(initial.includes(parent.vC[i])).toBe(false)
          }
        })
        if (kind === 'footer-text' || kind === 'insert-text' || kind === 'move-text') {
          expect(parent.vC[kind === 'footer-text' ? size : 4]).toMatchObject({ t: 'updated' })
        }
        if (kind === 'move-text') {
          expect(parent.vC[8]).toMatchObject({ t: 'extra' })
        }
        expect(parent.vR).toHaveLength(kind === 'replace-all' ? size : 0)
        parent.vR.forEach((child, i) => expect(child).toBe(initial[i]))
        return reads
      }

      const small = measure(64)
      const large = measure(128)
      expect(small.old).toBeGreaterThan(0)
      expect(small.new).toBeGreaterThan(0)
      expect(large.old).toBeLessThan(small.old * 3)
      expect(large.old + large.new).toBeLessThan((small.old + small.new) * 3)
    })

    it.each(['unkeyed-to-text', 'undefined-tag-to-text', 'text-to-unkeyed'])(
      'replaces incompatible children after indexing: %s',
      (kind) => {
        const ids = Array.from({ length: 40 }, (_, i) => i)
        const createRow = (id: number) => <div key={id}>{id}</div>
        let footer: Child = kind === 'text-to-unkeyed' ? 'original' : <span>footer</span>
        if (kind === 'undefined-tag-to-text') {
          // @ts-expect-error An undefined component still creates a NodeObject at runtime.
          footer = createElementForDom(undefined, {})
        }
        const parent = buildNode(<section>{[...ids.map(createRow), footer]}</section>) as NodeObject
        build([], parent)
        const initial = parent.vC
        const next = [...ids].reverse()
        const after: Child[] = next.map(createRow)
        // Consume an indexed child before replacing the incompatible footer.
        after.splice(4, 0, kind === 'text-to-unkeyed' ? <span>updated</span> : 'updated')
        parent.props.children = after

        build([], parent)

        expect(parent.vC).toHaveLength(ids.length + 1)
        next.forEach((id, i) => expect(parent.vC[i < 4 ? i : i + 1]).toBe(initial[id]))
        expect(initial.includes(parent.vC[4])).toBe(false)
        expect(parent.vC[4]).toMatchObject(
          kind === 'text-to-unkeyed' ? { tag: 'span' } : { t: 'updated', d: true }
        )
        expect(parent.vR).toHaveLength(1)
        expect(parent.vR[0]).toBe(initial[ids.length])
      }
    )

    it('restores previous children with indexed matching and ordered removals', () => {
      const ids = Array.from({ length: 40 }, (_, i) => i)
      const createRow = (id: number) => <div key={id}>{id}</div>
      const parent = buildNode(<section>{ids.map(createRow)}</section>) as NodeObject
      build([], parent)
      const initial = parent.vC
      let keyReads = 0
      initial.forEach((child, id) => {
        Object.defineProperty(child, 'key', {
          configurable: true,
          get: () => {
            keyReads++
            return id
          },
        })
      })

      // Explicit fallback children preserve the original list in pC.
      build([], parent, [<span>Loading</span>])
      const fallback = parent.vC
      expect(parent.pC).toBe(initial)
      const next = [...ids].reverse().filter((id) => id !== 10 && id !== 20)
      parent.props.children = next.map(createRow)
      keyReads = 0
      build([], parent)
      const matchingReads = keyReads

      expect(matchingReads).toBeGreaterThan(0)
      expect(matchingReads).toBeLessThan(ids.length * 8)
      expect(parent.vC).toHaveLength(next.length)
      next.forEach((id, i) => expect(parent.vC[i]).toBe(initial[id]))
      const removed = [...fallback, initial[10], initial[20]]
      expect(parent.vR).toHaveLength(removed.length)
      removed.forEach((child, i) => expect(parent.vR[i]).toBe(child))
      expect(parent.pC).toBeUndefined()
    })

    it.each([
      { name: 'duplicate old keys', oldKeys: [0, 0], newKeys: [0, 0], matches: [0, 1] },
      { name: 'an unkeyed old child', oldKeys: [undefined, 1], newKeys: [undefined], matches: [0] },
      {
        name: 'multiple unkeyed old children',
        oldKeys: [undefined, undefined],
        newKeys: [undefined, undefined],
        matches: [0, 1],
      },
      { name: 'NaN keys', oldKeys: [NaN, 1], newKeys: [NaN, 1], matches: [-1, 1] },
      {
        name: 'a different tag for the same key',
        oldKeys: [0, 1],
        newKeys: [0, 0],
        firstNewTag: 'span',
        matches: [-1, 0],
      },
      { name: 'a repeated new key', oldKeys: [0, 1], newKeys: [0, 0], matches: [0, -1] },
      {
        name: 'different old tags sharing a key',
        oldKeys: [0, 0],
        oldSecondTag: 'span',
        newKeys: [0, 0],
        firstNewTag: 'span',
        matches: [1, 0],
      },
    ])(
      'preserves matching and removal order with $name',
      ({ oldKeys, newKeys, matches, oldSecondTag, firstNewTag }) => {
        const before = Array.from({ length: 40 }, (_, i) =>
          createElement(i === 1 ? oldSecondTag || 'div' : 'div', {
            key: i < oldKeys.length ? oldKeys[i] : i,
          })
        )
        const parent = buildNode(<section>{before}</section>) as NodeObject
        build([], parent)
        const initial = parent.vC
        // This prefix exceeds the scan budget and attempts to index the remaining nodes.
        const prefix = [39, 38, 37, 36]
        parent.props.children = [
          ...prefix.map((key) => <div key={key} />),
          ...newKeys.map((key, i) =>
            createElement(i === 0 ? firstNewTag || 'div' : 'div', { key })
          ),
        ]
        build([], parent)

        const expected = [...prefix, ...matches]
        expect(parent.vC).toHaveLength(expected.length)
        expected.forEach((oldIndex, i) => {
          if (oldIndex === -1) {
            expect(initial.includes(parent.vC[i])).toBe(false)
          } else {
            expect(parent.vC[i]).toBe(initial[oldIndex])
          }
        })
        const removed = initial.filter((_, i) => !expected.includes(i))
        expect(parent.vR).toHaveLength(removed.length)
        removed.forEach((child, i) => expect(parent.vR[i]).toBe(child))
      }
    )

    it.each(['keyed', 'unkeyed', 'text'])(
      'preserves state and cleanup order after indexing followed by %s children',
      async (kind) => {
        let update: () => void = () => {}
        const cleaned: number[] = []
        const Row = ({ id }: { id: number }) => {
          const [initialId] = useState(id)
          const [count, setCount] = useState(0)
          useLayoutEffect(
            () => () => {
              cleaned.push(initialId)
            },
            []
          )
          return (
            <button onClick={() => setCount(count + 1)}>
              {id}:{initialId}:{count}
            </button>
          )
        }
        const before = Array.from({ length: 40 }, (_, i) => i)
        const after = [...before]
          .reverse()
          .filter((id) => id !== 10 && id !== 20 && (kind !== 'unkeyed' || id !== 0))
        // Consume an indexed child before introducing an unkeyed or text child.
        if (kind !== 'keyed') {
          after.splice(4, 0, kind === 'unkeyed' ? 100 : 101)
        }
        const App = () => {
          const [ids, setIds] = useState(before)
          update = () => setIds(after)
          return ids.map((id) =>
            id === 101 ? 'inserted' : <Row key={id === 100 ? undefined : id} id={id} />
          )
        }

        render(<App />, root)
        const initial = [...root.querySelectorAll('button')]
        initial[0].click()
        await Promise.resolve()
        update()
        await Promise.resolve()

        const rows = after.filter((id) => id !== 101)
        expect(root.children).toHaveLength(rows.length)
        rows.forEach((id, i) => {
          const oldId = id === 100 ? 0 : id
          expect(root.children[i]).toBe(initial[oldId])
          expect(root.children[i].textContent).toBe(`${id}:${oldId}:${oldId === 0 ? 1 : 0}`)
        })
        if (kind === 'text') {
          expect(root.childNodes[4].textContent).toBe('inserted')
        }
        expect(cleaned).toEqual([10, 20])
      }
    )
  })

  describe('attribute', () => {
    it('simple', () => {
      const App = () => <div id='app' class='app' />
      render(<App />, root)
      expect(root.innerHTML).toBe('<div id="app" class="app"></div>')
    })

    it('boolean', () => {
      const App = () => <div hidden />
      render(<App />, root)
      expect(root.innerHTML).toBe('<div hidden=""></div>')
    })

    it('style', () => {
      const App = () => <div style={{ fontSize: '10px' }} />
      render(<App />, root)
      expect(root.innerHTML).toBe('<div style="font-size: 10px;"></div>')
    })

    it('update style', () => {
      const App = () => <div style={{ fontSize: '10px' }} />
      render(<App />, root)
      expect(root.innerHTML).toBe('<div style="font-size: 10px;"></div>')
    })

    it('style with CSS variables - 1', () => {
      const App = () => <div style={{ '--my-var-1': '15px' }} />
      render(<App />, root)
      expect(root.innerHTML).toBe('<div style="--my-var-1: 15px;"></div>')
    })

    it('style with CSS variables - 2', () => {
      const App = () => <div style={{ '--myVar-2': '20px' }} />
      render(<App />, root)
      expect(root.innerHTML).toBe('<div style="--myVar-2: 20px;"></div>')
    })

    it('style with string', async () => {
      const App = () => {
        const [style, setStyle] = useState<{ fontSize?: string; color?: string }>({
          fontSize: '10px',
        })
        return <div style={style} onClick={() => setStyle({ color: 'red' })} />
      }
      render(<App />, root)
      expect(root.innerHTML).toBe('<div style="font-size: 10px;"></div>')
      root.querySelector('div')?.click()
      await Promise.resolve()
      expect(root.innerHTML).toBe('<div style="color: red;"></div>')
    })

    it('toString() is called', () => {
      const App = () => <div x-value={{ toString: () => 'value' }} />
      render(<App />, root)
      expect(root.innerHTML).toBe('<div x-value="value"></div>')
    })

    it('ignores invalid attribute keys without interrupting updates', async () => {
      const invalidKey = '" onfocus="alert(1)'

      const App = () => {
        const [includeInvalid, setIncludeInvalid] = useState(true)
        return includeInvalid ? (
          <div id='safe' {...{ [invalidKey]: 'x' }} onClick={() => setIncludeInvalid(false)}>
            Hello
          </div>
        ) : (
          <div class='updated'>Hello</div>
        )
      }

      render(<App />, root)
      expect(root.innerHTML).toBe('<div id="safe">Hello</div>')

      root.querySelector('div')?.click()
      await Promise.resolve()

      expect(root.innerHTML).toBe('<div class="updated">Hello</div>')
    })

