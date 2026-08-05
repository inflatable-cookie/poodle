<script module lang="ts">
  let nextDatePickerId = 0;
</script>

<script lang="ts">
  import { layerContains } from "@poodle/headless";
  import "@poodle/styles/date-picker.css";
  import { anchored } from "./anchored.ts";
  import { default as Calendar } from "./Calendar.svelte";
  import { formatDateLabel, monthAnchorIso, todayIsoDate } from "./date.ts";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation.ts";

  import type { CalendarWeekStart, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types.ts";

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
  let rootElement: HTMLDivElement | null = $state(null);
  let surfaceElement: HTMLDivElement | null = $state(null);
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

      // The surface is portalled out of the root, so both count as inside.
      if (!layerContains(event.target as Node, rootElement, surfaceElement)) {
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
    <div
      bind:this={surfaceElement}
      use:anchored={{ anchor: rootElement, placement: "bottom-start", offset: 6 }}
      id={surfaceId}
      class="poodle-date-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
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

