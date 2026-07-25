/**
 * Select behavior machinery.
 * Contract: docs/contracts/components/select.md, "Behavior Machine".
 *
 * Pure option-list and placement logic; lazy loading, query state, and DOM
 * wiring stay adapter-side. Dismissal uses the shared layer stack.
 */

export interface SelectOptionLike {
  value: string;
  label: string;
  disabled?: boolean;
  isDisabled?: boolean;
}

export interface SelectGroupLike<T extends SelectOptionLike = SelectOptionLike> {
  label: string;
  options: T[];
}

export function isSelectOptionDisabled(option: SelectOptionLike): boolean {
  return option.disabled === true || option.isDisabled === true;
}

/** Grouped or flat items → flat option list. */
export function flattenSelectOptions<T extends SelectOptionLike>(
  items: readonly (T | SelectGroupLike<T>)[],
): T[] {
  const first = items[0];

  if (first === undefined) {
    return [];
  }

  if ("options" in first) {
    return (items as SelectGroupLike<T>[]).flatMap((group) => group.options);
  }

  return [...(items as T[])];
}

/** Enabled options whose label contains the query (case-insensitive); empty query keeps all enabled. */
export function filterSelectOptions<T extends SelectOptionLike>(options: readonly T[], query: string): T[] {
  const enabled = options.filter((option) => !isSelectOptionDisabled(option));

  if (query.length === 0) {
    return enabled;
  }

  const lowered = query.toLowerCase();

  return enabled.filter((option) => option.label.toLowerCase().includes(lowered));
}

/** Per-group label filter; groups left empty by the filter are dropped. */
export function filterSelectGroups<T extends SelectOptionLike>(
  groups: readonly SelectGroupLike<T>[],
  query: string,
): SelectGroupLike<T>[] {
  const lowered = query.toLowerCase();

  return groups
    .map((group) => ({
      ...group,
      options: group.options.filter((option) => option.label.toLowerCase().includes(lowered)),
    }))
    .filter((group) => group.options.length > 0);
}

/** Highlight index on open: the selected option when present, else the first. */
export function selectOpenHighlightIndex<T extends SelectOptionLike>(
  filtered: readonly T[],
  selectedValue: string | null,
): number {
  if (selectedValue === null) {
    return 0;
  }

  const index = filtered.findIndex((option) => option.value === selectedValue);

  return index < 0 ? 0 : index;
}
