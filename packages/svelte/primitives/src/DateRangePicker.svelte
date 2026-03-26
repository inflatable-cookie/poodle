<script context="module" lang="ts">
  let nextDateRangePickerId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import RangeCalendar from "./RangeCalendar.svelte";
  import { formatDateLabel, monthAnchorIso, normalizeDateRange, todayIsoDate } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlSize, DateRangeValue, SemanticControlSizeRole } from "./types";

  export let value: DateRangeValue | null = null;
  export let defaultValue: DateRangeValue = { start: null, end: null };
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placeholder = "Select date range";
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";

  const dispatch = createEventDispatcher<{
    valueChange: { value: DateRangeValue };
    openChange: { open: boolean };
  }>();

  const surfaceId = `poodle-date-range-picker-surface-${++nextDateRangePickerId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = normalizeDateRange(defaultValue);
  let uncontrolledOpen = defaultOpen;
  let visibleMonth = monthAnchorIso(defaultValue.start ?? todayIsoDate());

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: currentValue = normalizeDateRange(value ?? uncontrolledValue);
  $: isOpen = open ?? uncontrolledOpen;
  $: if (currentValue.start) {
    visibleMonth = monthAnchorIso(currentValue.start);
  }
  $: valueLabel = currentValue.start
    ? `${formatDateLabel(currentValue.start, locale)}${
        currentValue.end ? ` – ${formatDateLabel(currentValue.end, locale)}` : " – End date"
      }`
    : placeholder;

  function setOpen(nextOpen: boolean): void {
    if (open === null) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function commitValue(nextValue: DateRangeValue): void {
    const normalized = normalizeDateRange(nextValue);

    if (value === null) {
      uncontrolledValue = normalized;
    }

    if (normalized.start) {
      visibleMonth = monthAnchorIso(normalized.start);
    }

    if (normalized.start && normalized.end) {
      setOpen(false);
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

<div bind:this={rootElement} class="date-range-picker" data-size={resolvedSize} data-open={isOpen}>
  <button
    type="button"
    class="date-range-picker__trigger"
    disabled={disabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    on:click={() => setOpen(!isOpen)}
  >
    <span class="date-range-picker__value" data-placeholder={currentValue.start === null}>
      {valueLabel}
    </span>
    <span class="date-range-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div
      id={surfaceId}
      class="date-range-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
      <RangeCalendar
        value={currentValue}
        visibleMonth={visibleMonth}
        {weekStartsOn}
        {locale}
        {disabled}
        ariaLabel={ariaLabel ?? placeholder}
        on:valueChange={(event) => commitValue(event.detail.value)}
        on:monthChange={(event) => (visibleMonth = event.detail.month)}
      />
    </div>
  {/if}
</div>

<style>
  .date-range-picker {
    position: relative;
    display: inline-grid;
    min-width: 16rem;
  }

  .date-range-picker__trigger {
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

  .date-range-picker__value {
    min-width: 0;
  }

  .date-range-picker__value[data-placeholder="true"] {
    color: var(--poodle-color-text-secondary);
  }

  .date-range-picker__indicator {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
  }

  .date-range-picker__surface {
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

  .date-range-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--poodle-color-background-surface) 86%,
      var(--poodle-color-background-elevated)
    );
  }

  .date-range-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .date-range-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .date-range-picker[data-size="xs"] .date-range-picker__trigger {
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
    font-size: 0.75rem;
  }

  .date-range-picker[data-size="sm"] .date-range-picker__trigger {
    min-height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .date-range-picker[data-size="lg"] .date-range-picker__trigger {
    min-height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
    font-size: 0.9375rem;
  }

  .date-range-picker[data-size="xl"] .date-range-picker__trigger {
    min-height: calc(var(--poodle-size-control-height) + 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.1875rem);
    font-size: 1rem;
  }
</style>
