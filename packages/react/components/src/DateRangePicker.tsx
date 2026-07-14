import { useEffect, useId, useRef, useState } from "react";
import { formatDateLabel, monthAnchorIso, normalizeDateRange, todayIsoDate } from "@poodle/headless";

import "@poodle/styles/date-range-picker.css";

import { Calendar } from "./Calendar";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  CalendarWeekStart,
  ControlDensity,
  ControlSize,
  DateRangeValue,
  SemanticControlSizeRole,
} from "./types";

export interface DateRangePickerProps {
  value?: DateRangeValue | null;
  defaultValue?: DateRangeValue;
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
  onValueChange?: (value: DateRangeValue) => void;
  onOpenChange?: (open: boolean) => void;
}

export function DateRangePicker({
  value,
  defaultValue = { start: null, end: null },
  open,
  defaultOpen = false,
  placeholder = "Select date range",
  weekStartsOn = "monday",
  locale = "en-US",
  disabled = false,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
  onOpenChange,
}: DateRangePickerProps) {
  const surfaceId = useId();
  const uiPresentation = useUiPresentation();
  const rootRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledValue, setUncontrolledValue] = useState<DateRangeValue>(() => normalizeDateRange(defaultValue));
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [visibleMonth, setVisibleMonth] = useState(() => monthAnchorIso(defaultValue.start ?? todayIsoDate()));

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasControlledValue = value !== undefined;
  const hasControlledOpen = open !== undefined;
  const currentValue = normalizeDateRange(hasControlledValue ? (value ?? { start: null, end: null }) : uncontrolledValue);
  const isOpen = hasControlledOpen ? open === true : uncontrolledOpen;
  const valueLabel = currentValue.start
    ? `${formatDateLabel(currentValue.start, locale)}${
        currentValue.end ? ` – ${formatDateLabel(currentValue.end, locale)}` : " – End date"
      }`
    : placeholder;

  useEffect(() => {
    if (currentValue.start) setVisibleMonth(monthAnchorIso(currentValue.start));
  }, [currentValue.start]);

  const setOpenRef = useRef<(nextOpen: boolean) => void>(() => {});
  setOpenRef.current = (nextOpen: boolean) => {
    if (!hasControlledOpen) setUncontrolledOpen(nextOpen);
    onOpenChange?.(nextOpen);
  };

  useEffect(() => {
    if (!isOpen) return;
    function handlePointerDown(event: MouseEvent): void {
      if (!rootRef.current) return;
      if (!rootRef.current.contains(event.target as Node)) setOpenRef.current(false);
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

  function commitValue(nextValue: DateRangeValue): void {
    const normalized = normalizeDateRange(nextValue);
    if (!hasControlledValue) setUncontrolledValue(normalized);
    if (normalized.start) setVisibleMonth(monthAnchorIso(normalized.start));
    if (normalized.start && normalized.end) setOpenRef.current(false);
    onValueChange?.(normalized);
  }

  return (
    <div
      ref={rootRef}
      className="poodle-date-range-picker"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-open={isOpen}
    >
      <button
        type="button"
        className="poodle-date-range-picker__trigger"
        disabled={disabled}
        aria-haspopup="dialog"
        aria-expanded={isOpen}
        aria-controls={isOpen ? surfaceId : undefined}
        aria-label={ariaLabel ?? undefined}
        onClick={() => setOpenRef.current(!isOpen)}
      >
        <span className="poodle-date-range-picker__value" data-placeholder={currentValue.start === null}>
          {valueLabel}
        </span>
        <span className="poodle-date-range-picker__indicator" aria-hidden="true">
          ▾
        </span>
      </button>

      {isOpen ? (
        <div id={surfaceId} className="poodle-date-range-picker__surface" role="dialog" aria-label={ariaLabel ?? placeholder}>
          <Calendar
            mode="range"
            value={currentValue}
            visibleMonth={visibleMonth}
            weekStartsOn={weekStartsOn}
            locale={locale}
            disabled={disabled}
            size={resolvedSize}
            density={resolvedDensity}
            ariaLabel={ariaLabel ?? placeholder}
            onValueChange={(nextValue: string | DateRangeValue) => {
              if (typeof nextValue === "object" && !Array.isArray(nextValue)) commitValue(nextValue);
            }}
            onMonthChange={setVisibleMonth}
          />
        </div>
      ) : null}
    </div>
  );
}
