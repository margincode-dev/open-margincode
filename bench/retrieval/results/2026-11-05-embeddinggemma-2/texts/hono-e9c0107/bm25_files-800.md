### src/hono-base.ts
```typescript
/**
 * @module
 * This module is the base module for the Hono object.
 */

/* eslint-disable @typescript-eslint/no-explicit-any */
import { compose } from './compose'
import { Context } from './context'
import type { ExecutionContext } from './context'
import type { Router } from './router'
import { METHODS, METHOD_NAME_ALL, METHOD_NAME_ALL_LOWERCASE } from './router'
import type {
  Env,
  ErrorHandler,
  FetchEventLike,
  H,
  HandlerInterface,
  MergePath,
  MergeSchemaPath,
  MiddlewareHandler,
  MiddlewareHandlerInterface,
  Next,
  NotFoundHandler,
  OnHandlerInterface,
  RouterRoute,
  Schema,
} from './types'
import { COMPOSED_HANDLER } from './utils/constants'
import { getPath, getPathNoStrict, mergePath } from './utils/url'

const notFoundHandler: NotFoundHandler = (c) => {
  return c.text('404 Not Found', 404)
}

const errorHandler: ErrorHandler = (err, c) => {
  if ('getResponse' in err) {
    const res = err.getResponse()
    return c.newResponse(res.body, res)
  }
  console.error(err)
  return c.text('Internal Server Error', 500)
}

type GetPath<E extends Env> = (request: Request, options?: { env?: E['Bindings'] }) => string

export type HonoOptions<E extends Env> = {
  /**
   * `strict` option specifies whether to distinguish whether the last path is a directory or not.
   *
   * @see {@link https://hono.dev/docs/api/hono#strict-mode}
   *
   * @default true
   */
  strict?: boolean
  /**
   * `router` option specifies which router to use.
   *
   * @see {@link https://hono.dev/docs/api/hono#router-option}
   *
   * @example
   * ```ts
   * const app = new Hono({ router: new RegExpRouter() })
   * ```
   */
  router?: Router<[H, RouterRoute]>
  /**
   * `getPath` can handle the host header value.
   *
   * @see {@link https://hono.dev/docs/api/routing#routing-with-host-header-value}
   *
   * @example
   * ```ts
   * const app = new Hono({
   *  getPath: (req) =>
   *   '/' + req.headers.get('host') + req.url.replace(/^https?:\/\/[^/]+(\/[^?]*)/, '$1'),
   * })
   *
   * app.get('/www1.example.com/hello', () => c.text('hello www1'))
   *
   * // A following request will match the route:
   * // new Request('http://www1.example.com/hello', {
   * //  headers: { host: 'www1.example.com' },
   * // })
   * ```
   */
  getPath?: GetPath<E>
}

type MountOptionHandler = (c: Context) => unknown
type MountReplaceRequest = (originalRequest: Request) => Request
type MountOptions =
  | MountOptionHandler
  | {
      optionHandler?: MountOptionHandler
      replaceRequest?: MountReplaceRequest | false
    }

class Hono<
  E extends Env = Env,
  S extends Schema = {},
  BasePath extends string = '/',
  CurrentPath extends string = BasePath,
> {
  get!: HandlerInterface<E, 'get', S, BasePath, CurrentPath>
  post!: HandlerInterface<E, 'post', S, BasePath, CurrentPath>
  put!: HandlerInterface<E, 'put', S, BasePath, CurrentPath>
  delete!: HandlerInterface<E, 'delete', S, BasePath, CurrentPath>
