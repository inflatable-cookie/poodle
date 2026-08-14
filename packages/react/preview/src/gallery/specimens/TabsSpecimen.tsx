import { useState, type CSSProperties } from "react";
import { Tabs, type TabItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const basicTabs: TabItem[] = [
  { value: "overview", label: "Overview" },
  { value: "features", label: "Features" },
  { value: "pricing", label: "Pricing" },
  { value: "faq", label: "FAQ", disabled: true },
];

const iconTabs: TabItem[] = [
  { value: "home", label: "Home", icon: "house" },
  { value: "settings", label: "Settings", icon: "settings" },
  { value: "users", label: "Users", icon: "users" },
];

const closableTabs: TabItem[] = [
  { value: "index.ts", label: "index.ts" },
  { value: "App.svelte", label: "App.svelte", closable: true },
  { value: "utils.ts", label: "utils.ts", closable: true },
  { value: "types.ts", label: "types.ts", closable: true },
];

const stripTabs: TabItem[] = [
  { value: "editor", label: "Editor", icon: "code" },
  { value: "preview", label: "Preview", icon: "eye" },
  { value: "terminal", label: "Terminal", icon: "terminal", closable: true },
  { value: "output", label: "Output", icon: "file-text", closable: true },
];

const panelTabs: TabItem[] = [
  { value: "explorer", label: "Explorer", icon: "folder", closable: true },
  { value: "search", label: "Search", icon: "search", closable: true },
  { value: "git", label: "Source Control", icon: "layers", closable: true },
  { value: "debug", label: "Debug", icon: "terminal", closable: true },
];

const detailTabs: TabItem[] = [
  { value: "details", label: "Details" },
  { value: "usage", label: "Usage", count: 12, separator: true },
  { value: "versions", label: "Versions", count: 3 },
];

const shedItems: TabItem[] = [
  { value: "screens", label: "Screens", icon: "monitor", count: 12 },
  { value: "components", label: "Components", icon: "box", count: 12 },
  { value: "assets", label: "Assets", icon: "image", count: 375 },
  { value: "info", label: "Info", icon: "info" },
];

const frameStyle: CSSProperties = {
  border: "0.0625rem solid var(--poodle-color-border-subtle)",
  borderRadius: "var(--poodle-radius-surface)",
  overflow: "hidden",
};

const frameRowStyle: CSSProperties = {
  ...frameStyle,
  display: "flex",
  flexDirection: "row",
};

const variantsDemoStyle: CSSProperties = { width: "min(100%, 28rem)" };

const surfaceBodyStyle: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  height: "6rem",
  color: "var(--poodle-color-text-muted)",
  fontSize: "0.8125rem",
  background: "var(--poodle-color-background-panel)",
};

const surfaceBodyFillStyle: CSSProperties = {
  ...surfaceBodyStyle,
  flex: 1,
  height: "auto",
  minHeight: "8rem",
};

const panelExpandedStyle: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  flex: 1,
  minWidth: 0,
};

const collapseBtnStyle: CSSProperties = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  width: "1.5rem",
  minHeight: 0,
  padding: 0,
  border: 0,
  borderLeft: "0.0625rem solid var(--poodle-color-border-subtle)",
  background: "var(--poodle-color-background-surface)",
  color: "var(--poodle-color-text-muted)",
  cursor: "pointer",
  fontSize: "0.75rem",
};

