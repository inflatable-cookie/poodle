import { useState } from "react";
import { Icon, IconButton, ListCard, ListCardCounter, Pill, type MenuItem } from "@poodle/react";
import { registerSpecimen, SpecimenSection } from "../harness";

const contextItems: MenuItem[] = [
  { label: "Open", value: "open" },
  { label: "Rename", value: "rename" },
  { label: "Duplicate", value: "duplicate" },
  { kind: "separator", label: "", value: "sep-1" },
  { label: "Delete", value: "delete", tone: "danger" },
];

function ListCardDemo() {
  const [lastClick, setLastClick] = useState("");
  const [selectedCard, setSelectedCard] = useState(false);

  return (
    <>
      <SpecimenSection title="Interactive list cards">
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <ListCard
            title="design-system-v2.figma"
            subtitle="Updated by Clay · 2h ago"
            meta="14.2 MB"
            interactive
            onClick={() => setLastClick("design-system-v2.figma")}
            leading={<Icon name="folder" />}
          />
          <ListCard
            title="component-specs.pdf"
            subtitle="Shared with team · Yesterday"
            meta="2.8 MB"
            interactive
            onClick={() => setLastClick("component-specs.pdf")}
            leading={<Icon name="file-text" />}
            corner={<Icon name="lock" ariaLabel="Restricted" size="sm" />}
          />
          <ListCard title="brand-assets.zip" subtitle="Archived" meta="48 MB" disabled leading={<Icon name="layers" />} />
        </div>
      </SpecimenSection>

      <SpecimenSection title="Link root, badges, actions">
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <ListCard
            title="Billing settings"
            subtitle="Manage invoices and payment methods"
            href="#billing"
            leading={<Icon name="credit-card" />}
            badges={
              <Pill appearance="badge" tone="neutral">
                2
              </Pill>
            }
          />
          <ListCard
            title="Project Alpha"
            subtitle="Explicit actions lane"
            interactive
            onClick={() => setLastClick("Project Alpha")}
            leading={<Icon name="folder" />}
            actions={<IconButton icon="ellipsis" ariaLabel="Project actions" variant="ghost" />}
          />
        </div>
      </SpecimenSection>

      <SpecimenSection title="Selectable and reorder">
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <ListCard
            title="Selected row"
            subtitle="Batch-selection ready"
            selectable
            selected={selectedCard}
            selectionIndicator="checkbox"
            onSelectedChange={setSelectedCard}
            badges={<Pill tone="success">Chosen</Pill>}
          />
          <ListCard
            title="Compact reorder item"
            layout="compact"
            showReorderHandle
            leading={<Icon name="grip" />}
            badges={
              <Pill appearance="badge" tone="neutral">
                Draft
              </Pill>
            }
          />
        </div>
      </SpecimenSection>

      <SpecimenSection title="Stacked layout">
        <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(16rem, 1fr))", gap: "0.75rem" }}>
          <ListCard
            title="Release notes"
            subtitle="Summarize the latest component changes."
            layout="stacked"
            leadingShape="rounded-square"
            leadingFill="solid"
            accentColor="#6366f1"
            interactive
            leading={<Icon name="layers" />}
            trailing={<Pill tone="success">Published</Pill>}
          />
          <ListCard
            title="Media library"
            subtitle="Review uploads and approve assets."
            layout="stacked"
            leadingShape="rounded-square"
            interactive
            leading={<Icon name="image" />}
            footer={
              <>
                <ListCardCounter icon="image" count={18} tooltip="18 images" />
                <ListCardCounter icon="file-text" count={6} tooltip="6 notes" />
              </>
            }
            trailing={<IconButton icon="ellipsis" ariaLabel="Media library actions" variant="ghost" />}
          />
        </div>
      </SpecimenSection>

      <SpecimenSection title="Context menu (right-click)">
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <ListCard
            title="Right-click for actions"
            subtitle="Context menu on the whole card"
            meta="12 KB"
            interactive
            contextMenuItems={contextItems}
            contextMenuAriaLabel="File actions"
            onContextAction={(value) => setLastClick(`Action: ${value}`)}
            onClick={() => setLastClick("Card clicked")}
            leading={<Icon name="file-text" />}
          />
          <ListCard
            title="Leading trigger menu"
            subtitle="Click the leading block for actions"
            interactive
            contextMenuItems={contextItems}
            contextMenuTrigger="leading"
            contextMenuAriaLabel="Leading actions"
            onContextAction={(value) => setLastClick(`Leading: ${value}`)}
            leading={<Icon name="folder" />}
          />
        </div>
      </SpecimenSection>

      <SpecimenSection title="Sash and not-live">
        <div style={{ display: "flex", flexDirection: "column", gap: "0.25rem" }}>
          <ListCard
            title="Premium integration"
            subtitle="Unlocks advanced features"
            sash="New"
            sashColor="#6366f1"
            leadingShape="rounded-square"
            leadingFill="solid"
            accentColor="#6366f1"
            interactive
            leading={<Icon name="grid-2x2" />}
          />
          <ListCard
            title="Unpublished draft"
            subtitle="Created yesterday · not yet deployed"
            meta="Draft"
            interactive
            notLive
            onClick={() => setLastClick("Unpublished draft")}
            leading={<Icon name="file-text" />}
          />
        </div>
      </SpecimenSection>

      <SpecimenSection title="Static">
        <ListCard title="Read-only item" subtitle="No click handler" />
      </SpecimenSection>

      {lastClick ? (
        <SpecimenSection title="Last click">
          <p data-testid="last-click">{lastClick}</p>
        </SpecimenSection>
      ) : null}
    </>
  );
}

registerSpecimen({
  slug: "list-card",
  title: "ListCard",
  render: () => <ListCardDemo />,
});
