<script context="module" lang="ts">
  let nextDateTimeRangePickerId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import RangeCalendar from "./RangeCalendar.svelte";
  import TimeField from "./TimeField.svelte";
  import {
    formatDateTimeRangeLabel,
    monthAnchorIso,
    normalizeDateTimeRangeValue,
    normalizeDateRange,
    todayIsoDate,
  } from "./date";

  import type { CalendarWeekStart, DateTimeRangeValue } from "./types";

  export let value: DateTimeRangeValue | null = null;
  export let defaultValue: DateTimeRangeValue = {
    start: { date: null, time: null },
    end: { date: null, time: null },
  };
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placeholder = "Select date and time range";
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: DateTimeRangeValue };
    openChange: { open: boolean };
  }>();

  const surfaceId = `flint-date-time-range-picker-surface-${++nextDateTimeRangePickerId}`;
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = normalizeDateTimeRangeValue(defaultValue);
  let uncontrolledOpen = defaultOpen;
  let visibleMonth = monthAnchorIso(defaultValue.start.date ?? todayIsoDate());

  $: currentValue = normalizeDateTimeRangeValue(value ?? uncontrolledValue);
  $: currentRange = normalizeDateRange({
    start: currentValue.start.date,
    end: currentValue.end.date,
  });
  $: isOpen = open ?? uncontrolledOpen;
  $: if (currentValue.start.date) {
    visibleMonth = monthAnchorIso(currentValue.start.date);
  }
  $: valueLabel = formatDateTimeRangeLabel(currentValue, locale) || placeholder;

  function setOpen(nextOpen: boolean): void {
    if (open === null) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function commitValue(nextValue: DateTimeRangeValue): void {
    const normalized = normalizeDateTimeRangeValue(nextValue);

    if (value === null) {
      uncontrolledValue = normalized;
    }

    if (normalized.start.date) {
      visibleMonth = monthAnchorIso(normalized.start.date);
    }

    dispatch("valueChange", { value: normalized });
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!isOpen || !rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && isOpen) {
        event.preventDefault();
        setOpen(false);
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<div bind:this={rootElement} class="date-time-range-picker" data-open={isOpen}>
  <button
    type="button"
    class="date-time-range-picker__trigger"
    disabled={isDisabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    on:click={() => setOpen(!isOpen)}
  >
    <span
      class="date-time-range-picker__value"
      data-placeholder={!currentValue.start.date || !currentValue.end.date}
    >
      {valueLabel}
    </span>
    <span class="date-time-range-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div
      id={surfaceId}
      class="date-time-range-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
      <div class="date-time-range-picker__body">
        <RangeCalendar
          value={currentRange}
          visibleMonth={visibleMonth}
          {weekStartsOn}
          {locale}
          {isDisabled}
          ariaLabel={ariaLabel ?? placeholder}
          on:valueChange={(event) =>
            commitValue({
              start: { ...currentValue.start, date: event.detail.value.start },
              end: { ...currentValue.end, date: event.detail.value.end },
            })}
          on:monthChange={(event) => (visibleMonth = event.detail.month)}
        />

        <div class="date-time-range-picker__times">
          <div class="date-time-range-picker__time-section">
            <label class="date-time-range-picker__time-label" for={`${surfaceId}-start-time`}>
              Start time
            </label>
            <TimeField
              id={`${surfaceId}-start-time`}
              value={currentValue.start.time}
              isDisabled={isDisabled}
              ariaLabel={ariaLabel ? `${ariaLabel} start time` : "Start time"}
              on:valueChange={(event) =>
                commitValue({
                  start: { ...currentValue.start, time: event.detail.value },
                  end: currentValue.end,
                })}
            />
          </div>

          <div class="date-time-range-picker__time-section">
            <label class="date-time-range-picker__time-label" for={`${surfaceId}-end-time`}>
              End time
            </label>
            <TimeField
              id={`${surfaceId}-end-time`}
              value={currentValue.end.time}
              isDisabled={isDisabled}
              ariaLabel={ariaLabel ? `${ariaLabel} end time` : "End time"}
              on:valueChange={(event) =>
                commitValue({
                  start: currentValue.start,
                  end: { ...currentValue.end, time: event.detail.value },
                })}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .date-time-range-picker {
    position: relative;
    display: inline-grid;
    min-width: 18rem;
  }

  .date-time-range-picker__trigger {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    min-height: var(--flint-size-control-height);
    padding: 0 var(--flint-space-control-x);
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: var(--flint-radius-control);
    background: var(--flint-color-background-surface);
    color: var(--flint-color-text-primary);
    cursor: pointer;
    font-family: var(--flint-typography-body-family);
    font-size: var(--flint-typography-body-size);
    line-height: var(--flint-typography-body-lineHeight);
    text-align: left;
  }

  .date-time-range-picker__value {
    min-width: 0;
  }

  .date-time-range-picker__value[data-placeholder="true"] {
    color: var(--flint-color-text-secondary);
  }

  .date-time-range-picker__indicator {
    color: var(--flint-color-text-secondary);
    font-size: 0.75rem;
  }

  .date-time-range-picker__surface {
    position: absolute;
    top: calc(100% + 0.375rem);
    left: 0;
    z-index: var(--flint-overlay-z-menu);
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-default) 72%, transparent);
    border-radius: var(--flint-radius-surface);
    background: color-mix(
      in srgb,
      var(--flint-color-background-elevated) 98%,
      var(--flint-color-background-panel)
    );
    box-shadow: var(--flint-elevation-overlay);
  }

  .date-time-range-picker__body {
    display: grid;
    gap: 0.875rem;
  }

  .date-time-range-picker__times {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
  }

  .date-time-range-picker__time-section {
    display: grid;
    gap: 0.375rem;
  }

  .date-time-range-picker__time-label {
    color: var(--flint-color-text-secondary);
    font-family: var(--flint-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .date-time-range-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--flint-color-background-surface) 86%,
      var(--flint-color-background-elevated)
    );
  }

  .date-time-range-picker__trigger:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .date-time-range-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
