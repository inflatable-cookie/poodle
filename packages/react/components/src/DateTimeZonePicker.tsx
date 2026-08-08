import { useEffect, useId, useRef, useState } from "react";
import { formatZonedDateTimeLabel, monthAnchorIso, normalizeZonedDateTimeValue, todayIsoDate, layerContains } from "@inflatable-cookie/poodle-headless";

import "@inflatable-cookie/poodle-styles/date-time-zone-picker.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { Calendar } from "./Calendar";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import { TimeInput } from "./TimeInput";
import { TimeZoneSelect } from "./TimeZoneSelect";
import type {
  CalendarWeekStart,
  ControlDensity,
  ControlSize,
  DateRangeValue,
  SemanticControlSizeRole,
  TimeZoneOption,
  ZonedDateTimeValue,
} from "./types";

export interface DateTimeZonePickerProps {
  value?: ZonedDateTimeValue | null;
  defaultValue?: ZonedDateTimeValue;
  open?: boolean;
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
  onValueChange?: (value: ZonedDateTimeValue) => void;
  onOpenChange?: (open: boolean) => void;
}

export function DateTimeZonePicker({
  value,
  defaultValue = { date: null, time: null, timeZone: null },
  open,
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
  onValueChange,
  onOpenChange,
}: DateTimeZonePickerProps) {
  const surfaceId = useId();
  const uiPresentation = useUiPresentation();
  // The root is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const surfaceRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledValue, setUncontrolledValue] = useState<ZonedDateTimeValue>(() =>
    normalizeZonedDateTimeValue(defaultValue),
  );
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [visibleMonth, setVisibleMonth] = useState(() => monthAnchorIso(defaultValue.date ?? todayIsoDate()));

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasControlledValue = value !== undefined;
  const hasControlledOpen = open !== undefined;
  const currentValue = normalizeZonedDateTimeValue((hasControlledValue ? value : uncontrolledValue) ?? defaultValue);
  const isOpen = hasControlledOpen ? open === true : uncontrolledOpen;
  const valueLabel =
    formatZonedDateTimeLabel(currentValue, locale) ||
    (currentValue.date || currentValue.time || currentValue.timeZone ? "Complete zoned date and time" : placeholder);

  useEffect(() => {
    if (currentValue.date) setVisibleMonth(monthAnchorIso(currentValue.date));
  }, [currentValue.date]);

  const setOpenRef = useRef<(nextOpen: boolean) => void>(() => {});
  setOpenRef.current = (nextOpen: boolean) => {
    if (!hasControlledOpen) setUncontrolledOpen(nextOpen);
    onOpenChange?.(nextOpen);
  };

  useEffect(() => {
    if (!isOpen) return;
    function handlePointerDown(event: MouseEvent): void {
      // The surface is portalled out of the root, so both count as inside.
      if (!layerContains(event.target as Node, rootElement, surfaceRef.current)) {
        setOpenRef.current(false);
      }
    }
    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape") {
        event.preventDefault();
        setOpenRef.current(false);
      }
    }
    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);
    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
  }, [isOpen]);

  function commitValue(nextValue: ZonedDateTimeValue): void {
    const normalized = normalizeZonedDateTimeValue(nextValue);
    if (!hasControlledValue) setUncontrolledValue(normalized);
    if (normalized.date) setVisibleMonth(monthAnchorIso(normalized.date));
    onValueChange?.(normalized);
  }

  return (
    <div
      ref={setRootElement}
      className="poodle-date-time-zone-picker"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-open={isOpen}
    >
      <button
        type="button"
        className="poodle-date-time-zone-picker__trigger"
        disabled={disabled}
        aria-haspopup="dialog"
        aria-expanded={isOpen}
        aria-controls={isOpen ? surfaceId : undefined}
        aria-label={ariaLabel ?? undefined}
        onClick={() => setOpenRef.current(!isOpen)}
      >
        <span
          className="poodle-date-time-zone-picker__value"
          data-placeholder={!currentValue.date || !currentValue.time || !currentValue.timeZone}
        >
          {valueLabel}
        </span>
        <span className="poodle-date-time-zone-picker__indicator" aria-hidden="true">
          ▾
        </span>
      </button>

      {isOpen ? (
        <AnchoredSurface
          ref={surfaceRef}
          anchor={rootElement}
          placement="bottom-start"
          offset={6}
          id={surfaceId}
          className="poodle-date-time-zone-picker__surface"
          role="dialog"
          aria-label={ariaLabel ?? placeholder}
        >
          <div className="poodle-date-time-zone-picker__body">
            <Calendar
              value={currentValue.date}
              visibleMonth={visibleMonth}
              weekStartsOn={weekStartsOn}
              locale={locale}
              disabled={disabled}
              size={resolvedSize}
              density={resolvedDensity}
              ariaLabel={ariaLabel ?? "Date"}
              onValueChange={(nextValue: string | DateRangeValue) => {
                if (typeof nextValue === "string") commitValue({ ...currentValue, date: nextValue });
              }}
              onMonthChange={setVisibleMonth}
            />

            <div className="poodle-date-time-zone-picker__fields">
              <div className="poodle-date-time-zone-picker__field">
                <label className="poodle-date-time-zone-picker__label" htmlFor={`${surfaceId}-time`}>
                  Time
                </label>
                <TimeInput
                  id={`${surfaceId}-time`}
                  value={currentValue.time}
                  disabled={disabled}
                  size={resolvedSize}
                  density={resolvedDensity}
                  ariaLabel={ariaLabel ? `${ariaLabel} time` : "Time"}
                  onValueChange={(nextValue) => commitValue({ ...currentValue, time: nextValue })}
                />
              </div>

              <div className="poodle-date-time-zone-picker__field">
                <label className="poodle-date-time-zone-picker__label" htmlFor={`${surfaceId}-timezone`}>
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
                  onValueChange={(nextValue) => commitValue({ ...currentValue, timeZone: nextValue })}
                />
              </div>
            </div>
          </div>
        </AnchoredSurface>
      ) : null}
    </div>
  );
}
