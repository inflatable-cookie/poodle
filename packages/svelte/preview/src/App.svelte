<script lang="ts">
  import {
    applyThemeAttributes,
    aliases,
    cssVars,
    controlSizes,
    densityModes,
    manifest,
    pxToRem,
    themes,
  } from "@pug/svelte-tokens";
  import {
    Breadcrumbs,
    BulkActionBar,
    Card,
    DataTable,
    DetailRow,
    DetailSection,
    DetailShell,
    EmbedShell,
    EmptyState,
    FilterToolbar,
    GridShell,
    ListShell,
    MediaPreview,
    MediaThumbnail,
    PaginationSummary,
    PageHeader,
    RelationPicker,
    type AspectRatio,
    type BrowseState,
    type BreadcrumbItem,
    type MediaKind,
    type MediaState,
    type PickerItem,
    type PickerVariant,
    type SelectionMode,
    type TableColumn,
    type TableRow,
    type TableSortDirection,
    type ToastItem,
    type ToastTone,
    ToastStack,
  } from "@pug/svelte-composites";
  import {
    Accordion,
    Banner,
    Button,
    Checkbox,
    Collapsible,
    Field,
    FormActions,
    Pill,
    SearchField,
    Select,
    Skeleton,
    TextInput,
    Toggle,
    ToggleGroup,
    type AccordionItem,
    type BannerTone,
    type SelectOption,
    type ToggleGroupOption,
    type ValidationState,
  } from "@pug/svelte-primitives";
  import {
    ActionDiscoveryPanel,
    AppHeader,
    CommandPalette,
    DockRegion,
    PanelSurface,
    ProjectHeader,
    SurfaceTabs,
    ShellStatusBar,
    SplitView,
    WorkspaceShell,
    parseWorkspaceLayoutSnapshot,
    serializeWorkspaceLayoutSnapshot,
    type ActionDiscoverySection,
    type CommandActionItem,
    type DiscoveryState,
    type PanelTabItem,
    type SurfaceTabItem,
    type WorkspaceLayoutSnapshot,
    type WorkspaceShellState,
  } from "@pug/svelte-workstation";
  import { onMount } from "svelte";
  import DocsCatalogHub from "./components/DocsCatalogHub.svelte";
  import SharedDemoApp from "./components/SharedDemoApp.svelte";
  import TokenToolsPanel from "./components/TokenToolsPanel.svelte";
  import { docsAdoptionChecklist, docsFamilies, docsSections } from "./catalog";
  import {
    buildPreviewUrl,
    docsNavigationSections,
    parsePreviewLocation,
    type ControlSizeName,
    type DensityName,
    type DocsSectionId,
    type ThemeName,
  } from "./parity";

  type SemanticTokenPath = keyof typeof cssVars;
  type DemoMediaAsset = {
    id: string;
    title: string;
    assetId: string;
    eyebrow: string;
    description: string;
    caption: string;
    kind: Exclude<MediaKind, "embed">;
    aspectRatio: AspectRatio;
    badge: string;
    thumbnailMeta: string;
    meta: string[];
  };
  type CommandResultScope = "all" | "workspace" | "navigation" | "assets" | "recent";
  type WorkspaceSurfaceMeta = {
    eyebrow: string;
    description: string;
    highlights: string[];
  };
  type DockPanelMeta = {
    title: string;
    summary: string;
    items: string[];
  };
  type AppearanceTreatmentName = "system" | "brand-raised";
  type SearchIndexEntry<T> = {
    item: T;
    haystack: string;
  };
  type SharedDemoScreenId =
    | "overview-shell"
    | "form-and-validation"
    | "browse-and-table"
    | "detail-and-related-data"
    | "picker-and-media"
    | "command-and-workspace";
  type PreviewNavigationEntry = {
    id: DocsSectionId;
    title: string;
    eyebrow: string;
    packageName: string;
    contractRoot: string;
    summary: string;
    exampleTypes: string[];
    demoScreenId?: SharedDemoScreenId;
  };

  function optionsFromValues(values: readonly string[]): ToggleGroupOption[] {
    return values.map((value) => ({ value, label: value }));
  }

  function normalizeSearchText(value: string): string {
    return value.trim().toLowerCase();
  }

  const tableColumns: TableColumn[] = [
    { id: "name", label: "Name", isSortable: true },
    { id: "status", label: "Status", isSortable: true },
    { id: "owner", label: "Owner", isSortable: true },
    { id: "updated", label: "Updated", align: "end", isSortable: true },
  ];
  const allRows: TableRow[] = [
    { id: "mix-001", cells: { name: "Kick stem cleanup", status: "Ready", owner: "Clay", updated: "2026-03-11" }, summary: "Drum bus export tranche" },
    { id: "mix-002", cells: { name: "Snare transient pass", status: "Review", owner: "Aura", updated: "2026-03-10" }, summary: "Needs timing sign-off" },
    { id: "mix-003", cells: { name: "Bass harmonic layer", status: "Blocked", owner: "Spark", updated: "2026-03-09" }, summary: "Awaiting synth preset recovery" },
    { id: "mix-004", cells: { name: "Lead vocal comp", status: "Ready", owner: "Clay", updated: "2026-03-08" }, summary: "Comping pass approved" },
    { id: "mix-005", cells: { name: "FX bus automation", status: "Review", owner: "Aura", updated: "2026-03-07" }, summary: "Automation lane handoff" },
    { id: "mix-006", cells: { name: "Room mic cleanup", status: "Ready", owner: "Spark", updated: "2026-03-06" }, summary: "Noise reduction applied" },
    { id: "mix-007", cells: { name: "Master limiter notes", status: "Ready", owner: "Clay", updated: "2026-03-05" }, summary: "Loudness snapshot attached" },
    { id: "mix-008", cells: { name: "Print stem export", status: "Blocked", owner: "Aura", updated: "2026-03-04" }, summary: "Destination folder unavailable" },
    { id: "mix-009", cells: { name: "Dialogue pass", status: "Review", owner: "Spark", updated: "2026-03-03" }, summary: "Noise floor still high" },
    { id: "mix-010", cells: { name: "Cue bounce approval", status: "Ready", owner: "Clay", updated: "2026-03-02" }, summary: "Producer sign-off pending" },
  ];
  const bulkActions = [
    { id: "assign", label: "Assign owner" },
    { id: "archive", label: "Archive" },
    { id: "delete", label: "Delete", tone: "danger" as const },
  ];
  const browseRows = [
    { id: "browser-001", title: "Reference vocal chain", kind: "Preset", status: "Ready", owner: "Aura" },
    { id: "browser-002", title: "Snare top transient pack", kind: "Asset", status: "Review", owner: "Clay" },
    { id: "browser-003", title: "Dialogue cleanup notes", kind: "Document", status: "Blocked", owner: "Spark" },
    { id: "browser-004", title: "Room mic noise profile", kind: "Asset", status: "Ready", owner: "Aura" },
    { id: "browser-005", title: "Broadcast limiter preset", kind: "Preset", status: "Ready", owner: "Clay" },
    { id: "browser-006", title: "Scene cue timing memo", kind: "Document", status: "Review", owner: "Spark" },
    { id: "browser-007", title: "Print master chain", kind: "Preset", status: "Ready", owner: "Aura" },
    { id: "browser-008", title: "Noise floor report", kind: "Document", status: "Blocked", owner: "Clay" },
  ];
  const browseCards = [
    { id: "card-001", title: "Studio dark", category: "Theme", meta: "Compact workstation palette" },
    { id: "card-002", title: "Broadcast review", category: "Template", meta: "Compliance-focused session shell" },
    { id: "card-003", title: "Master bus tools", category: "Collection", meta: "Limiter and metering favorites" },
    { id: "card-004", title: "Dialogue cleanup", category: "Workflow", meta: "Noise and breath reduction stack" },
    { id: "card-005", title: "Percussion archive", category: "Collection", meta: "Tagged loop browser view" },
    { id: "card-006", title: "Cue planning", category: "Template", meta: "Scene-based routing starter" },
  ];
  const browseStatuses = ["all", "Ready", "Review", "Blocked"] as const;
  const detailBreadcrumbs: BreadcrumbItem[] = [
    { value: "workspace", label: "Workspace" },
    { value: "mixes", label: "Mixes" },
    { value: "aura-review", label: "Aura review", isCurrent: true },
  ];
  const detailCards = [
    { id: "health", title: "Health", value: "Ready for review", meta: "No blockers across the active checklist." },
    { id: "owners", title: "Owners", value: "Clay + Aura", meta: "Shared handoff between mix and QA." },
    { id: "deliverables", title: "Deliverables", value: "6 stems", meta: "Broadcast, music, dialogue, FX, M&E, full mix." },
  ];
  const brandProofCards = [
    {
      id: "proof-surface",
      eyebrow: "Surface recipe",
      title: "Tonal panels become branded",
      summary: "Cards and framed sections can pick up gradients, gloss, and softer depth without redefining canonical background tokens.",
      variant: "elevated" as const,
    },
    {
      id: "proof-interactive",
      eyebrow: "Interactive recipe",
      title: "Buttons and tabs stay coordinated",
      summary: "A single scoped treatment keeps CTA chrome, input shells, and segmented controls moving together.",
      variant: "outlined" as const,
    },
    {
      id: "proof-wrapper",
      eyebrow: "App-owned wrapper",
      title: "Website composition stays local",
      summary: "The branded hero structure belongs to the consuming app while the primitives underneath stay shared and documented.",
      variant: "outlined" as const,
    },
  ];
  const relationItems: PickerItem[] = [
    { id: "rel-001", label: "Dialogue cleanup chain", description: "Noise reduction and de-click toolset", meta: "Preset" },
    { id: "rel-002", label: "Broadcast review template", description: "Compliance and routing starter", meta: "Template" },
    { id: "rel-003", label: "Scene cue checklist", description: "Editorial handoff checklist", meta: "Document" },
    { id: "rel-004", label: "Music stem archive", description: "Approved stem collection", meta: "Collection" },
    { id: "rel-005", label: "Lead vocal notes", description: "Review annotations from Aura", meta: "Comment thread" },
    { id: "rel-006", label: "FX impulse pack", description: "Spatial reference impulses", meta: "Asset pack" },
  ];
  const pickerVariants: PickerVariant[] = ["inline", "popover", "modal"];
  const selectionModes: SelectionMode[] = ["single", "multiple"];
  const mediaStates: MediaState[] = ["ready", "loading", "error", "empty"];
  const notificationTones: ToastTone[] = ["info", "success", "warning", "danger"];
  const mediaAssets: DemoMediaAsset[] = [
    {
      id: "asset-001",
      title: "Approval still",
      assetId: "img.approval-still.v4",
      eyebrow: "Image preview",
      description: "Static artwork and frame captures need stable aspect ratio, fallback, and metadata posture.",
      caption: "Image previews should keep labels and fallback meaning even when no bitmap renderer is available.",
      kind: "image",
      aspectRatio: "landscape",
      badge: "PNG",
      thumbnailMeta: "3840×2160",
      meta: ["still frame", "client-facing", "annotated"],
    },
    {
      id: "asset-002",
      title: "Stem waveform",
      assetId: "aud.dialogue-stem.preview",
      eyebrow: "Audio preview",
      description: "Audio-like previews surface timeline cues, peak context, and playback affordances without owning playback engines.",
      caption: "Playback transport remains host-owned; the shared surface only freezes preview framing, metadata, and fallback posture.",
      kind: "audio",
      aspectRatio: "landscape",
      badge: "WAV",
      thumbnailMeta: "01:42",
      meta: ["waveform", "dialogue stem", "48 kHz"],
    },
    {
      id: "asset-003",
      title: "Review clip",
      assetId: "vid.session-review.take-7",
      eyebrow: "Video preview",
      description: "Motion previews need explicit framed state, title, and optional duration even when embed or playback is unavailable.",
      caption: "Video surfaces should preserve preview, fallback, and title semantics consistently across web and desktop runtimes.",
      kind: "video",
      aspectRatio: "video",
      badge: "MOV",
      thumbnailMeta: "00:38",
      meta: ["review clip", "commentary", "client pass"],
    },
    {
      id: "asset-004",
      title: "Delivery brief",
      assetId: "doc.delivery-brief.final",
      eyebrow: "Document preview",
      description: "Document-style previews need a portrait surface, clear fallback, and metadata without pretending all content is renderable inline.",
      caption: "Document previews should degrade to textual summary and explicit open/download actions when embedded rendering is unavailable.",
      kind: "document",
      aspectRatio: "portrait",
      badge: "PDF",
      thumbnailMeta: "12 pages",
      meta: ["handoff", "signed off", "printable"],
    },
  ];
  const workspaceStates: WorkspaceShellState[] = ["ready", "loading", "offline", "disconnected", "empty"];
  const initialSurfaceItems: SurfaceTabItem[] = [
    { value: "mix-review", label: "Mix review" },
    { value: "delivery-brief", label: "Delivery brief", isClosable: true },
    { value: "asset-preview", label: "Asset preview", isClosable: true },
  ];
  const initialLeftDockItems: PanelTabItem[] = [
    { value: "navigator", label: "Navigator", icon: "◫" },
    { value: "sources", label: "Sources", icon: "◩", isClosable: true },
    { value: "queue", label: "Queue", icon: "≣", isClosable: true },
  ];
  const initialRightDockItems: PanelTabItem[] = [
    { value: "metadata", label: "Metadata", icon: "i" },
    { value: "review", label: "Review", icon: "✓", isClosable: true },
    { value: "activity", label: "Activity", icon: "↺", isClosable: true },
  ];
  const workspaceSurfaceCatalog: Record<string, WorkspaceSurfaceMeta> = {
    "mix-review": {
      eyebrow: "Primary work area",
      description: "The active mix surface keeps transport-adjacent detail, approval status, and current shell affordances in one center pane.",
      highlights: ["Approval notes pinned", "Automation diff visible", "Command targets scoped to current mix"],
    },
    "delivery-brief": {
      eyebrow: "Document surface",
      description: "Document-like tabs still participate in the same shell, persistence, and dock orchestration without becoming modal detours.",
      highlights: ["Revision summary attached", "Client checklist visible", "Readable alongside persistent docks"],
    },
    "asset-preview": {
      eyebrow: "Asset surface",
      description: "Asset-centered work stays inside the same workspace shell so relation pickers, metadata, and review utilities do not lose context.",
      highlights: ["Preview surface preserved", "Metadata dock remains adjacent", "Host playback still remains external"],
    },
  };
  const leftDockCatalog: Record<string, DockPanelMeta> = {
    navigator: {
      title: "Navigator",
      summary: "Left docks usually hold browse, jump, and source-scoped context without taking over the center surface.",
      items: ["Mix checklist", "Scene markers", "Open references"],
    },
    sources: {
      title: "Sources",
      summary: "Source lists can collapse, reorder, and restore without mutating the meaning of the active center tab.",
      items: ["Dialogue stem", "Music print", "Impulse archive"],
    },
    queue: {
      title: "Queue",
      summary: "Queued work should preserve explicit order and active selection even when the dock is collapsed and restored.",
      items: ["Render export", "Waveform index", "Delivery zip"],
    },
  };
  const rightDockCatalog: Record<string, DockPanelMeta> = {
    metadata: {
      title: "Metadata",
      summary: "The right dock often carries detail and context that should stay reachable without obscuring the center work area.",
      items: ["Owner: Aura", "Revision: r6", "Delivery target: client review"],
    },
    review: {
      title: "Review",
      summary: "Review utilities remain local to the active surface but do not become the main surface themselves.",
      items: ["2 approvals pending", "1 blocking note", "Latest pass: 19:42"],
    },
    activity: {
      title: "Activity",
      summary: "Activity and shell events belong in persistent utility regions when they need to survive tab switches and layout changes.",
      items: ["Sync resumed", "Layout restored", "Validation rerun"],
    },
  };
  const commandActions: CommandActionItem[] = [
    {
      id: "cmd-open-command-palette",
      title: "Open command palette",
      description: "Focus the shell launcher and search the full action registry.",
      group: "Navigation",
      shortcut: "⌘K",
      badge: "global",
      keywords: ["launcher", "palette", "search"],
    },
    {
      id: "cmd-show-mix-browser",
      title: "Show mix browser",
      description: "Jump to the central list and grid browse surface.",
      group: "Navigation",
      shortcut: "G B",
      keywords: ["browse", "browser", "list", "grid"],
    },
    {
      id: "cmd-focus-review-panel",
      title: "Focus review panel",
      description: "Move attention to the right-side review and approval panel.",
      group: "Workspace",
      shortcut: "⌥2",
      badge: "panel",
      keywords: ["panel", "focus", "review"],
    },
    {
      id: "cmd-toggle-metadata-dock",
      title: "Toggle metadata dock",
      description: "Collapse or restore the metadata dock without changing the active surface.",
      group: "Workspace",
      shortcut: "⌥M",
      badge: "dock",
      keywords: ["dock", "metadata", "toggle"],
    },
    {
      id: "cmd-attach-reference-asset",
      title: "Attach reference asset",
      description: "Open the relation picker scoped to reference assets.",
      group: "Assets",
      shortcut: "A R",
      badge: "asset",
      keywords: ["asset", "attach", "reference"],
    },
    {
      id: "cmd-reveal-current-export",
      title: "Reveal current export",
      description: "Open the host file browser at the active delivery location.",
      group: "Assets",
      shortcut: "⇧⌘R",
      keywords: ["export", "reveal", "finder", "explorer"],
    },
    {
      id: "cmd-rerun-validation",
      title: "Rerun validation",
      description: "Retry the latest async validation flow against the current form and detail surface.",
      group: "Recent",
      shortcut: "⌘↩",
      badge: "recent",
      keywords: ["retry", "validation", "recent"],
    },
    {
      id: "cmd-open-review-brief",
      title: "Open review brief",
      description: "Jump to the document-style media preview for the delivery brief.",
      group: "Recent",
      shortcut: "G D",
      badge: "recent",
      keywords: ["brief", "document", "review"],
    },
  ];
  const commandGroupOrder = ["Navigation", "Workspace", "Assets", "Recent"] as const;
  const commandSectionDescriptions: Record<(typeof commandGroupOrder)[number], string> = {
    Navigation: "Route between major surfaces quickly.",
    Workspace: "High-frequency shell and panel actions.",
    Assets: "Asset-centric commands tied to preview and relation flows.",
    Recent: "Previously used actions kept close for rediscovery.",
  };
  const tableRowSearchIndex: SearchIndexEntry<TableRow>[] = allRows.map((row) => ({
    item: row,
    haystack: normalizeSearchText(`${Object.values(row.cells).join(" ")} ${row.summary ?? ""}`),
  }));
  const browseRowSearchIndex: SearchIndexEntry<(typeof browseRows)[number]>[] = browseRows.map((row) => ({
    item: row,
    haystack: normalizeSearchText([row.title, row.kind, row.status, row.owner].join(" ")),
  }));
  const browseCardSearchIndex: SearchIndexEntry<(typeof browseCards)[number]>[] = browseCards.map((card) => ({
    item: card,
    haystack: normalizeSearchText([card.title, card.category, card.meta].join(" ")),
  }));
  const relationItemSearchIndex: SearchIndexEntry<PickerItem>[] = relationItems.map((item) => ({
    item,
    haystack: normalizeSearchText([item.label, item.description ?? "", item.meta ?? ""].join(" ")),
  }));
  const commandActionSearchIndex = commandActions.map((action) => ({
    action,
    haystack: normalizeSearchText([action.title, action.description ?? "", ...(action.keywords ?? [])].join(" ")),
    title: action.title.toLowerCase(),
  }));
  const catalogEntries = docsSections;
  const catalogEntryMap = Object.fromEntries(catalogEntries.map((entry) => [entry.id, entry]));
  const sectionEntries = docsNavigationSections;
  const sectionEntryMap = Object.fromEntries(sectionEntries.map((entry) => [entry.id, entry]));
  const sharedDemoScreens: Array<{
    id: SharedDemoScreenId;
    representativeSectionId: DocsSectionId;
    title: string;
    eyebrow: string;
    packageName: string;
    contractRoot: string;
    summary: string;
    exampleTypes: string[];
    sourceSectionIds: DocsSectionId[];
  }> = [
    {
      id: "overview-shell",
      representativeSectionId: "notification-suite",
      title: "Overview shell",
      eyebrow: "Shared demo",
      packageName: "@pug/svelte-primitives + @pug/svelte-composites + @pug/svelte-workstation",
      contractRoot: "packages/shared-demo-app-contract.json",
      summary: "Top-level shell posture, visible remediation, and shared runtime status without docs-only chrome.",
      exampleTypes: ["shell", "status", "remediation"],
      sourceSectionIds: ["notification-suite"],
    },
    {
      id: "form-and-validation",
      representativeSectionId: "form-suite",
      title: "Form and validation",
      eyebrow: "Shared demo",
      packageName: "@pug/svelte-primitives",
      contractRoot: "packages/shared-demo-app-contract.json",
      summary: "Real form workflow using field, text-entry, selection, dialog, and remediation primitives together.",
      exampleTypes: ["default", "invalid", "pending", "disabled"],
      sourceSectionIds: ["form-suite"],
    },
    {
      id: "browse-and-table",
      representativeSectionId: "browse-suite",
      title: "Browse and table",
      eyebrow: "Shared demo",
      packageName: "@pug/svelte-composites + @pug/svelte-primitives",
      contractRoot: "packages/shared-demo-app-contract.json",
      summary: "Search, filter, selection, list/grid switching, and structured data in one browse workflow.",
      exampleTypes: ["ready", "selection", "empty", "loading"],
      sourceSectionIds: ["browse-suite", "table-suite"],
    },
    {
      id: "detail-and-related-data",
      representativeSectionId: "detail-suite",
      title: "Detail and related data",
      eyebrow: "Shared demo",
      packageName: "@pug/svelte-composites",
      contractRoot: "packages/shared-demo-app-contract.json",
      summary: "Headers, metadata, summaries, and related-data posture as one detail screen.",
      exampleTypes: ["summary", "metadata", "related data"],
      sourceSectionIds: ["detail-suite"],
    },
    {
      id: "picker-and-media",
      representativeSectionId: "picker-suite",
      title: "Picker and media",
      eyebrow: "Shared demo",
      packageName: "@pug/svelte-composites + @pug/svelte-primitives",
      contractRoot: "packages/shared-demo-app-contract.json",
      summary: "Picker workflow, selection summary, preview framing, and embed fallback on one screen.",
      exampleTypes: ["inline", "overlay", "ready", "fallback"],
      sourceSectionIds: ["picker-suite", "media-suite"],
    },
    {
      id: "command-and-workspace",
      representativeSectionId: "command-suite",
      title: "Command and workspace",
      eyebrow: "Shared demo",
      packageName: "@pug/svelte-workstation",
      contractRoot: "packages/shared-demo-app-contract.json",
      summary: "Command discovery, docks, tabs, split views, and persistence inside one workstation scene.",
      exampleTypes: ["command", "workspace", "layout", "persistence"],
      sourceSectionIds: ["command-suite", "workspace-suite"],
    },
  ];
  const sharedDemoScreenMap = Object.fromEntries(sharedDemoScreens.map((screen) => [screen.id, screen]));
  const sharedDemoScreenBySection = Object.fromEntries(
    sharedDemoScreens.flatMap((screen) =>
      screen.sourceSectionIds.map((sectionId) => [sectionId, screen.id]),
    ),
  ) as Partial<Record<DocsSectionId, SharedDemoScreenId>>;
  const previewNavigationEntries: PreviewNavigationEntry[] = [
    {
      id: "catalog-hub",
      title: sectionEntryMap["catalog-hub"].title,
      eyebrow: sectionEntryMap["catalog-hub"].eyebrow,
      packageName: sectionEntryMap["catalog-hub"].packageName,
      contractRoot: sectionEntryMap["catalog-hub"].contractRoot,
      summary: sectionEntryMap["catalog-hub"].summary,
      exampleTypes: sectionEntryMap["catalog-hub"].exampleTypes,
    },
    ...sharedDemoScreens.map((screen) => ({
      id: screen.representativeSectionId,
      title: screen.title,
      eyebrow: screen.eyebrow,
      packageName: screen.packageName,
      contractRoot: screen.contractRoot,
      summary: screen.summary,
      exampleTypes: screen.exampleTypes,
      demoScreenId: screen.id,
    })),
    {
      id: "token-summary-section",
      title: sectionEntryMap["token-summary-section"].title,
      eyebrow: sectionEntryMap["token-summary-section"].eyebrow,
      packageName: sectionEntryMap["token-summary-section"].packageName,
      contractRoot: sectionEntryMap["token-summary-section"].contractRoot,
      summary: sectionEntryMap["token-summary-section"].summary,
      exampleTypes: sectionEntryMap["token-summary-section"].exampleTypes,
    },
    {
      id: "token-inspector",
      title: sectionEntryMap["token-inspector"].title,
      eyebrow: sectionEntryMap["token-inspector"].eyebrow,
      packageName: sectionEntryMap["token-inspector"].packageName,
      contractRoot: sectionEntryMap["token-inspector"].contractRoot,
      summary: sectionEntryMap["token-inspector"].summary,
      exampleTypes: sectionEntryMap["token-inspector"].exampleTypes,
    },
  ];
  const sectionNavigationOptions: SelectOption[] = previewNavigationEntries.map((entry) => ({
    value: entry.id,
    label: entry.title,
  }));
  const sharedDemoScreenOptions: ToggleGroupOption[] = sharedDemoScreens.map((screen) => ({
    value: screen.id,
    label: screen.title.replace(" and ", " + "),
  }));

  const themeEntries = Object.entries(themes) as [ThemeName, (typeof themes)[ThemeName]][];
  const densityEntries = Object.entries(densityModes) as [DensityName, (typeof densityModes)[DensityName]][];
  const controlSizeEntries = Object.entries(controlSizes) as [ControlSizeName, (typeof controlSizes)[ControlSizeName]][];
  const appearanceTreatmentEntries: Array<{
    name: AppearanceTreatmentName;
    description: string;
  }> = [
    {
      name: "system",
      description: "Canonical application treatment roles derived from semantic tokens.",
    },
    {
      name: "brand-raised",
      description:
        "Scoped raised/gradient override proving recipe-level extension without changing token meaning.",
    },
  ];
  const railSectionItems: AccordionItem[] = [
    {
      value: "display-controls",
      label: "Display controls",
      description: "Theme, density, control sizing, and appearance treatment for the active review surface.",
    },
    {
      value: "state-probes",
      label: "State probes",
      description: "Accessibility-oriented interaction checks for the current examples.",
    },
    {
      value: "reference",
      label: "Reference",
      description: "Artifact counts, command entry points, and ownership anchors for the current build.",
    },
  ];
  const themeOptions: ToggleGroupOption[] = themeEntries.map(([name]) => ({ value: name, label: name }));
  const densityOptions: ToggleGroupOption[] = densityEntries.map(([name]) => ({ value: name, label: name }));
  const controlSizeOptions: ToggleGroupOption[] = controlSizeEntries.map(([name]) => ({ value: name, label: name }));
  const appearanceTreatmentOptions: ToggleGroupOption[] = appearanceTreatmentEntries.map((entry) => ({
    value: entry.name,
    label: entry.name,
  }));
  const browseStatusOptions = optionsFromValues(browseStatuses);
  const browseStateOptions = optionsFromValues(["auto", "loading", "error", "empty"] as const);
  const detailStateOptions = optionsFromValues(["ready", "loading", "error", "empty"] as const);
  const pickerVariantOptions = optionsFromValues(pickerVariants);
  const selectionModeOptions = optionsFromValues(selectionModes);
  const pickerStateOptions = optionsFromValues(["auto", "loading", "error", "empty"] as const);
  const mediaStateOptions = optionsFromValues(mediaStates);
  const notificationToneOptions = optionsFromValues(notificationTones);
  const commandScopeOptions = optionsFromValues(["all", "workspace", "navigation", "assets", "recent"] as const);
  const commandStateOptions = optionsFromValues(["auto", "loading", "error", "empty"] as const);
  const workspaceStateOptions = optionsFromValues(workspaceStates);
  const semanticPaths = Object.keys(cssVars) as SemanticTokenPath[];
  const keySemanticPaths: SemanticTokenPath[] = [
    "semantic.color.background.canvas",
    "semantic.color.background.panel",
    "semantic.color.background.elevated",
    "semantic.color.text.primary",
    "semantic.color.text.secondary",
    "semantic.color.border.default",
    "semantic.color.accent.base",
    "semantic.color.status.success",
    "semantic.size.control.height",
    "semantic.space.control.x",
    "semantic.space.control.y",
  ];

  let appShell: HTMLElement | null = null;
  let previewRoot: HTMLElement | null = null;
  let theme: ThemeName = "loophole-studio";
  let density: DensityName = "compact";
  let controlSize: ControlSizeName = "md";
  let appearanceTreatment: AppearanceTreatmentName = "system";
  let activeSectionId: DocsSectionId = "catalog-hub";
  let disabled = false;
  let invalid = true;
  let busy = false;
  let projectTitle = "Aura mix review";
  let assetSearch = "track lane automation";
  let validationLog = "No validation cycle has run yet.";
  let tableStatus = "Bulk actions remain hidden until at least one visible row is selected.";
  let inspectorQuery = "";
  let liveTokenValues: Partial<Record<SemanticTokenPath, string>> = {};
  let filteredTokens: { path: SemanticTokenPath; value: string }[] = [];
  let keySemanticTokens: { path: SemanticTokenPath; value: string }[] = [];
  let matchingTokenCount = 0;
  let previewModeKey = "";
  let appliedPreviewModeKey = "";
  let hasMounted = false;
  let selectedRowIds: string[] = [];
  let sortColumnId: string | null = "updated";
  let sortDirection: TableSortDirection = "desc";
  let currentPage = 1;
  const pageSize = 5;
  let browseQuery = "";
  let browseStatus = "all";
  let browseStateOverride: "auto" | BrowseState = "auto";
  let browseVisibleCount = 4;
  let gridPage = 1;
  const gridPageSize = 4;
  let detailState: "ready" | "loading" | "empty" | "error" = "ready";
  let pickerQuery = "";
  let pickerVariant: PickerVariant = "popover";
  let pickerMode: SelectionMode = "multiple";
  let pickerStateOverride: "auto" | BrowseState = "auto";
  let selectedRelationIds: string[] = ["rel-001", "rel-004"];
  let pickerStatus = "Picker workflows stay host-controlled even when the shared shell owns search, browse, and confirm structure.";
  let mediaState: MediaState = "ready";
  let embedState: MediaState = "ready";
  let activeMediaId = mediaAssets[0]?.id ?? "";
  let bannerTone: BannerTone = "warning";
  let showPersistentBanner = true;
  let toastSequence = 3;
  let toastItems: ToastItem[] = [
    {
      id: "toast-001",
      title: "Autosave complete",
      message: "The current review surface checkpoint was stored successfully.",
      tone: "success",
    },
    {
      id: "toast-002",
      title: "Remote sync delayed",
      message: "Background sync is still retrying. Local work remains available.",
      tone: "warning",
      actionLabel: "View sync",
    },
  ];
  let commandPaletteOpen = false;
  let commandQuery = "";
  let commandScope: CommandResultScope = "all";
  let commandStateOverride: "auto" | DiscoveryState = "auto";
  let lastCommandId: string | null = null;
  let commandEventLog = "No command has been executed yet.";
  let workspaceState: WorkspaceShellState = "ready";
  let workspaceSurfaceItems = [...initialSurfaceItems];
  let workspaceSurfaceValue = workspaceSurfaceItems[0]?.value ?? "mix-review";
  let leftDockItems = [...initialLeftDockItems];
  let leftDockValue = leftDockItems[0]?.value ?? null;
  let leftDockCollapsed = false;
  let rightDockItems = [...initialRightDockItems];
  let rightDockValue = rightDockItems[0]?.value ?? null;
  let rightDockCollapsed = false;
  let primarySplitRatio = 0.24;
  let secondarySplitRatio = 0.72;
  let surfaceSequence = workspaceSurfaceItems.length;
  let workspaceEventLog = "No dock, split, or persistence event has been triggered yet.";

  $: titleValidationState =
    invalid ? "invalid" : busy ? "pending" : projectTitle.trim().length > 0 ? "valid" : "none";
  $: searchValidationState = busy ? "pending" : "none";
  $: titleError =
    invalid ? "Title must include a specific destination or revision marker for downstream handoff." : null;
  $: titlePendingMessage = busy ? "Checking title against remote validation rules..." : null;
  $: validationLog =
    busy
      ? "Pending validation is active. Both runtimes need equivalent busy and announcement semantics."
      : invalid
        ? "Validation is failing. Error text is attached through the field wrapper rather than placeholder copy."
        : "Field wrapper, help text, and action-row semantics are currently aligned for the Svelte baseline.";
  $: normalizedQuery = normalizeSearchText(assetSearch);
  $: filteredRows =
    normalizedQuery.length === 0
      ? allRows
      : tableRowSearchIndex
          .filter((entry) => entry.haystack.includes(normalizedQuery))
          .map((entry) => entry.item);
  $: sortedRows = [...filteredRows].sort((left, right) => {
    if (!sortColumnId) {
      return 0;
    }

    const leftValue = left.cells[sortColumnId] ?? "";
    const rightValue = right.cells[sortColumnId] ?? "";
    const direction = sortDirection === "asc" ? 1 : -1;

    return leftValue.localeCompare(rightValue) * direction;
  });
  $: totalPages = Math.max(1, Math.ceil(sortedRows.length / pageSize));
  $: if (currentPage > totalPages) {
    currentPage = totalPages;
  }
  $: visibleRows = sortedRows.slice((currentPage - 1) * pageSize, currentPage * pageSize);
  $: visibleRowIds = visibleRows.map((row) => row.id);
  $: sortedRowIdSet = new Set(sortedRows.map((row) => row.id));
  $: selectedRowIds = selectedRowIds.filter((rowId) => sortedRowIdSet.has(rowId));
  $: browseNormalizedQuery = normalizeSearchText(browseQuery);
  $: filteredBrowseRows = browseRowSearchIndex
    .filter(
      (entry) =>
        (browseStatus === "all" || entry.item.status === browseStatus) &&
        (browseNormalizedQuery.length === 0 || entry.haystack.includes(browseNormalizedQuery)),
    )
    .map((entry) => entry.item);
  $: filteredBrowseCards =
    browseNormalizedQuery.length === 0
      ? browseCards
      : browseCardSearchIndex
          .filter((entry) => entry.haystack.includes(browseNormalizedQuery))
          .map((entry) => entry.item);
  $: listShellState =
    browseStateOverride !== "auto"
      ? browseStateOverride
      : browseRows.length === 0
        ? "empty"
        : filteredBrowseRows.length === 0
          ? "no-results"
          : "ready";
  $: gridShellState =
    browseStateOverride !== "auto"
      ? browseStateOverride
      : browseCards.length === 0
        ? "empty"
        : filteredBrowseCards.length === 0
          ? "no-results"
          : "ready";
  $: visibleBrowseRows = filteredBrowseRows.slice(0, browseVisibleCount);
  $: gridTotalPages = Math.max(1, Math.ceil(filteredBrowseCards.length / gridPageSize));
  $: if (gridPage > gridTotalPages) {
    gridPage = gridTotalPages;
  }
  $: visibleBrowseCards = filteredBrowseCards.slice((gridPage - 1) * gridPageSize, gridPage * gridPageSize);
  $: browseSummary =
    browseStateOverride === "loading"
      ? "Loading browse results..."
      : browseStateOverride === "error"
        ? "Browse results unavailable."
        : `${filteredBrowseRows.length} list results, ${filteredBrowseCards.length} grid results, ${browseStatus === "all" ? "all statuses" : browseStatus}.`;
  $: pickerNormalizedQuery = normalizeSearchText(pickerQuery);
  $: filteredRelationItems =
    pickerNormalizedQuery.length === 0
      ? relationItems
      : relationItemSearchIndex
          .filter((entry) => entry.haystack.includes(pickerNormalizedQuery))
          .map((entry) => entry.item);
  $: pickerState =
    pickerStateOverride !== "auto"
      ? pickerStateOverride
      : relationItems.length === 0
        ? "empty"
        : filteredRelationItems.length === 0
          ? "no-results"
          : "ready";
  $: relationItemIdSet = new Set(relationItems.map((item) => item.id));
  $: selectedRelationIds = selectedRelationIds.filter((id) => relationItemIdSet.has(id));
  $: activeMedia = mediaAssets.find((asset) => asset.id === activeMediaId) ?? mediaAssets[0];
  $: secondaryMediaAssets = mediaAssets.filter((asset) => asset.id !== activeMedia.id).slice(0, 2);
  $: mediaStatus =
    mediaState === "ready"
      ? `${activeMedia.title} is active. Shared preview shells own framing, not playback engines or host-specific media tooling.`
      : mediaState === "loading"
        ? "Media previews are reserving layout and metadata while the renderer resolves."
        : mediaState === "error"
          ? "Preview rendering failed. Fallback text and asset actions remain reachable."
          : "No generated preview is available. Asset identity still remains visible.";
  $: embedStatus =
    embedState === "ready"
      ? "Embed shells are available. Host code still owns origin, permissions, and actual embedded runtime selection."
      : embedState === "loading"
        ? "Embed shells stay framed while external content or host-native views initialize."
        : embedState === "error"
          ? "Embed rendering failed. Fallback posture must still preserve title, context, and recovery actions."
          : "No embedded surface is available. Users still need an explicit fallback path.";
  $: notificationSummary = `${toastItems.length} transient notification(s) queued. Persistent banner is ${showPersistentBanner ? "visible" : "dismissed"}.`;
  $: scopedCommandActionEntries = commandActionSearchIndex.filter(({ action }) => {
    if (commandScope === "all") {
      return true;
    }

    if (commandScope === "recent") {
      return action.group === "Recent";
    }

    if (commandScope === "workspace") {
      return action.group === "Workspace";
    }

    if (commandScope === "navigation") {
      return action.group === "Navigation";
    }

    return action.group === "Assets";
  });
  $: scopedCommandActions = scopedCommandActionEntries.map((entry) => entry.action);
  $: normalizedCommandQuery = normalizeSearchText(commandQuery);
  $: filteredCommandEntries = scopedCommandActionEntries
    .map((entry) => {
      if (normalizedCommandQuery.length === 0) {
        return { action: entry.action, score: 0 };
      }

      if (!entry.haystack.includes(normalizedCommandQuery)) {
        return null;
      }

      const score = entry.title.startsWith(normalizedCommandQuery) ? 3 : entry.title.includes(normalizedCommandQuery) ? 2 : 1;
      return { action: entry.action, score };
    })
    .filter((entry): entry is { action: CommandActionItem; score: number } => entry !== null)
    .sort((left, right) => right.score - left.score || left.action.title.localeCompare(right.action.title))
  $: filteredCommandActions = filteredCommandEntries.map((entry) => entry.action);
  $: commandPaletteState =
    commandStateOverride !== "auto"
      ? commandStateOverride
      : scopedCommandActions.length === 0
        ? "empty"
        : filteredCommandActions.length === 0
          ? "no-results"
          : "ready";
  $: commandSections = (() => {
    const groupedActions: Record<(typeof commandGroupOrder)[number], CommandActionItem[]> = {
      Navigation: [],
      Workspace: [],
      Assets: [],
      Recent: [],
    };

    for (const action of filteredCommandActions) {
      const group = action.group as (typeof commandGroupOrder)[number];
      const limit = group === "Recent" ? 2 : 3;
      if (groupedActions[group].length < limit) {
        groupedActions[group].push(action);
      }
    }

    return commandGroupOrder.reduce<ActionDiscoverySection[]>((accumulator, group) => {
      const actions = groupedActions[group];
      if (actions.length > 0) {
        accumulator.push({
          id: group.toLowerCase(),
          title: group,
          description: commandSectionDescriptions[group],
          actions,
        });
      }
      return accumulator;
    }, []);
  })();
  $: commandStatus =
    commandPaletteState === "ready"
      ? `${filteredCommandActions.length} commands available in the ${commandScope} scope. Host ordering remains explicit and inspectable.`
      : commandPaletteState === "loading"
        ? "Command discovery is loading while the launcher stays open and focusable."
        : commandPaletteState === "error"
          ? "Command discovery failed. Recovery actions should stay adjacent to the launcher."
          : commandPaletteState === "empty"
            ? "No commands are configured for the current scope."
            : "No commands match the current query.";
  $: workspaceStatus =
    workspaceState === "ready"
      ? "Workspace shell is interactive. Utility regions, command launch, and active surface context are all visible."
      : workspaceState === "loading"
        ? "Workspace chrome stays visible while panels and center content are loading."
        : workspaceState === "offline"
          ? "The shell remains usable in offline mode, but sync-dependent actions need explicit status."
          : workspaceState === "disconnected"
            ? "Connection loss is distinct from intentional offline mode and should keep recovery actions nearby."
            : "No workspace content is configured for the current shell state.";
  $: activeWorkspaceSurface =
    workspaceSurfaceItems.find((item) => item.value === workspaceSurfaceValue) ?? workspaceSurfaceItems[0] ?? null;
  $: activeWorkspaceSurfaceMeta =
    workspaceSurfaceCatalog[activeWorkspaceSurface?.value ?? "mix-review"] ?? workspaceSurfaceCatalog["mix-review"];
  $: activeLeftDockItem = leftDockItems.find((item) => item.value === leftDockValue) ?? leftDockItems[0] ?? null;
  $: activeLeftDockMeta = activeLeftDockItem ? leftDockCatalog[activeLeftDockItem.value] : null;
  $: activeRightDockItem = rightDockItems.find((item) => item.value === rightDockValue) ?? rightDockItems[0] ?? null;
  $: activeRightDockMeta = activeRightDockItem ? rightDockCatalog[activeRightDockItem.value] : null;
  $: workspaceLayoutSnapshot = ({
    version: 1,
    activeSurface: activeWorkspaceSurface?.value ?? "mix-review",
    surfaceOrder: workspaceSurfaceItems.map((item) => item.value),
    primarySplitRatio,
    secondarySplitRatio,
    leftDock: {
      edge: "left",
      isCollapsed: leftDockCollapsed,
      activePanel: activeLeftDockItem?.value ?? null,
      order: leftDockItems.map((item) => item.value),
    },
    rightDock: {
      edge: "right",
      isCollapsed: rightDockCollapsed,
      activePanel: activeRightDockItem?.value ?? null,
      order: rightDockItems.map((item) => item.value),
    },
  }) as WorkspaceLayoutSnapshot;
  $: serializedWorkspaceLayout = serializeWorkspaceLayoutSnapshot(workspaceLayoutSnapshot);
  $: parsedWorkspaceLayout = parseWorkspaceLayoutSnapshot(serializedWorkspaceLayout);
  $: workspacePersistenceSummary =
    parsedWorkspaceLayout.activeSurface === workspaceLayoutSnapshot.activeSurface &&
    parsedWorkspaceLayout.primarySplitRatio === workspaceLayoutSnapshot.primarySplitRatio &&
    parsedWorkspaceLayout.secondarySplitRatio === workspaceLayoutSnapshot.secondarySplitRatio
      ? "Round-trip serialization preserves the current shell layout snapshot."
      : "Layout serialization drift detected.";
  $: currentDemoScreenId = sharedDemoScreenBySection[activeSectionId] ?? null;
  $: currentDemoScreen = currentDemoScreenId ? sharedDemoScreenMap[currentDemoScreenId] : null;
  $: activeNavigationId = currentDemoScreen?.representativeSectionId ?? activeSectionId;
  $: activeNavigationIndex = Math.max(
    0,
    previewNavigationEntries.findIndex((entry) => entry.id === activeNavigationId),
  );
  $: activeNavigationEntry =
    previewNavigationEntries.find((entry) => entry.id === activeNavigationId) ?? previewNavigationEntries[0];
  $: previousSection = activeNavigationIndex > 0 ? previewNavigationEntries[activeNavigationIndex - 1] : null;
  $: nextSection =
    activeNavigationIndex < previewNavigationEntries.length - 1
      ? previewNavigationEntries[activeNavigationIndex + 1]
      : null;
  $: heroEyebrow = currentDemoScreen ? "Shared demo target" : "Docs shell";
  $: heroTitle = currentDemoScreen
    ? "Cross-runtime demo target for Svelte and GPUI"
    : activeSectionId === "catalog-hub"
      ? "Catalog, contracts, and inspection surface"
      : "Token runtime inspection and artifact provenance";
  $: heroCopy = currentDemoScreen
    ? "The demo app now groups shared screens by believable workflows so GPUI can target the same UI rather than a loose pile of docs examples."
    : activeSectionId === "catalog-hub"
      ? "The docs shell remains the authority for catalog navigation, package ownership, contract provenance, and adoption framing."
      : "Token tools stay outside the shared demo target so live values and emitted artifacts remain inspectable without polluting the app shell.";

  function readSemanticTokenValues(element: HTMLElement): Partial<Record<SemanticTokenPath, string>> {
    const styles = getComputedStyle(element);

    return semanticPaths.reduce<Partial<Record<SemanticTokenPath, string>>>((accumulator, path) => {
      accumulator[path] = styles.getPropertyValue(cssVars[path]).trim();
      return accumulator;
    }, {});
  }

  function refreshPreviewSurface(): void {
    if (!appShell) {
      return;
    }

    applyThemeAttributes(appShell, { theme, density, controlSize });
    liveTokenValues = readSemanticTokenValues(appShell);
    appliedPreviewModeKey = previewModeKey;
  }

  $: filteredTokens = semanticPaths
    .filter((path) =>
      path.toLowerCase().includes(inspectorQuery.trim().toLowerCase()),
    )
    .map((path) => ({
      path,
      value: liveTokenValues[path] ?? "",
    }));

  $: keySemanticTokens = keySemanticPaths.map((path) => ({
    path,
    value: liveTokenValues[path] ?? "",
  }));

  $: matchingTokenCount = filteredTokens.length;

  $: previewModeKey = `${theme}:${density}:${controlSize}`;

  $: if (appShell && previewModeKey && previewModeKey !== appliedPreviewModeKey) {
    refreshPreviewSurface();
  }

  function syncCurrentLocation(): void {
    if (typeof window === "undefined") {
      return;
    }

    const nextState = parsePreviewLocation(
      new URLSearchParams(window.location.search),
      window.location.hash,
    );
    theme = nextState.theme;
    density = nextState.density;
    controlSize = nextState.controlSize;
    activeSectionId = nextState.sectionId;
  }

  onMount(() => {
    syncCurrentLocation();
    hasMounted = true;
    refreshPreviewSurface();
  });

  $: if (hasMounted && typeof window !== "undefined") {
    const nextUrl = buildPreviewUrl(
      {
        sectionId: activeSectionId,
        theme,
        density,
        controlSize,
      },
      window.location.pathname,
    );
    const currentUrl = `${window.location.pathname}${window.location.search}${window.location.hash}`;
    if (nextUrl !== currentUrl) {
      window.history.replaceState(null, "", nextUrl);
    }
  }

  function selectSection(sectionId: DocsSectionId): void {
    activeSectionId = sectionId;

    if (typeof window !== "undefined") {
      window.scrollTo({ top: 0, behavior: "smooth" });
    }
  }

  function handleSectionNavigationChange(event: CustomEvent<{ value: string }>): void {
    selectSection(event.detail.value as DocsSectionId);
  }

  function handleDemoScreenChange(screenId: string): void {
    const nextScreen = sharedDemoScreenMap[screenId as SharedDemoScreenId];
    if (nextScreen) {
      selectSection(nextScreen.representativeSectionId);
    }
  }

  function handleDisabledChange(event: CustomEvent<{ checked: boolean }>): void {
    disabled = event.detail.checked;
  }

  function handleInvalidChange(event: CustomEvent<{ checked: boolean }>): void {
    invalid = event.detail.checked;
  }

  function handleBusyChange(event: CustomEvent<{ checked: boolean }>): void {
    busy = event.detail.checked;
  }

  function handleTitleChange(event: CustomEvent<{ value: string }>): void {
    projectTitle = event.detail.value;
  }

  function remHeight(px: number): string {
    return pxToRem(px);
  }

  function handleSearchChange(event: CustomEvent<{ value: string }>): void {
    assetSearch = event.detail.value;
    currentPage = 1;
  }

  function handleSearchClear(): void {
    assetSearch = "";
    currentPage = 1;
  }

  function handleBrowseSearchChange(event: CustomEvent<{ value: string }>): void {
    browseQuery = event.detail.value;
    browseVisibleCount = 4;
    gridPage = 1;
  }

  function handleBrowseSearchClear(): void {
    browseQuery = "";
    browseVisibleCount = 4;
    gridPage = 1;
  }

  function handleSubmit(event: Event): void {
    event.preventDefault();
    validationLog = `Submitted title "${projectTitle}" with search query "${assetSearch}".`;
  }

  function handleSortChange(event: CustomEvent<{ columnId: string; direction: TableSortDirection }>): void {
    sortColumnId = event.detail.columnId;
    sortDirection = event.detail.direction;
    tableStatus = `Sorted by ${event.detail.columnId} in ${event.detail.direction} order.`;
  }

  function handleRowToggle(event: CustomEvent<{ rowId: string; selected: boolean }>): void {
    selectedRowIds = event.detail.selected
      ? [...selectedRowIds, event.detail.rowId]
      : selectedRowIds.filter((rowId) => rowId !== event.detail.rowId);
    tableStatus = `${selectedRowIds.length} row(s) selected across the filtered result set.`;
  }

  function handleToggleAll(event: CustomEvent<{ selected: boolean }>): void {
    selectedRowIds = event.detail.selected
      ? Array.from(new Set([...selectedRowIds, ...visibleRowIds]))
      : selectedRowIds.filter((rowId) => !visibleRowIds.includes(rowId));
    tableStatus = event.detail.selected
      ? `Selected all ${visibleRowIds.length} visible rows on the current page.`
      : "Cleared selection for the current page.";
  }

  function handleBulkAction(event: CustomEvent<{ id: string }>): void {
    tableStatus = `Bulk action "${event.detail.id}" requested for ${selectedRowIds.length} selected row(s).`;
  }

  function clearTableSelection(): void {
    selectedRowIds = [];
    tableStatus = "Selection cleared.";
  }

  function handleRowAction(event: CustomEvent<{ rowId: string }>): void {
    tableStatus = `Opened row action menu for ${event.detail.rowId}.`;
  }

  function handlePageChange(event: CustomEvent<{ page: number }>): void {
    currentPage = event.detail.page;
    tableStatus = `Moved to page ${event.detail.page}.`;
  }

  function setBrowseStatus(nextStatus: (typeof browseStatuses)[number]): void {
    browseStatus = nextStatus;
    browseVisibleCount = 4;
    gridPage = 1;
  }

  function setBrowseState(nextState: "auto" | BrowseState): void {
    browseStateOverride = nextState;
  }

  function handleBrowseStatusChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      setBrowseStatus(event.detail.value as (typeof browseStatuses)[number]);
    }
  }

  function handleBrowseStateChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      setBrowseState(event.detail.value as "auto" | BrowseState);
    }
  }

  function loadMoreBrowseRows(): void {
    browseVisibleCount = Math.min(filteredBrowseRows.length, browseVisibleCount + 3);
  }

  function handleGridPageChange(event: CustomEvent<{ page: number }>): void {
    gridPage = event.detail.page;
  }

  function handleBreadcrumbNavigate(event: CustomEvent<{ value: string }>): void {
    validationLog = `Breadcrumb navigation requested for "${event.detail.value}".`;
  }

  function handlePickerQueryChange(event: CustomEvent<{ value: string }>): void {
    pickerQuery = event.detail.value;
  }

  function handleRelationSelectionChange(event: CustomEvent<{ selectedIds: string[] }>): void {
    selectedRelationIds = event.detail.selectedIds;
    pickerStatus = `${event.detail.selectedIds.length} relation item(s) currently selected in ${pickerMode} mode.`;
  }

  function handlePickerConfirm(event: CustomEvent<{ selectedIds: string[] }>): void {
    pickerStatus = `Confirmed ${event.detail.selectedIds.length} relation item(s) from the ${pickerVariant} picker flow.`;
  }

  function handlePickerCancel(): void {
    pickerStatus = "Picker workflow cancelled. Selection remains host-owned.";
  }

  function handleDetailStateChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      detailState = event.detail.value as typeof detailState;
    }
  }

  function handlePickerVariantChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      pickerVariant = event.detail.value as PickerVariant;
    }
  }

  function handlePickerModeChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      pickerMode = event.detail.value as SelectionMode;
      selectedRelationIds =
        pickerMode === "single" ? selectedRelationIds.slice(0, 1) : selectedRelationIds;
    }
  }

  function handlePickerStateChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      pickerStateOverride = event.detail.value as "auto" | BrowseState;
    }
  }

  function getMediaStateTitle(state: MediaState, kind: MediaKind): string {
    if (state === "loading") {
      return `Loading ${kind} preview`;
    }

    if (state === "error") {
      return `${kind[0].toUpperCase()}${kind.slice(1)} preview unavailable`;
    }

    return `No ${kind} preview`;
  }

  function getMediaStateMessage(state: MediaState, kind: MediaKind): string | null {
    if (state === "loading") {
      return "Reserve the preview frame while metadata and shell actions remain stable.";
    }

    if (state === "error") {
      return `Keep ${kind} identity, fallback copy, and recovery actions visible when rendering fails.`;
    }

    if (state === "empty") {
      return "Not every asset guarantees a generated preview; the empty state should remain intentional rather than broken-looking.";
    }

    return null;
  }

  function getEmbedStateMessage(state: MediaState): string | null {
    if (state === "loading") {
      return "Maintain a stable framed region while the embedded surface initializes.";
    }

    if (state === "error") {
      return "Render fallback guidance and recovery actions instead of collapsing the shell.";
    }

    if (state === "empty") {
      return "Some destinations may choose an open-in-host path instead of inline embedding.";
    }

    return null;
  }

  function resetBrowseFilters(): void {
    browseQuery = "";
    browseStatus = "all";
    browseVisibleCount = 4;
    gridPage = 1;
    browseStateOverride = "auto";
  }

  function retryBrowseState(): void {
    browseStateOverride = "auto";
    toastSequence += 1;
    toastItems = [
      {
        id: `toast-${String(toastSequence).padStart(3, "0")}`,
        title: "Browse retry requested",
        message: "The shared shell posture was reset to automatic host-driven state.",
        tone: "info",
      },
      ...toastItems,
    ];
  }

  function resetPickerState(): void {
    pickerStateOverride = "auto";
    pickerQuery = "";
  }

  function handleMediaStateChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      mediaState = event.detail.value as MediaState;
    }
  }

  function handleActiveMediaChange(mediaId: string): void {
    activeMediaId = mediaId;
  }

  function handleEmbedStateChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      embedState = event.detail.value as MediaState;
    }
  }

  function handleBannerToneChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      bannerTone = event.detail.value as BannerTone;
    }
  }

  function enqueueToast(tone: ToastTone): void {
    toastSequence += 1;
    toastItems = [
      {
        id: `toast-${String(toastSequence).padStart(3, "0")}`,
        title:
          tone === "success"
            ? "Publish succeeded"
            : tone === "warning"
              ? "Review attention needed"
              : tone === "danger"
                ? "Export failed"
                : "Background refresh started",
        message:
          tone === "success"
            ? "The preview action completed without blocking the current surface."
            : tone === "warning"
              ? "A persistent banner may be more appropriate if the condition remains unresolved."
              : tone === "danger"
                ? "Transient failures still need a recoverable path and a persistent fallback when the problem persists."
                : "Transient notifications should confirm activity without stealing focus.",
        tone,
        actionLabel: tone === "danger" || tone === "warning" ? "Review" : null,
      },
      ...toastItems,
    ];
  }

  function dismissToast(event: CustomEvent<{ id: string }>): void {
    toastItems = toastItems.filter((item) => item.id !== event.detail.id);
  }

  function handleToastAction(event: CustomEvent<{ id: string }>): void {
    validationLog = `Toast action requested for "${event.detail.id}".`;
  }

  function openCommandPalette(): void {
    commandPaletteOpen = true;
  }

  function closeCommandPalette(): void {
    commandPaletteOpen = false;
  }

  function clearCommandDiscovery(): void {
    commandQuery = "";
    commandScope = "all";
    commandStateOverride = "auto";
  }

  function handleCommandScopeChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      commandScope = event.detail.value as CommandResultScope;
    }
  }

  function handleCommandStateChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      commandStateOverride = event.detail.value as "auto" | DiscoveryState;
    }
  }

  function reorderByValue<T extends { value: string }>(items: T[], orderedValues: string[]): T[] {
    const itemMap = new Map(items.map((item) => [item.value, item]));
    return orderedValues
      .map((value) => itemMap.get(value))
      .filter((item): item is T => item !== undefined);
  }

  function handleSurfaceTabChange(event: CustomEvent<{ value: string }>): void {
    workspaceSurfaceValue = event.detail.value;
    workspaceEventLog = `Activated workspace surface "${event.detail.value}".`;
  }

  function handleSurfaceReorder(event: CustomEvent<{ items: string[] }>): void {
    workspaceSurfaceItems = reorderByValue(workspaceSurfaceItems, event.detail.items);
    workspaceEventLog = `Reordered workspace surfaces to ${event.detail.items.join(" → ")}.`;
  }

  function handleSurfaceRename(event: CustomEvent<{ value: string }>): void {
    surfaceSequence += 1;
    workspaceSurfaceItems = workspaceSurfaceItems.map((item) =>
      item.value === event.detail.value ? { ...item, label: `Scratch ${surfaceSequence}` } : item,
    );
    workspaceEventLog = `Rename requested for "${event.detail.value}". The preview applied a host-side placeholder rename.`;
  }

  function handleSurfaceMove(event: CustomEvent<{ value: string }>): void {
    workspaceEventLog = `Move request emitted for workspace surface "${event.detail.value}". Cross-window orchestration remains host-owned.`;
  }

  function handleSurfaceClose(event: CustomEvent<{ value: string }>): void {
    if (workspaceSurfaceItems.length <= 1) {
      workspaceEventLog = "Close ignored because the preview keeps at least one workspace surface available.";
      return;
    }

    const currentIndex = workspaceSurfaceItems.findIndex((item) => item.value === event.detail.value);
    const nextItems = workspaceSurfaceItems.filter((item) => item.value !== event.detail.value);
    workspaceSurfaceItems = nextItems;

    if (workspaceSurfaceValue === event.detail.value) {
      const fallbackItem = nextItems[Math.max(0, currentIndex - 1)] ?? nextItems[0] ?? null;
      workspaceSurfaceValue = fallbackItem?.value ?? "mix-review";
    }

    workspaceEventLog = `Closed workspace surface "${event.detail.value}" and restored focus to "${workspaceSurfaceValue}".`;
  }

  function handleSurfaceAdd(): void {
    surfaceSequence += 1;
    const nextValue = `surface-${surfaceSequence}`;
    workspaceSurfaceItems = [
      ...workspaceSurfaceItems,
      {
        value: nextValue,
        label: `Scratch ${surfaceSequence}`,
        isClosable: true,
      },
    ];
    workspaceSurfaceValue = nextValue;
    workspaceEventLog = `Added workspace surface "${nextValue}" and made it active.`;
  }

  function handlePrimarySplitChange(event: CustomEvent<{ ratio: number }>): void {
    primarySplitRatio = event.detail.ratio;
    workspaceEventLog = `Primary split updated to ${Math.round(event.detail.ratio * 100)}%.`;
  }

  function handleSecondarySplitChange(event: CustomEvent<{ ratio: number }>): void {
    secondarySplitRatio = event.detail.ratio;
    workspaceEventLog = `Secondary split updated to ${Math.round(event.detail.ratio * 100)}%.`;
  }

  function handleDockValueChange(edge: "left" | "right", event: CustomEvent<{ value: string }>): void {
    if (edge === "left") {
      leftDockValue = event.detail.value;
    } else {
      rightDockValue = event.detail.value;
    }

    workspaceEventLog = `Activated ${edge} dock panel "${event.detail.value}".`;
  }

  function handleDockCollapsedChange(edge: "left" | "right", event: CustomEvent<{ collapsed: boolean }>): void {
    if (edge === "left") {
      leftDockCollapsed = event.detail.collapsed;
    } else {
      rightDockCollapsed = event.detail.collapsed;
    }

    workspaceEventLog = `${edge[0].toUpperCase()}${edge.slice(1)} dock ${event.detail.collapsed ? "collapsed" : "expanded"}.`;
  }

  function handleDockReorder(edge: "left" | "right", event: CustomEvent<{ items: string[] }>): void {
    if (edge === "left") {
      leftDockItems = reorderByValue(leftDockItems, event.detail.items);
    } else {
      rightDockItems = reorderByValue(rightDockItems, event.detail.items);
    }

    workspaceEventLog = `Reordered ${edge} dock panels to ${event.detail.items.join(" → ")}.`;
  }

  function handleDockClose(edge: "left" | "right", event: CustomEvent<{ value: string }>): void {
    const currentItems = edge === "left" ? leftDockItems : rightDockItems;
    const currentValue = edge === "left" ? leftDockValue : rightDockValue;
    const currentIndex = currentItems.findIndex((item) => item.value === event.detail.value);
    const nextItems = currentItems.filter((item) => item.value !== event.detail.value);
    const fallbackValue = nextItems[Math.max(0, currentIndex - 1)]?.value ?? nextItems[0]?.value ?? null;

    if (edge === "left") {
      leftDockItems = nextItems;
      leftDockValue = currentValue === event.detail.value ? fallbackValue : currentValue;
    } else {
      rightDockItems = nextItems;
      rightDockValue = currentValue === event.detail.value ? fallbackValue : currentValue;
    }

    workspaceEventLog = `Closed ${edge} dock panel "${event.detail.value}"${fallbackValue ? ` and restored "${fallbackValue}"` : ""}.`;
  }

  function handleDockContextMenu(edge: "left" | "right", event: CustomEvent<{ value: string | null }>): void {
    workspaceEventLog = `${edge[0].toUpperCase()}${edge.slice(1)} dock context menu requested for "${event.detail.value ?? "none"}".`;
  }

  function handleCommandSelect(event: CustomEvent<{ id: string }>): void {
    lastCommandId = event.detail.id;
    commandPaletteOpen = false;
    commandEventLog = `Command "${event.detail.id}" selected from the workstation palette.`;
    enqueueToast("success");
  }

  function handleActionDiscoverySelect(event: CustomEvent<{ id: string }>): void {
    handleCommandSelect(new CustomEvent("commandSelect", { detail: { id: event.detail.id } }));
  }

  function handleCommandPaletteQueryChange(event: CustomEvent<{ value: string }>): void {
    commandQuery = event.detail.value;
  }

  function handleWorkspaceStateChange(event: CustomEvent<{ value: string | string[] }>): void {
    if (typeof event.detail.value === "string") {
      workspaceState = event.detail.value as WorkspaceShellState;
    }
  }

  function handleInspectorQueryChange(event: CustomEvent<{ value: string }>): void {
    inspectorQuery = event.detail.value;
  }

  function handleInspectorQueryClear(): void {
    inspectorQuery = "";
  }

  function handlePreviewKeydown(event: KeyboardEvent): void {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      openCommandPalette();
    }
  }
