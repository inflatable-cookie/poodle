export type ValidationState = "none" | "invalid" | "valid" | "pending";

export type CalendarWeekStart = "sunday" | "monday";
export type SpaceScale = "none" | "sm" | "md" | "lg";
export type ControlSize = "sm" | "md" | "lg";
export type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";
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
export type PillAppearance = "solid" | "subtle";
export type PillSize = "xxs" | "xs" | "sm";
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
export type DrawerEdge = "left" | "right" | "top" | "bottom";
export type TriStateValue = "excluded" | "default" | "included";
export type EditableLabelActivationMode =
  | "doubleClick"
  | "enterOrSpace"
  | "programmatic";
export type FormActionAlign = "start" | "end" | "between";
export type BannerTone = "info" | "success" | "warning" | "danger";
export type BannerAnnounceMode = "none" | "polite" | "assertive";
export type SkeletonShape = "line" | "block" | "circle";

export interface AccordionItem {
  value: string;
  label: string;
  description?: string;
  isDisabled?: boolean;
}

export interface SegmentedControlOption {
  value: string;
  label: string;
  ariaLabel?: string;
  isDisabled?: boolean;
}

export interface ToggleGroupOption {
  value: string;
  label: string;
  ariaLabel?: string;
  isDisabled?: boolean;
}

export interface TabDefinition {
  value: string;
  label: string;
  isDisabled?: boolean;
}

export interface TabStripItem {
  value: string;
  label: string;
  isDisabled?: boolean;
  isClosable?: boolean;
}

export interface RadioGroupOption {
  value: string;
  label: string;
  isDisabled?: boolean;
}

export interface SelectOption {
  value: string;
  label: string;
  isDisabled?: boolean;
}

export interface MenuItem {
  value: string;
  label: string;
  isDisabled?: boolean;
  isChecked?: boolean;
  shortcutLabel?: string;
  kind?: "action" | "checkbox" | "radio" | "separator";
}

export interface NavigationMenuItem {
  value: string;
  label: string;
  isDisabled?: boolean;
  description?: string;
}

export interface MenubarItem {
  value: string;
  label: string;
  isDisabled?: boolean;
  items: MenuItem[];
}

export interface TimeZoneOption {
  value: string;
  label: string;
  isDisabled?: boolean;
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
  isDisabled?: boolean;
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

export interface ZonedDateTimeValue {
  date: string | null;
  time: string | null;
  timeZone: string | null;
}
