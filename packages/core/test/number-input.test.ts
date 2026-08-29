import { describe, expect, test } from "bun:test";

import {
  classifyNumberDraft,
  formatNumberCommitted,
  numberDraftConstraintValid,
  numberInBounds,
  numberInputConfigValid,
  numberInputContext,
  numberInputInvalid,
  numberInputTransition,
  numberStepAligned,
  stepNumberValue,
} from "../src/number-input.ts";

describe("draft classification", () => {
  test("accepts complete finite base-10 drafts", () => {
    expect(classifyNumberDraft("0").kind).toBe("complete");
    expect(classifyNumberDraft("01.20").kind).toBe("complete");
    expect(classifyNumberDraft("-12.5").kind).toBe("complete");
    expect(classifyNumberDraft(".5").kind).toBe("complete");
  });

  test("keeps empty, incomplete, and malformed drafts exact", () => {
    expect(classifyNumberDraft("").kind).toBe("empty");
    expect(classifyNumberDraft("-").kind).toBe("incomplete");
    expect(classifyNumberDraft(".").kind).toBe("incomplete");
    expect(classifyNumberDraft("-.").kind).toBe("incomplete");
    expect(classifyNumberDraft("1.").kind).toBe("incomplete");
    expect(classifyNumberDraft("1e2").kind).toBe("malformed");
    expect(classifyNumberDraft("0x10").kind).toBe("malformed");
    expect(classifyNumberDraft("1 2").kind).toBe("malformed");
    expect(classifyNumberDraft("NaN").kind).toBe("malformed");
  });
});

describe("config, bounds, and step", () => {
  test("rejects invalid authored configuration", () => {
    expect(numberInputConfigValid({ step: 0 })).toBe(false);
    expect(numberInputConfigValid({ step: -1 })).toBe(false);
    expect(numberInputConfigValid({ precision: 1.5 })).toBe(false);
    expect(numberInputConfigValid({ precision: -1 })).toBe(false);
    expect(numberInputConfigValid({ min: 5, max: 1 })).toBe(false);
    expect(numberInputConfigValid({ min: Number.NaN })).toBe(false);
    expect(numberInputConfigValid({ step: null, precision: 2 })).toBe(true);
  });

  test("anchors step alignment at min or zero", () => {
    expect(numberStepAligned(1, null, 1)).toBe(true);
    expect(numberStepAligned(0.3, null, 0.1)).toBe(true);
    expect(numberStepAligned(0.3, null, 0.2)).toBe(false);
    expect(numberStepAligned(1.2, 1, 0.1)).toBe(true);
    expect(numberStepAligned(1.25, 1, 0.1)).toBe(false);
    expect(numberInBounds(5, 0, 10)).toBe(true);
    expect(numberInBounds(11, 0, 10)).toBe(false);
  });

  test("precision rejection does not silently round", () => {
    expect(numberDraftConstraintValid("1.234", null, null, 0.001, 2)).toBe(false);
    expect(numberDraftConstraintValid("1.23", null, null, 0.01, 2)).toBe(true);
  });
});

describe("stepping", () => {
  test("starts from empty at min or zero and stops before invalid results", () => {
    expect(stepNumberValue(null, 1, null, null, null, null)).toBe(0);
    expect(stepNumberValue(null, 1, 5, 10, 1, null)).toBe(5);
    expect(stepNumberValue(10, 1, 0, 10, 1, null)).toBeNull();
    expect(stepNumberValue(0, -1, 0, 10, 1, null)).toBeNull();
    expect(stepNumberValue(1.2, 1, null, null, 0.1, 1)).toBe(1.3);
  });
});

describe("transitions", () => {
  test("raw edit preserves incomplete drafts and emits no value", () => {
    const result = numberInputTransition(numberInputContext({ committed: 1 }), {
      type: "RAW_EDIT",
      text: "-",
    });
    expect(result.context.draft).toBe("-");
    expect(result.context.committed).toBe(1);
    expect(result.effects).toEqual([{ type: "emitDraftValueChange", draft: "-" }]);
    expect(numberInputInvalid(result.context)).toBe(true);
  });

  test("complete valid edit emits value; clear emits null", () => {
    const live = numberInputTransition(numberInputContext({ committed: 1 }), {
      type: "RAW_EDIT",
      text: "2",
    });
    expect(live.context.committed).toBe(2);
    expect(live.effects).toEqual([
      { type: "emitDraftValueChange", draft: "2" },
      { type: "emitValueChange", value: 2 },
    ]);

    const cleared = numberInputTransition(live.context, { type: "CLEAR" });
    expect(cleared.context.committed).toBeNull();
    expect(cleared.context.draft).toBe("");
    expect(cleared.effects).toContainEqual({ type: "emitValueChange", value: null });
  });

  test("enter commits; unresolved enter is inert; blur and escape revert", () => {
    const editing = numberInputTransition(numberInputContext({ committed: 1 }), {
      type: "RAW_EDIT",
      text: "2",
    }).context;
    const committed = numberInputTransition(editing, { type: "ENTER" });
    expect(committed.context.draft).toBeNull();
    expect(committed.effects).toContainEqual({ type: "emitCommit", value: 2 });

    const invalid = numberInputTransition(numberInputContext({ committed: 1 }), {
      type: "RAW_EDIT",
      text: "1e2",
    }).context;
    expect(numberInputTransition(invalid, { type: "ENTER" }).effects).toEqual([]);

    const reverted = numberInputTransition(invalid, { type: "ESCAPE" });
    expect(reverted.context.draft).toBeNull();
    expect(reverted.context.committed).toBe(1);
    expect(reverted.effects).toEqual([{ type: "emitDraftValueChange", draft: null }]);

    const blurRevert = numberInputTransition(invalid, { type: "BLUR" });
    expect(blurRevert.context.draft).toBeNull();
    expect(blurRevert.effects).toEqual([{ type: "emitDraftValueChange", draft: null }]);
  });

  test("step emits value, clears draft, and commits", () => {
    const result = numberInputTransition(numberInputContext({ committed: 1, step: 0.5 }), {
      type: "STEP",
      direction: 1,
    });
    expect(result.context.committed).toBe(1.5);
    expect(result.context.draft).toBeNull();
    expect(result.effects).toEqual([
      { type: "emitValueChange", value: 1.5 },
      { type: "emitCommit", value: 1.5 },
    ]);
  });

  test("replace discards draft; disabled and read-only stay inert", () => {
    const withDraft = numberInputContext({ committed: 1, draft: "-" });
    const replaced = numberInputTransition(withDraft, { type: "REPLACE", value: 9 });
    expect(replaced.context).toEqual({ ...withDraft, committed: 9, draft: null });
    expect(replaced.effects).toEqual([{ type: "emitDraftValueChange", draft: null }]);

    const disabled = numberInputTransition(numberInputContext({ committed: 1, disabled: true }), {
      type: "STEP",
      direction: 1,
    });
    expect(disabled.effects).toEqual([]);

    const readOnly = numberInputTransition(numberInputContext({ committed: 1, readOnly: true }), {
      type: "RAW_EDIT",
      text: "2",
    });
    expect(readOnly.effects).toEqual([]);
  });

  test("invalid config produces no mutation", () => {
    const result = numberInputTransition(numberInputContext({ committed: 1, step: 0 }), {
      type: "STEP",
      direction: 1,
    });
    expect(result.effects).toEqual([]);
    expect(formatNumberCommitted(1.5, 2)).toBe("1.50");
  });
});
