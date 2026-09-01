<script module lang="ts">
  let nextOrderById = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/order-by.css";
  import {
    createDragDropController,
    layerContains,
    registerDismissLayer,
    type DragDropCommitResult,
    type DragSourceRegistration,
    type DropIntent,
    type DropPosition,
    type DropTargetRegistration,
  } from "@inflatable-cookie/poodle-core";
  import { tick } from "svelte";

  import { anchored } from "./anchored";
  import { default as Button } from "./Button.svelte";
  import { default as DragDropProvider } from "./DragDropProvider.svelte";
  import { default as IconButton } from "./IconButton.svelte";
  import { default as Select } from "./Select.svelte";
  import {
    dragDropSnapshotStore,
    dragSourceAction,
    dropTargetAction,
    tryDragDrop,
  } from "./drag-drop-context";
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
    dismissOnOutsideInteract?: boolean;
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
    dismissOnOutsideInteract = true,
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

  /**
   * Join the nearest provider, or own a controller.
   *
   * The sort panel is portalled out of the trigger's subtree, so an owned
   * provider wraps the *panel* rather than the root: a controller connected to
   * the root would never see a pointer press on a row.
   */
  const ambient = tryDragDrop();
  const ownDragController = ambient ? undefined : createDragDropController();
  const dragController = ambient?.controller ?? ownDragController!;
  const dragSource = dragSourceAction(dragController);
  const dropTarget = dropTargetAction(dragController);
  const dragSnapshot = dragDropSnapshotStore(dragController);

  /**
   * The registration namespace and the semantic family are both scoped to this
   * builder: two mounted OrderBys can legitimately sort the same field keys,
   * and under one ambient provider neither duplicate ids nor a cross-instance
   * drop are acceptable.
   */
  const subjectKind = `poodle.reorder-item:order-by:${panelId}`;
  const registrationScope = `order-by:${panelId}`;

  function sourceIdOf(key: string): string {
    return `${registrationScope}:source:${key}`;
  }

  function targetIdOf(key: string): string {
    return `${registrationScope}:target:${key}`;
  }

  function keyOfTargetId(targetId: string): string {
    const prefix = `${registrationScope}:target:`;
    return targetId.startsWith(prefix) ? targetId.slice(prefix.length) : "";
  }

  function indexOfKey(key: string): number {
    return effectiveValue.findIndex((item) => item.key === key);
  }

  function ownsKey(key: string): boolean {
    return indexOfKey(key) >= 0;
  }

  function sourceRegistration(key: string, label: string): DragSourceRegistration {
    return {
      sourceId: sourceIdOf(key),
      subject: { kind: subjectKind, id: key },
      allowedOperations: ["move"],
      label,
      disabled,
    };
  }

  function targetRegistration(key: string, label: string, index: number): DropTargetRegistration {
    return {
      targetId: targetIdOf(key),
      acceptedKinds: [subjectKind],
      disabled,
      label,
      // The whole row is one band: a field travelling down lands after its
      // target, one travelling up lands before it, so the dropped field ends
      // up *at* the row it was dropped on — the pre-substrate result.
      resolvePosition: ({ subject }): DropPosition =>
        indexOfKey(subject.id) < index ? "after" : "before",
      canDrop: (intent, subject) => {
        if (!ownsKey(subject.id)) {
          return { accepted: false, reason: "not this sort builder" };
        }
        return subject.id === key
          ? { accepted: false, reason: "same field" }
          : { accepted: true, intent };
      },
      onDrop: handleDrop,
    };
  }

  /**
   * One accepted drop, one complete ordering.
   *
   * Both indices are resolved again here rather than trusted from hover: the
   * host may have replaced `value` while the pointer was down, and a stale
   * index would move the wrong field.
   */
  function handleDrop(intent: DropIntent): DragDropCommitResult {
    if (disabled) return { status: "rejected", reason: "disabled" };

    const from = indexOfKey($dragSnapshot.session?.subject.id ?? "");
    const target = indexOfKey(keyOfTargetId(intent.targetId));
    if (from < 0 || target < 0 || from === target) {
      return { status: "rejected", reason: "missing field" };
    }

    const to =
      intent.position === "before"
        ? from < target
          ? target - 1
          : target
        : from < target
          ? target
          : target + 1;

    const nextValue = [...effectiveValue];
    const [item] = nextValue.splice(from, 1);
    nextValue.splice(to, 0, item);
    sync(nextValue);
    return { status: "committed" };
  }

  /**
   * Alt+Arrow: the contract's keyboard reorder, run as a real session so it
   * shares eligibility, revalidation, and the single commit with a drop.
   */
  function moveField(index: number, offset: -1 | 1): void {
    if (disabled) return;

    const from = effectiveValue[index];
    const target = effectiveValue[index + offset];
    if (!from || !target) return;

    dragController.requestKeyboardDrop({
      sourceId: sourceIdOf(from.key),
      targetId: targetIdOf(target.key),
      position: offset === 1 ? "after" : "before",
    });
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

  $effect(() => {
    if (!open) {
      return;
    }

    return registerDismissLayer({
      // The surface is portalled out of the root, so both are "inside".
      contains: (target) => layerContains(target, rootElement, panelElement),
      dismissOnOutsideInteract,
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
      {#if ambient}
        {@render panel()}
      {:else}
        <DragDropProvider controller={ownDragController}>
          {@render panel()}
        </DragDropProvider>
      {/if}
    </div>
  {/if}
</div>

{#snippet panel()}
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
            {#each effectiveValue as item, index (item.key)}
              {@const field = fieldMap.get(item.key)}
              {@const label = field?.label ?? item.key}
              <div
                class="poodle-order-by__item"
                class:poodle-order-by__item--dragging={$dragSnapshot.sourceId === sourceIdOf(item.key) &&
                  ($dragSnapshot.phase === "dragging" || $dragSnapshot.phase === "dropping")}
                class:poodle-order-by__item--drop-target={$dragSnapshot.targetId === targetIdOf(item.key) &&
                  $dragSnapshot.targetPosture === "accepted"}
                role="listitem"
                use:dropTarget={targetRegistration(item.key, label, index)}
              >
                <button
                  type="button"
                  class="poodle-order-by__drag-handle"
                  disabled={disabled}
                  aria-label={`Reorder ${label}. Drag or use Alt plus arrow keys.`}
                  use:dragSource={sourceRegistration(item.key, label)}
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
{/snippet}
