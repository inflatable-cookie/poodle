// Expected-failure half of the packed proof, package root (g16.033).
//
// The retired v2 field must not be reachable through the packed root export.
// This file is compiled on its own and MUST fail with a real diagnostic: it
// carries no compiler-suppression comment, no escape-hatch type, and no cast.
// The harness scans this source for those and asserts the exact diagnostic.
import type { HistoryEntry } from "@inflatable-cookie/poodle-svelte";

export function readBranchCount(entry: HistoryEntry): number {
  return entry.branchCount;
}
