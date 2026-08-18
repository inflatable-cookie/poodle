import { useState, type CSSProperties } from "react";
import { Tabs, type TabItem } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const sectionTabs: TabItem[] = [
  { value: "details", label: "Details" },
  { value: "usage", label: "Usage", count: 12, separator: true },
  { value: "versions", label: "Versions", count: 3 },
  { value: "archive", label: "Archive", disabled: true },
];

const surfaceTabs: TabItem[] = [
  { value: "editor", label: "Editor", icon: "code" },
  { value: "preview", label: "Preview", icon: "eye" },
  { value: "terminal", label: "Terminal", icon: "terminal", closable: true },
  { value: "output", label: "Output", icon: "file-text", closable: true },
];

const fileTabs: TabItem[] = [
  { value: "index.ts", label: "index.ts" },
  { value: "App.svelte", label: "App.svelte", closable: true },
  { value: "utils.ts", label: "utils.ts", closable: true },
  { value: "types.ts", label: "types.ts", closable: true },
];

const panelTabs: TabItem[] = [
  { value: "explorer", label: "Explorer", icon: "folder" },
  { value: "search", label: "Search", icon: "search" },
  { value: "git", label: "Source Control", icon: "layers" },
  { value: "debug", label: "Debug", icon: "terminal" },
];

// Four tabs with icons and counts — the shape that collapsed far too early.
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

const surfaceBodyStyle: CSSProperties = {
  display: "flex",
  flex: 1,
  alignItems: "center",
  justifyContent: "center",
  minHeight: "8rem",
  color: "var(--poodle-color-text-muted)",
  fontSize: "0.8125rem",
  background: "var(--poodle-color-background-panel)",
};

const resizableStyle: CSSProperties = {
  resize: "horizontal",
  overflow: "auto",
  minWidth: "12rem",
  maxWidth: "48rem",
  width: "min(34rem, 100%)",
  border: "0.0625rem dashed var(--poodle-color-border-subtle)",
  padding: "0.5rem",
};

const noteStyle: CSSProperties = {
  margin: 0,
  color: "var(--poodle-color-text-secondary)",
  fontSize: "0.8125rem",
};

const axisStyle: CSSProperties = { width: "min(100%, 28rem)" };

export function TabsSpecimen() {
  const [lastClosed, setLastClosed] = useState("");
  const [lastReorder, setLastReorder] = useState("");

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={axisStyle}>
          <Tabs items={sectionTabs} variant="card" defaultValue="details" ariaLabel={`${size} tabs`} size={size} />
        </div>
      )}
      densities={(density) => (
        <div style={axisStyle}>
          <Tabs items={sectionTabs} variant="card" defaultValue="details" ariaLabel={`${density} tabs`} density={density} />
        </div>
      )}
    >
      <div style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        <SpecimenGroup label="Tabs over a panel — counts, a separator, and one disabled tab">
          <Tabs
            items={sectionTabs}
            defaultValue="details"
            bordered
            historyKey="tab"
            ariaLabel="Detail sections"
          >
            {(activeValue) => (
              <p>
                Active tab: <strong>{activeValue}</strong>
              </p>
            )}
          </Tabs>

          {/* `bordered` is the difference between tabs that sit above content
              and tabs that sit flush in a titlebar or toolbar. */}
          <p style={noteStyle}>
            Without <code>bordered</code>, for titlebars and toolbars where the tabs are not above content:
          </p>
          <Tabs items={sectionTabs} defaultValue="details" bordered={false} ariaLabel="Flush section tabs" />
        </SpecimenGroup>

        <SpecimenGroup label="Variants — card, pill, and block">
          <Tabs items={sectionTabs} variant="card" defaultValue="details" ariaLabel="Card tabs" />
          <Tabs items={sectionTabs} variant="pill" defaultValue="details" ariaLabel="Pill tabs" />
          <div style={frameStyle}>
            <Tabs items={surfaceTabs} variant="block" defaultValue="editor" ariaLabel="Block tabs" />
          </div>
        </SpecimenGroup>

        {/* activeEdge and activeFill are variant-agnostic, so one variant is
            enough to teach them. Showing the full product was six groups. */}
        <SpecimenGroup label="Marking the active tab — an edge, a fill, or both">
          <Tabs items={sectionTabs} variant="pill" activeEdge="outline" defaultValue="details" ariaLabel="Outlined tabs" />
          <Tabs items={sectionTabs} variant="pill" activeFill="solid" defaultValue="details" ariaLabel="Solid tabs" />
          <div style={frameStyle}>
            <Tabs
              items={surfaceTabs}
              variant="block"
              activeEdge="underline"
              activeFill="none"
              defaultValue="editor"
              ariaLabel="Underlined tabs"
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Editable tabs — close one, or drag to reorder">
          <Tabs
            items={fileTabs}
            variant="card"
            defaultValue="App.svelte"
            reorderable
            ariaLabel="Open files"
            onClose={(value) => setLastClosed(value)}
            onReorder={(items) => setLastReorder(items.join(", "))}
          />
          {lastClosed && (
            <p style={noteStyle}>
              Closed: <strong>{lastClosed}</strong>
            </p>
          )}
          {lastReorder && (
            <p style={noteStyle}>
              Reordered: <strong>{lastReorder}</strong>
            </p>
          )}
        </SpecimenGroup>

        <SpecimenGroup label="When the row runs out of space — drag the right edge">
          {/* Figmatic's case: a pane whose width the operator drags. Rather than
              one threshold into a menu, the strip gives up icons, then counts,
              then collapses — each at the width where it actually stops fitting,
              so label length and count magnitude move the points on their own. */}
          <div style={resizableStyle}>
            <Tabs items={shedItems} overflowStrategy="shed" collapseWhenOverflow ariaLabel="Graded overflow" />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Vertical — a side panel's tab rail">
          <div style={frameRowStyle}>
            <Tabs
              items={panelTabs}
              variant="block"
              activeEdge="underline"
              orientation="vertical"
              defaultValue="explorer"
              ariaLabel="Side panel tabs"
            />
            <div style={surfaceBodyStyle}>
              <p>Panel content</p>
            </div>
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
