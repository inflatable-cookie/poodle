import { useEffect, useId, useRef, useState } from "react";
import { formatDateLabel, monthAnchorIso, todayIsoDate, layerContains } from "@poodle/headless";

import "@poodle/styles/date-picker.css";

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

export interface DatePickerProps {
  value?: string | null;
  defaultValue?: string | null;
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
  onValueChange?: (value: string) => void;
  onOpenChange?: (open: boolean) => void;
}

export function DatePicker({
  value,
  defaultValue = null,
  open,
  defaultOpen = false,
  placeholder = "Select date",
  weekStartsOn = "monday",
  locale = "en-US",
  disabled = false,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
  onOpenChange,
}: DatePickerProps) {
  const surfaceId = useId();
  const uiPresentation = useUiPresentation();
  // The root is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const surfaceRef = useRef<HTMLDivElement | null>(null);
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const [visibleMonth, setVisibleMonth] = useState(() => monthAnchorIso(defaultValue ?? todayIsoDate()));

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasControlledValue = value !== undefined;
  const hasControlledOpen = open !== undefined;
  const currentValue = hasControlledValue ? (value ?? null) : uncontrolledValue;
  const isOpen = hasControlledOpen ? open === true : uncontrolledOpen;
  const valueLabel = currentValue ? formatDateLabel(currentValue, locale) : placeholder;

  // follow the committed value's month
  useEffect(() => {
    if (currentValue) setVisibleMonth(monthAnchorIso(currentValue));
  }, [currentValue]);

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

  function commitValue(nextValue: string): void {
    if (!hasControlledValue) setUncontrolledValue(nextValue);
    setVisibleMonth(monthAnchorIso(nextValue));
    setOpenRef.current(false);
    onValueChange?.(nextValue);
  }

  return (
    <div
      ref={setRootElement}
      className="poodle-date-picker"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-open={isOpen}
    >
      <button
        type="button"
        className="poodle-date-picker__trigger"
        disabled={disabled}
        aria-haspopup="dialog"
        aria-expanded={isOpen}
        aria-controls={isOpen ? surfaceId : undefined}
        aria-label={ariaLabel ?? undefined}
        onClick={() => setOpenRef.current(!isOpen)}
      >
        <span className="poodle-date-picker__value" data-placeholder={currentValue === null}>
          {valueLabel}
        </span>
        <span className="poodle-date-picker__indicator" aria-hidden="true">
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
          className="poodle-date-picker__surface"
          role="dialog"
          aria-label={ariaLabel ?? placeholder}
        >
          <Calendar
            value={currentValue}
            visibleMonth={visibleMonth}
            weekStartsOn={weekStartsOn}
            locale={locale}
            disabled={disabled}
            size={resolvedSize}
            density={resolvedDensity}
            ariaLabel={ariaLabel ?? placeholder}
            onValueChange={(nextValue: string | DateRangeValue) => {
              if (typeof nextValue === "string") commitValue(nextValue);
            }}
            onMonthChange={setVisibleMonth}
          />
        </AnchoredSurface>
      ) : null}
    </div>
  );
}
