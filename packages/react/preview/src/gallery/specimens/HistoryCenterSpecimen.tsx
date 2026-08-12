import { useState } from "react";
import type {
  HistoryCenterRejectionCode,
  HistoryContinuation,
  HistoryPathPage,
} from "@inflatable-cookie/poodle-core";
import { HistoryCenter } from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const T = 1_750_000_000_000;

function page(entries: HistoryPathPage["entries"], rootContinuationCount = 1): HistoryPathPage {
  return {
    entries,
    offset: 0,
    rootContinuationCount,
    truncatedBefore: false,
    truncatedAfter: false,
  };
}

const continuation = (
  entryId: string,
  overrides: Partial<HistoryContinuation> = {},
): HistoryContinuation => ({
  entryId,
  label: entryId,
  preferred: false,
  entryCount: 2,
  branchId: `b-${entryId}`,
  branchName: null,
  ...overrides,
});

// Baseline: a linear spine, every entry inert (continuationCount 1/1/0).
const linearPages = [
  page([
    { id: "e3", label: "Current draft", position: "current", continuationCount: 0, recordedAtMs: T + 3_600_000 },
    { id: "e2", label: "Arranged intro", position: "past", continuationCount: 1, recordedAtMs: T + 600_000 },
    { id: "e1", label: "Committed mix 1", position: "past", continuationCount: 1, recordedAtMs: T },
  ]),
];

// Case 1 — two forks at one entry: the badge reads 2 and the picker offers
// two options.
const twoForkPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0, recordedAtMs: T + 3_600_000 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 3, recordedAtMs: T + 600_000 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1, recordedAtMs: T },
  ]),
];
const twoForkContinuations: Record<string, HistoryContinuation[]> = {
  c2: [
    continuation("l1", { label: "Lead intro", branchName: "feature/lead", entryCount: 2 }),
    continuation("x1", { label: "Alt intro", branchName: "feature/alt", preferred: true, entryCount: 1 }),
  ],
};
const twoForkRuns: Record<string, HistoryPathPage[]> = {
  l1: [
    page([
      { id: "l2", label: "Lead mix", position: "past", continuationCount: 0, recordedAtMs: T + 2_400_000 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1, recordedAtMs: T + 1_200_000 },
    ]),
  ],
};

// Case 2 — a fork off a fork: the outer run [l1, l2, l3] forks off l2 into
// the inner run [i1, i2], rendered at depth 2 in the same flat list.
const nestedPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0, recordedAtMs: T + 3_600_000 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 2, recordedAtMs: T + 600_000 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1, recordedAtMs: T },
  ]),
];
const nestedContinuations: Record<string, HistoryContinuation[]> = {
  c2: [continuation("l1", { label: "Lead intro", branchName: "feature/lead", preferred: true, entryCount: 3 })],
  l2: [continuation("i1", { label: "Inner intro", branchName: "feature/inner", preferred: true, entryCount: 2 })],
};
const nestedRuns: Record<string, HistoryPathPage[]> = {
  l1: [
    page([
      { id: "l3", label: "Lead outro", position: "past", continuationCount: 0, recordedAtMs: T + 3_000_000 },
      { id: "l2", label: "Lead bridge", position: "past", continuationCount: 2, recordedAtMs: T + 2_400_000 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1, recordedAtMs: T + 1_200_000 },
    ]),
  ],
  i1: [
    page([
      { id: "i2", label: "Inner mix", position: "past", continuationCount: 0, recordedAtMs: T + 2_700_000 },
      { id: "i1", label: "Inner intro", position: "past", continuationCount: 1, recordedAtMs: T + 2_500_000 },
    ]),
  ],
};

// Case 3 — a single continuation (continuationCount 1 → forkCount 0): the
// entry is inert — no fork icon, no badge, no chevron.
const singleContinuationPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0, recordedAtMs: T + 3_600_000 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 1, recordedAtMs: T + 600_000 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1, recordedAtMs: T },
  ]),
];

// Case 4 — a run's last entry: continuationCount 0 → no fork affordance.
const runTailPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0, recordedAtMs: T + 3_600_000 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 2, recordedAtMs: T + 600_000 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1, recordedAtMs: T },
  ]),
];
const runTailContinuations: Record<string, HistoryContinuation[]> = {
  c2: [continuation("l1", { label: "Lead intro", branchName: "feature/lead", preferred: true, entryCount: 3 })],
};
const runTailRuns: Record<string, HistoryPathPage[]> = {
  l1: [
    page([
      { id: "l3", label: "Lead outro", position: "past", continuationCount: 0, recordedAtMs: T + 3_000_000 },
      { id: "l2", label: "Lead bridge", position: "past", continuationCount: 1, recordedAtMs: T + 2_400_000 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1, recordedAtMs: T + 1_200_000 },
    ]),
  ],
};

// Case 5 — a rejection notice: the machine owns the display copy for the
// code the host's bridge maps ("Already at the requested target").
const rejectionPages = twoForkPages;

