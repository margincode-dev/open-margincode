## Task
# hono-ae401f0 · hono-ae401f0 · open
Goal: fix(css): keep the descendant combinator before a pseudo-class when minifying (#5451)

The minifier removed whitespace on both sides of ":", so in a nested
selector the space that means "descendant" was lost:

  & :first-child { margin: 0 }   ->  &:first-child{margin:0}
  .a :is(h1, h2) { color: red }  ->  .a:is(h1,h2){color:red}

That rule then matches the element itself instead of its descendants.
Only the whitespace after ":" is removed now.
Workspace: /root/bench/av_tei/ws/rec09fc-hono-ae401f0/hono-ae401f0
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
- sym:53982 splitRule (src/jsx/dom/css.ts:27-66)
  const splitRule = (rule: string): string[] =>
- sym:53771 cssCommon (src/helper/css/common.ts:203-240)
  const cssCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:53759 hasUnsafeSelectorChar (src/helper/css/common.ts:60-60)
  const hasUnsafeSelectorChar = (name: string): boolean =>
- sym:54438 ignoreTrailingWhitespace (src/utils/accept.ts:20-28)
  const ignoreTrailingWhitespace = (acceptHeader: string, startIndex: number): number =>
- sym:54436 isWhitespace (src/utils/accept.ts:7-8)
  const isWhitespace = (char: number): boolean =>
- sym:54437 consumeWhitespace (src/utils/accept.ts:10-18)
  const consumeWhitespace = (acceptHeader: string, startIndex: number): number =>
- sym:53744 PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
  const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
- sym:54494 trimCookieWhitespace (src/utils/cookie.ts:86-107)
  const trimCookieWhitespace = (value: string): string =>
- sym:53763 minify (src/helper/css/common.ts:106-108)
  const minify = (css: string): string =>
- sym:53984 createCssJsxDomObjects (src/jsx/dom/css.ts:77-143)
  const createCssJsxDomObjects: CreateCssJsxDomObjectsType = ({ id }) =>
- sym:53746 SELECTOR (src/helper/css/common.ts:6-6)
  const SELECTOR: unique symbol = Symbol()
- sym:53784 createCssContext (src/helper/css/index.ts:72-215)
  const createCssContext = ({ id, classNameSlug, onInvalidSlug, }: { id: Readonly<string> classNameSlug?: ClassNameSlug onInvalidSlug?: OnInvalidSlug }): DefaultContextType =>
- sym:54224 ComponentWithChildren (src/middleware/jsx-renderer/index.ts:26-29)
  type ComponentWithChildren = ( props: PropsWithChildren<PropsForRenderer & { Layout: FC }>, c: Context ) => ComponentResult
- sym:53752 CssClassName (src/helper/css/common.ts:13-19)
  interface CssClassName
- sym:53772 cxCommon (src/helper/css/common.ts:242-260)
  const cxCommon = ( args: (string | boolean | null | undefined | CssClassName)[] ): (string | boolean | null | undefined | CssClassName)[] =>
- sym:54039 findChildNodeIndex (src/jsx/dom/render.ts:371-386)
  const findChildNodeIndex = ( childNodes: NodeListOf<ChildNode>, child: ChildNode | undefined ): number | undefined =>
- sym:54045 replaceContainer (src/jsx/dom/render.ts:760-765)
  const replaceContainer = (node: NodeObject, from: DocumentFragment, to: Container): void =>
- sym:54451 MAX_NESTED_OBJECTS (src/utils/body.ts:10-10)
  const MAX_NESTED_OBJECTS = 10_000
- sym:53773 keyframesCommon (src/helper/css/common.ts:262-290)
  const keyframesCommon = ( strings: TemplateStringsArray, values: CssVariableType[], classNameSlug?: ClassNameSlug, onInvalidSlug?: OnInvalidSlug ): CssClassName =>
- sym:54033 applyProps (src/jsx/dom/render.ts:164-274)
  const applyProps = ( container: SupportedElement, attributes: Props, oldAttributes?: Props ): void =>
- sym:54466 handleParsingNestedValues (src/utils/body.ts:225-259)
  const handleParsingNestedValues = ( form: BodyData, key: string, value: BodyDataValue<Partial<ParseBodyOptions>>, state: { count: number } ): void =>
- sym:54040 applyNodeObject (src/jsx/dom/render.ts:389-483)
  const applyNodeObject = (node: NodeObject, container: Container, isNew: boolean): void =>
- sym:54102 style (src/jsx/intrinsic-element/components.ts:151-161)
  const style: FC<PropsWithChildren<IntrinsicElements['style']>> = ({ children, ...props }) =>
- sym:53933 Fragment (src/jsx/base.ts:452-465)
  const Fragment = ({ children, }: { key?: string children?: Child | HtmlEscapedString }): HtmlEscapedString =>
- sym:53997 cloneElement (src/jsx/dom/index.ts:60-74)
  const cloneElement = <T extends JSXNode | JSX.Element>( element: T, props: Props, ...children: Child[] ): T =>
- sym:53747 CLASS_NAME (src/helper/css/common.ts:7-7)
  const CLASS_NAME: unique symbol = Symbol()
- sym:53748 STYLE_STRING (src/helper/css/common.ts:8-8)
  const STYLE_STRING: unique symbol = Symbol()
- sym:53749 SELECTORS (src/helper/css/common.ts:9-9)
  const SELECTORS: unique symbol = Symbol()
- sym:53750 EXTERNAL_CLASS_NAMES (src/helper/css/common.ts:10-10)
  const EXTERNAL_CLASS_NAMES: unique symbol = Symbol()
- sym:53756 toHash (src/helper/css/common.ts:43-50)
  const toHash = (str: string): string =>
- sym:53757 normalizeLabel (src/helper/css/common.ts:52-54)
  const normalizeLabel = (label: string): string =>
- sym:53758 isValidClassName (src/helper/css/common.ts:56-56)
  const isValidClassName = (name: string): boolean =>
- sym:53762 defaultOnInvalidSlug (src/helper/css/common.ts:75-77)
  const defaultOnInvalidSlug = (slug: string) =>
- sym:53743 renderTest (src/helper/css/common.case.test.tsx:13-506)
  const renderTest = ( getEnv: () => { css: typeof cssHelper keyframes: typeof keyframesHelper viewTransition: typeof viewTransitionHelper rawCssString: typeof rawCssStringHelper // eslint-disable-next-line @typescript-eslint/no-explicit-any toString: (template: any) => Promise<string> Style: typeof StyleComponent support: Support } ) =>

---
tokens: 2211/4000
