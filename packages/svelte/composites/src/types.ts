export type TableSortDirection = "asc" | "desc";
export type BrowseState = "ready" | "empty" | "loading" | "error" | "no-results";
export type MinColumnWidth = "sm" | "md" | "lg";
export type { CardVariant } from "@pug/svelte-primitives";
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

export type BlockType =
  | "paragraph"
  | "heading"
  | "code"
  | "quote"
  | "list"
  | "image"
  | "divider";

export type EditorBlock = {
  id: string;
  type: BlockType;
  content: string;
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

export type { BulkAction } from "@pug/svelte-primitives";

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

export type { SortField, ActiveSort } from "@pug/svelte-primitives";

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
