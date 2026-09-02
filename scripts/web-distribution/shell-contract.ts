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

export const MARKDOWN_COMPONENT_NAMES = ["AgentMessage", "MarkdownEditor"] as const;
export const INTERNAL_SVELTE_NAMES = ["DragDropProvider", "MenuSurface"] as const;

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
] as const;

export const REACT_EXTERNAL_MODULES = [
  "react",
  "react-dom",
  "@inflatable-cookie/poodle-core",
  "marked",
] as const;

assertSorted([...SHELL_ROSTER_NAMES], "SHELL_ROSTER_NAMES");
assertSorted([...MARKDOWN_COMPONENT_NAMES], "MARKDOWN_COMPONENT_NAMES");
assertSorted([...INTERNAL_SVELTE_NAMES], "INTERNAL_SVELTE_NAMES");

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
    "./types": svelteCondition("./dist/types.d.ts", "./dist/types.js", "./dist/types.js"),
  };
}

export function reactPackageExports() {
  const exports: Record<string, { types: string; default: string }> = {
    ".": reactCondition("./dist/index.d.ts", "./dist/index.js"),
    "./markdown": reactCondition("./dist/markdown.d.ts", "./dist/markdown.js"),
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

export function reactLibraryEntries(): LibraryEntry[] {
  const entries: LibraryEntry[] = [
    { name: "index", source: "src/index.ts", outputExt: ".js" },
    { name: "markdown", source: "src/markdown.ts", outputExt: ".js" },
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
    "dist/markdown.d.ts",
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
  const expected = [...SHELL_ROSTER_NAMES, ...INTERNAL_SVELTE_NAMES].sort();
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
