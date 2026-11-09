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
- sym:53763 minify (src/helper/css/common.ts:106-108)
```typescript
export const minify = (css: string): string => {
  return css.replace(minifyCssRe, (_, $1, $2, $3, $4) => $1 || $2 || $3 || $4 || '')
}
```
- sym:53771 cssCommon (src/helper/css/common.ts:203-240)
  const cssCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:53757 normalizeLabel (src/helper/css/common.ts:52-54)
```typescript
const normalizeLabel = (label: string): string => {
  return label.trim().replace(/\s+/g, '-')
}
```
- sym:53984 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:53746 SELECTOR (src/helper/css/common.ts:6-6)
```typescript
export const SELECTOR: unique symbol = Symbol()
```
- sym:53784 createCssContext (src/helper/css/index.ts:72-215)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:53751 CSS_ESCAPED (src/helper/css/common.ts:11-11)
  const CSS_ESCAPED: unique symbol = Symbol()
- sym:53777 CssClassName (src/helper/css/index.ts:32-32)
  type CssClassName = HtmlEscapedString & CssClassNameCommon
- sym:53982 splitRule (src/jsx/dom/css.ts:27-66)
- sym:53752 CssClassName (src/helper/css/common.ts:13-19)
- sym:53756 toHash (src/helper/css/common.ts:43-50)
- sym:53747 CLASS_NAME (src/helper/css/common.ts:7-7)
- sym:53748 STYLE_STRING (src/helper/css/common.ts:8-8)
- sym:53749 SELECTORS (src/helper/css/common.ts:9-9)
- sym:53750 EXTERNAL_CLASS_NAMES (src/helper/css/common.ts:10-10)
- sym:53742 Support (src/helper/css/common.case.test.tsx:10-12)

---
tokens: 686/800
