/**
 * Date and time machinery (Calendar, DatePicker, DateRangePicker,
 * DateTimePicker, DateTimeRangePicker, DateTimeZonePicker, TimeZoneSelect,
 * DurationInput helpers).
 * Contracts: docs/contracts/components/calendar.md and the picker cluster,
 * "Behavior Machine" sections.
 *
 * Pure ISO-date math, calendar-grid construction, range/date-time
 * normalization, comparison, and Intl-based labels. Promoted wholesale from
 * the Svelte implementation's date module; the Svelte module re-exports
 * from here. The value types live in core so the Rust mirror can share the
 * same shapes.
 */

export type CalendarWeekStart = "sunday" | "monday";

export interface TimeZoneOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface DateRangeValue {
  start: string | null;
  end: string | null;
}

export interface DateTimeValue {
  date: string | null;
  time: string | null;
}

export interface DateTimeRangeValue {
  start: DateTimeValue;
  end: DateTimeValue;
}

export interface ZonedDateTimeValue {
  date: string | null;
  time: string | null;
  timeZone: string | null;
}

const DAY_MS = 24 * 60 * 60 * 1000;
const ISO_DATE_PATTERN = /^\d{4}-\d{2}-\d{2}$/;

export interface CalendarDay {
  iso: string;
  label: string;
  inMonth: boolean;
  isToday: boolean;
}

export function parseIsoDate(value: string | null | undefined): Date | null {
  if (!value || !ISO_DATE_PATTERN.test(value)) {
    return null;
  }

  const [yearPart, monthPart, dayPart] = value.split("-");
  const year = Number(yearPart);
  const month = Number(monthPart);
  const day = Number(dayPart);
  const date = new Date(Date.UTC(year, month - 1, day));

  if (
    Number.isNaN(date.getTime()) ||
    date.getUTCFullYear() !== year ||
    date.getUTCMonth() !== month - 1 ||
    date.getUTCDate() !== day
  ) {
    return null;
  }

  return date;
}

export function formatIsoDate(date: Date | null | undefined): string | null {
  if (!date) {
    return null;
  }

  const year = date.getUTCFullYear();
  const month = String(date.getUTCMonth() + 1).padStart(2, "0");
  const day = String(date.getUTCDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

export function todayIsoDate(): string {
  const today = new Date();
  const localDate = new Date(Date.UTC(today.getFullYear(), today.getMonth(), today.getDate()));
  return formatIsoDate(localDate) ?? "1970-01-01";
}

export function addDays(date: Date, amount: number): Date {
  return new Date(Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), date.getUTCDate() + amount));
}

export function addMonths(date: Date, amount: number): Date {
  return new Date(Date.UTC(date.getUTCFullYear(), date.getUTCMonth() + amount, 1));
}

export function startOfMonth(date: Date): Date {
  return new Date(Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), 1));
}

export function monthAnchorIso(value: string | null | undefined): string {
  return formatIsoDate(startOfMonth(parseIsoDate(value) ?? parseIsoDate(todayIsoDate())!)) ?? todayIsoDate();
}

export function compareIsoDate(left: string | null | undefined, right: string | null | undefined): number {
  if (!left && !right) {
    return 0;
  }

  if (!left) {
    return -1;
  }

  if (!right) {
    return 1;
  }

  return left.localeCompare(right);
}

export function normalizeDateRange(range: DateRangeValue): DateRangeValue {
  if (range.start && range.end && compareIsoDate(range.start, range.end) > 0) {
    return { start: range.end, end: range.start };
  }

  return range;
}

export function isIsoDateWithinRange(iso: string, range: DateRangeValue): boolean {
  const normalized = normalizeDateRange(range);

  if (!normalized.start || !normalized.end) {
    return false;
  }

  return compareIsoDate(iso, normalized.start) >= 0 && compareIsoDate(iso, normalized.end) <= 0;
}

export function formatMonthLabel(value: string, locale = "en-US"): string {
  const date = parseIsoDate(value);

  if (!date) {
    return value;
  }

  return new Intl.DateTimeFormat(locale, {
    month: "long",
    year: "numeric",
    timeZone: "UTC",
  }).format(date);
}

export function formatDateLabel(value: string | null, locale = "en-US"): string {
  const date = parseIsoDate(value);

  if (!date) {
    return "";
  }

  return new Intl.DateTimeFormat(locale, {
    month: "short",
    day: "numeric",
    year: "numeric",
    timeZone: "UTC",
  }).format(date);
}

