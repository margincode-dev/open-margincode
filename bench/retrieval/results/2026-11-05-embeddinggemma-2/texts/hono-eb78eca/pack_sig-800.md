## Task
# hono-eb78eca · hono-eb78eca · open

## Map
- repo map: .editorconfig .github/ .gitignore .tool-versions .vscode/ LICENSE README.md adapters/ benchmarks/ build/ bunfig.toml codecov.yml (583 files)

## Code
- sym:44929 _getQueryParam (src/utils/url.ts:232-318)
  const _getQueryParam = ( url: string, key?: string, multiple?: boolean ): string | undefined | Record<string, string> | string[] | Record<string, string[]> =>
- sym:44587 HonoRequest.query (src/request.ts:147-149)
  query(key?: string)
- sym:44477 DetectorOptions (src/middleware/language/language.ts:13-45)
  interface DetectorOptions
- sym:43584 EventV1Processor.getQueryString (adapters/aws-lambda/src/handler.ts:456-472)
  protected getQueryString(event: APIGatewayProxyEvent): string
- sym:44588 HonoRequest.queries (src/request.ts:166-168)
  queries(key?: string)
- sym:43757 EventV1Processor.getQueryString (src/adapter/aws-lambda/handler.ts:487-503)
  protected getQueryString(event: APIGatewayProxyEvent): string
- sym:44584 HonoRequest.#getDecodedParam (src/request.ts:103-107)
  #getDecodedParam(key: string): string | undefined
- sym:43913 buildSearchParams (src/client/utils.ts:26-47)
  const buildSearchParams = (query: Record<string, string | string[]>) =>
- sym:44414 createCacheKey (src/middleware/cache/index.ts:51-70)
  const createCacheKey = ( key: string, requestUrl: string, request: CacheKeyRequest, varyHeaders: [string, string][] ): string =>
- sym:43581 EventV1Processor (adapters/aws-lambda/src/handler.ts:447-503)
  class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
- sym:44930 getQueryParams (src/utils/url.ts:328-333)
  const getQueryParams = ( url: string, key?: string ): string[] | undefined | Record<string, string[]> =>
- sym:43754 EventV1Processor (src/adapter/aws-lambda/handler.ts:478-534)
  class EventV1Processor extends EventProcessor<APIGatewayProxyEvent>
- sym:43873 BuildSearchParamsFn (src/client/types.ts:30-30)
- sym:44583 HonoRequest.param (src/request.ts:99-101)
  param(key?: string): unknown
- sym:44923 getQueryStrings (src/utils/url.ts:140-152)
- sym:43557 APIGatewayProxyEvent (adapters/aws-lambda/src/handler.ts:65-83)
- sym:43571 EventProcessor.createRequest (adapters/aws-lambda/src/handler.ts:322-346)
- sym:43730 APIGatewayProxyEvent (src/adapter/aws-lambda/handler.ts:77-95)
- sym:43744 EventProcessor.createRequest (src/adapter/aws-lambda/handler.ts:347-371)

---
tokens: 685/800
