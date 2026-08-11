import { useState } from "react";
import { HistoryCenter, type HistoryBranch, type HistoryEntry } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const linearEntries: HistoryEntry[] = [
  { id: "mix-1", label: "Committed mix 1", position: "past" },
  { id: "arrange", label: "Arranged intro", position: "past" },
  { id: "draft", label: "Current draft", position: "current" },
];

const forkedEntries: HistoryEntry[] = [
  { id: "mix-1", label: "Committed mix 1", position: "past" },
  { id: "checkpoint", label: "Checkpoint: pre-fork", position: "past", checkpoint: true },
  { id: "fork", label: "Fork point", position: "past", branchCount: 2, groupId: "mix-session" },
  { id: "draft", label: "Current draft", position: "current" },
];

const initialBranches: HistoryBranch[] = [
  { id: "b-lead", name: "feature/lead", entryCount: 3, current: true },
  { id: "b-mix", name: "feature/mix-2", entryCount: 1 },
];

const rejectionEntries: HistoryEntry[] = [
  { id: "mix-1", label: "Committed mix 1", position: "past" },
  { id: "draft", label: "Current draft", position: "current" },
];

export function HistoryCenterSpecimen() {
  const [branches, setBranches] = useState(initialBranches);

  return (
    <SpecimenLayout bareVariants>
      <div style={{ display: "grid", gap: "2rem", minHeight: "40rem" }}>
        <SpecimenGroup label="linear">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              entries={linearEntries}
              totalEntries={3}
              defaultOpen
              canUndo
              onSelectEntry={(id) => console.log("select", id)}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="fork">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              entries={forkedEntries}
              totalEntries={4}
              branches={branches}
              totalBranches={2}
              defaultOpen
              canUndo
              onSelectEntry={(id) => console.log("select", id)}
              onCheckout={(branchId, entryId) => console.log("checkout", branchId, entryId)}
              onRenameBranch={(branchId, name) =>
                setBranches((current) => current.map((branch) => (branch.id === branchId ? { ...branch, name } : branch)))}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="rejection">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              entries={rejectionEntries}
              totalEntries={2}
              defaultOpen
              rejection="Branch name is already taken on the authority"
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="empty">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter entries={[]} totalEntries={0} defaultOpen />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="loading">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter entries={[]} totalEntries={0} status="loading" defaultOpen />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
