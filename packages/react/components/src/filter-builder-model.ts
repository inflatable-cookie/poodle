// FilterBuilder pure model — mirrors @inflatable-cookie/poodle-svelte's filter-builder-model.ts
// verbatim (React keeps its own copy, same as OrderBy). No app vocabulary, no
// expression evaluation.

import type {
  FilterClause,
  FilterFieldDefinition,
  FilterFieldKind,
  FilterOperand,
  FilterOperandKind,
  FilterOperatorDefinition,
  FilterOption,
} from "./types";

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

export function resolveOperators(field: FilterFieldDefinition): FilterOperatorDefinition[] {
  return field.operators?.length ? field.operators : defaultOperatorsForKind(field.kind);
}

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

export function isClauseComplete(
  field: FilterFieldDefinition,
  clause: { operator: string; operand: FilterOperand },
): boolean {
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
      const { min, max } = operand;
      if (min !== null && max !== null) return `${min} – ${max}`;
      if (min !== null) return `≥ ${min}`;
      if (max !== null) return `≤ ${max}`;
      return "";
    }
    default:
      return "";
  }
}

export function clauseLabel(field: FilterFieldDefinition | undefined, clause: FilterClause): string {
  if (!field) return `${clause.key} ${clause.operator}`.trim();
  const operator = findOperator(field, clause.operator);
  const operatorLabel = operator?.label ?? clause.operator;
  const value = operandText(field, clause.operand);
  return value ? `${field.label} ${operatorLabel} ${value}` : `${field.label} ${operatorLabel}`;
}
