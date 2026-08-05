import { describe, expect, test } from "bun:test";

import { editLabelTransition, listReorderKeyIntent, type EditLabelContext } from "../src/edit.ts";
import {
  clampCodePosition,
  codeInsertReplacement,
  codeSelectionRange,
  codeSlotSelection,
  sanitizeCodeValue,
} from "../src/code-input.ts";
import { mergeTokens, splitTokenInput, tokenBackspaceRemoves } from "../src/token.ts";

function ctx(overrides: Partial<EditLabelContext> = {}): EditLabelContext {
  return { value: "Original", draft: "Original", disabled: false, canStartEdit: true, ...overrides };
}

describe("editLabelTransition", () => {
  test("start edit seeds draft and emits start + focus", () => {
    const result = editLabelTransition("view", ctx({ draft: "stale" }), { type: "START_EDIT" });

    expect(result.state).toBe("editing");
    expect(result.context.draft).toBe("Original");
    expect(result.effects).toEqual([{ type: "emitEditStart" }, { type: "focusInput" }]);
  });

  test("guards: disabled and programmatic activation block start", () => {
    expect(editLabelTransition("view", ctx({ disabled: true }), { type: "START_EDIT" }).state).toBe("view");
    expect(editLabelTransition("view", ctx({ canStartEdit: false }), { type: "START_EDIT" }).state).toBe("view");
  });

  test("commit trims the draft and reports the previous value", () => {
    const editing = ctx({ draft: "  New name  " });
    const result = editLabelTransition("editing", editing, { type: "COMMIT" });

    expect(result.state).toBe("view");
    expect(result.effects).toEqual([{ type: "emitCommit", value: "New name", previousValue: "Original" }]);
  });

  test("cancel restores the draft and emits cancel", () => {
    const result = editLabelTransition("editing", ctx({ draft: "abandoned" }), { type: "CANCEL" });

    expect(result.state).toBe("view");
    expect(result.context.draft).toBe("Original");
    expect(result.effects).toEqual([{ type: "emitCancel" }]);
  });
});

describe("listReorderKeyIntent", () => {
  test("space grabs and drops; escape cancels only during a grab", () => {
    expect(listReorderKeyIntent(" ", 2, null, 5)).toEqual({ type: "grab" });
    expect(listReorderKeyIntent("Enter", 2, 2, 5)).toEqual({ type: "drop" });
    expect(listReorderKeyIntent("Escape", 2, 2, 5)).toEqual({ type: "cancelGrab" });
    expect(listReorderKeyIntent("Escape", 2, null, 5)).toBeNull();
  });

  test("arrows move the grabbed item when active, else the focused one; boundaries report", () => {
    expect(listReorderKeyIntent("ArrowDown", 1, null, 5)).toEqual({ type: "move", from: 1, to: 2 });
    expect(listReorderKeyIntent("ArrowUp", 1, 3, 5)).toEqual({ type: "move", from: 3, to: 2 });
    expect(listReorderKeyIntent("ArrowUp", 0, null, 5)).toEqual({ type: "boundary" });
    expect(listReorderKeyIntent("ArrowDown", 4, null, 5)).toEqual({ type: "boundary" });
    expect(listReorderKeyIntent("a", 1, null, 5)).toBeNull();
  });
});

describe("code-input math", () => {
  test("sanitize strips non-digits when numbersOnly and caps length", () => {
    expect(sanitizeCodeValue("1a2b3c4d5e6f7", 6, true)).toBe("123456");
    expect(sanitizeCodeValue("abc123", 4, false)).toBe("abc1");
  });

  test("position clamps into the filled prefix", () => {
    expect(clampCodePosition(9, 3, 6)).toBe(3);
    expect(clampCodePosition(-1, 3, 6)).toBe(0);
    expect(clampCodePosition(5, 6, 6)).toBe(5);
  });

  test("selection ranges select the filled digit when asked", () => {
    expect(codeSelectionRange(2, 4, true)).toEqual({ start: 2, end: 3 });
    expect(codeSelectionRange(4, 4, true)).toEqual({ start: 4, end: 4 });
    expect(codeSlotSelection(1, 4)).toEqual({ start: 1, end: 2 });
    expect(codeSlotSelection(5, 3)).toEqual({ start: 3, end: 3 });
  });

  test("insert replacement overwrites and advances the caret", () => {
    expect(codeInsertReplacement("123456", "9", 2, 2, 6, true)).toEqual({ value: "129456", caret: 3 });
    expect(codeInsertReplacement("123", "78", 3, 3, 6, true)).toEqual({ value: "12378", caret: 5 });
    expect(codeInsertReplacement("123", "x", 1, 1, 6, true)).toBeNull();
  });
});

describe("token machinery", () => {
  test("merge dedupes when asked", () => {
    expect(mergeTokens(["a"], ["b", "a"], true)).toEqual(["a", "b"]);
    expect(mergeTokens(["a"], ["b", "a"], false)).toEqual(["a", "b", "a"]);
  });

  test("split commits completed parts and keeps the remainder", () => {
    expect(splitTokenInput("one,two,thr", /[,;]/, ",;")).toEqual({ committed: ["one", "two"], remainder: "thr" });
    expect(splitTokenInput("one,", /[,;]/, ",;")).toEqual({ committed: ["one", ""], remainder: "" });
    expect(splitTokenInput("plain", /[,;]/, ",;")).toBeNull();
    expect(splitTokenInput("plain", null, ",;")).toBeNull();
  });

  test("backspace removes the last chip only on empty input", () => {
    expect(tokenBackspaceRemoves("", 3)).toBe(true);
    expect(tokenBackspaceRemoves("x", 3)).toBe(false);
    expect(tokenBackspaceRemoves("", 0)).toBe(false);
  });
});
