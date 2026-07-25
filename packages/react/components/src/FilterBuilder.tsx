import { useEffect, useId, useRef, useState, type MouseEvent } from "react";
import { layerContains, registerDismissLayer } from "@poodle/headless";

import "@poodle/styles/filter-builder.css";
// Reuse SelectionSummary's chip treatment (split-chip classes) for the inline
// clause pills — single CSS source, no visual fork. Pills render inline in the
// trigger block rather than via the SelectionSummary section component.
import "@poodle/styles/selection-summary.css";

import { AnchoredSurface } from "./AnchoredSurface";
import { Button } from "./Button";
import { Checkbox } from "./Checkbox";
import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { NumberInput } from "./NumberInput";
import { SegmentedControl } from "./SegmentedControl";
import { Select } from "./Select";
import { TextInput } from "./TextInput";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import {
  clauseLabel,
  cloneOperand,
  emptyOperand,
  findOperator,
  isClauseComplete,
  resolveDefaultOperator,
  resolveOperators,
} from "./filter-builder-model";
import type {
  ControlDensity,
  ControlSize,
  FilterClause,
  FilterCombinator,
  FilterExpression,
  FilterOperand,
  FilterFieldDefinition,
  SemanticControlSizeRole,
} from "./types";

let nextClauseId = 0;

export interface FilterBuilderProps {
  fields?: FilterFieldDefinition[];
  value?: FilterExpression;
  ariaLabel?: string;
  disabled?: boolean;
  sizeRole?: SemanticControlSizeRole;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  maxClauses?: number | null;
  compact?: boolean;
  showClearButton?: boolean;
  showPills?: boolean;
  /** Show the `Match all` / `Match any` root-combinator toggle (only ever appears
   * with 2+ clauses). Off by default — most filter sets are AND-only. The
   * expression still carries a combinator (defaults `"and"`); this gates the UI. */
  showCombinator?: boolean;
  onChange?: ((value: FilterExpression) => void) | null;
}

