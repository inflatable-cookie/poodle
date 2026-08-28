import { describe, expect, test } from "bun:test";

import {
  formatTime,
  parseTime,
  stepTimeSeconds,
  timeConstraintValid,
  timeInBounds,
  timeInputContext,
  timeInputInvalid,
  timeInputTransition,
  timeSecondsVisible,
  timeStepAligned,
  timeToSeconds,
} from "../src/time-input.ts";

describe("parse and format", () => {
  test("accepts zero-padded canonical forms", () => {
    expect(parseTime("14:30")).toEqual({ hour: 14, minute: 30, second: 0 });
    expect(parseTime("14:30:05")).toEqual({ hour: 14, minute: 30, second: 5 });
    expect(parseTime("00:00")).toEqual({ hour: 0, minute: 0, second: 0 });
    expect(parseTime("23:59:59")).toEqual({ hour: 23, minute: 59, second: 59 });
  });

  test("rejects malformed, unpadded, impossible, and fractional values", () => {
    expect(parseTime("9:30")).toBeNull();
    expect(parseTime("14:3")).toBeNull();
    expect(parseTime("24:00")).toBeNull();
    expect(parseTime("14:60")).toBeNull();
    expect(parseTime("14:30:60")).toBeNull();
    expect(parseTime("14:30:00.5")).toBeNull();
    expect(parseTime("14:30:0")).toBeNull();
    expect(parseTime("1430")).toBeNull();
    expect(parseTime("")).toBeNull();
  });

  test("formats with and without seconds", () => {
    const parts = { hour: 9, minute: 5, second: 7 };
    expect(formatTime(parts, false)).toBe("09:05");
    expect(formatTime(parts, true)).toBe("09:05:07");
  });
});

describe("seconds visibility", () => {
  test("appears from step, value, default, min, and max", () => {
    expect(timeSecondsVisible({ step: 60 })).toBe(false);
    expect(timeSecondsVisible({ step: 15 })).toBe(true);
    expect(timeSecondsVisible({ step: 60, committed: "09:30:15" })).toBe(true);
    expect(timeSecondsVisible({ step: 60, defaultValue: "09:30:15" })).toBe(true);
    expect(timeSecondsVisible({ step: 60, min: "08:00:00" })).toBe(true);
    expect(timeSecondsVisible({ step: 60, max: "18:00:00" })).toBe(true);
  });
});

describe("bounds and step alignment", () => {
  test("linear inclusive range and overnight excluded gap", () => {
    const nine = { hour: 9, minute: 0, second: 0 };
    expect(timeInBounds(nine, "08:00", "18:00")).toBe(true);
    expect(timeInBounds({ hour: 7, minute: 0, second: 0 }, "08:00", "18:00")).toBe(false);
    expect(timeInBounds({ hour: 23, minute: 30, second: 0 }, "22:00", "06:00")).toBe(true);
    expect(timeInBounds({ hour: 0, minute: 0, second: 0 }, "22:00", "06:00")).toBe(true);
    expect(timeInBounds({ hour: 12, minute: 0, second: 0 }, "22:00", "06:00")).toBe(false);
  });

  test("anchors the grid at midnight or min, and rejects off-grid direct entry", () => {
    expect(timeStepAligned({ hour: 9, minute: 5, second: 0 }, null, 300)).toBe(true);
    expect(timeStepAligned({ hour: 9, minute: 7, second: 0 }, null, 300)).toBe(false);
    expect(timeStepAligned({ hour: 8, minute: 0, second: 0 }, "08:00", 300)).toBe(true);
    expect(timeStepAligned({ hour: 8, minute: 5, second: 0 }, "08:00", 300)).toBe(true);
    expect(timeStepAligned({ hour: 8, minute: 7, second: 0 }, "08:00", 300)).toBe(false);
    expect(timeStepAligned({ hour: 0, minute: 0, second: 0 }, "22:00", 1800)).toBe(true);
    expect(timeConstraintValid("09:07", null, null, 300)).toBe(false);
  });
});

describe("stepping", () => {
  test("unbounded wrap, linear clamp, overnight wrap, excluded-gap stop", () => {
    expect(stepTimeSeconds(timeToSeconds({ hour: 23, minute: 30, second: 0 }), 1, null, null, 1800)).toBe(0);
    expect(stepTimeSeconds(0, -1, null, null, 1800)).toBe(23 * 3600 + 30 * 60);
    expect(stepTimeSeconds(timeToSeconds({ hour: 18, minute: 0, second: 0 }), 1, "08:00", "18:00", 60)).toBe(
      18 * 3600,
    );
    expect(stepTimeSeconds(timeToSeconds({ hour: 8, minute: 0, second: 0 }), -1, "08:00", "18:00", 60)).toBe(8 * 3600);
    expect(stepTimeSeconds(timeToSeconds({ hour: 23, minute: 30, second: 0 }), 1, "22:00", "06:00", 1800)).toBe(0);
    expect(stepTimeSeconds(6 * 3600, 1, "22:00", "06:00", 1800)).toBe(6 * 3600);
    expect(stepTimeSeconds(22 * 3600, -1, "22:00", "06:00", 1800)).toBe(22 * 3600);
  });

  test("empty start lands on origin or the opposite bound", () => {
    expect(stepTimeSeconds(null, 1, null, null, 60)).toBe(0);
    expect(stepTimeSeconds(null, -1, null, null, 60)).toBe(23 * 3600 + 59 * 60);
    expect(stepTimeSeconds(null, 1, "08:00", "18:00", 60)).toBe(8 * 3600);
    expect(stepTimeSeconds(null, -1, "08:00", "18:00", 60)).toBe(18 * 3600);
    expect(stepTimeSeconds(null, 1, "22:00", "06:00", 1800)).toBe(22 * 3600);
    expect(stepTimeSeconds(null, -1, "22:00", "06:00", 1800)).toBe(6 * 3600);
  });
});

