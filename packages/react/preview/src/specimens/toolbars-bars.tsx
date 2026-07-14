import { useState } from "react";
import {
  BulkActionBar,
  Button,
  CollapseToggle,
  FilterToolbar,
  IconButton,
  MetaBar,
  MetaItem,
  PageLoading,
  Pill,
  Select,
  SelectionSummary,
  TextInput,
  Toolbar,
  type BulkAction,
  type SelectionSummaryItem,
} from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const bulkActions: BulkAction[] = [
  { id: "archive", label: "Archive", icon: "archive" },
  { id: "tag", label: "Tag", icon: "tag" },
  { id: "delete", label: "Delete", icon: "trash-2", tone: "danger" },
];

function ToolbarsBarsDemo() {
  const [lastEvent, setLastEvent] = useState("");
  const [collapsed, setCollapsed] = useState(false);
  const [sideCollapsed, setSideCollapsed] = useState(false);
  const [selection, setSelection] = useState<SelectionSummaryItem[]>([
    { id: "1", label: "Alpha" },
    { id: "2", label: "Beta" },
    { id: "3", label: "Gamma" },
    { id: "4", label: "Delta" },
    { id: "5", label: "Epsilon" },
    { id: "6", label: "Zeta" },
  ]);
  const [loadingVisible, setLoadingVisible] = useState(false);
  const [bulkCount, setBulkCount] = useState(3);

  return (
    <>
      <SpecimenSection title="Toolbar">
        <Toolbar ariaLabel="Formatting">
          <IconButton icon="bold" ariaLabel="Bold" variant="ghost" />
          <IconButton icon="italic" ariaLabel="Italic" variant="ghost" />
          <IconButton icon="underline" ariaLabel="Underline" variant="ghost" />
          <Button size="sm" variant="secondary">
            Insert
          </Button>
        </Toolbar>
      </SpecimenSection>

      <SpecimenSection title="FilterToolbar">
        <FilterToolbar
          summaryText="2 filters active"
          collapsed={collapsed}
          onCollapsedChange={(next) => {
            setCollapsed(next);
            setLastEvent(`filters:${next ? "collapsed" : "expanded"}`);
          }}
          actions={
            <Button size="sm" variant="ghost" onClick={() => setLastEvent("filters:reset")}>
              Reset
            </Button>
          }
        >
          <TextInput ariaLabel="Search" placeholder="Search…" />
          <Select
            ariaLabel="Status"
            options={[
              { value: "all", label: "All" },
              { value: "open", label: "Open" },
            ]}
            value="all"
          />
        </FilterToolbar>
      </SpecimenSection>

      <SpecimenSection title="MetaBar">
        <MetaBar ariaLabel="Document meta">
          <MetaItem label="Owner">Tom</MetaItem>
          <MetaItem label="Updated">2h ago</MetaItem>
          <Pill tone="success">Live</Pill>
        </MetaBar>
      </SpecimenSection>

      <SpecimenSection title="BulkActionBar">
        <BulkActionBar
          selectionCount={bulkCount}
          totalCount={48}
          actions={bulkActions}
          showSelectAll={bulkCount < 48}
          onAction={(id) => setLastEvent(`bulk:${id}`)}
          onClear={() => {
            setBulkCount(0);
            setLastEvent("bulk:clear");
          }}
          onSelectAll={() => {
            setBulkCount(48);
            setLastEvent("bulk:select-all");
          }}
        />
      </SpecimenSection>

      <SpecimenSection title="SelectionSummary">
        <SelectionSummary
          items={selection}
          maxVisibleItems={4}
          onRemove={(id) => {
            setSelection((prev) => prev.filter((item) => item.id !== id));
            setLastEvent(`selection:remove:${id}`);
          }}
          onClear={() => {
            setSelection([]);
            setLastEvent("selection:clear");
          }}
        />
      </SpecimenSection>

      <SpecimenSection title="CollapseToggle">
        <CollapseToggle
          collapsed={sideCollapsed}
          direction="left"
          onToggle={(next) => {
            setSideCollapsed(next);
            setLastEvent(`side:${next ? "collapsed" : "expanded"}`);
          }}
        />
      </SpecimenSection>

      <SpecimenSection title="PageLoading">
        <Button onClick={() => setLoadingVisible(true)}>Show overlay</Button>
        <div style={{ position: "relative", minHeight: "8rem" }}>
          <PageLoading presentation="inline" value={64} message="Importing records…" canCancel onCancel={() => setLastEvent("loading:cancel")} />
        </div>
        {loadingVisible ? (
          <PageLoading
            message="Loading workspace…"
            canCancel
            onCancel={() => {
              setLoadingVisible(false);
              setLastEvent("overlay:cancel");
            }}
          />
        ) : null}
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
  slug: "toolbars-bars",
  title: "Toolbar / FilterToolbar / BulkActionBar / StatusBars",
  render: () => <ToolbarsBarsDemo />,
});
