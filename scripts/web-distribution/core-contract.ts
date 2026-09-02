import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { assertSorted } from "./hash";
import type { AssetCopy, JsExportMap, LibraryEntry } from "./types";

function jsExport(jsPath: string, typesPath: string): JsExportMap {
  return {
    types: typesPath,
    import: jsPath,
    default: jsPath,
  };
}

export const CORE_PACKAGE_NAME = "@inflatable-cookie/poodle-core";
export const CORE_PACKAGE_DIR = "packages/core";

export const CORE_THEME_NAMES = [
  "clay",
  "cobalt",
  "eclipse",
  "forest",
  "graphite",
  "hornet",
  "iceberg",
  "meadow",
  "midnight",
  "nord",
  "rose",
  "solarized",
] as const;

export const CORE_DENSITY_NAMES = ["comfortable", "compact", "default"] as const;

export const CORE_CONTROL_SIZE_NAMES = ["lg", "md", "sm", "xl", "xs"] as const;

export const CORE_STYLE_FILES = [
  "accordion.css",
  "action-discovery-panel.css",
  "agent-chat-input.css",
  "agent-message.css",
  "agent-plan-record.css",
  "agent-plan.css",
  "agent-question-record.css",
  "agent-question.css",
  "agent-subagent.css",
  "agent-transcript.css",
  "alert-dialog.css",
  "anchored-surface.css",
  "app-header.css",
  "audio-meter.css",
  "audio-player.css",
  "audio-switch.css",
  "avatar.css",
  "block-editor.css",
  "box.css",
  "breadcrumbs.css",
  "bulk-action-bar.css",
  "button.css",
  "calendar.css",
  "callout.css",
  "card-radio-group.css",
  "card-toggle-group.css",
  "card.css",
  "changed-files.css",
  "checkbox.css",
  "code-input.css",
  "code.css",
  "collapse-toggle.css",
  "collapsible.css",
  "color-picker.css",
  "command-palette.css",
  "data-table.css",
  "date-picker.css",
  "date-range-picker.css",
  "date-time-picker.css",
  "date-time-range-picker.css",
  "date-time-zone-picker.css",
  "detail-item.css",
  "detail-section-group.css",
  "detail-section.css",
  "detail-shell.css",
  "dialog.css",
  "dock-region.css",
  "drag-drop.css",
  "drag-number-field.css",
  "drawer.css",
  "duration-input.css",
  "editable-label.css",
  "editable-list.css",
  "embed-input.css",
  "embed-preview.css",
  "empty-state.css",
  "envelope-editor.css",
  "eyebrow.css",
  "fader.css",
  "field-set.css",
  "field.css",
  "file-upload.css",
  "filter-builder.css",
  "filter-toolbar.css",
  "form-actions.css",
  "form-dialog.css",
  "form-layout.css",
  "gain-reduction-meter.css",
  "grid.css",
  "history-center.css",
  "hover-card.css",
  "icon-button.css",
  "icon.css",
  "inline-list-section.css",
  "keyboard.css",
  "knob.css",
  "licence.css",
  "list-card-counter.css",
  "list-card.css",
  "list-container.css",
  "list-grid.css",
  "log-list.css",
  "markdown-editor.css",
  "media-browse-panel.css",
  "media-picker.css",
  "media-preview.css",
  "media-thumbnail.css",
  "menu-surface.css",
  "menu.css",
  "menubar.css",
  "message-center.css",
  "meta-bar.css",
  "meta-item.css",
  "meter-surface.css",
  "meter.css",
  "metric-tile.css",
  "mod-matrix-grid.css",
  "model-connection.css",
  "model-picker.css",
  "motion-policy-provider.css",
  "nav-card.css",
  "navigation-menu.css",
  "number-input.css",
  "order-by.css",
  "page-header.css",
  "page-loading.css",
  "pagination-summary.css",
  "pagination.css",
  "password-requirements.css",
  "picker-shell.css",
  "pill.css",
  "popover.css",
  "progress.css",
  "radio-group.css",
  "radio.css",
  "range-slider.css",
  "rating.css",
  "ref-select.css",
  "region.css",
  "relation-picker.css",
  "remediation-banner.css",
  "resize-handle.css",
  "scroll-shell.css",
  "segmented-control.css",
  "select.css",
  "selection-summary.css",
  "separator.css",
  "settings-shell.css",
  "sidebar-nav.css",
  "skeleton.css",
  "slider.css",
  "spacer.css",
  "spinner.css",
  "split-button.css",
  "split-view.css",
  "stack.css",
  "state-tile.css",
  "status-bar.css",
  "status-indicator.css",
  "stepper.css",
  "surface.css",
  "switch.css",
  "table.css",
  "tabs.css",
  "text-input.css",
  "text-link.css",
  "text.css",
  "theme-select.css",
  "time-ago.css",
  "time-input.css",
  "toast-host.css",
  "toast-stack.css",
  "toggle-group.css",
  "token-input.css",
  "tool-call-group.css",
  "tool-call.css",
  "toolbar.css",
  "tooltip.css",
  "tree.css",
  "tri-state-switch.css",
  "ui-presentation-provider.css",
  "update-center.css",
  "validation-summary.css",
  "value-readout.css",
  "video-player.css",
  "waveform-display.css",
  "xy-pad.css",
] as const;

