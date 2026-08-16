import { useEffect, useId, useRef, useState, type ChangeEvent, type FormEvent, type KeyboardEvent } from "react";
import {
  addDays,
  addMonths,
  addMonthsPreservingDay,
  buildCalendarWeeks,
  compareIsoDate,
  dayDeltaForWeekBoundary,
  formatDateLabel,
  formatIsoDate,
  formatMonthLabel,
  getWeekdayLabels,
  isIsoDateWithinRange,
  monthAnchorIso,
  normalizeDateRange,
  parseIsoDate,
  todayIsoDate,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/calendar.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  CalendarWeekStart,
  ControlDensity,
  ControlSize,
  DateRangeValue,
  SemanticControlSizeRole,
} from "./types";

export interface CalendarProps {
  mode?: "single" | "range";
  value?: string | DateRangeValue | null;
  defaultValue?: string | DateRangeValue | null;
  visibleMonth?: string | null;
  weekStartsOn?: CalendarWeekStart;
  locale?: string;
  disabled?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: (value: string | DateRangeValue) => void;
  onMonthChange?: (month: string) => void;
}

const EMPTY_RANGE: DateRangeValue = { start: null, end: null };

export function Calendar({
  mode = "single",
  value,
  defaultValue = null,
  visibleMonth,
  weekStartsOn = "monday",
  locale = "en-US",
  disabled = false,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
  onMonthChange,
}: CalendarProps) {
  const gridId = useId();
  const uiPresentation = useUiPresentation();

  const [uncontrolledSingleValue, setUncontrolledSingleValue] = useState<string | null>(() =>
    mode === "single" && typeof defaultValue === "string" ? defaultValue : null,
  );
  const [uncontrolledRangeValue, setUncontrolledRangeValue] = useState<DateRangeValue>(() =>
    mode === "range" && defaultValue !== null && typeof defaultValue === "object"
      ? normalizeDateRange(defaultValue as DateRangeValue)
      : EMPTY_RANGE,
  );
  const [uncontrolledMonth, setUncontrolledMonth] = useState(() =>
    monthAnchorIso(
      visibleMonth ??
        (mode === "range"
          ? ((typeof defaultValue === "object" && defaultValue !== null ? (defaultValue as DateRangeValue).start : null) ??
            todayIsoDate())
          : ((typeof defaultValue === "string" ? defaultValue : null) ?? todayIsoDate())),
    ),
  );
  const [focusIso, setFocusIso] = useState(() =>
    mode === "range"
      ? ((typeof defaultValue === "object" && defaultValue !== null ? (defaultValue as DateRangeValue).start : null) ??
        todayIsoDate())
      : ((typeof defaultValue === "string" ? defaultValue : null) ?? todayIsoDate()),
  );
  const [editingMonth, setEditingMonth] = useState(false);
  const [editingYear, setEditingYear] = useState(false);
  const [monthDraft, setMonthDraft] = useState("");
  const [yearDraft, setYearDraft] = useState("");
  const dayElements = useRef<Record<string, HTMLButtonElement | undefined>>({});
  const monthSelectRef = useRef<HTMLSelectElement | null>(null);
  const yearInputRef = useRef<HTMLInputElement | null>(null);
  const pendingDayFocus = useRef<string | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasControlledValue = value !== undefined;
  const hasControlledVisibleMonth = visibleMonth !== undefined;
  const currentSingleValue =
    mode === "single" ? (hasControlledValue ? (typeof value === "string" ? value : null) : uncontrolledSingleValue) : null;
  const currentRangeValue =
    mode !== "range"
      ? EMPTY_RANGE
      : normalizeDateRange(
          hasControlledValue
            ? value !== null && typeof value === "object"
              ? (value as DateRangeValue)
              : EMPTY_RANGE
            : uncontrolledRangeValue,
        );
  const currentMonth = monthAnchorIso(hasControlledVisibleMonth ? (visibleMonth ?? uncontrolledMonth) : uncontrolledMonth);
  const currentMonthDate = parseIsoDate(currentMonth) ?? parseIsoDate(todayIsoDate())!;
  const weeks = buildCalendarWeeks(currentMonth, weekStartsOn);
  const weekdayLabels = getWeekdayLabels(weekStartsOn, locale);
  const monthLabel = formatMonthLabel(currentMonth, locale);
  const monthName = new Intl.DateTimeFormat(locale, { month: "long", timeZone: "UTC" }).format(currentMonthDate);
  const yearLabel = String(currentMonthDate.getUTCFullYear());
  const monthOptions = Array.from({ length: 12 }, (_, monthIndex) => ({
    value: String(monthIndex),
    label: new Intl.DateTimeFormat(locale, { month: "long", timeZone: "UTC" }).format(new Date(Date.UTC(2000, monthIndex, 1))),
  }));

  // focus tracking follows the committed value (Svelte parity)
  useEffect(() => {
    if (mode === "single" && currentSingleValue) {
      setFocusIso(currentSingleValue);
      return;
    }
    if (mode === "range") {
      if (currentRangeValue.end) setFocusIso(currentRangeValue.end);
      else if (currentRangeValue.start) setFocusIso(currentRangeValue.start);
    }
  }, [mode, currentSingleValue, currentRangeValue.start, currentRangeValue.end]);

  useEffect(() => {
    if (editingMonth) monthSelectRef.current?.focus();
  }, [editingMonth]);

  useEffect(() => {
    if (editingYear) {
      yearInputRef.current?.focus();
      yearInputRef.current?.select();
    }
  }, [editingYear]);

  // focus a day cell after a month change re-render
  useEffect(() => {
    if (pendingDayFocus.current) {
      dayElements.current[pendingDayFocus.current]?.focus();
      pendingDayFocus.current = null;
    }
  });

  const isRangeStart = (iso: string) => mode === "range" && currentRangeValue.start === iso;
  const isRangeEnd = (iso: string) => mode === "range" && currentRangeValue.end === iso;
  const isInRange = (iso: string) => mode === "range" && isIsoDateWithinRange(iso, currentRangeValue);
  const isSelected = (iso: string) =>
    mode === "single" ? currentSingleValue === iso : isRangeStart(iso) || isRangeEnd(iso);

  function setMonth(nextMonth: string): void {
    if (!hasControlledVisibleMonth) setUncontrolledMonth(nextMonth);
    onMonthChange?.(nextMonth);
  }

  function commitSingleValue(nextValue: string): void {
    if (!hasControlledValue) setUncontrolledSingleValue(nextValue);
    setFocusIso(nextValue);
    onValueChange?.(nextValue);
  }

  function commitRange(nextValue: DateRangeValue): void {
    const normalized = normalizeDateRange(nextValue);
    if (!hasControlledValue) setUncontrolledRangeValue(normalized);
    setFocusIso(normalized.end ?? normalized.start ?? focusIso);
    onValueChange?.(normalized);
  }

  function selectDate(iso: string): void {
    if (disabled) return;
    if (mode === "single") {
      commitSingleValue(iso);
      return;
    }
    if (!currentRangeValue.start || currentRangeValue.end) {
      commitRange({ start: iso, end: null });
      return;
    }
    if (compareIsoDate(iso, currentRangeValue.start) < 0) {
      commitRange({ start: iso, end: currentRangeValue.start });
      return;
    }
    commitRange({ start: currentRangeValue.start, end: iso });
  }

  function focusDate(iso: string): void {
    setFocusIso(iso);
    const nextMonth = monthAnchorIso(iso);
    if (nextMonth !== currentMonth) {
      pendingDayFocus.current = iso;
      setMonth(nextMonth);
      return;
    }
    dayElements.current[iso]?.focus();
  }

  function handleDayKeydown(event: KeyboardEvent<HTMLButtonElement>, iso: string): void {
    const date = parseIsoDate(iso);
    if (!date) return;

    let nextDate = date;
    if (event.key === "ArrowRight") {
      event.preventDefault();
      nextDate = addDays(date, 1);
    } else if (event.key === "ArrowLeft") {
      event.preventDefault();
      nextDate = addDays(date, -1);
    } else if (event.key === "ArrowDown") {
      event.preventDefault();
      nextDate = addDays(date, 7);
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      nextDate = addDays(date, -7);
    } else if (event.key === "Home") {
      event.preventDefault();
      nextDate = addDays(date, dayDeltaForWeekBoundary(iso, weekStartsOn, "start"));
    } else if (event.key === "End") {
      event.preventDefault();
      nextDate = addDays(date, dayDeltaForWeekBoundary(iso, weekStartsOn, "end"));
    } else if (event.key === "PageDown") {
      event.preventDefault();
      nextDate = addMonthsPreservingDay(date, 1);
    } else if (event.key === "PageUp") {
      event.preventDefault();
      nextDate = addMonthsPreservingDay(date, -1);
    } else if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      selectDate(iso);
      return;
    } else {
      return;
    }

    focusDate(formatIsoDate(nextDate) ?? todayIsoDate());
  }

  function commitMonthDraft(draft: string): void {
    const parsedMonth = Number(draft);
    if (!Number.isInteger(parsedMonth) || parsedMonth < 0 || parsedMonth > 11) {
      setEditingMonth(false);
      return;
    }
    const nextDate = new Date(Date.UTC(currentMonthDate.getUTCFullYear(), parsedMonth, 1));
    setMonth(monthAnchorIso(formatIsoDate(nextDate) ?? currentMonth));
    setEditingMonth(false);
  }

  function commitYearDraft(): void {
    const trimmed = yearDraft.trim();
    const parsedYear = Number(trimmed);
    if (!Number.isInteger(parsedYear) || trimmed.length !== 4) {
      setEditingYear(false);
      return;
    }
    const nextDate = new Date(Date.UTC(parsedYear, currentMonthDate.getUTCMonth(), 1));
    setMonth(monthAnchorIso(formatIsoDate(nextDate) ?? currentMonth));
    setEditingYear(false);
  }

  function handleYearDraftBeforeInput(event: FormEvent<HTMLInputElement>): void {
    const native = event.nativeEvent as InputEvent;
    if (
      native.data &&
      !/^\d+$/.test(native.data) &&
      native.inputType !== "deleteContentBackward" &&
      native.inputType !== "deleteContentForward"
    ) {
      event.preventDefault();
      return;
    }
    if (native.data && /^\d+$/.test(native.data) && yearDraft.length >= 4 && native.inputType?.startsWith("insert")) {
      event.preventDefault();
    }
  }

  return (
    <div
      className="poodle-calendar"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-mode={mode}
      aria-label={ariaLabel ?? undefined}
    >
      <div className="poodle-calendar__header">
        <button
          type="button"
          className="poodle-calendar__nav"
          disabled={disabled}
          aria-label="Previous month"
          onClick={() => setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, -1))))}
        >
          <span aria-hidden="true">‹</span>
        </button>

        <div className="poodle-calendar__month" aria-live="polite">
          {editingMonth ? (
            <select
              ref={monthSelectRef}
              className="poodle-calendar__month-select"
              value={monthDraft}
              aria-label="Select month"
              onChange={(event: ChangeEvent<HTMLSelectElement>) => {
                setMonthDraft(event.currentTarget.value);
                commitMonthDraft(event.currentTarget.value);
              }}
              onBlur={() => setEditingMonth(false)}
              onKeyDown={(event) => {
                if (event.key === "Escape") {
                  event.preventDefault();
                  setEditingMonth(false);
                }
              }}
            >
              {monthOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          ) : (
            <button
              type="button"
              className="poodle-calendar__month-button"
              disabled={disabled}
              aria-label={`Edit month, currently ${monthName}`}
              onDoubleClick={() => {
                if (disabled) return;
                setEditingYear(false);
                setMonthDraft(String(currentMonthDate.getUTCMonth()));
                setEditingMonth(true);
              }}
            >
              <span className="poodle-calendar__month-name">{monthName}</span>
            </button>
          )}
          {editingYear ? (
            <input
              ref={yearInputRef}
              className="poodle-calendar__year-input"
              type="number"
              min={1}
              max={9999}
              step={1}
              inputMode="numeric"
              aria-label="Edit year"
              value={yearDraft}
              onBeforeInput={handleYearDraftBeforeInput}
              onChange={(event) => setYearDraft(event.currentTarget.value)}
              onBlur={commitYearDraft}
              onKeyDown={(event) => {
                if (["e", "E", "+", "-", "."].includes(event.key)) {
                  event.preventDefault();
                  return;
                }
                if (event.key === "Enter") {
                  event.preventDefault();
                  commitYearDraft();
                } else if (event.key === "Escape") {
                  event.preventDefault();
                  setEditingYear(false);
                }
              }}
            />
          ) : (
            <button
              type="button"
              className="poodle-calendar__year-button"
              disabled={disabled}
              aria-label={`Edit year, currently ${yearLabel}`}
              onDoubleClick={() => {
                if (disabled) return;
                setEditingMonth(false);
                setYearDraft(yearLabel);
                setEditingYear(true);
              }}
            >
              {yearLabel}
            </button>
          )}
        </div>

        <button
          type="button"
          className="poodle-calendar__nav"
          disabled={disabled}
          aria-label="Next month"
          onClick={() => setMonth(monthAnchorIso(formatIsoDate(addMonths(parseIsoDate(currentMonth)!, 1))))}
        >
          <span aria-hidden="true">›</span>
        </button>
      </div>

      <div className="poodle-calendar__weekdays" aria-hidden="true">
        {weekdayLabels.map((label, i) => (
          <span key={i} className="poodle-calendar__weekday">
            {label}
          </span>
        ))}
      </div>

      <div id={gridId} className="poodle-calendar__grid" role="grid" aria-label={ariaLabel ?? monthLabel}>
        {weeks.map((week, wi) => (
          <div key={wi} className="poodle-calendar__week" role="row">
            {week.map((day) => (
              <div key={day.iso} className="poodle-calendar__cell" role="gridcell" aria-selected={isSelected(day.iso)}>
                <button
                  ref={(node) => {
                    dayElements.current[day.iso] = node ?? undefined;
                  }}
                  type="button"
                  className="poodle-calendar__day"
                  data-current-month={day.inMonth}
                  data-selected={mode === "single" && currentSingleValue === day.iso}
                  data-today={day.isToday}
                  data-range-start={isRangeStart(day.iso)}
                  data-range-end={isRangeEnd(day.iso)}
                  data-in-range={isInRange(day.iso)}
                  disabled={disabled}
                  aria-label={formatDateLabel(day.iso, locale)}
                  tabIndex={focusIso === day.iso ? 0 : -1}
                  onClick={() => selectDate(day.iso)}
                  onFocus={() => setFocusIso(day.iso)}
                  onKeyDown={(event) => handleDayKeydown(event, day.iso)}
                >
                  {day.label}
                </button>
              </div>
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}
