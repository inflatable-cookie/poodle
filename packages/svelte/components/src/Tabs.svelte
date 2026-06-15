<script module lang="ts">
  let nextTabsId = 0;
</script>

<script lang="ts">
  import { onDestroy, onMount, tick, type Snippet } from "svelte";

  import { default as Button } from "./Button.svelte";
  import { default as Icon } from "./Icon.svelte";
  import { default as Menu } from "./Menu.svelte";
  import { default as Pill } from "./Pill.svelte";
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

  interface Props {
    value?: string | null;
    defaultValue?: string | null;
    items?: TabItem[];
    variant?: TabVariant;
    orientation?: Orientation;
    activationMode?: TabActivationMode;
    bordered?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    collapseWhenOverflow?: boolean;
    fullWidth?: boolean;
    collapseLabel?: string | null;
    reorderable?: boolean;
    ariaLabel?: string | null;
    showTooltips?: boolean;
    historyKey?: string | null;
    onValueChange?: ((value: string) => void) | undefined;
    onReorder?: ((items: string[]) => void) | undefined;
    onClose?: ((value: string) => void) | undefined;
    children?: Snippet<[string]>;
    actions?: Snippet<[]>;
  }

  let {
    value = $bindable<string | null>(null),
    defaultValue = null,
    items = [],
    variant = "text",
    orientation = "horizontal",
    activationMode = "automatic",
    bordered = true,
    size = null,
    sizeRole = "chrome",
    density = null,
    collapseWhenOverflow = false,
    fullWidth = false,
    collapseLabel = null,
    reorderable = false,
    ariaLabel = null,
    showTooltips = false,
    historyKey = null,
    onValueChange = undefined,
    onReorder = undefined,
    onClose = undefined,
    children,
    actions,
  }: Props = $props();

  const tabsId = ++nextTabsId;
  const isBrowser = typeof window !== "undefined";
  const uiPresentation = getUiPresentation();
  let tabElements = $state<Array<HTMLButtonElement | null>>([]);
  let rootElement = $state<HTMLDivElement | null>(null);
  let measureListElement = $state<HTMLDivElement | null>(null);
  let uncontrolledValue = $state<string | null>(null);
  let seededDefaultValue = $state(false);
  let focusIndex = $state(0);
  let renderedItems = $state<TabItem[]>([]);
  let lastItemsSignature = $state("");
  let lastSyncedValue = $state<string | null>(null);
  let tooltipIndex = $state<number | null>(null);
  let tooltipTimer = $state<ReturnType<typeof setTimeout> | null>(null);
  let dragSourceIndex = $state<number | null>(null);
  let dropTargetIndex = $state<number | null>(null);
  let collapsedByOverflow = $state(false);
  let historyReady = $state(false);

  function getItemsSignature(nextItems: TabItem[]): string {
    return JSON.stringify(
      nextItems.map((item) => ({
        value: item.value,
        label: item.label,
        icon: item.icon ?? null,
        disabled: item.disabled ?? false,
        closable: item.closable ?? false,
        count: item.count ?? null,
        separator: item.separator ?? false,
      })),
    );
  }

  $effect.pre(() => {
    const itemsSignature = getItemsSignature(items);
    if (itemsSignature === lastItemsSignature) {
      return;
    }

    lastItemsSignature = itemsSignature;
    renderedItems = items;
  });

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
    }
  });

  const isControlled = $derived(value !== null);
  const currentValue = $derived(
    (isControlled ? value : uncontrolledValue) ??
      renderedItems[firstEnabledIndex(renderedItems)]?.value ??
      null,
  );
  const selectedIndex = $derived(renderedItems.findIndex((item) => item.value === currentValue));
  const hasPanel = $derived(children !== undefined);
  const isVertical = $derived(orientation === "vertical");
  const hasTooltips = $derived(isVertical || showTooltips);
  const canCollapse = $derived(collapseWhenOverflow && !isVertical);
  const resolvedVariant = $derived(variant === "underline" ? "text" : variant);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const resolvedIconSize = $derived(resolveSupportingVisualSize(resolvedSize));
  const selectedItem = $derived(renderedItems.find((item) => item.value === currentValue) ?? null);
  const collapseTriggerLabel = $derived(collapseLabel ?? selectedItem?.label ?? "Sections");
  const collapsedMenuItems = $derived(
    renderedItems.map((item) => ({
      value: item.value,
      label: item.count === undefined ? item.label : `${item.label} (${item.count})`,
      disabled: item.disabled,
      kind: "radio" as const,
      checked: item.value === currentValue,
    })),
  );

  $effect(() => {
    if (selectedIndex >= 0) {
      focusIndex = selectedIndex;
    }
  });

  async function evaluateCollapsedOverflow(): Promise<void> {
    if (!canCollapse) {
      collapsedByOverflow = false;
      return;
    }

    await tick();

    if (!rootElement || !measureListElement) {
      return;
    }

    const naturalWidth = measureListElement.getBoundingClientRect().width;
    const availableWidth = rootElement.getBoundingClientRect().width;
    collapsedByOverflow = naturalWidth > availableWidth + 1;
  }

  function handleViewportChange(): void {
    void evaluateCollapsedOverflow();
  }

  // ── Tooltip (vertical icon-only mode) ──

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
    } else {
      value = nextValue;
    }

    onValueChange?.(nextValue);
  }

  function replaceUrlTabParam(nextValue: string): void {
    if (!isBrowser || !historyKey) return;
    const url = new URL(window.location.href);
    const defaultValue = renderedItems[firstEnabledIndex(renderedItems)]?.value ?? null;
    if (defaultValue && nextValue === defaultValue) {
      url.searchParams.delete(historyKey);
    } else {
      url.searchParams.set(historyKey, nextValue);
    }
    window.history.replaceState(window.history.state, "", url);
  }

  onMount(() => {
    const resizeObserver =
      typeof ResizeObserver === "undefined"
        ? null
        : new ResizeObserver(() => {
            void evaluateCollapsedOverflow();
          });

    if (resizeObserver) {
      if (rootElement) resizeObserver.observe(rootElement);
      if (measureListElement) resizeObserver.observe(measureListElement);
    }

    let handlePopState: (() => void) | null = null;
    if (isBrowser && historyKey) {
      const urlValue = new URL(window.location.href).searchParams.get(historyKey);
      if (urlValue) {
        setValue(urlValue);
        lastSyncedValue = urlValue;
      } else if (currentValue) {
        replaceUrlTabParam(currentValue);
        lastSyncedValue = currentValue;
      }

      handlePopState = () => {
        const nextValue = new URL(window.location.href).searchParams.get(historyKey);
        if (nextValue && nextValue !== currentValue) {
          setValue(nextValue);
          lastSyncedValue = nextValue;
          return;
        }

        if (!nextValue) {
          const fallbackValue = renderedItems[firstEnabledIndex(renderedItems)]?.value ?? null;
          if (fallbackValue && fallbackValue !== currentValue) {
            setValue(fallbackValue);
            lastSyncedValue = fallbackValue;
          }
        }
      };

      window.addEventListener("popstate", handlePopState);
    }
    
    void tick().then(() => {
      historyReady = true;
    });

    return () => {
      resizeObserver?.disconnect();
      if (handlePopState) {
        window.removeEventListener("popstate", handlePopState);
      }
    };
  });

  onMount(() => {
    window.addEventListener("resize", handleViewportChange);
    return () => window.removeEventListener("resize", handleViewportChange);
  });

  $effect(() => {
    if (!historyReady) return;
    if (isBrowser && historyKey && currentValue && currentValue !== lastSyncedValue) {
      replaceUrlTabParam(currentValue);
      lastSyncedValue = currentValue;
    }
  });

  $effect(() => {
    void currentValue;
    void renderedItems;
    void resolvedDensity;
    void resolvedSize;
    void resolvedVariant;
    void canCollapse;
    void actions;
    void evaluateCollapsedOverflow();
  });

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

  function applyReorder(fromIndex: number, toIndex: number): void {
    const result = applyReorderItems(renderedItems, fromIndex, toIndex);
    renderedItems = result.items;
    focusIndex = result.focusIndex;
    tick().then(() => tabElements[result.focusIndex]?.focus());
    onReorder?.(result.items.map((item) => item.value));
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
      onClose?.(renderedItems[index].value);
    }
  }
