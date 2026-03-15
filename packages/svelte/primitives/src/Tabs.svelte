<script context="module" lang="ts">
  let nextTabsId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, tick } from "svelte";

  import Icon from "./Icon.svelte";
  import { findNextEnabledIndex, firstEnabledIndex } from "./internal";

  import type { Orientation, TabActivationMode, TabItem, TabVariant } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let items: TabItem[] = [];
  export let variant: TabVariant = "underline";
  export let orientation: Orientation = "horizontal";
  export let activationMode: TabActivationMode = "automatic";
  export let isReorderable = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    reorder: { items: string[] };
    close: { value: string };
  }>();

  const tabsId = ++nextTabsId;
  let tabElements: Array<HTMLButtonElement | null> = [];
  let uncontrolledValue = defaultValue;
  let focusIndex = 0;
  let renderedItems: TabItem[] = items;
  let prevItems = items;

  $: if (items !== prevItems) {
    prevItems = items;
    renderedItems = items;
  }
  $: isControlled = value !== null;
  $: currentValue =
    (isControlled ? value : uncontrolledValue) ??
    renderedItems[firstEnabledIndex(renderedItems)]?.value ??
    null;
  $: selectedIndex = renderedItems.findIndex((item) => item.value === currentValue);
  $: if (selectedIndex >= 0) {
    focusIndex = selectedIndex;
  }
  $: hasPanel = $$slots.default;

  function setValue(nextValue: string): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function moveFocus(nextIndex: number): void {
    focusIndex = nextIndex;
    tabElements[nextIndex]?.focus();

    if (activationMode === "automatic") {
      const nextValue = renderedItems[nextIndex]?.value;

      if (nextValue) {
        setValue(nextValue);
      }
    }
  }

  // ── Reorder (keyboard + drag-and-drop) ──

  let dragSourceIndex: number | null = null;
  let dropTargetIndex: number | null = null;

  function applyReorder(fromIndex: number, toIndex: number): void {
    if (fromIndex === toIndex) return;

    const nextItems = [...renderedItems];
    const [moved] = nextItems.splice(fromIndex, 1);
    nextItems.splice(toIndex, 0, moved);
    renderedItems = nextItems;
    focusIndex = toIndex;

    tick().then(() => tabElements[toIndex]?.focus());
    dispatch("reorder", { items: nextItems.map((item) => item.value) });
  }

  function requestReorder(index: number, direction: -1 | 1): void {
    if (!isReorderable) return;

    const nextIndex = index + direction;

    if (nextIndex < 0 || nextIndex >= renderedItems.length) return;

    applyReorder(index, nextIndex);
  }

  function handleDragStart(event: DragEvent, index: number): void {
    if (!isReorderable) return;

    dragSourceIndex = index;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(index));
    }
  }

  function handleDragOver(event: DragEvent, index: number): void {
    if (dragSourceIndex === null) return;

    event.preventDefault();
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = "move";
    }
    dropTargetIndex = index;
  }

  function handleDragLeave(): void {
    dropTargetIndex = null;
  }

  function handleDrop(event: DragEvent, index: number): void {
    event.preventDefault();

    if (dragSourceIndex !== null) {
      applyReorder(dragSourceIndex, index);
    }

    dragSourceIndex = null;
    dropTargetIndex = null;
  }

  function handleDragEnd(): void {
    dragSourceIndex = null;
    dropTargetIndex = null;
  }

  function handleKeydown(event: KeyboardEvent, index: number): void {
    const horizontal = orientation === "horizontal";

    if (
      (horizontal && event.key === "ArrowRight") ||
      (!horizontal && event.key === "ArrowDown")
    ) {
      if (isReorderable && event.altKey) {
        event.preventDefault();
        requestReorder(index, 1);
      } else {
        event.preventDefault();
        moveFocus(findNextEnabledIndex(renderedItems, index, 1));
      }
      return;
    }

    if (
      (horizontal && event.key === "ArrowLeft") ||
      (!horizontal && event.key === "ArrowUp")
    ) {
      if (isReorderable && event.altKey) {
        event.preventDefault();
        requestReorder(index, -1);
      } else {
        event.preventDefault();
        moveFocus(findNextEnabledIndex(renderedItems, index, -1));
      }
      return;
    }

    if (event.key === "Home") {
      event.preventDefault();
      moveFocus(firstEnabledIndex(renderedItems));
      return;
    }

    if (event.key === "End") {
      event.preventDefault();
      moveFocus(findNextEnabledIndex(renderedItems, 0, -1));
      return;
    }

    if (
      activationMode === "manual" &&
      (event.key === "Enter" || event.key === " ")
    ) {
      event.preventDefault();
      const nextValue = renderedItems[index]?.value;

      if (nextValue) {
        setValue(nextValue);
      }
      return;
    }

    if (event.key === "Delete" && renderedItems[index]?.isClosable) {
      event.preventDefault();
      dispatch("close", { value: renderedItems[index].value });
    }
  }
