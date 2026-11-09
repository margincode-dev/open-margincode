## Task
# hono-5ca49b0 · hono-5ca49b0 · open
Goal: fix(css): Fix multiline CSS handling in style replacement (#5405)

* Fix multiline CSS handling in style replacement

* Use dotAll flag for replaceStyleRe

* add test

* format

---------
Workspace: /root/bench/ws/rebf815-hono-5ca49b0/hono-5ca49b0
Changed (0): nothing yet

## Map
- repo map (583 files):
  .editorconfig
  .github/ (12 files)
    FUNDING.yml
    ISSUE_TEMPLATE/ (3 files)
    actions/ (1 files)
    pull_request_template.md
    workflows/ (6 files)
  .gitignore
  .tool-versions
  .vscode/ (2 files)
    extensions.json
    settings.json
  LICENSE
  README.md
  adapters/ (113 files)
    aws-lambda/ (15 files)
    bun/ (21 files)
    cloudflare-workers/ (16 files)
    deno/ (24 files)
    lambda-edge/ (10 files)
    netlify/ (10 files)
    service-worker/ (8 files)
    vercel/ (9 files)
  benchmarks/ (72 files)
    fetch/ (8 files)
    http-server/ (3 files)
    jsx-dom/ (1 files)
    jsx/ (20 files)
    query-param/ (7 files)
    routers-deno/ (12 files)
    routers/ (17 files)
    utils/ (4 files)
  build/ (8 files)
    declaration-emit.test.ts
    dts-plugins.ts
    fixtures/ (4 files)
    jsx-types.test.ts
    release-adapter.ts
  bunfig.toml
  codecov.yml
  docs/ (10 files)
    CODE_OF_CONDUCT.md
    CONTRIBUTING.md
    MAINTAINING.md
    MIGRATION.md
    images/ (6 files)
  jsr.json
  package.cjs.json
  package.json
  perf-measures/ (12 files)
    .octocov.consolidated.perf-measures.main.yml
    .octocov.consolidated.perf-measures.yml
    bundle-check/ (3 files)
    type-check/ (7 files)
  pnpm-lock.yaml
  pnpm-workspace.yaml
  runtime-tests/ (22 files)
    bun/ (3 files)
    deno-jsx/ (3 files)
    deno/ (6 files)
    fastly/ (3 files)
    node/ (3 files)
    workerd/ (4 files)
  src/ (314 files)
    adapter/ (57 files)
    client/ (8 files)
    compose.test.ts
    compose.ts
    context.test.ts
    context.ts
    helper/ (43 files)
    hono-base.ts
    hono.test.ts
    hono.ts
    http-exception.test.ts
    http-exception.ts
    index.ts
    jsx/ (46 files)
    middleware/ (66 files)
    preset/ (4 files)
    request.test.ts
    request.ts
    request/ (1 files)
    router.ts
    router/ (24 files)
    types.test.ts
    types.ts
    utils/ (45 files)
    validator/ (5 files)
  tsconfig.base.json
  tsconfig.build.json
  tsconfig.json
  tsconfig.spec.json
  vite.config.ts
  vitest.config.ts

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
  const SELECTORS: unique symbol = Symbol()
- sym:46806 toCSS (src/helper/css/index.test.tsx:24-30)
  async function toCSS( template: JSXNode | Promise<HtmlEscapedString> | Promise<string> | HtmlEscapedString )
- sym:46773 renderTest (src/helper/css/common.case.test.tsx:13-506)
  const renderTest = ( getEnv: () => { css: typeof cssHelper keyframes: typeof keyframesHelper viewTransition: typeof viewTransitionHelper rawCssString: typeof rawCssStringHelper // eslint-disable-next-line @typescript-eslint/no-explicit-any toString: (template: any) => Promise<string> Style: typeof StyleComponent support: Support } ) =>

---
tokens: 1421/2000
