<script context="module" lang="ts">
  let nextCalendarId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import {
    addDays,
    addMonths,
    buildCalendarWeeks,
    compareIsoDate,
    dayDeltaForWeekBoundary,
    formatDateLabel,
    formatIsoDate,
    formatMonthLabel,
    getWeekdayLabels,
    isIsoDateWithinRange,
    monthAnchorIso,
    normalizeDateRange,
    parseIsoDate,
    todayIsoDate,
  } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlDensity, ControlSize, DateRangeValue, SemanticControlSizeRole } from "./types";

  export let mode: "single" | "range" = "single";
  export let value: string | DateRangeValue | null = null;
  export let defaultValue: string | DateRangeValue | null = null;
  export let visibleMonth: string | null = null;
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string } | { value: DateRangeValue };
    monthChange: { month: string };
  }>();

  const gridId = `poodle-calendar-grid-${++nextCalendarId}`;
  const uiPresentation = getUiPresentation();

  // Single mode state
  let uncontrolledSingleValue: string | null =
    mode === "single" && typeof defaultValue === "string" ? defaultValue : null;

  // Range mode state
  let uncontrolledRangeValue: DateRangeValue =
    mode === "range" && defaultValue !== null && typeof defaultValue === "object"
      ? normalizeDateRange(defaultValue as DateRangeValue)
      : { start: null, end: null };

  let uncontrolledMonth = monthAnchorIso(
    visibleMonth ??
    (mode === "range"
      ? (typeof defaultValue === "object" && defaultValue !== null ? (defaultValue as DateRangeValue).start : null) ?? todayIsoDate()
      : (typeof defaultValue === "string" ? defaultValue : null) ?? todayIsoDate())
  );

  let focusIso = mode === "range"
    ? (typeof defaultValue === "object" && defaultValue !== null ? (defaultValue as DateRangeValue).start : null) ?? todayIsoDate()
    : (typeof defaultValue === "string" ? defaultValue : null) ?? todayIsoDate();

  let dayElements: Record<string, HTMLButtonElement | undefined> = {};

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  // Resolved current values based on mode
  $: currentSingleValue = mode === "single"
    ? (typeof value === "string" ? value : null) ?? uncontrolledSingleValue
    : null;
  $: currentRangeValue = mode === "range"
    ? normalizeDateRange(
        (value !== null && typeof value === "object" ? value as DateRangeValue : null) ?? uncontrolledRangeValue
      )
    : { start: null, end: null };

  $: currentMonth = monthAnchorIso(visibleMonth ?? uncontrolledMonth);
  $: weeks = buildCalendarWeeks(currentMonth, weekStartsOn);
  $: weekdayLabels = getWeekdayLabels(weekStartsOn, locale);
  $: monthLabel = formatMonthLabel(currentMonth, locale);

  // Update focusIso when values change
  $: if (mode === "single" && currentSingleValue) {
    focusIso = currentSingleValue;
  }
  $: if (mode === "range") {
    if (currentRangeValue.end) {
      focusIso = currentRangeValue.end;
    } else if (currentRangeValue.start) {
      focusIso = currentRangeValue.start;
    }
  }

  // Day state helpers for range mode
  function isRangeStart(iso: string): boolean {
    return mode === "range" && currentRangeValue.start === iso;
  }
  function isRangeEnd(iso: string): boolean {
    return mode === "range" && currentRangeValue.end === iso;
  }
  function isInRange(iso: string): boolean {
    return mode === "range" && isIsoDateWithinRange(iso, currentRangeValue);
  }
  function isSelected(iso: string): boolean {
    if (mode === "single") {
      return currentSingleValue === iso;
    }
    return isRangeStart(iso) || isRangeEnd(iso);
  }

  function setMonth(nextMonth: string): void {
    if (visibleMonth === null) {
      uncontrolledMonth = nextMonth;
    }

    dispatch("monthChange", { month: nextMonth });
  }

  function selectDate(iso: string): void {
    if (disabled) {
      return;
    }

    if (mode === "single") {
      if (value === null) {
        uncontrolledSingleValue = iso;
      }

      focusIso = iso;
      dispatch("valueChange", { value: iso });
    } else {
      // Range mode: two-click selection
      if (!currentRangeValue.start || currentRangeValue.end) {
        commitRange({ start: iso, end: null });
        return;
      }

      if (compareIsoDate(iso, currentRangeValue.start) < 0) {
        commitRange({ start: iso, end: currentRangeValue.start });
        return;
      }

      commitRange({ start: currentRangeValue.start, end: iso });
    }
  }

  function commitRange(nextValue: DateRangeValue): void {
    const normalized = normalizeDateRange(nextValue);

    if (value === null) {
      uncontrolledRangeValue = normalized;
    }

    focusIso = normalized.end ?? normalized.start ?? focusIso;
    dispatch("valueChange", { value: normalized });
  }

  async function focusDate(iso: string): Promise<void> {
    focusIso = iso;
    const nextMonth = monthAnchorIso(iso);

    if (nextMonth !== currentMonth) {
      setMonth(nextMonth);
      await tick();
    }

    dayElements[iso]?.focus();
  }

  function handleDayKeydown(event: KeyboardEvent, iso: string): void {
    const date = parseIsoDate(iso);

    if (!date) {
      return;
    }

    let nextDate = date;

    if (event.key === "ArrowRight") {
      event.preventDefault();
      nextDate = addDays(date, 1);
    } else if (event.key === "ArrowLeft") {
      event.preventDefault();
      nextDate = addDays(date, -1);
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      nextDate = addDays(date, 7);
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      nextDate = addDays(date, -7);
    } else if (event.key === "Home") {
      event.preventDefault();
      nextDate = addDays(date, dayDeltaForWeekBoundary(iso, weekStartsOn, "start"));
    } else if (event.key === "End") {
      event.preventDefault();
      nextDate = addDays(date, dayDeltaForWeekBoundary(iso, weekStartsOn, "end"));
    } else if (event.key === "PageDown") {
      event.preventDefault();
      nextDate = addMonths(date, 1);
    } else if (event.key === "PageUp") {
      event.preventDefault();
      nextDate = addMonths(date, -1);
    } else if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      selectDate(iso);
      return;
    } else {
      return;
    }

    focusDate(formatIsoDate(nextDate) ?? todayIsoDate());
  }
