export type WorkspaceShellState = "ready" | "loading" | "empty" | "offline" | "disconnected";
export type SplitOrientation = "horizontal" | "vertical";
export type DockEdge = "left" | "right" | "top" | "bottom";
export type StripEdge = "top" | "bottom" | "left" | "right";
export type StripMode = "icon" | "mixed";
export type HostedSurfaceState = "ready" | "loading" | "unavailable" | "blocked" | "degraded";
export type PanelVariant = "utility" | "standard" | "focused";
export type DockEmphasis = "standard" | "quiet" | "strong";
export type DockCollapsedPosture = "hidden" | "icon-strip";
export type DockSizing = "static" | "flexible";

export type PanelDragData = {
  panelId: string;
  sourceEdge: DockEdge;
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

export type WindowSurface = {
  id: string;
  label: string;
  isClosable?: boolean;
};

export type StripItem = {
  id: string;
  label: string;
  icon: string;
  badge?: string | null;
  group?: string | null;
  isDisabled?: boolean;
};

// --- Snapshot types (v2) ---

export type StripRegionSnapshot = {
  isCollapsed: boolean;
  activeItem: string | null;
};

export type CenterRegionSnapshot = {
  activeSurface: string | null;
};

export type DockRegionSnapshot = {
  edge: DockEdge;
  isCollapsed: boolean;
  activePanel: string | null;
  order: string[];
  tabsPlacement?: "edge" | "top";
};

export type WorkspaceLayoutSnapshot = {
  version: 1 | 2;
  activeSurface: string;
  surfaceOrder: string[];
  // v1 fields (kept for backwards compatibility)
  primarySplitRatio?: number;
  secondarySplitRatio?: number;
  leftDock?: DockRegionSnapshot;
  rightDock?: DockRegionSnapshot;
  // v2 fields
  regions?: {
    topStrip?: StripRegionSnapshot;
    bottomStrip?: StripRegionSnapshot;
    leftStrip?: StripRegionSnapshot;
    rightStrip?: StripRegionSnapshot;
    left?: DockRegionSnapshot;
    right?: DockRegionSnapshot;
    top?: DockRegionSnapshot;
    bottom?: DockRegionSnapshot;
    centerTop?: CenterRegionSnapshot;
    centerBottom?: CenterRegionSnapshot;
  };
  splitRatios?: {
    primary: number;
    secondary: number;
  };
};
