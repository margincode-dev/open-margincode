## Task
# hono-5ca49b0 · hono-5ca49b0 · open
Goal: fix(css): Fix multiline CSS handling in style replacement (#5405)

* Fix multiline CSS handling in style replacement

* Use dotAll flag for replaceStyleRe

* add test

* format

---------

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:46800 buildStyleString (src/helper/css/common.ts:143-203)
  const buildStyleString = ( strings: TemplateStringsArray, values: CssVariableType[] ): [string, string, CssClassName[], string[]] =>
- sym:46814 createCssContext (src/helper/css/index.ts:72-215)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:47014 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:46793 minify (src/helper/css/common.ts:108-110)
  const minify = (css: string): string =>
- sym:47439 Trie.buildRegExp (src/router/reg-exp-router/trie.ts:59-83)
  buildRegExp(): [RegExp, ReplacementMap, ReplacementMap]
- sym:46775 DEFAULT_STYLE_ID (src/helper/css/common.ts:4-4)
  const DEFAULT_STYLE_ID = 'hono-css'
- sym:47173 hasUnsafeStyleValue (src/jsx/utils.ts:122-167)
  const hasUnsafeStyleValue = (value: string): boolean =>
- sym:46778 STYLE_STRING (src/helper/css/common.ts:8-8)
  const STYLE_STRING: unique symbol = Symbol()
- sym:47012 splitRule (src/jsx/dom/css.ts:27-66)
  const splitRule = (rule: string): string[] =>
- sym:46783 IS_CSS_ESCAPED (src/helper/css/common.ts:21-21)
  const IS_CSS_ESCAPED = Symbol()
- sym:46486 main (benchmarks/http-server/benchmark.ts:234-324)
  const main = async () =>
- sym:46774 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
  const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
- sym:46776 SELECTOR (src/helper/css/common.ts:6-6)
  const SELECTOR: unique symbol = Symbol()
- sym:46777 CLASS_NAME (src/helper/css/common.ts:7-7)
  const CLASS_NAME: unique symbol = Symbol()
- sym:46779 SELECTORS (src/helper/css/common.ts:9-9)
- sym:46806 toCSS (src/helper/css/index.test.tsx:24-30)
- sym:46773 renderTest (src/helper/css/common.case.test.tsx:13-506)

---
tokens: 672/800