export const CORE_ICON_MODULES = [
  "alert-circle",
  "alert-triangle",
  "arrow-down",
  "arrow-left",
  "arrow-right",
  "arrow-up",
  "arrow-up-down",
  "audio-waveform",
  "bell",
  "bold",
  "calendar",
  "check",
  "check-check",
  "check-circle",
  "check-square",
  "chevron-down",
  "chevron-left",
  "chevron-right",
  "chevron-up",
  "circle",
  "circle-alert",
  "circle-check",
  "circle-help",
  "circle-pause",
  "circle-question-mark",
  "circle-x",
  "clock",
  "cloud-off",
  "code",
  "columns-2",
  "columns-3",
  "copy",
  "diff",
  "dot",
  "download",
  "edit",
  "ellipsis",
  "ellipsis-vertical",
  "external-link",
  "eye",
  "file",
  "file-pen",
  "file-question",
  "file-question-mark",
  "file-text",
  "filter",
  "folder",
  "git-branch",
  "git-commit-horizontal",
  "grip-vertical",
  "heading",
  "heart",
  "help-circle",
  "home",
  "house",
  "image",
  "inbox",
  "info",
  "italic",
  "link",
  "list",
  "list-filter",
  "loader",
  "loader-circle",
  "lock",
  "lock-open",
  "mail",
  "maximize-2",
  "menu",
  "minimize-2",
  "minus",
  "monitor",
  "monitor-play",
  "more-horizontal",
  "more-vertical",
  "music",
  "package",
  "pause",
  "pause-circle",
  "pencil",
  "piano",
  "play",
  "plus",
  "quote",
  "redo",
  "refresh-cw",
  "save",
  "search",
  "settings",
  "spinner",
  "square",
  "square-check",
  "star",
  "tag",
  "terminal",
  "trash-2",
  "trending-down",
  "trending-up",
  "triangle-alert",
  "undo",
  "unlock",
  "upload",
  "user",
  "users",
  "volume-2",
  "volume-x",
  "x",
  "x-circle",
] as const;

export const CORE_TOKEN_JS_ENTRIES = [
  "tokens/index",
  "tokens/runtime",
  "tokens/css",
  "tokens/themes",
  "tokens/metadata",
  "tokens/units",
] as const;

export const CORE_FILES = [
  "dist",
  "README.md",
  "LICENSE",
  "THIRD_PARTY_NOTICES.md",
] as const;

export const CORE_SIDE_EFFECTS = ["**/*.css"] as const;

export const CORE_FORBIDDEN_MODULES = ["marked", "svelte", "react", "react-dom"] as const;

assertSorted(CORE_THEME_NAMES, "CORE_THEME_NAMES");
assertSorted(CORE_DENSITY_NAMES, "CORE_DENSITY_NAMES");
assertSorted(CORE_CONTROL_SIZE_NAMES, "CORE_CONTROL_SIZE_NAMES");
assertSorted(CORE_STYLE_FILES, "CORE_STYLE_FILES");
assertSorted(CORE_ICON_MODULES, "CORE_ICON_MODULES");

