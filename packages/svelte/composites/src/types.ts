export type TableSortDirection = "asc" | "desc";
export type BrowseState = "ready" | "empty" | "loading" | "error" | "no-results";
export type MinColumnWidth = "sm" | "md" | "lg";
export type CardVariant = "default" | "outlined" | "elevated";
export type PickerVariant = "inline" | "popover" | "modal";
export type SelectionMode = "single" | "multiple";
export type MediaState = "ready" | "loading" | "error" | "empty";
export type MediaKind = "image" | "audio" | "video" | "document" | "embed";
export type AspectRatio = "square" | "landscape" | "portrait" | "video";
export type EmptyStateVariant = "neutral" | "search" | "firstRun";
export type ToastTone = "info" | "success" | "warning" | "danger";

export type TableColumn = {
  id: string;
  label: string;
  align?: "start" | "end";
  isSortable?: boolean;
};

export type TableRow = {
  id: string;
  cells: Record<string, string>;
  summary?: string | null;
};

export type BulkAction = {
  id: string;
  label: string;
  tone?: "default" | "danger";
};

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

export type ToastItem = {
  id: string;
  title: string;
  message?: string | null;
  tone?: ToastTone;
  actionLabel?: string | null;
};
