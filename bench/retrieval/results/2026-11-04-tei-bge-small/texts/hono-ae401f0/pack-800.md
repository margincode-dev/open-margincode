## Task
# hono-ae401f0 · hono-ae401f0 · open

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
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
- sym:53759 hasUnsafeSelectorChar (src/helper/css/common.ts:60-60)
  const hasUnsafeSelectorChar = (name: string): boolean =>
- sym:53744 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
```typescript
export const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
```
- sym:54494 trimCookieWhitespace (src/utils/cookie.ts:86-107)
  const trimCookieWhitespace = (value: string): string =>
- sym:53771 cssCommon (src/helper/css/common.ts:203-240)
  const cssCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:54438 ignoreTrailingWhitespace (src/utils/accept.ts:20-28)
- sym:53763 minify (src/helper/css/common.ts:106-108)
- sym:53984 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
- sym:53746 SELECTOR (src/helper/css/common.ts:6-6)
- sym:54436 isWhitespace (src/utils/accept.ts:7-8)
- sym:53784 createCssContext (src/helper/css/index.ts:72-215)
- sym:54437 consumeWhitespace (src/utils/accept.ts:10-18)
- sym:54224 ComponentWithChildren (src/middleware/jsx-renderer/index.ts:26-29)
- sym:53752 CssClassName (src/helper/css/common.ts:13-19)
  interface CssClassName
- sym:53772 cxCommon (src/helper/css/common.ts:242-260)

---
tokens: 679/800
