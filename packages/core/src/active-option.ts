import { findNextEnabledIndex, firstEnabledIndex } from "./nav";

export interface ActiveOption {
  value: string;
  disabled?: boolean;
}

export type ActiveOptionMove = "next" | "previous" | "first" | "last";

/** Move among enabled options. A missing or disabled active value starts at the requested end. */
export function moveActiveOption(
  items: readonly ActiveOption[],
  activeValue: string | null,
  move: ActiveOptionMove,
  wrap = true,
): string | null {
  const flags = items.map((item) => ({ disabled: item.disabled === true }));
  const first = firstEnabledIndex(flags);
  if (first === -1) return null;
  if (move === "first") return items[first]?.value ?? null;
  if (move === "last") return items[findNextEnabledIndex(flags, 0, -1)]?.value ?? null;

  const current = items.findIndex((item) => item.value === activeValue && !item.disabled);
  const direction = move === "next" ? 1 : -1;
  const next = findNextEnabledIndex(flags, current === -1 && direction === -1 ? 0 : current, direction);
  if (!wrap && current !== -1 && (next - current) * direction < 0) return activeValue;
  return items[next]?.value ?? null;
}

/** Keep an enabled identity or use the enabled item nearest its former index, preferring the later item on a tie. */
export function reconcileActiveOption(
  previousItems: readonly ActiveOption[],
  items: readonly ActiveOption[],
  activeValue: string | null,
): string | null {
  if (activeValue === null) return null;
  if (items.some((item) => item.value === activeValue && !item.disabled)) return activeValue;

  const previousIndex = previousItems.findIndex((item) => item.value === activeValue);
  let nearest: string | null = null;
  let distance = Infinity;
  for (const [index, item] of items.entries()) {
    const offset = Math.abs(index - previousIndex);
    if (!item.disabled && offset <= distance) {
      nearest = item.value;
      distance = offset;
    }
  }
  return nearest;
}
