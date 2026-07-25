<script module lang="ts">
  let nextDateTimeRangePickerId = 0;
</script>

<script lang="ts">
  import { layerContains } from "@poodle/headless";
  import "@poodle/styles/date-time-range-picker.css";
  import { anchored } from "./anchored";
  import { default as Calendar } from "./Calendar.svelte";
  import { default as TimeInput } from "./TimeInput.svelte";
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
  let rootElement: HTMLDivElement | null = $state(null);
  let surfaceElement: HTMLDivElement | null = $state(null);
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
  const currentValue = $derived(normalizeDateTimeRangeValue((hasControlledValue ? value : uncontrolledValue) ?? defaultValue));
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

  function handleRangeChange(nextRange: string | string[] | DateRangeValue): void {
    if (typeof nextRange !== "object" || Array.isArray(nextRange)) {
      return;
    }

    commitValue({
      start: { ...currentValue.start, date: nextRange.start },
      end: { ...currentValue.end, date: nextRange.end },
    });
  }

  function handleStartTimeChange(nextValue: string | null): void {
    commitValue({
      start: { ...currentValue.start, time: nextValue },
      end: currentValue.end,
    });
  }

  function handleEndTimeChange(nextValue: string | null): void {
    commitValue({
      start: currentValue.start,
      end: { ...currentValue.end, time: nextValue },
    });
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
      bind:this={surfaceElement}
      use:anchored={{ anchor: rootElement, placement: "bottom-start", offset: 6 }}
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
          onValueChange={handleRangeChange}
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
              onValueChange={handleStartTimeChange}
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
              onValueChange={handleEndTimeChange}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

