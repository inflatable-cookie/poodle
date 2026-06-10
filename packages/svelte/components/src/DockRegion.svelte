<script lang="ts">
  import { onDestroy, tick, type Snippet } from "svelte";

  import { default as CollapseToggle } from "./CollapseToggle.svelte";
  import { default as Tabs } from "./Tabs.svelte";
  import type {
    ControlDensity,
    ControlSize,
    DockCollapsedPosture,
    DockEdge,
    DockEmphasis,
    DockSizing,
    PanelDragData,
    PanelTabItem,
    SemanticControlSizeRole,
    TabItem,
  } from "./types";

  interface Props {
    edge?: DockEdge;
    sizing?: DockSizing;
    collapsible?: boolean;
    collapsed?: boolean;
    collapsedPosture?: DockCollapsedPosture;
    emphasis?: DockEmphasis;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    items?: PanelTabItem[];
    value?: string | null;
    ariaLabel?: string | null;
    canAcceptPanel?: ((panelId: string, sourceEdge: DockEdge) => boolean) | null;
    onValueChange?: ((value: string) => void) | undefined;
    onCollapsedChange?: ((isCollapsed: boolean) => void) | undefined;
    onClose?: ((value: string) => void) | undefined;
    onReorder?: ((items: string[]) => void) | undefined;
    onPanelDrop?: ((payload: { panel: PanelDragData; targetEdge: DockEdge }) => void) | undefined;
    panel?: Snippet<[PanelTabItem]>;
    children?: Snippet<[PanelTabItem | null]>;
  }

  let {
    edge = "left",
    sizing = "flexible",
    collapsible = false,
    collapsed = false,
    collapsedPosture = "icon-strip",
    emphasis = "standard",
    size = null,
    sizeRole = "chrome",
    density = null,
    items = [],
    value = null,
    ariaLabel = null,
    canAcceptPanel = null,
    onValueChange = undefined,
    onCollapsedChange = undefined,
    onClose = undefined,
    onReorder = undefined,
    onPanelDrop = undefined,
    panel,
    children,
  }: Props = $props();

  const PANEL_DRAG_TYPE = "application/x-poodle-panel-drag";

  const isVerticalEdge = $derived(edge === "left" || edge === "right");
  const activeItem = $derived(items.find((item) => item.value === value) ?? items[0] ?? null);
  const collapseDirection = $derived(({ left: "left", right: "right", top: "up", bottom: "down" } as const)[edge]);
  const tabOrientation = $derived(
    collapsed && collapsedPosture === "icon-strip" && isVerticalEdge ? "vertical" : "horizontal",
  );
  const tabItems = $derived.by<TabItem[]>(() =>
    items.map((item) => ({
      value: item.value,
      label: item.label,
      icon: item.icon ?? undefined,
      closable: item.closable,
    })),
  );
  const stackDirection = $derived(isVerticalEdge ? "column" : "row");
  const showIconStrip = $derived(collapsed && collapsedPosture === "icon-strip");
  const showHidden = $derived(collapsed && collapsedPosture === "hidden");

  let stripEl = $state<HTMLElement | null>(null);
  let isCompact = $state(false);
  let resizeObserver = $state<ResizeObserver | null>(null);
  let fullLabelScrollWidth = $state(0);

  function checkCompact(el: HTMLElement): void {
    if (!el || items.length === 0) {
      isCompact = false;
      return;
    }
    const list = el.querySelector("[role='tablist']") as HTMLElement | null;
    if (!list) return;

    const containerWidth = el.clientWidth;

    if (!isCompact) {
      fullLabelScrollWidth = list.scrollWidth;
      isCompact = list.scrollWidth > containerWidth + 2;
      return;
    }

    isCompact = fullLabelScrollWidth > containerWidth;
  }

  function observeStrip(el: HTMLElement) {
    stripEl = el;
    resizeObserver?.disconnect();

    const check = () => checkCompact(el);

    resizeObserver = new ResizeObserver(check);
    resizeObserver.observe(el);

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

  let isDragOver = $state(false);
  let dropInsertIndex = $state(-1);
  let dragSourceIndex = $state(-1);

  function handleValueChange(nextValue: string): void {
    onValueChange?.(nextValue);
    if (collapsed) {
      onCollapsedChange?.(false);
    }
  }

  function handleReorder(nextItems: string[]): void {
    onReorder?.(nextItems);
  }

  function handleClose(nextValue: string): void {
    onClose?.(nextValue);
  }

  function handleCollapseToggle(): void {
    onCollapsedChange?.(!collapsed);
    if (!collapsed) {
      void tick();
    }
  }

  function handleStripDragStart(event: DragEvent): void {
    if (!event.dataTransfer) return;

    const target = event.target as HTMLElement;
    const tab = target.querySelector?.("[role='tab']") ?? target.closest?.("[role='tab']");
    if (!tab) return;

    const tabId = tab.getAttribute("id") ?? "";
    const item = items.find((entry) => tabId.endsWith(`-${entry.value}`));
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

    onPanelDrop?.({ panel: data, targetEdge: edge });
  }

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

    if (data.sourceEdge === edge && dragSourceIndex >= 0) {
      const order = items.map((item) => item.value);
      const [moved] = order.splice(dragSourceIndex, 1);
      order.splice(index, 0, moved);
      dragSourceIndex = -1;
      onReorder?.(order);
      return;
    }

    if (canAcceptPanel && !canAcceptPanel(data.panelId, data.sourceEdge)) return;
    onPanelDrop?.({ panel: data, targetEdge: edge });
  }

  function handleStackDragEnd(): void {
    isDragOver = false;
    dragSourceIndex = -1;
    dropInsertIndex = -1;
  }
