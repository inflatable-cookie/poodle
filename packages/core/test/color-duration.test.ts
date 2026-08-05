import { describe, expect, test } from "bun:test";

import { hexToRgb, hexToHsv, hsvToHex, isValidHex, normalizeHex, rgbToHex } from "../src/color.ts";
import { adjustDurationSegment, durationTotalSeconds, setDurationSegment } from "../src/duration.ts";

describe("color conversions", () => {
  test("hex validation and normalization", () => {
    expect(isValidHex("#ff0000")).toBe(true);
    expect(isValidHex("#f00")).toBe(true);
    expect(isValidHex("red")).toBe(false);
    expect(normalizeHex("#F00")).toBe("#ff0000");
  });

  test("hex <-> rgb round trip", () => {
    expect(hexToRgb("#3366cc")).toMatchObject({ r: 51, g: 102, b: 204 });
    expect(rgbToHex(51, 102, 204)).toBe("#3366cc");
  });

  test("hex <-> hsv round trip preserves the color", () => {
    const hsv = hexToHsv("#3366cc");
    expect(hsvToHex(hsv.h, hsv.s, hsv.v)).toBe("#3366cc");
  });
});

describe("duration segments", () => {
  const base = { hours: 1, minutes: 59, seconds: 59 };

  test("total seconds", () => {
    expect(durationTotalSeconds(base)).toBe(7199);
  });

  test("carry: seconds roll into minutes into hours", () => {
    expect(adjustDurationSegment(base, "seconds", 1, 99)).toEqual({ hours: 2, minutes: 0, seconds: 0 });
    expect(adjustDurationSegment({ hours: 0, minutes: 0, seconds: 0 }, "seconds", -1, 99)).toEqual({
      hours: 0,
      minutes: 59,
      seconds: 59,
    });
  });

  test("hours clamp to [0, maxHours]; carry at the bound is swallowed", () => {
    expect(adjustDurationSegment({ hours: 99, minutes: 59, seconds: 0 }, "minutes", 1, 99)).toEqual({
      hours: 99,
      minutes: 0,
      seconds: 0,
    });
    expect(adjustDurationSegment({ hours: 0, minutes: 30, seconds: 0 }, "hours", -1, 99).hours).toBe(0);
  });

  test("direct entry clamps into segment range", () => {
    expect(setDurationSegment(base, "minutes", 75, 99).minutes).toBe(59);
    expect(setDurationSegment(base, "hours", 200, 99).hours).toBe(99);
    expect(setDurationSegment(base, "seconds", -5, 99).seconds).toBe(0);
  });
});
