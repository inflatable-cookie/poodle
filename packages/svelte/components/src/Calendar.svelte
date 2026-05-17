<script module lang="ts">
  let nextCalendarId = 0;
</script>

<script lang="ts">
  import { tick } from "svelte";

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

  interface Props {
    mode?: "single" | "range";
    value?: string | DateRangeValue | null | undefined;
    defaultValue?: string | DateRangeValue | null;
    visibleMonth?: string | null | undefined;
    weekStartsOn?: CalendarWeekStart;
    locale?: string;
    disabled?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string | DateRangeValue) => void) | undefined;
    onMonthChange?: ((month: string) => void) | undefined;
  }

  const EMPTY_RANGE: DateRangeValue = { start: null, end: null };

  let {
    mode = "single",
    value = $bindable<string | DateRangeValue | null | undefined>(undefined),
    defaultValue = null,
    visibleMonth = $bindable<string | null | undefined>(undefined),
    weekStartsOn = "monday",
    locale = "en-US",
    disabled = false,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
    onMonthChange = undefined,
  }: Props = $props();

  const gridId = `poodle-calendar-grid-${++nextCalendarId}`;
  const uiPresentation = getUiPresentation();
  let uncontrolledSingleValue = $state<string | null>(null);
  let uncontrolledRangeValue = $state<DateRangeValue>(EMPTY_RANGE);
  let uncontrolledMonth = $state(todayIsoDate());
  let focusIso = $state(todayIsoDate());
  let seededDefaults = $state(false);
  let editingMonth = $state(false);
  let editingYear = $state(false);
  let monthDraft = $state("");
  let yearDraft = $state("");
  let dayElements: Record<string, HTMLButtonElement | undefined> = {};
  let monthSelectElement: HTMLSelectElement | null = $state(null);
  let yearInputElement: HTMLInputElement | null = $state(null);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledValue = $derived(value !== undefined);
  const hasControlledVisibleMonth = $derived(visibleMonth !== undefined);
  const currentSingleValue = $derived(
    mode === "single" ? (hasControlledValue ? (typeof value === "string" ? value : null) : uncontrolledSingleValue) : null,
  );
  const currentRangeValue = $derived.by(() => {
    if (mode !== "range") {
      return EMPTY_RANGE;
    }

    return normalizeDateRange(
      hasControlledValue
        ? value !== null && typeof value === "object"
          ? (value as DateRangeValue)
          : EMPTY_RANGE
        : uncontrolledRangeValue,
    );
  });
  const currentMonth = $derived(monthAnchorIso(hasControlledVisibleMonth ? visibleMonth ?? uncontrolledMonth : uncontrolledMonth));
  const currentMonthDate = $derived(parseIsoDate(currentMonth) ?? parseIsoDate(todayIsoDate())!);
  const weeks = $derived(buildCalendarWeeks(currentMonth, weekStartsOn));
  const weekdayLabels = $derived(getWeekdayLabels(weekStartsOn, locale));
  const monthLabel = $derived(formatMonthLabel(currentMonth, locale));
  const monthName = $derived(
    new Intl.DateTimeFormat(locale, { month: "long", timeZone: "UTC" }).format(currentMonthDate)
  );
  const yearLabel = $derived(String(currentMonthDate.getUTCFullYear()));
  const monthOptions = $derived(
    Array.from({ length: 12 }, (_, monthIndex) => ({
      value: String(monthIndex),
      label: new Intl.DateTimeFormat(locale, { month: "long", timeZone: "UTC" }).format(
        new Date(Date.UTC(2000, monthIndex, 1))
      ),
    }))
  );

  $effect.pre(() => {
    if (seededDefaults) {
      return;
    }

    uncontrolledSingleValue = mode === "single" && typeof defaultValue === "string" ? defaultValue : null;
    uncontrolledRangeValue =
      mode === "range" && defaultValue !== null && typeof defaultValue === "object"
        ? normalizeDateRange(defaultValue as DateRangeValue)
        : EMPTY_RANGE;
    uncontrolledMonth = monthAnchorIso(
      visibleMonth ??
        (mode === "range"
          ? (typeof defaultValue === "object" && defaultValue !== null ? (defaultValue as DateRangeValue).start : null) ??
              todayIsoDate()
          : (typeof defaultValue === "string" ? defaultValue : null) ?? todayIsoDate()),
    );
    focusIso =
      mode === "range"
        ? (typeof defaultValue === "object" && defaultValue !== null ? (defaultValue as DateRangeValue).start : null) ??
          todayIsoDate()
        : (typeof defaultValue === "string" ? defaultValue : null) ?? todayIsoDate();
    seededDefaults = true;
  });

  $effect(() => {
    if (mode === "single" && currentSingleValue) {
      focusIso = currentSingleValue;
      return;
    }

    if (mode === "range") {
      if (currentRangeValue.end) {
        focusIso = currentRangeValue.end;
      } else if (currentRangeValue.start) {
        focusIso = currentRangeValue.start;
      }
    }
  });

  $effect(() => {
    if (!editingMonth || !monthSelectElement) {
      return;
    }

    tick().then(() => {
      monthSelectElement?.focus();
    });
  });

  $effect(() => {
    if (!editingYear || !yearInputElement) {
      return;
    }

    tick().then(() => {
      yearInputElement?.focus();
      yearInputElement?.select();
    });
  });

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
    if (hasControlledVisibleMonth) {
      visibleMonth = nextMonth;
    } else {
      uncontrolledMonth = nextMonth;
    }

    onMonthChange?.(nextMonth);
  }

  function startYearEditing(): void {
    if (disabled) {
      return;
    }

    editingMonth = false;
    yearDraft = yearLabel;
    editingYear = true;
  }

  function startMonthEditing(): void {
    if (disabled) {
      return;
    }

    editingYear = false;
    monthDraft = String(currentMonthDate.getUTCMonth());
    editingMonth = true;
  }

  function stopMonthEditing(): void {
    editingMonth = false;
  }

  function stopYearEditing(): void {
    editingYear = false;
  }

  function commitMonthDraft(): void {
    const parsedMonth = Number(monthDraft);

    if (!Number.isInteger(parsedMonth) || parsedMonth < 0 || parsedMonth > 11) {
      stopMonthEditing();
      return;
    }

    const nextDate = new Date(Date.UTC(currentMonthDate.getUTCFullYear(), parsedMonth, 1));
    setMonth(monthAnchorIso(formatIsoDate(nextDate) ?? currentMonth));
    stopMonthEditing();
  }

  function commitYearDraft(): void {
    const trimmed = yearDraft.trim();
    const parsedYear = Number(trimmed);

    if (!Number.isInteger(parsedYear) || trimmed.length !== 4) {
      stopYearEditing();
      return;
    }

    const nextDate = new Date(Date.UTC(parsedYear, currentMonthDate.getUTCMonth(), 1));
    setMonth(monthAnchorIso(formatIsoDate(nextDate) ?? currentMonth));
    stopYearEditing();
  }

  function handlePreviousMonth(): void {
    setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, -1))));
  }

  function handleNextMonth(): void {
    setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, 1))));
  }

  function handleMonthDraftChange(event: Event): void {
    monthDraft = (event.currentTarget as HTMLSelectElement).value;
    commitMonthDraft();
  }

  function handleYearDraftBeforeInput(event: InputEvent): void {
    if (
      event.data &&
      !/^\d+$/.test(event.data) &&
      event.inputType !== "deleteContentBackward" &&
      event.inputType !== "deleteContentForward"
    ) {
      event.preventDefault();
      return;
    }

    if (
      event.data &&
      /^\d+$/.test(event.data) &&
      yearDraft.length >= 4 &&
      event.inputType.startsWith("insert")
    ) {
      event.preventDefault();
    }
  }

  function handleYearDraftInput(event: Event): void {
    yearDraft = (event.currentTarget as HTMLInputElement).value;
  }

  function commitSingleValue(nextValue: string): void {
    if (hasControlledValue) {
      value = nextValue;
    } else {
      uncontrolledSingleValue = nextValue;
    }

    focusIso = nextValue;
    onValueChange?.(nextValue);
  }

  function commitRange(nextValue: DateRangeValue): void {
    const normalized = normalizeDateRange(nextValue);

    if (hasControlledValue) {
      value = normalized;
    } else {
      uncontrolledRangeValue = normalized;
    }

    focusIso = normalized.end ?? normalized.start ?? focusIso;
    onValueChange?.(normalized);
  }

  function selectDate(iso: string): void {
    if (disabled) {
      return;
    }

    if (mode === "single") {
      commitSingleValue(iso);
      return;
    }

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
      onclick={handlePreviousMonth}
    >
      <span aria-hidden="true">&#x2039;</span>
    </button>

    <div class="poodle-calendar__month" aria-live="polite">
      {#if editingMonth}
        <select
          bind:this={monthSelectElement}
          class="poodle-calendar__month-select"
          value={monthDraft}
          aria-label="Select month"
          onchange={handleMonthDraftChange}
          onblur={stopMonthEditing}
          onkeydown={(event) => {
            if (event.key === "Escape") {
              event.preventDefault();
              stopMonthEditing();
            }
          }}
        >
          {#each monthOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      {:else}
        <button
          type="button"
          class="poodle-calendar__month-button"
          disabled={disabled}
          aria-label={`Edit month, currently ${monthName}`}
          ondblclick={startMonthEditing}
        >
          <span class="poodle-calendar__month-name">{monthName}</span>
        </button>
      {/if}
      {#if editingYear}
        <input
          bind:this={yearInputElement}
          class="poodle-calendar__year-input"
          type="number"
          min="1"
          max="9999"
          step="1"
          inputmode="numeric"
          aria-label="Edit year"
          value={yearDraft}
          onbeforeinput={handleYearDraftBeforeInput}
          oninput={handleYearDraftInput}
          onblur={commitYearDraft}
          onkeydown={(event) => {
            if (["e", "E", "+", "-", "."].includes(event.key)) {
              event.preventDefault();
              return;
            }

            if (event.key === "Enter") {
              event.preventDefault();
              commitYearDraft();
            } else if (event.key === "Escape") {
              event.preventDefault();
              stopYearEditing();
            }
          }}
        />
      {:else}
        <button
          type="button"
          class="poodle-calendar__year-button"
          disabled={disabled}
          aria-label={`Edit year, currently ${yearLabel}`}
          ondblclick={startYearEditing}
        >
          {yearLabel}
        </button>
      {/if}
    </div>

    <button
      type="button"
      class="poodle-calendar__nav"
      disabled={disabled}
      aria-label="Next month"
      onclick={handleNextMonth}
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
              onclick={() => selectDate(day.iso)}
              onfocus={() => (focusIso = day.iso)}
              onkeydown={(event) => handleDayKeydown(event, day.iso)}
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
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.375rem;
    font-family: var(--poodle-typography-label-family);
    font-size: 0.8125rem;
    font-weight: 600;
    letter-spacing: 0.02em;
    text-align: center;
  }

  .poodle-calendar__month-name {
    white-space: nowrap;
  }

  .poodle-calendar__month-button,
  .poodle-calendar__year-button {
    padding: 0;
    border: 0;
    border-bottom: 0.0625rem dashed color-mix(in srgb, var(--poodle-color-text-secondary) 72%, transparent);
    background: transparent;
    color: inherit;
    font: inherit;
    letter-spacing: inherit;
    line-height: 1.2;
    cursor: text;
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-calendar__month-button:hover:not(:disabled),
  .poodle-calendar__year-button:hover:not(:disabled) {
    color: var(--poodle-color-text-primary);
    border-bottom-color: color-mix(in srgb, var(--poodle-color-accent-base) 72%, transparent);
  }

  .poodle-calendar__month-select,
  .poodle-calendar__year-input {
    box-sizing: border-box;
    min-width: 4.5ch;
    min-height: 1.75em;
    padding: 0.125rem 0.25rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-accent-base) 42%, var(--poodle-color-border-default));
    border-radius: 0.25rem;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 88%, var(--poodle-color-background-elevated));
    color: inherit;
    font: inherit;
    letter-spacing: inherit;
    line-height: 1.4;
    text-align: center;
    outline: 0;
    box-shadow: 0 0 0 0.125rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 18%, transparent);
  }

  .poodle-calendar__month-select {
    width: auto;
  }

  .poodle-calendar__year-input {
    width: 5ch;
  }

  .poodle-calendar__year-input::-webkit-outer-spin-button,
  .poodle-calendar__year-input::-webkit-inner-spin-button {
    margin: 0;
    -webkit-appearance: none;
  }

  .poodle-calendar__year-input[type="number"] {
    appearance: textfield;
    -moz-appearance: textfield;
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

  .poodle-calendar__day[data-selected="true"] {
    background: var(--poodle-color-accent-base);
    color: var(--poodle-color-text-inverse);
  }

  .poodle-calendar__day[data-in-range="true"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 16%, transparent);
  }

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

  .poodle-calendar[data-density="compact"] .poodle-calendar__grid { gap: 0; }
  .poodle-calendar[data-density="compact"] .poodle-calendar__week,
  .poodle-calendar[data-density="compact"] .poodle-calendar__weekdays { gap: 0; }
  .poodle-calendar[data-density="comfortable"] .poodle-calendar__grid { gap: 0.25rem; }
  .poodle-calendar[data-density="comfortable"] .poodle-calendar__week,
  .poodle-calendar[data-density="comfortable"] .poodle-calendar__weekdays { gap: 0.25rem; }
</style>
