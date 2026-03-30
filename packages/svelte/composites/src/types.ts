import type { Readable } from "svelte/store";

import type { IconProp } from "@poodle/svelte-primitives/types";

export type TableSortDirection = "asc" | "desc";
export type TableCellValue = string | number | null;
export type TableFilterType = "text" | "select" | "date";
export type TableFilters = Record<string, string>;
export type BrowseState = "ready" | "empty" | "loading" | "error" | "no-results";
export type MinColumnWidth = "sm" | "md" | "lg";
export type { CardVariant } from "@poodle/svelte-primitives";
export type PickerVariant = "inline" | "popover" | "modal";
export type SelectionMode = "single" | "multiple";
export type MediaState = "ready" | "loading" | "error" | "empty";
export type MediaKind = "image" | "audio" | "video" | "document" | "embed";
export type AspectRatio = "square" | "landscape" | "portrait" | "video";
export type EmptyStateVariant = "neutral" | "search" | "firstRun";
export type EmptyStateSize = "default" | "compact";
export type ToastTone = "info" | "success" | "warning" | "danger";
export type LogLevel = "info" | "warn" | "error";
export type LogActionType =
  | "create"
  | "update"
  | "delete"
  | "restore"
  | "upload"
  | "login"
  | "logout"
  | "security"
  | "other";

export type StreamLogEntry = {
  id?: string;
  timestamp: Date | string | number;
  level: LogLevel;
  message: string;
};

export type LogActor = {
  id: string;
  email?: string;
  name?: string;
};

export type LogFilter = {
  field: string;
  label: string;
  type: "select" | "date";
  options?: { value: string; label: string }[];
  placeholder?: string;
};

export type AuditLogEntry = {
  id: string;
  occurredAt: string;
  actor?: LogActor | null;
  action: string;
  resourceType: string;
  resourceId: string;
  resourceLabel?: string;
  details?: Record<string, unknown>;
};

export type LogEntry = StreamLogEntry | AuditLogEntry;

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
  align?: "start" | "center" | "end";
  sortable?: boolean;
  hideable?: boolean;
  width?: string;
  minWidth?: string;
  hideOnMobile?: boolean;
  isRowHeader?: boolean;
  filterable?: boolean;
  filterType?: TableFilterType;
  filterOptions?: Array<{ value: string; label: string } | string>;
};

export type TableRow<TData = unknown> = {
  id: string;
  cells: Record<string, TableCellValue>;
  summary?: string | null;
  data?: TData;
};

export type TableRowAction = {
  value: string;
  label: string;
  disabled?: boolean;
  kind?: "action" | "separator";
  href?: string | null;
  shortcutLabel?: string;
  tone?: "default" | "danger";
  hidden?: boolean;
};

export type TablePagination = {
  page: number;
  limit: number;
  total: number;
};

export type { BulkAction } from "@poodle/svelte-primitives";

export type BreadcrumbItem = {
  value: string;
  label: string;
  href?: string;
  current?: boolean;
};

export type PickerItem = {
  id: string;
  label: string;
  description?: string | null;
  meta?: string | null;
};

export type DrillDownItem = PickerItem & {
  count?: number;
  expandable?: boolean;
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
  meta?: string | null;
};

export type MediaUploadWorkflowStep =
  | "checking"
  | "duplicate"
  | "uploading"
  | "finalising"
  | "complete"
  | "error";

export type MediaWorkflowPaginationParams = {
  cursor?: string;
  limit?: number;
};

export type MediaWorkflowPageResponse<TItem> = {
  data: TItem[];
  nextCursor?: string | null;
  hasMore?: boolean;
};

export type MediaBrowseState<TItem> = {
  items: TItem[];
  nextCursor: string | null;
  hasMore: boolean;
};

export type MediaDuplicateCheckResult<TExisting> = {
  exists: boolean;
  item?: TExisting | null;
};

export type MediaUploadProgress = {
  loaded: number;
  total: number;
  percent: number;
};

export type MediaUploadPlan = {
  uploadUrl: string;
  method: string;
  headers?: Record<string, string> | null;
  expiresAt: string;
  maxBytes: number;
  allowedContentTypes?: string[] | null;
  objectKey?: string;
};

export type MediaUploadInitResult = {
  versionId: string;
  uploadPlan: MediaUploadPlan;
};

export type MediaUploadDisplayStep = "select" | MediaUploadWorkflowStep;

export type MediaUploadDuplicateResult<TExisting> = {
  kind: "duplicate";
  fileHash: string;
  existingItem: TExisting;
};

export type MediaUploadCompleteResult<TCreated> = {
  kind: "uploaded";
  fileHash: string;
  createdItem: TCreated;
};

export type MediaUploadWorkflowResult<TExisting, TCreated> =
  | MediaUploadDuplicateResult<TExisting>
  | MediaUploadCompleteResult<TCreated>;

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
  disabled?: boolean;
};

export type ReorderableItem = {
  id: string;
  label: string;
};

export type {
  ActiveSort,
  OrderByField,
  OrderByFieldDefinition,
  OrderByValue,
  SortDirection,
  SortField,
} from "@poodle/svelte-primitives";

export type DiscoveryState = "ready" | "loading" | "error" | "empty" | "no-results";

export type CommandActionItem = {
  id: string;
  title: string;
  description?: string | null;
  group?: string | null;
  shortcut?: string | null;
  keywords?: string[];
  badge?: string | null;
  disabled?: boolean;
};

export type SidebarNavItem = {
  value: string;
  label: string;
  href?: string | null;
  disabled?: boolean;
};

export type SidebarNavGroup = {
  id: string;
  label?: string | null;
  items: SidebarNavItem[];
};

export type ToastItem = {
  id: string;
  title: string;
  message?: string | null;
  tone?: ToastTone;
  actionLabel?: string | null;
};

export type ToastHostPlacement = "bottom-end" | "bottom-start" | "top-end" | "top-start";

export type ToastHostStoreItem = {
  id: string;
  title?: string;
  message: string;
  tone?: ToastTone;
  variant?: "info" | "success" | "warning" | "error" | "danger";
  actionLabel?: string | null;
  sticky?: boolean;
};

export type ToastHostStore = {
  toasts: Readable<ToastHostStoreItem[]>;
  dismiss: (id: string) => void;
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
  closable?: boolean;
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
