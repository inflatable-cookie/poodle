<script context="module" lang="ts">
  let nextTabsId = 0;
</script>

<script lang="ts">
  import { createEventDispatcher, onDestroy, onMount, tick } from "svelte";

  import Icon from "./Icon.svelte";
  import Pill from "./Pill.svelte";
  import { findNextEnabledIndex, firstEnabledIndex } from "./internal";
  import {
    applyReorder as applyReorderItems,
    handleDragStart as startDrag,
    handleDragOver as overDrag,
    handleDrop as dropDrag,
  } from "./tabs-reorder";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    Orientation,
    SemanticControlSizeRole,
    TabActivationMode,
    TabItem,
    TabVariant,
  } from "./types";

  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let items: TabItem[] = [];
  export let variant: TabVariant = "underline";
  export let orientation: Orientation = "horizontal";
  export let activationMode: TabActivationMode = "automatic";
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;
  export let reorderable = false;
  export let ariaLabel: string | null = null;
  export let showTooltips = false;
  export let historyKey: string | null = null;

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
  let lastSyncedValue: string | null = null;
  const isBrowser = typeof window !== "undefined";
  const uiPresentation = getUiPresentation();

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
  $: isVertical = orientation === "vertical";
  $: hasTooltips = isVertical || showTooltips;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: resolvedIconSize = resolveSupportingVisualSize(resolvedSize);

  // ── Tooltip (vertical icon-only mode) ──

  let tooltipIndex: number | null = null;
  let tooltipTimer: ReturnType<typeof setTimeout> | null = null;

  function scheduleTooltip(index: number): void {
    clearTooltip();
    tooltipTimer = setTimeout(() => (tooltipIndex = index), 300);
  }

  function dismissTooltip(): void {
    clearTooltip();
    tooltipIndex = null;
  }

  function clearTooltip(): void {
    if (tooltipTimer) {
      clearTimeout(tooltipTimer);
      tooltipTimer = null;
    }
  }

  onDestroy(() => clearTooltip());

  function setValue(nextValue: string): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function replaceUrlTabParam(nextValue: string): void {
    if (!isBrowser || !historyKey) return;
    const url = new URL(window.location.href);
    url.searchParams.set(historyKey, nextValue);
    window.history.replaceState(window.history.state, "", url);
  }

  onMount(() => {
    if (!isBrowser || !historyKey) return;

    const urlValue = new URL(window.location.href).searchParams.get(historyKey);
    if (urlValue) {
      setValue(urlValue);
      lastSyncedValue = urlValue;
    } else if (currentValue) {
      replaceUrlTabParam(currentValue);
      lastSyncedValue = currentValue;
    }

    const handlePopState = () => {
      const nextValue = new URL(window.location.href).searchParams.get(historyKey);
      if (nextValue && nextValue !== currentValue) {
        setValue(nextValue);
        lastSyncedValue = nextValue;
      }
    };

    window.addEventListener("popstate", handlePopState);
    return () => window.removeEventListener("popstate", handlePopState);
  });

  $: if (isBrowser && historyKey && currentValue && currentValue !== lastSyncedValue) {
    replaceUrlTabParam(currentValue);
    lastSyncedValue = currentValue;
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
    const result = applyReorderItems(renderedItems, fromIndex, toIndex);
    renderedItems = result.items;
    focusIndex = result.focusIndex;
    tick().then(() => tabElements[result.focusIndex]?.focus());
    dispatch("reorder", { items: result.items.map((item) => item.value) });
  }

  function requestReorder(index: number, direction: -1 | 1): void {
    if (!reorderable) return;
    const nextIndex = index + direction;
    if (nextIndex < 0 || nextIndex >= renderedItems.length) return;
    applyReorder(index, nextIndex);
  }

  function handleDragStart(event: DragEvent, index: number): void {
    const result = startDrag(event, index, reorderable);
    if (result.dragSourceIndex !== null) {
      dragSourceIndex = result.dragSourceIndex;
    }
  }

  function handleDragOver(event: DragEvent, index: number): void {
    const result = overDrag(event, index, dragSourceIndex);
    if (result.dropTargetIndex !== null) {
      dropTargetIndex = result.dropTargetIndex;
    }
  }

  function handleDragLeave(): void {
    dropTargetIndex = null;
  }

  function handleDrop(event: DragEvent, index: number): void {
    const result = dropDrag(event, index, dragSourceIndex);
    if (result.fromIndex !== null && result.toIndex !== null) {
      applyReorder(result.fromIndex, result.toIndex);
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
      if (reorderable && event.altKey) {
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
      if (reorderable && event.altKey) {
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

    if (event.key === "Delete" && renderedItems[index]?.closable) {
      event.preventDefault();
      dispatch("close", { value: renderedItems[index].value });
    }
  }
</script>

<div
  class="poodle-tabs"
  data-variant={variant}
  data-orientation={orientation}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <div
    class="poodle-tabs__list"
    role="tablist"
    aria-label={ariaLabel ?? undefined}
    aria-orientation={orientation}
  >
    {#each renderedItems as item, index (item.value)}
      <div
        class="poodle-tabs__item"
        role="presentation"
        data-selected={currentValue === item.value}
        data-drag-source={dragSourceIndex === index || undefined}
        data-drop-target={dropTargetIndex === index && dropTargetIndex !== dragSourceIndex || undefined}
        draggable={reorderable && !item.disabled}
        on:dragstart={(e) => handleDragStart(e, index)}
        on:dragover={(e) => handleDragOver(e, index)}
        on:dragleave={handleDragLeave}
        on:drop={(e) => handleDrop(e, index)}
        on:dragend={handleDragEnd}
        on:mouseenter={() => hasTooltips && scheduleTooltip(index)}
        on:mouseleave={() => hasTooltips && dismissTooltip()}
      >
        {#if item.separator}
          <span class="poodle-tabs__separator" aria-hidden="true"></span>
        {/if}
        <button
          bind:this={tabElements[index]}
          type="button"
          class="poodle-tabs__tab"
          disabled={item.disabled === true}
          id={`poodle-tab-${tabsId}-${item.value}`}
          role="tab"
          tabindex={focusIndex === index ? 0 : -1}
          aria-selected={currentValue === item.value ? "true" : "false"}
          aria-controls={hasPanel ? `poodle-tabpanel-${tabsId}-${item.value}` : undefined}
          on:focus={() => { focusIndex = index; if (isVertical) scheduleTooltip(index); }}
          on:blur={() => hasTooltips && dismissTooltip()}
          on:pointerdown={(event) => {
            if (
              reorderable &&
              event.button === 0 &&
              item.disabled !== true &&
              currentValue !== item.value
            ) {
              setValue(item.value);
            }
          }}
          on:click={() => setValue(item.value)}
          on:keydown={(event) => {
            if (event.key === "Escape" && hasTooltips) dismissTooltip();
            handleKeydown(event, index);
          }}
        >
          {#if item.icon}
            <Icon icon={item.icon} size={resolvedIconSize} />
          {/if}
          <span class="poodle-tabs__label">{item.label}</span>
          {#if item.count !== undefined}
            <Pill tone="neutral" appearance="badge" size={resolvedIconSize} muted ariaLabel={`${item.count}`}>
              {item.count}
            </Pill>
          {/if}
        </button>

        {#if item.closable}
          <button
            type="button"
            class="poodle-tabs__close"
            aria-label={`Close ${item.label}`}
            on:click|stopPropagation={() => dispatch("close", { value: item.value })}
          >
            <Icon name="x" size={resolvedIconSize} />
          </button>
        {/if}

        {#if hasTooltips && tooltipIndex === index}
          <span class="poodle-tabs__tooltip" data-placement={isVertical ? "right" : "bottom"} role="tooltip">
            {item.label}
          </span>
        {/if}
      </div>
    {/each}

    {#if $$slots.actions}
      <div class="poodle-tabs__actions">
        <slot name="actions" />
      </div>
    {/if}
  </div>

  {#if hasPanel && currentValue}
    <div
      class="poodle-tabs__panel"
      id={`poodle-tabpanel-${tabsId}-${currentValue}`}
      role="tabpanel"
      tabindex="0"
      aria-labelledby={`poodle-tab-${tabsId}-${currentValue}`}
    >
      <slot activeValue={currentValue} />
    </div>
  {/if}
</div>

<style>
  /* ── Root ── */

  .poodle-tabs {
    --poodle-tabs-control-height: var(--poodle-size-control-height);
    --poodle-tabs-control-x: var(--poodle-space-control-x);
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
  }

  .poodle-tabs[data-size="sm"] {
    --poodle-tabs-control-height: 1.75rem;
  }

  .poodle-tabs[data-size="md"] {
    --poodle-tabs-control-height: 2.25rem;
  }

  .poodle-tabs[data-size="lg"] {
    --poodle-tabs-control-height: 2.75rem;
  }

  .poodle-tabs[data-size="xs"] {
    --poodle-tabs-control-height: 1.5rem;
  }

  .poodle-tabs[data-size="xl"] {
    --poodle-tabs-control-height: 3.25rem;
  }

  .poodle-tabs[data-density="compact"] {
    --poodle-tabs-control-x: 0.5rem;
  }

  .poodle-tabs[data-density="default"] {
    --poodle-tabs-control-x: 0.75rem;
  }

  .poodle-tabs[data-density="comfortable"] {
    --poodle-tabs-control-x: 1rem;
  }

  .poodle-tabs[data-orientation="vertical"] {
    grid-template-columns: auto minmax(0, 1fr);
    align-items: start;
  }

  /* ── List ── */

  .poodle-tabs__list {
    display: inline-flex;
    flex-wrap: wrap;
    align-items: stretch;
    gap: var(--poodle-space-inline-sm);
  }

  /* Underline: bottom border on list */
  .poodle-tabs[data-variant="underline"] .poodle-tabs__list {
    padding-bottom: var(--poodle-space-inline-sm);
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent);
  }

  .poodle-tabs[data-variant="underline"][data-orientation="vertical"] .poodle-tabs__list {
    flex-direction: column;
    padding-bottom: 0;
    padding-right: 0.5rem;
    border-bottom: 0;
    border-right: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent);
  }

  /* Card + Pill + Strip + Block: no wrapping, allow scroll on main axis only */
  .poodle-tabs[data-variant="card"] .poodle-tabs__list,
  .poodle-tabs[data-variant="pill"] .poodle-tabs__list,
  .poodle-tabs[data-variant="strip"] .poodle-tabs__list,
  .poodle-tabs[data-variant="block"] .poodle-tabs__list {
    flex-wrap: nowrap;
    overflow-x: auto;
    overflow-y: hidden;
  }

  .poodle-tabs[data-variant="card"][data-orientation="vertical"] .poodle-tabs__list,
  .poodle-tabs[data-variant="pill"][data-orientation="vertical"] .poodle-tabs__list,
  .poodle-tabs[data-variant="strip"][data-orientation="vertical"] .poodle-tabs__list,
  .poodle-tabs[data-variant="block"][data-orientation="vertical"] .poodle-tabs__list {
    flex-direction: column;
    overflow-x: hidden;
    overflow-y: auto;
  }

  /* Strip: full-width cohesive bar */
  .poodle-tabs[data-variant="strip"] .poodle-tabs__list {
    display: flex;
    gap: 0;
    padding: 0 var(--poodle-space-panel-x, 0.75rem);
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 92%, transparent);
  }

  /* Block: full-width tabs with separators, no radius, no outer border chrome */
  .poodle-tabs[data-variant="block"] .poodle-tabs__list {
    display: flex;
    width: 100%;
    gap: 0;
    padding: 0;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 90%, transparent);
  }

  /* ── Item wrapper (for tab + close) ── */

  .poodle-tabs__item {
    position: relative;
    display: inline-flex;
    align-items: center;
    min-width: 0;
  }

  .poodle-tabs__separator {
    width: 0.0625rem;
    align-self: stretch;
    margin-right: var(--poodle-space-inline-sm);
    background: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  /* Card variant: bordered card items */
  .poodle-tabs[data-variant="card"] .poodle-tabs__item {
    gap: 0;
    border: 0.0625rem solid var(
      --poodle-treatment-interactive-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 68%, transparent)
    );
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
    background: var(
      --poodle-treatment-interactive-fill,
      color-mix(in srgb, var(--poodle-color-background-surface) 92%, transparent)
    );
    box-shadow: var(--poodle-treatment-interactive-shadow, none);
  }

  .poodle-tabs[data-variant="card"] .poodle-tabs__item[data-selected="true"] {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 32%, var(--poodle-color-border-subtle));
    background: color-mix(in srgb, var(--poodle-color-accent-base) 14%, var(--poodle-color-background-surface));
  }

  .poodle-tabs[data-variant="block"] .poodle-tabs__item {
    display: flex;
    flex: 0 0 auto;
    min-width: 0;
  }

  .poodle-tabs[data-variant="block"] .poodle-tabs__close {
    margin-left: -0.25rem;
    margin-right: 0.25rem;
  }

  .poodle-tabs[data-variant="block"] .poodle-tabs__item + .poodle-tabs__item {
    border-left: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  .poodle-tabs[data-variant="block"][data-orientation="vertical"] .poodle-tabs__item + .poodle-tabs__item {
    border-left: 0;
    border-top: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
  }

  /* Drag-and-drop states */
  .poodle-tabs__item[draggable="true"] {
    cursor: grab;
  }

  .poodle-tabs__item[data-drag-source] {
    opacity: 0.4;
  }

  .poodle-tabs__item[data-drop-target] {
    box-shadow: inset 0 0 0 0.125rem var(--poodle-color-accent-base);
    border-radius: var(--poodle-radius-control);
  }

  /* ── Tab button ── */

  .poodle-tabs__tab {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: calc(var(--poodle-tabs-control-height) - 0.25rem);
    padding: 0 var(--poodle-tabs-control-x);
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: 1;
    white-space: nowrap;
  }

  /* Underline variant: pill-shaped highlight on selected */
  .poodle-tabs[data-variant="underline"] .poodle-tabs__tab {
    border-radius: var(--poodle-radius-control);
  }

  .poodle-tabs[data-variant="underline"] .poodle-tabs__item[data-selected="true"] .poodle-tabs__tab {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent);
    color: var(--poodle-color-text-primary);
  }

  /* Card variant: transparent tab inside card */
  .poodle-tabs[data-variant="card"] .poodle-tabs__tab {
    padding: 0 var(--poodle-tabs-control-x);
    color: var(--poodle-color-text-primary);
  }

  .poodle-tabs[data-variant="block"] .poodle-tabs__tab {
    justify-content: center;
    width: auto;
    min-height: var(--poodle-tabs-control-height);
    padding: 0 var(--poodle-tabs-control-x);
    border-radius: 0;
  }

  .poodle-tabs[data-variant="block"] .poodle-tabs__item[data-selected="true"] {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 14%, var(--poodle-color-background-surface));
  }

  .poodle-tabs[data-variant="block"] .poodle-tabs__item[data-selected="true"] .poodle-tabs__tab {
    color: var(--poodle-color-text-primary);
  }

  .poodle-tabs[data-variant="block"] .poodle-tabs__item:not([data-selected="true"]):hover {
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 50%, transparent);
  }

  .poodle-tabs[data-variant="block"] .poodle-tabs__item[data-selected="true"]:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 18%, var(--poodle-color-background-surface));
  }

  /* Strip variant: compact tabs in a bar */
  .poodle-tabs[data-variant="strip"] .poodle-tabs__item {
    border-bottom: 0.125rem solid transparent;
    margin-bottom: -0.0625rem;
  }

  .poodle-tabs[data-variant="strip"] .poodle-tabs__separator {
    margin-right: 0;
    margin-left: var(--poodle-space-inline-sm);
    margin-block: 0.375rem;
  }

  .poodle-tabs[data-orientation="vertical"] .poodle-tabs__separator {
    width: auto;
    height: 0.0625rem;
    margin-right: 0;
    margin-bottom: var(--poodle-space-inline-sm);
    align-self: stretch;
  }

  .poodle-tabs[data-variant="strip"] .poodle-tabs__item[data-selected="true"] {
    border-bottom-color: var(--poodle-color-accent-base);
  }

  .poodle-tabs[data-variant="strip"] .poodle-tabs__item[data-selected="true"] .poodle-tabs__tab {
    color: var(--poodle-color-text-primary);
  }

  .poodle-tabs[data-variant="strip"] .poodle-tabs__tab {
    min-height: var(--poodle-tabs-control-height);
    padding: 0 var(--poodle-tabs-control-x);
    border-radius: 0;
  }

  .poodle-tabs[data-variant="strip"] .poodle-tabs__item:hover {
    background: color-mix(in srgb, var(--poodle-color-surface-hover) 50%, transparent);
  }

  .poodle-tabs[data-variant="strip"] .poodle-tabs__close {
    margin-right: 0.25rem;
  }

  /* Strip vertical: border shifts to right edge, icon-only compact tabs */
  .poodle-tabs[data-variant="strip"][data-orientation="vertical"] .poodle-tabs__list {
    padding: 0;
    border-bottom: 0;
    border-right: 0.0625rem solid var(--poodle-color-border-subtle);
    overflow: visible;
  }

  .poodle-tabs[data-variant="strip"][data-orientation="vertical"] .poodle-tabs__item {
    border-bottom: 0;
    border-right: 0.125rem solid transparent;
    margin-bottom: 0;
    margin-right: -0.125rem;
  }

  .poodle-tabs[data-variant="strip"][data-orientation="vertical"] .poodle-tabs__item[data-selected="true"] {
    border-right-color: var(--poodle-color-accent-base);
  }

  .poodle-tabs[data-variant="strip"][data-orientation="vertical"] .poodle-tabs__tab {
    justify-content: center;
    min-height: 0;
    min-width: var(--poodle-tabs-control-height);
    padding: var(--poodle-tabs-control-x);
  }

  .poodle-tabs[data-variant="strip"][data-orientation="vertical"] .poodle-tabs__item:first-child .poodle-tabs__tab {
    padding-top: 0.75rem;
  }

  .poodle-tabs[data-variant="strip"][data-orientation="vertical"] .poodle-tabs__item:last-child .poodle-tabs__tab {
    padding-bottom: 0.75rem;
  }

  /* Vertical orientation: collapse to icon-only, hide label + close */
  .poodle-tabs[data-orientation="vertical"] .poodle-tabs__label {
    display: none;
  }

  .poodle-tabs[data-orientation="vertical"] .poodle-tabs__close {
    display: none;
  }

  /* Pill variant: track container around group */
  .poodle-tabs[data-variant="pill"] .poodle-tabs__list {
    width: fit-content;
    padding: 0.1875rem;
    border: 0.125rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 68%, transparent);
    border-radius: 999px;
    gap: 0.125rem;
  }

  .poodle-tabs[data-variant="pill"] .poodle-tabs__tab {
    min-height: calc(var(--poodle-tabs-control-height) - 0.5rem);
    padding: 0 var(--poodle-tabs-control-x);
    border-radius: 999px;
  }

  .poodle-tabs[data-variant="pill"] .poodle-tabs__item[data-selected="true"] .poodle-tabs__tab {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent);
    color: var(--poodle-color-text-primary);
  }

  /* Focus */
  .poodle-tabs__tab:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  /* Disabled */
  .poodle-tabs__tab:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* ── Label ── */

  .poodle-tabs__label {
    min-width: 0;
    white-space: nowrap;
  }

  /* ── Close button ── */

  .poodle-tabs__close {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    min-height: 0;
    padding: 0;
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
  }

  .poodle-tabs__close:hover {
    background: color-mix(in srgb, var(--poodle-color-background-surface) 84%, transparent);
    color: var(--poodle-color-text-primary);
  }

  .poodle-tabs__close:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  /* ── Actions slot ── */

  .poodle-tabs__actions {
    display: inline-flex;
    align-items: center;
    margin-left: auto;
  }

  /* ── Tooltip (vertical icon-only mode) ── */

  .poodle-tabs__tooltip {
    position: absolute;
    z-index: var(--poodle-overlay-z-menu);
    max-width: 16rem;
    padding: 0.375rem 0.5rem;
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-default) 72%, transparent);
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel));
    box-shadow: var(--poodle-elevation-overlay);
    color: var(--poodle-color-text-primary);
    font-size: 0.6875rem;
    line-height: 1.35;
    white-space: nowrap;
    pointer-events: none;
  }

  .poodle-tabs__tooltip[data-placement="right"] {
    top: 50%;
    left: calc(100% + 0.375rem);
    transform: translateY(-50%);
  }

  .poodle-tabs__tooltip[data-placement="bottom"] {
    top: calc(100% + 0.375rem);
    left: 50%;
    transform: translateX(-50%);
  }

  /* ── Panel ── */

  .poodle-tabs__panel {
    min-width: 0;
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 96%, transparent);
  }
</style>
