/**
 * Select behavior machinery.
 * Contract: docs/contracts/components/select.md, "Behavior Machine".
 *
 * Pure option-list helpers plus the open/query/highlight/value transition.
 * Async loading, snippets, placement, portals, and DOM/native focus stay
 * adapter-owned.
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

export interface SelectOptionState {
  value: string;
  label: string;
  disabled: boolean;
}

export interface SelectContext {
  value: string;
  open: boolean;
  query: string;
  highlightedValue: string | null;
  options: SelectOptionState[];
  clearValue: string;
  searchable: boolean;
  freeform: boolean;
  disabled: boolean;
}

export type SelectEvent =
  | { type: "OPEN" }
  | { type: "CLOSE" }
  | { type: "TOGGLE" }
  | { type: "QUERY"; query: string }
  | { type: "HIGHLIGHT"; value: string }
  | { type: "HIGHLIGHT_PREV" }
  | { type: "HIGHLIGHT_NEXT" }
  | { type: "HIGHLIGHT_FIRST" }
  | { type: "HIGHLIGHT_LAST" }
  | { type: "COMMIT_HIGHLIGHTED" }
  | { type: "COMMIT_OPTION"; value: string }
  | { type: "COMMIT_FREEFORM" }
  | { type: "CLEAR" };

export type SelectEffect =
  | { type: "openChanged"; open: boolean }
  | { type: "queryChanged"; query: string }
  | { type: "valueChanged"; value: string };

export interface SelectResult {
  context: SelectContext;
  effects: SelectEffect[];
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

export function selectFreeformEnabled(context: SelectContext): boolean {
  return context.searchable && context.freeform;
}

export function selectMatchesQuery(label: string, query: string): boolean {
  if (query.length === 0) {
    return true;
  }

  return label.toLowerCase().includes(query.toLowerCase());
}

/** Visible options for the current query, including disabled rows. */
export function selectVisibleOptions(context: SelectContext): SelectOptionState[] {
  return context.options.filter((option) => selectMatchesQuery(option.label, context.query));
}

export function selectEnabledVisibleValues(context: SelectContext): string[] {
  return selectVisibleOptions(context)
    .filter((option) => !option.disabled)
    .map((option) => option.value);
}

export function selectCommittedQuery(context: SelectContext): string {
  if (context.value === context.clearValue) {
    return "";
  }

  return context.options.find((option) => option.value === context.value)?.label ?? "";
}

/** Opening highlight: selected enabled visible option, else first enabled visible, else null. */
export function selectOpenHighlightValue(context: SelectContext): string | null {
  const enabled = selectEnabledVisibleValues(context);

  if (enabled.length === 0) {
    return null;
  }

  if (context.value !== context.clearValue && enabled.includes(context.value)) {
    return context.value;
  }

  return enabled[0] ?? null;
}

function inert(context: SelectContext): SelectResult {
  return { context, effects: [] };
}

function pushOpen(effects: SelectEffect[], previous: boolean, next: boolean): void {
  if (previous !== next) {
    effects.push({ type: "openChanged", open: next });
  }
}

function pushQuery(effects: SelectEffect[], previous: string, next: string): void {
  if (previous !== next) {
    effects.push({ type: "queryChanged", query: next });
  }
}

function pushValue(effects: SelectEffect[], previous: string, next: string): void {
  if (previous !== next) {
    effects.push({ type: "valueChanged", value: next });
  }
}

function orderedEffects(
  previous: SelectContext,
  next: Pick<SelectContext, "open" | "query" | "value">,
  includeQuery: boolean,
): SelectEffect[] {
  const effects: SelectEffect[] = [];

  pushOpen(effects, previous.open, next.open);

  if (includeQuery) {
    pushQuery(effects, previous.query, next.query);
  }

  pushValue(effects, previous.value, next.value);

  return effects;
}

function openList(context: SelectContext): SelectResult {
  if (context.open) {
    return inert(context);
  }

  const next: SelectContext = {
    ...context,
    open: true,
    highlightedValue: selectOpenHighlightValue({ ...context, open: true }),
  };

  return { context: next, effects: orderedEffects(context, next, false) };
}

function closeList(context: SelectContext): SelectResult {
  if (!context.open) {
    return inert(context);
  }

  const next: SelectContext = {
    ...context,
    open: false,
    query: selectFreeformEnabled(context) ? context.query : selectCommittedQuery(context),
  };

  return { context: next, effects: orderedEffects(context, next, false) };
}

