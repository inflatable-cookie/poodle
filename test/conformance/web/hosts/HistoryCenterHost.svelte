<script lang="ts">
  /**
   * Svelte HistoryCenter conformance host (g14.007).
   *
   * The host half of the boundary: it holds the fork catalogue the case
   * declared, answers `onLoadContinuations` and `onLoadContinuationRun` from
   * it, and records every command with the payload it carried. It answers only
   * when the component asks — nothing here loads ahead of a disclosure.
   *
   * It hands back every child of the anchor, the one already on the list
   * included. Filtering that one out by id is the component's job, and a host
   * that pre-filtered would hide a bug rather than expose one.
   */
  import HistoryCenter from "../../../../packages/svelte/components/src/HistoryCenter.svelte";
  import type {
    HistoryContinuation,
    HistoryPathPage,
  } from "../../../../packages/core/src/history-center";

  type Fork = {
    anchorEntryId: string;
    entryId: string;
    label: string;
    preferred: boolean;
    entryCount: number;
    branchId: string;
    branchName?: string;
  };
  type RunEntry = {
    fromEntryId: string;
    id: string;
    label: string;
    position: "past" | "current" | "future";
    checkpoint?: boolean;
    continuationCount: number;
  };
  type Trace = { event: string; payload?: Record<string, unknown> };

  const {
    props = {},
    host = {},
    trace,
  }: {
    props?: Record<string, unknown>;
    host?: Record<string, unknown>;
    trace: Trace[];
  } = $props();

  const forks = (host.continuations ?? []) as Fork[];
  const runEntries = (host.runEntries ?? []) as RunEntry[];

  let continuationsResult = $state<{
    entryId: string;
    continuations: HistoryContinuation[];
  } | null>(null);
  let runResult = $state<{ fromEntryId: string; pages: HistoryPathPage[] } | null>(null);

  function record(event: string, payload?: Record<string, unknown>): void {
    trace.push(payload === undefined ? { event } : { event, payload });
  }

  function loadContinuations(entryId: string): void {
    record("loadContinuations", { entryId });
    continuationsResult = {
      entryId,
      continuations: forks
        .filter((fork) => fork.anchorEntryId === entryId)
        .map((fork) => ({
          entryId: fork.entryId,
          label: fork.label,
          preferred: fork.preferred,
          entryCount: fork.entryCount,
          branchId: fork.branchId,
          branchName: fork.branchName ?? null,
        })),
    };
  }

  function loadContinuationRun(fromEntryId: string): void {
    record("loadContinuationRun", { fromEntryId });
    // Entries are supplied newest first, which is the order a real page
    // arrives in; the component owns the one reversal.
    const entries = runEntries
      .filter((entry) => entry.fromEntryId === fromEntryId)
      .map(({ fromEntryId: _from, ...entry }) => entry);
    runResult =
      entries.length === 0
        ? null
        : {
            fromEntryId,
            pages: [
              {
                entries,
                offset: 0,
                precedingContinuationCount: 0,
                truncatedBefore: false,
                truncatedAfter: false,
              },
            ],
          };
  }
</script>

<HistoryCenter
  {...props}
  {continuationsResult}
  {runResult}
  onUndo={() => record("undo")}
  onRedo={() => record("redo")}
  onOpenChange={(open) => record("openChange", { open })}
  onNavigateEntry={(branchId, entryId) =>
    record("navigateEntry", { branchId: branchId ?? "", entryId })}
  onRenameBranch={(branchId, name) => record("renameBranch", { branchId, name })}
  onLoadContinuations={loadContinuations}
  onLoadContinuationRun={loadContinuationRun}
  onCheckoutContinuation={(entryId) => record("checkoutContinuation", { entryId })}
/>
