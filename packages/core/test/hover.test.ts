import { describe, expect, test } from "bun:test";

import { hoverTransition, type HoverContext } from "../src/hover";

const tooltip: HoverContext = { openDelayMs: 300, closeDelayMs: 0 };
const hoverCard: HoverContext = { openDelayMs: 250, closeDelayMs: 200 };

describe("hoverTransition", () => {
  test("enter schedules open; timer fire opens", () => {
    const opening = hoverTransition("closed", tooltip, { type: "ENTER" });
    expect(opening.state).toBe("opening");
    expect(opening.effects).toEqual([{ type: "clearTimer" }, { type: "startTimer", ms: 300 }]);

    const open = hoverTransition("opening", tooltip, { type: "TIMER_FIRE" });
    expect(open.state).toBe("open");
    expect(open.effects).toEqual([{ type: "emitOpenChange", open: true }]);
  });

  test("zero close delay closes immediately, even from opening", () => {
    const fromOpen = hoverTransition("open", tooltip, { type: "LEAVE" });
    expect(fromOpen.state).toBe("closed");
    expect(fromOpen.effects).toEqual([{ type: "clearTimer" }, { type: "emitOpenChange", open: false }]);

    const fromOpening = hoverTransition("opening", tooltip, { type: "LEAVE" });
    expect(fromOpening.state).toBe("closed");
    expect(fromOpening.effects).toContainEqual({ type: "emitOpenChange", open: false });
  });

  test("close delay schedules close; re-enter cancels it", () => {
    const closing = hoverTransition("open", hoverCard, { type: "LEAVE" });
    expect(closing.state).toBe("closing");
    expect(closing.effects).toEqual([{ type: "clearTimer" }, { type: "startTimer", ms: 200 }]);

    const reentered = hoverTransition("closing", hoverCard, { type: "ENTER" });
    expect(reentered.state).toBe("open");
    expect(reentered.effects).toEqual([{ type: "clearTimer" }]);

    const closed = hoverTransition("closing", hoverCard, { type: "TIMER_FIRE" });
    expect(closed.state).toBe("closed");
    expect(closed.effects).toEqual([{ type: "emitOpenChange", open: false }]);
  });

  test("dismiss closes from any non-closed state; stale timer fire is inert", () => {
    expect(hoverTransition("opening", hoverCard, { type: "DISMISS" }).state).toBe("closed");
    expect(hoverTransition("open", hoverCard, { type: "DISMISS" }).state).toBe("closed");
    expect(hoverTransition("closed", hoverCard, { type: "DISMISS" }).effects).toEqual([]);
    expect(hoverTransition("closed", hoverCard, { type: "TIMER_FIRE" }).effects).toEqual([]);
    expect(hoverTransition("open", hoverCard, { type: "TIMER_FIRE" }).effects).toEqual([]);
  });

  test("SET_OPEN forces state without callbacks", () => {
    const forced = hoverTransition("closed", hoverCard, { type: "SET_OPEN", open: true });
    expect(forced.state).toBe("open");
    expect(forced.effects).toEqual([{ type: "clearTimer" }]);
  });
});
