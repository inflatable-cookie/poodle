<script context="module" lang="ts">
  let nextCalendarId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import {
    addDays,
    addMonths,
    buildCalendarWeeks,
    dayDeltaForWeekBoundary,
    formatDateLabel,
    formatIsoDate,
    formatMonthLabel,
    getWeekdayLabels,
    monthAnchorIso,
    parseIsoDate,
    todayIsoDate,
  } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let visibleMonth: string | null = null;
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    monthChange: { month: string };
  }>();

  const gridId = `poodle-calendar-grid-${++nextCalendarId}`;
  const uiPresentation = getUiPresentation();
  let uncontrolledValue = defaultValue;
  let uncontrolledMonth = monthAnchorIso(visibleMonth ?? defaultValue ?? todayIsoDate());
  let focusIso = defaultValue ?? todayIsoDate();
  let dayElements: Record<string, HTMLButtonElement | undefined> = {};

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: currentValue = value ?? uncontrolledValue;
  $: currentMonth = monthAnchorIso(visibleMonth ?? uncontrolledMonth);
  $: weeks = buildCalendarWeeks(currentMonth, weekStartsOn);
  $: weekdayLabels = getWeekdayLabels(weekStartsOn, locale);
  $: monthLabel = formatMonthLabel(currentMonth, locale);
  $: if (currentValue) {
    focusIso = currentValue;
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

    if (value === null) {
      uncontrolledValue = iso;
    }

    focusIso = iso;
    dispatch("valueChange", { value: iso });
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

<div class="calendar" data-size={resolvedSize} data-density={resolvedDensity} aria-label={ariaLabel ?? undefined}>
  <div class="calendar__header">
    <button
      type="button"
      class="calendar__nav"
      disabled={disabled}
      aria-label="Previous month"
      on:click={() => setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, -1))))}
    >
      <span aria-hidden="true">‹</span>
    </button>

    <div class="calendar__month" aria-live="polite">
      {monthLabel}
    </div>

    <button
      type="button"
      class="calendar__nav"
      disabled={disabled}
      aria-label="Next month"
      on:click={() => setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, 1))))}
    >
      <span aria-hidden="true">›</span>
    </button>
  </div>

  <div class="calendar__weekdays" aria-hidden="true">
    {#each weekdayLabels as label}
      <span class="calendar__weekday">{label}</span>
    {/each}
  </div>

  <div id={gridId} class="calendar__grid" role="grid" aria-label={ariaLabel ?? monthLabel}>
    {#each weeks as week}
      <div class="calendar__week" role="row">
        {#each week as day}
          <div
            class="calendar__cell"
            role="gridcell"
            aria-selected={currentValue === day.iso ? "true" : "false"}
          >
            <button
              bind:this={dayElements[day.iso]}
              type="button"
              class="calendar__day"
              data-current-month={day.inMonth}
              data-selected={currentValue === day.iso}
              data-today={day.isToday}
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
  .calendar {
    display: grid;
    gap: 0.75rem;
    width: 18rem;
  }

  .calendar__header,
  .calendar__weekdays,
  .calendar__week {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    align-items: center;
  }

  .calendar__header {
    grid-template-columns: auto minmax(0, 1fr) auto;
    gap: 0.5rem;
  }

  .calendar__month {
    font-family: var(--poodle-typography-label-family);
    font-size: 0.8125rem;
    font-weight: 600;
    letter-spacing: 0.02em;
    text-align: center;
  }

  .calendar__nav {
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

  .calendar__nav:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 82%, var(--poodle-color-background-elevated));
  }

  .calendar__weekdays {
    gap: 0.125rem;
  }

  .calendar__weekday {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-align: center;
    text-transform: uppercase;
  }

  .calendar__grid {
    display: grid;
    gap: 0.125rem;
  }

  .calendar__week {
    gap: 0.125rem;
  }

  .calendar__cell {
    display: grid;
  }

  .calendar__day {
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

  .calendar__day[data-current-month="false"] {
    color: var(--poodle-color-text-secondary);
    opacity: 0.72;
  }

  .calendar__day[data-today="true"] {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 44%, var(--poodle-color-border-default));
  }

  .calendar__day[data-selected="true"] {
    background: var(--poodle-color-accent-base);
    color: var(--poodle-color-text-inverse);
  }

  .calendar__day:hover:not(:disabled),
  .calendar__day:focus-visible {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 46%, var(--poodle-color-border-default));
    background: color-mix(in srgb, var(--poodle-color-accent-base) 14%, transparent);
    outline: none;
  }

  .calendar__day[data-selected="true"]:hover:not(:disabled),
  .calendar__day[data-selected="true"]:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 88%, white 8%);
  }

  .calendar__nav:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .calendar__nav:disabled,
  .calendar__day:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .calendar[data-size="xs"] { width: 14.5rem; }
  .calendar[data-size="sm"] { width: 16rem; }
  .calendar[data-size="lg"] { width: 20.5rem; }
  .calendar[data-size="xl"] { width: 23rem; }

  .calendar[data-size="xs"] .calendar__nav {
    width: 1.5rem;
    height: 1.5rem;
  }

  .calendar[data-size="xs"] .calendar__day {
    min-height: 1.75rem;
    font-size: 0.6875rem;
  }

  .calendar[data-size="xs"] .calendar__month {
    font-size: 0.6875rem;
  }

  .calendar[data-size="sm"] .calendar__nav {
    width: 1.75rem;
    height: 1.75rem;
  }

  .calendar[data-size="sm"] .calendar__day {
    min-height: 2rem;
  }

  .calendar[data-size="lg"] .calendar__nav {
    width: 2.25rem;
    height: 2.25rem;
  }

  .calendar[data-size="lg"] .calendar__day {
    min-height: 2.5rem;
    font-size: 0.8125rem;
  }

  .calendar[data-size="lg"] .calendar__month {
    font-size: 0.875rem;
  }

  .calendar[data-size="xl"] .calendar__nav {
    width: 2.5rem;
    height: 2.5rem;
  }

  .calendar[data-size="xl"] .calendar__day {
    min-height: 2.75rem;
    font-size: 0.875rem;
  }

  .calendar[data-size="xl"] .calendar__month {
    font-size: 0.9375rem;
  }

  /* Density variants */
  .calendar[data-density="compact"] .calendar__grid { gap: 0.0625rem; }
  .calendar[data-density="compact"] .calendar__cell { padding: 0.0625rem; }
  .calendar[data-density="comfortable"] .calendar__grid { gap: 0.1875rem; }
  .calendar[data-density="comfortable"] .calendar__cell { padding: 0.1875rem; }
</style>
