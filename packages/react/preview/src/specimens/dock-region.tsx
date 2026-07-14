import { useState } from "react";
import { DockRegion, SegmentedControl, Tabs, type PanelTabItem, type TabItem } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const initialPanels: PanelTabItem[] = [
  { value: "explorer", label: "Explorer", icon: "folder" },
  { value: "search", label: "Search", icon: "search" },
  { value: "output", label: "Output", icon: "terminal", closable: true },
];

const richTabs: TabItem[] = [
  { value: "overview", label: "Overview", icon: "layout-dashboard", count: 3 },
  { value: "activity", label: "Activity", count: 12 },
  { value: "settings", label: "Settings", closable: true },
];

function DockRegionDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [panels, setPanels] = useState(initialPanels);
  const [active, setActive] = useState("explorer");
  const [dockCollapsed, setDockCollapsed] = useState(false);

  return (
    <>
      <SpecimenSection title="Tabs (full: icons, counts, closable, reorderable)">
        <Tabs
          items={richTabs}
          reorderable
          ariaLabel="Project sections"
          onValueChange={(value) => setLastEvent(`tabs:${value}`)}
          onClose={(value) => setLastEvent(`tabs:close:${value}`)}
          onReorder={(order) => setLastEvent(`tabs:reorder:${order.join(",")}`)}
        >
          {(value) => <p data-testid="tab-panel">Panel: {value}</p>}
        </Tabs>
      </SpecimenSection>

      <SpecimenSection title="SegmentedControl (icon + iconOnly options)">
        <SegmentedControl
          ariaLabel="View mode"
          options={[
            { value: "list", label: "List", icon: "list", iconOnly: true },
            { value: "grid", label: "Grid", icon: "grid-2x2", iconOnly: true },
            { value: "table", label: "Table", icon: "table" },
          ]}
          defaultValue="list"
          onValueChange={(value) => setLastEvent(`seg:${value}`)}
        />
      </SpecimenSection>

      <SpecimenSection title="DockRegion (flexible, collapsible)">
        <div style={{ height: "14rem", display: "flex", border: "1px solid var(--poodle-color-border-subtle)" }}>
          <DockRegion
            edge="left"
            collapsible
            collapsed={dockCollapsed}
            items={panels}
            value={active}
            tabVariant="strip"
            onValueChange={(value) => {
              setActive(value);
              setLastEvent(`dock:${value}`);
            }}
            onCollapsedChange={(next) => {
              setDockCollapsed(next);
              setLastEvent(`dock:collapsed:${next}`);
            }}
            onClose={(value) => {
              setPanels((prev) => prev.filter((p) => p.value !== value));
              setLastEvent(`dock:close:${value}`);
            }}
            onReorder={(order) => {
              setPanels((prev) => order.map((v) => prev.find((p) => p.value === v)!).filter(Boolean));
              setLastEvent(`dock:reorder:${order.join(",")}`);
            }}
          >
            {(item) => <div data-testid="dock-body">Body: {item?.label ?? "none"}</div>}
          </DockRegion>
          <div style={{ flex: 1, padding: "0.5rem" }}>Main content</div>
        </div>
      </SpecimenSection>

      <SpecimenSection title="DockRegion (static stack)">
        <DockRegion
          edge="bottom"
          sizing="static"
          items={panels}
          panel={(item) => <div data-testid={`stack-${item.value}`}>{item.label} panel</div>}
        />
      </SpecimenSection>

      {lastEvent ? (
        <SpecimenSection title="Last event">
          <p data-testid="last-event">{lastEvent}</p>
        </SpecimenSection>
      ) : null}
    </>
  );
}

registerSpecimen({
  slug: "dock-region",
  title: "DockRegion / Tabs full / SegmentedControl icons",
  render: () => <DockRegionDemo />,
});