function findOption(context: SelectContext, value: string): SelectOptionState | undefined {
  return context.options.find((option) => option.value === value);
}

function moveHighlight(context: SelectContext, direction: 1 | -1): SelectResult {
  if (!context.open) {
    return openList(context);
  }

  const enabled = selectEnabledVisibleValues(context);

  if (enabled.length === 0) {
    if (context.highlightedValue === null) {
      return inert(context);
    }

    return { context: { ...context, highlightedValue: null }, effects: [] };
  }

  const currentIndex = context.highlightedValue === null ? -1 : enabled.indexOf(context.highlightedValue);
  let nextIndex: number;

  if (currentIndex < 0) {
    nextIndex = direction === 1 ? 0 : enabled.length - 1;
  } else {
    nextIndex = Math.max(0, Math.min(enabled.length - 1, currentIndex + direction));
  }

  const highlightedValue = enabled[nextIndex] ?? null;

  if (highlightedValue === context.highlightedValue) {
    return inert(context);
  }

  return { context: { ...context, highlightedValue }, effects: [] };
}

function jumpHighlight(context: SelectContext, position: "first" | "last"): SelectResult {
  if (!context.open) {
    return inert(context);
  }

  const enabled = selectEnabledVisibleValues(context);
  const highlightedValue = position === "first" ? (enabled[0] ?? null) : (enabled[enabled.length - 1] ?? null);

  if (highlightedValue === context.highlightedValue) {
    return inert(context);
  }

  return { context: { ...context, highlightedValue }, effects: [] };
}

function commitOption(context: SelectContext, value: string): SelectResult {
  const option = findOption(context, value);

  if (!option || option.disabled) {
    return inert(context);
  }

  const next: SelectContext = {
    ...context,
    value: option.value,
    query: option.label,
    open: false,
    highlightedValue: option.value,
  };

  return { context: next, effects: orderedEffects(context, next, true) };
}

function commitFreeform(context: SelectContext): SelectResult {
  if (!selectFreeformEnabled(context) || context.highlightedValue !== null || context.query === context.value) {
    return inert(context);
  }

  const next: SelectContext = {
    ...context,
    value: context.query,
    open: false,
    highlightedValue: null,
  };

  return { context: next, effects: orderedEffects(context, next, false) };
}

export function selectTransition(context: SelectContext, event: SelectEvent): SelectResult {
  if (context.disabled) {
    return inert(context);
  }

  switch (event.type) {
    case "OPEN":
      return openList(context);
    case "CLOSE":
      return closeList(context);
    case "TOGGLE":
      return context.open ? closeList(context) : openList(context);
    case "QUERY": {
      const next: SelectContext = {
        ...context,
        query: event.query,
        open: true,
      };
      next.highlightedValue = selectOpenHighlightValue(next);

      return { context: next, effects: orderedEffects(context, next, true) };
    }
    case "HIGHLIGHT": {
      const option = findOption(context, event.value);

      if (!option || option.disabled || !selectMatchesQuery(option.label, context.query)) {
        return inert(context);
      }

      if (context.highlightedValue === option.value) {
        return inert(context);
      }

      return { context: { ...context, highlightedValue: option.value }, effects: [] };
    }
    case "HIGHLIGHT_PREV":
      return moveHighlight(context, -1);
    case "HIGHLIGHT_NEXT":
      return moveHighlight(context, 1);
    case "HIGHLIGHT_FIRST":
      return jumpHighlight(context, "first");
    case "HIGHLIGHT_LAST":
      return jumpHighlight(context, "last");
    case "COMMIT_HIGHLIGHTED":
      if (context.highlightedValue !== null) {
        return commitOption(context, context.highlightedValue);
      }

      return commitFreeform(context);
    case "COMMIT_OPTION":
      return commitOption(context, event.value);
    case "COMMIT_FREEFORM":
      return commitFreeform(context);
    case "CLEAR": {
      const next: SelectContext = {
        ...context,
        value: context.clearValue,
        query: "",
        highlightedValue: context.open ? selectOpenHighlightValue({ ...context, value: context.clearValue, query: "" }) : null,
      };

      return { context: next, effects: orderedEffects(context, next, true) };
    }
  }
}
