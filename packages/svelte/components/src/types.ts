import type { Component } from "svelte";
import type { Readable } from "svelte/store";

import type { IconNodes } from "./icon-registry";

// Generic in the panel and edge types; this target binds them to its own
// `PanelTabItem` / `DockEdge` further down.
import type {
  DockExternalDragCancelContext as HeadlessDockExternalDragCancelContext,
  DockExternalDragEndContext as HeadlessDockExternalDragEndContext,
  DockExternalDragPreparation as HeadlessDockExternalDragPreparation,
  DockExternalDragPrepareContext as HeadlessDockExternalDragPrepareContext,
  DockExternalDragSource as HeadlessDockExternalDragSource,
  DockExternalDragStartContext as HeadlessDockExternalDragStartContext,
  DockExternalDropContext as HeadlessDockExternalDropContext,
  DockExternalDropEligibilityContext as HeadlessDockExternalDropEligibilityContext,
  DockExternalDropTarget as HeadlessDockExternalDropTarget,
} from "@inflatable-cookie/poodle-core";

export type {
  OverlaySurfaceGeometry,
  OverlaySurfaceGeometryChange,
  OverlaySurfaceGeometryChangeHandler,
  OverlayViewportRect,
} from "@inflatable-cookie/poodle-core";

export type IconProp = IconNodes | string;

export type ValidationState = "none" | "invalid" | "valid" | "pending";
export type AnnouncementMode = "none" | "polite" | "assertive";
export type ValidationSummaryEntry = {
  fieldId: string;
  label: string;
  message: string;
  validationState: ValidationState;
};
export type InputValidationStatus = "idle" | "validating" | "valid" | "invalid";
export type TextInputValidationChange = {
  status: InputValidationStatus;
  valid: boolean;
  message: string;
};

export type ValidationResult = {
  valid: boolean;
  message?: string;
  suggestion?: string;
};

export type InputValidator = (
  value: string,
  context?: unknown,
) => ValidationResult | Promise<ValidationResult>;

export type CalendarWeekStart = "sunday" | "monday";
export type CollapseDirection = "left" | "right" | "up" | "down";
export type SpaceScale = "none" | "sm" | "md" | "lg";
export type ControlSize = "xs" | "sm" | "md" | "lg" | "xl";
export type ControlDensity = "compact" | "default" | "comfortable";
export type SemanticControlSizeRole = "chrome" | "control" | "prominent";
export type ButtonVariant = "primary" | "secondary" | "ghost";
export type RemediationAction = {
  id: string;
  label: string;
  variant: ButtonVariant;
  isDisabled: boolean;
};
export type ButtonTone = "default" | "danger" | "success" | "warning";
export type LayoutAlign = "start" | "center" | "end" | "stretch";
export type LayoutJustify = "start" | "center" | "end" | "between";
export type OverflowMode = "visible" | "hidden" | "clip";
export type SurfaceTone = "canvas" | "panel" | "elevated";
export type SurfaceBorder = "none" | "subtle" | "default";
export type SeparatorTone = "subtle" | "default";
export type ScrollDirection = "vertical" | "horizontal" | "both";
export type StatusTone =
  | "neutral"
  | "info"
  | "success"
  | "warning"
  | "danger"
  | "pending";
export type PillTone = "neutral" | "info" | "success" | "warning" | "danger";
export type PillAppearance = "solid" | "subtle" | "badge";
export type PillSize = "xs" | "sm" | "md" | "lg" | "xl";
export type PillFont = "normal" | "mono";
export type Orientation = "vertical" | "horizontal";
export type TabActivationMode = "automatic" | "manual";
export type OverlayPlacement =
  | "top"
  | "top-start"
  | "top-end"
  | "right"
  | "right-start"
  | "right-end"
  | "bottom"
  | "bottom-start"
  | "bottom-end"
  | "left"
  | "left-start"
  | "left-end";
export type PopoverInitialFocus = "first-focusable" | "content" | "none";
export type DialogKind = "dialog" | "alertdialog";
export type AlertDialogTone = "danger" | "warning";
export type DrawerEdge = "left" | "right" | "top" | "bottom";
export type TriStateValue = "excluded" | "default" | "included";
export type EditableLabelActivationMode =
  | "doubleClick"
  | "enterOrSpace"
  | "programmatic";
export type FormActionAlign = "start" | "end" | "between";
export type FormActionDangerItem = {
  label: string;
  onSelect: () => void;
  value?: string;
  disabled?: boolean;
};
export type CalloutAnnounceMode = AnnouncementMode;
export type ColorInputMode = "hex" | "rgb" | "hsl";
export type SkeletonShape = "line" | "block" | "circle";
export type SkeletonPreset =
  | "table-row"
  | "card"
  | "list-item"
  | "detail-section"
  | "avatar-line";
