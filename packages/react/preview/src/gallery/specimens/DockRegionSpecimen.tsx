import { useState, type CSSProperties } from "react";
import { DockRegion } from "@inflatable-cookie/poodle-react";
import type { PanelTabItem, DockEdge } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const frameBase: CSSProperties = {
  height: "16rem",
  border: "0.0625rem solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-surface)",
  overflow: "hidden",
};
const frameFlex: CSSProperties = { ...frameBase, display: "flex", alignItems: "stretch" };
const frameShort: CSSProperties = { ...frameBase, height: "6rem" };
const frameVariant: CSSProperties = { ...frameBase, display: "flex", alignItems: "stretch", height: "10rem" };
const frameBottom: CSSProperties = { ...frameBase, display: "flex", flexDirection: "column", height: "22rem" };
const frameDnd: CSSProperties = { ...frameBase, height: "18rem" };

const flexMain: CSSProperties = {
  flex: "1 1 0",
  minWidth: 0,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
};
const flexMainBottom: CSSProperties = { ...flexMain, minHeight: 0 };

const panelContent: CSSProperties = {
  padding: "0.75rem",
  fontSize: "0.8125rem",
  color: "var(--poodle-color-text-secondary)",
  lineHeight: 1.5,
};
const panelStrong: CSSProperties = {
  display: "block",
  marginBottom: "0.25rem",
  color: "var(--poodle-color-text-primary)",
  textTransform: "capitalize",
};

const dndLayout: CSSProperties = { display: "grid", gridTemplateColumns: "1fr 1fr", gap: "1rem" };
const variantBlock: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: "0.5rem",
  width: "min(100%, 24rem)",
};
const variantLabel: CSSProperties = {
  color: "var(--poodle-color-text-muted)",
  fontSize: "0.75rem",
  fontWeight: 700,
  letterSpacing: "0.16em",
  textTransform: "uppercase",
};

