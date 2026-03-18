<script lang="ts">
  import { createEventDispatcher, onDestroy, tick } from "svelte";

  import { CollapseToggle, Tabs } from "@pug/svelte-primitives";
  import type { TabItem } from "@pug/svelte-primitives";

  import type {
    DockEdge,
    DockEmphasis,
    DockCollapsedPosture,
    DockSizing,
    PanelDragData,
    PanelTabItem,
  } from "./types";

  export let edge: DockEdge = "left";
  export let sizing: DockSizing = "flexible";
  export let isCollapsed = false;
  export let collapsedPosture: DockCollapsedPosture = "icon-strip";
  export let emphasis: DockEmphasis = "standard";
  export let items: PanelTabItem[] = [];
  export let value: string | null = null;
  export let ariaLabel: string | null = null;
  export let canAcceptPanel: ((panelId: string, sourceEdge: DockEdge) => boolean) | null = null;

  const PANEL_DRAG_TYPE = "application/x-pug-panel-drag";

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    collapsedChange: { isCollapsed: boolean };
    close: { value: string };
    reorder: { items: string[] };
    panelDrop: { panel: PanelDragData; targetEdge: DockEdge };
  }>();

  // ── Derived state ──────────────────────────────────────────────────

  $: activeItem = items.find((item) => item.value === value) ?? items[0] ?? null;

  $: collapseDirection = (
    { left: "left", right: "right", top: "up", bottom: "down" } as const
  )[edge];

  $: tabOrientation =
    isCollapsed && collapsedPosture === "icon-strip" && isVerticalEdge ? "vertical" : "horizontal";

  $: tabItems = items.map<TabItem>((item) => ({
    value: item.value,
    label: item.label,
    icon: item.icon ?? undefined,
    isClosable: item.isClosable,
  }));

  $: isVerticalEdge = edge === "left" || edge === "right";
  $: stackDirection = isVerticalEdge ? "row" : "column";
  $: showIconStrip = isCollapsed && collapsedPosture === "icon-strip";
  $: showHidden = isCollapsed && collapsedPosture === "hidden";

  // ── Compact mode (auto-collapse labels when too narrow) ─────────────

  let stripEl: HTMLElement | null = null;
  let isCompact = false;
  let resizeObserver: ResizeObserver | null = null;
  let fullLabelScrollWidth = 0;

  function checkCompact(el: HTMLElement): void {
    if (!el || items.length === 0) {
      isCompact = false;
      return;
    }
    const list = el.querySelector("[role='tablist']") as HTMLElement | null;
    if (!list) return;

    const containerWidth = el.clientWidth;

    if (!isCompact) {
      // Currently showing full labels — cache their natural width
      fullLabelScrollWidth = list.scrollWidth;
      const shouldCompact = list.scrollWidth > containerWidth + 2;
      isCompact = shouldCompact;
    } else {
      // Currently compact — only un-compact if container is wide enough for full labels
      // No buffer needed — if fullLabelScrollWidth fits, show full labels
      isCompact = fullLabelScrollWidth > containerWidth;
    }
  }

  function observeStrip(el: HTMLElement) {
    stripEl = el;
    resizeObserver?.disconnect();

    const check = () => checkCompact(el);

    resizeObserver = new ResizeObserver(check);
    resizeObserver.observe(el);

    // Also observe the tab list itself — its scrollWidth changes when items render
    tick().then(() => {
      const list = el.querySelector("[role='tablist']");
      if (list) resizeObserver?.observe(list);
      check();
    });

    return {
      destroy() {
        resizeObserver?.disconnect();
        resizeObserver = null;
      },
    };
  }

  onDestroy(() => resizeObserver?.disconnect());

  // ── Drag-and-drop state ────────────────────────────────────────────

  let isDragOver = false;
  let dropInsertIndex = -1;
  let dragSourceIndex = -1;

  // ── Flexible mode: tab event handlers ──────────────────────────────

  function handleValueChange(event: CustomEvent<{ value: string }>): void {
    dispatch("valueChange", event.detail);
    if (isCollapsed) {
      dispatch("collapsedChange", { isCollapsed: false });
    }
  }

  function handleReorder(event: CustomEvent<{ items: string[] }>): void {
    dispatch("reorder", event.detail);
  }

  function handleClose(event: CustomEvent<{ value: string }>): void {
    dispatch("close", event.detail);
  }

  function handleCollapseToggle(): void {
    dispatch("collapsedChange", { isCollapsed: !isCollapsed });

    if (!isCollapsed) {
      // Collapsing — focus will move to collapse toggle via DOM
      void tick();
    }
  }

  // ── Cross-region drag-and-drop (flexible + static) ─────────────────

  function handleStripDragStart(event: DragEvent): void {
    if (!event.dataTransfer) return;

    // Find which tab is being dragged by examining the event target
    // event.target is the .pug-tabs__item div (draggable element)
    // The tab button (with id="pug-tab-{tabsId}-{item.value}") is a child
    const target = event.target as HTMLElement;
    const tab = target.querySelector?.("[role='tab']") ?? target.closest?.("[role='tab']");
    if (!tab) return;

    const tabId = tab.getAttribute("id") ?? "";
    const item = items.find((i) => tabId.endsWith(`-${i.value}`));
    if (!item) return;

    const data: PanelDragData = { panelId: item.value, sourceEdge: edge };
    event.dataTransfer.setData(PANEL_DRAG_TYPE, JSON.stringify(data));
  }

  function handleRegionDragOver(event: DragEvent): void {
    if (!event.dataTransfer?.types.includes(PANEL_DRAG_TYPE)) return;
    event.preventDefault();
    isDragOver = true;
    event.dataTransfer.dropEffect = "move";
  }

  function handleRegionDragLeave(event: DragEvent): void {
    const current = event.currentTarget as HTMLElement;
    const related = event.relatedTarget as Node | null;
    if (related && current.contains(related)) return;
    isDragOver = false;
  }

  function handleRegionDrop(event: DragEvent): void {
    event.preventDefault();
    isDragOver = false;

    const raw = event.dataTransfer?.getData(PANEL_DRAG_TYPE);
    if (!raw) return;

    let data: PanelDragData;
    try {
      data = JSON.parse(raw);
    } catch {
      return;
    }

    if (data.sourceEdge === edge && sizing === "flexible") return;

    if (canAcceptPanel && !canAcceptPanel(data.panelId, data.sourceEdge)) return;

    dispatch("panelDrop", { panel: data, targetEdge: edge });
  }

  // ── Static mode: internal reorder drag-and-drop ────────────────────

  function handleStackItemDragStart(event: DragEvent, index: number): void {
    if (!event.dataTransfer) return;
    dragSourceIndex = index;
    const data: PanelDragData = { panelId: items[index].value, sourceEdge: edge };
    event.dataTransfer.setData(PANEL_DRAG_TYPE, JSON.stringify(data));
    event.dataTransfer.effectAllowed = "move";
  }

  function handleStackItemDragOver(event: DragEvent, index: number): void {
    if (!event.dataTransfer?.types.includes(PANEL_DRAG_TYPE)) return;
    event.preventDefault();
    dropInsertIndex = index;
    event.dataTransfer.dropEffect = "move";
  }

  function handleStackItemDragLeave(): void {
    dropInsertIndex = -1;
  }

  function handleStackItemDrop(event: DragEvent, index: number): void {
    event.preventDefault();
    event.stopPropagation();
    isDragOver = false;
    dropInsertIndex = -1;

    const raw = event.dataTransfer?.getData(PANEL_DRAG_TYPE);
    if (!raw) return;

    let data: PanelDragData;
    try {
      data = JSON.parse(raw);
    } catch {
      return;
    }

    // Internal reorder within static region
    if (data.sourceEdge === edge && dragSourceIndex >= 0) {
      const order = items.map((i) => i.value);
      const [moved] = order.splice(dragSourceIndex, 1);
      order.splice(index, 0, moved);
      dragSourceIndex = -1;
      dispatch("reorder", { items: order });
      return;
    }

    // Cross-region drop
    if (canAcceptPanel && !canAcceptPanel(data.panelId, data.sourceEdge)) return;
    dispatch("panelDrop", { panel: data, targetEdge: edge });
  }

  function handleStackDragEnd(): void {
    isDragOver = false;
    dragSourceIndex = -1;
    dropInsertIndex = -1;
  }
