type FlatToken = {
  path: string;
  value: string | number;
};

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

export function flattenTokens(
  value: Record<string, unknown>,
  prefix = "",
): FlatToken[] {
  const entries: FlatToken[] = [];

  for (const [key, child] of Object.entries(value)) {
    const path = prefix ? `${prefix}.${key}` : key;

    if (isPlainObject(child)) {
      entries.push(...flattenTokens(child, path));
      continue;
    }

    if (typeof child === "string" || typeof child === "number") {
      entries.push({ path, value: child });
    }
  }

  return entries;
}
