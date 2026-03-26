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
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlSize, SemanticControlSizeRole, TimeZoneOption, ZonedDateTimeValue } from "./types";

  export let value: ZonedDateTimeValue | null = null;
  export let defaultValue: ZonedDateTimeValue = { date: null, time: null, timeZone: null };
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placeholder = "Select date, time, and zone";
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let timeZoneOptions: TimeZoneOption[] = [];
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";

  const dispatch = createEventDispatcher<{
    valueChange: { value: ZonedDateTimeValue };
    openChange: { open: boolean };
  }>();

  const surfaceId = `poodle-zoned-date-time-picker-surface-${++nextZonedDateTimePickerId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = normalizeZonedDateTimeValue(defaultValue);
  let uncontrolledOpen = defaultOpen;
  let visibleMonth = monthAnchorIso(defaultValue.date ?? todayIsoDate());

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
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

<div bind:this={rootElement} class="zoned-date-time-picker" data-size={resolvedSize} data-open={isOpen}>
  <button
    type="button"
    class="zoned-date-time-picker__trigger"
    disabled={disabled}
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
          {disabled}
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
              disabled={disabled}
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
              disabled={disabled}
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
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    text-align: left;
  }

  .zoned-date-time-picker__value {
    min-width: 0;
  }

  .zoned-date-time-picker__value[data-placeholder="true"] {
    color: var(--poodle-color-text-secondary);
  }

  .zoned-date-time-picker__indicator {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
  }

  .zoned-date-time-picker__surface {
    position: absolute;
    top: calc(100% + 0.375rem);
    left: 0;
    z-index: var(--poodle-overlay-z-menu);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(
      in srgb,
      var(--poodle-color-background-elevated) 98%,
      var(--poodle-color-background-panel)
    );
    box-shadow: var(--poodle-elevation-overlay);
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
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .zoned-date-time-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--poodle-color-background-surface) 86%,
      var(--poodle-color-background-elevated)
    );
  }

  .zoned-date-time-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .zoned-date-time-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .zoned-date-time-picker[data-size="xs"] .zoned-date-time-picker__trigger {
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
    font-size: 0.75rem;
  }

  .zoned-date-time-picker[data-size="sm"] .zoned-date-time-picker__trigger {
    min-height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .zoned-date-time-picker[data-size="lg"] .zoned-date-time-picker__trigger {
    min-height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
    font-size: 0.9375rem;
  }

  .zoned-date-time-picker[data-size="xl"] .zoned-date-time-picker__trigger {
    min-height: calc(var(--poodle-size-control-height) + 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.1875rem);
    font-size: 1rem;
  }
</style>
