import { useState, type CSSProperties } from "react";
import { BulkActionBar, type BulkAction } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const actions: BulkAction[] = [
  { id: "export", label: "Export", icon: "download" },
  { id: "archive", label: "Archive", icon: "inbox" },
  { id: "delete", label: "Delete", icon: "trash-2", tone: "danger" },
  { id: "review", label: "Review", icon: "triangle-alert", tone: "warning" },
];

const hintStyle: CSSProperties = {
  margin: 0,
  fontSize: "0.75rem",
  color: "var(--poodle-color-text-secondary)",
};

// The BulkActionBar renders position:fixed by default (a floating action bar).
// The Svelte specimen neutralizes that via a :global override so it renders
// inline for the gallery. React BulkActionBar accepts no className/style, so we
// reproduce the same global override with a scoped <style> block.
const inlineOverride = (
  <style>{`
    .poodle-bulk-action-bar-specimen__inline { display: flex; }
    .poodle-bulk-action-bar-specimen__inline .poodle-bulk-action-bar {
      position: static;
      right: auto;
      bottom: auto;
      left: auto;
      flex: 1 1 auto;
      width: 100%;
      max-width: none;
    }
  `}</style>
);

export function BulkActionBarSpecimen() {
  const [lastAction, setLastAction] = useState("");
  const [allSelected, setAllSelected] = useState(false);

  return (
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <>
          {inlineOverride}
          <div className="poodle-bulk-action-bar-specimen__inline">
            <BulkActionBar selectionCount={5} actions={actions} size={size} />
          </div>
        </>
      )}
      densities={(density) => (
        <>
          {inlineOverride}
          <div className="poodle-bulk-action-bar-specimen__inline">
            <BulkActionBar selectionCount={5} actions={actions} density={density} />
          </div>
        </>
      )}
    >
      {inlineOverride}
      <div className="poodle-specimen">
        <SpecimenGroup label="With selection count and select all" bare>
          <div className="poodle-bulk-action-bar-specimen__inline">
            <BulkActionBar
              selectionCount={5}
              totalCount={42}
              actions={actions}
              showSelectAll
              allSelected={allSelected}
              onAction={(id) => setLastAction(id)}
              onSelectAll={() => setAllSelected(true)}
              onClear={() => setAllSelected(false)}
            />
          </div>
          {lastAction ? (
            <p style={hintStyle}>Last action: <strong>{lastAction}</strong></p>
          ) : null}
        </SpecimenGroup>

        <SpecimenGroup label="Single item selected" bare>
          <div className="poodle-bulk-action-bar-specimen__inline">
            <BulkActionBar selectionCount={1} actions={actions.slice(0, 2)} />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="Loading and disabled actions" bare>
          <div className="poodle-bulk-action-bar-specimen__inline">
            <BulkActionBar
              selectionCount={12}
              totalCount={12}
              actions={[
                { id: "publish", label: "Publish", icon: "rocket" },
                { id: "delete", label: "Delete", icon: "trash-2", tone: "danger", disabled: true },
              ]}
              showSelectAll
              allSelected
              loading
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
