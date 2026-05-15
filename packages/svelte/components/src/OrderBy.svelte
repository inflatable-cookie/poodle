<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Button from "./Button.svelte";
  import IconButton from "./IconButton.svelte";
  import Popover from "./Popover.svelte";
  import Select from "./Select.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ActiveSort,
    ControlDensity,
    ControlSize,
    OrderByFieldDefinition,
    OrderByValue,
    SemanticControlSizeRole,
    SortDirection,
    SortField,
  } from "./types";

  export let fields: SortField[] = [];
  export let value: OrderByValue = [];
  export let activeSort: ActiveSort | null = null;
  export let ariaLabel = "Sort by";
  export let disabled = false;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let size: ControlSize | null = null;
  export let density: ControlDensity | null = null;
  export let maxFields: number | null = null;
  export let compact = false;
  export let showClearButton = true;
  export let onChange: ((value: OrderByValue) => void) | null = null;

  const dispatch = createEventDispatcher<{
    change: { value: OrderByValue; sort: ActiveSort | null };
  }>();

  const uiPresentation = getUiPresentation();
  let open = false;
  let addFieldValue = "";
  let dragIndex: number | null = null;
  let dragOverIndex: number | null = null;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  $: normalizedFields = fields.map<OrderByFieldDefinition>((field) => ({
    key: field.key ?? field.value ?? "",
    label: field.label,
    disabled: field.disabled,
    defaultDirection: field.defaultDirection ?? "asc",
  })).filter((field) => field.key.length > 0);

  $: fieldMap = new Map(normalizedFields.map((field) => [field.key, field]));
  $: legacyValue = activeSort ? [{ key: activeSort.field, direction: activeSort.direction }] : [];
  $: effectiveValue = value.length > 0 ? value : legacyValue;
  $: canAddMore = maxFields === null || effectiveValue.length < maxFields;
  $: availableFields = normalizedFields.filter((field) => !effectiveValue.some((item) => item.key === field.key));
  $: selectItems = availableFields.map((field) => ({ value: field.key, label: field.label }));
  $: activeSortValue = effectiveValue.length > 0
    ? { field: effectiveValue[0].key, direction: effectiveValue[0].direction }
    : null;
  $: triggerText = summarizeValue(effectiveValue);

  function summarizeValue(nextValue: OrderByValue): string {
    if (nextValue.length === 0) {
      return "Sort by...";
    }

    const items = nextValue.map((item) => {
      const field = fieldMap.get(item.key);
      const directionLabel = item.direction === "asc" ? "↑" : "↓";
      return `${field?.label ?? item.key} ${directionLabel}`;
    });

    if (compact && items.length > 2) {
      return `${items.slice(0, 2).join(", ")} +${items.length - 2}`;
    }

    return items.join(", ");
  }

  function sync(nextValue: OrderByValue): void {
    value = nextValue;
    activeSort = nextValue.length > 0
      ? { field: nextValue[0].key, direction: nextValue[0].direction }
      : null;

    onChange?.(nextValue);
    dispatch("change", { value: nextValue, sort: activeSort });
  }

  function addField(key: string): void {
    if (!key || disabled || effectiveValue.some((item) => item.key === key)) return;

    const field = fieldMap.get(key);
    const direction: SortDirection = field?.defaultDirection ?? "asc";
    addFieldValue = "";
    sync([...effectiveValue, { key, direction }]);
  }

  function removeField(index: number): void {
    if (disabled) return;
    sync(effectiveValue.filter((_, itemIndex) => itemIndex !== index));
  }

  function toggleDirection(index: number): void {
    if (disabled) return;

    sync(effectiveValue.map((item, itemIndex) => {
      if (itemIndex !== index) return item;
      return {
        ...item,
        direction: item.direction === "asc" ? "desc" : "asc",
      };
    }));
  }

  function moveField(index: number, offset: -1 | 1): void {
    if (disabled) return;

    const nextIndex = index + offset;
    if (nextIndex < 0 || nextIndex >= effectiveValue.length) return;

    const nextValue = [...effectiveValue];
    const [item] = nextValue.splice(index, 1);
    nextValue.splice(nextIndex, 0, item);
    sync(nextValue);
  }

  function handleDragStart(index: number): void {
    if (disabled) return;
    dragIndex = index;
    dragOverIndex = index;
  }

  function handleDragEnter(index: number): void {
    if (dragIndex === null || disabled) return;
    dragOverIndex = index;
  }

  function handleDrop(index: number): void {
    if (dragIndex === null || disabled) return;

    const nextValue = [...effectiveValue];
    const [item] = nextValue.splice(dragIndex, 1);
    nextValue.splice(index, 0, item);
    dragIndex = null;
    dragOverIndex = null;
    sync(nextValue);
  }

  function clearDragState(): void {
    dragIndex = null;
    dragOverIndex = null;
  }

  function clearAll(): void {
    if (disabled) return;
    sync([]);
  }
