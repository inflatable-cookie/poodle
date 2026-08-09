import { describe, expect, it } from "bun:test";

import {
  clauseLabel,
  cloneOperand,
  defaultOperatorsForKind,
  emptyOperand,
  isClauseComplete,
  isOperandValid,
  resolveDefaultOperator,
  resolveOperators,
  type FilterClause,
  type FilterFieldDefinition,
} from "../src/filter-builder";

const formatField: FilterFieldDefinition = {
  key: "format",
  label: "Format",
  kind: "multi-enum",
  options: [
    { value: "clap", label: "CLAP" },
    { value: "vst3", label: "VST3" },
  ],
};

describe("filter builder", () => {
  it("provides the standard operators for every field kind", () => {
    expect(defaultOperatorsForKind("boolean").map(({ key }) => key)).toEqual(["is"]);
    expect(defaultOperatorsForKind("enum").map(({ key }) => key)).toEqual(["is", "is_not"]);
    expect(defaultOperatorsForKind("multi-enum").map(({ key }) => key)).toEqual([
      "any_of",
      "all_of",
      "none_of",
    ]);
    expect(defaultOperatorsForKind("text").map(({ key }) => key)).toEqual([
      "contains",
      "not_contains",
      "equals",
      "starts_with",
      "ends_with",
    ]);
    expect(defaultOperatorsForKind("number").map(({ key }) => key)).toEqual([
      "eq",
      "neq",
      "gt",
      "gte",
      "lt",
      "lte",
    ]);
    expect(defaultOperatorsForKind("range").map(({ key }) => key)).toEqual([
      "between",
      "outside",
    ]);
  });

  it("honors a valid custom default and falls back to the first operator", () => {
    const field: FilterFieldDefinition = {
      key: "score",
      label: "Score",
      kind: "number",
      defaultOperator: "gte",
    };
    expect(resolveDefaultOperator(field)).toBe("gte");
    expect(resolveDefaultOperator({ ...field, defaultOperator: "unknown" })).toBe("eq");
  });

  it("uses custom operator definitions without merging defaults", () => {
    const operators = [{ key: "matches", label: "matches", operandKind: "text" as const }];
    expect(resolveOperators({ key: "name", label: "Name", kind: "text", operators })).toBe(
      operators,
    );
  });

  it("creates invalid blank operands until their required value is present", () => {
    expect(isOperandValid(emptyOperand("none"))).toBe(true);
    expect(isOperandValid(emptyOperand("boolean"))).toBe(true);
    expect(isOperandValid(emptyOperand("text"))).toBe(false);
    expect(isOperandValid(emptyOperand("number"))).toBe(false);
    expect(isOperandValid(emptyOperand("options"))).toBe(false);
    expect(isOperandValid(emptyOperand("range"))).toBe(false);
  });

  it("requires a known operator with the matching operand kind", () => {
    expect(
      isClauseComplete(formatField, {
        operator: "any_of",
        operand: { kind: "options", values: ["clap"] },
      }),
    ).toBe(true);
    expect(
      isClauseComplete(formatField, {
        operator: "any_of",
        operand: { kind: "text", value: "clap" },
      }),
    ).toBe(false);
    expect(
      isClauseComplete(formatField, {
        operator: "missing",
        operand: { kind: "options", values: ["clap"] },
      }),
    ).toBe(false);
  });

  it("clones option arrays instead of sharing draft state", () => {
    const source = { kind: "options" as const, values: ["clap"] };
    const copy = cloneOperand(source);
    expect(copy).toEqual(source);
    expect(copy).not.toBe(source);
    if (copy.kind === "options") expect(copy.values).not.toBe(source.values);
  });

  it("builds labels from field, operator, and option display names", () => {
    const clause: FilterClause = {
      id: "format-1",
      key: "format",
      operator: "any_of",
      operand: { kind: "options", values: ["clap", "vst3"] },
    };
    expect(clauseLabel(formatField, clause)).toBe("Format is any of CLAP, VST3");
    expect(clauseLabel(undefined, clause)).toBe("format any_of");
  });
});
