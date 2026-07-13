<script lang="ts">
  import "@poodle/styles/dock-region.css";
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