describe("draft transitions", () => {
  test("partial digits stay local; a complete valid pair commits", () => {
    const started = timeInputTransition(timeInputContext({ committed: "14:30" }), {
      type: "DIGIT",
      segment: "hour",
      digit: 1,
    });
    expect(started.effects).toEqual([]);
    expect(timeInputInvalid(started.context)).toBe(true);
    expect(started.context.draft).toEqual({ hour: "1", minute: "30", second: "00" });

    const committed = timeInputTransition(started.context, { type: "DIGIT", segment: "hour", digit: 5 });
    expect(committed.effects).toEqual([{ type: "emitValueChange", value: "15:30" }]);
    expect(committed.context.committed).toBe("15:30");
    expect(committed.context.draft).toBeNull();
  });

  test("out-of-range and off-step drafts do not emit", () => {
    const overnight = timeInputContext({ committed: "23:00", min: "22:00", max: "06:00", step: 1800 });
    const gap = timeInputTransition(overnight, { type: "COMMIT_TEXT", text: "12:00" });
    expect(gap.effects).toEqual([]);
    expect(timeInputInvalid(gap.context)).toBe(true);

    const offStep = timeInputTransition(timeInputContext({ committed: "09:00", step: 300 }), {
      type: "COMMIT_TEXT",
      text: "09:07",
    });
    expect(offStep.effects).toEqual([]);
    expect(timeInputInvalid(offStep.context)).toBe(true);
  });

  test("blur and Escape revert a draft without emitting", () => {
    const drafting = timeInputTransition(timeInputContext({ committed: "14:30" }), {
      type: "DIGIT",
      segment: "minute",
      digit: 1,
    }).context;

    for (const type of ["BLUR", "ESCAPE"] as const) {
      const reverted = timeInputTransition(drafting, { type });
      expect(reverted.effects).toEqual([]);
      expect(reverted.context.committed).toBe("14:30");
      expect(reverted.context.draft).toBeNull();
    }
  });

  test("one-segment clear drafts; whole-control clear emits null", () => {
    const one = timeInputTransition(timeInputContext({ committed: "14:30" }), {
      type: "CLEAR_SEGMENT",
      segment: "minute",
    });
    expect(one.effects).toEqual([]);
    expect(one.context.draft).toEqual({ hour: "14", minute: "", second: "00" });

    const all = timeInputTransition(timeInputContext({ committed: "14:30" }), { type: "CLEAR_ALL" });
    expect(all.effects).toEqual([{ type: "emitValueChange", value: null }]);
    expect(all.context.committed).toBeNull();
  });

  test("controlled replacement discards a draft", () => {
    const drafting = timeInputTransition(timeInputContext({ committed: "14:30" }), {
      type: "DIGIT",
      segment: "hour",
      digit: 2,
    }).context;
    const replaced = timeInputTransition(drafting, { type: "REPLACE", value: "08:00" });
    expect(replaced.effects).toEqual([]);
    expect(replaced.context.committed).toBe("08:00");
    expect(replaced.context.draft).toBeNull();
  });

  test("repeated blur, Escape, and clear stay inert; disabled events stay inert", () => {
    const empty = timeInputContext();
    expect(timeInputTransition(empty, { type: "BLUR" }).effects).toEqual([]);
    expect(timeInputTransition(empty, { type: "ESCAPE" }).effects).toEqual([]);
    expect(timeInputTransition(empty, { type: "CLEAR_ALL" }).effects).toEqual([]);

    const disabled = timeInputContext({ committed: "14:30", disabled: true });
    expect(timeInputTransition(disabled, { type: "DIGIT", segment: "hour", digit: 1 }).effects).toEqual([]);
    expect(timeInputTransition(disabled, { type: "STEP", direction: 1 }).effects).toEqual([]);
    expect(timeInputTransition(disabled, { type: "CLEAR_ALL" }).effects).toEqual([]);
    expect(timeInputTransition(disabled, { type: "COMMIT_TEXT", text: "15:00" }).effects).toEqual([]);
    expect(timeInputTransition(disabled, { type: "REPLACE", value: "08:00" }).context.committed).toBe("08:00");
  });
});
