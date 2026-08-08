<script module lang="ts">
  let nextDateTimeZonePickerId = 0;
</script>

<script lang="ts">
  import { layerContains } from "@inflatable-cookie/poodle-core";
  import "@inflatable-cookie/poodle-core/styles/date-time-zone-picker.css";
  import { anchored } from "./anchored";
  import { default as Calendar } from "./Calendar.svelte";
  import { default as TimeInput } from "./TimeInput.svelte";
  import { default as TimeZoneSelect } from "./TimeZoneSelect.svelte";
  import {
    formatZonedDateTimeLabel,
    monthAnchorIso,
    normalizeZonedDateTimeValue,
    todayIsoDate,
  } from "./date";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { CalendarWeekStart, ControlDensity, ControlSize, SemanticControlSizeRole, TimeZoneOption, ZonedDateTimeValue } from "./types";

  interface Props {
    value?: ZonedDateTimeValue | null | undefined;
    defaultValue?: ZonedDateTimeValue;
    open?: boolean | null | undefined;
    defaultOpen?: boolean;
    placeholder?: string;
    weekStartsOn?: CalendarWeekStart;
    locale?: string;
    timeZoneOptions?: TimeZoneOption[];
    disabled?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: ZonedDateTimeValue) => void) | undefined;
    onOpenChange?: ((open: boolean) => void) | undefined;
  }

  let {
    value = undefined,
    defaultValue = { date: null, time: null, timeZone: null },
    open = undefined,
    defaultOpen = false,
    placeholder = "Select date, time, and zone",
    weekStartsOn = "monday",
    locale = "en-US",
    timeZoneOptions = [],
    disabled = false,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
    onOpenChange = undefined,
  }: Props = $props();

  const surfaceId = `poodle-date-time-zone-picker-surface-${++nextDateTimeZonePickerId}`;
  const uiPresentation = getUiPresentation();
  let rootElement: HTMLDivElement | null = $state(null);
  let surfaceElement: HTMLDivElement | null = $state(null);
  let uncontrolledValue = $state<ZonedDateTimeValue>({ date: null, time: null, timeZone: null });
  let uncontrolledOpen = $state(false);
  let visibleMonth = $state(todayIsoDate());
  let seededDefaults = $state(false);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasControlledValue = $derived(value !== undefined);
  const hasControlledOpen = $derived(open !== undefined);
  const currentValue = $derived(normalizeZonedDateTimeValue((hasControlledValue ? value : uncontrolledValue) ?? defaultValue));
  const isOpen = $derived(hasControlledOpen ? open === true : uncontrolledOpen);
  const valueLabel = $derived(
    formatZonedDateTimeLabel(currentValue, locale) ||
      (currentValue.date || currentValue.time || currentValue.timeZone ? "Complete zoned date and time" : placeholder),
  );

  $effect.pre(() => {
    if (seededDefaults) {
      return;
    }

    uncontrolledValue = normalizeZonedDateTimeValue(defaultValue);
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

  function commitValue(nextValue: ZonedDateTimeValue): void {
    const normalized = normalizeZonedDateTimeValue(nextValue);

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

  function handleTimeZoneChange(nextValue: string): void {
    commitValue({ ...currentValue, timeZone: nextValue });
  }
</script>

<div bind:this={rootElement} class="poodle-date-time-zone-picker" data-size={resolvedSize} data-density={resolvedDensity} data-open={isOpen}>
  <button
    type="button"
    class="poodle-date-time-zone-picker__trigger"
    disabled={disabled}
    aria-haspopup="dialog"
    aria-expanded={isOpen ? "true" : "false"}
    aria-controls={isOpen ? surfaceId : undefined}
    aria-label={ariaLabel ?? undefined}
    onclick={() => setOpen(!isOpen)}
  >
    <span
      class="poodle-date-time-zone-picker__value"
      data-placeholder={!currentValue.date || !currentValue.time || !currentValue.timeZone}
    >
      {valueLabel}
    </span>
    <span class="poodle-date-time-zone-picker__indicator" aria-hidden="true">▾</span>
  </button>

  {#if isOpen}
    <div
      bind:this={surfaceElement}
      use:anchored={{ anchor: rootElement, placement: "bottom-start", offset: 6 }}
      id={surfaceId}
      class="poodle-date-time-zone-picker__surface"
      role="dialog"
      aria-label={ariaLabel ?? placeholder}
    >
      <div class="poodle-date-time-zone-picker__body">
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

        <div class="poodle-date-time-zone-picker__fields">
          <div class="poodle-date-time-zone-picker__field">
            <label class="poodle-date-time-zone-picker__label" for={`${surfaceId}-time`}>
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

          <div class="poodle-date-time-zone-picker__field">
            <label class="poodle-date-time-zone-picker__label" for={`${surfaceId}-timezone`}>
              Time zone
            </label>
            <TimeZoneSelect
              id={`${surfaceId}-timezone`}
              value={currentValue.timeZone}
              options={timeZoneOptions}
              disabled={disabled}
              size={resolvedSize}
              density={resolvedDensity}
              ariaLabel={ariaLabel ? `${ariaLabel} time zone` : "Time zone"}
              onValueChange={handleTimeZoneChange}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

