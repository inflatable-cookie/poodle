<script lang="ts">
  import { DockRegion } from "@inflatable-cookie/poodle-svelte";
  import type { PanelTabItem, DockEdge } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const [folder, code, listIcon, terminal] = ["folder", "code", "list", "terminal"] as const;

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

  // Icon-less panels — the case that shipped broken. Compacting to icon-only
  // has nothing to fall back to here, so the labels must survive the squeeze.
  const iconlessItems: PanelTabItem[] = [
    { value: "inspector", label: "Inspector", closable: false },
    { value: "browser", label: "Media Browser", closable: false },
    { value: "clips", label: "Clip Editor", closable: false },
  ];

  let flexItems: PanelTabItem[] = $state([
    { value: "explorer", label: "Explorer", icon: folder, closable: true },
    { value: "search", label: "Search", icon: "search", closable: true },
    { value: "git", label: "Source Control", icon: code, closable: false },
  ]);

  function handleFlexReorder(order: string[]): void {
    flexItems = order.map((id) => flexItems.find((i) => i.value === id)!);
  }

  function handleFlexClose(value: string): void {
    const next = flexItems.filter((i) => i.value !== value);
    flexItems = next;
    if (flexActivePanel === value) {
      flexActivePanel = next[0]?.value ?? "";
    }
  }

  // ── Interactive collapse state ─────────────────────────────────────

  let interactiveCollapsed = $state(false);
  let interactiveActive = $state("files");

  let interactiveItems: PanelTabItem[] = $state([
    { value: "files", label: "Files", icon: folder, closable: true },
    { value: "outline", label: "Outline", icon: listIcon, closable: true },
    { value: "debug", label: "Debug", icon: terminal, closable: false },
  ]);

  function handleInteractiveReorder(order: string[]): void {
    interactiveItems = order.map((id) => interactiveItems.find((i) => i.value === id)!);
  }

  function handleInteractiveClose(value: string): void {
    const next = interactiveItems.filter((i) => i.value !== value);
    interactiveItems = next;
    if (interactiveActive === value) {
      interactiveActive = next[0]?.value ?? "";
    }
  }

  // ── Bottom dock state ───────────────────────────────────────────────

  let bottomCollapsed = $state(false);
  let bottomActive = $state("terminal");

  let bottomItems: PanelTabItem[] = $state([
    { value: "terminal", label: "Terminal", icon: terminal, closable: true },
    { value: "output", label: "Output", icon: "file-text", closable: true },
    { value: "problems", label: "Problems", icon: "alert-circle", closable: false },
  ]);

  function handleBottomReorder(order: string[]): void {
    bottomItems = order.map((id) => bottomItems.find((i) => i.value === id)!);
  }

  function handleBottomClose(value: string): void {
    const next = bottomItems.filter((i) => i.value !== value);
    bottomItems = next;
    if (bottomActive === value) {
      bottomActive = next[0]?.value ?? "";
    }
  }

  // ── Cross-region drag-and-drop state ───────────────────────────────

  let leftItems: PanelTabItem[] = $state([
    { value: "explorer", label: "Explorer", icon: folder, closable: false },
    { value: "search", label: "Search", icon: "search", closable: false },
    { value: "git", label: "Source Control", icon: code, closable: false },
  ]);
  let rightItems: PanelTabItem[] = $state([
    { value: "outline", label: "Outline", icon: listIcon, closable: false },
  ]);
  let leftActive = $state("explorer");
  let rightActive = $state("outline");

  // Size/density axes teach control scale only — no enabled close/reorder.
  const axisItems: PanelTabItem[] = [
    { value: "explorer", label: "Explorer", icon: folder, closable: false },
    { value: "search", label: "Search", icon: "search", closable: false },
    { value: "git", label: "Source Control", icon: code, closable: false },
  ];

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
      <SpecimenGroup label="Expanded side dock" bare>
        <div class="poodle-dock-region-specimen__pair">
          <div class="poodle-specimen__frame poodle-specimen__frame--flex">
            <DockRegion
              edge="left"
              sizing="flexible"
              items={flexItems}
              value={flexActivePanel}
              collapsed={false}
              onValueChange={(value) => (flexActivePanel = value)}
              onReorder={handleFlexReorder}
              onClose={handleFlexClose}
            >
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>{flexActivePanel}</strong>
                  <p>Panel content for the active tab. Tabs are closable and reorderable.</p>
                </div>
              {/snippet}
            </DockRegion>
            <div class="poodle-specimen__flex-main">Main content area</div>
          </div>
          <div class="poodle-specimen__frame poodle-specimen__frame--flex poodle-dock-region-specimen__narrow">
            <DockRegion edge="left" sizing="flexible" items={iconlessItems} value="inspector">
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>Inspector</strong>
                  <p>Panels without icons keep their labels when the strip is squeezed.</p>
                </div>
              {/snippet}
            </DockRegion>
            <div class="poodle-specimen__flex-main">Main content area</div>
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Collapse and edge placement" bare>
        <div class="poodle-dock-region-specimen__stack">
          <div class="poodle-specimen__frame poodle-specimen__frame--flex">
            <DockRegion
              edge="left"
              sizing="flexible"
              items={flexItems}
              value={flexActivePanel}
              collapsed={true}
              collapsedPosture="icon-strip"
              onValueChange={(value) => (flexActivePanel = value)}
              onReorder={handleFlexReorder}
              onClose={handleFlexClose}
            />
            <div class="poodle-specimen__flex-main">Main content area</div>
          </div>
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
              onReorder={handleInteractiveReorder}
              onClose={handleInteractiveClose}
            >
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>{interactiveActive}</strong>
                  <p>Click the collapse toggle to switch between expanded and icon-strip modes.</p>
                </div>
              {/snippet}
            </DockRegion>
            <div class="poodle-specimen__flex-main">Main content area</div>
          </div>
          <div class="poodle-specimen__frame poodle-specimen__frame--bottom-layout">
            <div class="poodle-specimen__flex-main">Editor area</div>
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
              onReorder={handleBottomReorder}
              onClose={handleBottomClose}
            >
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>{bottomActive}</strong>
                  <p>Bottom panel content. Collapses downward, keeping horizontal tabs.</p>
                </div>
              {/snippet}
            </DockRegion>
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Tab strip presentation" bare>
        <div class="poodle-specimen__dnd-layout">
          <div class="poodle-specimen__frame poodle-specimen__dnd-region">
            <DockRegion
              edge="left"
              sizing="flexible"
              items={flexItems}
              value={flexActivePanel}
              tabActiveEdge="none"
              onValueChange={(value) => (flexActivePanel = value)}
              onReorder={handleFlexReorder}
              onClose={handleFlexClose}
            >
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>{flexActivePanel}</strong>
                  <p>tabActiveEdge="none" — no active underline; the tint fill alone marks selection.</p>
                </div>
              {/snippet}
            </DockRegion>
          </div>
          <div class="poodle-specimen__frame poodle-specimen__dnd-region">
            <DockRegion
              edge="left"
              sizing="flexible"
              items={flexItems}
              value={flexActivePanel}
              tabReorderable={false}
              onValueChange={(value) => (flexActivePanel = value)}
              onReorder={handleFlexReorder}
              onClose={handleFlexClose}
            >
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>{flexActivePanel}</strong>
                  <p>tabReorderable={false} — tabs cannot be dragged out of order.</p>
                </div>
              {/snippet}
            </DockRegion>
          </div>
          <div class="poodle-specimen__frame poodle-specimen__dnd-region">
            <DockRegion
              edge="left"
              sizing="flexible"
              items={flexItems}
              value={flexActivePanel}
              tabVariant="pill"
              tabActiveFill="solid"
              onValueChange={(value) => (flexActivePanel = value)}
              onReorder={handleFlexReorder}
              onClose={handleFlexClose}
            >
              {#snippet children()}
                <div class="poodle-specimen__panel-content">
                  <strong>{flexActivePanel}</strong>
                  <p>tabVariant="pill" + tabActiveFill="solid" — a fill combination that was previously unreachable through DockRegion.</p>
                </div>
              {/snippet}
            </DockRegion>
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Move panels between docks" bare>
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

      <SpecimenGroup label="Static panel stacks" bare>
        <div class="poodle-dock-region-specimen__stack">
          <div class="poodle-specimen__frame poodle-specimen__frame--short">
            <DockRegion edge="top" sizing="static" items={staticItems} onReorder={handleStaticReorder}>
              {#snippet panel(item)}
                <div class="poodle-specimen__static-panel">{item.label}</div>
              {/snippet}
            </DockRegion>
          </div>
          <div class="poodle-specimen__frame">
            <DockRegion edge="left" sizing="static" items={staticVerticalItems} onReorder={handleStaticVerticalReorder}>
              {#snippet panel(item)}
                <div class="poodle-specimen__static-panel">{item.label}</div>
              {/snippet}
            </DockRegion>
          </div>
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
          items={axisItems}
          value="git"
          {size}
        >
          {#snippet children()}
            <div class="poodle-specimen__panel-content">
              <strong>git</strong>
              <p>Size axis — presentation only; close and reorder live in the Examples tab.</p>
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
          items={axisItems}
          value="git"
          {density}
        >
          {#snippet children()}
            <div class="poodle-specimen__panel-content">
              <strong>git</strong>
              <p>Density axis — presentation only; close and reorder live in the Examples tab.</p>
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

  .poodle-dock-region-specimen__stack {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-dock-region-specimen__pair {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-dock-region-specimen__narrow {
    max-width: 14rem;
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
