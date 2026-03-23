import type { IconProp } from "@flint/svelte-primitives/types";

export type TableSortDirection = "asc" | "desc";
export type BrowseState = "ready" | "empty" | "loading" | "error" | "no-results";
export type MinColumnWidth = "sm" | "md" | "lg";
export type { CardVariant } from "@flint/svelte-primitives";
export type PickerVariant = "inline" | "popover" | "modal";
export type SelectionMode = "single" | "multiple";
export type MediaState = "ready" | "loading" | "error" | "empty";
export type MediaKind = "image" | "audio" | "video" | "document" | "embed";
export type AspectRatio = "square" | "landscape" | "portrait" | "video";
export type EmptyStateVariant = "neutral" | "search" | "firstRun";
export type ToastTone = "info" | "success" | "warning" | "danger";
export type LogLevel = "info" | "warn" | "error";

export type LogEntry = {
  id?: string;
  timestamp: Date | string | number;
  level: LogLevel;
  message: string;
};

export type BlockType = string;

export type BlockTypeDefinition = {
  type: BlockType;
  label: string;
  icon: IconProp;
};

export type EditorBlock = {
  id: string;
  type: BlockType;
  content: string;
  [key: string]: unknown;
};

export type TableColumn = {
  id: string;
  label: string;
  align?: "start" | "end";
  isSortable?: boolean;
  isHideable?: boolean;
};

export type TableRow = {
  id: string;
  cells: Record<string, string>;
  summary?: string | null;
};

export type { BulkAction } from "@flint/svelte-primitives";

export type BreadcrumbItem = {
  value: string;
  label: string;
  href?: string;
  isCurrent?: boolean;
};

export type PickerItem = {
  id: string;
  label: string;
  description?: string | null;
  meta?: string | null;
};

export type DrillDownItem = PickerItem & {
  count?: number;
  hasChildren?: boolean;
};

export type DrillDownContext = Record<string, string>;

export type DrillDownSearchFn = (
  query: string,
  context: DrillDownContext,
) => DrillDownItem[] | Promise<DrillDownItem[]>;

export type DrillDownLevel = {
  key: string;
  label: string;
  items: DrillDownItem[] | DrillDownSearchFn;
  searchPlaceholder?: string;
};

export type DrillDownItemsFn = (
  query: string,
  context: DrillDownContext,
) => PickerItem[] | Promise<PickerItem[]>;

export type DrillDownConfig = {
  levels: DrillDownLevel[];
  finalItems?: DrillDownItemsFn;
};

export type MediaPickerItem = {
  id: string;
  label: string;
  thumbnailUrl?: string | null;
  mimeType?: string | null;
  kind?: MediaKind;
};

export type ParsedEmbed = {
  provider: string;
  id: string;
  originalUrl?: string;
  originalEmbed?: string;
  width?: number;
  height?: number;
};

export type CardRadioItem = {
  value: string;
  label: string;
  description?: string | null;
  isDisabled?: boolean;
};

export type ReorderableItem = {
  id: string;
  label: string;
};

export type { SortField, ActiveSort } from "@flint/svelte-primitives";

export type DiscoveryState = "ready" | "loading" | "error" | "empty" | "no-results";

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

export type ToastItem = {
  id: string;
  title: string;
  message?: string | null;
  tone?: ToastTone;
  actionLabel?: string | null;
};

// --- Layout / Dock types ---

export type SplitOrientation = "horizontal" | "vertical";
export type DockEdge = "left" | "right" | "top" | "bottom";
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
  icon?: IconProp | null;
  isClosable?: boolean;
};

// --- Snapshot types ---

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
  primarySplitRatio?: number;
  secondarySplitRatio?: number;
  leftDock?: DockRegionSnapshot;
  rightDock?: DockRegionSnapshot;
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
