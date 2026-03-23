<script context="module" lang="ts">
  let nextDateRangePickerId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import RangeCalendar from "./RangeCalendar.svelte";
  import { formatDateLabel, monthAnchorIso, normalizeDateRange, todayIsoDate } from "./date";

  import type { CalendarWeekStart, DateRangeValue } from "./types";

  export let value: DateRangeValue | null = null;
  export let defaultValue: DateRangeValue = { start: null, end: null };
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placeholder = "Select date range";
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: DateRangeValue };
    openChange: { open: boolean };
  }>();

  const surfaceId = `flint-date-range-picker-surface-${++nextDateRangePickerId}`;
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = normalizeDateRange(defaultValue);
  let uncontrolledOpen = defaultOpen;
  let visibleMonth = monthAnchorIso(defaultValue.start ?? todayIsoDate());

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

<div bind:this={rootElement} class="date-range-picker" data-open={isOpen}>
  <button
    type="button"
    class="date-range-picker__trigger"
    disabled={isDisabled}
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
        {isDisabled}
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

  .date-range-picker__value {
    min-width: 0;
  }

  .date-range-picker__value[data-placeholder="true"] {
    color: var(--flint-color-text-secondary);
  }

  .date-range-picker__indicator {
    color: var(--flint-color-text-secondary);
    font-size: 0.75rem;
  }

  .date-range-picker__surface {
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

  .date-range-picker__trigger:hover:not(:disabled) {
    background: color-mix(
      in srgb,
      var(--flint-color-background-surface) 86%,
      var(--flint-color-background-elevated)
    );
  }

  .date-range-picker__trigger:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .date-range-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
