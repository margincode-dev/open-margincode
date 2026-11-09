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
- sym:46775 DEFAULT_STYLE_ID (src/helper/css/common.ts:4-4)
  const DEFAULT_STYLE_ID = 'hono-css'
- sym:46778 STYLE_STRING (src/helper/css/common.ts:8-8)
  const STYLE_STRING: unique symbol = Symbol()
- sym:47014 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:46814 createCssContext (src/helper/css/index.ts:72-215)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:46875 handleNonDot (src/helper/ssg/utils.ts:35-40)
  const handleNonDot = (path: string, resultPaths: string[]): void =>
- sym:46781 CSS_ESCAPED (src/helper/css/common.ts:11-11)
  const CSS_ESCAPED: unique symbol = Symbol()
- sym:46813 StyleType (src/helper/css/index.ts:59-61)
  interface StyleType
- sym:47439 Trie.buildRegExp (src/router/reg-exp-router/trie.ts:59-83)
  buildRegExp(): [RegExp, ReplacementMap, ReplacementMap]
- sym:46793 minify (src/helper/css/common.ts:108-110)
  const minify = (css: string): string =>
- sym:46486 main (benchmarks/http-server/benchmark.ts:234-324)
  const main = async () =>
- sym:46723 Set (src/context.ts:105-106)
  interface Set<E extends Env>
- sym:46774 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
  const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
- sym:46776 SELECTOR (src/helper/css/common.ts:6-6)
  const SELECTOR: unique symbol = Symbol()
- sym:46777 CLASS_NAME (src/helper/css/common.ts:7-7)
  const CLASS_NAME: unique symbol = Symbol()
- sym:46773 renderTest (src/helper/css/common.case.test.tsx:13-506)

---
tokens: 652/800