export function FilterBuilder({
  fields = [],
  value,
  ariaLabel = "Filter",
  disabled = false,
  sizeRole = "control",
  size = null,
  density = null,
  maxClauses = null,
  compact = false,
  showClearButton = true,
  showPills = true,
  showCombinator = false,
  onChange = null,
}: FilterBuilderProps) {
  const uiPresentation = useUiPresentation();
  const panelId = useId();

  const [open, setOpen] = useState(false);
  const [uncontrolledValue, setUncontrolledValue] = useState<FilterExpression>({
    combinator: "and",
    clauses: [],
  });
  const [draftKey, setDraftKey] = useState("");
  const [draftOperator, setDraftOperator] = useState("");
  const [draftOperand, setDraftOperand] = useState<FilterOperand>({ kind: "none" });
  const [editingId, setEditingId] = useState<string | null>(null);
  // The root is state, not a ref: the portalled surface has to re-render
  // once it exists so it can be positioned against it.
  const [rootElement, setRootElement] = useState<HTMLDivElement | null>(null);
  const panelRef = useRef<HTMLDivElement | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const hasValueProp = value !== undefined;
  const effectiveValue: FilterExpression = hasValueProp
    ? value ?? { combinator: "and", clauses: [] }
    : uncontrolledValue;
  const clauses = effectiveValue.clauses;
  const combinator = effectiveValue.combinator;
  const fieldMap = new Map(fields.map((field) => [field.key, field]));

  const draftField = draftKey ? fieldMap.get(draftKey) : undefined;
  const draftOperators = draftField ? resolveOperators(draftField) : [];
  const draftOperatorDef = draftField ? findOperator(draftField, draftOperator) : undefined;
  const draftValid = draftField
    ? isClauseComplete(draftField, { operator: draftOperator, operand: draftOperand })
    : false;

  const activeCount = clauses.length;
  const canAddMore = maxClauses === null || activeCount < maxClauses;
  const availableFields = fields
    .filter((field) => !field.disabled)
    .filter((field) => field.allowMultiple || !clauses.some((clause) => clause.key === field.key));
  const addSelectItems = availableFields.map((field) => ({ value: field.key, label: field.label }));
  // Mode is "active" whenever opted in with 2+ clauses (label reflects it in
  // every state); the switch only renders when the popover was opened from the
  // trigger, not when editing an individual chip.
  const combinatorActive = showCombinator && clauses.length >= 2;
  const combinatorVisible = combinatorActive && editingId === null;
  const openerLabel = combinatorActive ? (combinator === "and" ? "All" : "Any") : "Filter";
  const isDrafting = draftKey !== "";
  const showAddRow = !isDrafting && canAddMore && availableFields.length > 0;
  const summaryText =
    activeCount === 0 ? "Filter" : activeCount === 1 ? "1 filter" : `${activeCount} filters`;
  const triggerAriaLabel = `${ariaLabel}${
    combinatorActive ? (combinator === "and" ? ", match all" : ", match any") : ""
  }${activeCount > 0 ? `, ${activeCount} active` : ""}`;

  useEffect(() => {
    if (!open) return;
    const firstFocusable = panelRef.current?.querySelector<HTMLElement>(
      'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])',
    );
    firstFocusable?.focus();
  }, [open]);

  useEffect(() => {
    if (!open) return;
    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target as Node, rootElement, panelRef.current),
      dismissOnOutsideInteract: true,
      onDismiss: () => {
        setOpen(false);
        resetDraft();
      },
    });
  }, [open]);

  function resetDraft(): void {
    setDraftKey("");
    setDraftOperator("");
    setDraftOperand({ kind: "none" });
    setEditingId(null);
  }

  function sync(next: FilterExpression): void {
    if (!hasValueProp) setUncontrolledValue(next);
    onChange?.(next);
  }

  function makeClauseId(key: string): string {
    return `${key}-${++nextClauseId}`;
  }

  function beginDraftField(key: string): void {
    if (!key || disabled) return;
    const field = fieldMap.get(key);
    if (!field) return;
    const operatorKey = resolveDefaultOperator(field);
    const operatorDef = findOperator(field, operatorKey);
    setDraftKey(key);
    setEditingId(null);
    setDraftOperator(operatorKey);
    setDraftOperand(operatorDef ? emptyOperand(operatorDef.operandKind) : { kind: "none" });
  }

  function changeDraftOperator(operatorKey: string): void {
    if (!draftField) return;
    const operatorDef = findOperator(draftField, operatorKey);
    setDraftOperator(operatorKey);
    if (operatorDef && operatorDef.operandKind !== draftOperand.kind) {
      setDraftOperand(emptyOperand(operatorDef.operandKind));
    }
  }

  function commitDraft(): void {
    if (disabled || !draftField || !draftValid) return;
    const clause: FilterClause = {
      id: editingId ?? makeClauseId(draftKey),
      key: draftKey,
      operator: draftOperator,
      operand: draftOperand,
    };
    const nextClauses = editingId
      ? clauses.map((existing) => (existing.id === editingId ? clause : existing))
      : [...clauses, clause];
    sync({ combinator, clauses: nextClauses });
    resetDraft();
  }

  function editClause(id: string): void {
    if (disabled) return;
    const clause = clauses.find((existing) => existing.id === id);
    if (!clause) return;
    setDraftKey(clause.key);
    setDraftOperator(clause.operator);
    setDraftOperand(cloneOperand(clause.operand));
    setEditingId(id);
    setOpen(true);
  }

  function removeClause(id: string): void {
    if (disabled) return;
    if (editingId === id) resetDraft();
    sync({ combinator, clauses: clauses.filter((clause) => clause.id !== id) });
  }

  function setCombinator(next: string): void {
    if (disabled) return;
    sync({ combinator: next as FilterCombinator, clauses });
  }

  function clearAll(): void {
    if (disabled) return;
    resetDraft();
    sync({ combinator, clauses: [] });
  }

  function handleResetClick(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    clearAll();
  }

  function setNumberOperand(next: number | string | null): void {
    const parsed = next === null || next === "" ? Number.NaN : Number(next);
    setDraftOperand({ kind: "number", value: parsed });
  }

  function toggleOption(optionValue: string, checked: boolean): void {
    const current = draftOperand.kind === "options" ? draftOperand.values : [];
    const values = checked
      ? [...current, optionValue]
      : current.filter((value) => value !== optionValue);
    setDraftOperand({ kind: "options", values });
  }

  function setRangeBound(bound: "min" | "max", next: number | string | null): void {
    const parsed = next === null || next === "" ? null : Number(next);
    const value = parsed !== null && Number.isNaN(parsed) ? null : parsed;
    const base =
      draftOperand.kind === "range" ? draftOperand : { kind: "range" as const, min: null, max: null };
    setDraftOperand({
      kind: "range",
      min: bound === "min" ? value : base.min,
      max: bound === "max" ? value : base.max,
    });
  }

  const booleanOperandValue =
    draftOperand.kind === "boolean" ? (draftOperand.value ? "true" : "false") : "true";
  const textOperandValue = draftOperand.kind === "text" ? draftOperand.value : "";
  const numberOperandValue =
    draftOperand.kind === "number" && Number.isFinite(draftOperand.value) ? draftOperand.value : null;
  const selectedOptionValues = draftOperand.kind === "options" ? draftOperand.values : [];
  const rangeMin = draftOperand.kind === "range" ? draftOperand.min : null;
  const rangeMax = draftOperand.kind === "range" ? draftOperand.max : null;
  const enumSelectValue = selectedOptionValues[0] ?? "";

  return (
    <div
      ref={setRootElement}
      className="poodle-filter-builder-popover"
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div
        className="poodle-filter-builder"
        role="group"
        aria-label={ariaLabel}
        data-disabled={disabled}
        data-compact={compact}
        data-open={open}
        data-size={resolvedSize}
        data-density={resolvedDensity}
      >
        <button
          type="button"
          className="poodle-filter-builder__trigger"
          disabled={disabled}
          aria-label={triggerAriaLabel}
          aria-haspopup="dialog"
          aria-expanded={open}
          aria-controls={open ? panelId : undefined}
          onClick={() => {
            if (disabled) return;
            setOpen((o) => {
              if (o) resetDraft();
              return !o;
            });
          }}
        >
          {!compact ? (
            <span
              className="poodle-filter-builder__label"
              data-combinator={combinatorActive ? "true" : "false"}
            >
              {openerLabel}
            </span>
          ) : null}
          {!(showPills && activeCount > 0) ? (
            <span className="poodle-filter-builder__summary" data-placeholder={activeCount === 0}>
              {summaryText}
            </span>
          ) : null}
          <span className="poodle-filter-builder__chevron" aria-hidden="true">
            ▾
          </span>
        </button>

        {showPills && activeCount > 0
          ? clauses.map((clause) => {
              const pillText = clauseLabel(fieldMap.get(clause.key), clause);
              return (
                <span
                  key={clause.id}
                  className="poodle-selection-summary__chip poodle-selection-summary__chip--split poodle-filter-builder__pill"
                >
                  <button
                    type="button"
                    className="poodle-selection-summary__chip-activate"
                    disabled={disabled}
                    onClick={() => editClause(clause.id)}
                    aria-label={`Edit ${pillText}`}
                  >
                    {pillText}
                  </button>
                  <button
                    type="button"
                    className="poodle-selection-summary__chip-remove"
                    disabled={disabled}
                    onClick={() => removeClause(clause.id)}
                    aria-label={`Remove ${pillText}`}
                  >
                    <Icon name="x" size="xs" />
                  </button>
                </span>
              );
            })
          : null}

        {activeCount > 0 && (showPills || showClearButton) ? (
          <span className="poodle-filter-builder__trailing">
            {showPills ? (
              <span className="poodle-filter-builder__count" aria-hidden="true">
                {activeCount}
              </span>
            ) : null}
            {showClearButton ? (
              <span className="poodle-filter-builder__reset">
                <IconButton
                  icon="x"
                  ariaLabel="Clear filters"
                  variant="ghost"
                  size={resolvedSize}
                  disabled={disabled}
                  onClick={handleResetClick}
                />
              </span>
            ) : null}
          </span>
        ) : null}
      </div>

      {open ? (
        <AnchoredSurface
          ref={panelRef}
          anchor={rootElement}
          placement="bottom-start"
          offset={8}
          id={panelId}
          className="poodle-filter-builder__surface"
          role="dialog"
          aria-label={`Edit ${ariaLabel.toLowerCase()}s`}
          tabIndex={-1}
        >
          <div className="poodle-filter-builder__panel">
            {combinatorVisible ? (
              <div className="poodle-filter-builder__combinator">
                <SegmentedControl
                  options={[
                    { value: "and", label: "Match all" },
                    { value: "or", label: "Match any" },
                  ]}
                  value={combinator}
                  ariaLabel="Combine filters"
                  size={resolvedSize}
                  density={resolvedDensity}
                  equalWidth
                  disabled={disabled}
                  onValueChange={setCombinator}
                />
              </div>
            ) : null}

            {isDrafting && draftField ? (
              <div className="poodle-filter-builder__draft">
                <span className="poodle-filter-builder__draft-field">{draftField.label}</span>

                {draftOperators.length > 1 ? (
                  <Select
                    options={draftOperators.map((op) => ({ value: op.key, label: op.label }))}
                    value={draftOperator}
                    ariaLabel={`Operator for ${draftField.label}`}
                    size={resolvedSize}
                    density={resolvedDensity}
                    disabled={disabled}
                    onValueChange={changeDraftOperator}
                  />
                ) : null}

                {draftOperatorDef ? (
                  <FilterOperandEditor
                    operandKind={draftOperatorDef.operandKind}
                    field={draftField}
                    fieldLabel={draftField.label}
                    resolvedSize={resolvedSize}
                    resolvedDensity={resolvedDensity}
                    disabled={disabled}
                    booleanValue={booleanOperandValue}
                    textValue={textOperandValue}
                    numberValue={numberOperandValue}
                    enumValue={enumSelectValue}
                    selectedValues={selectedOptionValues}
                    rangeMin={rangeMin}
                    rangeMax={rangeMax}
                    onBoolean={(v) => setDraftOperand({ kind: "boolean", value: v === "true" })}
                    onText={(v) => setDraftOperand({ kind: "text", value: v })}
                    onNumber={setNumberOperand}
                    onEnum={(v) => setDraftOperand({ kind: "options", values: v ? [v] : [] })}
                    onToggleOption={toggleOption}
                    onRange={setRangeBound}
                  />
                ) : null}

                <div className="poodle-filter-builder__draft-actions">
                  <Button
                    variant="primary"
                    size={resolvedSize}
                    disabled={disabled || !draftValid}
                    onClick={commitDraft}
                  >
                    {editingId ? "Update" : "Add"}
                  </Button>
                  <Button variant="ghost" size={resolvedSize} disabled={disabled} onClick={resetDraft}>
                    Cancel
                  </Button>
                </div>
              </div>
            ) : null}

            {showAddRow ? (
              <div className="poodle-filter-builder__add">
                <Select
                  options={addSelectItems}
                  value=""
                  placeholder="+ Add filter"
                  ariaLabel="Add filter field"
                  size={resolvedSize}
                  density={resolvedDensity}
                  disabled={disabled}
                  onValueChange={beginDraftField}
                />
              </div>
            ) : null}

            {activeCount === 0 && !isDrafting ? (
              <p className="poodle-filter-builder__empty">No filters</p>
            ) : null}
          </div>
        </AnchoredSurface>
      ) : null}
    </div>
  );
}

