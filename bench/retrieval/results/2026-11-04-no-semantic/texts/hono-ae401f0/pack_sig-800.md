## Task
# hono-ae401f0 · hono-ae401f0-2 · open
Goal: fix(css): keep the descendant combinator before a pseudo-class when minifying (#5451)

The minifier removed whitespace on both sides of ":", so in a nested
selector the space that means "descendant" was lost:

  & :first-child { margin: 0 }   ->  &:first-child{margin:0}
  .a :is(h1, h2) { color: red }  ->  .a:is(h1,h2){color:red}

That rule then matches the element itself instead of its descendants.
Only the whitespace after ":" is removed now.
Workspace: /root/bench/ws/rebf815-hono-ae401f0/hono-ae401f0-2
Changed (0): nothing yet

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:53744 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
  const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
- sym:53771 cssCommon (src/helper/css/common.ts:203-240)
  const cssCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:53984 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:53746 SELECTOR (src/helper/css/common.ts:6-6)
  const SELECTOR: unique symbol = Symbol()
- sym:53784 createCssContext (src/helper/css/index.ts:72-215)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:53759 hasUnsafeSelectorChar (src/helper/css/common.ts:60-60)
  const hasUnsafeSelectorChar = (name: string): boolean =>
- sym:53982 splitRule (src/jsx/dom/css.ts:27-66)
  const splitRule = (rule: string): string[] =>
- sym:53752 CssClassName (src/helper/css/common.ts:13-19)
  interface CssClassName
- sym:53693 Set (src/context.ts:105-106)
  interface Set<E extends Env>
- sym:53747 CLASS_NAME (src/helper/css/common.ts:7-7)
  const CLASS_NAME: unique symbol = Symbol()
- sym:53748 STYLE_STRING (src/helper/css/common.ts:8-8)
  const STYLE_STRING: unique symbol = Symbol()
- sym:53749 SELECTORS (src/helper/css/common.ts:9-9)
  const SELECTORS: unique symbol = Symbol()

---
tokens: 650/800
