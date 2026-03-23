<script context="module" lang="ts">
  let nextRangeCalendarId = 0;
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

  import type { CalendarWeekStart, DateRangeValue } from "./types";

  export let value: DateRangeValue | null = null;
  export let defaultValue: DateRangeValue = { start: null, end: null };
  export let visibleMonth: string | null = null;
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: DateRangeValue };
    monthChange: { month: string };
  }>();

  const gridId = `flint-range-calendar-grid-${++nextRangeCalendarId}`;
  let uncontrolledValue = normalizeDateRange(defaultValue);
  let uncontrolledMonth = monthAnchorIso(visibleMonth ?? defaultValue.start ?? todayIsoDate());
  let focusIso = defaultValue.start ?? todayIsoDate();
  let dayElements: Record<string, HTMLButtonElement | undefined> = {};

  $: currentValue = normalizeDateRange(value ?? uncontrolledValue);
  $: currentMonth = monthAnchorIso(visibleMonth ?? uncontrolledMonth);
  $: weeks = buildCalendarWeeks(currentMonth, weekStartsOn);
  $: weekdayLabels = getWeekdayLabels(weekStartsOn, locale);
  $: monthLabel = formatMonthLabel(currentMonth, locale);
  $: if (currentValue.end) {
    focusIso = currentValue.end;
  } else if (currentValue.start) {
    focusIso = currentValue.start;
  }

  function setMonth(nextMonth: string): void {
    if (visibleMonth === null) {
      uncontrolledMonth = nextMonth;
    }

    dispatch("monthChange", { month: nextMonth });
  }

  function commitValue(nextValue: DateRangeValue): void {
    const normalized = normalizeDateRange(nextValue);

    if (value === null) {
      uncontrolledValue = normalized;
    }

    focusIso = normalized.end ?? normalized.start ?? focusIso;
    dispatch("valueChange", { value: normalized });
  }

  function selectDate(iso: string): void {
    if (isDisabled) {
      return;
    }

    if (!currentValue.start || currentValue.end) {
      commitValue({ start: iso, end: null });
      return;
    }

    if (compareIsoDate(iso, currentValue.start) < 0) {
      commitValue({ start: iso, end: currentValue.start });
      return;
    }

    commitValue({ start: currentValue.start, end: iso });
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

<div class="range-calendar" aria-label={ariaLabel ?? undefined}>
  <div class="range-calendar__header">
    <button
      type="button"
      class="range-calendar__nav"
      disabled={isDisabled}
      aria-label="Previous month"
      on:click={() => setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, -1))))}
    >
      <span aria-hidden="true">‹</span>
    </button>

    <div class="range-calendar__month" aria-live="polite">
      {monthLabel}
    </div>

    <button
      type="button"
      class="range-calendar__nav"
      disabled={isDisabled}
      aria-label="Next month"
      on:click={() => setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, 1))))}
    >
      <span aria-hidden="true">›</span>
    </button>
  </div>

  <div class="range-calendar__weekdays" aria-hidden="true">
    {#each weekdayLabels as label}
      <span class="range-calendar__weekday">{label}</span>
    {/each}
  </div>

  <div id={gridId} class="range-calendar__grid" role="grid" aria-label={ariaLabel ?? monthLabel}>
    {#each weeks as week}
      <div class="range-calendar__week" role="row">
        {#each week as day}
          <div
            class="range-calendar__cell"
            role="gridcell"
            aria-selected={
              currentValue.start === day.iso || currentValue.end === day.iso ? "true" : "false"
            }
          >
            <button
              bind:this={dayElements[day.iso]}
              type="button"
              class="range-calendar__day"
              data-current-month={day.inMonth}
              data-today={day.isToday}
              data-range-start={currentValue.start === day.iso}
              data-range-end={currentValue.end === day.iso}
              data-in-range={isIsoDateWithinRange(day.iso, currentValue)}
              disabled={isDisabled}
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
  .range-calendar {
    display: grid;
    gap: 0.75rem;
    min-width: 16rem;
  }

  .range-calendar__header,
  .range-calendar__weekdays,
  .range-calendar__week {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    align-items: center;
  }

  .range-calendar__header {
    grid-template-columns: auto minmax(0, 1fr) auto;
    gap: 0.5rem;
  }

  .range-calendar__month {
    font-family: var(--flint-typography-label-family);
    font-size: 0.8125rem;
    font-weight: 600;
    letter-spacing: 0.02em;
    text-align: center;
  }

  .range-calendar__nav {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: var(--flint-radius-control);
    background: var(--flint-color-background-surface);
    color: var(--flint-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .range-calendar__nav:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--flint-color-background-surface) 82%,
      var(--flint-color-background-elevated)
    );
  }

  .range-calendar__weekdays {
    gap: 0.125rem;
  }

  .range-calendar__weekday {
    color: var(--flint-color-text-secondary);
    font-family: var(--flint-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-align: center;
    text-transform: uppercase;
  }

  .range-calendar__grid {
    display: grid;
    gap: 0.125rem;
  }

  .range-calendar__week {
    gap: 0.125rem;
  }

  .range-calendar__cell {
    display: grid;
  }

  .range-calendar__day {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 2.25rem;
    padding: 0.25rem;
    border: 0.0625rem solid transparent;
    border-radius: var(--flint-radius-control);
    background: transparent;
    color: var(--flint-color-text-primary);
    cursor: pointer;
    font-family: var(--flint-typography-label-family);
    font-size: 0.75rem;
    font-weight: 500;
  }

  .range-calendar__day[data-current-month="false"] {
    color: var(--flint-color-text-secondary);
    opacity: 0.72;
  }

  .range-calendar__day[data-today="true"] {
    border-color: color-mix(in srgb, var(--flint-color-accent-base) 44%, var(--flint-color-border-default));
  }

  .range-calendar__day[data-in-range="true"] {
    background: color-mix(in srgb, var(--flint-color-accent-base) 16%, transparent);
  }

  .range-calendar__day[data-range-start="true"],
  .range-calendar__day[data-range-end="true"] {
    background: var(--flint-color-accent-base);
    color: var(--flint-color-text-inverse);
  }

  .range-calendar__day:hover:not(:disabled),
  .range-calendar__day:focus-visible {
    border-color: color-mix(in srgb, var(--flint-color-accent-base) 46%, var(--flint-color-border-default));
    outline: none;
  }

  .range-calendar__nav:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .range-calendar__nav:disabled,
  .range-calendar__day:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