export function tokenCssBasenames(): string[] {
  return [
    "poodle-tokens.css",
    "poodle-themes.css",
    ...CORE_THEME_NAMES.map((name) => `poodle-theme-${name}.css`),
    ...CORE_DENSITY_NAMES.map((name) => `poodle-density-${name}.css`),
    ...CORE_CONTROL_SIZE_NAMES.map((name) => `poodle-control-size-${name}.css`),
  ];
}

export function coreLibraryEntries(): LibraryEntry[] {
  const entries: LibraryEntry[] = [
    { name: "index", source: "src/index.ts", outputExt: ".js" },
    { name: "icons/index", source: "src/icons/index.ts", outputExt: ".js" },
    ...CORE_ICON_MODULES.map((name) => ({
      name: `icons/icons/${name}`,
      source: `src/icons/icons/${name}.ts`,
      outputExt: ".js" as const,
    })),
    { name: "tokens/css", source: "src/tokens/css.ts", outputExt: ".js" },
    { name: "tokens/index", source: "src/tokens/index.ts", outputExt: ".js" },
    { name: "tokens/metadata", source: "src/tokens/metadata.ts", outputExt: ".js" },
    { name: "tokens/runtime", source: "src/tokens/runtime.ts", outputExt: ".js" },
    { name: "tokens/themes", source: "src/tokens/themes.ts", outputExt: ".js" },
    { name: "tokens/units", source: "src/tokens/units.ts", outputExt: ".js" },
  ];
  entries.sort((left, right) => left.name.localeCompare(right.name));
  assertSorted(
    entries.map((entry) => entry.name),
    "core library entry names",
  );
  return entries;
}

export function coreAssetCopies(): AssetCopy[] {
  const styles = CORE_STYLE_FILES.map((file) => ({
    from: `src/styles/${file}`,
    to: `dist/styles/${file}`,
  }));
  const tokens = tokenCssBasenames().map((file) => ({
    from: `src/tokens/generated/css/${file}`,
    to: `dist/tokens/generated/css/${file}`,
  }));
  const cli = [
    { from: "src/icons/build.mjs", to: "dist/icons/build.mjs" },
    { from: "src/icons/build.d.mts", to: "dist/icons/build.d.mts" },
  ];
  return [...cli, ...styles, ...tokens];
}

export function corePublicJsFiles(): string[] {
  return [
    "dist/index.js",
    "dist/icons/index.js",
    "dist/icons/build.mjs",
    ...CORE_ICON_MODULES.map((name) => `dist/icons/icons/${name}.js`),
    "dist/tokens/index.js",
    "dist/tokens/runtime.js",
    "dist/tokens/css.js",
    "dist/tokens/themes.js",
    "dist/tokens/metadata.js",
    "dist/tokens/units.js",
  ];
}

export function corePublicCssFiles(): string[] {
  return [
    ...CORE_STYLE_FILES.map((file) => `dist/styles/${file}`),
    ...tokenCssBasenames().map((file) => `dist/tokens/generated/css/${file}`),
  ];
}

export function corePublicDeclarationFiles(): string[] {
  return [
    "dist/index.d.ts",
    "dist/icons/index.d.ts",
    "dist/icons/build.d.mts",
    ...CORE_ICON_MODULES.map((name) => `dist/icons/icons/${name}.d.ts`),
    "dist/tokens/index.d.ts",
    "dist/tokens/runtime.d.ts",
    "dist/tokens/css.d.ts",
    "dist/tokens/themes.d.ts",
    "dist/tokens/metadata.d.ts",
    "dist/tokens/units.d.ts",
  ];
}