</script>

<div class="poodle-calendar" data-size={resolvedSize} data-density={resolvedDensity} data-mode={mode} aria-label={ariaLabel ?? undefined}>
  <div class="poodle-calendar__header">
    <button
      type="button"
      class="poodle-calendar__nav"
      disabled={disabled}
      aria-label="Previous month"
      on:click={() => setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, -1))))}
    >
      <span aria-hidden="true">&#x2039;</span>
    </button>

    <div class="poodle-calendar__month" aria-live="polite">
      {monthLabel}
    </div>

    <button
      type="button"
      class="poodle-calendar__nav"
      disabled={disabled}
      aria-label="Next month"
      on:click={() => setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, 1))))}
    >
      <span aria-hidden="true">&#x203A;</span>
    </button>
  </div>

  <div class="poodle-calendar__weekdays" aria-hidden="true">
    {#each weekdayLabels as label}
      <span class="poodle-calendar__weekday">{label}</span>
    {/each}
  </div>

  <div id={gridId} class="poodle-calendar__grid" role="grid" aria-label={ariaLabel ?? monthLabel}>
    {#each weeks as week}
      <div class="poodle-calendar__week" role="row">
        {#each week as day}
          <div
            class="poodle-calendar__cell"
            role="gridcell"
            aria-selected={isSelected(day.iso) ? "true" : "false"}
          >
            <button
              bind:this={dayElements[day.iso]}
              type="button"
              class="poodle-calendar__day"
              data-current-month={day.inMonth}
              data-selected={mode === "single" && currentSingleValue === day.iso}
              data-today={day.isToday}
              data-range-start={isRangeStart(day.iso)}
              data-range-end={isRangeEnd(day.iso)}
              data-in-range={isInRange(day.iso)}
              disabled={disabled}
              aria-label={formatDateLabel(day.iso, locale)}
              tabindex={focusIso === day.iso ? 0 : -1}
              on:click={() => selectDate(day.iso)}
              on:focus={() => (focusIso = day.iso)}
              on:keydown={(event) => handleDayKeydown(event, day.iso)}
            >
              {day.label}
            </button>
          </div>
        {/each}
      </div>
    {/each}
  </div>
</div>

<style>
  .poodle-calendar {
    display: grid;
    gap: 0.75rem;
    width: fit-content;
  }

  .poodle-calendar__header,
  .poodle-calendar__weekdays,
  .poodle-calendar__week {
    display: grid;
    grid-template-columns: repeat(7, var(--calendar-cell-size, 2.25rem));
    align-items: center;
  }

  .poodle-calendar__header {
    grid-template-columns: auto minmax(0, 1fr) auto;
    gap: 0.5rem;
  }

  .poodle-calendar__month {
    font-family: var(--poodle-typography-label-family);
    font-size: 0.8125rem;
    font-weight: 600;
    letter-spacing: 0.02em;
    text-align: center;
  }

  .poodle-calendar__nav {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .poodle-calendar__nav:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 82%, var(--poodle-color-background-elevated));
  }

  .poodle-calendar__weekdays {
    gap: 0.125rem;
  }

  .poodle-calendar__weekday {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-align: center;
    text-transform: uppercase;
  }

  .poodle-calendar__grid {
    display: grid;
    gap: 0.125rem;
  }

  .poodle-calendar__week {
    gap: 0.125rem;
  }

  .poodle-calendar__cell {
    display: grid;
  }

  .poodle-calendar__day {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 2.25rem;
    padding: 0.25rem;
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: 500;
  }

  .poodle-calendar__day[data-current-month="false"] {
    color: var(--poodle-color-text-secondary);
    opacity: 0.72;
  }

  .poodle-calendar__day[data-today="true"] {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 44%, var(--poodle-color-border-default));
  }

  /* Single mode: selected day */
  .poodle-calendar__day[data-selected="true"] {
    background: var(--poodle-color-accent-base);
    color: var(--poodle-color-text-inverse);
  }

  /* Range mode: in-range days */
  .poodle-calendar__day[data-in-range="true"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
  }

  /* Range mode: range endpoints */
  .poodle-calendar__day[data-range-start="true"],
  .poodle-calendar__day[data-range-end="true"] {
    background: var(--poodle-color-accent-base);
    color: var(--poodle-color-text-inverse);
  }

  .poodle-calendar__day:hover:not(:disabled),
  .poodle-calendar__day:focus-visible {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 46%, var(--poodle-color-border-default));
    background: color-mix(in srgb, var(--poodle-color-accent-base) 14%, transparent);
    outline: none;
  }

  .poodle-calendar__day[data-selected="true"]:hover:not(:disabled),
  .poodle-calendar__day[data-selected="true"]:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 88%, white 8%);
  }

  .poodle-calendar__day[data-range-start="true"]:hover:not(:disabled),
  .poodle-calendar__day[data-range-start="true"]:focus-visible,
  .poodle-calendar__day[data-range-end="true"]:hover:not(:disabled),
  .poodle-calendar__day[data-range-end="true"]:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 88%, white 8%);
  }

  .poodle-calendar__nav:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-calendar__nav:disabled,
  .poodle-calendar__day:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .poodle-calendar[data-size="xs"] { --calendar-cell-size: 1.75rem; }
  .poodle-calendar[data-size="sm"] { --calendar-cell-size: 2rem; }
  .poodle-calendar[data-size="lg"] { --calendar-cell-size: 2.5rem; }
  .poodle-calendar[data-size="xl"] { --calendar-cell-size: 2.75rem; }

  .poodle-calendar[data-size="xs"] .poodle-calendar__nav {
    width: 1.5rem;
    height: 1.5rem;
  }

  .poodle-calendar[data-size="xs"] .poodle-calendar__day {
    min-height: 1.75rem;
    font-size: 0.6875rem;
  }

  .poodle-calendar[data-size="xs"] .poodle-calendar__month {
    font-size: 0.6875rem;
  }

  .poodle-calendar[data-size="sm"] .poodle-calendar__nav {
    width: 1.75rem;
    height: 1.75rem;
  }

  .poodle-calendar[data-size="sm"] .poodle-calendar__day {
    min-height: 2rem;
  }

  .poodle-calendar[data-size="lg"] .poodle-calendar__nav {
    width: 2.25rem;
    height: 2.25rem;
  }

  .poodle-calendar[data-size="lg"] .poodle-calendar__day {
    min-height: 2.5rem;
    font-size: 0.8125rem;
  }

  .poodle-calendar[data-size="lg"] .poodle-calendar__month {
    font-size: 0.875rem;
  }

  .poodle-calendar[data-size="xl"] .poodle-calendar__nav {
    width: 2.5rem;
    height: 2.5rem;
  }

  .poodle-calendar[data-size="xl"] .poodle-calendar__day {
    min-height: 2.75rem;
    font-size: 0.875rem;
  }

  .poodle-calendar[data-size="xl"] .poodle-calendar__month {
    font-size: 0.9375rem;
  }

  /* Density variants */
  .poodle-calendar[data-density="compact"] .poodle-calendar__grid { gap: 0; }
  .poodle-calendar[data-density="compact"] .poodle-calendar__week,
  .poodle-calendar[data-density="compact"] .poodle-calendar__weekdays { gap: 0; }
  .poodle-calendar[data-density="comfortable"] .poodle-calendar__grid { gap: 0.25rem; }
  .poodle-calendar[data-density="comfortable"] .poodle-calendar__week,
  .poodle-calendar[data-density="comfortable"] .poodle-calendar__weekdays { gap: 0.25rem; }
</style>
