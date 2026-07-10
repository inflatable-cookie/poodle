import { describe, expect, test } from "bun:test";

import { checkboxParts, checkboxState, checkboxTransition, type CheckboxContext } from "../src/checkbox";

function ctx(overrides: Partial<CheckboxContext> = {}): CheckboxContext {
  return { checked: false, mixed: false, disabled: false, readOnly: false, ...overrides };
}

describe("checkboxTransition", () => {
  test("TOGGLE sets checked and emits callback effect", () => {
    const result = checkboxTransition(ctx(), { type: "TOGGLE", nextChecked: true });

    expect(result.context.checked).toBe(true);
    expect(result.effects).toEqual([{ type: "emitCheckedChange", checked: true }]);
  });

  test("TOGGLE while disabled is inert", () => {
    const result = checkboxTransition(ctx({ disabled: true }), { type: "TOGGLE", nextChecked: true });

    expect(result.context.checked).toBe(false);
    expect(result.effects).toEqual([]);
  });

  test("TOGGLE while readOnly reverts without callback", () => {
    const result = checkboxTransition(ctx({ readOnly: true, checked: true }), {
      type: "TOGGLE",
      nextChecked: false,
    });

    expect(result.context.checked).toBe(true);
    expect(result.effects).toEqual([{ type: "revertNativeChecked" }]);
  });

  test("mixed resolves to checked on first toggle", () => {
    const result = checkboxTransition(ctx({ mixed: true }), { type: "TOGGLE", nextChecked: false });

    expect(result.context.checked).toBe(true);
    expect(result.effects).toEqual([{ type: "emitCheckedChange", checked: true }]);
  });

  test("SET_CHECKED updates without callback", () => {
    const result = checkboxTransition(ctx(), { type: "SET_CHECKED", checked: true });

    expect(result.context.checked).toBe(true);
    expect(result.effects).toEqual([]);
  });
});

describe("checkboxState / checkboxParts", () => {
  test("data-state reflects mixed > checked > unchecked", () => {
    expect(checkboxState(ctx({ mixed: true, checked: true }))).toBe("mixed");
    expect(checkboxState(ctx({ checked: true }))).toBe("checked");
    expect(checkboxState(ctx())).toBe("unchecked");
  });

  test("aria-label only applies without a visible label", () => {
    const props = { ariaLabel: "Accept", describedBy: null, hasVisibleLabel: false };
    expect(checkboxParts(ctx(), props).control["aria-label"]).toBe("Accept");
    expect(checkboxParts(ctx(), { ...props, hasVisibleLabel: true }).control["aria-label"]).toBeUndefined();
  });

  test("aria-readonly set only when readOnly", () => {
    const props = { hasVisibleLabel: true };
    expect(checkboxParts(ctx({ readOnly: true }), props).control["aria-readonly"]).toBe("true");
    expect(checkboxParts(ctx(), props).control["aria-readonly"]).toBeUndefined();
  });
});
