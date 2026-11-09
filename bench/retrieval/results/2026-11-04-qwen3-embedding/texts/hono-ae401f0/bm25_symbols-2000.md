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
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-4027435072{background-color:blue;content:\'I\\\'m a variable!\'}</style><h1 class="css-4027435072">Hello!</h1>'
      )
    })

    it('Should escape </style>', async () => {
      const headerClass = css`
        background-color: blue;
        content: '${'</style>'}';
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-372954897{background-color:blue;content:\'<\\/style>\'}</style><h1 class="css-372954897">Hello!</h1>'
      )
    })

    it('Should not escape URL', async () => {
      const headerClass = css`
        background-color: blue;
        background: url('${'http://www.example.com/path/to/file.jpg'}');
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-1321888780{background-color:blue;background:url(\'http://www.example.com/path/to/file.jpg\')}</style><h1 class="css-1321888780">Hello!</h1>'
      )
    })

    it('Should render CSS with escaped variable', async () => {
      const headerClass = css`
        background-color: blue;
        content: '${rawCssString('say "Hello!"')}';
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-2238574885{background-color:blue;content:\'say "Hello!"\'}</style><h1 class="css-2238574885">Hello!</h1>'
      )
    })

    it('Should render CSS with number', async () => {
      const headerClass = css`
        background-color: blue;
        font-size: ${1}rem;
      `
      const template = (
        <>
          <Style />
          <h1 class={headerClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-1847536026{background-color:blue;font-size:1rem}</style><h1 class="css-1847536026">Hello!</h1>'
      )
    })

    it('Should render CSS with array', async () => {
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
      const extendedHeaderClass = css`
        ${headerClass}
        color: red;
      `
      const template = (
        <>
          <Style />
          <h1 class={extendedHeaderClass}>Hello!</h1>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">.css-2558359670{background-color:blue;animation:css-9294673 1s ease-in-out;color:red}@keyframes css-9294673{from{opacity:0}to{opacity:1}}</style><h1 class="css-2558359670">Hello!</h1>'
      )
    })

    it.runIf(support.nest)(
      'Should be used as a class name for syntax `${className} {`',
      async () => {
        const headerClass = css`
          font-weight: bold;
        `
        const containerClass = css`
          ${headerClass} {
            h1 {
              color: red;
            }
          }
        `
        const template = (
          <>
            <Style />
            <div class={containerClass}>
              <h1 class={headerClass}>Hello!</h1>
            </div>
          </>
        )
        expect(await toString(template)).toBe(
          '<style id="hono-css">.css-4220297002{.css-1032195302{h1{color:red}}}.css-1032195302{font-weight:bold}</style><div class="css-4220297002"><h1 class="css-1032195302">Hello!</h1></div>'
        )
      }
    )

    it('Should be inserted to global if style string starts with :-hono-root', async () => {
      const globalClass = css`
        :-hono-global {
          html {
            color: red;
          }
          body {
            display: flex;
          }
        }
      `
      const template = (
        <>
          <Style />
          <div class={globalClass}>
            <h1>Hello!</h1>
          </div>
        </>
      )
      expect(await toString(template)).toBe(
        '<style id="hono-css">html{color:red}body{display:flex}</style><div class=""><h1>Hello!</h1></div>'
      )
    })

    it.runIf(support.nest)(
      'Should be inserted to global if style string starts with :-hono-root and extends class name',
      async () => {
        const headerClass = css`
          display: flex;
        `
        const specialHeaderClass = css`
          :-hono-global {
            ${headerClass} {
