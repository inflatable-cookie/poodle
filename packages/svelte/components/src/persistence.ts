import type { WorkspaceLayoutSnapshot } from "./types";

export function serializeWorkspaceLayoutSnapshot(snapshot: WorkspaceLayoutSnapshot): string {
  return JSON.stringify(snapshot, null, 2);
}

export function parseWorkspaceLayoutSnapshot(serialized: string): WorkspaceLayoutSnapshot {
  return JSON.parse(serialized) as WorkspaceLayoutSnapshot;
}
