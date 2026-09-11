import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { assertSorted } from "./hash";
import type { LibraryEntry } from "./types";

export const SVELTE_PACKAGE_NAME = "@inflatable-cookie/poodle-svelte";
export const SVELTE_PACKAGE_DIR = "packages/svelte/components";
export const REACT_PACKAGE_NAME = "@inflatable-cookie/poodle-react";
export const REACT_PACKAGE_DIR = "packages/react/components";

export const SHELL_ROSTER_NAMES = [
  "Accordion",
  "ActionDiscoveryPanel",
  "AgentChatInput",
  "AgentMessage",
  "AgentPlan",
  "AgentPlanRecord",
  "AgentQuestion",
  "AgentQuestionRecord",
  "AgentSubagent",
  "AgentTranscript",
  "AlertDialog",
  "AppHeader",
  "AudioMeter",
  "AudioPlayer",
  "AudioSwitch",
  "Avatar",
  "BlockEditor",
  "Box",
  "Breadcrumbs",
  "BulkActionBar",
  "Button",
  "Calendar",
  "Callout",
  "Card",
  "CardRadioGroup",
  "CardToggleGroup",
  "ChangedFiles",
  "Checkbox",
  "Code",
  "CodeInput",
  "CollapseToggle",
  "Collapsible",
  "ColorPicker",
  "CommandPalette",
  "ConfirmAction",
  "ContextMenu",
  "DataTable",
  "DatePicker",
  "DateRangePicker",
  "DateTimePicker",
  "DateTimeRangePicker",
  "DateTimeZonePicker",
  "DebugDialog",
  "DetailItem",
  "DetailSection",
  "DetailSectionGroup",
  "DetailShell",
  "Dialog",
  "DockRegion",
  "DragNumberField",
  "Drawer",
  "DurationInput",
  "EditableLabel",
  "EditableList",
  "EmbedInput",
  "EmbedPreview",
  "EmptyState",
  "EnvelopeEditor",
  "ErrorBoundary",
  "Eyebrow",
  "Fader",
  "Field",
  "FieldSet",
  "FileUpload",
  "FilterBuilder",
  "FilterToolbar",
  "FormActions",
  "FormDialog",
  "FormLayout",
  "GainReductionMeter",
  "Grid",
  "HistoryCenter",
  "HoverCard",
  "Icon",
  "IconButton",
  "IconProvider",
  "InlineListSection",
  "Keyboard",
  "Knob",
  "LicenceActivation",
  "LicenceSeats",
  "LicenceStatus",
  "ListCard",
  "ListCardCounter",
  "ListContainer",
  "ListGrid",
  "LogList",
  "MarkdownEditor",
  "MediaBrowsePanel",
  "MediaPicker",
  "MediaPreview",
  "MediaThumbnail",
  "Menu",
  "Menubar",
  "MessageCenter",
  "MetaBar",
  "MetaItem",
  "Meter",
  "MeterSurface",
  "MetricTile",
  "ModMatrixGrid",
  "ModelCatalogueEditor",
  "ModelConnectionCard",
  "ModelConnectionPicker",
  "ModelConnectionSetup",
  "ModelPicker",
  "MotionPolicyProvider",
  "NavCard",
  "NavigationMenu",
  "NumberInput",
  "OrderBy",
  "PageHeader",
  "PageLoading",
  "Pagination",
  "PaginationSummary",
  "PasswordRequirements",
  "PickerShell",
  "Pill",
  "Popover",
  "Progress",
  "Radio",
  "RadioGroup",
  "RangeSlider",
  "Rating",
  "RefSelect",
  "Region",
  "RelationPicker",
  "RemediationBanner",
  "ResizeHandle",
  "ScrollShell",
  "SegmentedControl",
  "Select",
  "SelectionSummary",
  "Separator",
  "SettingsShell",
  "SidebarNav",
  "Skeleton",
  "Slider",
  "Spacer",
  "Spinner",
  "SplitButton",
  "SplitView",
  "Stack",
  "StateTile",
  "StatusBar",
  "StatusIndicator",
  "Stepper",
  "Surface",
  "Switch",
  "Table",
  "Tabs",
  "Text",
  "TextInput",
  "TextLink",
  "ThemeSelect",
  "TimeAgo",
  "TimeInput",
  "TimeZoneSelect",
  "ToastHost",
  "ToastStack",
  "ToggleGroup",
  "TokenInput",
  "ToolCall",
  "ToolCallGroup",
  "Toolbar",
  "Tooltip",
  "Tree",
  "TriStateSwitch",
  "UiPresentationProvider",
  "UpdateCenter",
  "UpdateStatus",
  "ValidationSummary",
  "ValueReadout",
  "VideoPlayer",
  "WaveformDisplay",
  "XYPad",
] as const;

