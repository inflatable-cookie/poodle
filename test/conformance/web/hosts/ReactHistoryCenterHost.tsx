/**
 * React HistoryCenter conformance host (g14.007). The React half of the same
 * boundary the Svelte host implements: it answers the two named commands from
 * the case's declared host records and records every command with its payload.
 */

import { useCallback, useState } from "react";

import { HistoryCenter } from "../../../../packages/react/components/src/HistoryCenter";

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

export function ReactHistoryCenterHost({
  props = {},
  host = {},
  trace,
}: {
  props?: Record<string, unknown>;
  host?: Record<string, unknown>;
  trace: Trace[];
}) {
  const forks = (host.continuations ?? []) as Fork[];
  const runEntries = (host.runEntries ?? []) as RunEntry[];
  const [continuationsResult, setContinuationsResult] = useState<unknown>(null);
  const [runResult, setRunResult] = useState<unknown>(null);

  const record = useCallback(
    (event: string, payload?: Record<string, unknown>) => {
      trace.push(payload === undefined ? { event } : { event, payload });
    },
    [trace],
  );

  const loadContinuations = useCallback(
    (entryId: string) => {
      record("loadContinuations", { entryId });
      // Every child of the anchor, the one already on the list included:
      // filtering that out by id is the component's job.
      setContinuationsResult({
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
      });
    },
    [forks, record],
  );

  const loadContinuationRun = useCallback(
    (fromEntryId: string) => {
      record("loadContinuationRun", { fromEntryId });
      const entries = runEntries
        .filter((entry) => entry.fromEntryId === fromEntryId)
        .map(({ fromEntryId: _from, ...entry }) => entry);
      setRunResult(
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
            },
      );
    },
    [record, runEntries],
  );

  return (
    <HistoryCenter
      {...(props as Record<string, never>)}
      continuationsResult={continuationsResult as never}
      runResult={runResult as never}
      onUndo={() => record("undo")}
      onRedo={() => record("redo")}
      onOpenChange={(open: boolean) => record("openChange", { open })}
      onNavigateEntry={(branchId: string | null, entryId: string) =>
        record("navigateEntry", { branchId: branchId ?? "", entryId })
      }
      onRenameBranch={(branchId: string, name: string) =>
        record("renameBranch", { branchId, name })
      }
      onLoadContinuations={loadContinuations}
      onLoadContinuationRun={loadContinuationRun}
      onCheckoutContinuation={(entryId: string) =>
        record("checkoutContinuation", { entryId })
      }
    />
  );
}
