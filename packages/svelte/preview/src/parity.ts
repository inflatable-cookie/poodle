import { controlSizes, densityModes, themes } from "@poodle/svelte-tokens";
import { docsSections } from "./catalog";

export type ThemeName = keyof typeof themes;
export type DensityName = keyof typeof densityModes;
export type ControlSizeName = keyof typeof controlSizes;
export type DocsSectionId = "catalog-hub" | (typeof docsSections)[number]["id"];
export type HarnessCoverage = "automated" | "hybrid" | "manual" | "not-applicable";

export type PreviewHarnessState = {
  sectionId: DocsSectionId;
  theme: ThemeName;
  density: DensityName;
  controlSize: ControlSizeName;
};

export type ReviewRoutePreset = {
  id: string;
  label: string;
  note: string;
  state: PreviewHarnessState;
};

export type SectionParityTarget = {
  sectionId: DocsSectionId;
  title: string;
  layer: string;
  packageName: string;
  contractRoot: string;
  summary: string;
  harnessCoverage: {
    contract: HarnessCoverage;
    visual: HarnessCoverage;
    interaction: HarnessCoverage;
  };
  automatedChecks: string[];
  manualChecks: string[];
  reviewRoutes: ReviewRoutePreset[];
};

export type PublicSveltePackageName = "@poodle/svelte";

export type PackageSurfaceCoverageStatus = "previewed" | "contract-only";

export type PackageSurfaceCoverageEntry = {
  packageName: PublicSveltePackageName;
  exportName: string;
  kind: "component" | "helper";
  status: PackageSurfaceCoverageStatus;
  sectionIds: DocsSectionId[];
  note: string;
};

type DocsNavigationEntry = {
  id: DocsSectionId;
  title: string;
  eyebrow: string;
  layer: "components" | "workstation" | "tokens";
  packageName: string;
  contractRoot: string;
  summary: string;
  exampleTypes: string[];
};

type SectionParityConfig = Omit<
  SectionParityTarget,
  "sectionId" | "title" | "layer" | "packageName" | "contractRoot" | "summary"
>;

type PackageSurfaceCoverageGroup = Omit<PackageSurfaceCoverageEntry, "exportName"> & {
  exportNames: string[];
};

export const previewStateDefaults: PreviewHarnessState = {
  sectionId: "catalog-hub",
  theme: "loophole-studio",
  density: "compact",
  controlSize: "md",
};

export const catalogHubEntry: DocsNavigationEntry = {
  id: "catalog-hub",
  title: "Catalog hub",
  eyebrow: "Docs baseline",
  layer: "tokens",
  packageName: "docs + packages/svelte/*",
  contractRoot: "packages/svelte/preview/",
  summary:
    "Entry point for package ownership, family coverage, example scope, and adoption expectations.",
  exampleTypes: ["directory", "adoption bar", "section map"],
};

export const docsNavigationSections: DocsNavigationEntry[] = [catalogHubEntry, ...docsSections];

const themeNames = Object.keys(themes) as ThemeName[];
const densityNames = Object.keys(densityModes) as DensityName[];
const controlSizeNames = Object.keys(controlSizes) as ControlSizeName[];

function route(
  sectionId: DocsSectionId,
  label: string,
  note: string,
  overrides: Partial<Omit<PreviewHarnessState, "sectionId">> = {},
): ReviewRoutePreset {
  return {
    id: `${sectionId}-${label.toLowerCase().replace(/[^a-z0-9]+/g, "-")}`,
    label,
    note,
    state: normalizePreviewState({ ...previewStateDefaults, sectionId, ...overrides }),
  };
}

