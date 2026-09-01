<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/dock-region.css";
  import { onDestroy, tick, untrack, type Snippet } from "svelte";

  import {
    createDragDropController,
    decodeDockPanelSubject,
    encodeDockPanelSubject,
    DOCK_PANEL_SUBJECT_KIND,
    type CrossWindowDragSourceBridge,
    type CrossWindowDragTargetBridge,
    type DragDropCommitResult,
    type DropIntent,
    type DropTargetRegistration,
  } from "@inflatable-cookie/poodle-core";
  import { default as CollapseToggle } from "./CollapseToggle.svelte";
  import { default as DragDropProvider } from "./DragDropProvider.svelte";
  import { default as Tabs } from "./Tabs.svelte";
  import {
    dragDropSnapshotStore,
    dragSourceAction,
    dropTargetAction,
    tryDragDrop,
  } from "./drag-drop-context";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import { setTabsForeignInsert } from "./tabs-foreign-insert";
  import type {
    ActiveEdge,
    ActiveFill,
    ControlDensity,
    ControlSize,
    DockCollapsedPosture,
    DockEdge,
    DockEmphasis,
    DockSizing,
    DockPanelDropPayload,
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
    /** Selection edge on the active tab, forwarded to Tabs. Defaults to the
     * former strip indicator (`"underline"`). */
    tabActiveEdge?: ActiveEdge;
    /** Selection fill on the active tab, forwarded to Tabs. */
    tabActiveFill?: ActiveFill;
    /** Draw the strip border around the tab list, forwarded to Tabs. */
    tabBordered?: boolean;
    /** Stretch tabs to fill the strip, forwarded to Tabs. */
    tabFullWidth?: boolean;
    /** Allow dragging tabs to reorder; forwarded to Tabs. */
    tabReorderable?: boolean;
    items?: PanelTabItem[];
    value?: string | null;
    ariaLabel?: string | null;
    canAcceptPanel?: ((panelId: string, sourceEdge: DockEdge) => boolean) | null;
    /**
     * Host preparation for a panel that may leave this window.
     *
     * Forwarded to the tab strip as its `crossWindowSourceBridge`: the host
     * owns the transaction and only an opaque receipt leaves the window.
     */
    crossWindowDragSource?: CrossWindowDragSourceBridge;
    /** Distinguishes drop zones that share an edge; defaults to `edge`. */
    dragZoneId?: string | null;
    /**
     * Incoming host projection and commit for this window.
     *
     * Only meaningful when this region owns its controller. A region that
     * joined an ambient `DragDropProvider` is not the window: the provider is,
     * and the bridge belongs there.
     */
    crossWindowDropTarget?: CrossWindowDragTargetBridge;
    onValueChange?: ((value: string) => void) | undefined;
    onCollapsedChange?: ((isCollapsed: boolean) => void) | undefined;
    onClose?: ((value: string) => void) | undefined;
    onReorder?: ((items: string[]) => void) | undefined;
    onPanelDrop?: ((payload: DockPanelDropPayload) => void) | undefined;
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
    tabActiveEdge = "underline",
    tabActiveFill = "tint",
    tabBordered = false,
    tabFullWidth = false,
    tabReorderable = true,
    items = [],
    value = null,
    ariaLabel = null,
    canAcceptPanel = null,
    crossWindowDragSource = undefined,
    dragZoneId = null,
    crossWindowDropTarget = undefined,
    onValueChange = undefined,
    onCollapsedChange = undefined,
    onClose = undefined,
    onReorder = undefined,
    onPanelDrop = undefined,
    panel,
    children,
  }: Props = $props();

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
  /**
   * The strip's tabs carry the encoded subject id as their value.
   *
   * That is the whole composition seam: the tab set shares the
   * `poodle.dock-panel` family, and each tab's semantic subject id says which
   * panel, from which edge, and from which zone — which is what a sibling
   * region needs during hover. It is substrate identity only, so every public
   * boundary below decodes it back to the consumer's own panel value.
   */
  const tabItems = $derived.by<TabItem[]>(() =>
    items.map((item) => ({
      value: panelSubjectId(item.value),
      label: item.label,
      icon: item.icon ?? undefined,
      closable: item.closable,
    })),
  );

  /** The public panel value behind a strip value. Never leaks the encoding. */
  function panelValueOf(encoded: string): string {
    return decodeDockPanelSubject(encoded)?.panelId ?? encoded;
  }
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
  });

  // Every one of these takes a strip value and hands out a panel value. The
  // encoding is substrate identity; it must never reach a consumer callback.
  function handleValueChange(nextValue: string): void {
    onValueChange?.(panelValueOf(nextValue));
    if (collapsed) {
      onCollapsedChange?.(false);
    }
  }

  function handleReorder(nextItems: string[]): void {
    onReorder?.(nextItems.map(panelValueOf));
  }

  function handleClose(nextValue: string): void {
    onClose?.(panelValueOf(nextValue));
  }

  function handleCollapseToggle(): void {
    onCollapsedChange?.(!collapsed);
    if (!collapsed) {
      void tick();
    }
  }

  // ── Panel movement (shared drag substrate) ─────────────────────────────

  /**
   * Join the nearest provider, or own a controller.
   *
   * Two sibling regions can only see each other's targets when one controller
   * holds both registrations, so a consumer that wants cross-region transfer
   * wraps its regions in one `DragDropProvider`. A region with no provider
   * still reorders its own stack — it simply does not discover anyone else.
   * There is no document-global session restoring that link implicitly.
   */
  const ambient = tryDragDrop();
  // Both are fixed for the region's lifetime, like the controller they
  // configure: a bridge swapped mid-session would be a second window host.
  const windowBridge = untrack(() => crossWindowDropTarget);
  if (ambient && windowBridge) {
    throw new Error(
      "DockRegion: crossWindowDropTarget belongs on the DragDropProvider that owns this window, not on a region that joined one",
    );
  }
  const ownController = ambient
    ? undefined
    : createDragDropController({ crossWindowTargetBridge: windowBridge });
  const controller = ambient?.controller ?? ownController!;
  const dragSource = dragSourceAction(controller);
  const dropTarget = dropTargetAction(controller);
  const snapshot = dragDropSnapshotStore(controller);

  const isDragOver = $derived(
    $snapshot.targetId === dropZoneId && $snapshot.targetPosture === "accepted",
  );

  function panelSubjectId(panelId: string): string {
    return encodeDockPanelSubject({ panelId, sourceEdge: edge, sourceZone: dropZoneId });
  }

  /**
   * The one eligibility rule every dock target shares.
   *
   * `canAcceptPanel` is consumer policy and runs on hover *and* again at
   * commit, which is what the substrate guarantees and the old side channel
   * could only approximate.
   */
  function acceptsPanel(intent: DropIntent, subjectId: string): boolean {
    const panel = decodeDockPanelSubject(subjectId);
    if (!panel) return false;
    return canAcceptPanel === null || canAcceptPanel(panel.panelId, panel.sourceEdge as DockEdge);
  }

  function panelFrom(subjectId: string): PanelDragData | null {
    const decoded = decodeDockPanelSubject(subjectId);
    if (!decoded) return null;
    return {
      panelId: decoded.panelId,
      sourceEdge: decoded.sourceEdge as DockEdge,
      sourceZone: decoded.sourceZone,
    };
  }

  /** The region itself: anything from another zone may land here. */
  const regionTarget = $derived<DropTargetRegistration>({
    targetId: dropZoneId,
    acceptedKinds: [DOCK_PANEL_SUBJECT_KIND],
    label: ariaLabel ?? `${edge} dock`,
    // Deepest wins, so a stack item always beats the region it sits in.
    priority: -1,
    resolvePosition: () => "inside",
    canDrop: (intent, subject) => {
      const panel = decodeDockPanelSubject(subject.id);
      if (!panel) return { accepted: false, reason: "not a panel" };
      // A panel dropped back on its own flexible zone is a same-strip reorder,
      // which the strip already owns.
      if (panel.sourceZone === dropZoneId && sizing === "flexible") {
        return { accepted: false, reason: "same zone" };
      }
      return acceptsPanel(intent, subject.id)
        ? { accepted: true, intent }
        : { accepted: false, reason: "refused by host" };
    },
    onDrop: (): DragDropCommitResult => {
      const panel = panelFrom($snapshot.session?.subject.id ?? "");
      if (!panel) return { status: "rejected", reason: "not a panel" };
      if (canAcceptPanel !== null && !canAcceptPanel(panel.panelId, panel.sourceEdge as DockEdge)) {
        return { status: "rejected", reason: "refused by host" };
      }
      onPanelDrop?.({ panel, targetEdge: edge, index: items.length });
      return { status: "committed" };
    },
  });

  /**
   * One stacked panel: a source, and a target for the insert position.
   *
   * Static mode is where DockRegion owns the panels themselves, so this is the
   * local move the substrate can carry end to end. A drop from this region's
   * own zone is a reorder; anything else is a transfer.
   */
  function stackSource(item: PanelTabItem, index: number) {
    return {
      sourceId: `${dropZoneId}:${item.value}`,
      subject: { kind: DOCK_PANEL_SUBJECT_KIND, id: panelSubjectId(item.value) },
      allowedOperations: ["move"] as const,
      label: item.label ?? `Panel ${index + 1}`,
      crossWindowSourceBridge: crossWindowDragSource,
    };
  }

  function stackTarget(item: PanelTabItem, index: number): DropTargetRegistration {
    return {
      targetId: `${dropZoneId}:slot:${item.value}`,
      acceptedKinds: [DOCK_PANEL_SUBJECT_KIND],
      label: item.label ?? `Panel ${index + 1}`,
      resolvePosition: ({ x, y, rect }) => {
        return stackDirection === "column"
          ? y < rect.top + rect.height / 2
            ? "before"
            : "after"
          : x < rect.left + rect.width / 2
            ? "before"
            : "after";
      },
      canDrop: (intent, subject) => {
        const panel = decodeDockPanelSubject(subject.id);
        if (!panel) return { accepted: false, reason: "not a panel" };
        if (panel.sourceZone === dropZoneId && panel.panelId === item.value) {
          return { accepted: false, reason: "same panel" };
        }
        return acceptsPanel(intent, subject.id)
          ? { accepted: true, intent }
          : { accepted: false, reason: "refused by host" };
      },
      onDrop: (intent): DragDropCommitResult => {
        const panel = panelFrom($snapshot.session?.subject.id ?? "");
        if (!panel) return { status: "rejected", reason: "not a panel" };
        const after = intent.position === "after";

        if (panel.sourceZone === dropZoneId) {
          const order = items.map((entry) => entry.value);
          const from = order.indexOf(panel.panelId);
          if (from < 0) return { status: "rejected", reason: "unknown panel" };
          const to = after
            ? from < index
              ? index
              : index + 1
            : from < index
              ? index - 1
              : index;
          if (from === to) return { status: "rejected", reason: "same panel" };
          const [moved] = order.splice(from, 1);
          order.splice(to, 0, moved);
          onReorder?.(order);
          return { status: "committed" };
        }

        if (canAcceptPanel !== null && !canAcceptPanel(panel.panelId, panel.sourceEdge as DockEdge)) {
          return { status: "rejected", reason: "refused by host" };
        }
        onPanelDrop?.({ panel, targetEdge: edge, index: after ? index + 1 : index });
        return { status: "committed" };
      },
    };
  }

  function handleForeignDrop(subjectId: string, index: number): DragDropCommitResult {
    const panel = panelFrom(subjectId);
    if (!panel) return { status: "rejected", reason: "not a panel" };
    if (canAcceptPanel !== null && !canAcceptPanel(panel.panelId, panel.sourceEdge as DockEdge)) {
      return { status: "rejected", reason: "refused by host" };
    }
    onPanelDrop?.({ panel, targetEdge: edge, index });
    return { status: "committed" };
  }

  setTabsForeignInsert({
    canAccept: (subjectId) => acceptsPanel({ targetId: "", position: "inside", operation: "move" }, subjectId),
    commit: handleForeignDrop,
  });

  function stackItemState(item: PanelTabItem): { dragging: boolean; over: boolean } {
    return {
      dragging:
        $snapshot.sourceId === `${dropZoneId}:${item.value}` &&
        ($snapshot.phase === "dragging" || $snapshot.phase === "dropping"),
      over:
        $snapshot.targetId === `${dropZoneId}:slot:${item.value}` &&
        $snapshot.targetPosture === "accepted",
    };
  }

