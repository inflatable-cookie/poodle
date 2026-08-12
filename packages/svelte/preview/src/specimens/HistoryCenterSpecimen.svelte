<script lang="ts">
  import { onMount } from "svelte";
  import type {
    HistoryCenterRejectionCode,
    HistoryContinuation,
    HistoryPathPage,
  } from "@inflatable-cookie/poodle-core";
  import { HistoryCenter } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const T = 1_750_000_000_000;

  function page(entries: HistoryPathPage["entries"], precedingContinuationCount = 1): HistoryPathPage {
    return {
      entries,
      offset: 0,
      precedingContinuationCount,
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

  // Case 1 — two forks at one entry: the badge reads 2 and the picker is a
  // persistent Select. R3 opens on the current fork (x1, preferred): the
  // select shows it with a Current marker, checkout stays disabled, and the
  // run renders below the select — the reported defect (picker vanishes
  // after a choice) is what this capture disproves.
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
    x1: [
      page([
        { id: "x2", label: "Alt mix", position: "past", continuationCount: 0, recordedAtMs: T + 2_300_000 },
        { id: "x1", label: "Alt intro", position: "past", continuationCount: 1, recordedAtMs: T + 1_100_000 },
      ]),
    ],
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

  // Host simulation: each group's two result feeds resolve synchronously from
  // the fixtures above, exactly as a real host resolves the three operations.
  let twoForks = $state<{ continuations: { entryId: string; continuations: HistoryContinuation[] } | null; run: { fromEntryId: string; pages: HistoryPathPage[] } | null }>({
    continuations: null,
    run: null,
  });
  let nested = $state({ ...twoForks });
  let runTail = $state({ ...twoForks });
  let noTimestamp = $state({ ...twoForks });
  let renameHost = $state({ ...twoForks });
  let singleForkOpen = $state({ ...twoForks });

  function loadContinuations(
    state: typeof twoForks,
    set: (next: typeof twoForks) => void,
    continuations: Record<string, HistoryContinuation[]>,
  ): (entryId: string) => void {
    return (entryId) => set({ ...state, continuations: { entryId, continuations: continuations[entryId] ?? [] } });
  }

  function loadRun(
    state: typeof twoForks,
    set: (next: typeof twoForks) => void,
    runs: Record<string, HistoryPathPage[]>,
  ): (fromEntryId: string) => void {
    return (fromEntryId) => set({ ...state, run: { fromEntryId, pages: runs[fromEntryId] ?? [] } });
  }

  // The capture never clicks, so the single-fork-open group drives its own
  // single interaction on mount: disclose the fork. The capture then shows
  // the unified picker row for one fork — the disabled Select with the fork
  // icon, name, branch, entry count and relative time, the rename pencil,
  // the opt-in delete button (the host supplies the callback here) and the
  // disabled checkout. The popover portals to the theme root, so only one
  // group opens at once.
  onMount(() => {
    const run = (): boolean => {
      const disclosure = document.querySelector<HTMLButtonElement>('[data-part="fork-disclosure"]');
      if (disclosure === null) {
        return false;
      }
      disclosure.click();
      return true;
    };

    if (!run()) {
      const timer = setTimeout(() => run(), 60);
      return () => clearTimeout(timer);
    }
  });
</script>

<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-history-center-specimen">
      <!-- The single-fork-open group opens by default so the capture shows
           the unified picker row for one fork — the disabled Select with the
           fork icon, name, branch, entry count and relative time, the rename
           pencil, the opt-in delete and the disabled checkout (the mount-time
           driver discloses the fork); the popover portals to the theme root,
           so only one group opens at once. -->
      <SpecimenGroup label="linear">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={linearPages}
            canUndo
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="two-forks">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={twoForkPages}
            canUndo
            continuationsResult={twoForks.continuations}
            runResult={twoForks.run}
            onLoadContinuations={loadContinuations(twoForks, (next) => (twoForks = next), twoForkContinuations)}
            onLoadContinuationRun={loadRun(twoForks, (next) => (twoForks = next), twoForkRuns)}
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="fork-off-fork">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={nestedPages}
            canUndo
            continuationsResult={nested.continuations}
            runResult={nested.run}
            onLoadContinuations={loadContinuations(nested, (next) => (nested = next), nestedContinuations)}
            onLoadContinuationRun={loadRun(nested, (next) => (nested = next), nestedRuns)}
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="single-continuation">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={singleContinuationPages}
            canUndo
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="run-tail">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={runTailPages}
            canUndo
            continuationsResult={runTail.continuations}
            runResult={runTail.run}
            onLoadContinuations={loadContinuations(runTail, (next) => (runTail = next), runTailContinuations)}
            onLoadContinuationRun={loadRun(runTail, (next) => (runTail = next), runTailRuns)}
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="single-fork-open">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={runTailPages}
            defaultOpen
            canUndo
            continuationsResult={singleForkOpen.continuations}
            runResult={singleForkOpen.run}
            onLoadContinuations={loadContinuations(singleForkOpen, (next) => (singleForkOpen = next), runTailContinuations)}
            onLoadContinuationRun={loadRun(singleForkOpen, (next) => (singleForkOpen = next), runTailRuns)}
            onDeleteContinuation={(entryId) => console.log("delete", entryId)}
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="rejection">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={rejectionPages}
            rejection={"AlreadyAtTarget" satisfies HistoryCenterRejectionCode}
            canUndo
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="no-timestamp">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={noTimestampPages}
            canUndo
            continuationsResult={noTimestamp.continuations}
            runResult={noTimestamp.run}
            onLoadContinuations={loadContinuations(noTimestamp, (next) => (noTimestamp = next), noTimestampContinuations)}
            onLoadContinuationRun={loadRun(noTimestamp, (next) => (noTimestamp = next), noTimestampRuns)}
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="rename">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={nestedPages}
            canUndo
            continuationsResult={renameHost.continuations}
            runResult={renameHost.run}
            onLoadContinuations={loadContinuations(renameHost, (next) => (renameHost = next), nestedContinuations)}
            onLoadContinuationRun={loadRun(renameHost, (next) => (renameHost = next), nestedRuns)}
            onNavigateEntry={(branchId, entryId) => console.log("navigate", branchId, entryId)}
            onRenameBranch={(branchId, name) => console.log("rename", branchId, name)}
          />
        </div>
      </SpecimenGroup>
    </div>
  {/snippet}

  <!-- Axis tabs are advertised by SpecimenLayout whether or not a specimen
       fills them (showSizes/showDensities default true), so a specimen that
       omits these renders empty Sizes and Densities tabs. Triggers stay closed
       here: several open popovers would stack in one place. -->
  {#snippet sizes(size)}
    <HistoryCenter pages={linearPages} {size} canUndo canRedo />
  {/snippet}

  {#snippet densities(density)}
    <HistoryCenter pages={linearPages} {density} canUndo canRedo />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-history-center-specimen { display: grid; gap: 2rem; min-height: 40rem; }
  .poodle-history-center-specimen__anchor { display: flex; justify-content: flex-end; width: min(42rem, 100%); }
</style>