</script>

<div class="pug-tabs" data-variant={variant} data-orientation={orientation}>
  <div
    class="pug-tabs__list"
    role="tablist"
    aria-label={ariaLabel ?? undefined}
    aria-orientation={orientation}
  >
    {#each renderedItems as item, index (item.value)}
      <div
        class="pug-tabs__item"
        data-selected={currentValue === item.value}
        data-drag-source={dragSourceIndex === index || undefined}
        data-drop-target={dropTargetIndex === index && dropTargetIndex !== dragSourceIndex || undefined}
        draggable={isReorderable && !item.isDisabled}
        on:dragstart={(e) => handleDragStart(e, index)}
        on:dragover={(e) => handleDragOver(e, index)}
        on:dragleave={handleDragLeave}
        on:drop={(e) => handleDrop(e, index)}
        on:dragend={handleDragEnd}
      >
        <button
          bind:this={tabElements[index]}
          type="button"
          class="pug-tabs__tab"
          disabled={item.isDisabled === true}
          id={`pug-tab-${tabsId}-${item.value}`}
          role="tab"
          tabindex={focusIndex === index ? 0 : -1}
          aria-selected={currentValue === item.value ? "true" : "false"}
          aria-controls={hasPanel ? `pug-tabpanel-${tabsId}-${item.value}` : undefined}
          on:focus={() => (focusIndex = index)}
          on:click={() => setValue(item.value)}
          on:keydown={(event) => handleKeydown(event, index)}
        >
          {#if item.icon}
            <Icon name={item.icon} size="sm" />
          {/if}
          <span class="pug-tabs__label">{item.label}</span>
        </button>

        {#if item.isClosable}
          <button
            type="button"
            class="pug-tabs__close"
            aria-label={`Close ${item.label}`}
            on:click|stopPropagation={() => dispatch("close", { value: item.value })}
          >
            <Icon name="x" size="sm" />
          </button>
        {/if}
      </div>
    {/each}

    {#if $$slots.actions}
      <div class="pug-tabs__actions">
        <slot name="actions" />
      </div>
    {/if}
  </div>

  {#if hasPanel && currentValue}
    <div
      class="pug-tabs__panel"
      id={`pug-tabpanel-${tabsId}-${currentValue}`}
      role="tabpanel"
      tabindex="0"
      aria-labelledby={`pug-tab-${tabsId}-${currentValue}`}
    >
      <slot activeValue={currentValue} />
    </div>
  {/if}
</div>

<style>
  /* ── Root ── */

  .pug-tabs {
    display: grid;
    gap: var(--pug-space-stack-md);
    min-width: 0;
  }

  .pug-tabs[data-orientation="vertical"] {
    grid-template-columns: auto minmax(0, 1fr);
    align-items: start;
  }

  /* ── List ── */

  .pug-tabs__list {
    display: inline-flex;
    flex-wrap: wrap;
    align-items: stretch;
    gap: 0.25rem;
  }

  /* Underline: bottom border on list */
  .pug-tabs[data-variant="underline"] .pug-tabs__list {
    padding-bottom: 0.25rem;
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent);
  }

  .pug-tabs[data-variant="underline"][data-orientation="vertical"] .pug-tabs__list {
    flex-direction: column;
    padding-bottom: 0;
    padding-right: 0.5rem;
    border-bottom: 0;
    border-right: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent);
  }

  /* Card + Pill: no wrapping, allow scroll */
  .pug-tabs[data-variant="card"] .pug-tabs__list,
  .pug-tabs[data-variant="pill"] .pug-tabs__list {
    flex-wrap: nowrap;
    overflow: auto;
  }

  .pug-tabs[data-variant="card"][data-orientation="vertical"] .pug-tabs__list,
  .pug-tabs[data-variant="pill"][data-orientation="vertical"] .pug-tabs__list {
    flex-direction: column;
  }

  /* ── Item wrapper (for tab + close) ── */

  .pug-tabs__item {
    display: inline-flex;
    align-items: center;
    min-width: 0;
  }

  /* Card variant: bordered card items */
  .pug-tabs[data-variant="card"] .pug-tabs__item {
    gap: 0;
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 68%, transparent);
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 92%, transparent);
  }

  .pug-tabs[data-variant="card"] .pug-tabs__item[data-selected="true"] {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 32%, var(--pug-color-border-subtle));
    background: color-mix(in srgb, var(--pug-color-accent-base) 14%, var(--pug-color-background-surface));
  }

  /* Drag-and-drop states */
  .pug-tabs__item[draggable="true"] {
    cursor: grab;
  }

  .pug-tabs__item[data-drag-source] {
    opacity: 0.4;
  }

  .pug-tabs__item[data-drop-target] {
    box-shadow: inset 0 0 0 0.125rem var(--pug-color-accent-base);
    border-radius: var(--pug-radius-control);
  }

  /* ── Tab button ── */

  .pug-tabs__tab {
    display: inline-flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
    min-height: calc(var(--pug-size-control-height) - 0.25rem);
    padding: 0 0.75rem;
    border: 0;
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
    white-space: nowrap;
  }

  /* Underline variant: pill-shaped highlight on selected */
  .pug-tabs[data-variant="underline"] .pug-tabs__tab {
    border-radius: var(--pug-radius-control);
  }

  .pug-tabs[data-variant="underline"] .pug-tabs__item[data-selected="true"] .pug-tabs__tab {
    background: color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
    color: var(--pug-color-text-primary);
  }

  /* Card variant: transparent tab inside card */
  .pug-tabs[data-variant="card"] .pug-tabs__tab {
    min-height: calc(var(--pug-size-control-height) - 0.75rem);
    padding: 0 0.625rem;
    color: var(--pug-color-text-primary);
  }

  /* Pill variant: track container around group */
  .pug-tabs[data-variant="pill"] .pug-tabs__list {
    width: fit-content;
    padding: 0.1875rem;
    border: 0.125rem solid color-mix(in srgb, var(--pug-color-border-subtle) 68%, transparent);
    border-radius: 999px;
    gap: 0.125rem;
  }

  .pug-tabs[data-variant="pill"] .pug-tabs__tab {
    min-height: calc(var(--pug-size-control-height) - 0.5rem);
    padding: 0 0.625rem;
    border-radius: 999px;
  }

  .pug-tabs[data-variant="pill"] .pug-tabs__item[data-selected="true"] .pug-tabs__tab {
    background: color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
    color: var(--pug-color-text-primary);
  }

  /* Focus */
  .pug-tabs__tab:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  /* Disabled */
  .pug-tabs__tab:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  /* ── Label ── */

  .pug-tabs__label {
    min-width: 0;
    white-space: nowrap;
  }

  /* ── Close button ── */

  .pug-tabs__close {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    padding: 0;
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
  }

  .pug-tabs__close:hover {
    background: color-mix(in srgb, var(--pug-color-background-surface) 84%, transparent);
    color: var(--pug-color-text-primary);
  }

  .pug-tabs__close:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  /* ── Actions slot ── */

  .pug-tabs__actions {
    display: inline-flex;
    align-items: center;
    margin-left: auto;
  }

  /* ── Panel ── */

  .pug-tabs__panel {
    min-width: 0;
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--pug-color-border-subtle) 74%, transparent);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 96%, transparent);
  }
</style>