export function DockRegionSpecimen() {
  // ── Static dock state ──────────────────────────────────────────────
  const [staticItems, setStaticItems] = useState<PanelTabItem[]>([
    { value: "meter", label: "Meter Strip" },
    { value: "transport", label: "Transport" },
    { value: "mixer", label: "Mixer" },
  ]);

  function handleStaticReorder(order: string[]): void {
    setStaticItems((current) => order.map((id) => current.find((i) => i.value === id)!));
  }

  const [staticVerticalItems, setStaticVerticalItems] = useState<PanelTabItem[]>([
    { value: "toolbar", label: "Toolbar" },
    { value: "inspector", label: "Inspector" },
  ]);

  function handleStaticVerticalReorder(order: string[]): void {
    setStaticVerticalItems((current) => order.map((id) => current.find((i) => i.value === id)!));
  }

  // ── Flexible dock state ────────────────────────────────────────────
  const [flexActivePanel, setFlexActivePanel] = useState("explorer");

  const flexItems: PanelTabItem[] = [
    { value: "explorer", label: "Explorer", icon: "folder", closable: true },
    { value: "search", label: "Search", icon: "search", closable: true },
    { value: "git", label: "Source Control", icon: "code", closable: false },
  ];

  // ── Interactive collapse state ─────────────────────────────────────
  const [interactiveCollapsed, setInteractiveCollapsed] = useState(false);
  const [interactiveActive, setInteractiveActive] = useState("files");

  const interactiveItems: PanelTabItem[] = [
    { value: "files", label: "Files", icon: "folder", closable: true },
    { value: "outline", label: "Outline", icon: "list", closable: true },
    { value: "debug", label: "Debug", icon: "terminal", closable: false },
  ];

  // ── Bottom dock state ───────────────────────────────────────────────
  const [bottomCollapsed, setBottomCollapsed] = useState(false);
  const [bottomActive, setBottomActive] = useState("terminal");

  const bottomItems: PanelTabItem[] = [
    { value: "terminal", label: "Terminal", icon: "terminal", closable: true },
    { value: "output", label: "Output", icon: "file-text", closable: true },
    { value: "problems", label: "Problems", icon: "alert-circle", closable: false },
  ];

  // ── Cross-region drag-and-drop state ───────────────────────────────
  const [leftItems, setLeftItems] = useState<PanelTabItem[]>([
    { value: "explorer", label: "Explorer", icon: "folder", closable: true },
    { value: "search", label: "Search", icon: "search", closable: true },
    { value: "git", label: "Source Control", icon: "code", closable: true },
  ]);
  const [rightItems, setRightItems] = useState<PanelTabItem[]>([
    { value: "outline", label: "Outline", icon: "list", closable: true },
  ]);
  const [leftActive, setLeftActive] = useState("explorer");
  const [rightActive, setRightActive] = useState("outline");

  function canAcceptPanel(_panelId: string, _sourceEdge: DockEdge): boolean {
    return true;
  }

  function handleLeftDrop({ panel }: { panel: { panelId: string; sourceEdge: DockEdge }; targetEdge: DockEdge }): void {
    const { panelId, sourceEdge } = panel;
    if (sourceEdge === "right") {
      const item = rightItems.find((i) => i.value === panelId);
      if (!item) return;
      setRightItems((current) => current.filter((i) => i.value !== panelId));
      setLeftItems((current) => [...current, item]);
      if (rightActive === panelId) {
        setRightActive(rightItems.find((i) => i.value !== panelId)?.value ?? "");
      }
    }
  }

  function handleRightDrop({ panel }: { panel: { panelId: string; sourceEdge: DockEdge }; targetEdge: DockEdge }): void {
    const { panelId, sourceEdge } = panel;
    if (sourceEdge === "left") {
      const item = leftItems.find((i) => i.value === panelId);
      if (!item) return;
      setLeftItems((current) => current.filter((i) => i.value !== panelId));
      setRightItems((current) => [...current, item]);
      if (leftActive === panelId) {
        setLeftActive(leftItems.find((i) => i.value !== panelId)?.value ?? "");
      }
    }
  }

  function handleLeftReorder(items: string[]): void {
    setLeftItems((current) => items.map((id) => current.find((i) => i.value === id)!));
  }

  function handleRightReorder(items: string[]): void {
    setRightItems((current) => items.map((id) => current.find((i) => i.value === id)!));
  }

  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <div style={variantBlock}>
          <div style={variantLabel}>{size.toUpperCase()}</div>
          <div style={frameVariant}>
            <div style={{ flex: "0 0 16rem", minWidth: 0, minHeight: 0 }}>
              <DockRegion edge="left" sizing="flexible" items={flexItems} value="git" size={size}>
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>git</strong>
                    <p style={{ margin: 0 }}>Panel content for the active tab. Tabs are closable and reorderable.</p>
                  </div>
                )}
              </DockRegion>
            </div>
            <div style={flexMain}>Main content area</div>
          </div>
        </div>
      )}
      densities={(density) => (
        <div style={variantBlock}>
          <div style={variantLabel}>{density.toUpperCase()}</div>
          <div style={frameVariant}>
            <div style={{ flex: "0 0 16rem", minWidth: 0, minHeight: 0 }}>
              <DockRegion edge="left" sizing="flexible" items={flexItems} value="git" density={density}>
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>git</strong>
                    <p style={{ margin: 0 }}>Panel content for the active tab. Tabs are closable and reorderable.</p>
                  </div>
                )}
              </DockRegion>
            </div>
            <div style={flexMain}>Main content area</div>
          </div>
        </div>
      )}
    >
      <div className="poodle-specimen">
        {/* 1. Flexible dock (expanded) */}
        <SpecimenGroup label="Flexible dock — expanded (left edge)" bare>
          <div style={frameFlex}>
            <div style={{ flex: "0 0 16rem", minWidth: 0, minHeight: 0 }}>
              <DockRegion
                edge="left"
                sizing="flexible"
                items={flexItems}
                value={flexActivePanel}
                collapsed={false}
                onValueChange={(value) => setFlexActivePanel(value)}
              >
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>{flexActivePanel}</strong>
                    <p style={{ margin: 0 }}>Panel content for the active tab. Tabs are closable and reorderable.</p>
                  </div>
                )}
              </DockRegion>
            </div>
            <div style={flexMain}>Main content area</div>
          </div>
        </SpecimenGroup>

        {/* 1c. Tab pass-throughs (g13-040) */}
        <SpecimenGroup label="Tab pass-throughs — no underline, no reorder, solid fill (g13-040)" bare>
          <div style={dndLayout}>
            <div style={frameDnd}>
              <DockRegion
                edge="left"
                sizing="flexible"
                items={flexItems}
                value={flexActivePanel}
                tabActiveEdge="none"
                onValueChange={(value) => setFlexActivePanel(value)}
              >
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>{flexActivePanel}</strong>
                    <p style={{ margin: 0 }}>
                      tabActiveEdge="none" — no active underline; the tint fill alone marks selection.
                    </p>
                  </div>
                )}
              </DockRegion>
            </div>
            <div style={frameDnd}>
              <DockRegion
                edge="left"
                sizing="flexible"
                items={flexItems}
                value={flexActivePanel}
                tabReorderable={false}
                onValueChange={(value) => setFlexActivePanel(value)}
              >
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>{flexActivePanel}</strong>
                    <p style={{ margin: 0 }}>tabReorderable={false} — tabs cannot be dragged out of order.</p>
                  </div>
                )}
              </DockRegion>
            </div>
            <div style={frameDnd}>
              <DockRegion
                edge="left"
                sizing="flexible"
                items={flexItems}
                value={flexActivePanel}
                tabVariant="pill"
                tabActiveFill="solid"
                onValueChange={(value) => setFlexActivePanel(value)}
              >
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>{flexActivePanel}</strong>
                    <p style={{ margin: 0 }}>
                      tabVariant="pill" + tabActiveFill="solid" — a fill combination that was previously
                      unreachable through DockRegion.
                    </p>
                  </div>
                )}
              </DockRegion>
            </div>
          </div>
        </SpecimenGroup>

        {/* 2. Flexible dock (collapsed icon-strip) */}
        <SpecimenGroup label="Flexible dock — collapsed icon-strip (left edge)" bare>
          <div style={frameFlex}>
            <div style={{ flex: "0 0 auto", minWidth: 0, minHeight: 0 }}>
              <DockRegion
                edge="left"
                sizing="flexible"
                items={flexItems}
                value={flexActivePanel}
                collapsed={true}
                collapsedPosture="icon-strip"
                onValueChange={(value) => setFlexActivePanel(value)}
              />
            </div>
            <div style={flexMain}>Main content area</div>
          </div>
        </SpecimenGroup>

        {/* 3. Interactive collapse toggle */}
        <SpecimenGroup label="Interactive collapse toggle (click to toggle)" bare>
          <div style={frameFlex}>
            <div
              style={{ flex: interactiveCollapsed ? "0 0 auto" : "0 0 16rem", minWidth: 0, minHeight: 0 }}
            >
              <DockRegion
                edge="left"
                sizing="flexible"
                collapsible
                items={interactiveItems}
                value={interactiveActive}
                collapsed={interactiveCollapsed}
                collapsedPosture="icon-strip"
                onValueChange={(value) => setInteractiveActive(value)}
                onCollapsedChange={(isCollapsed) => setInteractiveCollapsed(isCollapsed)}
              >
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>{interactiveActive}</strong>
                    <p style={{ margin: 0 }}>
                      Click the collapse toggle to switch between expanded and icon-strip modes.
                    </p>
                  </div>
                )}
              </DockRegion>
            </div>
            <div style={flexMain}>Main content area</div>
          </div>
        </SpecimenGroup>

        {/* 4. Bottom edge collapsible dock */}
        <SpecimenGroup label="Bottom edge dock (click to toggle)" bare>
          <div style={frameBottom}>
            <div style={flexMainBottom}>Editor area</div>
            <div style={{ flex: "0 0 auto", maxHeight: "10rem", minHeight: 0 }}>
              <DockRegion
                edge="bottom"
                sizing="flexible"
                collapsible
                items={bottomItems}
                value={bottomActive}
                collapsed={bottomCollapsed}
                collapsedPosture="icon-strip"
                onValueChange={(value) => setBottomActive(value)}
                onCollapsedChange={(isCollapsed) => setBottomCollapsed(isCollapsed)}
              >
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>{bottomActive}</strong>
                    <p style={{ margin: 0 }}>Bottom panel content. Collapses downward, keeping horizontal tabs.</p>
                  </div>
                )}
              </DockRegion>
            </div>
          </div>
        </SpecimenGroup>

        {/* 5. Cross-region drag-and-drop */}
        <SpecimenGroup label="Cross-region drag-and-drop (drag tabs between docks)" bare>
          <div style={dndLayout}>
            <div style={frameDnd}>
              <DockRegion
                edge="left"
                sizing="flexible"
                items={leftItems}
                value={leftActive}
                ariaLabel="Left dock"
                canAcceptPanel={canAcceptPanel}
                onValueChange={(value) => setLeftActive(value)}
                onReorder={handleLeftReorder}
                onPanelDrop={handleLeftDrop}
              >
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>{leftActive}</strong>
                    <p style={{ margin: 0 }}>Left dock — {leftItems.length} panels</p>
                  </div>
                )}
              </DockRegion>
            </div>
            <div style={frameDnd}>
              <DockRegion
                edge="right"
                sizing="flexible"
                items={rightItems}
                value={rightActive}
                ariaLabel="Right dock"
                canAcceptPanel={canAcceptPanel}
                onValueChange={(value) => setRightActive(value)}
                onReorder={handleRightReorder}
                onPanelDrop={handleRightDrop}
              >
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>{rightActive}</strong>
                    <p style={{ margin: 0 }}>Right dock — {rightItems.length} panels</p>
                  </div>
                )}
              </DockRegion>
            </div>
          </div>
        </SpecimenGroup>

        {/* 6a. Static dock — horizontal (top edge, panels stack vertically) */}
        <SpecimenGroup label="Static dock — horizontal (top edge)" bare>
          <div style={frameShort}>
            <DockRegion edge="top" sizing="static" items={staticItems} onReorder={handleStaticReorder}
              panel={(item) => <div className="poodle-specimen__static-panel">{item.label}</div>}
            />
          </div>
        </SpecimenGroup>

        {/* 6b. Static dock — vertical (left edge, panels stack horizontally) */}
        <SpecimenGroup label="Static dock — vertical (left edge)" bare>
          <div style={frameBase}>
            <DockRegion
              edge="left"
              sizing="static"
              items={staticVerticalItems}
              onReorder={handleStaticVerticalReorder}
              panel={(item) => <div className="poodle-specimen__static-panel">{item.label}</div>}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