export type SpinnerVariant = "ring" | "grid" | "dots";
export type SpinnerSize = "xs" | "sm" | "md" | "lg" | "xl";
export type SpinnerTone = "current" | "accent" | "muted";
export type SwitchTone = "default" | "primary" | "success" | "warning" | "danger";

export interface AccordionItem {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

/** One step in a `Stepper`. */
export interface StepperStep {
  value: string;
  label: string;
  /**
   * Given, never derived from position.
   *
   * A step that ran and was rejected has to read as `failed`; deriving state
   * from `index < current` would render it as "not yet reached", which is
   * misleading rather than merely imprecise. See stepper.md §1.
   */
  status: "pending" | "running" | "complete" | "failed";
  isDisabled?: boolean;
  description?: string | null;
}

export interface SegmentedControlOption {
  value: string;
  label: string;
  icon?: IconProp;
  iconOnly?: boolean;
  ariaLabel?: string;
  title?: string;
  disabled?: boolean;
}

export interface ToggleGroupOption {
  value: string;
  label: string;
  ariaLabel?: string;
  disabled?: boolean;
}

export interface TabItem {
  value: string;
  label: string;
  icon?: IconProp;
  disabled?: boolean;
  closable?: boolean;
  count?: number;
  separator?: boolean;
}

export type TabVariant = "card" | "pill" | "block";

/**
 * Selection edge on the active control: `"none"` draws no edge, `"outline"`
 * draws the accent border around the active control, `"underline"` draws the
 * accent edge along the inline-end side. One enum because outline and
 * underline are both borders on the same property and cannot compose. Shared
 * type — see `docs/contracts/004-shared-control-types.md`.
 */
export type ActiveEdge = "none" | "outline" | "underline";

/**
 * Selection treatment on the active control: `"tint"` is the accent-tinted
 * fill; `"solid"` fills with `accent-base` and switches the foreground to
 * `text-inverse`. Shared type — see `docs/contracts/004-shared-control-types.md`.
 */
export type ActiveFill = "tint" | "solid";

/** @deprecated Use TabItem instead */
export type TabDefinition = TabItem;
/** @deprecated Use TabItem instead */
export type TabStripItem = TabItem;

export interface RadioGroupOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface SelectOption {
  value: string;
  label: string;
  description?: string;
  icon?: IconProp;
  disabled?: boolean;
  /** @deprecated Use `disabled` instead */
  isDisabled?: boolean;
  group?: string;
}

export interface SelectOptionGroup {
  label: string;
  options: SelectOption[];
}

export type SelectItems = SelectOption[] | SelectOptionGroup[];

export interface SelectTriggerRenderState {
  selectedOption: SelectOption | null;
  open: boolean;
  placeholder: string | null;
}

export interface SelectOptionRenderState {
  option: SelectOption;
  highlighted: boolean;
  selected: boolean;
  index: number;
}

export interface SelectEmptyRenderState {
  query: string;
}

export interface SelectLoadContext {
  query?: string;
  value?: string | null;
  loadKey?: string | null;
}

/** Async option loader for Select. Returns flat or grouped options. */
export type SelectLoadOptions = (context?: SelectLoadContext) => Promise<SelectItems>;

export interface MenuItem {
  value: string;
  label: string;
  disabled?: boolean;
  checked?: boolean;
  shortcutLabel?: string;
  tone?: "default" | "danger";
  kind?: "action" | "checkbox" | "radio" | "separator";
  /**
   * Nested items turning this entry into a submenu parent. The surface
   * renders a flyout (hover / ArrowRight / Enter) instead of emitting an
   * action for the parent itself; leaf actions bubble to the root menu.
   */
  children?: MenuItem[];
}

export interface NavigationMenuItem {
  value: string;
  label: string;
  disabled?: boolean;
  description?: string;
}

export interface MenubarItem {
  value: string;
  label: string;
  disabled?: boolean;
  items: MenuItem[];
}

export interface TimeZoneOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface PasswordRequirementsPolicy {
  minLength: number;
  requireMixedCase: boolean;
  requireDigit: boolean;
  requireSpecial: boolean;
  minStrengthScore?: number;
  description?: string | null;
}

export interface DateRangeValue {
  start: string | null;
  end: string | null;
}

export interface DateTimeValue {
  date: string | null;
  time: string | null;
}

export interface DateTimeRangeValue {
  start: DateTimeValue;
  end: DateTimeValue;
}

export interface FileUploadItem {
  file: File;
  id: string;
  progress: number;
  status: "pending" | "uploading" | "complete" | "error";
  error?: string;
  previewUrl?: string | null;
  originalFile?: File;
}

export interface ZonedDateTimeValue {
  date: string | null;
  time: string | null;
  timeZone: string | null;
}

export type CardVariant = "default" | "outlined" | "elevated";

export type BulkAction = {
  id: string;
  label: string;
  icon?: IconProp | Component<any>;
  tone?: "default" | "danger" | "warning";
  disabled?: boolean;
};

export type SortDirection = "asc" | "desc";

export type SortField = {
  label: string;
  value?: string;
  key?: string;
  disabled?: boolean;
  defaultDirection?: SortDirection;
};

export type OrderByFieldDefinition = {
  key: string;
  label: string;
  disabled?: boolean;
  defaultDirection?: SortDirection;
};

export type OrderByField = {
  key: string;
  direction: SortDirection;
};

export type OrderByValue = OrderByField[];

export type OrderByTriggerVariant = "summary" | "icon";

export type ActiveSort = {
  field: string;
  direction: SortDirection;
};

// ── FilterBuilder ──────────────────────────────────────────────────────────
export type {
  FilterClause,
  FilterCombinator,
  FilterExpression,
  FilterFieldDefinition,
  FilterFieldKind,
  FilterOperand,
  FilterOperandKind,
  FilterOperatorDefinition,
  FilterOption,
} from "@inflatable-cookie/poodle-core";

// ── ThemeSelect ─────────────────────────────────────────────────────────────
/** Representative colors for a theme's mini preview swatch. */
export type ThemeSwatch = {
  canvas: string;
  surface: string;
  accent: string;
  text: string;
  border: string;
};

/** A selectable theme: `data-theme` value + label + preview swatch. Matches
 * `themeOptions()` from `@inflatable-cookie/poodle-core/tokens`. */
export type ThemeOption = {
  value: string;
  label: string;
  description?: string;
  swatch: ThemeSwatch;
};

export type SplitOrientation = "horizontal" | "vertical";

/** When a SplitView shows its collapse-toggle pill: always, or only while the
 * pointer is on the seam / a toggle holds focus. */
export type SplitToggleVisibility = "always" | "hover";

export type BreadcrumbItem = {
  value: string;
  label: string;
  href?: string;
  current?: boolean;
};

// --- Composite types ---

export type TableSortDirection = "asc" | "desc";
export type TableCellValue = string | number | null;
export type TableFilterType = "text" | "select" | "date";
export type TableFilters = Record<string, string>;
export type BrowseState = "ready" | "empty" | "loading" | "error" | "no-results";
export type MinColumnWidth = "sm" | "md" | "lg";
export type PickerVariant = "inline" | "popover" | "modal";
export type SelectionMode = "single" | "multiple";
export type MediaState = "ready" | "loading" | "error" | "empty";
export type MediaKind = "image" | "audio" | "video" | "document" | "embed" | "pdf" | "other";
export type AspectRatio = "auto" | "square" | "landscape" | "portrait" | "video";
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
export type BlockEditorMode = "single" | "multi";

export type BlockTypeDefinition = {
  type: BlockType;
  label: string;
  icon: IconProp;
};

export type BlockTypeGroup = {
  label: string;
  options: BlockTypeDefinition[];
};

export type BlockTypeItems = BlockTypeDefinition[] | BlockTypeGroup[];

export type EditorBlock = {
  id: string;
  type: BlockType;
  version?: string | number;
  hash?: string | null;
  data?: unknown;
  content?: string;
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

export type {
  DrillDownConfig,
  DrillDownContext,
  DrillDownItem,
  DrillDownItemsFn,
  DrillDownLevel,
  DrillDownSearchFn,
  PickerFilterConfig,
  PickerFilterOption,
  PickerItem,
} from "@inflatable-cookie/poodle-core";

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
  embedType?: "video" | "audio" | "playlist" | "generic";
};

export type EmbedParseResult = {
  success: boolean;
  parsed: ParsedEmbed | null;
  error?: string;
};

export type EmbedMeta = {
  title?: string;
  description?: string;
  duration?: number;
  thumbnailUrl?: string;
  authorName?: string;
};

export type CardRadioItem = {
  value: string;
  label: string;
  description?: string | null;
  disabled?: boolean;
};

export type CardToggleItem = {
  value: string;
  label: string;
  description?: string | null;
  count?: string | number | null;
  disabled?: boolean;
};

export type EditableListItem = {
  id: string;
  label: string;
};

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

export type MessageCenterItem = {
  id: string;
  title: string;
  message?: string | null;
  meta?: string | null;
  timestamp?: Date | string | number | null;
  read: boolean;
  tone?: StatusTone;
  icon?: IconProp | null;
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

export type TreeNode = {
  value: string;
  label: string;
  /** Optional compact metadata aligned to the end of the row. */
  endLabel?: string | null;
  icon?: string | null;
  children?: TreeNode[];
  /** Force branch posture even when `children` is empty (empty / lazy folder). */
  isBranch?: boolean;
  isDisabled?: boolean;
  /** Reduce passive emphasis without disabling interaction. */
  isMuted?: boolean;
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

export type DockEdge = "left" | "right" | "top" | "bottom";
export type PanelVariant = "utility" | "standard" | "focused";
export type DockEmphasis = "standard" | "quiet" | "strong";
export type DockCollapsedPosture = "hidden" | "icon-strip";
export type DockSizing = "static" | "flexible";

export type PanelDragData = {
  panelId: string;
  sourceEdge: DockEdge;
  /**
   * Identifies the exact drag-source zone. Edges are too coarse when a host
   * maps several regions onto one edge (two "top" docks): without this, a
   * cross-region drop looks same-edge and is ignored.
   */
  sourceZone?: string;
};

export type PanelTabItem = {
  value: string;
  label: string;
  icon?: IconProp | null;
  closable?: boolean;
};

/**
 * DockRegion external-drag types.
 *
 * Re-exported from `@inflatable-cookie/poodle-core` rather than redeclared: the session
 * ordering these describe is run by `createDockExternalDragController` there,
 * and a second declaration here would be a second thing to keep in step.
 * `PanelTabItem` and `DockEdge` are the concrete arguments in this target.
 */
export type {
  DockExternalDragCancelReason,
  DockExternalDragController,
} from "@inflatable-cookie/poodle-core";

export type DockExternalDragPrepareContext =
  HeadlessDockExternalDragPrepareContext<PanelTabItem, DockEdge>;
export type DockExternalDragStartContext =
  HeadlessDockExternalDragStartContext<PanelTabItem, DockEdge>;
export type DockExternalDragEndContext =
  HeadlessDockExternalDragEndContext<PanelTabItem, DockEdge>;
export type DockExternalDragCancelContext =
  HeadlessDockExternalDragCancelContext<PanelTabItem, DockEdge>;
export type DockExternalDragPreparation =
  HeadlessDockExternalDragPreparation<PanelTabItem, DockEdge>;
export type DockExternalDragSource =
  HeadlessDockExternalDragSource<PanelTabItem, DockEdge>;
export type DockExternalDropEligibilityContext =
  HeadlessDockExternalDropEligibilityContext<DockEdge>;
export type DockExternalDropContext = HeadlessDockExternalDropContext<DockEdge>;
export type DockExternalDropTarget = HeadlessDockExternalDropTarget<DockEdge>;

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

// ---------------------------------------------------------------------------
// ModelPicker
// ---------------------------------------------------------------------------

export type {
  ModelAxisBinding,
  ModelAxisKind,
  ModelAxisOption,
  ModelAxisRef,
  ModelAxisValue,
  ModelCapabilityAxis,
  ModelImage,
  ModelOption,
  ModelSelection,
} from "@inflatable-cookie/poodle-core";

// ---------------------------------------------------------------------------
// AgentChatInput
// ---------------------------------------------------------------------------

export type AgentChatStatus = "idle" | "busy" | "questioning" | "reviewing-plan";

export type { AgentChatAttachment } from "@inflatable-cookie/poodle-core";

// ---------------------------------------------------------------------------
// RefSelect
// ---------------------------------------------------------------------------

export type RefKind = "branch" | "tag" | "commit";

/** One version-control ref. Poodle knows the shape, never git itself. */
export type RefOption = {
  value: string;
  label: string;
  /** Drives the default glyph. Unknown kinds fall back to the branch glyph. */
  kind?: RefKind;
  /** Secondary line — a short sha, an ahead/behind summary, a commit subject. */
  description?: string;
  /** Overrides the kind glyph. */
  icon?: string;
  group?: string;
  disabled?: boolean;
};

/**
 * Agent transcript types.
 *
 * Re-exported from `@inflatable-cookie/poodle-core` rather than redeclared: grouping,
 * windowing and the Rust mirror all key off these shapes, and a second
 * declaration here would be a second thing to keep in step.
 */
export type {
  AgentPlanDecision,
  AgentPlanSettledStatus,
  AgentPlanStatus,
  AgentQuestionOption,
  AgentQuestionItem,
  AgentQuestionOutcome,
  AgentQuestionAnswer,
  AgentSubagentStatus,
  AgentSubagentItem,
  AnsweredQuestion,
  ChangedFile,
  ChangedFileStatus,
  ToolCallStatus,
  TranscriptActivity,
  TranscriptAnsweredQuestion,
  TranscriptBlock,
  TranscriptChangedFiles,
  TranscriptDecidedPlan,
  TranscriptSubagentGroup,
  TranscriptItem,
  TranscriptMessage,
  TranscriptRole,
  TranscriptToolCall,
  TranscriptToolRun,
} from "@inflatable-cookie/poodle-core";
