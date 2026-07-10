import { describe, expect, test } from "bun:test";

import { modalTransition, type ModalContext } from "../src/modal";

function ctx(overrides: Partial<ModalContext> = {}): ModalContext {
  return { dismissOnEscape: true, dismissOnBackdrop: true, ...overrides };
}

describe("modalTransition", () => {
  test("OPEN emits open-change, focus entry, and scroll lock", () => {
    const result = modalTransition("closed", ctx(), { type: "OPEN" });

    expect(result.state).toBe("open");
    expect(result.effects).toEqual([
      { type: "emitOpenChange", open: true },
      { type: "saveFocusAndEnter" },
      { type: "lockBodyScroll" },
    ]);
  });

  test("user close paths emit request-close before open-change, then unlock and restore", () => {
    for (const event of [{ type: "REQUEST_CLOSE" }, { type: "ESCAPE" }, { type: "BACKDROP_CLICK" }] as const) {
      const result = modalTransition("open", ctx(), event);

      expect(result.state).toBe("closed");
      expect(result.effects).toEqual([
        { type: "emitRequestClose" },
        { type: "emitOpenChange", open: false },
        { type: "unlockBodyScroll" },
        { type: "restoreFocus" },
      ]);
    }
  });

  test("programmatic CLOSE skips request-close", () => {
    const result = modalTransition("open", ctx(), { type: "CLOSE" });

    expect(result.effects[0]).toEqual({ type: "emitOpenChange", open: false });
    expect(result.effects.some((effect) => effect.type === "emitRequestClose")).toBe(false);
  });

  test("dismiss guards hold", () => {
    expect(modalTransition("open", ctx({ dismissOnEscape: false }), { type: "ESCAPE" }).state).toBe("open");
    expect(modalTransition("open", ctx({ dismissOnBackdrop: false }), { type: "BACKDROP_CLICK" }).state).toBe("open");
  });

  test("events in the wrong state are inert", () => {
    expect(modalTransition("open", ctx(), { type: "OPEN" }).effects).toEqual([]);
    expect(modalTransition("closed", ctx(), { type: "ESCAPE" }).effects).toEqual([]);
    expect(modalTransition("closed", ctx(), { type: "CLOSE" }).effects).toEqual([]);
  });
});
