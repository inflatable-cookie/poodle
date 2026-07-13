/**
 * Adapt a headless parts attribute record to React DOM props: the core emits
 * lowercase HTML attribute names (`tabindex`); React wants camelCase for the
 * handful it renames.
 */
export function reactifyPart(part: Record<string, unknown>): Record<string, unknown> {
  const { tabindex, class: className, ...rest } = part as Record<string, unknown> & {
    tabindex?: number;
    class?: string;
  };
  return {
    ...rest,
    ...(tabindex !== undefined ? { tabIndex: tabindex } : null),
    ...(className !== undefined ? { className } : null),
  };
}
