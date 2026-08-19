<script lang="ts">
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
  type HostFeed = {
    continuations: { entryId: string; continuations: HistoryContinuation[] } | null;
    run: { fromEntryId: string; pages: HistoryPathPage[] } | null;
  };
  const emptyFeed = (): HostFeed => ({ continuations: null, run: null });
  let twoForks = $state<HostFeed>(emptyFeed());
  let nested = $state(emptyFeed());
  let runTail = $state(emptyFeed());
  let noTimestamp = $state(emptyFeed());
  let renameHost = $state(emptyFeed());
  let manageHost = $state(emptyFeed());

  let failureCommand = $state("");
  let manageCommand = $state("");
  let hostCommand = $state("");

  function undoHistory(): void {
    hostCommand = "Undo";
  }

  function navigateEntry(_branchId: string | null, entryId: string): void {
    hostCommand = `Navigate ${entryId}`;
  }

  function loadContinuations(
    state: HostFeed,
    set: (next: HostFeed) => void,
    continuations: Record<string, HistoryContinuation[]>,
  ): (entryId: string) => void {
    return (entryId) => set({ ...state, continuations: { entryId, continuations: continuations[entryId] ?? [] } });
  }

  function loadRun(
    state: HostFeed,
    set: (next: HostFeed) => void,
    runs: Record<string, HistoryPathPage[]>,
  ): (fromEntryId: string) => void {
    return (fromEntryId) => set({ ...state, run: { fromEntryId, pages: runs[fromEntryId] ?? [] } });
  }

</script>


<SpecimenLayout bareVariants>
  {#snippet children()}
    <div class="poodle-history-center-specimen">
      {#if hostCommand}
        <p class="poodle-history-center-specimen__hint">Last host command: <strong>{hostCommand}</strong></p>
      {/if}
      <SpecimenGroup label="Linear history">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter pages={linearPages} canUndo onUndo={undoHistory} onNavigateEntry={navigateEntry} />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Choosing between continuations">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={twoForkPages}
            canUndo
            continuationsResult={twoForks.continuations}
            runResult={twoForks.run}
            onLoadContinuations={loadContinuations(twoForks, (next) => (twoForks = next), twoForkContinuations)}
            onLoadContinuationRun={loadRun(twoForks, (next) => (twoForks = next), twoForkRuns)}
            onNavigateEntry={navigateEntry}
            onUndo={undoHistory}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Nested continuation runs">
        <div class="poodle-history-center-specimen__anchor">
          <HistoryCenter
            pages={nestedPages}
            canUndo
            continuationsResult={nested.continuations}
            runResult={nested.run}
            onLoadContinuations={loadContinuations(nested, (next) => (nested = next), nestedContinuations)}
            onLoadContinuationRun={loadRun(nested, (next) => (nested = next), nestedRuns)}
            onNavigateEntry={navigateEntry}
            onUndo={undoHistory}
          />
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Single continuation and run boundaries">
        <div class="poodle-history-center-specimen__stack">
          <div class="poodle-history-center-specimen__anchor">
            <HistoryCenter pages={singleContinuationPages} canUndo onUndo={undoHistory} onNavigateEntry={navigateEntry} />
          </div>
          <div class="poodle-history-center-specimen__anchor">
            <HistoryCenter
              pages={runTailPages}
              canUndo
              continuationsResult={runTail.continuations}
              runResult={runTail.run}
              onLoadContinuations={loadContinuations(runTail, (next) => (runTail = next), runTailContinuations)}
              onLoadContinuationRun={loadRun(runTail, (next) => (runTail = next), runTailRuns)}
              onNavigateEntry={navigateEntry}
            onUndo={undoHistory}
            />
          </div>
        </div>
      </SpecimenGroup>

      <SpecimenGroup label="Rename and manage a continuation">
        <div class="poodle-history-center-specimen__stack">
          <div class="poodle-history-center-specimen__anchor">
            <HistoryCenter
              pages={runTailPages}
              canUndo
              continuationsResult={manageHost.continuations}
              runResult={manageHost.run}
              onLoadContinuations={loadContinuations(manageHost, (next) => (manageHost = next), runTailContinuations)}
              onLoadContinuationRun={loadRun(manageHost, (next) => (manageHost = next), runTailRuns)}
              onRenameBranch={(branchId, name) => (manageCommand = `Rename ${branchId} to ${name}`)}
              onNavigateEntry={navigateEntry}
              onUndo={undoHistory}
            />
          </div>
          <div class="poodle-history-center-specimen__anchor">
            <HistoryCenter
              pages={nestedPages}
              canUndo
              continuationsResult={renameHost.continuations}
              runResult={renameHost.run}
              onLoadContinuations={loadContinuations(renameHost, (next) => (renameHost = next), nestedContinuations)}
              onLoadContinuationRun={loadRun(renameHost, (next) => (renameHost = next), nestedRuns)}
              onNavigateEntry={navigateEntry}
            onUndo={undoHistory}
              onRenameBranch={(branchId, name) => (manageCommand = `Rename ${branchId} to ${name}`)}
            />
          </div>
        </div>
        {#if manageCommand}
          <p class="poodle-history-center-specimen__hint">Last command: <strong>{manageCommand}</strong></p>
        {/if}
      </SpecimenGroup>

      <SpecimenGroup label="Failure and incomplete metadata">
        <div class="poodle-history-center-specimen__stack">
          <div class="poodle-history-center-specimen__anchor">
            <HistoryCenter
              pages={rejectionPages}
              rejection={"AlreadyAtTarget" satisfies HistoryCenterRejectionCode}
              canUndo
              onNavigateEntry={navigateEntry}
            onUndo={undoHistory}
            />
          </div>
          <div class="poodle-history-center-specimen__anchor">
            <HistoryCenter
              pages={noTimestampPages}
              canUndo
              continuationsResult={noTimestamp.continuations}
              runResult={noTimestamp.run}
              onLoadContinuations={loadContinuations(noTimestamp, (next) => (noTimestamp = next), noTimestampContinuations)}
              onLoadContinuationRun={loadRun(noTimestamp, (next) => (noTimestamp = next), noTimestampRuns)}
              onNavigateEntry={navigateEntry}
            onUndo={undoHistory}
            />
          </div>
        </div>
        {#if failureCommand}
          <p class="poodle-history-center-specimen__hint">Last command: <strong>{failureCommand}</strong></p>
        {/if}
      </SpecimenGroup>
    </div>
  {/snippet}

  {#snippet sizes(size)}
    <HistoryCenter pages={linearPages} {size} canUndo onUndo={undoHistory} canRedo />
  {/snippet}

  {#snippet densities(density)}
    <HistoryCenter pages={linearPages} {density} canUndo onUndo={undoHistory} canRedo />
  {/snippet}
</SpecimenLayout>

<style>
  .poodle-history-center-specimen { display: grid; gap: 2rem; min-height: 40rem; }
  .poodle-history-center-specimen__anchor { display: flex; justify-content: flex-end; width: min(42rem, 100%); }
  .poodle-history-center-specimen__stack { display: grid; gap: 1.5rem; }
  .poodle-history-center-specimen__hint { margin: 0.5rem 0 0; font-size: 0.8125rem; color: var(--poodle-color-text-secondary); }
</style>
