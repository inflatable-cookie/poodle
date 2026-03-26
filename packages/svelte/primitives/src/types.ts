import type { IconNodes } from "./icon-registry";

export type IconProp = IconNodes | string;

export type ValidationState = "none" | "invalid" | "valid" | "pending";
export type InputValidationStatus = "idle" | "validating" | "valid" | "invalid";

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
export type ButtonTone = "default" | "danger";
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
export type ToggleVariant = "primary" | "secondary" | "ghost";
export type PillTone = "neutral" | "success" | "danger";
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
export type CalloutAnnounceMode = "none" | "polite" | "assertive";
export type ColorInputMode = "hex" | "rgb" | "hsl";
export type SkeletonShape = "line" | "block" | "circle";
export type SkeletonPreset =
  | "table-row"
  | "card"
  | "list-item"
  | "detail-section"
  | "avatar-line";
export type SpinnerVariant = "ring" | "grid";
export type SpinnerSize = "xs" | "sm" | "md" | "lg" | "xl";
export type SpinnerTone = "current" | "accent" | "muted";

export interface AccordionItem {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

export interface SegmentedControlOption {
  value: string;
  label: string;
  ariaLabel?: string;
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

export type TabVariant = "underline" | "card" | "pill" | "strip";

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
  isDisabled?: boolean;
  group?: string;
}

export interface SelectOptionGroup {
  label: string;
  options: SelectOption[];
}

export type SelectItems = SelectOption[] | SelectOptionGroup[];

export interface MenuItem {
  value: string;
  label: string;
  disabled?: boolean;
  checked?: boolean;
  shortcutLabel?: string;
  kind?: "action" | "checkbox" | "radio" | "separator";
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

export interface TableColumn {
  id: string;
  label: string;
  align?: "start" | "end";
  isRowHeader?: boolean;
}

export interface TableRow {
  id: string;
  cells: Record<string, string>;
  summary?: string | null;
}

export interface ComboboxOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
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
  icon?: IconProp;
  tone?: "default" | "danger";
};

export type SortField = {
  value: string;
  label: string;
  disabled?: boolean;
};

export type ActiveSort = {
  field: string;
  direction: "asc" | "desc";
};

export type SplitOrientation = "horizontal" | "vertical";

export type BreadcrumbItem = {
  value: string;
  label: string;
  href?: string;
  current?: boolean;
};