function parseDisplayDateValue(value: Date | string | number | null | undefined): Date | null {
  if (value == null) {
    return null;
  }

  const date = value instanceof Date ? value : new Date(value);

  if (Number.isNaN(date.getTime())) {
    return null;
  }

  return date;
}

export function formatDisplayDate(
  value: Date | string | number | null | undefined,
  locale = "en-US",
): string {
  const date = parseDisplayDateValue(value);

  if (!date) {
    return "";
  }

  return date.toLocaleDateString(locale);
}

export function formatDisplayDateTime(
  value: Date | string | number | null | undefined,
  locale = "en-US",
): string {
  const date = parseDisplayDateValue(value);

  if (!date) {
    return "";
  }

  return date.toLocaleString(locale);
}

export function isTimeValue(value: string | null | undefined): boolean {
  if (!value) {
    return false;
  }

  const match = /^(\d{2}):(\d{2})$/.exec(value);

  if (!match) {
    return false;
  }

  const hours = Number(match[1]);
  const minutes = Number(match[2]);
  return hours >= 0 && hours <= 23 && minutes >= 0 && minutes <= 59;
}

export function formatTimeLabel(value: string | null, locale = "en-US"): string {
  if (!isTimeValue(value)) {
    return "";
  }

  const [hoursPart, minutesPart] = value!.split(":");
  const date = new Date(Date.UTC(2026, 0, 1, Number(hoursPart), Number(minutesPart)));

  return new Intl.DateTimeFormat(locale, {
    hour: "numeric",
    minute: "2-digit",
    timeZone: "UTC",
  }).format(date);
}

export function normalizeDateTimeValue(value: {
  date: string | null;
  time: string | null;
}): {
  date: string | null;
  time: string | null;
} {
  return {
    date: parseIsoDate(value.date) ? value.date : null,
    time: isTimeValue(value.time) ? value.time : null,
  };
}

export function formatDateTimeLabel(
  value: { date: string | null; time: string | null },
  locale = "en-US"
): string {
  const dateLabel = formatDateLabel(value.date, locale);
  const timeLabel = formatTimeLabel(value.time, locale);

  if (dateLabel && timeLabel) {
    return `${dateLabel}, ${timeLabel}`;
  }

  return dateLabel || timeLabel;
}

export function compareDateTimeValue(left: DateTimeValue, right: DateTimeValue): number | null {
  if (!left.date || !right.date) {
    return null;
  }

  const dateComparison = compareIsoDate(left.date, right.date);

  if (dateComparison !== 0) {
    return dateComparison;
  }

  if (!left.time || !right.time) {
    return null;
  }

  return left.time.localeCompare(right.time);
}

export function normalizeDateTimeRangeValue(value: DateTimeRangeValue): DateTimeRangeValue {
  const normalized = {
    start: normalizeDateTimeValue(value.start),
    end: normalizeDateTimeValue(value.end),
  };
  const comparison = compareDateTimeValue(normalized.start, normalized.end);

  if (comparison !== null && comparison > 0) {
    return {
      start: normalized.end,
      end: normalized.start,
    };
  }

  if (
    normalized.start.date &&
    normalized.end.date &&
    compareIsoDate(normalized.start.date, normalized.end.date) > 0
  ) {
    return {
      start: normalized.end,
      end: normalized.start,
    };
  }

  return normalized;
}

export function formatDateTimeRangeLabel(
  value: DateTimeRangeValue,
  locale = "en-US"
): string {
  const startLabel = formatDateTimeLabel(value.start, locale);
  const endLabel = formatDateTimeLabel(value.end, locale);

  if (startLabel && endLabel) {
    return `${startLabel} – ${endLabel}`;
  }

  if (startLabel) {
    return `${startLabel} – End`;
  }

  if (endLabel) {
    return `Start – ${endLabel}`;
  }

  return "";
}

export function isValidTimeZone(value: string | null | undefined): boolean {
  if (!value) {
    return false;
  }

  try {
    new Intl.DateTimeFormat("en-US", { timeZone: value });
    return true;
  } catch {
    return false;
  }
}

export function formatTimeZoneLabel(value: string | null): string {
  if (!value) {
    return "";
  }

  return value.replaceAll("_", " ");
}

