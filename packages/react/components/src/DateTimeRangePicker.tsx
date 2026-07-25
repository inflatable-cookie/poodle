import { useEffect, useId, useRef, useState } from "react";
import {
  formatDateTimeRangeLabel,
  monthAnchorIso,
  normalizeDateRange,
  normalizeDateTimeRangeValue,
  todayIsoDate,
  layerContains,
} from "@poodle/headless";

import "@poodle/styles/date-time-range-picker.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { Calendar } from "./Calendar";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import { TimeInput } from "./TimeInput";
import type {
  CalendarWeekStart,
  ControlDensity,
  ControlSize,
  DateRangeValue,
  DateTimeRangeValue,
  SemanticControlSizeRole,
} from "./types";

export interface DateTimeRangePickerProps {
  value?: DateTimeRangeValue | null;
  defaultValue?: DateTimeRangeValue;
  open?: boolean;
  defaultOpen?: boolean;
  placeholder?: string;
  weekStartsOn?: CalendarWeekStart;
  locale?: string;
  disabled?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: (value: DateTimeRangeValue) => void;
  onOpenChange?: (open: boolean) => void;
}

const EMPTY: DateTimeRangeValue = { start: { date: null, time: null }, end: { date: null, time: null } };

export function DateTimeRangePicker({
  value,
  defaultValue = EMPTY,
  open,
  defaultOpen = false,
  placeholder = "Select date and time range",
  weekStartsOn = "monday",
  locale = "en-US",
  disabled = false,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
  onOpenChange,
}: DateTimeRangePickerProps) {
  const surfaceId = useId();
  const uiPresentation = useUiPresentation();
  // The root is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const surfaceRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledValue, setUncontrolledValue] = useState<DateTimeRangeValue>(() =>
    normalizeDateTimeRangeValue(defaultValue),
  );
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [visibleMonth, setVisibleMonth] = useState(() => monthAnchorIso(defaultValue.start.date ?? todayIsoDate()));

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasControlledValue = value !== undefined;
  const hasControlledOpen = open !== undefined;
  const currentValue = normalizeDateTimeRangeValue((hasControlledValue ? value : uncontrolledValue) ?? defaultValue);
  const currentRange = normalizeDateRange({ start: currentValue.start.date, end: currentValue.end.date });
  const isOpen = hasControlledOpen ? open === true : uncontrolledOpen;
  const valueLabel = formatDateTimeRangeLabel(currentValue, locale) || placeholder;

  useEffect(() => {
    if (currentValue.start.date) setVisibleMonth(monthAnchorIso(currentValue.start.date));
  }, [currentValue.start.date]);

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

  function commitValue(nextValue: DateTimeRangeValue): void {
    const normalized = normalizeDateTimeRangeValue(nextValue);
    if (!hasControlledValue) setUncontrolledValue(normalized);
    if (normalized.start.date) setVisibleMonth(monthAnchorIso(normalized.start.date));
    onValueChange?.(normalized);
  }

  return (
    <div
      ref={setRootElement}
      className="poodle-date-time-range-picker"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-open={isOpen}
    >
      <button
        type="button"
        className="poodle-date-time-range-picker__trigger"
        disabled={disabled}
        aria-haspopup="dialog"
        aria-expanded={isOpen}
        aria-controls={isOpen ? surfaceId : undefined}
        aria-label={ariaLabel ?? undefined}
        onClick={() => setOpenRef.current(!isOpen)}
      >
        <span
          className="poodle-date-time-range-picker__value"
          data-placeholder={!currentValue.start.date || !currentValue.end.date}
        >
          {valueLabel}
        </span>
        <span className="poodle-date-time-range-picker__indicator" aria-hidden="true">
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
          className="poodle-date-time-range-picker__surface"
          role="dialog"
          aria-label={ariaLabel ?? placeholder}
        >
          <div className="poodle-date-time-range-picker__body">
            <Calendar
              mode="range"
              value={currentRange}
              visibleMonth={visibleMonth}
              weekStartsOn={weekStartsOn}
              locale={locale}
              disabled={disabled}
              size={resolvedSize}
              density={resolvedDensity}
              ariaLabel={ariaLabel ?? placeholder}
              onValueChange={(nextRange: string | DateRangeValue) => {
                if (typeof nextRange !== "object" || Array.isArray(nextRange)) return;
                commitValue({
                  start: { ...currentValue.start, date: nextRange.start },
                  end: { ...currentValue.end, date: nextRange.end },
                });
              }}
              onMonthChange={setVisibleMonth}
            />

            <div className="poodle-date-time-range-picker__times">
              <div className="poodle-date-time-range-picker__time-section">
                <label className="poodle-date-time-range-picker__time-label" htmlFor={`${surfaceId}-start-time`}>
                  Start time
                </label>
                <TimeInput
                  id={`${surfaceId}-start-time`}
                  value={currentValue.start.time}
                  disabled={disabled}
                  size={resolvedSize}
                  density={resolvedDensity}
                  ariaLabel={ariaLabel ? `${ariaLabel} start time` : "Start time"}
                  onValueChange={(nextValue) =>
                    commitValue({ start: { ...currentValue.start, time: nextValue }, end: currentValue.end })
                  }
                />
              </div>

              <div className="poodle-date-time-range-picker__time-section">
                <label className="poodle-date-time-range-picker__time-label" htmlFor={`${surfaceId}-end-time`}>
                  End time
                </label>
                <TimeInput
                  id={`${surfaceId}-end-time`}
                  value={currentValue.end.time}
                  disabled={disabled}
                  size={resolvedSize}
                  density={resolvedDensity}
                  ariaLabel={ariaLabel ? `${ariaLabel} end time` : "End time"}
                  onValueChange={(nextValue) =>
                    commitValue({ start: currentValue.start, end: { ...currentValue.end, time: nextValue } })
                  }
                />
              </div>
            </div>
          </div>
        </AnchoredSurface>
      ) : null}
    </div>
  );
}
