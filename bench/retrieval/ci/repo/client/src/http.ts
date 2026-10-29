// Thin wrapper around fetch: URLs, timeouts and retry hints.
export interface RequestOptions {
  method?: string;
  query?: Record<string, string>;
  timeoutMs?: number;
}

export function buildUrl(base: string, path: string, query: Record<string, string> = {}): string {
  const url = new URL(path, base);
  for (const [k, v] of Object.entries(query)) {
    url.searchParams.set(k, v);
  }
  return url.toString();
}

export function parseRetryAfter(header: string | null): number | null {
  if (header === null) return null;
  const seconds = Number(header);
  if (Number.isNaN(seconds)) return null;
  return seconds * 1000;
}

export function withTimeout<T>(work: Promise<T>, timeoutMs: number): Promise<T> {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("request timed out")), timeoutMs);
    work.then(
      (v) => {
        clearTimeout(timer);
        resolve(v);
      },
      (e) => {
        clearTimeout(timer);
        reject(e);
      },
    );
  });
}

export async function request(base: string, path: string, opts: RequestOptions = {}): Promise<Response> {
  const url = buildUrl(base, path, opts.query);
  return withTimeout(fetch(url, { method: opts.method ?? "GET" }), opts.timeoutMs ?? 10000);
}