const sectionParityConfig: Record<DocsSectionId, SectionParityConfig> = {
  "catalog-hub": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "manual",
    },
    automatedChecks: [
      "catalog registration is complete",
      "family-to-section coverage is resolvable",
      "review routes are generated from stable preview state",
    ],
    manualChecks: [
      "landing-page hierarchy and adoption messaging still read clearly",
      "section browser remains usable across themes and densities",
    ],
    reviewRoutes: [
      route("catalog-hub", "baseline", "Primary docs landing review."),
      route(
        "catalog-hub",
        "light",
        "Light-theme docs shell review.",
        { theme: "light", density: "comfortable" },
      ),
    ],
  },
  "form-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "hybrid",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "stateful route matrix exists for invalid, pending, and sizing review",
      "token overlay switching remains deep-linkable",
    ],
    manualChecks: [
      "caret, selection, and IME behavior",
      "screen-reader announcement quality for validation and help text",
    ],
    reviewRoutes: [
      route("form-suite", "baseline", "Default workstation-oriented form review."),
      route("form-suite", "light", "Light-theme form contrast review.", {
        theme: "light",
        density: "comfortable",
      }),
      route("form-suite", "small", "Minimum control sizing review.", {
        controlSize: "sm",
      }),
      route("form-suite", "large", "Large control sizing review.", {
        controlSize: "lg",
      }),
    ],
  },
  "table-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "hybrid",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "sorting and selection review routes are generated",
      "preview URL is stable for repeat review",
    ],
    manualChecks: [
      "header affordance clarity and density fit",
      "row action naming and keyboard flow",
    ],
    reviewRoutes: [
      route("table-suite", "baseline", "Default table review."),
      route("table-suite", "light", "Light-theme table review.", {
        theme: "light",
        density: "comfortable",
      }),
    ],
  },
  "browse-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "manual",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "browse-state review routes are generated",
      "contract and package ownership remain linked",
    ],
    manualChecks: [
      "empty versus no-results posture clarity",
      "progressive-loading rhythm and content hierarchy",
    ],
    reviewRoutes: [
      route("browse-suite", "baseline", "Default browse-shell review."),
      route("browse-suite", "light", "Light-theme browse review.", {
        theme: "light",
        density: "comfortable",
      }),
    ],
  },
  "detail-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "manual",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "detail review routes are generated",
      "package and contract provenance remain exposed",
    ],
    manualChecks: [
      "visual hierarchy, spacing, and summary-card contrast",
      "header action emphasis and reading order",
    ],
    reviewRoutes: [
      route("detail-suite", "baseline", "Primary detail-surface review."),
      route("detail-suite", "light", "Light-theme detail review.", {
        theme: "light",
        density: "comfortable",
      }),
    ],
  },
  "picker-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "hybrid",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "picker workflow routes are generated",
      "stateful review surface stays deep-linkable",
    ],
    manualChecks: [
      "multi-select traversal and confirmation rhythm",
      "announcement quality for search and selection summary updates",
    ],
    reviewRoutes: [
      route("picker-suite", "baseline", "Default picker workflow review."),
      route("picker-suite", "light", "Light-theme picker review.", {
        theme: "light",
        density: "comfortable",
      }),
    ],
  },
  "media-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "manual",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "media preview routes are generated",
      "theme/density review surface stays stable",
    ],
    manualChecks: [
      "thumbnail framing and metadata hierarchy",
      "renderer-failure and fallback posture clarity",
    ],
    reviewRoutes: [
      route("media-suite", "baseline", "Default media review."),
      route("media-suite", "light", "Light-theme media review.", {
        theme: "light",
        density: "comfortable",
      }),
    ],
  },
  "notification-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "hybrid",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "notification routes are generated",
      "stateful remediation surface remains deep-linkable",
    ],
    manualChecks: [
      "toast timing appropriateness and message hierarchy",
      "banner tone fit and remediation action emphasis",
    ],
    reviewRoutes: [
      route("notification-suite", "baseline", "Default state-surface review."),
      route("notification-suite", "light", "Light-theme state-surface review.", {
        theme: "light",
        density: "comfortable",
      }),
    ],
  },
  "command-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "hybrid",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "command discovery routes are generated",
      "modal and inline discovery surface stay addressable",
    ],
    manualChecks: [
      "focus containment and restoration quality",
      "result ranking readability and command grouping clarity",
    ],
    reviewRoutes: [
      route("command-suite", "baseline", "Default command-discovery review."),
      route("command-suite", "dark", "Dark neutral command review.", {
        theme: "dark",
        density: "comfortable",
      }),
    ],
  },
  "workspace-suite": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "hybrid",
    },
    automatedChecks: [
      "section remains catalog-registered",
      "workspace shell routes are generated",
      "layout review surface stays deep-linkable",
    ],
    manualChecks: [
      "dock, split, and shell hierarchy legibility",
      "resize affordance feel and focus order through shell regions",
    ],
    reviewRoutes: [
      route("workspace-suite", "baseline", "Primary workstation-shell review."),
      route("workspace-suite", "dark", "Dark neutral workstation-shell review.", {
        theme: "dark",
        density: "comfortable",
      }),
    ],
  },
  "token-summary-section": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "not-applicable",
    },
    automatedChecks: [
      "theme, density, and control-size routes are generated",
      "runtime token readout remains addressable",
      "artifact provenance remains linked",
    ],
    manualChecks: [
      "live readout remains legible in light and dark themes",
    ],
    reviewRoutes: [
      route("token-summary-section", "loophole", "Workstation token overlay review."),
      route("token-summary-section", "dark", "Dark token overlay review.", {
        theme: "dark",
        density: "comfortable",
      }),
      route("token-summary-section", "light", "Light token overlay review.", {
        theme: "light",
        density: "comfortable",
      }),
    ],
  },
  "token-inspector": {
    harnessCoverage: {
      contract: "automated",
      visual: "hybrid",
      interaction: "manual",
    },
    automatedChecks: [
      "token inspector routes are generated",
      "artifact provenance remains addressable",
      "section remains catalog-registered",
    ],
    manualChecks: [
      "search usability and token-path readability",
    ],
    reviewRoutes: [
      route("token-inspector", "loophole", "Workstation token-inspector review."),
      route("token-inspector", "dark", "Dark token-inspector review.", {
        theme: "dark",
        density: "comfortable",
      }),
      route("token-inspector", "light", "Light token-inspector review.", {
        theme: "light",
        density: "comfortable",
      }),
    ],
  },
};

