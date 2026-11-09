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
