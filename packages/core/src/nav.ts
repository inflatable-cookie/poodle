/**
 * Roving-tabindex index navigation: wrapping, disabled-skipping.
 * Extracted from packages/svelte/components/src/internal.ts.
 */

interface MaybeDisabled {
  disabled?: boolean;
  isDisabled?: boolean;
}

function isEnabled(item: MaybeDisabled | undefined): boolean {
  return item !== undefined && !item.disabled && !item.isDisabled;
}

/**
 * Next enabled index in `direction`, wrapping modulo item count and skipping
 * disabled items. Returns `startIndex` when no other enabled item exists, and
 * `-1` for an empty list.
 */
export function findNextEnabledIndex<T extends MaybeDisabled>(
  items: T[],
  startIndex: number,
  direction: 1 | -1,
): number {
  const count = items.length;

  if (count === 0) {
    return -1;
  }

  let index = startIndex;

  for (let step = 0; step < count; step += 1) {
    index = (index + direction + count) % count;

    if (isEnabled(items[index])) {
      return index;
    }
  }

  return startIndex;
}

export function firstEnabledIndex<T extends MaybeDisabled>(items: T[]): number {
  return items.findIndex((item) => isEnabled(item));
}
