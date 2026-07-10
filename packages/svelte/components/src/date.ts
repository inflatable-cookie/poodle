/**
 * Date and time utilities, now provided by the headless core; re-exported
 * here so the public `@poodle/svelte` surface and internal imports are
 * unchanged. The value types remain exported from ./types (which re-uses
 * the core shapes structurally).
 */
export {
  parseIsoDate,
  formatIsoDate,
  todayIsoDate,
  addDays,
  addMonths,
  startOfMonth,
  monthAnchorIso,
  compareIsoDate,
  normalizeDateRange,
  isIsoDateWithinRange,
  formatMonthLabel,
  formatDateLabel,
  formatDisplayDate,
  formatDisplayDateTime,
  isTimeValue,
  formatTimeLabel,
  normalizeDateTimeValue,
  formatDateTimeLabel,
  compareDateTimeValue,
  normalizeDateTimeRangeValue,
  formatDateTimeRangeLabel,
  isValidTimeZone,
  formatTimeZoneLabel,
  defaultTimeZoneOptions,
  normalizeZonedDateTimeValue,
  formatZonedDateTimeLabel,
  startOfWeek,
  buildCalendarWeeks,
  getWeekdayLabels,
  dayDeltaForWeekBoundary,
  daysBetween,
  type CalendarDay,
} from "@poodle/headless";
