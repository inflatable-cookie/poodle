<script module lang="ts">
  let nextOrderById = 0;
</script>

<script lang="ts">
  import { onMount, tick } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Select } from "./Select.svelte";
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

  interface Props {
    fields?: SortField[];
    value?: OrderByValue | undefined;
    activeSort?: ActiveSort | null | undefined;
    ariaLabel?: string;
    disabled?: boolean;
    sizeRole?: SemanticControlSizeRole;
    size?: ControlSize | null;
    density?: ControlDensity | null;
    maxFields?: number | null;
    compact?: boolean;
    showClearButton?: boolean;
    onChange?: ((value: OrderByValue) => void) | null;
  }

  let {
    fields = [],
    value = $bindable<OrderByValue | undefined>(undefined),
    activeSort = $bindable<ActiveSort | null | undefined>(undefined),
    ariaLabel = "Sort by",
    disabled = false,
    sizeRole = "control",
    size = null,
    density = null,
    maxFields = null,
    compact = false,
    showClearButton = true,
    onChange = null,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const panelId = `poodle-order-by-${++nextOrderById}`;
  let open = $state(false);
  let addFieldValue = $state("");
  let dragIndex = $state<number | null>(null);
  let dragOverIndex = $state<number | null>(null);
  let rootElement = $state<HTMLDivElement | null>(null);
  let panelElement = $state<HTMLDivElement | null>(null);
  let uncontrolledValue = $state<OrderByValue>([]);
  let seededUncontrolledValue = $state(false);

  $effect.pre(() => {
    if (seededUncontrolledValue || value !== undefined || activeSort !== undefined) {
      seededUncontrolledValue = true;
      return;
    }

    uncontrolledValue = [];
    seededUncontrolledValue = true;
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const normalizedFields = $derived(
    fields
      .map<OrderByFieldDefinition>((field) => ({
        key: field.key ?? field.value ?? "",
        label: field.label,
        disabled: field.disabled,
        defaultDirection: field.defaultDirection ?? "asc",
      }))
      .filter((field) => field.key.length > 0),
  );
  const fieldMap = $derived(new Map(normalizedFields.map((field) => [field.key, field])));
  const legacyValue = $derived(
    activeSort ? [{ key: activeSort.field, direction: activeSort.direction }] : [],
  );
  const hasValueProp = $derived(value !== undefined);
  const hasLegacyProp = $derived(activeSort !== undefined);
  const effectiveValue = $derived(
    hasValueProp ? value ?? [] : hasLegacyProp ? legacyValue : uncontrolledValue,
  );
  const canAddMore = $derived(maxFields === null || effectiveValue.length < maxFields);
  const availableFields = $derived(
    normalizedFields.filter((field) => !effectiveValue.some((item) => item.key === field.key)),
  );
  const selectItems = $derived(availableFields.map((field) => ({ value: field.key, label: field.label })));
  const triggerText = $derived(summarizeValue(effectiveValue));

  $effect(() => {
    if (!open) {
      return;
    }

    tick().then(() => {
      const firstFocusable = panelElement?.querySelector<HTMLElement>(
        'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])',
      );
      firstFocusable?.focus();
    });
  });

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
    if (hasValueProp) {
      value = nextValue;
    } else if (hasLegacyProp) {
      activeSort = nextValue.length > 0
        ? { field: nextValue[0].key, direction: nextValue[0].direction }
        : null;
    } else {
      uncontrolledValue = nextValue;
    }

    if (hasValueProp && activeSort !== undefined) {
      activeSort = nextValue.length > 0
        ? { field: nextValue[0].key, direction: nextValue[0].direction }
        : null;
    }

    onChange?.(nextValue);
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

    sync(
      effectiveValue.map((item, itemIndex) => {
        if (itemIndex !== index) return item;
        return {
          ...item,
          direction: item.direction === "asc" ? "desc" : "asc",
        };
      }),
    );
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

  function setOpen(nextOpen: boolean): void {
    if (disabled) return;
    open = nextOpen;
  }

  function toggleOpen(): void {
    setOpen(!open);
  }

  function handleResetClick(event: MouseEvent): void {
    event.preventDefault();
    event.stopPropagation();
    clearAll();
  }

  function handleDragOver(event: DragEvent): void {
    event.preventDefault();
  }

  function handleDropEvent(event: DragEvent, index: number): void {
    event.preventDefault();
    handleDrop(index);
  }

  onMount(() => {
    function handlePointerDown(event: MouseEvent): void {
      if (!open || !rootElement) {
        return;
      }

      if (!rootElement.contains(event.target as Node)) {
        open = false;
      }
    }

    function handleKeydown(event: KeyboardEvent): void {
      if (event.key === "Escape" && open) {
        event.preventDefault();
        open = false;
      }
    }

    document.addEventListener("mousedown", handlePointerDown);
    document.addEventListener("keydown", handleKeydown);

    return () => {
      document.removeEventListener("mousedown", handlePointerDown);
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<div
  bind:this={rootElement}
  class="poodle-order-by-popover"
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <div
    class="poodle-order-by"
    role="group"
    aria-label={ariaLabel}
    data-disabled={disabled}
    data-compact={compact}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    <div
      class="poodle-order-by__trigger-wrap"
    >
      <button
        type="button"
        class="poodle-order-by__trigger"
        disabled={disabled}
        aria-label={ariaLabel}
        aria-expanded={open ? "true" : "false"}
        aria-controls={open ? panelId : undefined}
        onclick={toggleOpen}
      >
        <span class="poodle-order-by__label">Sort by</span>
        <span class="poodle-order-by__summary" data-placeholder={effectiveValue.length === 0}>
          {triggerText}
        </span>
        <span class="poodle-order-by__chevron" aria-hidden="true">▾</span>
      </button>
    </div>

    {#if showClearButton && effectiveValue.length > 0}
      <span class="poodle-order-by__reset">
        <IconButton
          icon="x"
          ariaLabel="Clear sort"
          variant="ghost"
          size={resolvedSize}
          disabled={disabled}
          onClick={handleResetClick}
        />
      </span>
    {/if}
  </div>

  {#if open}
    <div
      bind:this={panelElement}
      id={panelId}
      class="poodle-order-by__surface"
      role="dialog"
      aria-label={ariaLabel}
      tabindex="-1"
    >
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
                  ondragstart={() => handleDragStart(index)}
                  ondragenter={() => handleDragEnter(index)}
                  ondragover={handleDragOver}
                  ondrop={(event) => handleDropEvent(event, index)}
                  ondragend={clearDragState}
                  onkeydown={(event) => {
                    if (event.altKey && event.key === "ArrowUp" && index > 0) {
                      event.preventDefault();
                      moveField(index, -1);
                    }
                    if (event.altKey && event.key === "ArrowDown" && index < effectiveValue.length - 1) {
                      event.preventDefault();
                      moveField(index, 1);
                    }
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
                  onClick={() => toggleDirection(index)}
                />
                <IconButton
                  icon="x"
                  ariaLabel={`Remove ${field?.label ?? item.key}`}
                  tooltip="Remove"
                  size="xs"
                  variant="ghost"
                  disabled={disabled}
                  onClick={() => removeField(index)}
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
    </div>
  {/if}
</div>

<style>
  .poodle-order-by-popover {
    position: relative;
    display: flex;
    width: 100%;
    min-width: 0;
  }

  .poodle-order-by {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    width: 100%;
    min-width: 0;
  }

  .poodle-order-by__trigger-wrap {
    display: flex;
    flex: 1;
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
    user-select: none;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-order-by__trigger:hover {
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
  }

  .poodle-order-by__surface {
    position: absolute;
    z-index: var(--poodle-overlay-z-menu);
    top: calc(100% + 0.5rem);
    left: 0;
    min-width: 14rem;
    max-width: min(24rem, 90vw);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)
    );
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface));
    background: var(--poodle-color-background-elevated);
    --poodle-surface: var(--poodle-color-background-elevated);
    box-shadow:
      inset 0 0.0625rem 0 rgba(255, 255, 255, 0.08),
      0 0.625rem 1.5rem rgba(9, 13, 18, 0.22),
      0 0.125rem 0.375rem rgba(0, 0, 0, 0.15);
  }

  .poodle-order-by[data-compact="true"] .poodle-order-by__label {
    display: none;
  }

  .poodle-order-by__panel {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
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
