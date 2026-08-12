import { useState } from "react";
import type { HistoryBranch, HistoryEntry } from "@inflatable-cookie/poodle-core";
import { HistoryCenter } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const T = 1_750_000_000_000;

const linearBranches: HistoryBranch[] = [{ id: "b-main", name: "main", current: true }];
const linearPaths: Record<string, HistoryEntry[]> = {
  "b-main": [
    { id: "e1", label: "Committed mix 1", position: "past" },
    { id: "e2", label: "Arranged intro", position: "past" },
    { id: "e3", label: "Current draft", position: "current" },
  ],
};

const forkBranches: HistoryBranch[] = [
  { id: "b-main", name: "main", current: true },
  { id: "b-lead", name: "feature/lead" },
];
const forkPaths: Record<string, HistoryEntry[]> = {
  "b-main": [
    { id: "e1", label: "Committed mix 1", position: "past", recordedAtMs: T },
    { id: "e2", label: "Arranged intro", position: "past", recordedAtMs: T + 600_000 },
    { id: "e3", label: "Current draft", position: "current", recordedAtMs: T + 3_600_000 },
  ],
  "b-lead": [
    { id: "e1", label: "Committed mix 1", position: "past", recordedAtMs: T },
    { id: "e2", label: "Arranged intro", position: "past", recordedAtMs: T + 600_000 },
    { id: "l1", label: "Lead intro", position: "past", recordedAtMs: T + 1_200_000 },
    { id: "l2", label: "Lead mix", position: "past", recordedAtMs: T + 2_400_000 },
  ],
};

const nestedBranches: HistoryBranch[] = [
  { id: "b-main", name: "main", current: true },
  { id: "b-outer", name: "feature/outer" },
  { id: "b-inner", name: "feature/inner" },
];
const nestedPaths: Record<string, HistoryEntry[]> = {
  "b-main": [
    { id: "e1", label: "Committed mix 1", position: "past" },
    { id: "e2", label: "Arranged intro", position: "past" },
    { id: "e3", label: "Bridge midi", position: "past" },
    { id: "e4", label: "Current draft", position: "current" },
  ],
  "b-outer": [
    { id: "e1", label: "Committed mix 1", position: "past" },
    { id: "e2", label: "Arranged intro", position: "past" },
    { id: "o1", label: "Outer intro", position: "past" },
    { id: "o2", label: "Outer mix", position: "past" },
  ],
  "b-inner": [
    { id: "e1", label: "Committed mix 1", position: "past" },
    { id: "e2", label: "Arranged intro", position: "past" },
    { id: "o1", label: "Outer intro", position: "past" },
    { id: "i1", label: "Inner intro", position: "past" },
    { id: "i2", label: "Inner mix", position: "past" },
  ],
};

const manyBranches: HistoryBranch[] = [
  { id: "b-main", name: "main", current: true },
  { id: "b-1", name: "take/session-1" },
  { id: "b-2", name: "take/session-2" },
  { id: "b-3", name: "take/session-3" },
  { id: "b-4", name: "take/session-4" },
  { id: "b-5", name: "take/session-5" },
  { id: "b-6", name: "take/session-6" },
];
const manyPaths: Record<string, HistoryEntry[]> = {
  "b-main": [
    { id: "r1", label: "Root edit", position: "past" },
    { id: "r2", label: "Current draft", position: "current" },
  ],
  "b-1": [{ id: "r1", label: "Root edit", position: "past" }, { id: "t1", label: "Take 1", position: "past" }],
  "b-2": [{ id: "r1", label: "Root edit", position: "past" }, { id: "t2", label: "Take 2", position: "past" }],
  "b-3": [{ id: "r1", label: "Root edit", position: "past" }, { id: "t3", label: "Take 3", position: "past" }],
  "b-4": [{ id: "r1", label: "Root edit", position: "past" }, { id: "t4", label: "Take 4", position: "past" }],
  "b-5": [{ id: "r1", label: "Root edit", position: "past" }, { id: "t5", label: "Take 5", position: "past" }],
  "b-6": [{ id: "r1", label: "Root edit", position: "past" }, { id: "t6", label: "Take 6", position: "past" }],
};

export function HistoryCenterSpecimen() {
  const [renameBranches, setRenameBranches] = useState<HistoryBranch[]>(forkBranches);

  return (
    // Axis tabs are advertised by SpecimenLayout whether or not a specimen
    // fills them (showSizes/showDensities default true), so omitting these
    // renders empty Sizes and Densities tabs. Triggers stay closed here:
    // several open popovers would stack in one place.
    <SpecimenLayout
      bareVariants
      sizes={(size) => (
        <HistoryCenter branches={linearBranches} paths={linearPaths} totalEntries={3} size={size} canUndo canRedo />
      )}
      densities={(density) => (
        <HistoryCenter branches={linearBranches} paths={linearPaths} totalEntries={3} density={density} canUndo canRedo />
      )}
    >
      {/* Only the fork group opens by default so the capture shows the lane
          rendering. The popover portals to the theme root, so several open
          at once would stack on top of each other. */}
      <div style={{ display: "grid", gap: "2rem", minHeight: "40rem" }}>
        <SpecimenGroup label="linear">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              branches={linearBranches}
              paths={linearPaths}
              totalEntries={3}
              canUndo
              onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="fork">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              branches={forkBranches}
              paths={forkPaths}
              totalEntries={5}
              totalBranches={2}
              defaultOpen
              canUndo
              onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
              onRenameBranch={(branchId, name) =>
                setRenameBranches((current) =>
                  current.map((branch) => (branch.id === branchId ? { ...branch, name } : branch)))}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="fork-off-fork">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              branches={nestedBranches}
              paths={nestedPaths}
              totalEntries={7}
              totalBranches={3}
              canUndo
              onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="many-forks">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              branches={manyBranches}
              paths={manyPaths}
              totalEntries={8}
              totalBranches={7}
              canUndo
              onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="rejection">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              branches={forkBranches}
              paths={forkPaths}
              totalEntries={5}
              totalBranches={2}
              rejection="Branch name is already taken on the authority"
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="rename">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              branches={renameBranches}
              paths={forkPaths}
              totalEntries={5}
              totalBranches={2}
              canUndo
              onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
              onRenameBranch={(branchId, name) =>
                setRenameBranches((current) =>
                  current.map((branch) => (branch.id === branchId ? { ...branch, name } : branch)))}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
