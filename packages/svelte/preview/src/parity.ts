import { controlSizes, densityModes, themes } from "@pug/svelte-tokens";
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

export type PublicSveltePackageName =
  | "@pug/svelte-primitives"
  | "@pug/svelte-composites"
  | "@pug/svelte-workstation";

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
  layer: "foundation" | "composites" | "workstation" | "tokens";
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
    packageName: "@pug/svelte-primitives",
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
      "SearchField",
      "Switch",
      "TextArea",
      "TextInput",
    ],
  },
  {
    packageName: "@pug/svelte-primitives",
    kind: "component",
    status: "previewed",
    sectionIds: ["notification-suite"],
    note: "The rebuilt shell now exercises shared feedback, menu, and toolbar primitives through the notification-facing shared demo route.",
    exportNames: ["Banner", "Menu", "Skeleton", "StatusIndicator", "Tabs", "Toolbar"],
  },
  {
    packageName: "@pug/svelte-primitives",
    kind: "component",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "The docs shell and shared demo navigation exercise foundational disclosure, control, and chip primitives directly instead of custom shell-only controls.",
    exportNames: ["Accordion", "Button", "Checkbox", "Collapsible", "Pill", "Select", "Toggle", "ToggleGroup"],
  },
  {
    packageName: "@pug/svelte-primitives",
    kind: "component",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "The shared demo target now visibly adopts additional workflow and overlay primitives even when no single legacy docs section owns them cleanly.",
    exportNames: ["Drawer", "Popover", "SegmentedControl", "Table"],
  },
  {
    packageName: "@pug/svelte-primitives",
    kind: "component",
    status: "previewed",
    sectionIds: ["catalog-hub"],
    note: "The rebuilt shared demo now includes a dedicated primitive coverage deck so every public primitive export is directly reviewable from the parity target shell.",
    exportNames: [
      "Badge",
      "Box",
      "Calendar",
      "Combobox",
      "ContextMenu",
      "DateRangePicker",
      "DateTimePicker",
      "DateTimeRangePicker",
      "EditableLabel",
      "Grid",
      "HoverCard",
      "IconButton",
      "Inline",
      "Meter",
      "Menubar",
      "NavigationMenu",
      "NumberEntry",
      "Pagination",
      "PinInput",
      "Progress",
      "RangeCalendar",
      "RangeSlider",
      "Rating",
      "ScrollShell",
      "Separator",
      "Slider",
      "Spacer",
      "Stack",
      "Surface",
      "TabStrip",
      "TimeField",
      "TimeZoneSelect",
      "Tooltip",
      "TriStateSwitch",
      "ZonedDateTimePicker",
    ],
  },
  {
    packageName: "@pug/svelte-composites",
    kind: "component",
    status: "previewed",
    sectionIds: ["table-suite"],
    note: "Table suite exercises structured data, selection, and pagination posture.",
    exportNames: ["BulkActionBar", "DataTable", "FilterToolbar", "PaginationSummary"],
  },
  {
    packageName: "@pug/svelte-composites",
    kind: "component",
    status: "previewed",
    sectionIds: ["browse-suite"],
    note: "Browse suite exercises list, grid, and empty-state surface posture.",
    exportNames: ["EmptyState", "GridShell", "ListShell"],
  },
  {
    packageName: "@pug/svelte-composites",
    kind: "component",
    status: "previewed",
    sectionIds: ["detail-suite"],
    note: "Detail suite exercises readonly hierarchy, navigation, and card summaries.",
    exportNames: ["Breadcrumbs", "Card", "DetailRow", "DetailSection", "DetailShell", "PageHeader"],
  },
  {
    packageName: "@pug/svelte-composites",
    kind: "component",
    status: "previewed",
    sectionIds: ["picker-suite", "detail-suite"],
    note: "Picker and detail screens now exercise direct selection summary and picker-shell posture in addition to relation selection.",
    exportNames: ["PickerShell", "RelationPicker", "SelectionSummary"],
  },
  {
    packageName: "@pug/svelte-composites",
    kind: "component",
    status: "previewed",
    sectionIds: ["media-suite"],
    note: "Media suite exercises thumbnail framing, preview fallback, and embed posture.",
    exportNames: ["EmbedShell", "MediaPreview", "MediaThumbnail"],
  },
  {
    packageName: "@pug/svelte-composites",
    kind: "component",
    status: "previewed",
    sectionIds: ["notification-suite"],
    note: "Notification suite exercises transient toast stack posture alongside banner remediation.",
    exportNames: ["ToastStack"],
  },
  {
    packageName: "@pug/svelte-workstation",
    kind: "component",
    status: "previewed",
    sectionIds: ["command-suite"],
    note: "Command and workspace screen exercises modal launch, inline discovery, and panel-level workstation chrome.",
    exportNames: ["ActionDiscoveryPanel", "CommandPalette", "PanelHeader", "PanelTabs"],
  },
  {
    packageName: "@pug/svelte-workstation",
    kind: "component",
    status: "previewed",
    sectionIds: ["workspace-suite"],
    note: "Workspace suite exercises shell hierarchy, docking, and panel-surface posture.",
    exportNames: [
      "AppHeader",
      "DockRegion",
      "PanelSurface",
      "ProjectHeader",
      "ShellStatusBar",
      "SplitView",
      "SurfaceTabs",
      "WorkspaceShell",
    ],
  },
  {
    packageName: "@pug/svelte-workstation",
    kind: "helper",
    status: "previewed",
    sectionIds: ["workspace-suite"],
    note: "Workspace suite exercises public layout snapshot round-tripping through the shell state review surface.",
    exportNames: ["parseWorkspaceLayoutSnapshot", "serializeWorkspaceLayoutSnapshot"],
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
    sectionId: hash.replace(/^#/, ""),
    theme: searchParams.get("theme"),
    density: searchParams.get("density"),
    controlSize: searchParams.get("controlSize"),
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
