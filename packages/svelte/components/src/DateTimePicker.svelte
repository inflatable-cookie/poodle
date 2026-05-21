<script module lang="ts">
  let nextDateTimePickerId = 0;
</script>

<script lang="ts">
  import { default as Calendar } from "./Calendar.svelte";
  import { default as TimeInput } from "./TimeInput.svelte";
  import {
    formatDateTimeLabel,
    monthAnchorIso,
    normalizeDateTimeValue,
    todayIsoDate,
  } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlDensity, ControlSize, DateTimeValue, SemanticControlSizeRole } from "./types";

  interface Props {
    value?: DateTimeValue | null | undefined;
    defaultValue?: DateTimeValue;
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
    onValueChange?: ((value: DateTimeValue) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
  }

  let {
    value = undefined,
    defaultValue = { date: null, time: null },
    open = undefined,
    defaultOpen = false,
    placeholder = "Select date and time",
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

  const surfaceId = `poodle-date-time-picker-surface-${++nextDateTimePickerId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = $state<DateTimeValue>({ date: null, time: null });
  let uncontrolledOpen = $state(false);
  let visibleMonth = $state(todayIsoDate());
  let seededDefaults = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledValue = $derived(value !== undefined);
  const hasControlledOpen = $derived(open !== undefined);
  const currentValue = $derived(normalizeDateTimeValue((hasControlledValue ? value : uncontrolledValue) ?? defaultValue));
  const isOpen = $derived(hasControlledOpen ? open === true : uncontrolledOpen);
  const valueLabel = $derived(
    formatDateTimeLabel(currentValue, locale) ||
      (currentValue.date ? "Select time" : currentValue.time ? "Select date" : placeholder),
  );

  $effect.pre(() => {
    if (seededDefaults) {
      return;
    }

    uncontrolledValue = normalizeDateTimeValue(defaultValue);
    uncontrolledOpen = defaultOpen;
    visibleMonth = monthAnchorIso(defaultValue.date ?? todayIsoDate());
    seededDefaults = true;
  });

  $effect(() => {
    if (currentValue.date) {
      visibleMonth = monthAnchorIso(currentValue.date);
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

  function commitValue(nextValue: DateTimeValue): void {
    const normalized = normalizeDateTimeValue(nextValue);

    if (!hasControlledValue) {
      uncontrolledValue = normalized;
    }

    if (normalized.date) {
      visibleMonth = monthAnchorIso(normalized.date);
    }

    onValueChange?.(normalized);
  }

  function handleDateChange(nextValue: string | string[] | { start: string | null; end: string | null }): void {
    if (typeof nextValue === "string") {
      commitValue({ ...currentValue, date: nextValue });
    }
  }

  function handleTimeChange(nextValue: string | null): void {
    commitValue({ ...currentValue, time: nextValue });
  }
</script>

<div bind:this={rootElement} class="poodle-date-time-picker" data-size={resolvedSize} data-density={resolvedDensity} data-open={isOpen}>
  <button
    type="button"
    class="poodle-date-time-picker__trigger"
    disabled={disabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    onclick={() => setOpen(!isOpen)}
  >
    <span
      class="poodle-date-time-picker__value"
      data-placeholder={!currentValue.date || !currentValue.time}
    >
      {valueLabel}
    </span>
    <span class="poodle-date-time-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div
      id={surfaceId}
      class="poodle-date-time-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
      <div class="poodle-date-time-picker__body">
        <Calendar
          value={currentValue.date}
          visibleMonth={visibleMonth}
          {weekStartsOn}
          {locale}
          {disabled}
          size={resolvedSize}
          density={resolvedDensity}
          ariaLabel={ariaLabel ?? "Date"}
          onValueChange={handleDateChange}
          onMonthChange={(month) => (visibleMonth = month)}
        />

        <div class="poodle-date-time-picker__time-section">
          <label class="poodle-date-time-picker__time-label" for={`${surfaceId}-time`}>
            Time
          </label>
          <TimeInput
            id={`${surfaceId}-time`}
            value={currentValue.time}
            disabled={disabled}
            size={resolvedSize}
            density={resolvedDensity}
            ariaLabel={ariaLabel ? `${ariaLabel} time` : "Time"}
            onValueChange={handleTimeChange}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .poodle-date-time-picker {
    --poodle-date-time-picker-trigger-height: var(--poodle-size-control-height);
    position: relative;
    display: inline-grid;
    min-width: 16rem;
  }

  .poodle-date-time-picker__trigger {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    min-height: var(--poodle-date-time-picker-trigger-height);
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

  .poodle-date-time-picker__value {
    min-width: 0;
  }

  .poodle-date-time-picker__value[data-placeholder="true"] {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-date-time-picker__indicator {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
  }

  .poodle-date-time-picker__surface {
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

  .poodle-date-time-picker__body {
    display: grid;
    gap: 0.875rem;
  }

  .poodle-date-time-picker__time-section {
    display: grid;
    gap: 0.375rem;
  }

  .poodle-date-time-picker__time-label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .poodle-date-time-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--poodle-color-background-surface) 86%,
      var(--poodle-color-background-elevated)
    );
  }

  .poodle-date-time-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-date-time-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-date-time-picker[data-size="xs"] { --poodle-date-time-picker-trigger-height: 1.5rem; }
  .poodle-date-time-picker[data-size="xs"] .poodle-date-time-picker__trigger { font-size: 0.75rem; }

  .poodle-date-time-picker[data-size="xs"] .poodle-date-time-picker__indicator { font-size: 0.625rem; }

  .poodle-date-time-picker[data-size="sm"] { --poodle-date-time-picker-trigger-height: 1.75rem; }
  .poodle-date-time-picker[data-size="sm"] .poodle-date-time-picker__trigger { font-size: 0.8125rem; }

  .poodle-date-time-picker[data-size="sm"] .poodle-date-time-picker__indicator { font-size: 0.6875rem; }

  .poodle-date-time-picker[data-size="md"] {
    --poodle-date-time-picker-trigger-height: var(
      --poodle-size-control-height-md,
      var(--poodle-size-control-height)
    );
  }
  .poodle-date-time-picker[data-size="lg"] { --poodle-date-time-picker-trigger-height: 2.75rem; }
  .poodle-date-time-picker[data-size="lg"] .poodle-date-time-picker__trigger { font-size: 0.9375rem; }

  .poodle-date-time-picker[data-size="lg"] .poodle-date-time-picker__indicator { font-size: 0.8125rem; }

  .poodle-date-time-picker[data-size="xl"] { --poodle-date-time-picker-trigger-height: 3.25rem; }
  .poodle-date-time-picker[data-size="xl"] .poodle-date-time-picker__trigger { font-size: 1rem; }

  .poodle-date-time-picker[data-size="xl"] .poodle-date-time-picker__indicator { font-size: 0.875rem; }

  .poodle-date-time-picker[data-density="compact"] .poodle-date-time-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-date-time-picker[data-density="comfortable"] .poodle-date-time-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
