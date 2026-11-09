## Task
# hono-5ca49b0 · hono-5ca49b0-2 · open
Goal: fix(css): Fix multiline CSS handling in style replacement (#5405)

* Fix multiline CSS handling in style replacement

* Use dotAll flag for replaceStyleRe

* add test

* format

---------
Workspace: /root/bench/ws/rebf815-hono-5ca49b0/hono-5ca49b0-2
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
- sym:46814 createCssContext (src/helper/css/index.ts:72-215)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:46800 buildStyleString (src/helper/css/common.ts:143-203)
  const buildStyleString = ( strings: TemplateStringsArray, values: CssVariableType[] ): [string, string, CssClassName[], string[]] =>
- sym:47439 Trie.buildRegExp (src/router/reg-exp-router/trie.ts:59-83)
  buildRegExp(): [RegExp, ReplacementMap, ReplacementMap]
- sym:46775 DEFAULT_STYLE_ID (src/helper/css/common.ts:4-4)
  const DEFAULT_STYLE_ID = 'hono-css'
- sym:46778 STYLE_STRING (src/helper/css/common.ts:8-8)
  const STYLE_STRING: unique symbol = Symbol()
- sym:47014 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:46486 main (benchmarks/http-server/benchmark.ts:234-324)
  const main = async () =>
- sym:47174 styleObjectForEach (src/jsx/utils.ts:169-202)
  const styleObjectForEach = ( style: Record<string, unknown>, fn: (key: string, value: string | null) => void ): void =>
- sym:46875 handleNonDot (src/helper/ssg/utils.ts:35-40)
  const handleNonDot = (path: string, resultPaths: string[]): void =>
- sym:46815 DefaultContextType (src/helper/css/index.ts:217-223)
  interface DefaultContextType
- sym:46801 cssCommon (src/helper/css/common.ts:205-242)
  const cssCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:46813 StyleType (src/helper/css/index.ts:59-61)
  interface StyleType
- sym:46782 CssClassName (src/helper/css/common.ts:13-19)
  interface CssClassName
- sym:46803 keyframesCommon (src/helper/css/common.ts:264-292)
  const keyframesCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:47437 Trie (src/router/reg-exp-router/trie.ts:7-84)
  class Trie
- sym:46798 ClassNameSlug (src/helper/css/common.ts:134-134)
  type ClassNameSlug = (hash: string, label: string, styleString: string) => string
- sym:47020 createCssContext (src/jsx/dom/css.ts:180-226)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:47019 DefaultContextType (src/jsx/dom/css.ts:163-169)
  interface DefaultContextType
- sym:47259 unauthorizedResponse (src/middleware/jwk/jwk.ts:176-196)
  function unauthorizedResponse(opts: { ctx: Context error: string errDescription: string statusText?: string realm?: string })
- sym:47262 unauthorizedResponse (src/middleware/jwt/jwt.ts:165-185)
  function unauthorizedResponse(opts: { ctx: Context error: string errDescription: string statusText?: string realm?: string })
- sym:46793 minify (src/helper/css/common.ts:108-110)
  const minify = (css: string): string =>
- sym:46787 normalizeLabel (src/helper/css/common.ts:52-54)
  const normalizeLabel = (label: string): string =>
- sym:46802 cxCommon (src/helper/css/common.ts:244-262)
  const cxCommon = ( args: (string | boolean | null | undefined | CssClassName)[] ): (string | boolean | null | undefined | CssClassName)[] =>
- sym:47063 applyProps (src/jsx/dom/render.ts:164-274)
  const applyProps = ( container: SupportedElement, attributes: Props, oldAttributes?: Props ): void =>
- sym:47088 setInternalTagFlag (src/jsx/dom/utils.ts:3-7)
  const setInternalTagFlag = (fn: Function): Function =>
- sym:47487 BodyDataValue (src/utils/body.ts:35-41)
  type BodyDataValue<T> = | BodyDataValueComponent<T> | (T extends { dot: false } ? never // explicitly set to false : T extends { dot: true } | { dot: boolean } ? BodyDataValueObject<T> // use dot option : never) // without options
- sym:47436 ReplacementMap (src/router/reg-exp-router/trie.ts:5-5)
  type ReplacementMap = number[]
- sym:46693 ToEndpoints (src/client/types.ts:338-346)
  type ToEndpoints<Def extends GlobalResponseDefinition, R> = { [S in keyof Def & StatusCode]: { [F in keyof Def[S] & KnownResponseFormat]: Omit<R, 'output' | 'status' | 'outputFormat'> & { output: Def[S][F] status: S outputFormat: F } }[keyof Def[S] & KnownResponseFormat] }[keyof Def & StatusCode]
- sym:47584 CHAR_CODE_DOT (src/utils/ipaddr.ts:52-52)
  const CHAR_CODE_DOT = 46
- sym:47489 ParseBodyOptions (src/utils/body.ts:46-78)
  type ParseBodyOptions = { /** * Determines whether all fields with multiple values should be parsed as arrays. * @default false * @example * const data = new FormData() * data.append('file', 'aaa') * data.append('file', 'bbb') * data.append('message', 'hello') * * If all is false: * parseBody should return { file: 'bbb', message: 'hello' } * * If all is true: * parseBody should return { file: ['aaa', 'bbb'], message: 'hello' } */ all: boolean /** * Determines whether all fields with dot notation should be parsed as nested objects. * @default false * @example * const data = new FormData() * data.append('obj.key1', 'value1') * data.append('obj.key2', 'value2') * * If dot is false: * parseBody should return { 'obj.key1': 'value1', 'obj.key2': 'value2' } * * If dot is true: * parseBody should return { obj: { key1: 'value1', key2: 'value2' } } */ dot: boolean }
- sym:46774 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
  const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
- sym:46776 SELECTOR (src/helper/css/common.ts:6-6)
  const SELECTOR: unique symbol = Symbol()
- sym:46777 CLASS_NAME (src/helper/css/common.ts:7-7)
  const CLASS_NAME: unique symbol = Symbol()
- sym:46779 SELECTORS (src/helper/css/common.ts:9-9)
  const SELECTORS: unique symbol = Symbol()
- sym:46780 EXTERNAL_CLASS_NAMES (src/helper/css/common.ts:10-10)
  const EXTERNAL_CLASS_NAMES: unique symbol = Symbol()
- sym:46781 CSS_ESCAPED (src/helper/css/common.ts:11-11)
  const CSS_ESCAPED: unique symbol = Symbol()
- sym:46784 CssEscapedString (src/helper/css/common.ts:23-25)
  interface CssEscapedString
- sym:46797 CssVariableType (src/helper/css/common.ts:122-122)
  type CssVariableType = CssVariableBasicType | CssVariableAsyncType | CssVariableArrayType
- sym:46799 OnInvalidSlug (src/helper/css/common.ts:141-141)
  type OnInvalidSlug = (slug: string) => void
- sym:46804 ViewTransitionType (src/helper/css/common.ts:294-303)
  type ViewTransitionType = { ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName (content: CssClassName): CssClassName (): CssClassName }
- sym:46807 CssClassName (src/helper/css/index.ts:32-32)
  type CssClassName = HtmlEscapedString & CssClassNameCommon
- sym:46808 usedClassNameData (src/helper/css/index.ts:34-37)
  type usedClassNameData = [ Record<string, string>, // class name to add Record<string, true>, // class name already added ]
- sym:46809 CssType (src/helper/css/index.ts:39-41)
  interface CssType
- sym:46810 CxType (src/helper/css/index.ts:43-47)
  interface CxType
- sym:46811 KeyframesType (src/helper/css/index.ts:49-51)
  interface KeyframesType
- sym:46773 renderTest (src/helper/css/common.case.test.tsx:13-506)
  const renderTest = ( getEnv: () => { css: typeof cssHelper keyframes: typeof keyframesHelper viewTransition: typeof viewTransitionHelper rawCssString: typeof rawCssStringHelper // eslint-disable-next-line @typescript-eslint/no-explicit-any toString: (template: any) => Promise<string> Style: typeof StyleComponent support: Support } ) =>
- sym:46806 toCSS (src/helper/css/index.test.tsx:24-30)
  async function toCSS( template: JSXNode | Promise<HtmlEscapedString> | Promise<string> | HtmlEscapedString )
- sym:46805 toString (src/helper/css/index.test.tsx:12-22)
  async function toString( template: JSXNode | Promise<HtmlEscapedString> | Promise<string> | HtmlEscapedString )

---
tokens: 2927/8000
