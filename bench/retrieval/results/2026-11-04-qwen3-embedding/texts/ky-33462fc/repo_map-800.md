source/types/hooks.ts:
  L30: export type InitHook = (options: InitOptions) => void;
  L42: export type BeforeRequestHook = (state: BeforeRequestState) => Request | Response | void | Promise<Request | Response | void>;
  L55: export type BeforeRetryHook = (state: BeforeRetryState) => Request | Response | typeof stop | void | Promise<Request | Response | typeof stop | void>;
  L73: export type BeforeErrorHook = (state: BeforeErrorState) => Error | Promise<Error>;
  L88: export type AfterResponseHook = (state: AfterResponseState) => Response | RetryMarker | void | Promise<Response | RetryMarker | void>;
  L32: export type BeforeRequestState = {
  L44: export type BeforeRetryState = {
  L57: export type BeforeErrorState = {
  L75: export type AfterResponseState = {
  L90: export type Hooks = {
  L376: export type NormalizedHooks = {[Key in keyof Hooks]-?: Array<NonNullable<Hooks[Key]>[number]>};
source/types/options.ts:
  L6: export type SearchParamsInit = string | string[][] | Record<string, string> | URLSearchParams | undefined;
  L10: export type SearchParamsOption =
  L16: export type RequestHttpMethod = 'get' | 'post' | 'put' | 'patch' | 'head' | 'delete' | 'query';
  L17: export type HttpMethod = LiteralUnion<RequestHttpMethod | 'options' | 'trace', string>;
  L19: export type Input = string | URL | Request;
  L21: export type Progress = {
  L39: export type KyHeadersInit = NonNullable<RequestInit['headers']> | Record<string, string | undefined> | ReadonlyArray<readonly [string, string]>;
  L45: export type KyOptions = {
  L439: export type KyOptionsRegistry = {[K in keyof KyOptions]-?: true};
  L441: type RequestOptions = {
  L448: export interface Options extends KyOptions, RequestOptions { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
  L522: export type InitOptions = Omit<Options, 'retry' | 'hooks' | 'headers' | 'context'> & {
  L531: type NormalizedRetryOptions = {
  L535: export type InternalOptions = Omit<Options, 'hooks' | 'retry' | 'context' | 'throwHttpErrors'> & {
  L551: export interface NormalizedOptions extends Readonly<RequestInit> { // eslint-disable-line @typescript-eslint/consistent-type-definitions -- This must stay an interface so that it can be extended outside of Ky for use in `ky.create`.
source/utils/delay.ts:
  L5: export type DelayOptions = {
  L9: export default async function delay(
source/core/constants.ts:
  L40: const validate = <T extends Array<true>>() => undefined as unknown as T;
  L73: export type ForceRetryOptions = {
  L157: export class RetryMarker {
  L159: get isRetryMarker(): true {
  L165: constructor(options?: ForceRetryOptions) {
  L170: export const isRetryMarker = (value: unknown): value is RetryMarker =>
  L256: export const retry = (options?: ForceRetryOptions) => new RetryMarker(options);
source/types/request.ts:
  L1: export type KyRequest<T = unknown> = {
