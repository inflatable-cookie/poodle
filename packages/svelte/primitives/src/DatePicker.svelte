<script context="module" lang="ts">
  let nextDatePickerId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import Calendar from "./Calendar.svelte";
  import { formatDateLabel, monthAnchorIso, todayIsoDate } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placeholder = "Select date";
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    openChange: { open: boolean };
  }>();

  const surfaceId = `poodle-date-picker-surface-${++nextDatePickerId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = defaultValue;
  let uncontrolledOpen = defaultOpen;
  let visibleMonth = monthAnchorIso(defaultValue ?? todayIsoDate());

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: currentValue = value ?? uncontrolledValue;
  $: isOpen = open ?? uncontrolledOpen;
  $: if (currentValue) {
    visibleMonth = monthAnchorIso(currentValue);
  }
  $: valueLabel = currentValue ? formatDateLabel(currentValue, locale) : placeholder;

  function setOpen(nextOpen: boolean): void {
    if (open === null) {
      uncontrolledOpen = nextOpen;
    }

    dispatch("openChange", { open: nextOpen });
  }

  function commitValue(nextValue: string): void {
    if (value === null) {
      uncontrolledValue = nextValue;
    }

    visibleMonth = monthAnchorIso(nextValue);
    setOpen(false);
    dispatch("valueChange", { value: nextValue });
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

<div bind:this={rootElement} class="date-picker" data-size={resolvedSize} data-density={resolvedDensity} data-open={isOpen}>
  <button
    type="button"
    class="date-picker__trigger"
    disabled={disabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    on:click={() => setOpen(!isOpen)}
  >
    <span class="date-picker__value" data-placeholder={currentValue === null}>
      {valueLabel}
    </span>
    <span class="date-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div id={surfaceId} class="date-picker__surface" role="dialog" aria-label={ariaLabel ?? placeholder}>
      <Calendar
        value={currentValue}
        visibleMonth={visibleMonth}
        {weekStartsOn}
        {locale}
        {disabled}
        size={resolvedSize}
        density={resolvedDensity}
        ariaLabel={ariaLabel ?? placeholder}
        on:valueChange={(event) => commitValue(event.detail.value)}
        on:monthChange={(event) => (visibleMonth = event.detail.month)}
      />
    </div>
  {/if}
</div>

<style>
  .date-picker {
    position: relative;
    display: inline-grid;
    min-width: 14rem;
  }

  .date-picker__trigger {
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

  .date-picker__value {
    min-width: 0;
  }

  .date-picker__value[data-placeholder="true"] {
    color: var(--poodle-color-text-secondary);
  }

  .date-picker__indicator {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
  }

  .date-picker__surface {
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

  .date-picker__trigger:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 86%, var(--poodle-color-background-elevated));
  }

  .date-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .date-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .date-picker[data-size="xs"] .date-picker__trigger { min-height: calc(var(--poodle-size-control-height) - 0.5rem); font-size: 0.75rem; }
  .date-picker[data-size="xs"] .date-picker__indicator { font-size: 0.625rem; }
  .date-picker[data-size="sm"] .date-picker__trigger { min-height: calc(var(--poodle-size-control-height) - 0.25rem); font-size: 0.8125rem; }
  .date-picker[data-size="sm"] .date-picker__indicator { font-size: 0.6875rem; }
  .date-picker[data-size="lg"] .date-picker__trigger { min-height: calc(var(--poodle-size-control-height) + 0.25rem); font-size: 0.9375rem; }
  .date-picker[data-size="lg"] .date-picker__indicator { font-size: 0.8125rem; }
  .date-picker[data-size="xl"] .date-picker__trigger { min-height: calc(var(--poodle-size-control-height) + 0.5rem); font-size: 1rem; }
  .date-picker[data-size="xl"] .date-picker__indicator { font-size: 0.875rem; }

  /* Density variants */
  .date-picker[data-density="compact"] .date-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .date-picker[data-density="comfortable"] .date-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