</script>

<svelte:head>
  <title>Pug Docs Preview</title>
</svelte:head>

<svelte:window
  on:keydown={handlePreviewKeydown}
  on:hashchange={syncCurrentLocation}
  on:popstate={syncCurrentLocation}
/>

<div class="app-shell" data-appearance-treatment={appearanceTreatment} bind:this={appShell}>
  <aside class="control-rail">
    <div class="rail-header">
      <p class="eyebrow">Pug Docs Preview</p>
      <h1>Catalog and inspection surface</h1>
      <p class="rail-copy">
        The first docs-site baseline: live examples, package discoverability, token provenance, and accessibility-critical state posture rendered from the current artifact set.
      </p>
    </div>

    <section class="control-group" aria-labelledby="navigation-group">
      <div class="group-head">
        <h2 id="navigation-group">Preview navigation</h2>
        <p>Jump between the docs shell, shared demo screens, and token tools without one long scrolling catalog.</p>
      </div>
      <div class="rail-nav">
        <Select
          id="section-navigation-select"
          value={activeNavigationId}
          options={sectionNavigationOptions}
          ariaLabel="Preview destinations"
          on:valueChange={handleSectionNavigationChange}
        />
        <div class="nav-button-row">
          <Button
            variant="secondary"
            isDisabled={!previousSection}
            on:click={() => previousSection && selectSection(previousSection.id)}
          >
            Previous
          </Button>
          <Button
            variant="secondary"
            isDisabled={!nextSection}
            on:click={() => nextSection && selectSection(nextSection.id)}
          >
            Next
          </Button>
        </div>
      </div>
    </section>

    <Accordion
      items={railSectionItems}
      selectionMode="multiple"
      defaultValue={["display-controls"]}
      ariaLabel="Preview rail controls"
      let:item
    >
      {#if item.value === "display-controls"}
        <div class="rail-panel-body">
          <section class="control-group" aria-labelledby="theme-group">
            <div class="group-head">
              <h2 id="theme-group">Theme</h2>
              <p>Rendered from emitted theme overlays.</p>
            </div>
            <ToggleGroup
              value={theme}
              options={themeOptions}
              ariaLabel="Theme"
              on:valueChange={(event) => (theme = event.detail.value as ThemeName)}
            />
            <p class="control-caption">{themes[theme].description}</p>
          </section>

          <section class="control-group" aria-labelledby="density-group">
            <div class="group-head">
              <h2 id="density-group">Density</h2>
              <p>Shows layout compression against the same token baseline.</p>
            </div>
            <ToggleGroup
              value={density}
              options={densityOptions}
              ariaLabel="Density"
              on:valueChange={(event) => (density = event.detail.value as DensityName)}
            />
            <p class="control-caption">{densityModes[density].description}</p>
          </section>

          <section class="control-group" aria-labelledby="size-group">
            <div class="group-head">
              <h2 id="size-group">Control size</h2>
              <p>Exercises minimum control hit area across themes.</p>
            </div>
            <ToggleGroup
              value={controlSize}
              options={controlSizeOptions}
              ariaLabel="Control size"
              on:valueChange={(event) => (controlSize = event.detail.value as ControlSizeName)}
            />
            <p class="control-caption">{controlSizes[controlSize].description}</p>
          </section>

          <section class="control-group" aria-labelledby="treatment-group">
            <div class="group-head">
              <h2 id="treatment-group">Appearance treatment</h2>
              <p>Scoped recipe-level override across controls, cards, panels, and header framing.</p>
            </div>
            <ToggleGroup
              value={appearanceTreatment}
              options={appearanceTreatmentOptions}
              ariaLabel="Appearance treatment"
              on:valueChange={(event) =>
                (appearanceTreatment = event.detail.value as AppearanceTreatmentName)}
            />
            <p class="control-caption">
              {appearanceTreatmentEntries.find((entry) => entry.name === appearanceTreatment)?.description}
            </p>
          </section>
        </div>
      {:else if item.value === "state-probes"}
        <div class="rail-panel-body rail-panel-body--compact">
          <Checkbox
            isChecked={disabled}
            label="Disabled surfaces"
            on:checkedChange={handleDisabledChange}
          />
          <Checkbox
            isChecked={invalid}
            label="Invalid form state"
            on:checkedChange={handleInvalidChange}
          />
          <Checkbox
            isChecked={busy}
            label="Busy action state"
            on:checkedChange={handleBusyChange}
          />
        </div>
      {:else if item.value === "reference"}
        <div class="rail-panel-body">
          <section class="control-group" aria-labelledby="meta-group">
            <div class="group-head">
              <h2 id="meta-group">Artifact baseline</h2>
              <p>{manifest.canonicalFormat}</p>
            </div>
            <ul class="meta-list">
              <li>{manifest.requiredThemes.length} required themes</li>
              <li>{manifest.requiredDensityModes.length} density modes</li>
              <li>{manifest.requiredControlSizes.length} control sizes</li>
              <li>{aliases.length} bridge alias hooks</li>
            </ul>
          </section>

          <section class="control-group" aria-labelledby="entry-group">
            <div class="group-head">
              <h2 id="entry-group">Docs entry points</h2>
              <p>The preview now doubles as the first docs-site baseline.</p>
            </div>
            <div class="meta-stack">
              <div class="state-tile">
                <span class="token-path">bun run docs:dev</span>
                <strong>local docs surface</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">docs/contracts/</span>
                <strong>contract source of truth</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">packages/svelte/*</span>
                <strong>implementation packages</strong>
              </div>
            </div>
          </section>
        </div>
      {/if}
    </Accordion>
  </aside>

  <main class="preview-root" bind:this={previewRoot}>
    <section class="hero panel">
      <div class="hero-main">
        <p class="eyebrow">{heroEyebrow}</p>
        <h2>{heroTitle}</h2>
        <p class="hero-copy">{heroCopy}</p>
        <div class="hero-section-context" aria-label="active section context">
          <div class="hero-section-context__header">
            <span class="command-shortcut-hint">{activeNavigationEntry.eyebrow}</span>
            <strong>{activeNavigationEntry.title}</strong>
          </div>
          <p class="hero-section-context__summary">{activeNavigationEntry.summary}</p>
          <div class="hero-section-context__meta">
            <span class="token-path">{activeNavigationEntry.contractRoot}</span>
            <span class="command-shortcut-hint">{activeNavigationEntry.packageName}</span>
            {#each activeNavigationEntry.exampleTypes as exampleType}
              <span>{exampleType}</span>
            {/each}
          </div>
        </div>
      </div>
      <div class="hero-chips" aria-label="current modes">
        <Pill>{theme}</Pill>
        <Pill>{density}</Pill>
        <Pill>{controlSize}</Pill>
        <Pill>{appearanceTreatment}</Pill>
      </div>
    </section>

    {#if activeSectionId === "catalog-hub"}
      <DocsCatalogHub
        {docsFamilies}
        {docsAdoptionChecklist}
        {catalogEntries}
        {brandProofCards}
        onSelectSection={selectSection}
      />
    {:else if activeSectionId === "token-summary-section" || activeSectionId === "token-inspector"}
      <TokenToolsPanel
        activePanelId={activeSectionId}
        {keySemanticTokens}
        {filteredTokens}
        {matchingTokenCount}
        {inspectorQuery}
        onSelectPanel={(panelId) => selectSection(panelId as DocsSectionId)}
        onQueryChange={handleInspectorQueryChange}
        onQueryClear={handleInspectorQueryClear}
      />
    {:else if currentDemoScreen}
      <SharedDemoApp
        screenId={currentDemoScreen.id}
        screenTabs={sharedDemoScreenOptions}
        screenMeta={currentDemoScreen}
        {theme}
        {density}
        {controlSize}
        onScreenChange={handleDemoScreenChange}
        demo={{
          invalid,
          busy,
          disabled,
          projectTitle,
          assetSearch,
          titleError,
          titlePendingMessage,
          titleValidationState,
          searchValidationState,
          validationLog,
          handleTitleChange,
          handleSearchChange,
          handleSearchClear,
          handleSubmit,
          tableColumns,
          visibleRows,
          selectedRowIds,
          sortColumnId,
          sortDirection,
          handleSortChange,
          handleRowToggle,
          handleToggleAll,
          handleRowAction,
          tableStatus,
          currentPage,
          totalPages,
          sortedRows,
          pageSize,
          handlePageChange,
          bulkActions,
          handleBulkAction,
          clearTableSelection,
          browseQuery,
          browseStatus,
          browseStatusOptions,
          browseStateOptions,
          handleBrowseSearchChange,
          handleBrowseSearchClear,
          handleBrowseStatusChange,
          handleBrowseStateChange,
          listShellState,
          gridShellState,
          filteredBrowseRows,
          visibleBrowseRows,
          filteredBrowseCards,
          visibleBrowseCards,
          browseSummary,
          loadMoreBrowseRows,
          gridPage,
          gridTotalPages,
          resetBrowseFilters,
          retryBrowseState,
          detailBreadcrumbs,
          detailCards,
          detailState,
          handleDetailStateChange,
          handleBreadcrumbNavigate,
          relationItems,
          pickerQuery,
          handlePickerQueryChange,
          pickerVariant,
          pickerVariantOptions,
          handlePickerVariantChange,
          pickerMode,
          pickerModeOptions: selectionModeOptions,
          handlePickerModeChange,
          pickerState,
          pickerStateOptions,
          handlePickerStateChange,
          filteredRelationItems,
          selectedRelationIds,
          handleRelationSelectionChange,
          handlePickerConfirm,
          handlePickerCancel,
          pickerStatus,
          resetPickerState,
          mediaState,
          embedState,
          mediaStateOptions,
          handleMediaStateChange,
          handleEmbedStateChange,
          activeMedia,
          activeMediaId,
          secondaryMediaAssets,
          handleActiveMediaChange,
          getMediaStateTitle,
          getMediaStateMessage,
          getEmbedStateMessage,
          mediaStatus,
          embedStatus,
          bannerTone,
          bannerToneOptions: notificationToneOptions,
          handleBannerToneChange,
          showPersistentBanner,
          enqueueToast,
          toastItems,
          dismissToast,
          handleToastAction,
          notificationSummary,
          openCommandPalette,
          commandScope,
          commandScopeOptions,
          handleCommandScopeChange,
          commandStateOverride,
          commandStateOptions,
          handleCommandStateChange,
          clearCommandDiscovery,
          lastCommandId,
          filteredCommandActions,
          commandSections,
          commandStatus,
          commandEventLog,
          handleActionDiscoverySelect,
          workspaceState,
          workspaceStateOptions,
          handleWorkspaceStateChange,
          workspaceSurfaceItems,
          workspaceSurfaceValue,
          handleSurfaceTabChange,
          handleSurfaceReorder,
          handleSurfaceRename,
          handleSurfaceMove,
          handleSurfaceClose,
          handleSurfaceAdd,
          leftDockItems,
          leftDockValue,
          leftDockCollapsed,
          rightDockItems,
          rightDockValue,
          rightDockCollapsed,
          handleDockValueChange,
          handleDockCollapsedChange,
          handleDockReorder,
          handleDockClose,
          handleDockContextMenu,
          primarySplitRatio,
          secondarySplitRatio,
          handlePrimarySplitChange,
          handleSecondarySplitChange,
          activeWorkspaceSurface,
          activeWorkspaceSurfaceMeta,
          activeLeftDockMeta,
          activeRightDockMeta,
          workspacePersistenceSummary,
          serializedWorkspaceLayout,
          workspaceEventLog,
        }}
      />
    {/if}
  </main>
  <CommandPalette
    open={commandPaletteOpen}
    title="Workspace commands"
    description="Search across navigation, workspace, asset, and recent actions without leaving the current shell context."
    query={commandQuery}
    items={filteredCommandActions}
    state={commandPaletteState}
    invocationHint="Esc to close"
    on:queryChange={handleCommandPaletteQueryChange}
    on:commandSelect={handleCommandSelect}
    on:requestClose={closeCommandPalette}
  />
</div>
