<script module lang="ts">
  let nextFilterBuilderId = 0;
  let nextClauseId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-styles/filter-builder.css";
  // Reuse SelectionSummary's chip treatment (split-chip classes) for the inline
  // clause pills — single CSS source, no visual fork. The pills render inline in
  // the trigger block rather than via the SelectionSummary section component.
  import "@inflatable-cookie/poodle-styles/selection-summary.css";
  import { layerContains, registerDismissLayer } from "@inflatable-cookie/poodle-headless";
  import { tick } from "svelte";

  import { anchored } from "./anchored";
  import { default as Button } from "./Button.svelte";
  import { default as Checkbox } from "./Checkbox.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as NumberInput } from "./NumberInput.svelte";
  import { default as SegmentedControl } from "./SegmentedControl.svelte";
  import { default as Select } from "./Select.svelte";
  import { default as TextInput } from "./TextInput.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
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
    FilterFieldDefinition,
    FilterOperand,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    fields?: FilterFieldDefinition[];
    value?: FilterExpression | undefined;
    ariaLabel?: string;
    disabled?: boolean;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    maxClauses?: number | null;
    compact?: boolean;
    showClearButton?: boolean;
    showPills?: boolean;
    /** Show the `Match all` / `Match any` root-combinator toggle (only ever
     * appears with 2+ clauses). Off by default — most filter sets are AND-only,
     * so the toggle is irrelevant noise there. The expression still carries a
     * combinator (defaults `"and"`); this only gates the UI switch. */
    showCombinator?: boolean;
    onChange?: ((value: FilterExpression) => void) | null;
  }

  let {
    fields = [],
    value = $bindable<FilterExpression | undefined>(undefined),
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
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const panelId = `poodle-filter-builder-${++nextFilterBuilderId}`;

  let open = $state(false);
  let rootElement = $state<HTMLDivElement | null>(null);
  let panelElement = $state<HTMLDivElement | null>(null);
  let uncontrolledValue = $state<FilterExpression>({ combinator: "and", clauses: [] });

  // Draft: never emitted until valid. editingId is set when editing an existing
  // clause (its id is retained); otherwise a new clause is being composed.
  let draftKey = $state("");
  let draftOperator = $state("");
  let draftOperand = $state<FilterOperand>({ kind: "none" });
  let editingId = $state<string | null>(null);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const hasValueProp = $derived(value !== undefined);
  const effectiveValue = $derived<FilterExpression>(
    hasValueProp ? value ?? { combinator: "and", clauses: [] } : uncontrolledValue,
  );
  const clauses = $derived(effectiveValue.clauses);
  const combinator = $derived(effectiveValue.combinator);
  const fieldMap = $derived(new Map(fields.map((field) => [field.key, field])));

  const draftField = $derived<FilterFieldDefinition | undefined>(
    draftKey ? fieldMap.get(draftKey) : undefined,
  );
  const draftOperators = $derived(draftField ? resolveOperators(draftField) : []);
  const draftOperatorDef = $derived(
    draftField ? findOperator(draftField, draftOperator) : undefined,
  );
  const draftValid = $derived(
    draftField ? isClauseComplete(draftField, { operator: draftOperator, operand: draftOperand }) : false,
  );

  const activeCount = $derived(clauses.length);
  const canAddMore = $derived(maxClauses === null || activeCount < maxClauses);
  const availableFields = $derived(
    fields
      .filter((field) => !field.disabled)
      .filter((field) => field.allowMultiple || !clauses.some((clause) => clause.key === field.key)),
  );
  const addSelectItems = $derived(availableFields.map((field) => ({ value: field.key, label: field.label })));
  // The combinator mode is "active" whenever opted in with 2+ clauses — the
  // opener label reflects it in every state. The mode *switch* only renders in
  // the popover when it was opened from the trigger (the All/Any label), not when
  // editing an individual chip — the combinator combines the whole stack, so it
  // is noise in a single-chip edit.
  const combinatorActive = $derived(showCombinator && clauses.length >= 2);
  const combinatorVisible = $derived(combinatorActive && editingId === null);
  const openerLabel = $derived(
    combinatorActive ? (combinator === "and" ? "All" : "Any") : "Filter",
  );
  const isDrafting = $derived(draftKey !== "");
  const showAddRow = $derived(!isDrafting && canAddMore && availableFields.length > 0);
  const summaryText = $derived(
    activeCount === 0 ? "Filter" : activeCount === 1 ? "1 filter" : `${activeCount} filters`,
  );
  const triggerAriaLabel = $derived(
    `${ariaLabel}${combinatorActive ? (combinator === "and" ? ", match all" : ", match any") : ""}${activeCount > 0 ? `, ${activeCount} active` : ""}`,
  );

  $effect(() => {
    if (!open) return;
    tick().then(() => {
      const firstFocusable = panelElement?.querySelector<HTMLElement>(
        'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])',
      );
      firstFocusable?.focus();
    });
  });

  $effect(() => {
    if (!open) return;
    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, panelElement),
      dismissOnOutsideInteract: true,
      onDismiss: () => {
        open = false;
        resetDraft();
      },
    });
  });

  function sync(next: FilterExpression): void {
    if (hasValueProp) {
      value = next;
    } else {
      uncontrolledValue = next;
    }
    onChange?.(next);
  }

  function makeClauseId(key: string): string {
    return `${key}-${++nextClauseId}`;
  }

  function resetDraft(): void {
    draftKey = "";
    draftOperator = "";
    draftOperand = { kind: "none" };
    editingId = null;
  }

  function beginDraftField(key: string): void {
    if (!key || disabled) return;
    const field = fieldMap.get(key);
    if (!field) return;
    const operatorKey = resolveDefaultOperator(field);
    const operatorDef = findOperator(field, operatorKey);
    draftKey = key;
    editingId = null;
    draftOperator = operatorKey;
    draftOperand = operatorDef ? emptyOperand(operatorDef.operandKind) : { kind: "none" };
  }

  function setDraftOperator(operatorKey: string): void {
    if (!draftField) return;
    const operatorDef = findOperator(draftField, operatorKey);
    draftOperator = operatorKey;
    // Re-seed the operand when the operand kind changes.
    if (operatorDef && operatorDef.operandKind !== draftOperand.kind) {
      draftOperand = emptyOperand(operatorDef.operandKind);
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
    draftKey = clause.key;
    draftOperator = clause.operator;
    draftOperand = cloneOperand(clause.operand);
    editingId = id;
    open = true;
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

  function setOpen(next: boolean): void {
    if (disabled) return;
    open = next;
    if (!next) resetDraft();
  }

  function toggleOpen(): void {
    setOpen(!open);
  }

  function handleResetClick(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    clearAll();
  }

  // ── Operand setters ──────────────────────────────────────────────────
  function setTextOperand(next: string): void {
    draftOperand = { kind: "text", value: next };
  }

  function setNumberOperand(next: number | string | null): void {
    const parsed = next === null || next === "" ? Number.NaN : Number(next);
    draftOperand = { kind: "number", value: parsed };
  }

  function setBooleanOperand(next: string): void {
    draftOperand = { kind: "boolean", value: next === "true" };
  }

  function toggleOption(optionValue: string, checked: boolean): void {
    const current = draftOperand.kind === "options" ? draftOperand.values : [];
    const values = checked
      ? [...current, optionValue]
      : current.filter((value) => value !== optionValue);
    draftOperand = { kind: "options", values };
  }

  function setRangeBound(bound: "min" | "max", next: number | string | null): void {
    const parsed = next === null || next === "" ? null : Number(next);
    const value = parsed !== null && Number.isNaN(parsed) ? null : parsed;
    const base = draftOperand.kind === "range" ? draftOperand : { kind: "range" as const, min: null, max: null };
    draftOperand = { kind: "range", min: bound === "min" ? value : base.min, max: bound === "max" ? value : base.max };
  }

  const booleanOperandValue = $derived(
    draftOperand.kind === "boolean" ? (draftOperand.value ? "true" : "false") : "true",
  );
  const textOperandValue = $derived(draftOperand.kind === "text" ? draftOperand.value : "");
  const numberOperandValue = $derived(
    draftOperand.kind === "number" && Number.isFinite(draftOperand.value) ? draftOperand.value : null,
  );
  const selectedOptionValues = $derived(draftOperand.kind === "options" ? draftOperand.values : []);
  const rangeMin = $derived(draftOperand.kind === "range" ? draftOperand.min : null);
  const rangeMax = $derived(draftOperand.kind === "range" ? draftOperand.max : null);
  const enumSelectValue = $derived(selectedOptionValues[0] ?? "");
</script>

<div
  bind:this={rootElement}
  class="poodle-filter-builder-popover"
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <div
    class="poodle-filter-builder"
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
      class="poodle-filter-builder__trigger"
      disabled={disabled}
      aria-label={triggerAriaLabel}
      aria-haspopup="dialog"
      aria-expanded={open ? "true" : "false"}
      aria-controls={open ? panelId : undefined}
      onclick={toggleOpen}
    >
      {#if !compact}
        <span class="poodle-filter-builder__label" data-combinator={combinatorActive ? "true" : "false"}>{openerLabel}</span>
      {/if}
      {#if !(showPills && activeCount > 0)}
        <span class="poodle-filter-builder__summary" data-placeholder={activeCount === 0}>
          {summaryText}
        </span>
      {/if}
      <span class="poodle-filter-builder__chevron" aria-hidden="true">▾</span>
    </button>

    {#if showPills && activeCount > 0}
      {#each clauses as clause (clause.id)}
        {@const pillText = clauseLabel(fieldMap.get(clause.key), clause)}
        <span class="poodle-selection-summary__chip poodle-selection-summary__chip--split poodle-filter-builder__pill">
          <button
            type="button"
            class="poodle-selection-summary__chip-activate"
            disabled={disabled}
            onclick={() => editClause(clause.id)}
            aria-label={`Edit ${pillText}`}
          >
            {pillText}
          </button>
          <button
            type="button"
            class="poodle-selection-summary__chip-remove"
            disabled={disabled}
            onclick={() => removeClause(clause.id)}
            aria-label={`Remove ${pillText}`}
          >
            <Icon name="x" size="xs" />
          </button>
        </span>
      {/each}
    {/if}

    {#if activeCount > 0 && (showPills || showClearButton)}
      <span class="poodle-filter-builder__trailing">
        {#if showPills}
          <span class="poodle-filter-builder__count" aria-hidden="true">{activeCount}</span>
        {/if}
        {#if showClearButton}
          <span class="poodle-filter-builder__reset">
            <IconButton
              icon="x"
              ariaLabel="Clear filters"
              variant="ghost"
              size={resolvedSize}
              disabled={disabled}
              onClick={handleResetClick}
            />
          </span>
        {/if}
      </span>
    {/if}
  </div>

  {#if open}
    <div
      bind:this={panelElement}
      use:anchored={{ anchor: rootElement, placement: "bottom-start", offset: 8 }}
      id={panelId}
      class="poodle-filter-builder__surface"
      role="dialog"
      aria-label={`Edit ${ariaLabel.toLowerCase()}s`}
      tabindex="-1"
    >
      <div class="poodle-filter-builder__panel">
        {#if combinatorVisible}
          <div class="poodle-filter-builder__combinator">
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
        {/if}

        {#if isDrafting && draftField}
          <div class="poodle-filter-builder__draft">
            <span class="poodle-filter-builder__draft-field">{draftField.label}</span>

            {#if draftOperators.length > 1}
              <Select
                options={draftOperators.map((op) => ({ value: op.key, label: op.label }))}
                value={draftOperator}
                ariaLabel={`Operator for ${draftField.label}`}
                size={resolvedSize}
                density={resolvedDensity}
                disabled={disabled}
                onValueChange={setDraftOperator}
              />
            {/if}

            {#if draftOperatorDef}
              {#if draftOperatorDef.operandKind === "boolean"}
                <SegmentedControl
                  options={[
                    { value: "true", label: "True" },
                    { value: "false", label: "False" },
                  ]}
                  value={booleanOperandValue}
                  ariaLabel={`Value for ${draftField.label}`}
                  size={resolvedSize}
                  density={resolvedDensity}
                  equalWidth
                  disabled={disabled}
                  onValueChange={setBooleanOperand}
                />
              {:else if draftOperatorDef.operandKind === "text"}
                <TextInput
                  value={textOperandValue}
                  ariaLabel={`Value for ${draftField.label}`}
                  size={resolvedSize}
                  density={resolvedDensity}
                  disabled={disabled}
                  onValueChange={setTextOperand}
                />
              {:else if draftOperatorDef.operandKind === "number"}
                <NumberInput
                  value={numberOperandValue}
                  ariaLabel={`Value for ${draftField.label}`}
                  size={resolvedSize}
                  density={resolvedDensity}
                  disabled={disabled}
                  onValueChange={setNumberOperand}
                />
              {:else if draftOperatorDef.operandKind === "options"}
                {#if draftField.kind === "enum"}
                  <Select
                    options={(draftField.options ?? []).map((option) => ({
                      value: option.value,
                      label: option.label,
                      disabled: option.disabled,
                    }))}
                    value={enumSelectValue}
                    placeholder="Select…"
                    ariaLabel={`Value for ${draftField.label}`}
                    size={resolvedSize}
                    density={resolvedDensity}
                    disabled={disabled}
                    onValueChange={(next) => (draftOperand = { kind: "options", values: next ? [next] : [] })}
                  />
                {:else}
                  <div class="poodle-filter-builder__options" role="group" aria-label={`Values for ${draftField.label}`}>
                    {#each draftField.options ?? [] as option (option.value)}
                      <Checkbox
                        label={option.label}
                        checked={selectedOptionValues.includes(option.value)}
                        disabled={disabled || option.disabled}
                        size={resolvedSize}
                        onCheckedChange={(checked) => toggleOption(option.value, checked)}
                      />
                    {/each}
                  </div>
                {/if}
              {:else if draftOperatorDef.operandKind === "range"}
                <div class="poodle-filter-builder__range">
                  <NumberInput
                    value={rangeMin}
                    ariaLabel={`Minimum for ${draftField.label}`}
                    size={resolvedSize}
                    density={resolvedDensity}
                    disabled={disabled}
                    onValueChange={(next) => setRangeBound("min", next)}
                  />
                  <span class="poodle-filter-builder__range-sep" aria-hidden="true">–</span>
                  <NumberInput
                    value={rangeMax}
                    ariaLabel={`Maximum for ${draftField.label}`}
                    size={resolvedSize}
                    density={resolvedDensity}
                    disabled={disabled}
                    onValueChange={(next) => setRangeBound("max", next)}
                  />
                </div>
              {/if}
            {/if}

            <div class="poodle-filter-builder__draft-actions">
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
        {/if}

        {#if showAddRow}
          <div class="poodle-filter-builder__add">
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
        {/if}

        {#if activeCount === 0 && !isDrafting}
          <p class="poodle-filter-builder__empty">No filters</p>
        {/if}
      </div>
    </div>
  {/if}
</div>