// Case 6 — recordedAtMs null: the opened region shows a caption with the
// entry count and no time — never "Invalid Date".
const noTimestampPages = [
  page([
    { id: "c3", label: "Current draft", position: "current", continuationCount: 0 },
    { id: "c2", label: "Arranged intro", position: "past", continuationCount: 2 },
    { id: "c1", label: "Committed mix 1", position: "past", continuationCount: 1 },
  ]),
];
const noTimestampContinuations: Record<string, HistoryContinuation[]> = {
  c2: [continuation("l1", { label: "Lead intro", branchName: "feature/lead", preferred: true, entryCount: 2 })],
};
const noTimestampRuns: Record<string, HistoryPathPage[]> = {
  l1: [
    page([
      { id: "l2", label: "Lead mix", position: "past", continuationCount: 0 },
      { id: "l1", label: "Lead intro", position: "past", continuationCount: 1 },
    ]),
  ],
};

type HostFeed = {
  continuations: { entryId: string; continuations: HistoryContinuation[] } | null;
  run: { fromEntryId: string; pages: HistoryPathPage[] } | null;
};

export function HistoryCenterSpecimen() {
  const [twoForks, setTwoForks] = useState<HostFeed>({ continuations: null, run: null });
  const [nested, setNested] = useState<HostFeed>({ continuations: null, run: null });
  const [runTail, setRunTail] = useState<HostFeed>({ continuations: null, run: null });
  const [noTimestamp, setNoTimestamp] = useState<HostFeed>({ continuations: null, run: null });
  const [renameHost, setRenameHost] = useState<HostFeed>({ continuations: null, run: null });

  function loadContinuations(
    set: (updater: (current: HostFeed) => HostFeed) => void,
    continuations: Record<string, HistoryContinuation[]>,
  ): (entryId: string) => void {
    return (entryId) => set((current) => ({ ...current, continuations: { entryId, continuations: continuations[entryId] ?? [] } }));
  }

  function loadRun(
    set: (updater: (current: HostFeed) => HostFeed) => void,
    runs: Record<string, HistoryPathPage[]>,
  ): (fromEntryId: string) => void {
    return (fromEntryId) => set((current) => ({ ...current, run: { fromEntryId, pages: runs[fromEntryId] ?? [] } }));
  }

  const navigate = (branchId: string | null, entryId: string) => console.log("navigate", branchId, entryId);

  return (
    // Axis tabs are advertised by SpecimenLayout whether or not a specimen
    // fills them (showSizes/showDensities default true), so omitting these
    // renders empty Sizes and Densities tabs. Triggers stay closed here:
    // several open popovers would stack in one place.
    <SpecimenLayout
      bareVariants
      sizes={(size) => <HistoryCenter pages={linearPages} size={size} canUndo canRedo />}
      densities={(density) => <HistoryCenter pages={linearPages} density={density} canUndo canRedo />}
    >
      {/* The two-forks group opens by default so the capture shows the flat
          list with the counter badge; the popover portals to the theme root,
          so only one group opens at once. */}
      <div style={{ display: "grid", gap: "2rem", minHeight: "40rem" }}>
        <SpecimenGroup label="linear">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter pages={linearPages} canUndo onNavigateEntry={navigate} />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="two-forks">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              pages={twoForkPages}
              defaultOpen
              canUndo
              continuationsResult={twoForks.continuations}
              runResult={twoForks.run}
              onLoadContinuations={loadContinuations(setTwoForks, twoForkContinuations)}
              onLoadContinuationRun={loadRun(setTwoForks, twoForkRuns)}
              onNavigateEntry={navigate}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="fork-off-fork">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              pages={nestedPages}
              canUndo
              continuationsResult={nested.continuations}
              runResult={nested.run}
              onLoadContinuations={loadContinuations(setNested, nestedContinuations)}
              onLoadContinuationRun={loadRun(setNested, nestedRuns)}
              onNavigateEntry={navigate}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="single-continuation">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter pages={singleContinuationPages} canUndo onNavigateEntry={navigate} />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="run-tail">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              pages={runTailPages}
              canUndo
              continuationsResult={runTail.continuations}
              runResult={runTail.run}
              onLoadContinuations={loadContinuations(setRunTail, runTailContinuations)}
              onLoadContinuationRun={loadRun(setRunTail, runTailRuns)}
              onNavigateEntry={navigate}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="rejection">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              pages={rejectionPages}
              rejection={"AlreadyAtTarget" satisfies HistoryCenterRejectionCode}
              canUndo
              onNavigateEntry={navigate}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="no-timestamp">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              pages={noTimestampPages}
              canUndo
              continuationsResult={noTimestamp.continuations}
              runResult={noTimestamp.run}
              onLoadContinuations={loadContinuations(setNoTimestamp, noTimestampContinuations)}
              onLoadContinuationRun={loadRun(setNoTimestamp, noTimestampRuns)}
              onNavigateEntry={navigate}
            />
          </div>
        </SpecimenGroup>

        <SpecimenGroup label="rename">
          <div style={{ display: "flex", justifyContent: "flex-end", width: "min(42rem, 100%)" }}>
            <HistoryCenter
              pages={nestedPages}
              canUndo
              continuationsResult={renameHost.continuations}
              runResult={renameHost.run}
              onLoadContinuations={loadContinuations(setRenameHost, nestedContinuations)}
              onLoadContinuationRun={loadRun(setRenameHost, nestedRuns)}
              onNavigateEntry={navigate}
              onRenameBranch={(branchId, name) => console.log("rename", branchId, name)}
            />
          </div>
        </SpecimenGroup>
      </div>
    </SpecimenLayout>
  );
}