</script>

{#snippet region()}
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
  use:dropTarget={regionTarget}
>
  {#if isDragOver}
    <div class="poodle-dock-region__drop-zone"></div>
  {/if}

  {#if sizing === "static"}
    <div class="poodle-dock-region__stack" data-direction={stackDirection}>
      {#each items as item, index (item.value)}
        <div
          class="poodle-dock-region__stack-item"
          data-drop-target={stackItemState(item).over || undefined}
          data-drag-source={stackItemState(item).dragging || undefined}
          role="group"
          aria-label={item.label ?? `Panel ${index + 1}`}
          use:dragSource={stackSource(item, index)}
          use:dropTarget={stackTarget(item, index)}
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
          activeEdge={tabActiveEdge}
          activeFill={tabActiveFill}
          bordered={tabBordered}
          fullWidth={tabFullWidth}
          orientation="vertical"
          {size}
          {sizeRole}
          {density}
          items={tabItems}
          value={activeItem ? panelSubjectId(activeItem.value) : ""}
          reorderable={tabReorderable}
          ariaLabel={ariaLabel ?? `${edge} dock panels`}
          onValueChange={handleValueChange}
          onReorder={handleReorder}
          onClose={handleClose}
          crossWindowSourceBridge={crossWindowDragSource}
          dragSubjectKind={DOCK_PANEL_SUBJECT_KIND}
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
            activeEdge={tabActiveEdge}
            activeFill={tabActiveFill}
            bordered={tabBordered}
            fullWidth={tabFullWidth}
            orientation="horizontal"
            {size}
            {sizeRole}
            {density}
            showTooltips={isCompact}
            items={tabItems}
            value={activeItem ? panelSubjectId(activeItem.value) : ""}
            reorderable={tabReorderable}
            ariaLabel={ariaLabel ?? `${edge} dock panels`}
            onValueChange={handleValueChange}
            onReorder={handleReorder}
            onClose={handleClose}
          crossWindowSourceBridge={crossWindowDragSource}
          dragSubjectKind={DOCK_PANEL_SUBJECT_KIND}
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
            activeEdge={tabActiveEdge}
            activeFill={tabActiveFill}
            bordered={tabBordered}
            fullWidth={tabFullWidth}
            orientation="horizontal"
            {size}
            {sizeRole}
            {density}
            showTooltips={isCompact}
            items={tabItems}
            value={activeItem ? panelSubjectId(activeItem.value) : ""}
            reorderable={tabReorderable}
            ariaLabel={ariaLabel ?? `${edge} dock panels`}
            onValueChange={handleValueChange}
            onReorder={handleReorder}
            onClose={handleClose}
          crossWindowSourceBridge={crossWindowDragSource}
          dragSubjectKind={DOCK_PANEL_SUBJECT_KIND}
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
{/snippet}

{#if ambient}
  {@render region()}
{:else}
  <DragDropProvider controller={ownController}>
    {@render region()}
  </DragDropProvider>
{/if}
