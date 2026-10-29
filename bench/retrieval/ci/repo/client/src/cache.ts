// An in-memory response cache keyed by method and URL.
interface Entry {
  body: string;
  expiresAt: number;
}

export function cacheKey(method: string, url: string): string {
  return method.toUpperCase() + " " + url;
}

export class ResponseCache {
  private entries = new Map<string, Entry>();

  constructor(private ttlMs: number) {}

  get(method: string, url: string, now = Date.now()): string | undefined {
    const entry = this.entries.get(cacheKey(method, url));
    if (!entry || entry.expiresAt <= now) return undefined;
    return entry.body;
  }

  set(method: string, url: string, body: string, now = Date.now()): void {
    this.entries.set(cacheKey(method, url), { body, expiresAt: now + this.ttlMs });
  }

  evictExpired(now = Date.now()): number {
    let removed = 0;
    for (const [key, entry] of this.entries) {
      if (entry.expiresAt <= now) {
        this.entries.delete(key);
        removed++;
      }
    }
    return removed;
  }
}
