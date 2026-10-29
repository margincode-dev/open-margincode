// Following paginated list endpoints.
export interface Page<T> {
  items: T[];
  next: string | null;
}

export function nextPage<T>(page: Page<T>): string | null {
  return page.next;
}

export async function collectAll<T>(
  first: string,
  load: (cursor: string) => Promise<Page<T>>,
  limit = 1000,
): Promise<T[]> {
  const all: T[] = [];
  let cursor: string | null = first;
  while (cursor !== null && all.length < limit) {
    const page = await load(cursor);
    all.push(...page.items);
    cursor = nextPage(page);
  }
  return all;
}
