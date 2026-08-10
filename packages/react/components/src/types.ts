/**
 * Shared prop types for `@inflatable-cookie/poodle-react`. Shapes mirror the Svelte package's
 * `types.ts` (interface-invariance rule, g11.001) with React idioms:
 * `Snippet` becomes `ReactNode`. Grows per conversion batch.
 */

import type { ReactNode } from "react";
import type { IconNodes } from "@inflatable-cookie/poodle-core/icons";

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

export type ControlSize = "xs" | "sm" | "md" | "lg" | "xl";
export type ControlDensity = "compact" | "default" | "comfortable";
export type SemanticControlSizeRole = "control" | "chrome" | "prominent";

export type StatusTone =
  | "neutral"
  | "info"
  | "success"
  | "warning"
  | "danger"
  | "pending";
export type SeparatorTone = "subtle" | "default";
export type SkeletonShape = "line" | "block" | "circle";

export type { IconNodeElement, IconNodes, IconSet } from "@inflatable-cookie/poodle-core/icons";
export type IconProp = IconNodes | string;

export type PillTone = "neutral" | "info" | "success" | "warning" | "danger";
export type PillAppearance = "solid" | "subtle" | "badge";
export type PillSize = "xs" | "sm" | "md" | "lg" | "xl";
export type PillFont = "normal" | "mono";
export type PillTypography = "label" | "inherit";

export type SkeletonPreset =
  | "table-row"
  | "card"
  | "list-item"
  | "detail-section"
  | "avatar-line";
export type SpinnerVariant = "ring" | "grid" | "dots";
export type SpinnerSize = "xs" | "sm" | "md" | "lg" | "xl";
export type SpinnerTone = "current" | "accent" | "muted";

export type SpaceScale = "none" | "sm" | "md" | "lg";
export type LayoutAlign = "start" | "center" | "end" | "stretch";
export type LayoutJustify = "start" | "center" | "end" | "between";
export type OverflowMode = "visible" | "hidden" | "clip";

export type ButtonVariant = "primary" | "secondary" | "ghost";
export type RemediationAction = {
  id: string;
  label: string;
  variant: ButtonVariant;
  isDisabled: boolean;
};
export type ButtonTone = "default" | "danger" | "success" | "warning";
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

export interface RadioGroupOption {
  value: string;
  label: string;
  disabled?: boolean;
}

