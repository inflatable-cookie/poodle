// FilterBuilder pure model. Renderer-neutral logic: standard operators per field
// kind, operand construction, draft validity, and human-readable clause labels.
// This is the canonical TypeScript source; the React shell mirrors it and the
// Rust spec (`poodle-specs::filter_builder`) re-implements the same semantics.
// No application vocabulary and no expression evaluation live here.

import type {
  FilterClause,
  FilterFieldDefinition,
  FilterFieldKind,
  FilterOperand,
  FilterOperandKind,
  FilterOperatorDefinition,
  FilterOption,
} from "./types.ts";

/** Standard operator set for each field kind. A field definition may restrict or
 * relabel these via `field.operators`. Operator keys are stable identifiers; the
 * host owns their evaluation semantics. */
export function defaultOperatorsForKind(kind: FilterFieldKind): FilterOperatorDefinition[] {
  switch (kind) {
    case "boolean":
      return [{ key: "is", label: "is", operandKind: "boolean" }];
    case "enum":
      return [
        { key: "is", label: "is", operandKind: "options" },
        { key: "is_not", label: "is not", operandKind: "options" },
      ];
    case "multi-enum":
      return [
        { key: "any_of", label: "is any of", operandKind: "options" },
        { key: "all_of", label: "is all of", operandKind: "options" },
        { key: "none_of", label: "is none of", operandKind: "options" },
      ];
    case "text":
      return [
        { key: "contains", label: "contains", operandKind: "text" },
        { key: "not_contains", label: "does not contain", operandKind: "text" },
        { key: "equals", label: "equals", operandKind: "text" },
        { key: "starts_with", label: "starts with", operandKind: "text" },
        { key: "ends_with", label: "ends with", operandKind: "text" },
      ];
    case "number":
      return [
        { key: "eq", label: "equals", operandKind: "number" },
        { key: "neq", label: "not equal", operandKind: "number" },
        { key: "gt", label: "greater than", operandKind: "number" },
        { key: "gte", label: "at least", operandKind: "number" },
        { key: "lt", label: "less than", operandKind: "number" },
        { key: "lte", label: "at most", operandKind: "number" },
      ];
    case "range":
      return [
        { key: "between", label: "between", operandKind: "range" },
        { key: "outside", label: "outside", operandKind: "range" },
      ];
    default:
      return [];
  }
}

/** The effective operator list for a field (custom override or kind defaults). */
export function resolveOperators(field: FilterFieldDefinition): FilterOperatorDefinition[] {
  const operators = field.operators?.length ? field.operators : defaultOperatorsForKind(field.kind);
  return operators;
}

/** The operator key a fresh draft should start on. */
export function resolveDefaultOperator(field: FilterFieldDefinition): string {
  const operators = resolveOperators(field);
  if (field.defaultOperator && operators.some((op) => op.key === field.defaultOperator)) {
    return field.defaultOperator;
  }
  return operators[0]?.key ?? "";
}

export function findOperator(
  field: FilterFieldDefinition,
  operatorKey: string,
): FilterOperatorDefinition | undefined {
  return resolveOperators(field).find((op) => op.key === operatorKey);
}

/** A blank operand of the given kind, used to seed a draft. Number uses `NaN` as
 * the unset sentinel so `0` remains a valid entered value. */
export function emptyOperand(operandKind: FilterOperandKind): FilterOperand {
  switch (operandKind) {
    case "none":
      return { kind: "none" };
    case "text":
      return { kind: "text", value: "" };
    case "number":
      return { kind: "number", value: Number.NaN };
    case "boolean":
      return { kind: "boolean", value: true };
    case "options":
      return { kind: "options", values: [] };
    case "range":
      return { kind: "range", min: null, max: null };
    default:
      return { kind: "none" };
  }
}

/** Independent copy of an operand. Used when loading a committed clause into a
 * draft so draft edits don't mutate the source. Avoids `structuredClone`, which
 * throws on reactive proxies. */
export function cloneOperand(operand: FilterOperand): FilterOperand {
  switch (operand.kind) {
    case "options":
      return { kind: "options", values: [...operand.values] };
    case "range":
      return { kind: "range", min: operand.min, max: operand.max };
    default:
      return { ...operand };
  }
}

/** Whether an operand carries enough data to commit. Incomplete operands must
 * never be emitted through `onChange`. */
export function isOperandValid(operand: FilterOperand): boolean {
  switch (operand.kind) {
    case "none":
      return true;
    case "text":
      return operand.value.trim().length > 0;
    case "number":
      return Number.isFinite(operand.value);
    case "boolean":
      return true;
    case "options":
      return operand.values.length > 0;
    case "range":
      return operand.min !== null || operand.max !== null;
    default:
      return false;
  }
}

/** Whether a draft clause is complete: it names a valid operator whose operand
 * kind matches, and the operand itself is valid. */
export function isClauseComplete(field: FilterFieldDefinition, clause: {
  operator: string;
  operand: FilterOperand;
}): boolean {
  const operator = findOperator(field, clause.operator);
  if (!operator) return false;
  if (operator.operandKind !== clause.operand.kind) return false;
  return isOperandValid(clause.operand);
}

function optionLabel(options: FilterOption[] | undefined, value: string): string {
  return options?.find((option) => option.value === value)?.label ?? value;
}

function operandText(field: FilterFieldDefinition, operand: FilterOperand): string {
  switch (operand.kind) {
    case "none":
      return "";
    case "text":
      return `"${operand.value}"`;
    case "number":
      return Number.isFinite(operand.value) ? String(operand.value) : "";
    case "boolean":
      return operand.value ? "true" : "false";
    case "options":
      return operand.values.map((value) => optionLabel(field.options, value)).join(", ");
    case "range": {
      const min = operand.min;
      const max = operand.max;
      if (min !== null && max !== null) return `${min} – ${max}`;
      if (min !== null) return `≥ ${min}`;
      if (max !== null) return `≤ ${max}`;
      return "";
    }
    default:
      return "";
  }
}

/** Human-readable pill label for a committed clause, e.g. "Format is any of CLAP,
 * VST3" or "Hidden is false". Falls back to keys when a field/operator is unknown. */
export function clauseLabel(field: FilterFieldDefinition | undefined, clause: FilterClause): string {
  if (!field) return `${clause.key} ${clause.operator}`.trim();
  const operator = findOperator(field, clause.operator);
  const operatorLabel = operator?.label ?? clause.operator;
  const value = operandText(field, clause.operand);
  return value ? `${field.label} ${operatorLabel} ${value}` : `${field.label} ${operatorLabel}`;
}
