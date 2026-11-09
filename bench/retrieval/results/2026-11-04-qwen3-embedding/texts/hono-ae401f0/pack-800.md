## Task
# hono-ae401f0 · hono-ae401f0 · open

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:53744 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
```typescript
export const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
```
- sym:53759 hasUnsafeSelectorChar (src/helper/css/common.ts:60-60)
```typescript
const hasUnsafeSelectorChar = (name: string): boolean => /[<{}]/.test(name)
```
- sym:53746 SELECTOR (src/helper/css/common.ts:6-6)
```typescript
export const SELECTOR: unique symbol = Symbol()
```
- sym:53982 splitRule (src/jsx/dom/css.ts:27-66)
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
- sym:53771 cssCommon (src/helper/css/common.ts:203-240)
  const cssCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:53984 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:53753 IS_CSS_ESCAPED (src/helper/css/common.ts:21-21)
```typescript
export const IS_CSS_ESCAPED = Symbol()
```
- sym:53784 createCssContext (src/helper/css/index.ts:72-215)
- sym:53772 cxCommon (src/helper/css/common.ts:242-260)
- sym:53751 CSS_ESCAPED (src/helper/css/common.ts:11-11)
  const CSS_ESCAPED: unique symbol = Symbol()
- sym:53752 CssClassName (src/helper/css/common.ts:13-19)
  interface CssClassName
- sym:53754 CssEscapedString (src/helper/css/common.ts:23-25)

---
tokens: 686/800
