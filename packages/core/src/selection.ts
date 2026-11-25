export type SelectionEvent<T> =
  | { type: "SELECT" | "TOGGLE"; item: T; disabled?: boolean }
  | { type: "REMOVE"; item: T }
  | { type: "CLEAR" }
  | { type: "REPLACE"; items: readonly T[] };

export interface SelectionOptions<T> {
  mode?: "single" | "multiple";
  max?: number;
  key?: (item: T) => string;
}

/** Update an ordered selection independently of which items are visible. */
export function selectionTransition<T extends string | object>(
  selected: readonly T[],
  event: SelectionEvent<T>,
  { mode = "single", max = Infinity, key }: SelectionOptions<T> = {},
): readonly T[] {
  const identity: (item: T) => string | T = key ?? ((item: T): T => item);
  const limit = Math.min(mode === "single" ? 1 : Infinity, Math.max(0, Math.floor(max) || 0));
  if (event.type === "REPLACE") {
    const next: T[] = [];
    const seen = new Set<T | string>();
    for (const item of event.items) {
      if (next.length === limit) break;
      const itemKey = identity(item);
      if (seen.has(itemKey)) continue;
      seen.add(itemKey);
      next.push(item);
    }
    return next;
  }
  if (event.type === "CLEAR") return selected.length === 0 ? selected : [];
  if (event.type !== "REMOVE" && event.disabled) return selected;

  const itemKey = identity(event.item);
  const exists = selected.some((item) => identity(item) === itemKey);
  if (event.type === "REMOVE" || (event.type === "TOGGLE" && exists)) {
    return exists ? selected.filter((item) => identity(item) !== itemKey) : selected;
  }
  if (exists || limit === 0) return selected;
  if (mode === "single") return [event.item];
  return selected.length < limit ? [...selected, event.item] : selected;
}
