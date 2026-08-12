<script lang="ts">
  import type {
    HistoryCenterRejectionCode,
    HistoryContinuation,
    HistoryPathPage,
  } from "@inflatable-cookie/poodle-core";

  import HistoryCenter from "../src/HistoryCenter.svelte";

  // A minimal host simulation for the flow tests: the three host operations
  // resolve synchronously from fixture maps and feed their results back as
  // `continuationsResult` / `runResult`, exactly as a real host would after
  // its async call. `onLoadContinuations` / `onLoadContinuationRun` are
  // optional spies; `onCheckoutContinuation` passes through for assertions.
  let {
    pages = null,
    continuationsByEntry = {} as Record<string, HistoryContinuation[]>,
    runsByFork = {} as Record<string, HistoryPathPage[]>,
    defaultOpen = false,
    rejection = null,
    onLoadContinuations = null,
    onLoadContinuationRun = null,
    onNavigateEntry = null,
    onRenameBranch = null,
    onCheckoutContinuation = null,
    onDeleteContinuation = null,
    onOpenChange = null,
  }: {
    pages?: HistoryPathPage[] | null;
    continuationsByEntry?: Record<string, HistoryContinuation[]>;
    runsByFork?: Record<string, HistoryPathPage[]>;
    defaultOpen?: boolean;
    rejection?: HistoryCenterRejectionCode | null;
    onLoadContinuations?: ((entryId: string) => void) | null;
    onLoadContinuationRun?: ((fromEntryId: string) => void) | null;
    onNavigateEntry?: ((branchId: string | null, entryId: string) => void) | null;
    onRenameBranch?: ((branchId: string, name: string) => void) | null;
    onCheckoutContinuation?: ((entryId: string) => void) | null;
    onDeleteContinuation?: ((entryId: string) => void) | null;
    onOpenChange?: ((open: boolean) => void) | null;
  } = $props();

  let continuationsResult = $state<{ entryId: string; continuations: HistoryContinuation[] } | null>(null);
  let runResult = $state<{ fromEntryId: string; pages: HistoryPathPage[] } | null>(null);

  function loadContinuations(entryId: string): void {
    onLoadContinuations?.(entryId);
    continuationsResult = { entryId, continuations: continuationsByEntry[entryId] ?? [] };
  }

  function loadRun(fromEntryId: string): void {
    onLoadContinuationRun?.(fromEntryId);
    runResult = { fromEntryId, pages: runsByFork[fromEntryId] ?? [] };
  }
</script>

<HistoryCenter
  {pages}
  {defaultOpen}
  {rejection}
  {continuationsResult}
  {runResult}
  {onNavigateEntry}
  {onRenameBranch}
  {onCheckoutContinuation}
  {onDeleteContinuation}
  {onOpenChange}
  onLoadContinuations={loadContinuations}
  onLoadContinuationRun={loadRun}
/>