export function TabsSpecimen() {
  const [lastClosed, setLastClosed] = useState("");
  const [lastReorder, setLastReorder] = useState("");
  const [panelCollapsed, setPanelCollapsed] = useState(false);
  const [collapseHovered, setCollapseHovered] = useState(false);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={variantsDemoStyle}>
          <Tabs items={detailTabs} variant="card" defaultValue="details" ariaLabel={`${size} tabs`} size={size} />
        </div>
      )}
      densities={(density) => (
        <div style={variantsDemoStyle}>
          <Tabs items={detailTabs} variant="card" defaultValue="details" ariaLabel={`${density} tabs`} density={density} />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Graded overflow (drag the handle)">
          <div
            style={{
              resize: "horizontal",
              overflow: "auto",
              minWidth: "12rem",
              maxWidth: "48rem",
              width: "34rem",
              border: "1px dashed var(--poodle-color-border-subtle)",
              padding: "0.5rem",
            }}
          >
            <Tabs items={shedItems} overflowStrategy="shed" collapseWhenOverflow ariaLabel="Graded overflow" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Card variant (default, with indicator line)">
          <Tabs items={basicTabs} defaultValue="overview" bordered ariaLabel="Section tabs">
            {(activeValue) => (
              <p>
                Active tab: <strong>{activeValue}</strong>
              </p>
            )}
          </Tabs>
        </SpecimenGroup>

        <SpecimenGroup label="Card variant (no border)">
          <Tabs items={basicTabs} defaultValue="overview" bordered={false} ariaLabel="Section tabs without border" />
        </SpecimenGroup>

        <SpecimenGroup label="Card variant (closable, reorderable)">
          <Tabs
            items={closableTabs}
            variant="card"
            defaultValue="App.svelte"
            reorderable
            ariaLabel="Open files"
            onClose={(value) => setLastClosed(value)}
            onReorder={(items) => setLastReorder(items.join(", "))}
          />
          {lastClosed ? (
            <p>
              Closed: <strong>{lastClosed}</strong>
            </p>
          ) : null}
          {lastReorder ? (
            <p>
              Reordered: <strong>{lastReorder}</strong>
            </p>
          ) : null}
        </SpecimenGroup>

        <SpecimenGroup label="Card variant (active outline)">
          <Tabs items={basicTabs} variant="card" activeEdge="outline" defaultValue="overview" ariaLabel="Outlined section tabs" />
        </SpecimenGroup>

        <SpecimenGroup label="Card variant (solid fill)">
          <Tabs items={basicTabs} variant="card" activeFill="solid" defaultValue="overview" ariaLabel="Solid section tabs" />
        </SpecimenGroup>

        {/* The edges are variant-agnostic, so every variant needs coverage.
            Only card had it, which is why the block hover revert shipped
            unseen. */}
        <SpecimenGroup label="Pill variant (active outline)">
          <Tabs items={basicTabs} variant="pill" activeEdge="outline" defaultValue="overview" ariaLabel="Outlined pill tabs" />
        </SpecimenGroup>

        <SpecimenGroup label="Pill variant (solid fill)">
          <Tabs items={basicTabs} variant="pill" activeFill="solid" defaultValue="overview" ariaLabel="Solid pill tabs" />
        </SpecimenGroup>

        <SpecimenGroup label="Block variant (solid fill — hover the active tab)">
          <div style={frameStyle}>
            <Tabs items={stripTabs} variant="block" activeFill="solid" defaultValue="editor" ariaLabel="Solid block tabs" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Block variant (active outline)">
          <div style={frameStyle}>
            <Tabs items={stripTabs} variant="block" activeEdge="outline" defaultValue="editor" ariaLabel="Outlined block tabs" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Block variant (active underline, no fill — the former strip)">
          <div style={frameStyle}>
            <Tabs items={stripTabs} variant="block" activeEdge="underline" activeFill="none" defaultValue="editor" ariaLabel="Strip-equivalent block tabs" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Block variant (full-width shell tabs with separators)">
          <div style={frameStyle}>
            <Tabs items={stripTabs} variant="block" defaultValue="editor" reorderable ariaLabel="Workspace surfaces" />
            <div style={surfaceBodyStyle}>
              <p>Surface content area</p>
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Pill variant (with icons)">
          <Tabs items={iconTabs} variant="pill" defaultValue="home" ariaLabel="Navigation" />
        </SpecimenGroup>

        <SpecimenGroup label="Card variant (with icons, no panel)">
          <Tabs items={iconTabs} defaultValue="home" ariaLabel="Icon tabs" />
        </SpecimenGroup>

        <SpecimenGroup label="Block variant (full-width bar with icons, closable, reorderable)">
          <div style={frameStyle}>
            <Tabs
              items={stripTabs}
              variant="block"
              activeEdge="underline"
              defaultValue="editor"
              reorderable
              ariaLabel="Workspace surfaces"
              onClose={(value) => setLastClosed(value)}
              onReorder={(items) => setLastReorder(items.join(", "))}
            />
            <div style={surfaceBodyStyle}>
              <p>Surface content area</p>
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Block variant — vertical (icon-only, collapsed panel)">
          <div style={frameRowStyle}>
            <Tabs
              items={panelTabs}
              variant="block"
              activeEdge="underline"
              orientation="vertical"
              defaultValue="explorer"
              ariaLabel="Side panel tabs"
            />
            <div style={surfaceBodyFillStyle}>
              <p>Panel content</p>
            </div>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Block variant — collapse toggle (click to toggle orientation)">
          <div style={frameRowStyle}>
            {!panelCollapsed ? (
              <div style={panelExpandedStyle}>
                <Tabs
                  items={panelTabs}
                  variant="block"
                  activeEdge="underline"
                  orientation="horizontal"
                  defaultValue="explorer"
                  reorderable
                  ariaLabel="Side panel tabs"
                  onClose={(value) => setLastClosed(value)}
                />
                <div style={surfaceBodyFillStyle}>
                  <p>Panel body — expanded</p>
                </div>
              </div>
            ) : (
              <Tabs
                items={panelTabs}
                variant="block"
                activeEdge="underline"
                orientation="vertical"
                defaultValue="explorer"
                ariaLabel="Side panel tabs"
              />
            )}
            <button
              style={
                collapseHovered
                  ? {
                      ...collapseBtnStyle,
                      background: "var(--poodle-color-surface-hover)",
                      color: "var(--poodle-color-text-primary)",
                    }
                  : collapseBtnStyle
              }
              onMouseEnter={() => setCollapseHovered(true)}
              onMouseLeave={() => setCollapseHovered(false)}
              onClick={() => setPanelCollapsed((value) => !value)}
            >
              {panelCollapsed ? "→" : "←"}
            </button>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Card variant with counts, separators, and URL sync">
          <Tabs items={detailTabs} variant="card" defaultValue="details" bordered historyKey="tab" ariaLabel="Detail sections">
            {(activeValue) => (
              <p>
                Active tab: <strong>{activeValue}</strong>
              </p>
            )}
          </Tabs>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
