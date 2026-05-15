<script module lang="ts">
  let nextDateTimeRangePickerId = 0;
</script>

<script lang="ts">
  import Calendar from "./Calendar.svelte";
  import TimeInput from "./TimeInput.svelte";
  import {
    formatDateTimeRangeLabel,
    monthAnchorIso,
    normalizeDateTimeRangeValue,
    normalizeDateRange,
    todayIsoDate,
  } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlDensity, ControlSize, DateRangeValue, DateTimeRangeValue, SemanticControlSizeRole } from "./types";

  interface Props {
    value?: DateTimeRangeValue | null | undefined;
    defaultValue?: DateTimeRangeValue;
    open?: boolean | null | undefined;
    defaultOpen?: boolean;
    placeholder?: string;
    weekStartsOn?: CalendarWeekStart;
    locale?: string;
    disabled?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: DateTimeRangeValue) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
  }

  let {
    value = undefined,
    defaultValue = {
      start: { date: null, time: null },
      end: { date: null, time: null },
    },
    open = undefined,
    defaultOpen = false,
    placeholder = "Select date and time range",
    weekStartsOn = "monday",
    locale = "en-US",
    disabled = false,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
    onOpenChange = undefined,
  }: Props = $props();

  const surfaceId = `poodle-date-time-range-picker-surface-${++nextDateTimeRangePickerId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = $state<DateTimeRangeValue>({
    start: { date: null, time: null },
    end: { date: null, time: null },
  });
  let uncontrolledOpen = $state(false);
  let visibleMonth = $state(todayIsoDate());
  let seededDefaults = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledValue = $derived(value !== undefined);
  const hasControlledOpen = $derived(open !== undefined);
  const currentValue = $derived(normalizeDateTimeRangeValue(hasControlledValue ? value : uncontrolledValue));
  const currentRange = $derived(
    normalizeDateRange({
      start: currentValue.start.date,
      end: currentValue.end.date,
    }),
  );
  const isOpen = $derived(hasControlledOpen ? open === true : uncontrolledOpen);
  const valueLabel = $derived(formatDateTimeRangeLabel(currentValue, locale) || placeholder);

  $effect.pre(() => {
    if (seededDefaults) {
      return;
    }

    uncontrolledValue = normalizeDateTimeRangeValue(defaultValue);
    uncontrolledOpen = defaultOpen;
    visibleMonth = monthAnchorIso(defaultValue.start.date ?? todayIsoDate());
    seededDefaults = true;
  });

  $effect(() => {
    if (currentValue.start.date) {
      visibleMonth = monthAnchorIso(currentValue.start.date);
    }
  });

  $effect(() => {
    if (!isOpen) {
      return;
    }

    function handlePointerDown(event: MouseEvent): void {
      if (!rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        setOpen(false);
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape") {
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

  function setOpen(nextOpen: boolean): void {
    if (!hasControlledOpen) {
      uncontrolledOpen = nextOpen;
    }

    onOpenChange?.(nextOpen);
  }

  function commitValue(nextValue: DateTimeRangeValue): void {
    const normalized = normalizeDateTimeRangeValue(nextValue);

    if (!hasControlledValue) {
      uncontrolledValue = normalized;
    }

    if (normalized.start.date) {
      visibleMonth = monthAnchorIso(normalized.start.date);
    }

    onValueChange?.(normalized);
  }
</script>

<div bind:this={rootElement} class="poodle-date-time-range-picker" data-size={resolvedSize} data-density={resolvedDensity} data-open={isOpen}>
  <button
    type="button"
    class="poodle-date-time-range-picker__trigger"
    disabled={disabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    onclick={() => setOpen(!isOpen)}
  >
    <span
      class="poodle-date-time-range-picker__value"
      data-placeholder={!currentValue.start.date || !currentValue.end.date}
    >
      {valueLabel}
    </span>
    <span class="poodle-date-time-range-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div
      id={surfaceId}
      class="poodle-date-time-range-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
      <div class="poodle-date-time-range-picker__body">
        <Calendar
          mode="range"
          value={currentRange}
          visibleMonth={visibleMonth}
          {weekStartsOn}
          {locale}
          {disabled}
          size={resolvedSize}
          density={resolvedDensity}
          ariaLabel={ariaLabel ?? placeholder}
          onValueChange={(nextRange) =>
            commitValue({
              start: { ...currentValue.start, date: (nextRange as DateRangeValue).start },
              end: { ...currentValue.end, date: (nextRange as DateRangeValue).end },
            })}
          onMonthChange={(month) => (visibleMonth = month)}
        />

        <div class="poodle-date-time-range-picker__times">
          <div class="poodle-date-time-range-picker__time-section">
            <label class="poodle-date-time-range-picker__time-label" for={`${surfaceId}-start-time`}>
              Start time
            </label>
            <TimeInput
              id={`${surfaceId}-start-time`}
              value={currentValue.start.time}
              disabled={disabled}
              size={resolvedSize}
              density={resolvedDensity}
              ariaLabel={ariaLabel ? `${ariaLabel} start time` : "Start time"}
              onValueChange={(nextValue) =>
                commitValue({
                  start: { ...currentValue.start, time: nextValue },
                  end: currentValue.end,
                })}
            />
          </div>

          <div class="poodle-date-time-range-picker__time-section">
            <label class="poodle-date-time-range-picker__time-label" for={`${surfaceId}-end-time`}>
              End time
            </label>
            <TimeInput
              id={`${surfaceId}-end-time`}
              value={currentValue.end.time}
              disabled={disabled}
              size={resolvedSize}
              density={resolvedDensity}
              ariaLabel={ariaLabel ? `${ariaLabel} end time` : "End time"}
              onValueChange={(nextValue) =>
                commitValue({
                  start: currentValue.start,
                  end: { ...currentValue.end, time: nextValue },
                })}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .poodle-date-time-range-picker {
    position: relative;
    display: inline-grid;
    min-width: 18rem;
  }

  .poodle-date-time-range-picker__trigger {
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

  .poodle-date-time-range-picker__value {
    min-width: 0;
  }

  .poodle-date-time-range-picker__value[data-placeholder="true"] {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-date-time-range-picker__indicator {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
  }

  .poodle-date-time-range-picker__surface {
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

  .poodle-date-time-range-picker__body {
    display: grid;
    gap: 0.875rem;
  }

  .poodle-date-time-range-picker__times {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
  }

  .poodle-date-time-range-picker__time-section {
    display: grid;
    gap: 0.375rem;
  }

  .poodle-date-time-range-picker__time-label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .poodle-date-time-range-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--poodle-color-background-surface) 86%,
      var(--poodle-color-background-elevated)
    );
  }

  .poodle-date-time-range-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-date-time-range-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-date-time-range-picker[data-size="xs"] .poodle-date-time-range-picker__trigger {
    min-height: 1.5rem;
    font-size: 0.75rem;
  }

  .poodle-date-time-range-picker[data-size="xs"] .poodle-date-time-range-picker__indicator { font-size: 0.625rem; }

  .poodle-date-time-range-picker[data-size="sm"] .poodle-date-time-range-picker__trigger {
    min-height: 1.75rem;
  }

  .poodle-date-time-range-picker[data-size="sm"] .poodle-date-time-range-picker__indicator { font-size: 0.6875rem; }

  .poodle-date-time-range-picker[data-size="lg"] .poodle-date-time-range-picker__trigger {
    min-height: 2.75rem;
    font-size: 0.9375rem;
  }

  .poodle-date-time-range-picker[data-size="lg"] .poodle-date-time-range-picker__indicator { font-size: 0.8125rem; }

  .poodle-date-time-range-picker[data-size="xl"] .poodle-date-time-range-picker__trigger {
    min-height: 3.25rem;
    font-size: 1rem;
  }

  .poodle-date-time-range-picker[data-size="xl"] .poodle-date-time-range-picker__indicator { font-size: 0.875rem; }

  .poodle-date-time-range-picker[data-density="compact"] .poodle-date-time-range-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-date-time-range-picker[data-density="comfortable"] .poodle-date-time-range-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
