<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/dock-region.css";
  import { onDestroy, tick, type Snippet } from "svelte";

  import { createDockExternalDragController } from "@inflatable-cookie/poodle-core";
  import { default as CollapseToggle } from "./CollapseToggle.svelte";
  import { default as Tabs } from "./Tabs.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    DockCollapsedPosture,
    DockEdge,
    DockEmphasis,
    DockExternalDragSource,
    DockExternalDropTarget,
    DockSizing,
    PanelDragData,
    PanelTabItem,
    SemanticControlSizeRole,
    TabItem,
    TabVariant,
  } from "./types";

  interface Props {
    edge?: DockEdge;
    sizing?: DockSizing;
    collapsible?: boolean;
    /** Show the strip's own collapse toggle. Hosts with divider-level
     * collapse controls (e.g. SplitView pills) set this false to avoid a
     * redundant affordance. Collapse state rendering is unaffected. */
    showCollapseToggle?: boolean;
    /** When false, omit the tab strip (hosts render tabs elsewhere, e.g.
     * titlebar). Body / stack / collapse affordances are unchanged. */
    showTabs?: boolean;
    collapsed?: boolean;
    collapsedPosture?: DockCollapsedPosture;
    emphasis?: DockEmphasis;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    tabVariant?: TabVariant;
    items?: PanelTabItem[];
    value?: string | null;
    ariaLabel?: string | null;
    canAcceptPanel?: ((panelId: string, sourceEdge: DockEdge) => boolean) | null;
    externalDragSource?: DockExternalDragSource | null;
    /** Distinguishes drop zones that share an edge; defaults to `edge`. */
    dragZoneId?: string | null;
    externalDropTarget?: DockExternalDropTarget | null;
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
    showCollapseToggle = true,
    showTabs = true,
    collapsed = false,
    collapsedPosture = "icon-strip",
    emphasis = "standard",
    size = null,
    sizeRole = "chrome",
    density = null,
    tabVariant = "block",
    items = [],
    value = null,
    ariaLabel = null,
    canAcceptPanel = null,
    externalDragSource = null,
    dragZoneId = null,
    externalDropTarget = null,
    onValueChange = undefined,
    onCollapsedChange = undefined,
    onClose = undefined,
    onReorder = undefined,
    onPanelDrop = undefined,
    panel,
    children,
  }: Props = $props();

  const PANEL_DRAG_TYPE = "application/x-poodle-panel-drag";
  const dropZoneId = $derived(dragZoneId ?? edge);

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

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

  /** Compacting to icon-only is only a strategy when there is an icon to fall
   *  back to. Every panel needs one: hiding a label on a tab that has no icon
   *  leaves an empty 2.25rem square with nothing in it and no way to tell the
   *  panels apart. A strip of icon-less tabs overflows and scrolls instead —
   *  cramped beats unreadable. */
  const canCompact = $derived(items.length > 0 && items.every((item) => item.icon));

  function checkCompact(el: HTMLElement): void {
    if (!el || items.length === 0 || !canCompact) {
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

  onDestroy(() => {
    resizeObserver?.disconnect();
    externalDrag.cancel("unmounted");
  });

  let isDragOver = $state(false);
  let dropInsertIndex = $state(-1);
  let dragSourceIndex = $state(-1);

  const externalDrag = createDockExternalDragController({
    source: () => externalDragSource,
    panel: (panelId) => items.find((item) => item.value === panelId),
    edge: () => edge,
  });

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

  function handleTabDragStart(panelId: string, event: DragEvent): void {
    if (!event.dataTransfer) return;
    if (externalDragSource) {
      externalDrag.start(panelId, event);
      return;
    }

    const data: PanelDragData = {
      panelId,
      sourceEdge: edge,
      sourceZone: dropZoneId,
    };
    event.dataTransfer.setData(PANEL_DRAG_TYPE, JSON.stringify(data));
    event.dataTransfer.effectAllowed = "move";
  }

  function handleTabDragEnd(panelId: string, event: DragEvent): void {
    externalDrag.end(panelId, event);
  }

  function canAcceptExternalDrop(
    phase: "over" | "drop",
    event: DragEvent,
  ): boolean {
    if (!externalDropTarget || !event.dataTransfer) return false;
    return externalDropTarget.canDrop({
      phase,
      targetEdge: edge,
      event,
      dataTransfer: event.dataTransfer,
    });
  }

  function handleRegionDragOver(event: DragEvent): void {
    const hasPoodlePanel =
      event.dataTransfer?.types.includes(PANEL_DRAG_TYPE) === true;
    const acceptsExternal =
      !hasPoodlePanel && canAcceptExternalDrop("over", event);
    if (!hasPoodlePanel && !acceptsExternal) return;

    event.preventDefault();
    isDragOver = true;
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = "move";
    }
  }

  function handleRegionDragLeave(event: DragEvent): void {
    const current = event.currentTarget as HTMLElement;
    const related = event.relatedTarget as Node | null;
    if (related && current.contains(related)) return;
    isDragOver = false;
  }

  function handleRegionDrop(event: DragEvent): void {
    isDragOver = false;

    const raw = event.dataTransfer?.getData(PANEL_DRAG_TYPE);
    if (!raw) {
      if (!canAcceptExternalDrop("drop", event) || !event.dataTransfer) return;

      event.preventDefault();
      void externalDropTarget?.drop({
        targetEdge: edge,
        event,
        dataTransfer: event.dataTransfer,
      });
      return;
    }

    event.preventDefault();

    let data: PanelDragData;
    try {
      data = JSON.parse(raw);
    } catch {
      return;
    }

    if ((data.sourceZone ?? data.sourceEdge) === dropZoneId && sizing === "flexible") return;
    if (canAcceptPanel && !canAcceptPanel(data.panelId, data.sourceEdge)) return;

    onPanelDrop?.({ panel: data, targetEdge: edge });
  }

  function handleStackItemDragStart(event: DragEvent, index: number): void {
    if (!event.dataTransfer) return;
    dragSourceIndex = index;
    if (externalDragSource) {
      externalDrag.start(items[index].value, event);
      return;
    }

    // The stack path stamps the zone too. Its own reorder uses
    // `dragSourceIndex` and never reads this, but the payload can land in
    // another region — a stacked panel dragged onto a flexible region sharing
    // its edge is exactly the drop `dragZoneId` exists to let through.
    const data: PanelDragData = {
      panelId: items[index].value,
      sourceEdge: edge,
      sourceZone: dropZoneId,
    };
    event.dataTransfer.setData(PANEL_DRAG_TYPE, JSON.stringify(data));
    event.dataTransfer.effectAllowed = "move";
  }

  function handleStackItemDragOver(event: DragEvent, index: number): void {
    const isLocalReorder = dragSourceIndex >= 0;
    const hasPoodlePanel =
      event.dataTransfer?.types.includes(PANEL_DRAG_TYPE) === true;
    if (!isLocalReorder && !hasPoodlePanel) return;

    event.preventDefault();
    dropInsertIndex = index;
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = "move";
    }
  }

  function handleStackItemDragLeave(): void {
    dropInsertIndex = -1;
  }

  function handleStackItemDrop(event: DragEvent, index: number): void {
    isDragOver = false;
    dropInsertIndex = -1;

    if (dragSourceIndex >= 0) {
      event.preventDefault();
      event.stopPropagation();
      const order = items.map((item) => item.value);
      const [moved] = order.splice(dragSourceIndex, 1);
      order.splice(index, 0, moved);
      dragSourceIndex = -1;
      onReorder?.(order);
      return;
    }

    const raw = event.dataTransfer?.getData(PANEL_DRAG_TYPE);
    if (!raw) return;

    event.preventDefault();
    event.stopPropagation();

    let data: PanelDragData;
    try {
      data = JSON.parse(raw);
    } catch {
      return;
    }

    if (canAcceptPanel && !canAcceptPanel(data.panelId, data.sourceEdge)) return;
    onPanelDrop?.({ panel: data, targetEdge: edge });
  }

  function handleStackDragEnd(event: DragEvent): void {
    const panelId = externalDrag.activePanelId();
    if (panelId) {
      externalDrag.end(panelId, event);
    }
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
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-collapsed={collapsed || undefined}
  data-collapsed-posture={collapsed ? collapsedPosture : undefined}
  data-show-tabs={showTabs ? undefined : "false"}
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
          onpointerdown={(event) =>
            externalDrag.prepare(item.value, event)}
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
    {#if collapsible && showCollapseToggle}
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
      {#if collapsible && showCollapseToggle}
        <CollapseToggle
          {collapsed}
          direction={collapseDirection}
          ariaLabel={`Expand ${edge} dock`}
          onToggle={handleCollapseToggle}
        />
      {/if}
      {#if showTabs}
        <Tabs
          variant={tabVariant}
          activeEdge="underline"
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
          onDragPrepare={externalDrag.prepare}
          onDragStart={handleTabDragStart}
          onDragEnd={handleTabDragEnd}
        />
      {/if}
    </div>
  {:else if showIconStrip}
    <div
      class="poodle-dock-region__strip"
      data-orientation="horizontal"
      data-compact={isCompact || undefined}
    >
      {#if showTabs}
        <div class="poodle-dock-region__tabs" use:observeStrip>
          <Tabs
            variant={tabVariant}
            activeEdge="underline"
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
            onDragPrepare={externalDrag.prepare}
            onDragStart={handleTabDragStart}
            onDragEnd={handleTabDragEnd}
          />
        </div>
      {/if}
      {#if collapsible && showCollapseToggle}
        <CollapseToggle
          {collapsed}
          direction={collapseDirection}
          ariaLabel={`Expand ${edge} dock`}
          onToggle={handleCollapseToggle}
        />
      {/if}
    </div>
  {:else}
    {#if showTabs}
      <div
        class="poodle-dock-region__strip"
        data-orientation="horizontal"
        data-compact={isCompact || undefined}
      >
        <div class="poodle-dock-region__tabs" use:observeStrip>
          <Tabs
            variant={tabVariant}
            activeEdge="underline"
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
            onDragPrepare={externalDrag.prepare}
            onDragStart={handleTabDragStart}
            onDragEnd={handleTabDragEnd}
          />
        </div>
        {#if collapsible && showCollapseToggle}
          <CollapseToggle
            {collapsed}
            direction={collapseDirection}
            ariaLabel={`Collapse ${edge} dock`}
            onToggle={handleCollapseToggle}
          />
        {/if}
      </div>
    {:else if collapsible && showCollapseToggle}
      <div class="poodle-dock-region__edge-toggle">
        <CollapseToggle
          {collapsed}
          direction={collapseDirection}
          ariaLabel={`Collapse ${edge} dock`}
          onToggle={handleCollapseToggle}
        />
      </div>
    {/if}

    <div class="poodle-dock-region__body">
      {@render children?.(activeItem)}
    </div>
  {/if}
</section>
