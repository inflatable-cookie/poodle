<script context="module" lang="ts">
  let nextDatePickerId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";

  import Calendar from "./Calendar.svelte";
  import { formatDateLabel, monthAnchorIso, todayIsoDate } from "./date";

  import type { CalendarWeekStart } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let open: boolean | null = null;
  export let defaultOpen = false;
  export let placeholder = "Select date";
  export let weekStartsOn: CalendarWeekStart = "monday";
  export let locale = "en-US";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    openChange: { open: boolean };
  }>();

  const surfaceId = `flint-date-picker-surface-${++nextDatePickerId}`;
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = defaultValue;
  let uncontrolledOpen = defaultOpen;
  let visibleMonth = monthAnchorIso(defaultValue ?? todayIsoDate());

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

<div bind:this={rootElement} class="date-picker" data-open={isOpen}>
  <button
    type="button"
    class="date-picker__trigger"
    disabled={isDisabled}
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
        {isDisabled}
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

  .date-picker__value {
    min-width: 0;
  }

  .date-picker__value[data-placeholder="true"] {
    color: var(--flint-color-text-secondary);
  }

  .date-picker__indicator {
    color: var(--flint-color-text-secondary);
    font-size: 0.75rem;
  }

  .date-picker__surface {
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

  .date-picker__trigger:hover:not(:disabled) {
    background: color-mix(in srgb, var(--flint-color-background-surface) 86%, var(--flint-color-background-elevated));
  }

  .date-picker__trigger:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .date-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