export const previewHarnessBoundary = {
  automated: [
    "catalog registration and section-to-family coverage",
    "stable preview route generation for repeat review",
    "package and contract provenance attachment",
    "token overlay mode coverage at the route-manifest level",
  ],
  manual: [
    "final visual acceptance and hierarchy judgment",
    "assistive technology narration quality",
    "mounted GPUI runtime parity review until the native preview and acceptance harness move beyond artifact-backed evidence",
    "interaction nuance such as drag feel, resize feel, and motion quality",
  ],
};

const packageSurfaceCoverageGroups: PackageSurfaceCoverageGroup[] = [
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["form-suite"],
    note: "Form and validation screen now exercises the broader field, text-entry, selection, and remediation primitive set directly.",
    exportNames: [
      "Callout",
      "DatePicker",
      "Dialog",
      "Field",
      "FormActions",
      "RadioGroup",
      "Switch",
      "TextInput",
    ],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["notification-suite"],
    note: "Feedback, menu, and toolbar primitives exercised through notification-facing specimens.",
    exportNames: ["Menu", "Skeleton", "StatusIndicator", "Tabs", "Toolbar"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "Foundational disclosure, control, and chip primitives exercised directly in catalogue specimens.",
    exportNames: [
      "Accordion",
      "Button",
      "Checkbox",
      "Collapsible",
      "MetaBar",
      "MetaItem",
      "Pill",
      "Select",
      "ToggleGroup",
    ],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "Additional workflow and overlay primitives previewed in catalogue specimens.",
    exportNames: ["Drawer", "Popover", "SegmentedControl", "Table"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "Every public primitive export is directly reviewable from the catalogue.",
    exportNames: [
      "AlertDialog",
      "AudioPlayer",
      "Box",
      "Breadcrumbs",
      "BulkActionBar",
      "Calendar",
      "Card",
      "ColorPicker",
      "Code",
      "CollapseToggle",
      "ContextMenu",
      "DateRangePicker",
      "DateTimePicker",
      "DateTimeRangePicker",
      "DetailItem",
      "DurationInput",
      "EditableLabel",
      "Eyebrow",
      "FieldSet",
      "FileUpload",
      "Grid",
      "HoverCard",
      "Icon",
      "IconButton",
      "IconProvider",
      "ListCard",
      "ListCardCounter",
      "ListGrid",
      "Meter",
      "Menubar",
      "NavCard",
      "NavigationMenu",
      "NumberInput",
      "OrderBy",
      "Pagination",
      "PaginationSummary",
      "PasswordRequirements",
      "CodeInput",
      "Progress",
      "RangeSlider",
      "Region",
      "ResizeHandle",
      "Rating",
      "ScrollShell",
      "Separator",
      "SplitButton",
      "Slider",
      "Spinner",
      "Spacer",
      "Stack",
      "StatusBar",
      "Surface",
      "TimeAgo",
      "TimeInput",
      "TimeZoneSelect",
      "Tooltip",
      "TriStateSwitch",
      "UiPresentationProvider",
      "VideoPlayer",
      "DateTimeZonePicker",
    ],
  },
  {
    packageName: "@poodle/svelte",
    kind: "helper",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "Primitive helper exports remain reviewable through catalogue specimens and shared presentation-driven examples.",
    exportNames: [
      "DEFAULT_COMPRESSION",
      "compressImage",
      "controlHeightRem",
      "formatDisplayDate",
      "formatDisplayDateTime",
      "controlSpaceXRem",
      "formatFileSize",
      "getUiPresentation",
      "panelSpaceXRem",
      "panelSpaceYRem",
      "resolveSemanticControlSize",
      "resolveSupportingVisualSize",
      "type FileUploadValidationError",
      "type ImageCompressionOptions",
    ],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["table-suite"],
    note: "Table suite exercises structured data, selection, and pagination posture.",
    exportNames: ["DataTable", "FilterToolbar"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["browse-suite"],
    note: "Browse suite exercises list, grid, and empty-state surface posture.",
    exportNames: ["EmptyState", "ListContainer"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["detail-suite"],
    note: "Detail suite exercises readonly hierarchy, navigation, and card-summary composite posture.",
    exportNames: ["DetailSection", "DetailShell", "PageHeader"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["picker-suite", "detail-suite"],
    note: "Picker and detail screens now exercise direct selection summary and picker-shell posture in addition to relation selection.",
    exportNames: ["PickerShell", "RelationPicker", "SelectionSummary"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["media-suite"],
    note: "Media suite exercises thumbnail framing, preview fallback, and embed posture.",
    exportNames: ["MediaPreview", "MediaThumbnail"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["notification-suite"],
    note: "Notification suite exercises transient toast stack posture alongside banner remediation.",
    exportNames: ["ToastStack"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["command-suite"],
    note: "Grouped action list and command palette for keyboard-driven command discovery.",
    exportNames: ["ActionDiscoveryPanel", "CommandPalette"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["workspace-suite"],
    note: "Workspace suite exercises shell hierarchy, docking, split layout, and header posture.",
    exportNames: ["AppHeader", "DockRegion", "SplitView"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "component",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "The catalogue covers the remaining composite exports outside the main suite-specific groupings.",
    exportNames: [
      "BlockEditor",
      "CardRadioGroup",
      "ConfirmAction",
      "DebugDialog",
      "EditableList",
      "EmbedInput",
      "EmbedPreview",
      "ErrorBoundary",
      "FormDialog",
      "FormLayout",
      "InlineListSection",
      "LogList",
      "MarkdownEditor",
      "MediaBrowsePanel",
      "MediaPicker",
      "MetricTile",
      "MediaUploadStatusPanel",
      "PageLoading",
      "SidebarNav",
      "ToastHost",
    ],
  },
  {
    packageName: "@poodle/svelte",
    kind: "helper",
    status: "previewed",
    sectionIds: ["workspace-suite"],
    note: "Workspace suite exercises public layout snapshot round-tripping through the shell state review surface.",
    exportNames: ["parseWorkspaceLayoutSnapshot", "serializeWorkspaceLayoutSnapshot"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "helper",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "Embed helper exports remain reviewable through the catalogue specimen matrix and media suite surfaces.",
    exportNames: ["detectParsedEmbed", "getProviderAccent", "getThumbnailUrl", "lookupMeta", "parseEmbed", "renderEmbed", "resolveEmbedParseState"],
  },
  {
    packageName: "@poodle/svelte",
    kind: "helper",
    status: "previewed",
    sectionIds: ["media-suite"],
    note: "Media workflow helper exports support browse-panel pagination, upload orchestration, and hash-based deduplication.",
    exportNames: [
      "computeFileHash",
      "createResetMediaBrowseState",
      "loadMediaBrowsePage",
      "mergeMediaBrowseItems",
      "runMediaUploadWorkflow",
      "uploadMediaWithKnownHash",
    ],
  },
];

export const packageSurfaceCoverage: PackageSurfaceCoverageEntry[] = packageSurfaceCoverageGroups
  .flatMap((group) =>
    group.exportNames.map((exportName) => ({
      packageName: group.packageName,
      exportName,
      kind: group.kind,
      status: group.status,
      sectionIds: [...group.sectionIds],
      note: group.note,
    })),
  )
  .sort((left, right) =>
    `${left.packageName}:${left.exportName}`.localeCompare(`${right.packageName}:${right.exportName}`),
  );

export const parityTargets: SectionParityTarget[] = docsNavigationSections.map((section) => ({
  sectionId: section.id,
  title: section.title,
  layer: section.layer,
  packageName: section.packageName,
  contractRoot: section.contractRoot,
  summary: section.summary,
  ...sectionParityConfig[section.id],
}));

export function isThemeName(value: string | null | undefined): value is ThemeName {
  return Boolean(value) && themeNames.includes(value as ThemeName);
}

export function isDensityName(value: string | null | undefined): value is DensityName {
  return Boolean(value) && densityNames.includes(value as DensityName);
}

export function isControlSizeName(value: string | null | undefined): value is ControlSizeName {
  return Boolean(value) && controlSizeNames.includes(value as ControlSizeName);
}

export function isDocsSectionId(value: string | null | undefined): value is DocsSectionId {
  return Boolean(value) && docsNavigationSections.some((entry) => entry.id === value);
}

export function normalizePreviewState(
  state: Partial<PreviewHarnessState>,
): PreviewHarnessState {
  return {
    sectionId: isDocsSectionId(state.sectionId) ? state.sectionId : previewStateDefaults.sectionId,
    theme: isThemeName(state.theme) ? state.theme : previewStateDefaults.theme,
    density: isDensityName(state.density) ? state.density : previewStateDefaults.density,
    controlSize: isControlSizeName(state.controlSize)
      ? state.controlSize
      : previewStateDefaults.controlSize,
  };
}

export function parsePreviewLocation(
  searchParams: URLSearchParams,
  hash: string,
): PreviewHarnessState {
  return normalizePreviewState({
    sectionId: hash.replace(/^#/, "") as DocsSectionId,
    theme: searchParams.get("theme") as ThemeName,
    density: searchParams.get("density") as DensityName,
    controlSize: searchParams.get("controlSize") as ControlSizeName,
  });
}

export function buildPreviewUrl(
  state: Partial<PreviewHarnessState>,
  basePath = "/",
): string {
  const normalized = normalizePreviewState(state);
  const searchParams = new URLSearchParams({
    theme: normalized.theme,
    density: normalized.density,
    controlSize: normalized.controlSize,
  });

  return `${basePath}?${searchParams.toString()}#${normalized.sectionId}`;
}
