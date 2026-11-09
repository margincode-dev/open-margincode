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