</script>

<Popover bind:open placement="bottom-start" ariaLabel={ariaLabel} block>
  {#snippet trigger()}
    <div
      class="poodle-order-by"
      role="group"
      aria-label={ariaLabel}
      data-disabled={disabled}
      data-compact={compact}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <button
        type="button"
        class="poodle-order-by__trigger"
        disabled={disabled}
        aria-label={ariaLabel}
      >
        <span class="poodle-order-by__label">Sort by</span>
        <span class="poodle-order-by__summary" data-placeholder={effectiveValue.length === 0}>
          {triggerText}
        </span>
        <span class="poodle-order-by__chevron" aria-hidden="true">▾</span>
      </button>

      {#if showClearButton && effectiveValue.length > 0}
        <button
          type="button"
          class="poodle-order-by__reset"
          on:click|stopPropagation|preventDefault={clearAll}
          disabled={disabled}
          aria-label="Clear sort"
        >
          <IconButton
            icon="x"
            ariaLabel="Clear sort"
            variant="ghost"
            size={resolvedSize}
            disabled={disabled}
          />
        </button>
      {/if}
    </div>
  {/snippet}

  <div class="poodle-order-by__panel">
    {#if effectiveValue.length > 0}
      <div class="poodle-order-by__list" role="list">
        {#each effectiveValue as item, index (`${item.key}-${index}`)}
          {@const field = fieldMap.get(item.key)}
          <div
            class="poodle-order-by__item"
            class:poodle-order-by__item--dragging={dragIndex === index}
            class:poodle-order-by__item--drop-target={dragOverIndex === index && dragIndex !== index}
            role="listitem"
          >
            <button
              type="button"
              class="poodle-order-by__drag-handle"
              draggable={!disabled}
              disabled={disabled}
              aria-label={`Reorder ${field?.label ?? item.key}. Drag or use Alt plus arrow keys.`}
              on:dragstart={() => handleDragStart(index)}
              on:dragenter={() => handleDragEnter(index)}
              on:dragover|preventDefault
              on:drop|preventDefault={() => handleDrop(index)}
              on:dragend={clearDragState}
              on:keydown={(e) => {
                if (e.altKey && e.key === "ArrowUp" && index > 0) { e.preventDefault(); moveField(index, -1); }
                if (e.altKey && e.key === "ArrowDown" && index < effectiveValue.length - 1) { e.preventDefault(); moveField(index, 1); }
              }}
            >
              ⠿
            </button>
            <span class="poodle-order-by__item-label">{field?.label ?? item.key}</span>
            <IconButton
              icon={item.direction === "asc" ? "arrow-up" : "arrow-down"}
              ariaLabel={`${field?.label ?? item.key}: ${item.direction === "asc" ? "ascending" : "descending"}. Click to toggle.`}
              tooltip={item.direction === "asc" ? "Asc" : "Desc"}
              size="xs"
              variant="ghost"
              disabled={disabled}
              on:click={() => toggleDirection(index)}
            />
            <IconButton
              icon="x"
              ariaLabel={`Remove ${field?.label ?? item.key}`}
              tooltip="Remove"
              size="xs"
              variant="ghost"
              disabled={disabled}
              on:click={() => removeField(index)}
            />
          </div>
        {/each}
      </div>
    {:else}
      <p class="poodle-order-by__empty">No sort fields</p>
    {/if}

    {#if canAddMore && availableFields.length > 0}
      <div class="poodle-order-by__add">
        <Select
          options={selectItems}
          bind:value={addFieldValue}
          placeholder="+ Add field"
          ariaLabel="Add sort field"
          size={resolvedSize}
          density={resolvedDensity}
          onValueChange={addField}
          disabled={disabled}
        />
      </div>
    {/if}
  </div>
</Popover>

<style>
  .poodle-order-by {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    width: 100%;
    min-width: 0;
  }

  .poodle-order-by[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-order-by__trigger {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    min-width: 0;
    width: 100%;
    max-width: 100%;
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    box-sizing: border-box;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    text-align: left;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-order-by__trigger:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated));
  }

  .poodle-order-by__trigger:focus-visible,
  .poodle-order-by__reset:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-order-by__label {
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: var(--poodle-typography-label-weight);
    color: var(--poodle-color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    white-space: nowrap;
  }

  .poodle-order-by__summary {
    flex: 1;
    min-width: 0;
    font-size: 0.875rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .poodle-order-by__summary[data-placeholder="true"] {
    color: var(--poodle-color-text-muted);
  }

  .poodle-order-by__chevron {
    color: var(--poodle-color-text-secondary);
  }

  .poodle-order-by__reset {
    display: inline-flex;
    flex-shrink: 0;
    padding: 0;
    border: 0;
    background: transparent;
    cursor: pointer;
  }

  .poodle-order-by[data-compact="true"] .poodle-order-by__label {
    display: none;
  }

  .poodle-order-by__panel {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    /* Tighten popover surface padding */
    margin: calc(-0.5 * var(--poodle-space-panel-y)) calc(-0.5 * var(--poodle-space-panel-x));
    padding: 0.375rem;
  }

  .poodle-order-by__list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .poodle-order-by__item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.3125rem 0.5rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: calc(var(--poodle-radius-control) - 0.0625rem);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 90%, var(--poodle-color-background-elevated));
  }

  .poodle-order-by__item:hover {
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 60%, transparent);
  }

  .poodle-order-by__item:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: -0.0625rem;
  }

  .poodle-order-by__item--dragging {
    opacity: 0.65;
  }

  .poodle-order-by__item--drop-target {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 8%, transparent);
    box-shadow: inset 0.125rem 0 0 var(--poodle-color-accent-base);
  }

  .poodle-order-by__item-label {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-primary);
  }

  .poodle-order-by__drag-handle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.5rem;
    min-height: 1.5rem;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-muted);
    cursor: grab;
    user-select: none;
    font-size: 0.75rem;
    flex-shrink: 0;
  }

  .poodle-order-by__drag-handle:disabled {
    cursor: not-allowed;
  }

  .poodle-order-by__empty {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    margin: 0;
    padding: 0.25rem 0;
  }

  .poodle-order-by__add {
    display: flex;
    align-items: center;
  }

  .poodle-order-by[data-size="xs"] .poodle-order-by__trigger { min-height: 1.5rem; padding: 0 0.5rem; }
  .poodle-order-by[data-size="xs"] .poodle-order-by__label { font-size: 0.5625rem; }
  .poodle-order-by[data-size="xs"] .poodle-order-by__summary { font-size: 0.6875rem; }

  .poodle-order-by[data-size="sm"] .poodle-order-by__trigger { min-height: 1.75rem; }
  .poodle-order-by[data-size="sm"] .poodle-order-by__label { font-size: 0.625rem; }
  .poodle-order-by[data-size="sm"] .poodle-order-by__summary { font-size: 0.8125rem; }

  .poodle-order-by[data-size="lg"] .poodle-order-by__trigger { min-height: 2.75rem; padding: 0 1rem; }
  .poodle-order-by[data-size="lg"] .poodle-order-by__label { font-size: 0.8125rem; }
  .poodle-order-by[data-size="lg"] .poodle-order-by__summary { font-size: 0.9375rem; }

  .poodle-order-by[data-size="xl"] .poodle-order-by__trigger { min-height: 3.25rem; padding: 0 1.125rem; }
  .poodle-order-by[data-size="xl"] .poodle-order-by__label { font-size: 0.875rem; }
  .poodle-order-by[data-size="xl"] .poodle-order-by__summary { font-size: 1rem; }

  .poodle-order-by[data-density="compact"] .poodle-order-by__trigger {
    gap: 0.375rem;
  }

  .poodle-order-by[data-density="comfortable"] .poodle-order-by__trigger {
    gap: 0.625rem;
  }
</style>
