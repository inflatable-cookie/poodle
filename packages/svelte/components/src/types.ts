import type { Component } from "svelte";
import type { Readable } from "svelte/store";

import type { IconNodes } from "./icon-registry";

export type IconProp = IconNodes | string;

export type ValidationState = "none" | "invalid" | "valid" | "pending";
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
export type CalloutAnnounceMode = "none" | "polite" | "assertive";
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

export type TabVariant = "text" | "card" | "pill" | "strip" | "block" | "underline";

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
// Generic, renderer-neutral filter-expression model. Poodle understands fields,
// operators, operands and a single root AND/OR combinator. It never understands
// application vocabulary (formats, tags, vendors) and never evaluates the
// expression — the host owns evaluation and serialization.

export type FilterCombinator = "and" | "or";

export type FilterFieldKind = "boolean" | "enum" | "multi-enum" | "text" | "number" | "range";

export type FilterOperandKind = "none" | "text" | "number" | "boolean" | "options" | "range";

/** Discriminated operand payload. Never collapse to `unknown` — cross-renderer
 * semantics stay explicit. */
export type FilterOperand =
  | { kind: "none" }
  | { kind: "text"; value: string }
  | { kind: "number"; value: number }
  | { kind: "boolean"; value: boolean }
  | { kind: "options"; values: string[] }
  | { kind: "range"; min: number | null; max: number | null };

export type FilterOption = {
  value: string;
  label: string;
  disabled?: boolean;
  group?: string;
};

export type FilterOperatorDefinition = {
  key: string;
  label: string;
  operandKind: FilterOperandKind;
};

export type FilterFieldDefinition = {
  key: string;
  label: string;
  kind: FilterFieldKind;
  /** Restrict or relabel the standard operators for this field's kind. */
  operators?: FilterOperatorDefinition[];
  /** Options for `enum` / `multi-enum` fields. */
  options?: FilterOption[];
  /** Operator key selected by default when a draft for this field opens. */
  defaultOperator?: string;
  /** Whether the same field may hold more than one active clause. Default false. */
  allowMultiple?: boolean;
  disabled?: boolean;
};

/** A committed clause. `id` is a stable, opaque UI identity (required because a
 * field with `allowMultiple` may hold several clauses). */
export type FilterClause = {
  id: string;
  key: string;
  operator: string;
  operand: FilterOperand;
};

/** The controlled value. A flat clause list under one root combinator — v1 has
 * no nested Boolean groups, but the shape stays forward-compatible with a future
 * grouped-node model. */
export type FilterExpression = {
  combinator: FilterCombinator;
  clauses: FilterClause[];
};

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
 * `themeOptions()` from `@poodle/svelte-tokens`. */
export type ThemeOption = {
  value: string;
  label: string;
  description?: string;
  swatch: ThemeSwatch;
};

export type SplitOrientation = "horizontal" | "vertical";

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

export type PickerItem = {
  id: string;
  label: string;
  description?: string | null;
  meta?: string | null;
  disabled?: boolean;
};

export type PickerFilterOption = {
  id: string;
  label: string;
};

export type PickerFilterConfig = {
  key: string;
  label: string;
  options: PickerFilterOption[];
  includeAll?: boolean;
  allLabel?: string;
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
};

export type PanelTabItem = {
  value: string;
  label: string;
  icon?: IconProp | null;
  closable?: boolean;
};

export type DockExternalDragCancelReason =
  | "superseded"
  | "pointer-released"
  | "pointer-cancelled"
  | "not-ready"
  | "unmounted";

export type DockExternalDragPrepareContext = {
  panel: PanelTabItem;
  sourceEdge: DockEdge;
  event: PointerEvent;
  signal: AbortSignal;
};

export type DockExternalDragStartContext = {
  panel: PanelTabItem;
  sourceEdge: DockEdge;
  event: DragEvent;
  dataTransfer: DataTransfer;
};

export type DockExternalDragEndContext = {
  panel: PanelTabItem;
  sourceEdge: DockEdge;
  event: DragEvent;
  dropEffect: DataTransfer["dropEffect"];
};

export type DockExternalDragCancelContext = {
  panel: PanelTabItem;
  sourceEdge: DockEdge;
  reason: DockExternalDragCancelReason;
};

export type DockExternalDragPreparation = {
  start: (context: DockExternalDragStartContext) => void;
  end?: (context: DockExternalDragEndContext) => void | Promise<void>;
  cancel?: (context: DockExternalDragCancelContext) => void | Promise<void>;
};

