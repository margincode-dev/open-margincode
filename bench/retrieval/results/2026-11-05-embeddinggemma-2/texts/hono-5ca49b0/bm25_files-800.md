### src/helper/css/index.ts
```typescript
/**
 * @module
 * css Helper for Hono.
 */

import { raw } from '../../helper/html'
import { DOM_RENDERER } from '../../jsx/constants'
import { createCssJsxDomObjects } from '../../jsx/dom/css'
import { escapeToBuffer } from '../../utils/html'
import type { HtmlEscapedCallback, HtmlEscapedString } from '../../utils/html'
import type {
  ClassNameSlug,
  CssClassName as CssClassNameCommon,
  CssVariableType,
  OnInvalidSlug,
} from './common'
import {
  CLASS_NAME,
  DEFAULT_STYLE_ID,
  PSEUDO_GLOBAL_SELECTOR,
  SELECTOR,
  SELECTORS,
  STYLE_STRING,
  cssCommon,
  cxCommon,
  keyframesCommon,
  viewTransitionCommon,
} from './common'
export { rawCssString } from './common'
export type { ClassNameSlug, OnInvalidSlug } from './common'

type CssClassName = HtmlEscapedString & CssClassNameCommon

type usedClassNameData = [
  Record<string, string>, // class name to add
  Record<string, true>, // class name already added
]

interface CssType {
  (strings: TemplateStringsArray, ...values: CssVariableType[]): Promise<string>
}

interface CxType {
  (
    ...args: (CssClassName | Promise<string> | string | boolean | null | undefined)[]
  ): Promise<string>
}

interface KeyframesType {
  (strings: TemplateStringsArray, ...values: CssVariableType[]): CssClassNameCommon
}

interface ViewTransitionType {
  (strings: TemplateStringsArray, ...values: CssVariableType[]): Promise<string>
  (content: Promise<string>): Promise<string>
  (): Promise<string>
}

interface StyleType {
  (args?: { children?: Promise<string>; nonce?: string }): HtmlEscapedString
}

/**
 * @experimental
 * `createCssContext` is an experimental feature.
 * The API might be changed.
 *
 * @param options.id - The ID for the style element
 * @param options.classNameSlug - Optional function to customize generated CSS class names
 * @param options.onInvalidSlug - Optional callback function called when an invalid slug is returned from ClassNameSlug
 */
export const createCssContext = ({
  id,
  classNameSlug,
  onInvalidSlug,
}: {
  id: Readonly<string>
  classNameSlug?: ClassNameSlug
  onInvalidSlug?: OnInvalidSlug
}): DefaultContextType => {
  const [cssJsxDomObject, StyleRenderToDom] = createCssJsxDomObjects({ id })

  const contextMap: WeakMap<object, usedClassNameData> = new WeakMap()
  const nonceMap: WeakMap<object, string | undefined> = new WeakMap()

  const replaceStyleRe = new RegExp(`(<style id="${id}"(?: nonce="[^"]*")?>.*?)(</style>)`)

  const newCssClassNameObject = (cssClassName: CssClassNameCommon): Promise<string> => {
    const appendStyle: HtmlEscapedCallback = ({ buffer, context }): Promise<string> | undefined => {
      const [toAdd, added] = contextMap.get(context) as usedClassNameData
      const names = Object.keys(toAdd)

      if (!names.length) {
        return
      }

      let stylesStr = ''
      names.forEach((className) => {
        added[className] = true
        stylesStr += className.startsWith(PSEUDO_GLOBAL_SELECTOR)
          ? toAdd[className]
          : `${className[0] === '@' ? '' : '.'}${className}{${toAdd[className]}}`
      })
      contextMap.set(context, [{}, added])

      if (buffer && replaceStyleRe.test(buffer[0])) {