</script>

<section
  class="dock-region"
  data-edge={edge}
  data-sizing={sizing}
  data-emphasis={emphasis}
  data-collapsed={isCollapsed || undefined}
  data-collapsed-posture={isCollapsed ? collapsedPosture : undefined}
  aria-label={ariaLabel ?? `${edge} dock`}
  on:dragover={handleRegionDragOver}
  on:dragleave={handleRegionDragLeave}
  on:drop={handleRegionDrop}
>
  {#if isDragOver}
    <div class="dock-region__drop-zone" />
  {/if}

  {#if sizing === "static"}
    <!-- Static: stacked panels, no tabs, no collapse -->
    <div class="dock-region__stack" data-direction={stackDirection}>
      {#each items as item, index (item.value)}
        <div
          class="dock-region__stack-item"
          data-drop-target={dropInsertIndex === index || undefined}
          data-drag-source={dragSourceIndex === index || undefined}
          draggable="true"
          on:dragstart={(e) => handleStackItemDragStart(e, index)}
          on:dragover={(e) => handleStackItemDragOver(e, index)}
          on:dragleave={handleStackItemDragLeave}
          on:drop={(e) => handleStackItemDrop(e, index)}
          on:dragend={handleStackDragEnd}
        >
          <slot name="panel" {item} />
        </div>
      {/each}
    </div>

  {:else if showHidden}
    <!-- Flexible collapsed (hidden): just the collapse toggle -->
    <div class="dock-region__edge-toggle">
      <CollapseToggle
        {isCollapsed}
        direction={collapseDirection}
        ariaLabel={`Expand ${edge} dock`}
        on:toggle={handleCollapseToggle}
      />
    </div>

  {:else if showIconStrip && isVerticalEdge}
    <!-- Flexible collapsed (icon-strip) for left/right: collapse toggle + vertical tabs -->
    <div class="dock-region__strip" data-orientation="vertical">
      <CollapseToggle
        {isCollapsed}
        direction={collapseDirection}
        ariaLabel={`Expand ${edge} dock`}
        on:toggle={handleCollapseToggle}
      />
      <Tabs
        variant="strip"
        orientation="vertical"
        items={tabItems}
        value={activeItem?.value ?? ""}
        isReorderable={true}
        ariaLabel={ariaLabel ?? `${edge} dock panels`}
        on:valueChange={handleValueChange}
        on:reorder={handleReorder}
        on:close={handleClose}
      />
    </div>

  {:else if showIconStrip}
    <!-- Flexible collapsed (icon-strip) for top/bottom: horizontal tabs, no body -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="dock-region__strip"
      data-orientation="horizontal"
      data-compact={isCompact || undefined}
      on:dragstart={handleStripDragStart}
    >
      <div class="dock-region__tabs" use:observeStrip>
        <Tabs
          variant="strip"
          orientation="horizontal"
          showTooltips={isCompact}
          items={tabItems}
          value={activeItem?.value ?? ""}
          isReorderable={true}
          ariaLabel={ariaLabel ?? `${edge} dock panels`}
          on:valueChange={handleValueChange}
          on:reorder={handleReorder}
          on:close={handleClose}
        />
      </div>
      <CollapseToggle
        {isCollapsed}
        direction={collapseDirection}
        ariaLabel={`Expand ${edge} dock`}
        on:toggle={handleCollapseToggle}
      />
    </div>

  {:else}
    <!-- Flexible expanded: horizontal tabs + body -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="dock-region__strip"
      data-orientation="horizontal"
      data-compact={isCompact || undefined}
      on:dragstart={handleStripDragStart}
    >
      <div class="dock-region__tabs" use:observeStrip>
        <Tabs
          variant="strip"
          orientation="horizontal"
          showTooltips={isCompact}
          items={tabItems}
          value={activeItem?.value ?? ""}
          isReorderable={true}
          ariaLabel={ariaLabel ?? `${edge} dock panels`}
          on:valueChange={handleValueChange}
          on:reorder={handleReorder}
          on:close={handleClose}
        />
      </div>
      <CollapseToggle
        {isCollapsed}
        direction={collapseDirection}
        ariaLabel={`Collapse ${edge} dock`}
        on:toggle={handleCollapseToggle}
      />
    </div>

    <div class="dock-region__body">
      <slot activeItem={activeItem} />
    </div>
  {/if}
</section>

<style>
  .dock-region {
    position: relative;
    display: grid;
    min-width: 0;
    min-height: 0;
    height: 100%;
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 94%, transparent);
  }

  /* Emphasis variants */
  .dock-region[data-emphasis="quiet"] {
    border-color: transparent;
    background: transparent;
  }

  .dock-region[data-emphasis="strong"] {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 32%, var(--pug-color-border-subtle));
  }

  /* ── Static mode ── */

  .dock-region[data-sizing="static"] {
    grid-template-rows: 1fr;
  }

  .dock-region__stack {
    display: flex;
    min-width: 0;
    min-height: 0;
  }

  .dock-region__stack[data-direction="column"] {
    flex-direction: column;
  }

  .dock-region__stack[data-direction="row"] {
    flex-direction: row;
  }

  .dock-region__stack-item {
    flex: 1 1 0;
    min-width: 0;
    min-height: 0;
    cursor: grab;
  }

  .dock-region__stack-item[data-drag-source] {
    opacity: 0.4;
  }

  .dock-region__stack-item[data-drop-target] {
    box-shadow: inset 0 0 0 0.125rem var(--pug-color-accent-base);
    border-radius: var(--pug-radius-control);
  }

  /* ── Flexible expanded ── */

  .dock-region[data-sizing="flexible"]:not([data-collapsed]) {
    grid-template-rows: auto minmax(0, 1fr);
  }

  .dock-region__strip[data-orientation="horizontal"] {
    display: flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
    padding-right: 0.5rem;
    border-bottom: 0.0625rem solid var(--pug-color-border-subtle);
  }

  .dock-region__tabs {
    flex: 1 1 0;
    min-width: 0;
  }

  .dock-region__strip[data-orientation="horizontal"] :global(.pug-tabs[data-variant="strip"] .pug-tabs__list) {
    border-bottom: 0;
  }

  /* Compact mode: icon-only horizontal tabs when strip is too narrow */
  .dock-region__strip[data-compact] :global(.pug-tabs__label),
  .dock-region__strip[data-compact] :global(.pug-tabs__close) {
    display: none;
  }

  .dock-region__strip[data-compact] :global(.pug-tabs__tab) {
    padding: 0 0.5rem;
    justify-content: center;
  }

  .dock-region__strip[data-compact] :global(.pug-tabs__list) {
    overflow: visible;
  }

  .dock-region__body {
    min-height: 0;
    overflow: auto;
  }

  /* ── Flexible collapsed (icon-strip) ── */

  .dock-region[data-collapsed][data-collapsed-posture="icon-strip"] {
    grid-template-rows: 1fr;
  }

  /* Side docks collapse to narrow icon strip */
  .dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="left"],
  .dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="right"] {
    width: fit-content;
  }

  /* Top/bottom docks collapse to a thin horizontal strip */
  .dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="top"],
  .dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="bottom"] {
    height: fit-content;
  }

  .dock-region__strip[data-orientation="vertical"] {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--pug-space-stack-sm);
    padding: var(--pug-space-panel-y, 0.5rem) 0;
  }

  .dock-region__strip[data-orientation="vertical"] :global(.pug-tabs) {
    flex: 1 1 0;
    min-height: 0;
  }

  /* ── Flexible collapsed (hidden) ── */

  .dock-region[data-collapsed][data-collapsed-posture="hidden"] {
    border-color: transparent;
    background: transparent;
  }

  .dock-region__edge-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--pug-space-panel-y, 0.5rem);
  }

  /* ── Drop zone overlay ── */

  .dock-region__drop-zone {
    position: absolute;
    inset: 0;
    z-index: 10;
    border: 0.125rem dashed var(--pug-color-accent-base);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-accent-base) 10%, transparent);
    pointer-events: none;
  }
</style>
