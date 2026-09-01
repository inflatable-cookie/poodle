import { getContext, setContext } from "svelte";
import type { DragDropCommitResult } from "@inflatable-cookie/poodle-core";

/**
 * DockRegion's insert hook into a Tabs strip it owns.
 *
 * Not a public Tabs API: the contract lives on DockRegion (`canAcceptPanel` /
 * `onPanelDrop`). Tabs reads this only when composed inside a region.
 */
export interface TabsForeignInsert {
  canAccept: (subjectId: string) => boolean;
  commit: (subjectId: string, index: number) => DragDropCommitResult;
}

const KEY = Symbol("poodle-tabs-foreign-insert");

export function setTabsForeignInsert(value: TabsForeignInsert | null): void {
  setContext(KEY, value);
}

export function getTabsForeignInsert(): TabsForeignInsert | null {
  return getContext<TabsForeignInsert | null>(KEY) ?? null;
}
