<script module lang="ts">
  let nextDatePickerId = 0;
</script>

<script lang="ts">
  import { default as Calendar } from "./Calendar.svelte";
  import { formatDateLabel, monthAnchorIso, todayIsoDate } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    value?: string | null | undefined;
    defaultValue?: string | null;
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
    onValueChange?: ((value: string) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
  }

  let {
    value = undefined,
    defaultValue = null,
    open = undefined,
    defaultOpen = false,
    placeholder = "Select date",
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

  const surfaceId = `poodle-date-picker-surface-${++nextDatePickerId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = null;
  let uncontrolledValue = $state<string | null>(null);
  let uncontrolledOpen = $state(false);
  let visibleMonth = $state(todayIsoDate());
  let seededDefaults = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledValue = $derived(value !== undefined);
  const hasControlledOpen = $derived(open !== undefined);
  const currentValue = $derived(hasControlledValue ? value ?? null : uncontrolledValue);
  const isOpen = $derived(hasControlledOpen ? open === true : uncontrolledOpen);
  const valueLabel = $derived(currentValue ? formatDateLabel(currentValue, locale) : placeholder);

  $effect.pre(() => {
    if (seededDefaults) {
      return;
    }

    uncontrolledValue = defaultValue;
    uncontrolledOpen = defaultOpen;
    visibleMonth = monthAnchorIso(defaultValue ?? todayIsoDate());
    seededDefaults = true;
  });

  $effect(() => {
    if (currentValue) {
      visibleMonth = monthAnchorIso(currentValue);
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

  function commitValue(nextValue: string): void {
    if (!hasControlledValue) {
      uncontrolledValue = nextValue;
    }

    visibleMonth = monthAnchorIso(nextValue);
    setOpen(false);
    onValueChange?.(nextValue);
  }

  function handleCalendarValueChange(nextValue: string | string[] | { start: string | null; end: string | null }): void {
    if (typeof nextValue === "string") {
      commitValue(nextValue);
    }
  }
</script>

<div bind:this={rootElement} class="poodle-date-picker" data-size={resolvedSize} data-density={resolvedDensity} data-open={isOpen}>
  <button
    type="button"
    class="poodle-date-picker__trigger"
    disabled={disabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    onclick={() => setOpen(!isOpen)}
  >
    <span class="poodle-date-picker__value" data-placeholder={currentValue === null}>
      {valueLabel}
    </span>
    <span class="poodle-date-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div id={surfaceId} class="poodle-date-picker__surface" role="dialog" aria-label={ariaLabel ?? placeholder}>
      <Calendar
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
  .poodle-date-picker {
    --poodle-date-picker-trigger-height: var(--poodle-size-control-height);
    position: relative;
    display: inline-grid;
    min-width: 14rem;
  }

  .poodle-date-picker__trigger {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    min-height: var(--poodle-date-picker-trigger-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-recipe-date-picker-trigger-fill, var(--poodle-color-background-surface));
    color: var(--poodle-recipe-date-picker-trigger-text, var(--poodle-color-text-primary));
    cursor: pointer;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    text-align: left;
  }

  .poodle-date-picker__value {
    min-width: 0;
  }

  .poodle-date-picker__value[data-placeholder="true"] {
    color: var(--poodle-recipe-date-picker-value-text, var(--poodle-color-text-secondary));
  }

  .poodle-date-picker__indicator {
    color: var(--poodle-recipe-date-picker-indicator-text, var(--poodle-color-text-secondary));
    font-size: 0.75rem;
  }

  .poodle-date-picker__surface {
    position: absolute;
    top: calc(100% + 0.375rem);
    left: 0;
    z-index: var(--poodle-overlay-z-menu);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-recipe-date-picker-surface-fill, color-mix(
      in srgb,
      var(--poodle-color-background-elevated) 98%,
      var(--poodle-color-background-panel)
    ));
    box-shadow: var(--poodle-recipe-date-picker-surface-shadow, var(--poodle-elevation-overlay));
  }

  .poodle-date-picker__trigger:hover:not(:disabled) {
    background: var(--poodle-recipe-date-picker-hover-trigger-fill, color-mix(in srgb, var(--poodle-color-background-surface) 86%, var(--poodle-color-background-elevated)));
  }

  .poodle-date-picker__trigger:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-date-picker__trigger:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-date-picker[data-size="xs"] { --poodle-date-picker-trigger-height: 1.5rem; }
  .poodle-date-picker[data-size="xs"] .poodle-date-picker__trigger { font-size: 0.75rem; }
  .poodle-date-picker[data-size="xs"] .poodle-date-picker__indicator { font-size: 0.625rem; }
  .poodle-date-picker[data-size="sm"] { --poodle-date-picker-trigger-height: 1.75rem; }
  .poodle-date-picker[data-size="sm"] .poodle-date-picker__trigger { font-size: 0.8125rem; }
  .poodle-date-picker[data-size="sm"] .poodle-date-picker__indicator { font-size: 0.6875rem; }
  .poodle-date-picker[data-size="md"] {
    --poodle-date-picker-trigger-height: var(
      --poodle-size-control-height-md,
      var(--poodle-size-control-height)
    );
  }
  .poodle-date-picker[data-size="lg"] { --poodle-date-picker-trigger-height: 2.75rem; }
  .poodle-date-picker[data-size="lg"] .poodle-date-picker__trigger { font-size: 0.9375rem; }
  .poodle-date-picker[data-size="lg"] .poodle-date-picker__indicator { font-size: 0.8125rem; }
  .poodle-date-picker[data-size="xl"] { --poodle-date-picker-trigger-height: 3.25rem; }
  .poodle-date-picker[data-size="xl"] .poodle-date-picker__trigger { font-size: 1rem; }
  .poodle-date-picker[data-size="xl"] .poodle-date-picker__indicator { font-size: 0.875rem; }

  .poodle-date-picker[data-density="compact"] .poodle-date-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) - 0.125rem); }
  .poodle-date-picker[data-density="comfortable"] .poodle-date-picker__trigger { padding: 0 calc(var(--poodle-space-control-x) + 0.125rem); }
</style>
