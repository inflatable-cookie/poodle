import { describe, expect, test } from "bun:test";

import { popoverParts, popoverTransition, type PopoverContext } from "../src/popover.ts";

function ctx(overrides: Partial<PopoverContext> = {}): PopoverContext {
  return { disabled: false, dismissOnOutsideInteract: true, initialFocus: "first-focusable", ...overrides };
}

describe("popoverTransition", () => {
  test("TOGGLE opens from closed with open-change and focus effects", () => {
    const result = popoverTransition("closed", ctx(), { type: "TOGGLE" });

    expect(result.state).toBe("open");
    expect(result.effects).toEqual([
      { type: "emitOpenChange", open: true },
      { type: "focusOnOpen", strategy: "first-focusable" },
    ]);
  });

  test("every close path restores trigger focus", () => {
    for (const event of [{ type: "TOGGLE" }, { type: "CLOSE" }, { type: "ESCAPE" }] as const) {
      const result = popoverTransition("open", ctx(), event);

      expect(result.state).toBe("closed");
      expect(result.effects).toEqual([
        { type: "emitOpenChange", open: false },
        { type: "restoreTriggerFocus" },
      ]);
    }
  });

  test("OUTSIDE_INTERACT respects dismissOnOutsideInteract guard", () => {
    expect(popoverTransition("open", ctx(), { type: "OUTSIDE_INTERACT" }).state).toBe("closed");
    expect(
      popoverTransition("open", ctx({ dismissOnOutsideInteract: false }), { type: "OUTSIDE_INTERACT" }).state,
    ).toBe("open");
  });

  test("disabled blocks every transition", () => {
    for (const event of [
      { type: "TOGGLE" },
      { type: "OPEN" },
      { type: "ESCAPE" },
      { type: "OUTSIDE_INTERACT" },
    ] as const) {
      const result = popoverTransition("closed", ctx({ disabled: true }), event);
      expect(result.state).toBe("closed");
      expect(result.effects).toEqual([]);
    }
  });

  test("OPEN/CLOSE are idempotent in their target state", () => {
    expect(popoverTransition("open", ctx(), { type: "OPEN" }).effects).toEqual([]);
    expect(popoverTransition("closed", ctx(), { type: "CLOSE" }).effects).toEqual([]);
  });

  test("ESCAPE in closed state is inert", () => {
    expect(popoverTransition("closed", ctx(), { type: "ESCAPE" }).effects).toEqual([]);
  });
});

describe("popoverParts", () => {
  const props = {
    surfaceId: "poodle-popover-1",
    ariaLabel: "Settings",
    block: false,
    placement: "bottom-start",
    surfaceWidth: "content" as const,
  };

  test("trigger reflects open state", () => {
    const closed = popoverParts("closed", ctx(), props).trigger;
    expect(closed["aria-expanded"]).toBe("false");
    expect(closed["aria-controls"]).toBeUndefined();
    expect(closed["tabindex"]).toBe(0);

    const open = popoverParts("open", ctx(), props).trigger;
    expect(open["aria-expanded"]).toBe("true");
    expect(open["aria-controls"]).toBe("poodle-popover-1");
  });

  test("disabled trigger is removed from tab order", () => {
    const trigger = popoverParts("closed", ctx({ disabled: true }), props).trigger;
    expect(trigger["tabindex"]).toBe(-1);
    expect(trigger["aria-disabled"]).toBe("true");
  });

  test("interactive child mode delegates trigger semantics", () => {
    const trigger = popoverParts("open", ctx(), { ...props, triggerIsInteractive: true }).trigger;
    expect(trigger["role"]).toBeUndefined();
    expect(trigger["tabindex"]).toBeUndefined();
    expect(trigger["aria-expanded"]).toBeUndefined();
    expect(trigger["aria-controls"]).toBeUndefined();
  });

  test("surface tabindex follows initialFocus", () => {
    expect(popoverParts("open", ctx({ initialFocus: "content" }), props).surface["tabindex"]).toBe(0);
    expect(popoverParts("open", ctx(), props).surface["tabindex"]).toBe(-1);
  });
});