export const MARKDOWN_COMPONENT_NAMES = [
  "AgentMessage",
  "AgentPlan",
  "AgentPlanRecord",
  "AgentTranscript",
  "MarkdownEditor",
] as const;

/**
 * `MarkdownRenderer` is a web-only companion to `MarkdownEditor`: it shares the
 * `./markdown` entry and the editor's private rendering path but has no native
 * counterpart, no direct `./MarkdownRenderer.svelte` entry, and no membership
 * in the 176-name native-boundary roster. It is inventoried here so the Svelte
 * source/roster audit can still see it, and it is certified through the
 * web-only catalogue supplement and the installed `./markdown` smoke.
 */
export const MARKDOWN_RENDERER_SVELTE_NAMES = ["MarkdownRenderer"] as const;
export const INTERNAL_SVELTE_NAMES = ["DragDropProvider", "MenuSurface"] as const;
/**
 * The CodeEditor engine lives behind dedicated `./editor` entries so root
 * consumers never load CodeMirror or its language chunks. `CodeEditor` is
 * deliberately absent from `SHELL_ROSTER_NAMES`: it must not join the root
 * barrel, the 176-name roster, or any successor denominator.
 */
export const EDITOR_ENTRY_NAME = "editor";
export const EDITOR_SVELTE_NAMES = ["CodeEditor"] as const;

/**
 * The rich-text engine lives behind dedicated `./rich-text` entries so root,
 * `./markdown`, and `./editor` consumers never load TipTap or ProseMirror.
 * `RichTextEditor` and `RichTextRenderer` are deliberately absent from
 * `SHELL_ROSTER_NAMES`: they must not join the root barrel, the 176-name
 * roster, or any successor denominator.
 */
export const RICH_TEXT_ENTRY_NAME = "rich-text";
export const RICH_TEXT_SVELTE_NAMES = ["RichTextEditor", "RichTextRenderer"] as const;

export const TIPTAP_EXTERNAL_MODULES = [
  "@tiptap/core",
  "@tiptap/extension-blockquote",
  "@tiptap/extension-bold",
  "@tiptap/extension-code",
  "@tiptap/extension-code-block",
  "@tiptap/extension-document",
  "@tiptap/extension-hard-break",
  "@tiptap/extension-heading",
  "@tiptap/extension-horizontal-rule",
  "@tiptap/extension-image",
  "@tiptap/extension-italic",
  "@tiptap/extension-link",
  "@tiptap/extension-list",
  "@tiptap/extension-paragraph",
  "@tiptap/extension-strike",
  "@tiptap/extension-table",
  "@tiptap/extension-text",
  "@tiptap/extensions",
  "@tiptap/pm",
] as const;

export const CODEMIRROR_EXTERNAL_MODULES = [
  "@codemirror/commands",
  "@codemirror/language",
  "@codemirror/search",
  "@codemirror/state",
  "@codemirror/view",
  // g18.021: base editor presentation machinery. The internal token-bound
  // highlight style imports Lezer tags; this stays external like the pinned
  // base set and is not a grammar package.
  "@lezer/highlight",
] as const;

/**
 * Grammar packages Poodle must never depend on or import (g18.012): language
 * support is consumer-owned. Consumers install exactly the `@codemirror/lang-*`
 * packages their registry loads; these names must stay out of the shell
 * manifests and out of every emitted graph.
 */
export const FORBIDDEN_GRAMMAR_MODULES = [
  "@codemirror/lang-css",
  "@codemirror/lang-html",
  "@codemirror/lang-javascript",
  "@codemirror/lang-json",
  "@codemirror/lang-markdown",
  "@codemirror/lang-rust",
  "@codemirror/lang-yaml",
  "@codemirror/legacy-modes",
] as const;

export const SHELL_EXTERNAL_MODULES = [
  "svelte",
  "react",
  "react-dom",
  "@inflatable-cookie/poodle-core",
  "marked",
] as const;

export const SVELTE_EXTERNAL_MODULES = [
  "svelte",
  "@inflatable-cookie/poodle-core",
  "marked",
  ...CODEMIRROR_EXTERNAL_MODULES,
  ...TIPTAP_EXTERNAL_MODULES,
] as const;

