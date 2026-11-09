## Task
# hono-ae401f0 · hono-ae401f0 · open
Goal: fix(css): keep the descendant combinator before a pseudo-class when minifying (#5451)

The minifier removed whitespace on both sides of ":", so in a nested
selector the space that means "descendant" was lost:

  & :first-child { margin: 0 }   ->  &:first-child{margin:0}
  .a :is(h1, h2) { color: red }  ->  .a:is(h1,h2){color:red}

That rule then matches the element itself instead of its descendants.
Only the whitespace after ":" is removed now.

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:53744 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
  const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
- sym:53759 hasUnsafeSelectorChar (src/helper/css/common.ts:60-60)
  const hasUnsafeSelectorChar = (name: string): boolean =>
- sym:53746 SELECTOR (src/helper/css/common.ts:6-6)
  const SELECTOR: unique symbol = Symbol()
- sym:53982 splitRule (src/jsx/dom/css.ts:27-66)
  const splitRule = (rule: string): string[] =>
- sym:53771 cssCommon (src/helper/css/common.ts:203-240)
  const cssCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:53984 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:53753 IS_CSS_ESCAPED (src/helper/css/common.ts:21-21)
  const IS_CSS_ESCAPED = Symbol()
- sym:53784 createCssContext (src/helper/css/index.ts:72-215)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:53772 cxCommon (src/helper/css/common.ts:242-260)
  const cxCommon = ( args: (string | boolean | null | undefined | CssClassName)[] ): (string | boolean | null | undefined | CssClassName)[] =>
- sym:53751 CSS_ESCAPED (src/helper/css/common.ts:11-11)
  const CSS_ESCAPED: unique symbol = Symbol()
- sym:53752 CssClassName (src/helper/css/common.ts:13-19)
  interface CssClassName
- sym:53754 CssEscapedString (src/helper/css/common.ts:23-25)
  interface CssEscapedString

---
tokens: 652/800
