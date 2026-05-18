<script module lang="ts">
  let nextDateRangePickerId = 0;
</script>

<script lang="ts">
  import { default as Calendar } from "./Calendar.svelte";
  import { formatDateLabel, monthAnchorIso, normalizeDateRange, todayIsoDate } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlDensity, ControlSize, DateRangeValue, SemanticControlSizeRole } from "./types";

  interface Props {
    value?: DateRangeValue | null | undefined;
    defaultValue?: DateRangeValue;
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
    onValueChange?: ((value: DateRangeValue) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
  }

  let {
    value = undefined,
    defaultValue = { start: null, end: null },
    open = undefined,
    defaultOpen = false,
    placeholder = "Select date range",
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

  const surfaceId = `poodle-date-range-picker-surface-${++nextDateRangePickerId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = $state<DateRangeValue>({ start: null, end: null });
  let uncontrolledOpen = $state(false);
  let visibleMonth = $state(todayIsoDate());
  let seededDefaults = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledValue = $derived(value !== undefined);
  const hasControlledOpen = $derived(open !== undefined);
  const currentValue = $derived(normalizeDateRange(hasControlledValue ? value ?? { start: null, end: null } : uncontrolledValue));
  const isOpen = $derived(hasControlledOpen ? open === true : uncontrolledOpen);
  const valueLabel = $derived(
    currentValue.start
      ? `${formatDateLabel(currentValue.start, locale)}${
          currentValue.end ? ` – ${formatDateLabel(currentValue.end, locale)}` : " – End date"
        }`
      : placeholder,
  );

  $effect.pre(() => {
    if (seededDefaults) {
      return;
    }

    uncontrolledValue = normalizeDateRange(defaultValue);
    uncontrolledOpen = defaultOpen;
    visibleMonth = monthAnchorIso(defaultValue.start ?? todayIsoDate());
    seededDefaults = true;
  });

  $effect(() => {
    if (currentValue.start) {
      visibleMonth = monthAnchorIso(currentValue.start);
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

  function commitValue(nextValue: DateRangeValue): void {
    const normalized = normalizeDateRange(nextValue);

    if (!hasControlledValue) {
      uncontrolledValue = normalized;
    }

    if (normalized.start) {
      visibleMonth = monthAnchorIso(normalized.start);
    }

    if (normalized.start && normalized.end) {
      setOpen(false);
    }

    onValueChange?.(normalized);
  }

  function handleCalendarValueChange(nextValue: string | string[] | DateRangeValue): void {
    if (typeof nextValue === "object" && !Array.isArray(nextValue)) {
      commitValue(nextValue);
    }
  }
</script>

<div bind:this={rootElement} class="poodle-date-range-picker" data-size={resolvedSize} data-density={resolvedDensity} data-open={isOpen}>
  <button
    type="button"
    class="poodle-date-range-picker__trigger"
    disabled={disabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    onclick={() => setOpen(!isOpen)}
  >
    <span class="poodle-date-range-picker__value" data-placeholder={currentValue.start === null}>
      {valueLabel}
    </span>
    <span class="poodle-date-range-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div
      id={surfaceId}
      class="poodle-date-range-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
      <Calendar
        mode="range"
        value={currentValue}
        visibleMonth={visibleMonth}
        {weekStartsOn}
        {locale}
        {disabled}
        size={resolvedSize}
        density={resolvedDensity}
        ariaLabel={ariaLabel ?? placeholder}
        onValueChange={handleCalendarValueChange}
        onMonthChange={(month) => (visibleMonth = month)}
      />
    </div>
  {/if}
</div>

<style>
  .poodle-date-range-picker {
    --poodle-date-range-picker-trigger-height: var(--poodle-size-control-height);
    position: relative;
    display: inline-grid;
    min-width: 16rem;
  }

  .poodle-date-range-picker__trigger {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    min-height: var(--poodle-date-range-picker-trigger-height);
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

  .poodle-date-range-picker__value {
    min-width: 0;
  }

  .poodle-date-range-picker__value[data-placeholder="true"] {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-date-range-picker__indicator {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
  }

  .poodle-date-range-picker__surface {
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

  .poodle-date-range-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--poodle-color-background-surface) 86%,
      var(--poodle-color-background-elevated)
    );
  }

  .poodle-date-range-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-date-range-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-date-range-picker[data-size="xs"] { --poodle-date-range-picker-trigger-height: 1.5rem; }
  .poodle-date-range-picker[data-size="xs"] .poodle-date-range-picker__trigger { font-size: 0.75rem; }

  .poodle-date-range-picker[data-size="xs"] .poodle-date-range-picker__indicator { font-size: 0.625rem; }

  .poodle-date-range-picker[data-size="sm"] { --poodle-date-range-picker-trigger-height: 1.75rem; }
  .poodle-date-range-picker[data-size="sm"] .poodle-date-range-picker__trigger { font-size: 0.8125rem; }

  .poodle-date-range-picker[data-size="sm"] .poodle-date-range-picker__indicator { font-size: 0.6875rem; }

  .poodle-date-range-picker[data-size="md"] {
    --poodle-date-range-picker-trigger-height: var(
      --poodle-size-control-height-md,
      var(--poodle-size-control-height)
    );
  }
  .poodle-date-range-picker[data-size="lg"] { --poodle-date-range-picker-trigger-height: 2.75rem; }
  .poodle-date-range-picker[data-size="lg"] .poodle-date-range-picker__trigger { font-size: 0.9375rem; }

  .poodle-date-range-picker[data-size="lg"] .poodle-date-range-picker__indicator { font-size: 0.8125rem; }

  .poodle-date-range-picker[data-size="xl"] { --poodle-date-range-picker-trigger-height: 3.25rem; }
  .poodle-date-range-picker[data-size="xl"] .poodle-date-range-picker__trigger { font-size: 1rem; }

  .poodle-date-range-picker[data-size="xl"] .poodle-date-range-picker__indicator { font-size: 0.875rem; }

  .poodle-date-range-picker[data-density="compact"] .poodle-date-range-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-date-range-picker[data-density="comfortable"] .poodle-date-range-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