export const REACT_EXTERNAL_MODULES = [
  "react",
  "react-dom",
  "@inflatable-cookie/poodle-core",
  "marked",
  ...CODEMIRROR_EXTERNAL_MODULES,
  ...TIPTAP_EXTERNAL_MODULES,
] as const;

assertSorted([...SHELL_ROSTER_NAMES], "SHELL_ROSTER_NAMES");
assertSorted([...MARKDOWN_COMPONENT_NAMES], "MARKDOWN_COMPONENT_NAMES");
assertSorted([...MARKDOWN_RENDERER_SVELTE_NAMES], "MARKDOWN_RENDERER_SVELTE_NAMES");
assertSorted([...INTERNAL_SVELTE_NAMES], "INTERNAL_SVELTE_NAMES");
assertSorted([...FORBIDDEN_GRAMMAR_MODULES], "FORBIDDEN_GRAMMAR_MODULES");

const markdownSet = new Set<string>(MARKDOWN_COMPONENT_NAMES);

export function rootRosterNames(): string[] {
  return SHELL_ROSTER_NAMES.filter((name) => !markdownSet.has(name));
}

function svelteCondition(types: string, browser: string, fallback: string) {
  return {
    types,
    browser,
    default: fallback,
  };
}

function reactCondition(types: string, js: string) {
  return {
    types,
    default: js,
  };
}

export function sveltePackageExports() {
  return {
    ".": svelteCondition("./dist/index.d.ts", "./dist/index.client.js", "./dist/index.server.js"),
    "./*.svelte": svelteCondition(
      "./dist/*.svelte.d.ts",
      "./dist/*.client.js",
      "./dist/*.server.js",
    ),
    "./markdown": svelteCondition(
      "./dist/markdown.d.ts",
      "./dist/markdown.client.js",
      "./dist/markdown.server.js",
    ),
    "./editor": svelteCondition(
      "./dist/editor.d.ts",
      "./dist/editor.client.js",
      "./dist/editor.server.js",
    ),
    "./editor/codemirror": svelteCondition(
      "./dist/editor-codemirror.d.ts",
      "./dist/editor-codemirror.js",
      "./dist/editor-codemirror.js",
    ),
    "./rich-text": svelteCondition(
      "./dist/rich-text.d.ts",
      "./dist/rich-text.client.js",
      "./dist/rich-text.server.js",
    ),
    "./types": svelteCondition("./dist/types.d.ts", "./dist/types.js", "./dist/types.js"),
  };
}

export function reactPackageExports() {
  const exports: Record<string, { types: string; default: string }> = {
    ".": reactCondition("./dist/index.d.ts", "./dist/index.js"),
    "./markdown": reactCondition("./dist/markdown.d.ts", "./dist/markdown.js"),
    "./editor": reactCondition("./dist/editor.d.ts", "./dist/editor.js"),
    "./editor/codemirror": reactCondition(
      "./dist/editor-codemirror.d.ts",
      "./dist/editor-codemirror.js",
    ),
    "./rich-text": reactCondition("./dist/rich-text.d.ts", "./dist/rich-text.js"),
    "./types": reactCondition("./dist/types.d.ts", "./dist/types.js"),
  };
  for (const name of SHELL_ROSTER_NAMES) {
    exports[`./${name}`] = reactCondition(`./dist/${name}.d.ts`, `./dist/${name}.js`);
  }
  return exports;
}

export function svelteDualEntries(): LibraryEntry[] {
  const entries: LibraryEntry[] = [
    { name: "index", source: "src/index.ts", outputExt: ".js" },
    { name: "markdown", source: "src/markdown.ts", outputExt: ".js" },
    { name: EDITOR_ENTRY_NAME, source: "src/editor.ts", outputExt: ".js" },
    { name: RICH_TEXT_ENTRY_NAME, source: "src/rich-text.ts", outputExt: ".js" },
    ...SHELL_ROSTER_NAMES.map((name) => ({
      name,
      source: `src/${name}.svelte`,
      outputExt: ".js" as const,
    })),
  ];
  entries.sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0));
  return entries;
}

export function svelteTypesEntry(): LibraryEntry {
  return { name: "types", source: "src/types.ts", outputExt: ".js" };
}

/**
 * The `./editor/codemirror` adapter is isomorphic TypeScript, so like `types`
 * it is a single-lane entry: one compiled file serves browser and default
 * conditions. It is not a dual (client/server) Svelte component entry.
 */
export function svelteEditorAdapterEntry(): LibraryEntry {
  return { name: "editor-codemirror", source: "src/editor-codemirror.ts", outputExt: ".js" };
}