</script>

<div
  bind:this={rootElement}
  class="poodle-tabs"
  data-variant={resolvedVariant}
  data-bordered={bordered}
  data-orientation={orientation}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-collapsed={collapsedByOverflow || undefined}
  data-full-width={fullWidth || undefined}
>
  {#if canCollapse}
    <div class="poodle-tabs__measure-shell" aria-hidden="true">
      <div bind:this={measureListElement} class="poodle-tabs__list poodle-tabs__list--measure">
        {#each renderedItems as item (item.value)}
          <div class="poodle-tabs__item" role="presentation" data-selected={currentValue === item.value}>
            <span class="poodle-tabs__tab">
              {#if item.icon}
                <Icon icon={item.icon} size={resolvedIconSize} />
              {/if}
              <span class="poodle-tabs__label">{item.label}</span>
              {#if item.count !== undefined}
                <Pill
                  tone="neutral"
                  appearance="badge"
                  size={resolvedIconSize}
                  muted
                  adaptiveWidth
                  ariaLabel={`${item.count}`}
                >
                  {item.count}
                </Pill>
              {/if}
            </span>

            {#if item.closable}
              <span class="poodle-tabs__close" aria-hidden="true">
                <Icon name="x" size={resolvedIconSize} />
              </span>
            {/if}
          </div>
          {#if item.separator}
            <span class="poodle-tabs__separator" aria-hidden="true"></span>
          {/if}
        {/each}
      </div>
    </div>
  {/if}

  {#if collapsedByOverflow}
    <div class="poodle-tabs__collapsed">
      <Menu
        items={collapsedMenuItems}
        ariaLabel={ariaLabel ?? "Sections"}
        triggerAriaLabel={ariaLabel ?? "Sections"}
        size={resolvedSize}
        density={resolvedDensity}
        onAction={(value) => setValue(value)}
      >
        {#snippet trigger()}
          <Button
            type="button"
            variant="secondary"
            size={resolvedSize}
            density={resolvedDensity}
            leadingIcon="menu"
            chevron
            ariaLabel={ariaLabel ?? collapseTriggerLabel}
          >
            {collapseTriggerLabel}
          </Button>
        {/snippet}
      </Menu>

      {#if actions}
        <div class="poodle-tabs__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  {:else}
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
          ondragstart={(e) => handleDragStart(e, index)}
          ondragover={(e) => handleDragOver(e, index)}
          ondragleave={handleDragLeave}
          ondrop={(e) => handleDrop(e, index)}
          ondragend={handleDragEnd}
          onmouseenter={() => hasTooltips && scheduleTooltip(index)}
          onmouseleave={() => hasTooltips && dismissTooltip()}
        >
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
            onfocus={() => { focusIndex = index; if (isVertical) scheduleTooltip(index); }}
            onblur={() => hasTooltips && dismissTooltip()}
            onpointerdown={(event) => {
              if (
                reorderable &&
                event.button === 0 &&
                item.disabled !== true &&
                currentValue !== item.value
              ) {
                setValue(item.value);
              }
            }}
            onclick={() => setValue(item.value)}
            onkeydown={(event) => {
              if (event.key === "Escape" && hasTooltips) dismissTooltip();
              handleKeydown(event, index);
            }}
          >
            {#if item.icon}
              <Icon icon={item.icon} size={resolvedIconSize} />
            {/if}
            <span class="poodle-tabs__label">{item.label}</span>
            {#if item.count !== undefined}
              <Pill
                tone="neutral"
                appearance="badge"
                size={resolvedIconSize}
                muted
                adaptiveWidth
                ariaLabel={`${item.count}`}
              >
                {item.count}
              </Pill>
            {/if}
          </button>

          {#if item.closable}
            <button
              type="button"
              class="poodle-tabs__close"
              aria-label={`Close ${item.label}`}
              onclick={(event) => {
                event.stopPropagation();
                onClose?.(item.value);
              }}
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
        {#if item.separator}
          <span class="poodle-tabs__separator" aria-hidden="true"></span>
        {/if}
      {/each}

      {#if actions}
        <div class="poodle-tabs__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  {/if}

  {#if hasPanel && currentValue}
    <div
      class="poodle-tabs__panel"
      id={`poodle-tabpanel-${tabsId}-${currentValue}`}
      role="tabpanel"
      tabindex="0"
      aria-labelledby={`poodle-tab-${tabsId}-${currentValue}`}
    >
      {@render children?.(currentValue)}
    </div>
  {/if}
</div>

<style>
  /* ── Root ── */

  .poodle-tabs {
    --poodle-tabs-control-height: var(--poodle-size-control-height);
    --poodle-tabs-control-x: var(--poodle-space-control-x);
    --poodle-tabs-label-size: var(--poodle-typography-label-size);
    --poodle-tabs-content-gap: var(--poodle-space-inline-sm);
    --poodle-tabs-list-gap: var(--poodle-space-inline-sm);
    --poodle-tabs-separator-margin: var(--poodle-space-inline-sm);
    --poodle-tabs-strip-inline-padding: var(--poodle-tabs-control-x);
    --poodle-tabs-strip-tab-x: var(--poodle-tabs-control-x);
    --poodle-tabs-strip-close-margin-end: 0.25rem;
    display: grid;
    gap: var(--poodle-space-stack-md);
    min-width: 0;
    position: relative;
  }

  .poodle-tabs__measure-shell {
    position: absolute;
    inset: 0 auto auto 0;
    visibility: hidden;
    pointer-events: none;
    min-width: 100%;
    height: 0;
    overflow: hidden;
  }

  .poodle-tabs__collapsed {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-tabs-list-gap);
    min-width: 0;
  }

  .poodle-tabs__collapsed :global(.poodle-menu) {
    display: flex;
    flex: 1 1 auto;
    min-width: 0;
  }

  .poodle-tabs__collapsed :global(.poodle-menu__trigger) {
    display: flex;
    flex: 1 1 auto;
    min-width: 0;
  }

  .poodle-tabs__collapsed :global(.poodle-button) {
    width: 100%;
    justify-content: flex-start;
    text-align: left;
  }

  .poodle-tabs__collapsed :global(.poodle-button__label) {
    flex: 1 1 auto;
    min-width: 0;
    text-align: left;
  }

  .poodle-tabs__collapsed :global(.poodle-button__chevron) {
    margin-left: auto;
  }

  .poodle-tabs[data-size="sm"] {
    --poodle-tabs-control-height: 1.75rem;
    --poodle-tabs-label-size: 0.75rem;
  }

  .poodle-tabs[data-size="md"] {
    --poodle-tabs-control-height: 2.25rem;
    --poodle-tabs-label-size: 0.8125rem;
  }

  .poodle-tabs[data-size="lg"] {
    --poodle-tabs-control-height: 2.75rem;
    --poodle-tabs-label-size: 0.875rem;
  }

  .poodle-tabs[data-size="xs"] {
    --poodle-tabs-control-height: 1.5rem;
    --poodle-tabs-label-size: 0.6875rem;
  }

  .poodle-tabs[data-size="xl"] {
    --poodle-tabs-control-height: 3.25rem;
    --poodle-tabs-label-size: 0.9375rem;
  }

  .poodle-tabs[data-density="compact"] {
    --poodle-tabs-control-x: 0.5rem;
    --poodle-tabs-content-gap: 0.375rem;
    --poodle-tabs-list-gap: 0.25rem;
    --poodle-tabs-separator-margin: 0.375rem;
    --poodle-tabs-strip-inline-padding: 0.5rem;
    --poodle-tabs-strip-tab-x: 0.5rem;
    --poodle-tabs-strip-close-margin-end: 0.25rem;
  }

  .poodle-tabs[data-density="default"] {
    --poodle-tabs-control-x: 0.75rem;
    --poodle-tabs-content-gap: var(--poodle-space-inline-sm);
    --poodle-tabs-list-gap: var(--poodle-space-inline-sm);
    --poodle-tabs-separator-margin: var(--poodle-space-inline-sm);
    --poodle-tabs-strip-inline-padding: 0.75rem;
    --poodle-tabs-strip-tab-x: 0.75rem;
    --poodle-tabs-strip-close-margin-end: 0.375rem;
  }

  .poodle-tabs[data-density="comfortable"] {
    --poodle-tabs-control-x: 1rem;
    --poodle-tabs-content-gap: 0.625rem;
    --poodle-tabs-list-gap: 0.625rem;
    --poodle-tabs-separator-margin: 0.625rem;
    --poodle-tabs-strip-inline-padding: 0.75rem;
    --poodle-tabs-strip-tab-x: 0.75rem;
    --poodle-tabs-strip-close-margin-end: 0.375rem;
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
    gap: var(--poodle-tabs-list-gap);
  }

  .poodle-tabs[data-full-width="true"]:not([data-orientation="vertical"]) .poodle-tabs__list {
    display: flex;
    width: 100%;
  }

  .poodle-tabs__list--measure {
    flex-wrap: nowrap;
    width: max-content;
    max-width: none;
  }

  /* Underline: bottom border on list */
  .poodle-tabs[data-variant="text"] .poodle-tabs__list {
    padding-bottom: var(--poodle-space-inline-sm);
  }

  .poodle-tabs[data-variant="text"][data-bordered="true"] .poodle-tabs__list {
    border-bottom: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent);
  }

  .poodle-tabs[data-variant="text"][data-orientation="vertical"] .poodle-tabs__list {
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
    padding: 0 var(--poodle-tabs-strip-inline-padding);
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
    flex-shrink: 0;
    margin: 0 var(--poodle-tabs-separator-margin);
    background: var(--poodle-color-border-default);
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

  .poodle-tabs[data-full-width="true"]:not([data-orientation="vertical"]) .poodle-tabs__item {
    flex: 1 1 0;
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
    gap: var(--poodle-tabs-content-gap);
    min-height: calc(var(--poodle-tabs-control-height) - 0.25rem);
    padding: 0 var(--poodle-tabs-control-x);
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-tabs-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: 1;
    white-space: nowrap;
  }

  .poodle-tabs[data-full-width="true"]:not([data-orientation="vertical"]) .poodle-tabs__tab {
    width: 100%;
    justify-content: center;
  }

  /* Underline variant: pill-shaped highlight on selected */
  .poodle-tabs[data-variant="text"] .poodle-tabs__tab {
    border-radius: var(--poodle-radius-control);
  }

  .poodle-tabs[data-variant="text"] .poodle-tabs__item[data-selected="true"] .poodle-tabs__tab {
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
    margin-left: var(--poodle-tabs-separator-margin);
    margin-block: 0.375rem;
  }

  .poodle-tabs[data-orientation="vertical"] .poodle-tabs__separator {
    width: auto;
    height: 0.0625rem;
    margin-right: 0;
    margin-bottom: var(--poodle-tabs-separator-margin);
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
    padding: 0 var(--poodle-tabs-strip-tab-x);
    border-radius: 0;
  }

  .poodle-tabs[data-variant="strip"] .poodle-tabs__item:hover {
    background: color-mix(in srgb, var(--poodle-color-surface-hover) 50%, transparent);
  }

  .poodle-tabs[data-variant="strip"] .poodle-tabs__close {
    margin-right: var(--poodle-tabs-strip-close-margin-end);
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

  .poodle-tabs__actions {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-tabs-list-gap);
    flex-shrink: 0;
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

  .poodle-tabs[data-variant="text"] .poodle-tabs__panel {
    padding: 0;
    border: 0;
    border-radius: 0;
    background: transparent;
  }
</style>
