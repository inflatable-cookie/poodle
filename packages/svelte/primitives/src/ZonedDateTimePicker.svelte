<script context="module" lang="ts">
  let nextZonedDateTimePickerId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import Calendar from "./Calendar.svelte";
  import TimeField from "./TimeField.svelte";
  import TimeZoneSelect from "./TimeZoneSelect.svelte";
  import {
    formatZonedDateTimeLabel,
    monthAnchorIso,
    normalizeZonedDateTimeValue,
    todayIsoDate,
  } from "./date";

  import type { CalendarWeekStart, TimeZoneOption, ZonedDateTimeValue } from "./types";

  export let value: ZonedDateTimeValue | null = null;
  export let defaultValue: ZonedDateTimeValue = { date: null, time: null, timeZone: null };
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placeholder = "Select date, time, and zone";
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let timeZoneOptions: TimeZoneOption[] = [];
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: ZonedDateTimeValue };
    openChange: { open: boolean };
  }>();

  const surfaceId = `flint-zoned-date-time-picker-surface-${++nextZonedDateTimePickerId}`;
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = normalizeZonedDateTimeValue(defaultValue);
  let uncontrolledOpen = defaultOpen;
  let visibleMonth = monthAnchorIso(defaultValue.date ?? todayIsoDate());

  $: currentValue = normalizeZonedDateTimeValue(value ?? uncontrolledValue);
  $: isOpen = open ?? uncontrolledOpen;
  $: if (currentValue.date) {
    visibleMonth = monthAnchorIso(currentValue.date);
  }
  $: valueLabel =
    formatZonedDateTimeLabel(currentValue, locale) ||
    (currentValue.date || currentValue.time || currentValue.timeZone
      ? "Complete zoned date and time"
      : placeholder);

  function setOpen(nextOpen: boolean): void {
    if (open === null) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function commitValue(nextValue: ZonedDateTimeValue): void {
    const normalized = normalizeZonedDateTimeValue(nextValue);

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

<div bind:this={rootElement} class="zoned-date-time-picker" data-open={isOpen}>
  <button
    type="button"
    class="zoned-date-time-picker__trigger"
    disabled={isDisabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    on:click={() => setOpen(!isOpen)}
  >
    <span
      class="zoned-date-time-picker__value"
      data-placeholder={!currentValue.date || !currentValue.time || !currentValue.timeZone}
    >
      {valueLabel}
    </span>
    <span class="zoned-date-time-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div
      id={surfaceId}
      class="zoned-date-time-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
      <div class="zoned-date-time-picker__body">
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

        <div class="zoned-date-time-picker__fields">
          <div class="zoned-date-time-picker__field">
            <label class="zoned-date-time-picker__label" for={`${surfaceId}-time`}>
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

          <div class="zoned-date-time-picker__field">
            <label class="zoned-date-time-picker__label" for={`${surfaceId}-timezone`}>
              Time zone
            </label>
            <TimeZoneSelect
              id={`${surfaceId}-timezone`}
              value={currentValue.timeZone}
              options={timeZoneOptions}
              isDisabled={isDisabled}
              ariaLabel={ariaLabel ? `${ariaLabel} time zone` : "Time zone"}
              on:valueChange={(event) => commitValue({ ...currentValue, timeZone: event.detail.value })}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .zoned-date-time-picker {
    position: relative;
    display: inline-grid;
    min-width: 18rem;
  }

  .zoned-date-time-picker__trigger {
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

  .zoned-date-time-picker__value {
    min-width: 0;
  }

  .zoned-date-time-picker__value[data-placeholder="true"] {
    color: var(--flint-color-text-secondary);
  }

  .zoned-date-time-picker__indicator {
    color: var(--flint-color-text-secondary);
    font-size: 0.75rem;
  }

  .zoned-date-time-picker__surface {
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

  .zoned-date-time-picker__body {
    display: grid;
    gap: 0.875rem;
  }

  .zoned-date-time-picker__fields {
    display: grid;
    gap: 0.75rem;
  }

  .zoned-date-time-picker__field {
    display: grid;
    gap: 0.375rem;
  }

  .zoned-date-time-picker__label {
    color: var(--flint-color-text-secondary);
    font-family: var(--flint-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .zoned-date-time-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--flint-color-background-surface) 86%,
      var(--flint-color-background-elevated)
    );
  }

  .zoned-date-time-picker__trigger:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .zoned-date-time-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
