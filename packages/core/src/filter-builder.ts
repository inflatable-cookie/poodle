// Renderer-neutral FilterBuilder types and behavior. Poodle models fields,
// operators, operands, and a root combinator; hosts own evaluation and storage.

export type FilterCombinator = "and" | "or";

export type FilterFieldKind =
  | "boolean"
  | "enum"
  | "multi-enum"
  | "text"
  | "number"
  | "range";

export type FilterOperandKind =
  | "none"
  | "text"
  | "number"
  | "boolean"
  | "options"
  | "range";

/** Discriminated operand payload. */
export type FilterOperand =
  | { kind: "none" }
  | { kind: "text"; value: string }
  | { kind: "number"; value: number }
  | { kind: "boolean"; value: boolean }
  | { kind: "options"; values: string[] }
  | { kind: "range"; min: number | null; max: number | null };

export type FilterOption = {
  value: string;
  label: string;
  disabled?: boolean;
  group?: string;
};

export type FilterOperatorDefinition = {
  key: string;
  label: string;
  operandKind: FilterOperandKind;
};

export type FilterFieldDefinition = {
  key: string;
  label: string;
  kind: FilterFieldKind;
  /** Restricts or relabels the standard operators for this field. */
  operators?: FilterOperatorDefinition[];
  /** Options for `enum` and `multi-enum` fields. */
  options?: FilterOption[];
  /** Operator key selected when a new draft opens. */
  defaultOperator?: string;
  /** Allows more than one active clause for the field. Defaults to false. */
  allowMultiple?: boolean;
  disabled?: boolean;
};

/** A committed clause with a stable, opaque UI identity. */
export type FilterClause = {
  id: string;
  key: string;
  operator: string;
  operand: FilterOperand;
};

/** A flat clause list under one root combinator. */
export type FilterExpression = {
  combinator: FilterCombinator;
  clauses: FilterClause[];
};

/** Standard operators for a field kind. Hosts own their evaluation semantics. */
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
  if (field.defaultOperator && operators.some((operator) => operator.key === field.defaultOperator)) {
    return field.defaultOperator;
  }
  return operators[0]?.key ?? "";
}

export function findOperator(
  field: FilterFieldDefinition,
  operatorKey: string,
): FilterOperatorDefinition | undefined {
  return resolveOperators(field).find((operator) => operator.key === operatorKey);
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

/** Copies nested operand data without relying on `structuredClone`, which
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
  if (!operator || operator.operandKind !== clause.operand.kind) return false;
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
