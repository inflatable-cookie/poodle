import { getContext, setContext } from "svelte";

import iconNodes from "lucide-static/icon-nodes.json";

export type IconNodeElement = [string, Record<string, string>];
export type IconRegistry = Record<string, IconNodeElement[]>;

const PUG_ICON_CONTEXT = Symbol("pug-icon-registry");

const curatedNames: string[] = [
  // Navigation
  "arrow-down",
  "arrow-left",
  "arrow-right",
  "arrow-up",
  "chevron-down",
  "chevron-left",
  "chevron-right",
  "chevron-up",
  "chevrons-up-down",
  "menu",
  "trending-down",
  "trending-up",
  "x",

  // Actions
  "check",
  "copy",
  "download",
  "edit",
  "external-link",
  "link",
  "log-out",
  "minus",
  "move",
  "plus",
  "redo",
  "refresh-cw",
  "save",
  "send",
  "share",
  "trash-2",
  "undo",
  "upload",

  // Status / feedback
  "alert-circle",
  "triangle-alert",
  "ban",
  "check-circle",
  "circle-help",
  "info",
  "loader",
  "circle-x",

  // Objects / UI chrome
  "bell",
  "bookmark",
  "calendar",
  "clock",
  "cloud",
  "code",
  "columns-3",
  "eye",
  "eye-off",
  "file",
  "file-text",
  "list-filter",
  "folder",
  "globe",
  "grip-vertical",
  "hash",
  "heart",
  "house",
  "image",
  "inbox",
  "key",
  "layers",
  "list",
  "lock",
  "mail",
  "map-pin",
  "maximize",
  "minimize",
  "pin",
  "more-horizontal",
  "more-vertical",
  "paperclip",
  "search",
  "settings",
  "sliders-horizontal",
  "star",
  "tag",
  "terminal",
  "thumbs-up",
  "unlock",
  "user",
  "users",
  "zap",

  // Text formatting
  "bold",
  "heading",
  "italic",
  "quote",
  "text-align-center",
  "text-align-end",
  "text-align-start",
  "underline",

  // Media
  "mic",
  "music",
  "pause",
  "play",
  "skip-back",
  "skip-forward",
  "volume-2",
];

const allNodes = iconNodes as Record<string, IconNodeElement[]>;

/** Aliases for icon names that changed across Lucide versions. */
const iconAliases: Record<string, string> = {
  "filter": "list-filter",
  "edit": "pencil",
  "alert-circle": "circle-alert",
  "check-circle": "circle-check",
  "more-horizontal": "ellipsis",
  "more-vertical": "ellipsis-vertical",
  "unlock": "lock-open",
};

export const defaultIconRegistry: IconRegistry = Object.fromEntries([
  ...curatedNames.filter((name) => name in allNodes).map((name) => [name, allNodes[name]]),
  ...Object.entries(iconAliases)
    .filter(([, actual]) => actual in allNodes)
    .map(([alias, actual]) => [alias, allNodes[actual]]),
]);

export function setIconRegistry(registry: IconRegistry): void {
  setContext(PUG_ICON_CONTEXT, registry);
}

export function getIconRegistry(): IconRegistry {
  return getContext<IconRegistry>(PUG_ICON_CONTEXT) ?? defaultIconRegistry;
}
