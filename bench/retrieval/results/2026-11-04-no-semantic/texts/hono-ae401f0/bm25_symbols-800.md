- PSEUDO_GLOBAL_SELECTOR (src/helper/css/common.ts:2-2)
```typescript
export const PSEUDO_GLOBAL_SELECTOR = ':-hono-global'
```
- renderTest (src/helper/css/common.case.test.tsx:13-506)
```typescript
export const renderTest = (
  getEnv: () => {
    css: typeof cssHelper
    keyframes: typeof keyframesHelper
    viewTransition: typeof viewTransitionHelper
    rawCssString: typeof rawCssStringHelper
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    toString: (template: any) => Promise<string>
    Style: typeof StyleComponent
    support: Support
  }
) => {
  const { support } = getEnv()

  let css: typeof cssHelper
  let keyframes: typeof keyframesHelper
  let viewTransition: typeof viewTransitionHelper
  let rawCssString: typeof rawCssStringHelper
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let toString: (template: any) => Promise<string>
  let Style: typeof StyleComponent
  beforeEach(() => {
    ;({ css, keyframes, viewTransition, rawCssString, toString, Style } = getEnv())
  })

  describe('render css', () => {
    it('Should render CSS styles with JSX', async () => {
      const headerClass = css`
        background-color: blue;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-2458908649{background-color:blue}</style><h1 class="css-2458908649">Hello!</h1>'
      )
    })

    it('Should render CSS with keyframes', async () => {
      const animation = keyframes`
        from {
          opacity: 0;
        }
        to {
          opacity: 1;
        }
      `
      const headerClass = css`
        background-color: blue;
        animation: ${animation} 1s ease-in-out;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-1580801783{background-color:blue;animation:css-9294673 1s ease-in-out}@keyframes css-9294673{from{opacity:0}to{opacity:1}}</style><h1 class="css-1580801783">Hello!</h1>'
      )
    })

    it('Should not output the same class name multiple times.', async () => {
      const headerClass = css`
        background-color: blue;
      `
      const headerClass2 = css`
        background-color: blue;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
          <h1 class={headerClass2}>Hello2!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-2458908649{background-color:blue}</style><h1 class="css-2458908649">Hello!</h1><h1 class="css-2458908649">Hello2!</h1>'
      )
    })

    it('Should render CSS with variable', async () => {
      const headerClass = css`
        background-color: blue;
        content: '${"I'm a variable!"}';
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
