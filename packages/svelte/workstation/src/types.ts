export type DiscoveryState = "ready" | "loading" | "error" | "empty" | "no-results";
export type WorkspaceShellState = "ready" | "loading" | "empty" | "offline" | "disconnected";
export type SplitOrientation = "horizontal" | "vertical";
export type DockEdge = "left" | "right" | "top" | "bottom";

export type CommandActionItem = {
  id: string;
  title: string;
  description?: string | null;
  group?: string | null;
  shortcut?: string | null;
  keywords?: string[];
  badge?: string | null;
  isDisabled?: boolean;
};

export type ActionDiscoverySection = {
  id: string;
  title: string;
  description?: string | null;
  actions: CommandActionItem[];
};

export type PanelTabItem = {
  value: string;
  label: string;
  icon?: string | null;
  isClosable?: boolean;
};

export type SurfaceTabItem = {
  value: string;
  label: string;
  isClosable?: boolean;
};

export type DockRegionSnapshot = {
  edge: DockEdge;
  isCollapsed: boolean;
  activePanel: string | null;
  order: string[];
};

export type WorkspaceLayoutSnapshot = {
  version: 1;
  activeSurface: string;
  surfaceOrder: string[];
  primarySplitRatio: number;
  secondarySplitRatio: number;
  leftDock: DockRegionSnapshot;
  rightDock: DockRegionSnapshot;
};