export function defaultTimeZoneOptions(): TimeZoneOption[] {
  const intlWithSupportedValues = Intl as typeof Intl & {
    supportedValuesOf?: (key: string) => string[];
  };
  const values =
    typeof intlWithSupportedValues.supportedValuesOf === "function"
      ? intlWithSupportedValues.supportedValuesOf("timeZone")
      : [
          "UTC",
          "America/New_York",
          "America/Chicago",
          "America/Denver",
          "America/Los_Angeles",
          "Europe/London",
          "Europe/Paris",
          "Asia/Tokyo",
          "Australia/Sydney",
        ];

  return values.map((value) => ({
    value,
    label: formatTimeZoneLabel(value),
  }));
}

export function normalizeZonedDateTimeValue(value: ZonedDateTimeValue): ZonedDateTimeValue {
  return {
    date: parseIsoDate(value.date) ? value.date : null,
    time: isTimeValue(value.time) ? value.time : null,
    timeZone: isValidTimeZone(value.timeZone) ? value.timeZone : null,
  };
}

export function formatZonedDateTimeLabel(
  value: ZonedDateTimeValue,
  locale = "en-US"
): string {
  const dateTimeLabel = formatDateTimeLabel(value, locale);
  const timeZoneLabel = formatTimeZoneLabel(value.timeZone);

  if (dateTimeLabel && timeZoneLabel) {
    return `${dateTimeLabel} (${timeZoneLabel})`;
  }

  return dateTimeLabel || timeZoneLabel;
}

function weekdayOffset(day: number, weekStartsOn: CalendarWeekStart): number {
  return weekStartsOn === "monday" ? (day + 6) % 7 : day;
}

export function startOfWeek(date: Date, weekStartsOn: CalendarWeekStart): Date {
  return addDays(date, -weekdayOffset(date.getUTCDay(), weekStartsOn));
}

/**
 * `today` overrides what the grid treats as the current date.
 *
 * Reading the clock inside a pure builder makes every consumer's output change
 * at midnight — which is invisible until something compares renders, and then
 * shows up as a pixel baseline that expires overnight. Callers that want the
 * real date simply omit it.
 */
export function buildCalendarWeeks(
  visibleMonth: string,
  weekStartsOn: CalendarWeekStart,
  today: string = todayIsoDate()
): CalendarDay[][] {
  const monthDate = startOfMonth(parseIsoDate(visibleMonth) ?? parseIsoDate(today)!);
  const firstVisibleDay = startOfWeek(monthDate, weekStartsOn);
  const weeks: CalendarDay[][] = [];

  for (let weekIndex = 0; weekIndex < 6; weekIndex += 1) {
    const week: CalendarDay[] = [];

    for (let dayIndex = 0; dayIndex < 7; dayIndex += 1) {
      const date = addDays(firstVisibleDay, weekIndex * 7 + dayIndex);
      const iso = formatIsoDate(date) ?? today;

      week.push({
        iso,
        label: String(date.getUTCDate()),
        inMonth: date.getUTCMonth() === monthDate.getUTCMonth(),
        isToday: iso === today,
      });
    }

    weeks.push(week);
  }

  return weeks;
}

export function getWeekdayLabels(
  weekStartsOn: CalendarWeekStart,
  locale = "en-US"
): string[] {
  const sundayAnchor = new Date(Date.UTC(2026, 0, 4));

  return Array.from({ length: 7 }, (_, index) => {
    const offset = weekStartsOn === "monday" ? index + 1 : index;
    const date = addDays(sundayAnchor, offset);

    return new Intl.DateTimeFormat(locale, {
      weekday: "short",
      timeZone: "UTC",
    }).format(date);
  });
}

export function dayDeltaForWeekBoundary(
  iso: string,
  weekStartsOn: CalendarWeekStart,
  edge: "start" | "end"
): number {
  const date = parseIsoDate(iso);

  if (!date) {
    return 0;
  }

  const offset = weekdayOffset(date.getUTCDay(), weekStartsOn);
  return edge === "start" ? -offset : 6 - offset;
}

export function daysBetween(start: string, end: string): number {
  const startDate = parseIsoDate(start);
  const endDate = parseIsoDate(end);

  if (!startDate || !endDate) {
    return 0;
  }

  return Math.round((endDate.getTime() - startDate.getTime()) / DAY_MS);
}
