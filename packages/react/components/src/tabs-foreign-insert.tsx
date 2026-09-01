import { createContext, useContext, type ReactNode } from "react";
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

const TabsForeignInsertContext = createContext<TabsForeignInsert | null>(null);

export function TabsForeignInsertProvider({
  value,
  children,
}: {
  value: TabsForeignInsert;
  children: ReactNode;
}) {
  return (
    <TabsForeignInsertContext.Provider value={value}>{children}</TabsForeignInsertContext.Provider>
  );
}

export function useTabsForeignInsert(): TabsForeignInsert | null {
  return useContext(TabsForeignInsertContext);
}
