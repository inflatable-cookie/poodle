import { useEffect, useId, useRef, useState } from "react";
import { formatDateLabel, monthAnchorIso, normalizeDateRange, todayIsoDate, layerContains, registerDismissLayer } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/date-range-picker.css";

import { AnchoredSurface } from "./AnchoredSurface";
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
  // The root is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const surfaceRef = useRef<HTMLDivElement | null>(null);
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
    return registerDismissLayer({
      contains: (target) => layerContains(target, rootElement, surfaceRef.current),
      dismissOnOutsideInteract: true,
      onDismiss: () => setOpenRef.current(false),
      hostElement: rootElement,
    });
  }, [isOpen, rootElement]);

  function commitValue(nextValue: DateRangeValue): void {
    const normalized = normalizeDateRange(nextValue);
    if (!hasControlledValue) setUncontrolledValue(normalized);
    if (normalized.start) setVisibleMonth(monthAnchorIso(normalized.start));
    if (normalized.start && normalized.end) setOpenRef.current(false);
    onValueChange?.(normalized);
  }

  return (
    <div
      ref={setRootElement}
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
        <AnchoredSurface
          ref={surfaceRef}
          anchor={rootElement}
          placement="bottom-start"
          offset={6}
          id={surfaceId}
          className="poodle-date-range-picker__surface"
          role="dialog"
          aria-label={ariaLabel ?? placeholder}
        >
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
        </AnchoredSurface>
      ) : null}
    </div>
  );
}