</script>

<section
  class="poodle-dock-region"
  data-edge={edge}
  data-sizing={sizing}
  data-emphasis={emphasis}
  data-collapsed={collapsed || undefined}
  data-collapsed-posture={collapsed ? collapsedPosture : undefined}
  aria-label={ariaLabel ?? `${edge} dock`}
  ondragover={handleRegionDragOver}
  ondragleave={handleRegionDragLeave}
  ondrop={handleRegionDrop}
>
  {#if isDragOver}
    <div class="poodle-dock-region__drop-zone"></div>
  {/if}

  {#if sizing === "static"}
    <div class="poodle-dock-region__stack" data-direction={stackDirection}>
      {#each items as item, index (item.value)}
        <div
          class="poodle-dock-region__stack-item"
          data-drop-target={dropInsertIndex === index || undefined}
          data-drag-source={dragSourceIndex === index || undefined}
          draggable="true"
          role="group"
          aria-label={item.label ?? `Panel ${index + 1}`}
          ondragstart={(event) => handleStackItemDragStart(event, index)}
          ondragover={(event) => handleStackItemDragOver(event, index)}
          ondragleave={handleStackItemDragLeave}
          ondrop={(event) => handleStackItemDrop(event, index)}
          ondragend={handleStackDragEnd}
        >
          {@render panel?.(item)}
        </div>
      {/each}
    </div>
  {:else if showHidden}
    {#if collapsible}
      <div class="poodle-dock-region__edge-toggle">
        <CollapseToggle
          {collapsed}
          direction={collapseDirection}
          ariaLabel={`Expand ${edge} dock`}
          onToggle={handleCollapseToggle}
        />
      </div>
    {/if}
  {:else if showIconStrip && isVerticalEdge}
    <div class="poodle-dock-region__strip" data-orientation="vertical">
      {#if collapsible}
        <CollapseToggle
          {collapsed}
          direction={collapseDirection}
          ariaLabel={`Expand ${edge} dock`}
          onToggle={handleCollapseToggle}
        />
      {/if}
      <Tabs
        variant="strip"
        orientation="vertical"
        {size}
        {sizeRole}
        {density}
        items={tabItems}
        value={activeItem?.value ?? ""}
        reorderable={true}
        ariaLabel={ariaLabel ?? `${edge} dock panels`}
        onValueChange={handleValueChange}
        onReorder={handleReorder}
        onClose={handleClose}
      />
    </div>
  {:else if showIconStrip}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="poodle-dock-region__strip"
      data-orientation="horizontal"
      data-compact={isCompact || undefined}
      ondragstart={handleStripDragStart}
    >
      <div class="poodle-dock-region__tabs" use:observeStrip>
        <Tabs
          variant="strip"
          orientation="horizontal"
          {size}
          {sizeRole}
          {density}
          showTooltips={isCompact}
          items={tabItems}
          value={activeItem?.value ?? ""}
          reorderable={true}
          ariaLabel={ariaLabel ?? `${edge} dock panels`}
          onValueChange={handleValueChange}
          onReorder={handleReorder}
          onClose={handleClose}
        />
      </div>
      {#if collapsible}
        <CollapseToggle
          {collapsed}
          direction={collapseDirection}
          ariaLabel={`Expand ${edge} dock`}
          onToggle={handleCollapseToggle}
        />
      {/if}
    </div>
  {:else}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="poodle-dock-region__strip"
      data-orientation="horizontal"
      data-compact={isCompact || undefined}
      ondragstart={handleStripDragStart}
    >
      <div class="poodle-dock-region__tabs" use:observeStrip>
        <Tabs
          variant="strip"
          orientation="horizontal"
          {size}
          {sizeRole}
          {density}
          showTooltips={isCompact}
          items={tabItems}
          value={activeItem?.value ?? ""}
          reorderable={true}
          ariaLabel={ariaLabel ?? `${edge} dock panels`}
          onValueChange={handleValueChange}
          onReorder={handleReorder}
          onClose={handleClose}
        />
      </div>
      {#if collapsible}
        <CollapseToggle
          {collapsed}
          direction={collapseDirection}
          ariaLabel={`Collapse ${edge} dock`}
          onToggle={handleCollapseToggle}
        />
      {/if}
    </div>

    <div class="poodle-dock-region__body">
      {@render children?.(activeItem)}
    </div>
  {/if}
</section>

<style>
  .poodle-dock-region {
    position: relative;
    display: grid;
    min-width: 0;
    min-height: 0;
    height: 100%;
    border: 0;
    background: color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
  }

  .poodle-dock-region[data-emphasis="quiet"] {
    border-color: transparent;
    background: transparent;
  }

  .poodle-dock-region[data-emphasis="strong"] {
    border-color: color-mix(in srgb, var(--poodle-color-accent-base) 32%, var(--poodle-color-border-subtle));
  }

  .poodle-dock-region[data-sizing="static"] {
    grid-template-rows: 1fr;
  }

  .poodle-dock-region__stack {
    display: flex;
    min-width: 0;
    min-height: 0;
  }

  .poodle-dock-region__stack[data-direction="column"] {
    flex-direction: column;
  }

  .poodle-dock-region__stack[data-direction="row"] {
    flex-direction: row;
  }

  .poodle-dock-region__stack-item {
    flex: 1 1 0;
    min-width: 0;
    min-height: 0;
    cursor: grab;
  }

  .poodle-dock-region__stack-item[data-drag-source] {
    opacity: 0.4;
  }

  .poodle-dock-region__stack-item[data-drop-target] {
    box-shadow: inset 0 0 0 0.125rem var(--poodle-color-accent-base);
    border-radius: var(--poodle-radius-control);
  }

  .poodle-dock-region[data-sizing="flexible"]:not([data-collapsed]) {
    grid-template-rows: auto minmax(0, 1fr);
  }

  .poodle-dock-region[data-sizing="flexible"]:not([data-collapsed])[data-edge="left"] {
    border-right: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-dock-region[data-sizing="flexible"]:not([data-collapsed])[data-edge="right"] {
    border-left: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-dock-region[data-sizing="flexible"]:not([data-collapsed])[data-edge="top"] {
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-dock-region[data-sizing="flexible"]:not([data-collapsed])[data-edge="bottom"] {
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-dock-region__strip[data-orientation="horizontal"] {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    padding-right: 0.5rem;
    min-height: 2.75rem;
    border-bottom: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-dock-region__tabs {
    flex: 1 1 0;
    min-width: 0;
  }

  .poodle-dock-region__strip[data-orientation="horizontal"] :global(.poodle-tabs[data-variant="strip"] .poodle-tabs__list) {
    border-bottom: 0;
  }

  .poodle-dock-region__strip[data-compact] :global(.poodle-tabs__label),
  .poodle-dock-region__strip[data-compact] :global(.poodle-tabs__close) {
    display: none;
  }

  .poodle-dock-region__strip[data-compact] :global(.poodle-tabs__tab) {
    min-width: 2.25rem;
    justify-content: center;
    padding-inline: 0;
  }

  .poodle-dock-region__strip[data-compact] :global(.poodle-tabs__list) {
    justify-content: flex-start;
  }

  .poodle-dock-region__body {
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }

  .poodle-dock-region[data-collapsed][data-collapsed-posture="icon-strip"] {
    min-width: 0;
    min-height: 0;
  }

  .poodle-dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="left"],
  .poodle-dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="right"] {
    grid-template-columns: 1fr;
  }

  .poodle-dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="top"],
  .poodle-dock-region[data-collapsed][data-collapsed-posture="icon-strip"][data-edge="bottom"] {
    grid-template-rows: 1fr;
  }

  .poodle-dock-region__strip[data-orientation="vertical"] {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--poodle-space-stack-sm);
    padding-block: 0.5rem;
    min-width: 2.75rem;
    border-right: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-dock-region__strip[data-orientation="vertical"] :global(.poodle-tabs) {
    width: 100%;
  }

  .poodle-dock-region__strip[data-orientation="vertical"] :global(.poodle-tabs[data-orientation="vertical"]) {
    grid-template-columns: 1fr;
    --poodle-tabs-control-height: 2.5rem;
    --poodle-tabs-control-x: 0.75rem;
  }

  .poodle-dock-region__strip[data-orientation="vertical"] :global(.poodle-tabs__list) {
    flex-direction: column;
    align-items: stretch;
    width: 100%;
    border-bottom: 0;
    border-right: 0 !important;
  }

  .poodle-dock-region__strip[data-orientation="vertical"] :global(.poodle-tabs__item) {
    width: 100%;
  }

  /* Right-edge docks face their content on the left, so the selected-tab
     indicator and strip separator flip from the right edge to the left. */
  .poodle-dock-region[data-edge="right"] .poodle-dock-region__strip[data-orientation="vertical"] {
    border-right: 0;
    border-left: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-dock-region[data-edge="right"]
    .poodle-dock-region__strip[data-orientation="vertical"]
    :global(.poodle-tabs[data-variant="strip"] .poodle-tabs__item) {
    border-right: 0;
    margin-right: 0;
    border-left: 0.125rem solid transparent;
    margin-left: -0.125rem;
  }

  .poodle-dock-region[data-edge="right"]
    .poodle-dock-region__strip[data-orientation="vertical"]
    :global(.poodle-tabs[data-variant="strip"] .poodle-tabs__item[data-selected="true"]) {
    border-left-color: var(--poodle-color-accent-base);
  }

  .poodle-dock-region__strip[data-orientation="vertical"] > :global(.collapse-toggle) {
    flex: 0 0 auto;
  }

  .poodle-dock-region[data-collapsed][data-collapsed-posture="hidden"] {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .poodle-dock-region__edge-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
  }

  .poodle-dock-region__drop-zone {
    position: absolute;
    inset: 0;
    z-index: 1;
    pointer-events: none;
    border: 0.125rem dashed color-mix(in srgb, var(--poodle-color-accent-base) 60%, transparent);
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-accent-base) 8%, transparent);
  }
</style>
