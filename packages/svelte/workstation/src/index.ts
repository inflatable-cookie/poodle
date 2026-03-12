export { default as ActionDiscoveryPanel } from "./ActionDiscoveryPanel.svelte";
export { default as AppHeader } from "./AppHeader.svelte";
export { default as CommandPalette } from "./CommandPalette.svelte";
export { default as DockRegion } from "./DockRegion.svelte";
export { default as PanelHeader } from "./PanelHeader.svelte";
export { default as PanelSurface } from "./PanelSurface.svelte";
export { default as PanelTabs } from "./PanelTabs.svelte";
export { default as ProjectHeader } from "./ProjectHeader.svelte";
export { default as ShellStatusBar } from "./ShellStatusBar.svelte";
export { default as SplitView } from "./SplitView.svelte";
export { default as SurfaceTabs } from "./SurfaceTabs.svelte";
export { default as WorkspaceShell } from "./WorkspaceShell.svelte";
export {
  parseWorkspaceLayoutSnapshot,
  serializeWorkspaceLayoutSnapshot,
} from "./persistence";
export type {
  ActionDiscoverySection,
  CommandActionItem,
  DockEdge,
  DockRegionSnapshot,
  DiscoveryState,
  PanelTabItem,
  SplitOrientation,
  SurfaceTabItem,
  WorkspaceLayoutSnapshot,
  WorkspaceShellState,
} from "./types";
