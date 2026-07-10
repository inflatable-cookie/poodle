import { describe, expect, test } from "bun:test";

import { menuTransition } from "../src/menu";

describe("menuTransition", () => {
  test("TOGGLE opens with focus-first-item intent", () => {
    const result = menuTransition("closed", {}, { type: "TOGGLE" });

    expect(result.state).toBe("open");
    expect(result.effects).toEqual([
      { type: "emitOpenChange", open: true },
      { type: "focusFirstItem" },
    ]);
  });

  test("close paths emit open-change only (no trigger-focus restore)", () => {
    for (const event of [{ type: "TOGGLE" }, { type: "CLOSE" }, { type: "ESCAPE" }, { type: "OUTSIDE_INTERACT" }] as const) {
      const result = menuTransition("open", {}, event);

      expect(result.state).toBe("closed");
      expect(result.effects).toEqual([{ type: "emitOpenChange", open: false }]);
    }
  });

  test("ACTION emits the action before closing", () => {
    const result = menuTransition("open", {}, { type: "ACTION", value: "rename" });

    expect(result.state).toBe("closed");
    expect(result.effects).toEqual([
      { type: "emitAction", value: "rename" },
      { type: "emitOpenChange", open: false },
    ]);
  });

  test("wrong-state events and disabled context are inert", () => {
    expect(menuTransition("closed", {}, { type: "ESCAPE" }).effects).toEqual([]);
    expect(menuTransition("closed", {}, { type: "ACTION", value: "x" }).effects).toEqual([]);
    expect(menuTransition("open", {}, { type: "OPEN" }).effects).toEqual([]);
    expect(menuTransition("closed", { disabled: true }, { type: "TOGGLE" }).effects).toEqual([]);
  });
});
