/**
 * Shared prop types for `@poodle/react`. Shapes mirror the Svelte package's
 * `types.ts` (interface-invariance rule, g11.001) with React idioms:
 * `Snippet` becomes `ReactNode`. Grows per conversion batch.
 */

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

export type IconNodeElement = [string, Record<string, string>];
export type IconNodes = IconNodeElement[];
export type IconSet = Record<string, IconNodes>;
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
export type SpinnerVariant = "ring" | "grid";
export type SpinnerSize = "xs" | "sm" | "md" | "lg" | "xl";
export type SpinnerTone = "current" | "accent" | "muted";

export type SpaceScale = "none" | "sm" | "md" | "lg";
export type LayoutAlign = "start" | "center" | "end" | "stretch";
export type LayoutJustify = "start" | "center" | "end" | "between";
export type OverflowMode = "visible" | "hidden" | "clip";

export type ButtonVariant = "primary" | "secondary" | "ghost";
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

export interface SegmentedControlOption {
  value: string;
  label: string;
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
  icon?: string | null;
  children?: TreeNode[];
  /** Force branch posture even when `children` is empty (empty / lazy folder). */
  isBranch?: boolean;
  isDisabled?: boolean;
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
