## Task
# hono-ae401f0 · hono-ae401f0 · open
Goal: fix(css): keep the descendant combinator before a pseudo-class when minifying (#5451)

The minifier removed whitespace on both sides of ":", so in a nested
selector the space that means "descendant" was lost:

  & :first-child { margin: 0 }   ->  &:first-child{margin:0}
  .a :is(h1, h2) { color: red }  ->  .a:is(h1,h2){color:red}

That rule then matches the element itself instead of its descendants.
Only the whitespace after ":" is removed now.
Workspace: /root/bench/av_gemma/ws/rec728b-hono-ae401f0/hono-ae401f0
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
- sym:53744 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
  const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
- sym:53759 hasUnsafeSelectorChar (src/helper/css/common.ts:60-60)
  const hasUnsafeSelectorChar = (name: string): boolean =>
- sym:53746 SELECTOR (src/helper/css/common.ts:6-6)
  const SELECTOR: unique symbol = Symbol()
- sym:53982 splitRule (src/jsx/dom/css.ts:27-66)
  const splitRule = (rule: string): string[] =>
- sym:53984 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:53772 cxCommon (src/helper/css/common.ts:242-260)
  const cxCommon = ( args: (string | boolean | null | undefined | CssClassName)[] ): (string | boolean | null | undefined | CssClassName)[] =>
- sym:53763 minify (src/helper/css/common.ts:106-108)
  const minify = (css: string): string =>
- sym:53771 cssCommon (src/helper/css/common.ts:203-240)
  const cssCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:53757 normalizeLabel (src/helper/css/common.ts:52-54)
  const normalizeLabel = (label: string): string =>
- sym:53784 createCssContext (src/helper/css/index.ts:72-215)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:53751 CSS_ESCAPED (src/helper/css/common.ts:11-11)
  const CSS_ESCAPED: unique symbol = Symbol()
- sym:53777 CssClassName (src/helper/css/index.ts:32-32)
  type CssClassName = HtmlEscapedString & CssClassNameCommon
- sym:53752 CssClassName (src/helper/css/common.ts:13-19)
  interface CssClassName
- sym:53756 toHash (src/helper/css/common.ts:43-50)
  const toHash = (str: string): string =>
- sym:53754 CssEscapedString (src/helper/css/common.ts:23-25)
  interface CssEscapedString
- sym:53753 IS_CSS_ESCAPED (src/helper/css/common.ts:21-21)
  const IS_CSS_ESCAPED = Symbol()
- sym:54451 MAX_NESTED_OBJECTS (src/utils/body.ts:10-10)
  const MAX_NESTED_OBJECTS = 10_000
- sym:53773 keyframesCommon (src/helper/css/common.ts:262-290)
  const keyframesCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:54019 PreserveNodeType (src/jsx/dom/render.ts:37-39)
  type PreserveNodeType = | 1 // preserve only self | 2 // preserve self and children
- sym:54466 handleParsingNestedValues (src/utils/body.ts:225-259)
  const handleParsingNestedValues = ( form: BodyData, key: string, value: BodyDataValue<Partial<ParseBodyOptions>>, state: { count: number } ): void =>
- sym:53760 RESERVED_KEYFRAME_NAMES (src/helper/css/common.ts:63-71)
  const RESERVED_KEYFRAME_NAMES = new Set([
- sym:54436 isWhitespace (src/utils/accept.ts:7-8)
  const isWhitespace = (char: number): boolean =>
- sym:54029 toAttributeName (src/jsx/dom/render.ts:135-142)
  const toAttributeName = (element: SupportedElement, key: string): string =>
- sym:54437 consumeWhitespace (src/utils/accept.ts:10-18)
  const consumeWhitespace = (acceptHeader: string, startIndex: number): number =>
- sym:54438 ignoreTrailingWhitespace (src/utils/accept.ts:20-28)
  const ignoreTrailingWhitespace = (acceptHeader: string, startIndex: number): number =>
- sym:53770 buildStyleString (src/helper/css/common.ts:141-201)
  const buildStyleString = ( strings: TemplateStringsArray, values: CssVariableType[] ): [string, string, CssClassName[], string[]] =>
- sym:53909 getNameSpaceContext (src/jsx/base.ts:56-56)
  const getNameSpaceContext = () =>
- sym:53758 isValidClassName (src/helper/css/common.ts:56-56)
  const isValidClassName = (name: string): boolean =>
- sym:54026 getNameSpaceContext (src/jsx/dom/render.ts:111-111)
  const getNameSpaceContext = () =>
- sym:54095 isStylesheetLinkWithPrecedence (src/jsx/intrinsic-element/common.ts:15-16)
  const isStylesheetLinkWithPrecedence = (props: Props): boolean =>
- sym:54459 ParseBodyOptions (src/utils/body.ts:46-78)
  type ParseBodyOptions = { /** * Determines whether all fields with multiple values should be parsed as arrays. * @default false * @example * const data = new FormData() * data.append('file', 'aaa') * data.append('file', 'bbb') * data.append('message', 'hello') * * If all is false: * parseBody should return { file: 'bbb', message: 'hello' } * * If all is true: * parseBody should return { file: ['aaa', 'bbb'], message: 'hello' } */ all: boolean /** * Determines whether all fields with dot notation should be parsed as nested objects. * @default false * @example * const data = new FormData() * data.append('obj.key1', 'value1') * data.append('obj.key2', 'value2') * * If dot is false: * parseBody should return { 'obj.key1': 'value1', 'obj.key2': 'value2' } * * If dot is true: * parseBody should return { obj: { key1: 'value1', key2: 'value2' } } */ dot: boolean }
- sym:53783 StyleType (src/helper/css/index.ts:59-61)
  interface StyleType
- sym:54494 trimCookieWhitespace (src/utils/cookie.ts:86-107)
  const trimCookieWhitespace = (value: string): string =>
- sym:53910 toSVGAttributeName (src/jsx/base.ts:58-66)
  const toSVGAttributeName = (key: string): string =>
- sym:54104 meta (src/jsx/intrinsic-element/components.ts:171-177)
  const meta: FC<PropsWithChildren> = ({ children, ...props }) =>
- sym:53768 ClassNameSlug (src/helper/css/common.ts:132-132)
  type ClassNameSlug = (hash: string, label: string, styleString: string) => string
- sym:53832 combineBeforeRequestHooks (src/helper/ssg/ssg.ts:118-137)
  const combineBeforeRequestHooks = ( hooks: BeforeRequestHook | BeforeRequestHook[] ): BeforeRequestHook =>
- sym:53747 CLASS_NAME (src/helper/css/common.ts:7-7)
  const CLASS_NAME: unique symbol = Symbol()
- sym:54101 script (src/jsx/intrinsic-element/components.ts:136-149)
  const script: FC<PropsWithChildren<IntrinsicElements['script']>> = ({ children, ...props }) =>
- sym:53840 toSSG (src/helper/ssg/ssg.ts:369-471)
  const toSSG: ToSSGInterface = async (app, fs, options) =>
- sym:53745 DEFAULT_STYLE_ID (src/helper/css/common.ts:4-4)
  const DEFAULT_STYLE_ID = 'hono-css'
- sym:54100 title (src/jsx/intrinsic-element/components.ts:121-135)
  const title: FC<PropsWithChildren> = ({ children, ...props }) =>
- sym:54045 replaceContainer (src/jsx/dom/render.ts:760-765)
  const replaceContainer = (node: NodeObject, from: DocumentFragment, to: Container): void =>
- sym:54613 JwtTokenNotBefore (src/utils/jwt/types.ts:34-39)
  class JwtTokenNotBefore extends Error
- sym:53749 SELECTORS (src/helper/css/common.ts:9-9)
  const SELECTORS: unique symbol = Symbol()
- sym:54002 title (src/jsx/dom/intrinsic-element/components.ts:257-270)
  const title: FC<PropsWithChildren> = (props) =>
- sym:54144 styleObjectForEach (src/jsx/utils.ts:169-202)
  const styleObjectForEach = ( style: Record<string, unknown>, fn: (key: string, value: string | null) => void ): void =>
- sym:54441 getNextParam (src/utils/accept.ts:66-157)
  const getNextParam = ( acceptHeader: string, startIndex: number ): [number, string | undefined, string | undefined, boolean] =>
- sym:54614 JwtTokenNotBefore.constructor (src/utils/jwt/types.ts:35-38)
  constructor(token: string)
- sym:54143 hasUnsafeStyleValue (src/jsx/utils.ts:122-167)
  const hasUnsafeStyleValue = (value: string): boolean =>
- sym:54442 getNextAcceptValue (src/utils/accept.ts:159-209)
  const getNextAcceptValue = ( acceptHeader: string, startIndex: number ): [number, Accept | undefined] =>
- sym:53829 BeforeRequestHook (src/helper/ssg/ssg.ts:110-110)
  type BeforeRequestHook = (req: Request) => Request | false | Promise<Request | false>
- sym:54102 style (src/jsx/intrinsic-element/components.ts:151-161)
  const style: FC<PropsWithChildren<IntrinsicElements['style']>> = ({ children, ...props }) =>
- sym:53835 SSGPlugin (src/helper/ssg/ssg.ts:175-179)
  interface SSGPlugin
- sym:53986 CxType (src/jsx/dom/css.ts:149-151)
  interface CxType
- sym:53742 Support (src/helper/css/common.case.test.tsx:10-12)
  interface Support
- sym:53743 renderTest (src/helper/css/common.case.test.tsx:13-506)
  const renderTest = ( getEnv: () => { css: typeof cssHelper keyframes: typeof keyframesHelper viewTransition: typeof viewTransitionHelper rawCssString: typeof rawCssStringHelper // eslint-disable-next-line @typescript-eslint/no-explicit-any toString: (template: any) => Promise<string> Style: typeof StyleComponent support: Support } ) =>
- sym:53776 toCSS (src/helper/css/index.test.tsx:24-30)
  async function toCSS( template: JSXNode | Promise<HtmlEscapedString> | Promise<string> | HtmlEscapedString )

---
tokens: 3243/8000
