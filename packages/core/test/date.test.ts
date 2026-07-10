import { describe, expect, test } from "bun:test";

import {
  addDays,
  addMonths,
  buildCalendarWeeks,
  compareDateTimeValue,
  compareIsoDate,
  dayDeltaForWeekBoundary,
  daysBetween,
  formatIsoDate,
  isIsoDateWithinRange,
  isTimeValue,
  isValidTimeZone,
  monthAnchorIso,
  normalizeDateRange,
  normalizeDateTimeRangeValue,
  parseIsoDate,
  startOfWeek,
} from "../src/date";

describe("iso parse/format/arithmetic", () => {
  test("round trip and invalid handling", () => {
    expect(formatIsoDate(parseIsoDate("2026-07-10"))).toBe("2026-07-10");
    expect(parseIsoDate("2026-13-40")).toBeNull();
    expect(parseIsoDate("not-a-date")).toBeNull();
    expect(parseIsoDate(null)).toBeNull();
  });

  test("addDays crosses boundaries; addMonths anchors to the 1st (month paging)", () => {
    expect(formatIsoDate(addDays(parseIsoDate("2026-01-31")!, 1))).toBe("2026-02-01");
    expect(formatIsoDate(addMonths(parseIsoDate("2026-12-15")!, 1))).toBe("2027-01-01");
  });

  test("compare and monthAnchor", () => {
    expect(compareIsoDate("2026-01-01", "2026-01-02")).toBeLessThan(0);
    expect(compareIsoDate("2026-01-02", "2026-01-02")).toBe(0);
    expect(monthAnchorIso("2026-07-10")).toBe("2026-07-01");
  });
});

describe("ranges", () => {
  test("normalizeDateRange orders endpoints", () => {
    expect(normalizeDateRange({ start: "2026-02-01", end: "2026-01-01" })).toEqual({
      start: "2026-01-01",
      end: "2026-02-01",
    });
  });

  test("isIsoDateWithinRange inclusive", () => {
    const range = { start: "2026-01-01", end: "2026-01-31" };
    expect(isIsoDateWithinRange("2026-01-01", range)).toBe(true);
    expect(isIsoDateWithinRange("2026-01-31", range)).toBe(true);
    expect(isIsoDateWithinRange("2026-02-01", range)).toBe(false);
  });

  test("daysBetween", () => {
    expect(daysBetween("2026-01-01", "2026-01-11")).toBe(10);
  });
});

describe("calendar grid", () => {
  test("weeks are full 7-day rows containing the anchor month", () => {
    const weeks = buildCalendarWeeks("2026-07-01", "monday");

    expect(weeks.length).toBeGreaterThanOrEqual(4);
    for (const week of weeks) {
      expect(week).toHaveLength(7);
    }

    const isos = weeks.flat().map((day) => day.iso);
    expect(isos).toContain("2026-07-01");
    expect(isos).toContain("2026-07-31");
  });

  test("startOfWeek respects week start; boundary deltas map Home/End", () => {
    const friday = parseIsoDate("2026-07-10")!; // Friday
    expect(formatIsoDate(startOfWeek(friday, "monday"))).toBe("2026-07-06");
    expect(formatIsoDate(startOfWeek(friday, "sunday"))).toBe("2026-07-05");
    expect(dayDeltaForWeekBoundary("2026-07-10", "monday", "start")).toBe(-4);
    expect(dayDeltaForWeekBoundary("2026-07-10", "monday", "end")).toBe(2);
  });
});

describe("date-time values", () => {
  test("compare and range normalization", () => {
    expect(
      compareDateTimeValue({ date: "2026-01-01", time: "09:00" }, { date: "2026-01-01", time: "10:00" }),
    ).toBeLessThan(0);

    const normalized = normalizeDateTimeRangeValue({
      start: { date: "2026-01-02", time: "09:00" },
      end: { date: "2026-01-01", time: "09:00" },
    });
    expect(normalized.start.date).toBe("2026-01-01");
  });

  test("time and timezone validation", () => {
    expect(isTimeValue("23:59")).toBe(true);
    expect(isTimeValue("24:00")).toBe(false);
    expect(isValidTimeZone("Europe/London")).toBe(true);
    expect(isValidTimeZone("Not/AZone")).toBe(false);
  });
});
