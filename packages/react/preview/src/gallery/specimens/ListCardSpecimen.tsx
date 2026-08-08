import { useState, type CSSProperties } from "react";
import { ContextMenu, Icon, IconButton, ListCard, ListCardCounter, Pill } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const tiles: CSSProperties = {
  display: "grid",
  gridTemplateColumns: "repeat(auto-fit, minmax(16rem, 1fr))",
  gap: "0.75rem",
};
// The Svelte specimen scopes a tighter stack gap for list rows; the shared
// gallery rule is 0.5rem. Kept in sync so the visual gate compares the cards,
// not the harness spacing.
const stack: CSSProperties = { display: "flex", flexDirection: "column", gap: "0.25rem" };
// Svelte's ListCard specimen scopes `p { margin: 0 }`; other specimens keep the
// UA margin, so this reset stays local rather than global in gallery.css.
const inlineCopy: CSSProperties = { margin: 0 };
const densityDemo: CSSProperties = { width: "min(100%, 26rem)" };
const secondary: CSSProperties = { color: "var(--poodle-color-text-secondary)" };

export function ListCardSpecimen() {
  const [lastClick, setLastClick] = useState("");
  const [selectedCard, setSelectedCard] = useState(false);

  return (
    <SpecimenLayout
      sizes={(size) => (
        <div style={densityDemo}>
          <ListCard
            title="Workspace settings"
            subtitle="Shared defaults and access controls"
            interactive
            size={size}
            leading={<Icon icon="folder" />}
            trailing={<Pill tone="success" size={size}>Live</Pill>}
          />
        </div>
      )}
      densities={(density) => (
        <div style={densityDemo}>
          <ListCard
            title="Workspace settings"
            subtitle="Shared defaults and access controls"
            interactive
            density={density}
            leading={<Icon icon="folder" />}
            trailing={<Pill tone="success">Live</Pill>}
          />
        </div>
      )}
    >
      <div className="poodle-specimen">
        <SpecimenGroup label="Interactive list cards">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="design-system-v2.figma"
              subtitle="Updated by Clay · 2h ago"
              meta="14.2 MB"
              interactive
              onClick={() => setLastClick("design-system-v2.figma")}
              leading={<Icon icon="folder" />}
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
            <ListCard
              title="brand-assets.zip"
              subtitle="Archived"
              meta="48 MB"
              disabled
              leading={<Icon icon="layers" />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Link roots and explicit actions">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Billing settings"
              subtitle="Manage invoices and payment methods"
              href="#billing"
              leading={<Icon name="credit-card" />}
              badges={<Pill appearance="badge" tone="neutral">2</Pill>}
            />
            <ListCard
              title="Project Alpha"
              subtitle="Replaces the old media-trigger pattern with explicit actions"
              interactive
              onClick={() => setLastClick("Project Alpha")}
              leading={<Icon icon="folder" />}
              actions={<IconButton icon="ellipsis" ariaLabel="Project actions" variant="ghost" />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Hierarchy titles">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Cash flow forecasts"
              subtitle="Module content"
              interactive
              onClick={() => setLastClick("Cash flow forecasts")}
              leading={<Icon icon="grid-2x2" />}
              titleContent={
                <>
                  <span style={secondary}>Pathway</span>{" "}
                  <Icon name="chevron-right" size="xs" />{" "}
                  <span style={secondary}>Module</span>{" "}
                  <Icon name="chevron-right" size="xs" />{" "}
                  <span>Cash flow forecasts</span>
                </>
              }
              metaContent={<span style={secondary}>Weight 3</span>}
            />
            <ListCard
              title="Week 1: Cash Flow"
              subtitle="Move within section"
              layout="compact"
              showReorderHandle
              titleContent={
                <>
                  <span style={secondary}>Module</span>{" "}
                  <Icon name="chevron-right" size="xs" />{" "}
                  <span>Week 1: Cash Flow</span>
                </>
              }
              badges={<Pill tone="info">Move</Pill>}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Selectable cards">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Selected row"
              subtitle="Batch-selection ready"
              selectable
              selected={selectedCard}
              onSelectedChange={(selected) => setSelectedCard(selected)}
              leading={<Icon name="check-square" />}
              badges={<Pill tone="success">Chosen</Pill>}
            />
            <ListCard
              title="Compact reorder item"
              layout="compact"
              showReorderHandle
              leading={<Icon name="grip" />}
              badges={<Pill appearance="badge" tone="neutral">Draft</Pill>}
            />
            <ListCard
              title="Reordered item"
              subtitle="Move within section"
              layout="compact"
              showReorderHandle
              titleContent={
                <>
                  <span style={secondary}>Module</span>{" "}
                  <Icon name="chevron-right" size="xs" />{" "}
                  <span>Week 1: Cash Flow</span>
                </>
              }
              badges={<Pill tone="info">Move</Pill>}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Rounded-square leading (thumbnails)">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="hero-banner.png"
              subtitle="Uploaded by Jamie · 4h ago"
              meta="3.1 MB"
              leadingShape="rounded-square"
              interactive
              onClick={() => setLastClick("hero-banner.png")}
              leading={<Icon name="image" />}
            />
            <ListCard
              title="onboarding-flow.mp4"
              subtitle="Screen recording · Today"
              meta="128 MB"
              leadingShape="rounded-square"
              interactive
              onClick={() => setLastClick("onboarding-flow.mp4")}
              leading={<Icon name="play" />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Leading size offset">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Default leading"
              subtitle="Matches the card size ladder"
              interactive
              leading={<Icon name="file-text" />}
            />
            <ListCard
              title="Offset leading"
              subtitle="Leading block steps up relative to the card size"
              interactive
              leadingSizeOffset={1}
              leading={<Icon name="file-text" />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Stacked layout">
          <div style={tiles}>
            <ListCard
              title="Release notes"
              subtitle="Summarize the latest component changes and known migration edges."
              layout="stacked"
              leadingShape="rounded-square"
              leadingFill="solid"
              accentColor="#6366f1"
              interactive
              leading={<Icon icon="layers" />}
              trailing={<Pill tone="success">Published</Pill>}
            />
            <ListCard
              title="Media library"
              subtitle="Review uploads, approve assets, and keep metadata consistent across the library."
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
        </SpecimenGroup>

        <SpecimenGroup label="With badges">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="API integration guide"
              subtitle="Updated yesterday"
              meta="Draft"
              interactive
              onClick={() => setLastClick("API integration guide")}
              leading={<Icon name="file-text" />}
              badges={<Pill tone="info">New</Pill>}
            />
            <ListCard
              title="Q4 planning deck"
              subtitle="Shared with leadership"
              interactive
              onClick={() => setLastClick("Q4 planning deck")}
              leading={<Icon icon="layers" />}
              badges={
                <>
                  <Pill appearance="badge" tone="neutral">3</Pill>
                  <Pill tone="warning">Review</Pill>
                </>
              }
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="With footer counters">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Design system"
              subtitle="12 contributors"
              leadingShape="rounded-square"
              interactive
              onClick={() => setLastClick("Design system")}
              leading={<Icon icon="folder" />}
              badges={<Pill tone="success">Active</Pill>}
              footer={
                <>
                  <ListCardCounter icon="file-text" count={24} tooltip="24 documents" />
                  <ListCardCounter icon="image" count={8} tooltip="8 images" />
                  <ListCardCounter icon="layers" count={3} tooltip="3 sub-folders" href="#sub-folders" />
                </>
              }
            />
            <ListCard
              title="Brand guidelines"
              subtitle="Last updated 2 weeks ago"
              leadingShape="rounded-square"
              interactive
              onClick={() => setLastClick("Brand guidelines")}
              leading={<Icon icon="folder" />}
              footer={
                <>
                  <ListCardCounter icon="file-text" count={6} tooltip="6 documents" />
                  <ListCardCounter icon="image" count={42} tooltip="42 images" />
                </>
              }
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Inherited footer counters">
          <p className="poodle-specimen__inline-copy" style={inlineCopy}>
            Summary:{" "}
            <ListCardCounter icon="file-text" count={24} tooltip="24 documents" typography="inherit" />
            {" "}and{" "}
            <ListCardCounter icon="image" count={8} tooltip="8 images" typography="inherit" />
            {" "}published this week.
          </p>
        </SpecimenGroup>

        <SpecimenGroup label="Solid fill with accent colors">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Design tokens"
              subtitle="Foundation layer"
              leadingShape="rounded-square"
              leadingFill="solid"
              accentColor="#6366f1"
              interactive
              onClick={() => setLastClick("Design tokens")}
              leading={<Icon icon="layers" />}
            />
            <ListCard
              title="Components"
              subtitle="Primitives and composites"
              leadingShape="rounded-square"
              leadingFill="solid"
              accentColor="#ec4899"
              interactive
              onClick={() => setLastClick("Components")}
              leading={<Icon icon="grid-2x2" />}
            />
            <ListCard
              title="Documentation"
              subtitle="Contracts and guides"
              leadingShape="rounded-square"
              leadingFill="solid"
              accentColor="#14b8a6"
              interactive
              onClick={() => setLastClick("Documentation")}
              leading={<Icon name="file-text" />}
            />
            <ListCard
              title="Default accent (no custom color)"
              subtitle="Uses theme accent"
              leadingShape="rounded-square"
              leadingFill="solid"
              interactive
              onClick={() => setLastClick("Default accent")}
              leading={<Icon icon="folder" />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="With built-in context menu">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Right-click for actions"
              subtitle="Context menu on the whole card"
              meta="12 KB"
              interactive
              contextMenuItems={[
                { label: "Open", value: "open" },
                { label: "Rename", value: "rename" },
                { label: "Duplicate", value: "duplicate" },
                { kind: "separator", label: "", value: "sep-1" },
                { label: "Delete", value: "delete" },
              ]}
              contextMenuAriaLabel="File actions"
              onContextAction={(value) => setLastClick(`Action: ${value}`)}
              onClick={() => setLastClick("Card clicked")}
              leading={<Icon name="file-text" />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="With wrapped context menu">
          <div className="poodle-specimen__stack" style={stack}>
            <ContextMenu
              items={[
                { label: "Open", value: "open" },
                { label: "Rename", value: "rename" },
                { label: "Duplicate", value: "duplicate" },
                { kind: "separator", label: "", value: "sep-1" },
                { label: "Delete", value: "delete" },
              ]}
              onAction={(value) => setLastClick(`Action: ${value}`)}
            >
              <ListCard
                title="Legacy wrapper path"
                subtitle="Still supported for arbitrary content"
                meta="12 KB"
                interactive
                onClick={() => setLastClick("Card clicked")}
                leading={<Icon name="file-text" />}
              />
            </ContextMenu>
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Exclusive trailing lane">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Workspace settings"
              subtitle="Shared defaults and access controls"
              meta="Updated 2h ago"
              interactive
              leading={<Icon icon="folder" />}
              trailing={<Pill tone="success">Live</Pill>}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Not live (dashed border, interactive)">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Unpublished draft"
              subtitle="Created yesterday · not yet deployed"
              meta="Draft"
              interactive
              notLive
              onClick={() => setLastClick("Unpublished draft")}
              leading={<Icon name="file-text" />}
            />
            <ListCard
              title="Staging environment"
              subtitle="Pending approval"
              interactive
              notLive
              leadingShape="rounded-square"
              leading={<Icon icon="layers" />}
              badges={<Pill tone="warning" size="md">Pending</Pill>}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Corner sash badges">
          <div className="poodle-specimen__stack" style={stack}>
            <ListCard
              title="Free tier plan"
              subtitle="No credit card required"
              sash="Free"
              interactive
              onClick={() => setLastClick("Free tier plan")}
              leading={<Icon icon="layers" />}
            />
            <ListCard
              title="Premium integration"
              subtitle="Unlocks advanced features"
              sash="New"
              sashColor="#6366f1"
              leadingShape="rounded-square"
              leadingFill="solid"
              accentColor="#6366f1"
              interactive
              onClick={() => setLastClick("Premium integration")}
              leading={<Icon icon="grid-2x2" />}
            />
            <ListCard
              title="Legacy connector"
              subtitle="Deprecated — migrate by Q2"
              sash="EOL"
              sashColor="#ef4444"
              interactive
              onClick={() => setLastClick("Legacy connector")}
              leading={<Icon name="file-text" />}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Static list card">
          <ListCard title="Read-only item" subtitle="No click handler" />
        </SpecimenGroup>

        {lastClick && (
          <SpecimenGroup label="Last click">
            <p style={{ margin: 0 }}>{lastClick}</p>
          </SpecimenGroup>
        )}
      </div>
    </SpecimenLayout>
  );
}
