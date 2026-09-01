import { useState, type CSSProperties } from "react";
import { DockRegion, DragDropProvider } from "@inflatable-cookie/poodle-react";
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
const pairStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "1rem" };
const stackStyle: CSSProperties = { display: "flex", flexDirection: "column", gap: "1rem" };
const narrowFrame: CSSProperties = { maxWidth: "14rem" };

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
  const [flexItems, setFlexItems] = useState<PanelTabItem[]>([
    { value: "explorer", label: "Explorer", icon: "folder", closable: true },
    { value: "search", label: "Search", icon: "search", closable: true },
    { value: "git", label: "Source Control", icon: "code", closable: false },
  ]);

  const iconlessItems: PanelTabItem[] = [
    { value: "inspector", label: "Inspector", closable: false },
    { value: "browser", label: "Media Browser", closable: false },
    { value: "clips", label: "Clip Editor", closable: false },
  ];

  function handleFlexReorder(order: string[]): void {
    setFlexItems((current) => order.map((id) => current.find((i) => i.value === id)!));
  }

  function handleFlexClose(value: string): void {
    setFlexItems((current) => {
      const next = current.filter((i) => i.value !== value);
      if (flexActivePanel === value) {
        setFlexActivePanel(next[0]?.value ?? "");
      }
      return next;
    });
  }

  // ── Interactive collapse state ─────────────────────────────────────
  const [interactiveCollapsed, setInteractiveCollapsed] = useState(false);
  const [interactiveActive, setInteractiveActive] = useState("files");
  const [interactiveItems, setInteractiveItems] = useState<PanelTabItem[]>([
    { value: "files", label: "Files", icon: "folder", closable: true },
    { value: "outline", label: "Outline", icon: "list", closable: true },
    { value: "debug", label: "Debug", icon: "terminal", closable: false },
  ]);

  function handleInteractiveReorder(order: string[]): void {
    setInteractiveItems((current) => order.map((id) => current.find((i) => i.value === id)!));
  }

  function handleInteractiveClose(value: string): void {
    setInteractiveItems((current) => {
      const next = current.filter((i) => i.value !== value);
      if (interactiveActive === value) {
        setInteractiveActive(next[0]?.value ?? "");
      }
      return next;
    });
  }

  // ── Bottom dock state ───────────────────────────────────────────────
  const [bottomCollapsed, setBottomCollapsed] = useState(false);
  const [bottomActive, setBottomActive] = useState("terminal");
  const [bottomItems, setBottomItems] = useState<PanelTabItem[]>([
    { value: "terminal", label: "Terminal", icon: "terminal", closable: true },
    { value: "output", label: "Output", icon: "file-text", closable: true },
    { value: "problems", label: "Problems", icon: "alert-circle", closable: false },
  ]);

  function handleBottomReorder(order: string[]): void {
    setBottomItems((current) => order.map((id) => current.find((i) => i.value === id)!));
  }

  function handleBottomClose(value: string): void {
    setBottomItems((current) => {
      const next = current.filter((i) => i.value !== value);
      if (bottomActive === value) {
        setBottomActive(next[0]?.value ?? "");
      }
      return next;
    });
  }
  // ── Cross-region drag-and-drop state ───────────────────────────────
  const [leftItems, setLeftItems] = useState<PanelTabItem[]>([
    { value: "explorer", label: "Explorer", icon: "folder", closable: false },
    { value: "search", label: "Search", icon: "search", closable: false },
    { value: "git", label: "Source Control", icon: "code", closable: false },
  ]);
  const [rightItems, setRightItems] = useState<PanelTabItem[]>([
    { value: "outline", label: "Outline", icon: "list", closable: false },
  ]);
  const [leftActive, setLeftActive] = useState("explorer");
  const [rightActive, setRightActive] = useState("outline");

  const axisItems: PanelTabItem[] = [
    { value: "explorer", label: "Explorer", icon: "folder", closable: false },
    { value: "search", label: "Search", icon: "search", closable: false },
    { value: "git", label: "Source Control", icon: "code", closable: false },
  ];

  function canAcceptPanel(_panelId: string, _sourceEdge: DockEdge): boolean {
    return true;
  }

  function insertAt<T>(list: T[], item: T, index: number): T[] {
    const at = Math.max(0, Math.min(index, list.length));
    return [...list.slice(0, at), item, ...list.slice(at)];
  }

  function handleLeftDrop({
    panel,
    index,
  }: {
    panel: { panelId: string; sourceEdge: DockEdge };
    targetEdge: DockEdge;
    index: number;
  }): void {
    const { panelId, sourceEdge } = panel;
    if (sourceEdge === "right") {
      const item = rightItems.find((i) => i.value === panelId);
      if (!item) return;
      setRightItems((current) => current.filter((i) => i.value !== panelId));
      setLeftItems((current) => insertAt(current, item, index));
      if (rightActive === panelId) {
        setRightActive(rightItems.find((i) => i.value !== panelId)?.value ?? "");
      }
    }
  }

  function handleRightDrop({
    panel,
    index,
  }: {
    panel: { panelId: string; sourceEdge: DockEdge };
    targetEdge: DockEdge;
    index: number;
  }): void {
    const { panelId, sourceEdge } = panel;
    if (sourceEdge === "left") {
      const item = leftItems.find((i) => i.value === panelId);
      if (!item) return;
      setLeftItems((current) => current.filter((i) => i.value !== panelId));
      setRightItems((current) => insertAt(current, item, index));
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
              <DockRegion edge="left" sizing="flexible" items={axisItems} value="git" size={size}>
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>git</strong>
                    <p style={{ margin: 0 }}>
                      Size axis — presentation only; close and reorder live in the Examples tab.
                    </p>
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
              <DockRegion edge="left" sizing="flexible" items={axisItems} value="git" density={density}>
                {() => (
                  <div style={panelContent}>
                    <strong style={panelStrong}>git</strong>
                    <p style={{ margin: 0 }}>
                      Density axis — presentation only; close and reorder live in the Examples tab.
                    </p>
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
        <SpecimenGroup label="Expanded side dock" bare>
          <div style={pairStyle}>
            <div style={frameFlex}>
              <div style={{ flex: "0 0 16rem", minWidth: 0, minHeight: 0 }}>
                <DockRegion
                  edge="left"
                  sizing="flexible"
                  items={flexItems}
                  value={flexActivePanel}
                  collapsed={false}
                  onValueChange={(value) => setFlexActivePanel(value)}
                  onReorder={handleFlexReorder}
                  onClose={handleFlexClose}
                >
                  {() => (
                    <div style={panelContent}>
                      <strong style={panelStrong}>{flexActivePanel}</strong>
                      <p style={{ margin: 0 }}>
                        Panel content for the active tab. Tabs are closable and reorderable.
                      </p>
                    </div>
                  )}
                </DockRegion>
              </div>
              <div style={flexMain}>Main content area</div>
            </div>
            <div style={{ ...frameFlex, ...narrowFrame }}>
              <div style={{ flex: "0 0 16rem", minWidth: 0, minHeight: 0 }}>
                <DockRegion edge="left" sizing="flexible" items={iconlessItems} value="inspector">
                  {() => (
                    <div style={panelContent}>
                      <strong style={panelStrong}>Inspector</strong>
                      <p style={{ margin: 0 }}>
                        Panels without icons keep their labels when the strip is squeezed.
                      </p>
                    </div>
                  )}
                </DockRegion>
              </div>
              <div style={flexMain}>Main content area</div>
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Collapse and edge placement" bare>
          <div style={stackStyle}>
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
                  onReorder={handleFlexReorder}
                  onClose={handleFlexClose}
                />
              </div>
              <div style={flexMain}>Main content area</div>
            </div>
            <div style={frameFlex}>
              <div
                style={{
                  flex: interactiveCollapsed ? "0 0 auto" : "0 0 16rem",
                  minWidth: 0,
                  minHeight: 0,
                }}
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
                  onReorder={handleInteractiveReorder}
                  onClose={handleInteractiveClose}
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
                  onReorder={handleBottomReorder}
                  onClose={handleBottomClose}
                >
                  {() => (
                    <div style={panelContent}>
                      <strong style={panelStrong}>{bottomActive}</strong>
                      <p style={{ margin: 0 }}>
                        Bottom panel content. Collapses downward, keeping horizontal tabs.
                      </p>
                    </div>
                  )}
                </DockRegion>
              </div>
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Tab strip presentation" bare>
          <div style={dndLayout}>
            <div style={frameDnd}>
              <DockRegion
                edge="left"
                sizing="flexible"
                items={flexItems}
                value={flexActivePanel}
                tabActiveEdge="none"
                onValueChange={(value) => setFlexActivePanel(value)}
                  onReorder={handleFlexReorder}
                  onClose={handleFlexClose}
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
                  onReorder={handleFlexReorder}
                  onClose={handleFlexClose}
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
                  onReorder={handleFlexReorder}
                  onClose={handleFlexClose}
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

        <SpecimenGroup label="Move panels between docks" bare>
          {/* One provider around both docks. Cross-region transfer is ordinary
              controller scope now: two regions see each other's targets only
              when a single controller holds both registrations. */}
          <DragDropProvider>
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
          </DragDropProvider>
        </SpecimenGroup>

        <SpecimenGroup label="Static panel stacks" bare>
          <div style={stackStyle}>
            <div style={frameShort}>
              <DockRegion
                edge="top"
                sizing="static"
                items={staticItems}
                onReorder={handleStaticReorder}
                panel={(item) => <div className="poodle-specimen__static-panel">{item.label}</div>}
              />
            </div>
            <div style={frameBase}>
              <DockRegion
                edge="left"
                sizing="static"
                items={staticVerticalItems}
                onReorder={handleStaticVerticalReorder}
                panel={(item) => <div className="poodle-specimen__static-panel">{item.label}</div>}
              />
            </div>
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
