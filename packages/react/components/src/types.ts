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
