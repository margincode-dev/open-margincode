// Bearer tokens and when to refresh them.
export interface Token {
  value: string;
  expiresAt: number;
}

export function isTokenExpired(token: Token, now = Date.now()): boolean {
  return token.expiresAt <= now;
}

export function authHeader(token: Token): Record<string, string> {
  return { Authorization: "Bearer " + token.value };
}

export async function refreshToken(
  fetchToken: () => Promise<Token>,
  current: Token | undefined,
  now = Date.now(),
): Promise<Token> {
  if (current && !isTokenExpired(current, now)) return current;
  return fetchToken();
}
