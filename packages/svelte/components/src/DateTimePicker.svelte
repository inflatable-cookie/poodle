<script module lang="ts">
  let nextDateTimePickerId = 0;
</script>

<script lang="ts">
  import "@poodle/styles/date-time-picker.css";
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