export function reactLibraryEntries(): LibraryEntry[] {
  const entries: LibraryEntry[] = [
    { name: "index", source: "src/index.ts", outputExt: ".js" },
    { name: "markdown", source: "src/markdown.ts", outputExt: ".js" },
    { name: EDITOR_ENTRY_NAME, source: "src/editor.ts", outputExt: ".js" },
    { name: "editor-codemirror", source: "src/editor-codemirror.ts", outputExt: ".js" },
    { name: RICH_TEXT_ENTRY_NAME, source: "src/rich-text.ts", outputExt: ".js" },
    { name: "types", source: "src/types.ts", outputExt: ".js" },
    ...SHELL_ROSTER_NAMES.map((name) => ({
      name,
      source: `src/${name}.tsx`,
      outputExt: ".js" as const,
    })),
  ];
  entries.sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0));
  return entries;
}

export function sveltePublicFiles(): string[] {
  const files = [
    "dist/index.client.js",
    "dist/index.server.js",
    "dist/index.d.ts",
    "dist/markdown.client.js",
    "dist/markdown.server.js",
    "dist/markdown.d.ts",
    "dist/editor.client.js",
    "dist/editor.server.js",
    "dist/editor.d.ts",
    "dist/editor-codemirror.js",
    "dist/editor-codemirror.d.ts",
    "dist/rich-text.client.js",
    "dist/rich-text.server.js",
    "dist/rich-text.d.ts",
    "dist/types.js",
    "dist/types.d.ts",
  ];
  for (const name of SHELL_ROSTER_NAMES) {
    files.push(`dist/${name}.client.js`, `dist/${name}.server.js`, `dist/${name}.svelte.d.ts`);
  }
  return files.sort();
}

export function reactPublicFiles(): string[] {
  const files = [
    "dist/index.js",
    "dist/index.d.ts",
    "dist/markdown.js",
    "dist/editor.js",
    "dist/editor.d.ts",
    "dist/editor-codemirror.js",
    "dist/editor-codemirror.d.ts",
    "dist/rich-text.js",
    "dist/rich-text.d.ts",
    "dist/types.js",
    "dist/types.d.ts",
  ];
  for (const name of SHELL_ROSTER_NAMES) {
    files.push(`dist/${name}.js`, `dist/${name}.d.ts`);
  }
  return files.sort();
}

function listBasenames(directory: string, suffix: string): string[] {
  if (!existsSync(directory)) {
    throw new Error(`missing inventory directory ${directory}`);
  }
  return readdirSync(directory)
    .filter((name) => name.endsWith(suffix))
    .map((name) => name.slice(0, -suffix.length))
    .sort();
}

export function assertSvelteInventoriesMatchDisk(repoRoot: string): void {
  const packageRoot = join(repoRoot, SVELTE_PACKAGE_DIR);
  const svelteFiles = listBasenames(join(packageRoot, "src"), ".svelte");
  const expected = [
    ...new Set<string>([
      ...SHELL_ROSTER_NAMES,
      ...MARKDOWN_RENDERER_SVELTE_NAMES,
      ...INTERNAL_SVELTE_NAMES,
      ...EDITOR_SVELTE_NAMES,
      ...RICH_TEXT_SVELTE_NAMES,
    ]),
  ].sort();
  if (svelteFiles.join("\n") !== expected.join("\n")) {
    throw new Error("Svelte *.svelte inventory disagrees with spec 070 roster plus internals");
  }
  for (const entry of [...svelteDualEntries(), svelteTypesEntry()]) {
    if (!existsSync(join(packageRoot, entry.source))) {
      throw new Error(`missing Svelte entry ${entry.source}`);
    }
  }
}

export function assertReactInventoriesMatchDisk(repoRoot: string): void {
  const packageRoot = join(repoRoot, REACT_PACKAGE_DIR);
  for (const entry of reactLibraryEntries()) {
    if (!existsSync(join(packageRoot, entry.source))) {
      throw new Error(`missing React entry ${entry.source}`);
    }
  }
}

export function readPackageVersion(repoRoot: string, packageDir: string): string {
  const manifest = JSON.parse(readFileSync(join(repoRoot, packageDir, "package.json"), "utf8")) as {
    version?: string;
  };
  if (!manifest.version) throw new Error(`${packageDir} package.json is missing version`);
  return manifest.version;
}

export function shellFiles(): string[] {
  return ["dist", "README.md", "LICENSE"];
}

export function shellSideEffects(): string[] {
  return ["**/*.css"];
}
