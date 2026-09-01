// Expected-failure half of the packed proof, `/types` subpath (g16.033).
//
// Same claim as the root negative, one import path over. Compiled on its own,
// with no compiler-suppression comment, escape-hatch type, or cast, so the
// failure is the packed type surface rather than a silenced check.
import type { HistoryEntry } from "@inflatable-cookie/poodle-svelte/types";

export function readBranchCount(entry: HistoryEntry): number {
  return entry.branchCount;
}
