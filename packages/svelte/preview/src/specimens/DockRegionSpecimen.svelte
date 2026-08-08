<script lang="ts">
  import { DockRegion } from "@inflatable-cookie/poodle-svelte";
  import type { PanelTabItem, DockEdge } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  import { folder, code, list as listIcon, terminal } from "@inflatable-cookie/poodle-core/icons";

  // ── Static dock state ──────────────────────────────────────────────

  let staticItems: PanelTabItem[] = $state([
    { value: "meter", label: "Meter Strip" },
    { value: "transport", label: "Transport" },
    { value: "mixer", label: "Mixer" },
  ]);

  function handleStaticReorder(order: string[]): void {
    staticItems = order.map((id) => staticItems.find((i) => i.value === id)!);
  }

  let staticVerticalItems: PanelTabItem[] = $state([
    { value: "toolbar", label: "Toolbar" },
    { value: "inspector", label: "Inspector" },
  ]);

  function handleStaticVerticalReorder(order: string[]): void {
    staticVerticalItems = order.map((id) => staticVerticalItems.find((i) => i.value === id)!);
  }

  // ── Flexible dock state ────────────────────────────────────────────

  let flexActivePanel = $state("explorer");
  let flexCollapsed = false;

  const flexItems: PanelTabItem[] = [
    { value: "explorer", label: "Explorer", icon: folder, closable: true },
    { value: "search", label: "Search", icon: "search", closable: true },
    { value: "git", label: "Source Control", icon: code, closable: false },
  ];

  // ── Interactive collapse state ─────────────────────────────────────

  let interactiveCollapsed = $state(false);
  let interactiveActive = $state("files");

  const interactiveItems: PanelTabItem[] = [
    { value: "files", label: "Files", icon: folder, closable: true },
    { value: "outline", label: "Outline", icon: listIcon, closable: true },
    { value: "debug", label: "Debug", icon: terminal, closable: false },
  ];

  // ── Bottom dock state ───────────────────────────────────────────────

  let bottomCollapsed = $state(false);
  let bottomActive = $state("terminal");

  const bottomItems: PanelTabItem[] = [
    { value: "terminal", label: "Terminal", icon: terminal, closable: true },
    { value: "output", label: "Output", icon: "file-text", closable: true },
    { value: "problems", label: "Problems", icon: "alert-circle", closable: false },
  ];

  // ── Cross-region drag-and-drop state ───────────────────────────────

  let leftItems: PanelTabItem[] = $state([
    { value: "explorer", label: "Explorer", icon: folder, closable: true },
    { value: "search", label: "Search", icon: "search", closable: true },
    { value: "git", label: "Source Control", icon: code, closable: true },
  ]);
  let rightItems: PanelTabItem[] = $state([
    { value: "outline", label: "Outline", icon: listIcon, closable: true },
  ]);
  let leftActive = $state("explorer");
  let rightActive = $state("outline");

  function canAcceptPanel(panelId: string, _sourceEdge: DockEdge): boolean {
    return true;
  }

  function handleLeftDrop({ panel }: { panel: { panelId: string; sourceEdge: DockEdge }; targetEdge: DockEdge }): void {
    const { panelId, sourceEdge } = panel;
    if (sourceEdge === "right") {
      const item = rightItems.find((i) => i.value === panelId);
      if (!item) return;
      rightItems = rightItems.filter((i) => i.value !== panelId);
      leftItems = [...leftItems, item];
      if (rightActive === panelId) {
        rightActive = rightItems[0]?.value ?? "";
      }
    }
  }

  function handleRightDrop({ panel }: { panel: { panelId: string; sourceEdge: DockEdge }; targetEdge: DockEdge }): void {
    const { panelId, sourceEdge } = panel;
    if (sourceEdge === "left") {
      const item = leftItems.find((i) => i.value === panelId);
      if (!item) return;
      leftItems = leftItems.filter((i) => i.value !== panelId);
      rightItems = [...rightItems, item];
      if (leftActive === panelId) {
        leftActive = leftItems[0]?.value ?? "";
      }
    }
  }

  function handleLeftReorder(items: string[]): void {
    leftItems = items.map((id) => leftItems.find((i) => i.value === id)!);
  }

  function handleRightReorder(items: string[]): void {
    rightItems = items.map((id) => rightItems.find((i) => i.value === id)!);
  }
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-specimen">
      <!-- 1. Flexible dock (expanded) -->
      <SpecimenGroup label="Flexible dock — expanded (left edge)" bare>
        <div class="poodle-specimen__frame poodle-specimen__frame--flex">
          <DockRegion
            edge="left"
            sizing="flexible"
            items={flexItems}
            value={flexActivePanel}
            collapsed={false}
            onValueChange={(value) => (flexActivePanel = value)}
          >
            {#snippet children()}
              <div class="poodle-specimen__panel-content">
                <strong>{flexActivePanel}</strong>
                <p>Panel content for the active tab. Tabs are closable and reorderable.</p>
              </div>
            {/snippet}
          </DockRegion>
          <div class="poodle-specimen__flex-main">
            Main content area
          </div>
        </div>
      </SpecimenGroup>

      <!-- 2. Flexible dock (collapsed icon-strip) -->
      <SpecimenGroup label="Flexible dock — collapsed icon-strip (left edge)" bare>
        <div class="poodle-specimen__frame poodle-specimen__frame--flex">
          <DockRegion
            edge="left"
            sizing="flexible"
            items={flexItems}
            value={flexActivePanel}
            collapsed={true}
            collapsedPosture="icon-strip"
            onValueChange={(value) => (flexActivePanel = value)}
          />
          <div class="poodle-specimen__flex-main">
            Main content area
          </div>
        </div>
      </SpecimenGroup>

      <!-- 3. Interactive collapse toggle -->
      <SpecimenGroup label="Interactive collapse toggle (click to toggle)" bare>
        <div class="poodle-specimen__frame poodle-specimen__frame--flex">
          <DockRegion
            edge="left"
            sizing="flexible"
            collapsible
            items={interactiveItems}
            value={interactiveActive}
            collapsed={interactiveCollapsed}
            collapsedPosture="icon-strip"
            onValueChange={(value) => (interactiveActive = value)}
            onCollapsedChange={(isCollapsed) => (interactiveCollapsed = isCollapsed)}
          >
            {#snippet children()}
              <div class="poodle-specimen__panel-content">
                <strong>{interactiveActive}</strong>
                <p>Click the collapse toggle to switch between expanded and icon-strip modes.</p>
              </div>
            {/snippet}
          </DockRegion>
          <div class="poodle-specimen__flex-main">
            Main content area
          </div>
        </div>
      </SpecimenGroup>

      <!-- 4. Bottom edge collapsible dock -->
      <SpecimenGroup label="Bottom edge dock (click to toggle)" bare>
        <div class="poodle-specimen__frame poodle-specimen__frame--bottom-layout">
          <div class="poodle-specimen__flex-main">
            Editor area
          </div>
          <DockRegion
            edge="bottom"
            sizing="flexible"
            collapsible
            items={bottomItems}
            value={bottomActive}
            collapsed={bottomCollapsed}
            collapsedPosture="icon-strip"
            onValueChange={(value) => (bottomActive = value)}
            onCollapsedChange={(isCollapsed) => (bottomCollapsed = isCollapsed)}
          >
            {#snippet children()}
              <div class="poodle-specimen__panel-content">
                <strong>{bottomActive}</strong>
                <p>Bottom panel content. Collapses downward, keeping horizontal tabs.</p>
              </div>
            {/snippet}
          </DockRegion>
        </div>
      </SpecimenGroup>

      <!-- 5. Cross-region drag-and-drop -->
      <SpecimenGroup label="Cross-region drag-and-drop (drag tabs between docks)" bare>
        <div class="poodle-specimen__dnd-layout">
          <div class="poodle-specimen__frame poodle-specimen__dnd-region">
            <DockRegion
              edge="left"
              sizing="flexible"
              items={leftItems}
              value={leftActive}
              ariaLabel="Left dock"
              {canAcceptPanel}
              onValueChange={(value) => (leftActive = value)}
              onReorder={handleLeftReorder}
              onPanelDrop={handleLeftDrop}
            >
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>{leftActive}</strong>
                  <p>Left dock — {leftItems.length} panels</p>
                </div>
              {/snippet}
            </DockRegion>
          </div>
          <div class="poodle-specimen__frame poodle-specimen__dnd-region">
            <DockRegion
              edge="right"
              sizing="flexible"
              items={rightItems}
              value={rightActive}
              ariaLabel="Right dock"
              {canAcceptPanel}
              onValueChange={(value) => (rightActive = value)}
              onReorder={handleRightReorder}
              onPanelDrop={handleRightDrop}
            >
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>{rightActive}</strong>
                  <p>Right dock — {rightItems.length} panels</p>
                </div>
              {/snippet}
            </DockRegion>
          </div>
        </div>
      </SpecimenGroup>

      <!-- 6a. Static dock — horizontal (top edge, panels stack vertically) -->
      <SpecimenGroup label="Static dock — horizontal (top edge)" bare>
        <div class="poodle-specimen__frame poodle-specimen__frame--short">
          <DockRegion
            edge="top"
            sizing="static"
            items={staticItems}
            onReorder={handleStaticReorder}
          >
            {#snippet panel(item)}
              <div class="poodle-specimen__static-panel">
                {item.label}
              </div>
            {/snippet}
          </DockRegion>
        </div>
      </SpecimenGroup>

      <!-- 6b. Static dock — vertical (left edge, panels stack horizontally) -->
      <SpecimenGroup label="Static dock — vertical (left edge)" bare>
        <div class="poodle-specimen__frame">
          <DockRegion
            edge="left"
            sizing="static"
            items={staticVerticalItems}
            onReorder={handleStaticVerticalReorder}
          >
            {#snippet panel(item)}
              <div class="poodle-specimen__static-panel">
                {item.label}
              </div>
            {/snippet}
          </DockRegion>
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <div class="poodle-dock-region-specimen__variant-block">
      <div class="poodle-dock-region-specimen__label">{size.toUpperCase()}</div>
      <div class="poodle-specimen__frame poodle-specimen__frame--variant">
        <DockRegion
          edge="left"
          sizing="flexible"
          items={flexItems}
          value="git"
          {size}
        >
          {#snippet children()}
            <div class="poodle-specimen__panel-content">
              <strong>git</strong>
              <p>Panel content for the active tab. Tabs are closable and reorderable.</p>
            </div>
          {/snippet}
        </DockRegion>
        <div class="poodle-specimen__flex-main">
          Main content area
        </div>
      </div>
    </div>
  {/snippet}

  {#snippet densities(density)}
    <div class="poodle-dock-region-specimen__variant-block">
      <div class="poodle-dock-region-specimen__label">{density.toUpperCase()}</div>
      <div class="poodle-specimen__frame poodle-specimen__frame--variant">
        <DockRegion
          edge="left"
          sizing="flexible"
          items={flexItems}
          value="git"
          {density}
        >
          {#snippet children()}
            <div class="poodle-specimen__panel-content">
              <strong>git</strong>
              <p>Panel content for the active tab. Tabs are closable and reorderable.</p>
            </div>
          {/snippet}
        </DockRegion>
        <div class="poodle-specimen__flex-main">
          Main content area
        </div>
      </div>
    </div>
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__frame {
    height: 16rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  .poodle-specimen__frame--short {
    height: 6rem;
  }

  .poodle-specimen__frame--flex {
    display: flex;
    align-items: stretch;
  }

  .poodle-specimen__frame--variant {
    display: flex;
    align-items: stretch;
    height: 10rem;
  }

  .poodle-specimen__frame--bottom-layout {
    display: flex;
    flex-direction: column;
    height: 22rem;
  }

  .poodle-specimen__frame--bottom-layout .poodle-specimen__flex-main {
    flex: 1 1 0;
    min-height: 0;
  }

  .poodle-specimen__frame--bottom-layout :global(.poodle-dock-region) {
    height: auto;
    flex: 0 0 auto;
    max-height: 10rem;
  }

  .poodle-specimen__frame--flex :global(.poodle-dock-region:not([data-collapsed])) {
    flex: 0 0 16rem;
  }

  .poodle-specimen__frame--flex :global(.poodle-dock-region[data-collapsed]) {
    flex: 0 0 auto;
  }

  .poodle-specimen__flex-main {
    flex: 1 1 0;
    min-width: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.75rem;
    color: var(--poodle-color-text-secondary);
  }

  .poodle-specimen__panel-content {
    padding: 0.75rem;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    line-height: 1.5;
  }

  .poodle-specimen__panel-content strong {
    display: block;
    margin-bottom: 0.25rem;
    color: var(--poodle-color-text-primary);
    text-transform: capitalize;
  }

  .poodle-specimen__panel-content p {
    margin: 0;
  }

  .poodle-specimen__static-panel {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 0.75rem;
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--poodle-color-text-secondary);
    border-right: 0.0625rem solid var(--poodle-color-border-subtle);
  }

  .poodle-specimen__static-panel:last-child {
    border-right: 0;
  }


  .poodle-specimen__dnd-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .poodle-specimen__dnd-region {
    height: 18rem;
  }

  .poodle-dock-region-specimen__variant-block {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: min(100%, 24rem);
  }

  .poodle-dock-region-specimen__label {
    color: var(--poodle-color-text-muted);
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.16em;
    text-transform: uppercase;
  }
</style>
