<script module lang="ts">
  let nextOrderById = 0;
</script>

<script lang="ts">
  import "@poodle/styles/order-by.css";
  import { layerContains, registerDismissLayer } from "@poodle/headless";
  import { tick } from "svelte";

  import { anchored } from "./anchored";
  import { default as Button } from "./Button.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Select } from "./Select.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ActiveSort,
    ControlDensity,
    ControlSize,
    OrderByFieldDefinition,
    OrderByTriggerVariant,
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
    triggerVariant?: OrderByTriggerVariant;
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
    triggerVariant = "summary",
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

  $effect(() => {
    if (!open) {
      return;
    }

    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, panelElement),
      dismissOnOutsideInteract: true,
      onDismiss: () => {
        open = false;
      },
    });
  });
</script>

<div
  bind:this={rootElement}
  class="poodle-order-by-popover"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-trigger-variant={triggerVariant}
>
  <div
    class="poodle-order-by"
    role="group"
    aria-label={ariaLabel}
    data-disabled={disabled}
    data-compact={compact}
    data-trigger-variant={triggerVariant}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    {#if triggerVariant === "icon"}
      <IconButton
        icon="arrow-up-down"
        ariaLabel={ariaLabel}
        tooltip={ariaLabel}
        variant="secondary"
        size={resolvedSize}
        disabled={disabled}
        expanded={open}
        controls={open ? panelId : null}
        onClick={toggleOpen}
      />
    {:else}
      <div class="poodle-order-by__trigger-wrap">
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
    {/if}
  </div>

  {#if open}
    <div
      bind:this={panelElement}
      use:anchored={{ anchor: rootElement, placement: "bottom-start", offset: 8 }}
      id={panelId}
      class="poodle-order-by__surface"
      role="dialog"
      aria-label={ariaLabel}
      tabindex="-1"
    >
      <div class="poodle-order-by__panel">
        {#if triggerVariant === "icon"}
          <div class="poodle-order-by__panel-header">
            <span class="poodle-order-by__panel-title">Sort order</span>
            {#if showClearButton && effectiveValue.length > 0}
              <IconButton
                icon="x"
                ariaLabel="Clear sort"
                tooltip="Clear sort"
                variant="ghost"
                size="xs"
                disabled={disabled}
                onClick={handleResetClick}
              />
            {/if}
          </div>
        {/if}

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
