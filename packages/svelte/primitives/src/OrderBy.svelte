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

<Popover bind:open placement="bottom-start" ariaLabel={ariaLabel}>
  <div
    slot="trigger"
    class="order-by"
    role="group"
    aria-label={ariaLabel}
    data-disabled={disabled}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    <button
      type="button"
      class="order-by__trigger"
      disabled={disabled}
      aria-label={ariaLabel}
    >
      <span class="order-by__label">Sort by</span>
      <span class="order-by__summary" data-placeholder={effectiveValue.length === 0}>
        {triggerText}
      </span>
      <span class="order-by__chevron" aria-hidden="true">▾</span>
    </button>

    {#if effectiveValue.length > 0}
      <button
        type="button"
        class="order-by__reset"
        on:click|stopPropagation|preventDefault={clearAll}
        disabled={disabled}
        aria-label="Clear sort"
      >
        ×
      </button>
    {/if}
  </div>

  <div class="order-by__panel">
    {#if effectiveValue.length > 0}
      <div class="order-by__list" role="list">
        {#each effectiveValue as item, index (`${item.key}-${index}`)}
          {@const field = fieldMap.get(item.key)}
          <div
            class="order-by__item"
            class:order-by__item--dragging={dragIndex === index}
            class:order-by__item--drop-target={dragOverIndex === index && dragIndex !== index}
            role="listitem"
            draggable={!disabled}
            on:dragstart={() => handleDragStart(index)}
            on:dragenter={() => handleDragEnter(index)}
            on:dragover|preventDefault
            on:drop|preventDefault={() => handleDrop(index)}
            on:dragend={clearDragState}
          >
            <div class="order-by__item-main">
              <span class="order-by__item-label">
                <span class="order-by__drag-handle" aria-hidden="true">⠿</span>
                {field?.label ?? item.key}
              </span>
              <span class="order-by__item-direction">{item.direction === "asc" ? "Ascending" : "Descending"}</span>
            </div>
            <div class="order-by__item-actions">
              <IconButton
                icon={item.direction === "asc" ? "arrow-up" : "arrow-down"}
                ariaLabel={`Toggle ${field?.label ?? item.key} direction`}
                tooltip={item.direction === "asc" ? "Ascending" : "Descending"}
                size="sm"
                variant="ghost"
                disabled={disabled}
                on:click={() => toggleDirection(index)}
              />
              <IconButton
                icon="chevron-up"
                ariaLabel={`Move ${field?.label ?? item.key} earlier`}
                tooltip="Move up"
                size="sm"
                variant="ghost"
                disabled={disabled || index === 0}
                on:click={() => moveField(index, -1)}
              />
              <IconButton
                icon="chevron-down"
                ariaLabel={`Move ${field?.label ?? item.key} later`}
                tooltip="Move down"
                size="sm"
                variant="ghost"
                disabled={disabled || index === effectiveValue.length - 1}
                on:click={() => moveField(index, 1)}
              />
              <IconButton
                icon="x"
                ariaLabel={`Remove ${field?.label ?? item.key} from sort`}
                tooltip="Remove"
                size="sm"
                variant="ghost"
                tone="danger"
                disabled={disabled}
                on:click={() => removeField(index)}
              />
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <p class="order-by__empty">No sort fields selected</p>
    {/if}

    {#if canAddMore && availableFields.length > 0}
      <div class="order-by__add">
        <Select
          items={selectItems}
          bind:value={addFieldValue}
          placeholder="+ Add field"
          ariaLabel="Add sort field"
          size={resolvedSize}
          density={resolvedDensity}
          onchange={addField}
          disabled={disabled}
        />
      </div>
    {/if}

    {#if effectiveValue.length > 1}
      <div class="order-by__footer">
        <Button variant="ghost" tone="default" size="sm" on:click={clearAll} disabled={disabled}>
          Clear all
        </Button>
      </div>
    {/if}
  </div>
</Popover>

<style>
  .order-by {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
  }

  .order-by[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .order-by__trigger {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 12rem;
    max-width: min(28rem, 75vw);
    min-height: 2rem;
    padding: 0 0.75rem;
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

  .order-by__trigger:hover:not(:disabled) {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 84%, var(--poodle-color-background-elevated));
  }

  .order-by__trigger:focus-visible,
  .order-by__reset:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .order-by__label {
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: var(--poodle-typography-label-weight);
    color: var(--poodle-color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    white-space: nowrap;
  }

  .order-by__summary {
    flex: 1;
    min-width: 0;
    font-size: 0.875rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .order-by__summary[data-placeholder="true"] {
    color: var(--poodle-color-text-muted);
  }

  .order-by__chevron {
    color: var(--poodle-color-text-secondary);
  }

  .order-by__reset {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.75rem;
    height: 1.75rem;
    border: 0;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
  }

  .order-by__reset:hover:not(:disabled) {
    color: var(--poodle-color-text-primary);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 68%, var(--poodle-color-background-elevated));
  }

  .order-by__panel {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .order-by__list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .order-by__item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 88%, var(--poodle-color-background-elevated));
  }

  .order-by__item--dragging {
    opacity: 0.65;
  }

  .order-by__item--drop-target {
    border-color: var(--poodle-color-accent-base);
    box-shadow: 0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent);
  }

  .order-by__item-main {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-width: 0;
  }

  .order-by__item-label {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    font-weight: var(--poodle-typography-body-strong-weight, 600);
  }

  .order-by__drag-handle {
    color: var(--poodle-color-text-muted);
    cursor: grab;
    user-select: none;
  }

  .order-by__item-direction,
  .order-by__empty {
    color: var(--poodle-color-text-secondary);
    font-size: 0.875rem;
  }

  .order-by__item-actions {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    flex-shrink: 0;
  }

  .order-by__add,
  .order-by__footer {
    display: flex;
    align-items: center;
    justify-content: flex-start;
  }

  .order-by[data-size="xs"] .order-by__trigger {
    min-height: 1.625rem;
    padding: 0 0.5rem;
  }

  .order-by[data-size="xs"] .order-by__label {
    font-size: 0.625rem;
  }

  .order-by[data-size="sm"] .order-by__trigger {
    min-height: 1.75rem;
  }

  .order-by[data-size="lg"] .order-by__trigger {
    min-height: 2.25rem;
  }

  .order-by[data-size="xl"] .order-by__trigger {
    min-height: 2.5rem;
  }

  .order-by[data-density="compact"] .order-by__trigger {
    gap: 0.375rem;
  }

  .order-by[data-density="comfortable"] .order-by__trigger {
    gap: 0.625rem;
  }
</style>