export function corePackageExports(): Record<string, string | JsExportMap> {
  const tokenCss = (exportKey: string, basename: string): [string, string] => [
    exportKey,
    `./dist/tokens/generated/css/${basename}`,
  ];
  return {
    ".": jsExport("./dist/index.js", "./dist/index.d.ts"),
    "./icons": jsExport("./dist/icons/index.js", "./dist/icons/index.d.ts"),
    "./icons/build": jsExport("./dist/icons/build.mjs", "./dist/icons/build.d.mts"),
    "./icons/*": jsExport("./dist/icons/icons/*.js", "./dist/icons/icons/*.d.ts"),
    "./tokens": jsExport("./dist/tokens/index.js", "./dist/tokens/index.d.ts"),
    "./tokens/runtime": jsExport("./dist/tokens/runtime.js", "./dist/tokens/runtime.d.ts"),
    "./tokens/css": jsExport("./dist/tokens/css.js", "./dist/tokens/css.d.ts"),
    "./tokens/themes": jsExport("./dist/tokens/themes.js", "./dist/tokens/themes.d.ts"),
    "./tokens/metadata": jsExport("./dist/tokens/metadata.js", "./dist/tokens/metadata.d.ts"),
    "./tokens/units": jsExport("./dist/tokens/units.js", "./dist/tokens/units.d.ts"),
    "./styles/*": "./dist/styles/*",
    "./tokens/styles.css": "./dist/tokens/generated/css/poodle-tokens.css",
    "./tokens/css/poodle-tokens.css": "./dist/tokens/generated/css/poodle-tokens.css",
    "./tokens/themes.css": "./dist/tokens/generated/css/poodle-themes.css",
    ...Object.fromEntries(
      CORE_THEME_NAMES.map((name) =>
        tokenCss(`./tokens/theme-${name}.css`, `poodle-theme-${name}.css`),
      ),
    ),
    ...Object.fromEntries(
      CORE_DENSITY_NAMES.map((name) =>
        tokenCss(`./tokens/density-${name}.css`, `poodle-density-${name}.css`),
      ),
    ),
    ...Object.fromEntries(
      CORE_CONTROL_SIZE_NAMES.map((name) =>
        tokenCss(`./tokens/control-size-${name}.css`, `poodle-control-size-${name}.css`),
      ),
    ),
  };
}

function listFilenames(directory: string, suffix: string): string[] {
  if (!existsSync(directory)) {
    throw new Error(`missing inventory directory ${directory}`);
  }
  return readdirSync(directory)
    .filter((name) => name.endsWith(suffix))
    .sort();
}

export function assertCoreInventoriesMatchDisk(repoRoot: string): void {
  const coreRoot = join(repoRoot, CORE_PACKAGE_DIR);
  const styles = listFilenames(join(coreRoot, "src/styles"), ".css");
  if (styles.join("\n") !== CORE_STYLE_FILES.join("\n")) {
    throw new Error("core CSS inventory disagrees with packages/core/src/styles");
  }
  const icons = listFilenames(join(coreRoot, "src/icons/icons"), ".ts")
    .map((name) => name.slice(0, -3))
    .sort();
  if (icons.join("\n") !== CORE_ICON_MODULES.join("\n")) {
    throw new Error("core icon inventory disagrees with packages/core/src/icons/icons");
  }
  const tokenCss = listFilenames(join(coreRoot, "src/tokens/generated/css"), ".css");
  const expectedTokenCss = [...tokenCssBasenames()].sort();
  if (tokenCss.join("\n") !== expectedTokenCss.join("\n")) {
    throw new Error("core token CSS inventory disagrees with generated css");
  }
  for (const entry of coreLibraryEntries()) {
    if (!existsSync(join(coreRoot, entry.source))) {
      throw new Error(`missing core entry ${entry.source}`);
    }
  }
  for (const asset of coreAssetCopies()) {
    if (!existsSync(join(coreRoot, asset.from))) {
      throw new Error(`missing core asset ${asset.from}`);
    }
  }
}

export function readCorePackageVersion(repoRoot: string): string {
  const manifest = JSON.parse(
    readFileSync(join(repoRoot, CORE_PACKAGE_DIR, "package.json"), "utf8"),
  ) as { version?: string };
  if (!manifest.version) throw new Error("core package.json is missing version");
  return manifest.version;
}
