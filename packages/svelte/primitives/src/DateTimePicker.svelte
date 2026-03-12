<script context="module" lang="ts">
  let nextDateTimePickerId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import Calendar from "./Calendar.svelte";
  import TimeField from "./TimeField.svelte";
  import {
    formatDateTimeLabel,
    monthAnchorIso,
    normalizeDateTimeValue,
    todayIsoDate,
  } from "./date";

  import type { CalendarWeekStart, DateTimeValue } from "./types";

  export let value: DateTimeValue | null = null;
  export let defaultValue: DateTimeValue = { date: null, time: null };
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placeholder = "Select date and time";
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: DateTimeValue };
    openChange: { open: boolean };
  }>();

  const surfaceId = `pug-date-time-picker-surface-${++nextDateTimePickerId}`;
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = normalizeDateTimeValue(defaultValue);
  let uncontrolledOpen = defaultOpen;
  let visibleMonth = monthAnchorIso(defaultValue.date ?? todayIsoDate());

  $: currentValue = normalizeDateTimeValue(value ?? uncontrolledValue);
  $: isOpen = open ?? uncontrolledOpen;
  $: if (currentValue.date) {
    visibleMonth = monthAnchorIso(currentValue.date);
  }
  $: valueLabel =
    formatDateTimeLabel(currentValue, locale) ||
    (currentValue.date ? "Select time" : currentValue.time ? "Select date" : placeholder);

  function setOpen(nextOpen: boolean): void {
    if (open === null) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function commitValue(nextValue: DateTimeValue): void {
    const normalized = normalizeDateTimeValue(nextValue);

    if (value === null) {
      uncontrolledValue = normalized;
    }

    if (normalized.date) {
      visibleMonth = monthAnchorIso(normalized.date);
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

<div bind:this={rootElement} class="date-time-picker" data-open={isOpen}>
  <button
    type="button"
    class="date-time-picker__trigger"
    disabled={isDisabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    on:click={() => setOpen(!isOpen)}
  >
    <span
      class="date-time-picker__value"
      data-placeholder={!currentValue.date || !currentValue.time}
    >
      {valueLabel}
    </span>
    <span class="date-time-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div
      id={surfaceId}
      class="date-time-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
      <div class="date-time-picker__body">
        <Calendar
          value={currentValue.date}
          visibleMonth={visibleMonth}
          {weekStartsOn}
          {locale}
          {isDisabled}
          ariaLabel={ariaLabel ?? "Date"}
          on:valueChange={(event) => commitValue({ ...currentValue, date: event.detail.value })}
          on:monthChange={(event) => (visibleMonth = event.detail.month)}
        />

        <div class="date-time-picker__time-section">
          <label class="date-time-picker__time-label" for={`${surfaceId}-time`}>
            Time
          </label>
          <TimeField
            id={`${surfaceId}-time`}
            value={currentValue.time}
            isDisabled={isDisabled}
            ariaLabel={ariaLabel ? `${ariaLabel} time` : "Time"}
            on:valueChange={(event) => commitValue({ ...currentValue, time: event.detail.value })}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .date-time-picker {
    position: relative;
    display: inline-grid;
    min-width: 16rem;
  }

  .date-time-picker__trigger {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    min-height: var(--pug-size-control-height);
    padding: 0 var(--pug-space-control-x);
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
    text-align: left;
  }

  .date-time-picker__value {
    min-width: 0;
  }

  .date-time-picker__value[data-placeholder="true"] {
    color: var(--pug-color-text-secondary);
  }

  .date-time-picker__indicator {
    color: var(--pug-color-text-secondary);
    font-size: 0.75rem;
  }

  .date-time-picker__surface {
    position: absolute;
    top: calc(100% + 0.375rem);
    left: 0;
    z-index: var(--pug-overlay-z-menu);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-default) 72%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(
      in srgb,
      var(--pug-color-background-elevated) 98%,
      var(--pug-color-background-panel)
    );
    box-shadow: var(--pug-elevation-overlay);
  }

  .date-time-picker__body {
    display: grid;
    gap: 0.875rem;
  }

  .date-time-picker__time-section {
    display: grid;
    gap: 0.375rem;
  }

  .date-time-picker__time-label {
    color: var(--pug-color-text-secondary);
    font-family: var(--pug-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .date-time-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--pug-color-background-surface) 86%,
      var(--pug-color-background-elevated)
    );
  }

  .date-time-picker__trigger:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .date-time-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }
</style>