/** One step in a `Stepper`. */
export interface StepperStep {
  value: string;
  label: string;
  /** Given, never derived from position — see stepper.md §1. */
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

export type TriStateValue = "excluded" | "default" | "included";

export type EditableLabelActivationMode = "doubleClick" | "enterOrSpace" | "programmatic";

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
export type InputValidator = (value: string, context?: unknown) => ValidationResult | Promise<ValidationResult>;

export type PopoverInitialFocus = "first-focusable" | "content" | "none";

export interface PasswordRequirementsPolicy {
  minLength: number;
  requireMixedCase: boolean;
  requireDigit: boolean;
  requireSpecial: boolean;
  minStrengthScore?: number;
  description?: string | null;
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
export type SelectLoadOptions = (context?: SelectLoadContext) => Promise<SelectItems>;

export type SortDirection = "asc" | "desc";
export type SortField = {
  label: string;
  value?: string;
  key?: string;
  disabled?: boolean;
  defaultDirection?: SortDirection;
};
export type OrderByField = {
  key: string;
  direction: SortDirection;
};
export type OrderByValue = OrderByField[];
export type OrderByTriggerVariant = "summary" | "icon";
export type OrderByFieldDefinition = {
  key: string;
  label: string;
  disabled?: boolean;
  defaultDirection?: SortDirection;
};
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

// ── ThemeSelect ──────────────────────────────────────────────────────────
export type ThemeSwatch = {
  canvas: string;
  surface: string;
  accent: string;
  text: string;
  border: string;
};
export type ThemeOption = {
  value: string;
  label: string;
  description?: string;
  swatch: ThemeSwatch;
};

export interface AccordionItem {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

export interface MenuItem {
  value: string;
  label: string;
  disabled?: boolean;
  checked?: boolean;
  shortcutLabel?: string;
  tone?: "default" | "danger";
  kind?: "action" | "checkbox" | "radio" | "separator";
}

export type DrawerEdge = "left" | "right" | "top" | "bottom";

export type AlertDialogTone = "danger" | "warning";

export type FormActionAlign = "start" | "end" | "between";
export type FormActionDangerItem = {
  label: string;
  onSelect: () => void;
  value?: string;
  disabled?: boolean;
};

export type ToastTone = "info" | "success" | "warning" | "danger";
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
/** Svelte-store-shaped contract: framework-neutral subscribe + dismiss. */
export type ToastHostStore = {
  toasts: { subscribe: (run: (items: ToastHostStoreItem[]) => void) => () => void };
  dismiss: (id: string) => void;
};

export type TableFilterType = "text" | "select" | "date";
export type TableCellValue = string | number | null;
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

export type CalendarWeekStart = "sunday" | "monday";
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

export interface TimeZoneOption {
  value: string;
  label: string;
  disabled?: boolean;
}
export interface ZonedDateTimeValue {
  date: string | null;
  time: string | null;
  timeZone: string | null;
}

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
export type LogActor = {
  id: string;
  email?: string;
  name?: string;
};
export type StreamLogEntry = {
  id?: string;
  timestamp: Date | string | number;
  level: LogLevel;
  message: string;
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
export type LogFilter = {
  field: string;
  label: string;
  type: "select" | "date";
  options?: { value: string; label: string }[];
  placeholder?: string;
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
export type TreeDropPosition = "before" | "after" | "inside";

export type TableSortDirection = "asc" | "desc";
export type TableFilters = Record<string, string>;
export type TablePagination = {
  page: number;
  limit: number;
  total: number;
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

export type EmptyStateVariant = "neutral" | "search" | "firstRun";
export type EmptyStateSize = "default" | "compact";

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

export interface MenubarItem {
  value: string;
  label: string;
  disabled?: boolean;
  items: MenuItem[];
}

export interface NavigationMenuItem {
  value: string;
  label: string;
  disabled?: boolean;
  description?: string;
}

export type BreadcrumbItem = {
  value: string;
  label: string;
  href?: string;
  current?: boolean;
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

export type CardVariant = "default" | "outlined" | "elevated";

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

export type SurfaceTone = "canvas" | "panel" | "elevated";
export type SurfaceBorder = "none" | "subtle" | "default";

export type Orientation = "vertical" | "horizontal";
export type CollapseDirection = "left" | "right" | "up" | "down";

export type BulkAction = {
  id: string;
  label: string;
  icon?: IconProp | ReactNode;
  tone?: "default" | "danger" | "warning";
  disabled?: boolean;
};

export type BrowseState = "ready" | "empty" | "loading" | "error" | "no-results";
export type PickerVariant = "inline" | "popover" | "modal";
export type SplitOrientation = "horizontal" | "vertical";
/** When a SplitView shows its collapse-toggle pill: always, or only while the
 * pointer is on the seam / a toggle holds focus. */
export type SplitToggleVisibility = "always" | "hover";
export type CalloutAnnounceMode = AnnouncementMode;

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

export interface FileUploadItem {
  file: File;
  id: string;
  progress: number;
  status: "pending" | "uploading" | "complete" | "error";
  error?: string;
  previewUrl?: string | null;
  originalFile?: File;
}

export type ColorInputMode = "hex" | "rgb" | "hsl";

export type SelectionMode = "single" | "multiple";

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

export type MediaState = "ready" | "loading" | "error" | "empty";
export type MediaKind = "image" | "audio" | "video" | "document" | "embed" | "pdf" | "other";
export type AspectRatio = "auto" | "square" | "landscape" | "portrait" | "video";

export type MediaPickerItem = {
  id: string;
  label: string;
  thumbnailUrl?: string | null;
  mimeType?: string | null;
  kind?: MediaKind;
  meta?: string | null;
};

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

export type ScrollDirection = "vertical" | "horizontal" | "both";

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
export type TabActivationMode = "automatic" | "manual";

export type DockEdge = "left" | "right" | "top" | "bottom";
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

export type AgentChatStatus = "idle" | "busy";

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
} from "@inflatable-cookie/poodle-core";
