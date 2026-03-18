export { default as DockRegion } from "./DockRegion.svelte";
export { default as PanelSurface } from "./PanelSurface.svelte";
export { default as ProjectHeader } from "./ProjectHeader.svelte";
export { default as ResizeHandle } from "./ResizeHandle.svelte";
export { default as ShellStatusBar } from "./ShellStatusBar.svelte";
export { default as SplitDivider } from "./SplitDivider.svelte";
export { default as SplitView } from "./SplitView.svelte";
export { default as StripRail } from "./StripRail.svelte";
export { default as WorkspaceShell } from "./WorkspaceShell.svelte";
export { default as WorkspaceWindow } from "./WorkspaceWindow.svelte";
export {
  parseWorkspaceLayoutSnapshot,
  serializeWorkspaceLayoutSnapshot,
} from "./persistence";
export type {
  CenterRegionSnapshot,
  DockCollapsedPosture,
  DockEdge,
  DockEmphasis,
  DockRegionSnapshot,
  DockSizing,
  PanelDragData,
  PanelTabItem,
  PanelVariant,
  SplitOrientation,
  StripEdge,
  StripItem,
  StripMode,
  StripRegionSnapshot,
  WindowSurface,
  WorkspaceLayoutSnapshot,
  WorkspaceShellState,
} from "./types";