interface OperandEditorProps {
  operandKind: string;
  field: FilterFieldDefinition;
  fieldLabel: string;
  resolvedSize: ControlSize;
  resolvedDensity: ControlDensity;
  disabled: boolean;
  booleanValue: string;
  textValue: string;
  numberValue: number | null;
  enumValue: string;
  selectedValues: string[];
  rangeMin: number | null;
  rangeMax: number | null;
  onBoolean: (value: string) => void;
  onText: (value: string) => void;
  onNumber: (value: number | string | null) => void;
  onEnum: (value: string) => void;
  onToggleOption: (value: string, checked: boolean) => void;
  onRange: (bound: "min" | "max", value: number | string | null) => void;
}

function FilterOperandEditor({
  operandKind,
  field,
  fieldLabel,
  resolvedSize,
  resolvedDensity,
  disabled,
  booleanValue,
  textValue,
  numberValue,
  enumValue,
  selectedValues,
  rangeMin,
  rangeMax,
  onBoolean,
  onText,
  onNumber,
  onEnum,
  onToggleOption,
  onRange,
}: OperandEditorProps) {
  if (operandKind === "boolean") {
    return (
      <SegmentedControl
        options={[
          { value: "true", label: "True" },
          { value: "false", label: "False" },
        ]}
        value={booleanValue}
        ariaLabel={`Value for ${fieldLabel}`}
        size={resolvedSize}
        density={resolvedDensity}
        equalWidth
        disabled={disabled}
        onValueChange={onBoolean}
      />
    );
  }
  if (operandKind === "text") {
    return (
      <TextInput
        value={textValue}
        ariaLabel={`Value for ${fieldLabel}`}
        size={resolvedSize}
        density={resolvedDensity}
        disabled={disabled}
        onValueChange={onText}
      />
    );
  }
  if (operandKind === "number") {
    return (
      <NumberInput
        value={numberValue}
        ariaLabel={`Value for ${fieldLabel}`}
        size={resolvedSize}
        density={resolvedDensity}
        disabled={disabled}
        onValueChange={onNumber}
      />
    );
  }
  if (operandKind === "options") {
    if (field.kind === "enum") {
      return (
        <Select
          options={(field.options ?? []).map((option) => ({
            value: option.value,
            label: option.label,
            disabled: option.disabled,
          }))}
          value={enumValue}
          placeholder="Select…"
          ariaLabel={`Value for ${fieldLabel}`}
          size={resolvedSize}
          density={resolvedDensity}
          disabled={disabled}
          onValueChange={onEnum}
        />
      );
    }
    return (
      <div
        className="poodle-filter-builder__options"
        role="group"
        aria-label={`Values for ${fieldLabel}`}
      >
        {(field.options ?? []).map((option) => (
          <Checkbox
            key={option.value}
            label={option.label}
            checked={selectedValues.includes(option.value)}
            disabled={disabled || option.disabled}
            size={resolvedSize}
            onCheckedChange={(checked) => onToggleOption(option.value, checked)}
          />
        ))}
      </div>
    );
  }
  if (operandKind === "range") {
    return (
      <div className="poodle-filter-builder__range">
        <NumberInput
          value={rangeMin}
          ariaLabel={`Minimum for ${fieldLabel}`}
          size={resolvedSize}
          density={resolvedDensity}
          disabled={disabled}
          onValueChange={(next) => onRange("min", next)}
        />
        <span className="poodle-filter-builder__range-sep" aria-hidden="true">
          –
        </span>
        <NumberInput
          value={rangeMax}
          ariaLabel={`Maximum for ${fieldLabel}`}
          size={resolvedSize}
          density={resolvedDensity}
          disabled={disabled}
          onValueChange={(next) => onRange("max", next)}
        />
      </div>
    );
  }
  return null;
}