export type DockExternalDragSource = {
  prepare: (
    context: DockExternalDragPrepareContext,
  ) =>
    | DockExternalDragPreparation
    | null
    | Promise<DockExternalDragPreparation | null>;
  onPrepareError?: (
    error: unknown,
    context: DockExternalDragPrepareContext,
  ) => void;
};

export type DockExternalDropEligibilityContext = {
  phase: "over" | "drop";
  targetEdge: DockEdge;
  event: DragEvent;
  dataTransfer: DataTransfer;
};

export type DockExternalDropContext = {
  targetEdge: DockEdge;
  event: DragEvent;
  dataTransfer: DataTransfer;
};

export type DockExternalDropTarget = {
  canDrop: (context: DockExternalDropEligibilityContext) => boolean;
  drop: (context: DockExternalDropContext) => void | Promise<void>;
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

// ---------------------------------------------------------------------------
// ModelPicker
// ---------------------------------------------------------------------------

/** One selectable engine/model. Poodle knows nothing about what a model *is* —
 * labels, descriptions, badges and grouping are all host vocabulary. */
export type ModelOption = {
  value: string;
  label: string;
  description?: string;
  badge?: string;
  icon?: string;
  group?: string;
  disabled?: boolean;
  /** Arbitrary image (a provider logo, say) shown in place of `icon`. Takes
   * precedence over `icon` when both are set. */
  image?: ModelImage;
  /** Which capability axes this model exposes, by key, in display order. Each
   * entry is either an axis key or a binding that overrides parts of the shared
   * definition for this model (different levels, a different default). Omit to
   * inherit every declared axis — the single-provider case. */
  axes?: ModelAxisRef[];
};

/** An image source for a model row. `alt` defaults to `""`: the model label sits
 * beside it, so the image is decorative unless the host says otherwise. */
export type ModelImage = { src: string; alt?: string };

export type ModelAxisKind = "select" | "toggle";

export type ModelAxisOption = {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
};

/** A host-declared capability axis, keyed. The key is what lands in
 * `ModelSelection.axes`, so hosts can read `selection.axes.effort` across
 * providers whose level sets differ. */
export type ModelCapabilityAxis = {
  key: string;
  label: string;
  kind: ModelAxisKind;
  description?: string;
  /** `select` axes only. */
  options?: ModelAxisOption[];
  /** Control shape for a `select` axis. `auto` (the default) uses a
   * SegmentedControl up to three options and a vertical list beyond that. */
  control?: "auto" | "segmented" | "list";
  /** Applied when the axis has no value for the selected model. */
  defaultValue?: ModelAxisValue;
  /** Trigger-summary labels for a `toggle` axis. */
  onLabel?: string;
  offLabel?: string;
  /** Default true. */
  showInSummary?: boolean;
  disabled?: boolean;
};

/** A model's reference to a declared axis, overriding any part of it for that
 * model. Everything but `key` is optional and falls back to the shared
 * definition. */
export type ModelAxisBinding = {
  key: string;
  label?: string;
  description?: string;
  options?: ModelAxisOption[];
  control?: "auto" | "segmented" | "list";
  defaultValue?: ModelAxisValue;
  onLabel?: string;
  offLabel?: string;
  showInSummary?: boolean;
  disabled?: boolean;
};

export type ModelAxisRef = string | ModelAxisBinding;

export type ModelAxisValue = string | boolean;

export type ModelSelection = { model: string; axes: Record<string, ModelAxisValue> };

// ---------------------------------------------------------------------------
// AgentChatInput
// ---------------------------------------------------------------------------

export type AgentChatStatus = "idle" | "busy" | "questioning";

export type AgentChatAttachment = {
  id: string;
  label: string;
  /** Host-defined kind, surfaced as `data-kind` for styling hooks. */
  kind?: string;
  icon?: string;
  /** Image source for a visual attachment. When set the chip is replaced by a
   * thumbnail tile — an image says more than its filename does. */
  thumbnailUrl?: string;
  disabled?: boolean;
};

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
 * Re-exported from `@poodle/headless` rather than redeclared: grouping,
 * windowing and the Rust mirror all key off these shapes, and a second
 * declaration here would be a second thing to keep in step.
 */
export type {
  AgentQuestionOption,
  AgentQuestionItem,
  AgentQuestionOutcome,
  AgentQuestionAnswer,
  AnsweredQuestion,
  ChangedFile,
  ChangedFileStatus,
  ToolCallStatus,
  TranscriptActivity,
  TranscriptAnsweredQuestion,
  TranscriptBlock,
  TranscriptChangedFiles,
  TranscriptItem,
  TranscriptMessage,
  TranscriptRole,
  TranscriptToolCall,
  TranscriptToolRun,
} from "@poodle/headless";
