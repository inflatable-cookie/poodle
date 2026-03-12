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
  const sectionNavigationOptions: SelectOption[] = sectionEntries.map((entry) => ({
    value: entry.id,
    label: entry.title,
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
  $: activeSectionIndex = Math.max(
    0,
    sectionEntries.findIndex((entry) => entry.id === activeSectionId),
  );
  $: activeSection = sectionEntries.find((entry) => entry.id === activeSectionId) ?? sectionEntries[0];
  $: previousSection = activeSectionIndex > 0 ? sectionEntries[activeSectionIndex - 1] : null;
  $: nextSection = activeSectionIndex < sectionEntries.length - 1 ? sectionEntries[activeSectionIndex + 1] : null;

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
        <h2 id="navigation-group">Section navigation</h2>
        <p>Move section by section instead of scrolling one long catalog column.</p>
      </div>
      <div class="rail-nav">
        <Select
          id="section-navigation-select"
          value={activeSectionId}
          options={sectionNavigationOptions}
          ariaLabel="Catalog sections"
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
        <p class="eyebrow">Docs-site baseline</p>
        <h2>First serious catalog surface for Pug</h2>
        <p class="hero-copy">
          This browser surface now groups tokens, contracts, Svelte packages, and stateful examples so adopters can inspect the shared system without hopping blind between markdown and code.
        </p>
        <div class="hero-section-context" aria-label="active section context">
          <div class="hero-section-context__header">
            <span class="command-shortcut-hint">{activeSection.eyebrow}</span>
            <strong>{activeSection.title}</strong>
          </div>
          <p class="hero-section-context__summary">{activeSection.summary}</p>
          <div class="hero-section-context__meta">
            <span class="token-path">{activeSection.contractRoot}</span>
            <span class="command-shortcut-hint">{activeSection.packageName}</span>
            {#each activeSection.exampleTypes as exampleType}
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
    <section id="catalog-hub" class="panel token-summary" aria-labelledby="catalog-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Catalog hub</p>
          <h2 id="catalog-heading">Information architecture, coverage, and adoption bar</h2>
        </div>
      </div>
      <div class="docs-overview-stack">
        <div class="docs-overview-top">
          <article class="demo-card docs-overview-card">
            <div class="card-header">
              <h3>Family directory</h3>
              <p>The first docs-site baseline groups examples by the same package and contract layers adopters will actually consume.</p>
            </div>
            <div class="docs-family-grid">
              {#each docsFamilies as family}
                <article class="docs-family-card">
                  <div class="docs-family-card__header">
                    <div>
                      <p class="eyebrow">{family.eyebrow}</p>
                      <h4>{family.title}</h4>
                    </div>
                    <span class="command-shortcut-hint">{family.packageName}</span>
                  </div>
                  <p class="detail-card-meta">{family.summary}</p>
                  <div class="docs-family-meta">
                    <span class="token-path">{family.contractRoot}</span>
                    <strong>{family.adoptionBar}</strong>
                  </div>
                  <div class="docs-link-row">
                    {#each family.sectionIds as sectionId}
                      {#if catalogEntryMap[sectionId]}
                        <Button
                          className="docs-link-chip"
                          variant="ghost"
                          size="sm"
                          on:click={() => selectSection(catalogEntryMap[sectionId].id)}
                        >
                          {catalogEntryMap[sectionId].title}
                        </Button>
                      {/if}
                    {/each}
                  </div>
                </article>
              {/each}
            </div>
          </article>

        <article class="demo-card docs-overview-card docs-overview-card--narrow">
          <div class="card-header">
            <h3>Adoption-ready minimum</h3>
            <p>`g02.012` freezes what must be visible before wider rollout, not just what exists somewhere in the repo.</p>
          </div>
            <div class="behavior-list">
              {#each docsAdoptionChecklist as item}
                <div class="behavior-item">
                  <strong>Required</strong>
                  <p>{item}</p>
                </div>
              {/each}
            </div>
          </article>

          <article class="demo-card docs-overview-card docs-overview-card--narrow">
            <div class="card-header">
              <h3>Disclosure primitives</h3>
              <p>The preview shell now uses real disclosure primitives instead of ad hoc details styling.</p>
            </div>
            <div class="demo-stack">
              <Accordion
                items={[
                  { value: "accordion-foundation", label: "Accordion", description: "Grouped disclosure for repeated docs or settings sections." },
                  { value: "accordion-boundary", label: "Boundary", description: "Use grouped disclosure only when repeated sections are the real semantic pattern." },
                ]}
                defaultValue="accordion-foundation"
                ariaLabel="Disclosure primitive example"
                let:item
              >
                <p class="detail-card-meta">
                  {item.value === "accordion-foundation"
                    ? "Foundation-safe grouped disclosure now exists for more web-oriented products and docs surfaces."
                    : "Single-block reveal belongs to Collapsible; grouped disclosure belongs to Accordion."}
                </p>
              </Accordion>

              <Collapsible
                title="Collapsible"
                description="Single revealable content block for compact notes, diagnostics, or settings groups."
                defaultOpen={true}
              >
                <p class="detail-card-meta">
                  This surface owns one trigger and one revealable region without pretending to be grouped navigation.
                </p>
              </Collapsible>
            </div>
          </article>
        </div>

        <article class="demo-card docs-overview-card">
          <div class="card-header">
            <h3>Example directory</h3>
            <p>Every section below states which contract layer it belongs to, which package owns the implementation, and what example types it covers.</p>
          </div>
          <div class="docs-section-list">
            {#each catalogEntries as entry}
              <Toggle
                className="docs-section-card"
                isPressed={activeSectionId === entry.id}
                layout="stack"
                variant="ghost"
                ariaLabel={`Open ${entry.title}`}
                on:pressedChange={() => selectSection(entry.id)}
              >
                <div>
                  <p class="eyebrow">{entry.eyebrow}</p>
                  <strong>{entry.title}</strong>
                </div>
                <p>{entry.summary}</p>
                <div class="docs-section-meta">
                  <span class="token-path">{entry.contractRoot}</span>
                  <span class="command-shortcut-hint">{entry.packageName}</span>
                </div>
                <div class="docs-tag-row">
                  {#each entry.exampleTypes as exampleType}
                    <Pill appearance="subtle">{exampleType}</Pill>
                  {/each}
                </div>
              </Toggle>
            {/each}
          </div>
        </article>

        <article class="demo-card docs-overview-card">
          <div class="card-header">
            <h3>Scoped brand proof</h3>
            <p>The same Pug components can sit inside a more expressive website wrapper through scoped recipe variables instead of token redefinition.</p>
          </div>
          <div class="brand-proof-scope">
            <PageHeader
              title="Make room for brand styling without rebuilding the system"
              eyebrow="Website-style wrapper"
              subtitle="This proof uses app-owned composition plus scoped appearance recipes so cards, header framing, and CTA chrome can shift together."
            >
              <div slot="actions" class="action-cluster brand-proof-actions">
                <Button variant="secondary">Read pattern notes</Button>
                <Button variant="primary">Launch branded preview</Button>
              </div>
            </PageHeader>

            <div class="brand-proof-grid">
              {#each brandProofCards as card}
                <Card variant={card.variant}>
                  <div slot="header">
                    <p class="eyebrow">{card.eyebrow}</p>
                  </div>
                  <strong class="detail-card-value">{card.title}</strong>
                  <p class="detail-card-meta">{card.summary}</p>
                </Card>
              {/each}
            </div>
          </div>
        </article>
      </div>
    </section>
    {/if}

    {#if activeSectionId === "form-suite"}
    <section id="form-suite" class="panel token-summary" aria-labelledby="form-suite-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Foundation examples</p>
          <h2 id="form-suite-heading">Form baseline and validation posture</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-primitives</span>
        <span class="token-path">docs/contracts/foundation/field.md</span>
        <span class="token-path">docs/contracts/foundation/text-input.md</span>
        <span class="token-path">docs/contracts/foundation/search-field.md</span>
      </div>
      <div class="demo-grid" aria-label="form system baseline preview">
      <article class="demo-card">
        <div class="card-header">
          <h3>Form system baseline</h3>
          <p>First contract-backed Svelte field wrapper, text input, search field, and action row.</p>
        </div>
        <form class="demo-form" on:submit={handleSubmit}>
          <Field
            id="project-title"
            label="Project title"
            description="Used for handoff labels, activity logs, and downstream validation review."
            error={titleError}
            pendingMessage={titlePendingMessage}
            validationState={titleValidationState as ValidationState}
            isRequired={true}
            let:describedBy
            let:validationState
          >
            <TextInput
              id="project-title"
              value={projectTitle}
              placeholder="Enter project title"
              isDisabled={disabled}
              validationState={validationState}
              describedBy={describedBy}
              on:valueChange={handleTitleChange}
            >
              <span slot="leading" aria-hidden="true">Aa</span>
              <span slot="trailing" class="field-shortcut" aria-hidden="true">⌘K</span>
            </TextInput>
          </Field>

          <Field
            id="asset-search"
            label="Asset search"
            description="Search stays native on the Svelte side while clear and pending semantics remain explicit."
            pendingMessage={busy ? "Refreshing indexed asset results..." : null}
            validationState={searchValidationState as ValidationState}
            let:describedBy
            let:validationState
          >
            <SearchField
              id="asset-search"
              value={assetSearch}
              isDisabled={disabled}
              validationState={validationState}
              describedBy={describedBy}
              on:valueChange={handleSearchChange}
              on:clear={handleSearchClear}
            />
          </Field>

          <FormActions align="between">
            <p class="demo-status">{validationLog}</p>
            <div class="action-cluster">
              <Button variant="secondary" isDisabled={disabled}>
                Cancel
              </Button>
              <Button variant="primary" type="submit" isDisabled={disabled} isLoading={busy}>
                {busy ? "Validating..." : "Save changes"}
              </Button>
            </div>
          </FormActions>
        </form>
      </article>

      <article class="demo-card">
        <div class="card-header">
          <h3>Behavior probe</h3>
          <p>The preview state toggles drive visible form semantics instead of purely cosmetic examples.</p>
        </div>
        <div class="demo-stack behavior-list">
          <div class="behavior-item">
            <strong>Invalid</strong>
            <p>Error copy is attached by field wrapper IDs rather than buried in placeholder text.</p>
          </div>
          <div class="behavior-item">
            <strong>Pending</strong>
            <p>Pending state stays visible in field messaging and action-row status so announcement rules are testable.</p>
          </div>
          <div class="behavior-item">
            <strong>Disabled</strong>
            <p>Input reachability, clear behavior, and action buttons all collapse together when the disabled toggle is active.</p>
          </div>
        </div>
      </article>

      <article class="demo-card">
        <div class="card-header">
          <h3>Current form state</h3>
          <p>Live values and validation posture from the implemented Svelte primitives.</p>
        </div>
        <div class="demo-stack">
          <div class="state-tile">
            <span class="token-path">projectTitle</span>
            <strong>{projectTitle || "∅"}</strong>
          </div>
          <div class="state-tile">
            <span class="token-path">assetSearch</span>
            <strong>{assetSearch || "∅"}</strong>
          </div>
          <div class="state-tile">
            <span class="token-path">titleValidationState</span>
            <strong>{titleValidationState}</strong>
          </div>
          <div class="state-tile">
            <span class="token-path">searchValidationState</span>
            <strong>{searchValidationState}</strong>
          </div>
        </div>
      </article>
      </div>
    </section>
    {/if}

    {#if activeSectionId === "table-suite"}
    <section id="table-suite" class="panel token-summary" aria-labelledby="table-suite-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Data table suite</p>
          <h2 id="table-suite-heading">Selection, sorting, and bulk actions</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-composites</span>
        <span class="token-path">docs/contracts/composites/data-table.md</span>
        <span class="token-path">docs/contracts/composites/bulk-action-bar.md</span>
      </div>
      <div class="table-toolbar">
        <div class="table-toolbar__search">
          <SearchField
            id="table-search"
            value={assetSearch}
            ariaLabel="Filter visible rows"
            on:valueChange={handleSearchChange}
            on:clear={handleSearchClear}
          />
        </div>
        <p class="table-toolbar__summary">
          {filteredRows.length} matching rows, {selectedRowIds.length} selected, virtualization remains a documented future concern.
        </p>
      </div>
      {#if selectedRowIds.length > 0}
        <BulkActionBar
          selectionCount={selectedRowIds.length}
          totalCount={visibleRows.length}
          actions={bulkActions}
          on:action={handleBulkAction}
          on:clear={clearTableSelection}
        />
      {/if}
      <div class="table-stack">
        <DataTable
          ariaLabel="Mix tasks"
          columns={tableColumns}
          rows={visibleRows}
          {selectedRowIds}
          {sortColumnId}
          {sortDirection}
          on:sortChange={handleSortChange}
          on:rowToggle={handleRowToggle}
          on:toggleAll={handleToggleAll}
          on:rowAction={handleRowAction}
        />
        <PaginationSummary
          {currentPage}
          {totalPages}
          totalItems={sortedRows.length}
          {pageSize}
          on:pageChange={handlePageChange}
        />
        <p class="demo-status">{tableStatus}</p>
      </div>
    </section>
    {/if}

    {#if activeSectionId === "browse-suite"}
    <section id="browse-suite" class="panel token-summary" aria-labelledby="browse-suite-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Browse shells</p>
          <h2 id="browse-suite-heading">Lists, grids, filters, and search depth</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-composites</span>
        <span class="token-path">docs/contracts/composites/list-shell.md</span>
        <span class="token-path">docs/contracts/composites/grid-shell.md</span>
        <span class="token-path">docs/contracts/composites/filter-toolbar.md</span>
      </div>
      <div class="browse-controls">
        <FilterToolbar
          ariaLabel="Browse controls"
          summaryText={browseSummary}
        >
          <div class="browse-search">
            <SearchField
              id="browse-search"
              value={browseQuery}
              ariaLabel="Search browse results"
              on:valueChange={handleBrowseSearchChange}
              on:clear={handleBrowseSearchClear}
            />
          </div>
          <ToggleGroup
            value={browseStatus}
            options={browseStatusOptions}
            ariaLabel="Browse status"
            on:valueChange={handleBrowseStatusChange}
          />
          <svelte:fragment slot="secondary">
            <ToggleGroup
              value={browseStateOverride}
              options={browseStateOptions}
              ariaLabel="Browse state override"
              on:valueChange={handleBrowseStateChange}
            />
          </svelte:fragment>
        </FilterToolbar>
      </div>
      <div class="browse-shell-grid">
        <div class="browse-column">
          <h3 class="browse-column__title">List shell with progressive loading</h3>
          <ListShell
            ariaLabel="Browse list"
            state={listShellState}
            itemCount={filteredBrowseRows.length}
            stateTitle={listShellState === "no-results" ? "No matching list results" : listShellState === "loading" ? "Loading list results" : listShellState === "error" ? "List unavailable" : "No list content"}
            stateMessage={listShellState === "no-results" ? "Try clearing the search query or widening the status filter." : listShellState === "loading" ? "The host owns the async fetch policy; the shell only owns posture." : listShellState === "error" ? "Error remediation remains host-owned, but the shell makes the state legible." : "Empty collections stay distinct from no-results states."}
          >
            <div slot="header" class="browse-header-note">
              Progressive loading can append more rows without switching the shell to pagination.
            </div>
            <div slot="state" class="state-stack">
              {#if listShellState === "loading"}
                <Banner
                  tone="info"
                  title="Loading list results"
                  message="Loading posture stays explicit while result fetching remains host-owned."
                />
                <div class="state-skeleton-list" aria-hidden="true">
                  {#each Array.from({ length: 4 }) as _}
                    <div class="state-skeleton-row">
                      <Skeleton shape="circle" width="1.125rem" height="1.125rem" />
                      <div class="state-skeleton-copy">
                        <Skeleton width="58%" />
                        <Skeleton width="34%" />
                      </div>
                    </div>
                  {/each}
                </div>
              {:else if listShellState === "error"}
                <Banner
                  tone="danger"
                  title="List unavailable"
                  message="Persistent errors need remediation action, not just a textual state string."
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary" on:click={retryBrowseState}>Retry</Button>
                  </div>
                </Banner>
                <EmptyState
                  title="Browse results could not be loaded"
                  message="The shell keeps structure and action placement stable while the host decides how to recover."
                />
              {:else if listShellState === "no-results"}
                <EmptyState
                  title="No list results match the current filters"
                  message="No-results stays distinct from a genuinely empty collection."
                  variant="search"
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary" on:click={resetBrowseFilters}>Clear filters</Button>
                  </div>
                </EmptyState>
              {:else}
                <EmptyState
                  title="No list content available yet"
                  message="First-run and truly empty collections should not masquerade as search failures."
                />
              {/if}
            </div>
            {#each visibleBrowseRows as row}
              <li class="browse-row">
                <div>
                  <strong>{row.title}</strong>
                  <p>{row.kind} owned by {row.owner}</p>
                </div>
                <span class={`status-chip status-chip--${row.status.toLowerCase()}`}>{row.status}</span>
              </li>
            {/each}
            <svelte:fragment slot="footer">
              {#if listShellState === "ready" && visibleBrowseRows.length < filteredBrowseRows.length}
                <Button variant="secondary" on:click={loadMoreBrowseRows}>
                  Load more results
                </Button>
              {/if}
            </svelte:fragment>
          </ListShell>
        </div>
        <div class="browse-column">
          <h3 class="browse-column__title">Grid shell with pagination</h3>
          <GridShell
            ariaLabel="Browse grid"
            state={gridShellState}
            itemCount={filteredBrowseCards.length}
            minColumnWidth="sm"
            stateTitle={gridShellState === "no-results" ? "No matching grid results" : gridShellState === "loading" ? "Loading grid results" : gridShellState === "error" ? "Grid unavailable" : "No grid content"}
            stateMessage={gridShellState === "no-results" ? "Search and filter composition is host-owned, but the shell keeps the no-results posture explicit." : gridShellState === "loading" ? "Pagination and progressive loading remain separate postures in this baseline." : gridShellState === "error" ? "Retry or remediation actions belong to the host screen." : "Empty collections remain distinct from query-driven no-results."}
          >
            <div slot="header" class="browse-header-note">
              Pagination works well for stable result sets where range summary matters more than incremental append.
            </div>
            <div slot="state" class="state-stack">
              {#if gridShellState === "loading"}
                <Banner
                  tone="info"
                  title="Loading grid results"
                  message="Decorative skeletons can reserve layout without claiming to be real content."
                />
                <div class="state-skeleton-grid" aria-hidden="true">
                  {#each Array.from({ length: 4 }) as _}
                    <div class="state-skeleton-card">
                      <Skeleton shape="block" height="7.5rem" />
                      <Skeleton width="62%" />
                      <Skeleton width="40%" />
                    </div>
                  {/each}
                </div>
              {:else if gridShellState === "error"}
                <Banner
                  tone="danger"
                  title="Grid results unavailable"
                  message="Retry and support actions should stay adjacent to the failed surface."
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary" on:click={retryBrowseState}>Retry</Button>
                  </div>
                </Banner>
                <EmptyState
                  title="Card gallery could not be loaded"
                  message="Error states need clear recovery affordances and should remain visually distinct from empty states."
                />
              {:else if gridShellState === "no-results"}
                <EmptyState
                  title="No cards match the current query"
                  message="Clear filters or widen the search scope to restore results."
                  variant="search"
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary" on:click={resetBrowseFilters}>Clear filters</Button>
                  </div>
                </EmptyState>
              {:else}
                <EmptyState
                  title="No cards available yet"
                  message="A genuinely empty destination should present the next step, not read like a failed fetch."
                />
              {/if}
            </div>
            {#each visibleBrowseCards as card}
              <article class="browse-card">
                <p class="eyebrow">{card.category}</p>
                <h4>{card.title}</h4>
                <p>{card.meta}</p>
              </article>
            {/each}
            <svelte:fragment slot="footer">
              {#if gridShellState === "ready"}
                <PaginationSummary
                  currentPage={gridPage}
                  totalPages={gridTotalPages}
                  totalItems={filteredBrowseCards.length}
                  pageSize={gridPageSize}
                  on:pageChange={handleGridPageChange}
                />
              {/if}
            </svelte:fragment>
          </GridShell>
        </div>
      </div>
    </section>
    {/if}

    {#if activeSectionId === "detail-suite"}
    <section id="detail-suite" class="panel token-summary" aria-labelledby="detail-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Detail display suite</p>
          <h2 id="detail-heading">Cards, headers, breadcrumbs, and summary/detail composition</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-composites</span>
        <span class="token-path">docs/contracts/composites/detail-shell.md</span>
        <span class="token-path">docs/contracts/composites/page-header.md</span>
        <span class="token-path">docs/contracts/composites/breadcrumbs.md</span>
      </div>
      <div class="detail-controls">
        <ToggleGroup
          value={detailState}
          options={detailStateOptions}
          ariaLabel="Detail state"
          on:valueChange={handleDetailStateChange}
        />
      </div>
      <DetailShell
        ariaLabel="Mix review detail"
        state={detailState}
        stateTitle={detailState === "loading" ? "Loading detail surface" : detailState === "error" ? "Detail unavailable" : "No detail content"}
        stateMessage={detailState === "loading" ? "Detail shells keep header identity distinct while body content loads." : detailState === "error" ? "Error remediation remains host-owned while the shell preserves region structure." : "Empty detail destinations remain distinct from browse no-results states."}
      >
        <PageHeader
          slot="header"
          title="Aura review delivery"
          eyebrow="Detail surface"
          subtitle="Local identity, breadcrumb context, and summary actions composed above readonly detail sections."
        >
          <Breadcrumbs
            slot="breadcrumbs"
            items={detailBreadcrumbs}
            on:navigate={handleBreadcrumbNavigate}
          />
          <div slot="actions" class="action-cluster">
            <Button variant="secondary">Share</Button>
            <Button variant="primary">Approve</Button>
          </div>
        </PageHeader>
        <div slot="state" class="state-stack">
          {#if detailState === "loading"}
            <Banner
              tone="info"
              title="Loading detail surface"
              message="Header identity and action placement should remain stable while body sections resolve."
            />
            <div class="detail-loading-grid" aria-hidden="true">
              <Skeleton shape="block" height="5.25rem" />
              <Skeleton shape="block" height="5.25rem" />
              <Skeleton shape="block" height="5.25rem" />
            </div>
          {:else if detailState === "error"}
            <Banner
              tone="danger"
              title="Detail surface unavailable"
              message="Persistent failures need explicit retry or fallback actions near the affected surface."
            >
              <div slot="actions" class="action-cluster">
                <Button variant="secondary" on:click={() => (detailState = "ready")}>Retry</Button>
              </div>
            </Banner>
            <EmptyState
              title="This detail record could not be displayed"
              message="The shell still preserves hierarchy and remediation placement even when data retrieval fails."
            />
          {:else}
            <EmptyState
              title="No detail record selected"
              message="Empty detail destinations should offer a calm, explanatory posture instead of looking broken."
            />
          {/if}
        </div>

        <div class="detail-card-grid">
          {#each detailCards as card}
            <Card variant={card.id === "health" ? "elevated" : "outlined"}>
              <div slot="header">
                <p class="eyebrow">{card.title}</p>
              </div>
              <strong class="detail-card-value">{card.value}</strong>
              <p class="detail-card-meta">{card.meta}</p>
            </Card>
          {/each}
        </div>

        <DetailSection
          title="Delivery metadata"
          description="Readonly rows emphasize label/value semantics instead of form editing posture."
        >
          <dl class="detail-list">
            <DetailRow label="Sample rate" value="48 kHz" />
            <DetailRow label="Loudness target" value="-16 LUFS integrated" />
            <DetailRow
              label="Destination"
              value="/clients/aura/review/v4/final-deliverables"
              truncateValue={true}
            >
              <Button slot="action" variant="secondary">Reveal</Button>
            </DetailRow>
          </dl>
        </DetailSection>

        <DetailSection
          title="Checklist"
          description="Cards and detail rows can mix inside the same detail shell without collapsing hierarchy."
          isSeparated={true}
        >
          <div class="detail-inline-cards">
            <Card variant="outlined">
              <div slot="header">
                <h4 class="mini-card-title">Mix notes</h4>
              </div>
              <p class="detail-card-meta">Lead vocal automation cleaned, sibilance pass approved, limiter margin preserved.</p>
            </Card>
            <Card variant="outlined">
              <div slot="header">
                <h4 class="mini-card-title">Review notes</h4>
              </div>
              <p class="detail-card-meta">Broadcast compliance still needs one final offline bounce confirmation.</p>
            </Card>
          </div>
        </DetailSection>
      </DetailShell>
    </section>
    {/if}

    {#if activeSectionId === "picker-suite"}
    <section id="picker-suite" class="panel token-summary" aria-labelledby="picker-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Picker workflows</p>
          <h2 id="picker-heading">Relation and selection flows</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-composites</span>
        <span class="token-path">docs/contracts/composites/picker-shell.md</span>
        <span class="token-path">docs/contracts/composites/relation-picker.md</span>
        <span class="token-path">docs/contracts/composites/selection-summary.md</span>
      </div>
      <div class="picker-controls">
        <div class="picker-control-group">
          <span class="token-path">Variant</span>
          <ToggleGroup
            value={pickerVariant}
            options={pickerVariantOptions}
            ariaLabel="Picker variant"
            on:valueChange={handlePickerVariantChange}
          />
        </div>
        <div class="picker-control-group">
          <span class="token-path">Selection mode</span>
          <ToggleGroup
            value={pickerMode}
            options={selectionModeOptions}
            ariaLabel="Picker selection mode"
            on:valueChange={handlePickerModeChange}
          />
        </div>
        <div class="picker-control-group">
          <span class="token-path">State</span>
          <ToggleGroup
            value={pickerStateOverride}
            options={pickerStateOptions}
            ariaLabel="Picker state"
            on:valueChange={handlePickerStateChange}
          />
        </div>
      </div>
      <div class="picker-demo-grid">
        <div class="picker-demo-column">
          <RelationPicker
            title="Attach related assets"
            description="One shared picker shell supports inline, popover-style, and modal-style relation workflows."
            items={relationItems}
            selectedIds={selectedRelationIds}
            query={pickerQuery}
            selectionMode={pickerMode}
            variant={pickerVariant}
            state={pickerState}
            on:queryChange={handlePickerQueryChange}
            on:selectionChange={handleRelationSelectionChange}
            on:confirm={handlePickerConfirm}
            on:cancel={handlePickerCancel}
          >
            <div slot="state" class="state-stack">
              {#if pickerState === "loading"}
                <Banner
                  tone="info"
                  title="Loading picker candidates"
                  message="Selection summary and confirm posture remain stable while candidates load."
                />
                <div class="state-skeleton-list" aria-hidden="true">
                  {#each Array.from({ length: 3 }) as _}
                    <div class="state-skeleton-row">
                      <Skeleton shape="circle" width="1.125rem" height="1.125rem" />
                      <div class="state-skeleton-copy">
                        <Skeleton width="56%" />
                        <Skeleton width="28%" />
                      </div>
                    </div>
                  {/each}
                </div>
              {:else if pickerState === "error"}
                <Banner
                  tone="danger"
                  title="Picker unavailable"
                  message="Error handling remains host-owned, but retry and escape routes need to stay visible."
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary" on:click={resetPickerState}>Reset</Button>
                  </div>
                </Banner>
                <EmptyState
                  title="Candidates could not be loaded"
                  message="Keep selection context and remediation reachability visible instead of collapsing the picker."
                />
              {:else if pickerState === "no-results"}
                <EmptyState
                  title="No candidates match the current query"
                  message="No-results should offer a fast way back to the full candidate set."
                  variant="search"
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary" on:click={resetPickerState}>Clear search</Button>
                  </div>
                </EmptyState>
              {:else}
                <EmptyState
                  title="No candidates available"
                  message="Empty relation states remain distinct from failed fetches and filtered no-results."
                />
              {/if}
            </div>
          </RelationPicker>
        </div>
        <div class="picker-demo-column">
          <Card variant="outlined">
            <div slot="header">
              <p class="eyebrow">Workflow notes</p>
            </div>
            <div class="picker-state-stack">
              <div class="state-tile">
                <span class="token-path">pickerState</span>
                <strong>{pickerState}</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">selectedRelationIds</span>
                <strong>{selectedRelationIds.length === 0 ? "∅" : selectedRelationIds.join(", ")}</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">filteredRelationItems</span>
                <strong>{filteredRelationItems.length}</strong>
              </div>
            </div>
            <p class="detail-card-meta">{pickerStatus}</p>
          </Card>
        </div>
      </div>
    </section>
    {/if}

    {#if activeSectionId === "media-suite"}
    <section id="media-suite" class="panel token-summary" aria-labelledby="media-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Media and asset suite</p>
          <h2 id="media-heading">Preview framing, embeds, and fallback posture</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-composites</span>
        <span class="token-path">docs/contracts/composites/media-preview.md</span>
        <span class="token-path">docs/contracts/composites/embed-shell.md</span>
      </div>
      <div class="media-controls">
        <div class="picker-control-group">
          <span class="token-path">Preview state</span>
          <ToggleGroup
            value={mediaState}
            options={mediaStateOptions}
            ariaLabel="Media preview state"
            on:valueChange={handleMediaStateChange}
          />
        </div>
        <div class="picker-control-group">
          <span class="token-path">Embed state</span>
          <ToggleGroup
            value={embedState}
            options={mediaStateOptions}
            ariaLabel="Embed state"
            on:valueChange={handleEmbedStateChange}
          />
        </div>
      </div>
      <div class="media-demo-grid">
        <div class="media-main-column">
          <div class="media-strip" role="tablist" aria-label="Asset previews">
            {#each mediaAssets as asset}
              <Toggle
                className="media-strip__item"
                isPressed={activeMediaId === asset.id}
                layout="stack"
                variant="ghost"
                ariaLabel={`Show ${asset.title}`}
                on:pressedChange={() => (activeMediaId = asset.id)}
              >
                <MediaThumbnail
                  kind={asset.kind}
                  state={mediaState}
                  aspectRatio={asset.aspectRatio}
                  presentation="compact"
                  title={asset.title}
                  badge={asset.badge}
                  meta={asset.thumbnailMeta}
                  stateTitle={getMediaStateTitle(mediaState, asset.kind)}
                  stateMessage={getMediaStateMessage(mediaState, asset.kind)}
                >
                  {#if asset.kind === "image"}
                    <div class="mock-media mock-media--image" aria-hidden="true">
                      <div class="mock-media__panel"></div>
                      <div class="mock-media__panel"></div>
                    </div>
                  {:else if asset.kind === "audio"}
                    <div class="mock-media mock-media--audio" aria-hidden="true">
                      <div class="mock-waveform">
                        {#each Array.from({ length: 16 }) as _, index}
                          <span style={`height: ${remHeight(18 + ((index % 5) * 10))};`}></span>
                        {/each}
                      </div>
                    </div>
                  {:else if asset.kind === "video"}
                    <div class="mock-media mock-media--video" aria-hidden="true">
                      <div class="mock-video__screen"></div>
                      <div class="mock-video__timeline"></div>
                    </div>
                  {:else if asset.kind === "document"}
                    <div class="mock-media mock-media--document" aria-hidden="true">
                      <div class="mock-document">
                        <span></span>
                        <span></span>
                        <span></span>
                      </div>
                    </div>
                  {/if}
                </MediaThumbnail>
              </Toggle>
            {/each}
          </div>

          <MediaPreview
            title={activeMedia.title}
            description={activeMedia.description}
            eyebrow={activeMedia.eyebrow}
            caption={activeMedia.caption}
            meta={activeMedia.meta}
            badge={activeMedia.badge}
            thumbnailMeta={activeMedia.thumbnailMeta}
            kind={activeMedia.kind}
            state={mediaState}
            aspectRatio={activeMedia.aspectRatio}
            stateTitle={getMediaStateTitle(mediaState, activeMedia.kind)}
            stateMessage={getMediaStateMessage(mediaState, activeMedia.kind)}
            variant="elevated"
          >
            <svelte:fragment slot="media">
              {#if activeMedia.kind === "image"}
                <div class="mock-media mock-media--image" aria-hidden="true">
                  <div class="mock-media__panel"></div>
                  <div class="mock-media__panel"></div>
                </div>
              {:else if activeMedia.kind === "audio"}
                <div class="mock-media mock-media--audio" aria-hidden="true">
                  <div class="mock-waveform">
                    {#each Array.from({ length: 24 }) as _, index}
                      <span style={`height: ${remHeight(18 + ((index % 7) * 10))};`}></span>
                    {/each}
                  </div>
                </div>
              {:else if activeMedia.kind === "video"}
                <div class="mock-media mock-media--video" aria-hidden="true">
                  <div class="mock-video__screen"></div>
                  <div class="mock-video__timeline"></div>
                </div>
              {:else if activeMedia.kind === "document"}
                <div class="mock-media mock-media--document" aria-hidden="true">
                  <div class="mock-document mock-document--large">
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                  </div>
                </div>
              {/if}
            </svelte:fragment>
            <div class="media-preview__notes">
              <div class="state-tile">
                <span class="token-path">assetId</span>
                <strong>{activeMedia.assetId}</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">kind</span>
                <strong>{activeMedia.kind}</strong>
              </div>
            </div>
            <div slot="footer" class="media-preview-footer">
              <span class="token-path">{activeMedia.assetId}</span>
              <div class="action-cluster">
                <Button variant="secondary" isDisabled={disabled}>Open source</Button>
                <Button variant="primary" isDisabled={disabled}>Attach asset</Button>
              </div>
            </div>
          </MediaPreview>

          <div class="media-secondary-grid">
            {#each secondaryMediaAssets as asset}
              <MediaPreview
                title={asset.title}
                description={asset.description}
                eyebrow={asset.eyebrow}
                caption={asset.caption}
                meta={asset.meta}
                badge={asset.badge}
                thumbnailMeta={asset.thumbnailMeta}
                kind={asset.kind}
                state={mediaState}
                aspectRatio={asset.aspectRatio}
                stateTitle={getMediaStateTitle(mediaState, asset.kind)}
                stateMessage={getMediaStateMessage(mediaState, asset.kind)}
                variant="outlined"
              >
                <svelte:fragment slot="media">
                  {#if asset.kind === "audio"}
                    <div class="mock-media mock-media--audio" aria-hidden="true">
                      <div class="mock-waveform">
                        {#each Array.from({ length: 18 }) as _, index}
                          <span style={`height: ${remHeight(18 + ((index % 6) * 8))};`}></span>
                        {/each}
                      </div>
                    </div>
                  {:else if asset.kind === "video"}
                    <div class="mock-media mock-media--video" aria-hidden="true">
                      <div class="mock-video__screen"></div>
                      <div class="mock-video__timeline"></div>
                    </div>
                  {:else if asset.kind === "document"}
                    <div class="mock-media mock-media--document" aria-hidden="true">
                      <div class="mock-document">
                        <span></span>
                        <span></span>
                        <span></span>
                      </div>
                    </div>
                  {:else}
                    <div class="mock-media mock-media--image" aria-hidden="true">
                      <div class="mock-media__panel"></div>
                      <div class="mock-media__panel"></div>
                    </div>
                  {/if}
                </svelte:fragment>
              </MediaPreview>
            {/each}
          </div>
        </div>

        <div class="media-sidebar">
          <EmbedShell
            title="External review embed"
            description="Embed shells frame host-native or external surfaces while preserving fallback copy and recovery actions."
            provider="Bridge viewer"
            state={embedState}
            stateTitle={embedState === "loading" ? "Loading review embed" : embedState === "error" ? "Review embed unavailable" : "No embed target"}
            stateMessage={getEmbedStateMessage(embedState)}
          >
            <div slot="state" class="state-stack">
              {#if embedState === "loading"}
                <Banner
                  tone="info"
                  title="Loading review embed"
                  message="The framed embed region stays stable while host-native or external content initializes."
                />
                <Skeleton shape="block" height="16.25rem" />
              {:else if embedState === "error"}
                <Banner
                  tone="danger"
                  title="Review embed unavailable"
                  message="A failed embed still needs visible fallback actions and preserved context."
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary" on:click={() => (embedState = "ready")}>Retry</Button>
                  </div>
                </Banner>
                <EmptyState
                  title="Open this review in the host instead"
                  message="Inline embedding is optional; recovery posture is not."
                >
                  <div slot="actions" class="action-cluster">
                    <Button variant="secondary">Open external</Button>
                  </div>
                </EmptyState>
              {:else}
                <EmptyState
                  title="No embedded destination configured"
                  message="Some workflows intentionally prefer a host-owned open action over inline rendering."
                />
              {/if}
            </div>
            <div class="mock-embed" aria-hidden="true">
              <div class="mock-embed__header">
                <span></span>
                <span></span>
                <span></span>
              </div>
              <div class="mock-embed__body">
                <div class="mock-embed__sidebar"></div>
                <div class="mock-embed__canvas"></div>
              </div>
            </div>
            <div slot="footer" class="media-preview-footer">
              <span class="token-path">embed.review.pass-4</span>
              <div class="action-cluster">
                <Button variant="secondary" isDisabled={disabled}>Open external</Button>
                <Button variant="primary" isDisabled={disabled || embedState !== "ready"}>
                  Focus embed
                </Button>
              </div>
            </div>
          </EmbedShell>

          <Card variant="outlined">
            <div slot="header">
              <p class="eyebrow">Current media posture</p>
            </div>
            <div class="picker-state-stack">
              <div class="state-tile">
                <span class="token-path">mediaState</span>
                <strong>{mediaState}</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">embedState</span>
                <strong>{embedState}</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">activeMedia</span>
                <strong>{activeMedia.title}</strong>
              </div>
            </div>
            <p class="detail-card-meta">{mediaStatus}</p>
            <p class="detail-card-meta">{embedStatus}</p>
          </Card>
        </div>
      </div>
    </section>
    {/if}

    {#if activeSectionId === "notification-suite"}
    <section id="notification-suite" class="panel token-summary" aria-labelledby="notification-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">State hardening</p>
          <h2 id="notification-heading">Banners, toasts, skeletons, and remediation</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-primitives + @pug/svelte-composites</span>
        <span class="token-path">docs/contracts/foundation/banner.md</span>
        <span class="token-path">docs/contracts/composites/toast-stack.md</span>
      </div>
      <div class="notification-controls">
        <div class="picker-control-group">
          <span class="token-path">Banner tone</span>
          <ToggleGroup
            value={bannerTone}
            options={notificationToneOptions}
            ariaLabel="Banner tone"
            on:valueChange={handleBannerToneChange}
          />
        </div>
        <div class="picker-control-group">
          <span class="token-path">Transient notifications</span>
          <div class="toast-action-row">
            {#each notificationTones as tone}
              <Button variant="secondary" on:click={() => enqueueToast(tone)}>
                push {tone}
              </Button>
            {/each}
          </div>
        </div>
      </div>
      <div class="notification-grid">
        <div class="notification-column">
          {#if showPersistentBanner}
            <Banner
              tone={bannerTone}
              title="Persistent inline remediation"
              message="Use banners when the condition should stay attached to the current surface until the user resolves or dismisses it."
              isDismissible={true}
              on:dismiss={() => (showPersistentBanner = false)}
            >
              <div slot="actions" class="action-cluster">
                <Button variant="secondary" on:click={() => enqueueToast("info")}>Inspect</Button>
                <Button variant="primary" on:click={() => enqueueToast("success")}>Resolve</Button>
              </div>
            </Banner>
          {:else}
            <EmptyState
              title="Persistent banner dismissed"
              message="Dismissal should be explicit and reversible when the condition may return."
              variant="neutral"
            >
              <div slot="actions" class="action-cluster">
                <Button variant="secondary" on:click={() => (showPersistentBanner = true)}>Restore banner</Button>
              </div>
            </EmptyState>
          {/if}

          <Card variant="outlined">
            <div slot="header">
              <p class="eyebrow">Loading scaffolds</p>
            </div>
            <div class="state-skeleton-card" aria-hidden="true">
              <Skeleton shape="block" height="8.25rem" />
              <Skeleton width="64%" />
              <Skeleton width="44%" />
            </div>
            <p class="detail-card-meta">
              Skeletons are decorative only. They reserve layout while the real loading announcement comes from the surrounding state surface.
            </p>
          </Card>
        </div>
        <div class="notification-column">
          <Card variant="outlined">
            <div slot="header">
              <p class="eyebrow">Transient notification stack</p>
            </div>
            <ToastStack items={toastItems} on:dismiss={dismissToast} on:action={handleToastAction} />
            <p class="detail-card-meta">{notificationSummary}</p>
          </Card>
        </div>
      </div>
    </section>
    {/if}

    {#if activeSectionId === "command-suite"}
    <section id="command-suite" class="panel token-summary" aria-labelledby="command-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Workstation command discovery</p>
          <h2 id="command-heading">Palette search, grouped actions, and inline rediscovery</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-workstation</span>
        <span class="token-path">docs/contracts/workstation/command-palette.md</span>
        <span class="token-path">docs/contracts/workstation/action-discovery-panel.md</span>
      </div>
      <div class="command-controls">
        <div class="picker-control-group">
          <span class="token-path">Scope</span>
          <ToggleGroup
            value={commandScope}
            options={commandScopeOptions}
            ariaLabel="Command scope"
            on:valueChange={handleCommandScopeChange}
          />
        </div>
        <div class="picker-control-group">
          <span class="token-path">Palette state</span>
          <ToggleGroup
            value={commandStateOverride}
            options={commandStateOptions}
            ariaLabel="Command palette state"
            on:valueChange={handleCommandStateChange}
          />
        </div>
      </div>
      <div class="command-grid">
        <div class="command-column">
          <Card variant="elevated">
            <div slot="header" class="command-launcher-header">
              <div>
                <p class="eyebrow">Launcher entry</p>
                <h3 class="command-card-title">Global command palette</h3>
              </div>
              <span class="command-shortcut-hint">⌘K</span>
            </div>
            <p class="detail-card-meta">
              The launcher is modal, grouped, keyboard-navigable, and still host-owned for ranking and execution.
            </p>
            <div class="action-cluster">
              <Button variant="primary" on:click={openCommandPalette}>Open palette</Button>
              <Button variant="secondary" on:click={clearCommandDiscovery}>Reset filters</Button>
            </div>
            <div class="picker-state-stack">
              <div class="state-tile">
                <span class="token-path">commandPaletteState</span>
                <strong>{commandPaletteState}</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">lastCommandId</span>
                <strong>{lastCommandId ?? "∅"}</strong>
              </div>
              <div class="state-tile">
                <span class="token-path">query</span>
                <strong>{commandQuery || "∅"}</strong>
              </div>
            </div>
            <p class="detail-card-meta">{commandStatus}</p>
            <p class="detail-card-meta">{commandEventLog}</p>
          </Card>
        </div>
        <div class="command-column">
          <ActionDiscoveryPanel
            title="Inline action discovery"
            description="Suggested and recent actions stay visible outside the modal launcher so command discovery is not palette-only."
            sections={commandSections}
            invocationHint="Open palette with ⌘K"
            on:actionSelect={handleActionDiscoverySelect}
          />
        </div>
      </div>
    </section>
    {/if}

    {#if activeSectionId === "workspace-suite"}
    <section id="workspace-suite" class="panel token-summary" aria-labelledby="workspace-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Workspace shell depth</p>
          <h2 id="workspace-heading">Headers, utility regions, and shell-state posture</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-workstation</span>
        <span class="token-path">docs/contracts/workstation/workspace-shell.md</span>
        <span class="token-path">docs/contracts/workstation/dock-region.md</span>
        <span class="token-path">docs/contracts/workstation/split-view.md</span>
      </div>
      <div class="command-controls">
        <div class="picker-control-group">
          <span class="token-path">Workspace state</span>
          <ToggleGroup
            value={workspaceState}
            options={workspaceStateOptions}
            ariaLabel="Workspace state"
            on:valueChange={handleWorkspaceStateChange}
          />
        </div>
      </div>
      <WorkspaceShell
        ariaLabel="Workstation shell preview"
        state={workspaceState}
        activeSurfaceLabel="Mix review workspace"
        stateTitle={workspaceState === "loading" ? "Loading workspace shell" : workspaceState === "offline" ? "Offline workspace shell" : workspaceState === "disconnected" ? "Workspace disconnected" : "No workspace content"}
        stateMessage={workspaceState === "loading" ? "Headers and status rails remain stable while shell content resolves." : workspaceState === "offline" ? "Offline posture should stay explicit without collapsing local tools." : workspaceState === "disconnected" ? "Disconnection needs recovery posture distinct from deliberate offline mode." : "Workspace shells should explain the missing surface instead of appearing broken."}
      >
        <SurfaceTabs
          slot="surfaceTabs"
          items={workspaceSurfaceItems}
          value={workspaceSurfaceValue}
          ariaLabel="Workspace surfaces"
          on:valueChange={handleSurfaceTabChange}
          on:reorder={handleSurfaceReorder}
          on:requestRename={handleSurfaceRename}
          on:requestMove={handleSurfaceMove}
          on:requestClose={handleSurfaceClose}
          on:requestAdd={handleSurfaceAdd}
        />

        <AppHeader slot="appHeader" title="Pug Workstation" ariaLabel="Application header">
          <div slot="identity" class="workspace-identity">
            <span class="workspace-identity__mark" aria-hidden="true">P</span>
            <strong>Pug Workstation</strong>
          </div>
          <div slot="actions" class="action-cluster">
            <Button variant="secondary">Settings</Button>
            <Button variant="primary" on:click={openCommandPalette}>Commands</Button>
          </div>
          <div slot="utility" class="workspace-status-pill-row">
            <span class="command-shortcut-hint">{workspaceState === "offline" ? "offline" : workspaceState === "disconnected" ? "disconnected" : "connected"}</span>
            <span class="command-shortcut-hint">surface: mix review</span>
          </div>
        </AppHeader>

        <ProjectHeader
          slot="projectHeader"
          title="Aura review delivery"
          subtitle="Project-scoped context, shell actions, and utility state stay beneath the global app header."
          isDirty={workspaceState !== "ready"}
        >
          <div slot="actions" class="action-cluster">
            <Button variant="secondary">Share</Button>
            <Button variant="secondary">Layout</Button>
          </div>
          <div slot="status" class="workspace-status-pill-row">
            <span class="command-shortcut-hint">{commandScope}</span>
            <span class="command-shortcut-hint">{lastCommandId ?? "no command yet"}</span>
          </div>
        </ProjectHeader>

        <div slot="state" class="state-stack">
          {#if workspaceState === "loading"}
            <Banner
              tone="info"
              title="Loading workspace shell"
              message="App and project headers should stay stable while the main shell content resolves."
            />
            <div class="workspace-loading-grid" aria-hidden="true">
              <Skeleton shape="block" height="15rem" />
              <Skeleton shape="block" height="15rem" />
              <Skeleton shape="block" height="15rem" />
            </div>
          {:else if workspaceState === "offline"}
            <Banner
              tone="warning"
              title="Offline mode active"
              message="Offline work is deliberate and should keep local actions available while remote sync remains paused."
            >
              <div slot="actions" class="action-cluster">
                <Button variant="secondary" on:click={() => (workspaceState = "ready")}>Resume sync</Button>
              </div>
            </Banner>
            <EmptyState
              title="Remote collaboration is paused"
              message="Workspace shell context remains visible so users can keep working locally."
            />
          {:else if workspaceState === "disconnected"}
            <Banner
              tone="danger"
              title="Workspace connection lost"
              message="Unexpected disconnection needs adjacent retry and recovery actions."
            >
              <div slot="actions" class="action-cluster">
                <Button variant="secondary" on:click={() => (workspaceState = "ready")}>Reconnect</Button>
              </div>
            </Banner>
            <EmptyState
              title="Shell utilities are waiting for reconnection"
              message="Keep action-discovery and shell identity visible while the host negotiates recovery."
            />
          {:else}
            <EmptyState
              title="No workspace surface configured"
              message="Empty workspace shells should still preserve identity, context, and next steps."
            />
          {/if}
        </div>

        <div class="workspace-orchestration-shell">
          <SplitView
            orientation="horizontal"
            ratio={primarySplitRatio}
            minPrimarySize={leftDockCollapsed ? 72 : 240}
            minSecondarySize={480}
            ariaLabel="Primary workspace split"
            on:ratioChange={handlePrimarySplitChange}
          >
            <div slot="primary" class="workspace-pane">
              <DockRegion
                edge="left"
                items={leftDockItems}
                value={leftDockValue}
                isCollapsed={leftDockCollapsed}
                ariaLabel="Left dock"
                on:valueChange={(event) => handleDockValueChange("left", event)}
                on:collapsedChange={(event) => handleDockCollapsedChange("left", event)}
                on:reorder={(event) => handleDockReorder("left", event)}
                on:requestClose={(event) => handleDockClose("left", event)}
                on:requestContextMenu={(event) => handleDockContextMenu("left", event)}
                let:activeItem
              >
                <div class="workspace-dock-content">
                  <p class="eyebrow">{activeLeftDockMeta?.title ?? activeItem?.label ?? "Left dock"}</p>
                  <p class="detail-card-meta">{activeLeftDockMeta?.summary ?? "Dock content remains host-defined."}</p>
                  <ul class="workspace-list">
                    {#each activeLeftDockMeta?.items ?? [] as item}
                      <li>{item}</li>
                    {/each}
                  </ul>
                </div>
              </DockRegion>
            </div>

            <div slot="secondary" class="workspace-pane">
              <SplitView
                orientation="horizontal"
                ratio={secondarySplitRatio}
                minPrimarySize={420}
                minSecondarySize={rightDockCollapsed ? 72 : 240}
                ariaLabel="Secondary workspace split"
                on:ratioChange={handleSecondarySplitChange}
              >
                <div slot="primary" class="workspace-pane">
                  <PanelSurface
                    title={activeWorkspaceSurface?.label ?? "Workspace surface"}
                    isActive={true}
                    isElevated={true}
                    bodyPadding="md"
                    ariaLabel="Active workspace surface"
                  >
                    <div class="workspace-surface-stack">
                      <div class="workspace-surface-copy">
                        <p class="eyebrow">{activeWorkspaceSurfaceMeta.eyebrow}</p>
                        <h3 class="workspace-surface-title">{activeWorkspaceSurface?.label ?? "Workspace surface"}</h3>
                        <p class="detail-card-meta">{activeWorkspaceSurfaceMeta.description}</p>
                      </div>
                      <div class="workspace-signal-grid">
                        {#each activeWorkspaceSurfaceMeta.highlights as highlight}
                          <div class="state-tile">
                            <span class="token-path">surface signal</span>
                            <strong>{highlight}</strong>
                          </div>
                        {/each}
                      </div>
                      <div class="workspace-demo-grid">
                        <Card variant="outlined">
                          <div slot="header">
                            <p class="eyebrow">Utility rail</p>
                          </div>
                          <div class="workspace-rail-stack">
                            <div class="state-tile">
                              <span class="token-path">activeSurface</span>
                              <strong>{activeWorkspaceSurface?.value ?? "none"}</strong>
                            </div>
                            <div class="state-tile">
                              <span class="token-path">sync</span>
                              <strong>{workspaceState === "offline" ? "paused" : workspaceState === "disconnected" ? "retrying" : "live"}</strong>
                            </div>
                          </div>
                          <p class="detail-card-meta">
                            Utility rails should stay stable under shell-state changes and preserve visible recovery posture.
                          </p>
                        </Card>

                        <Card variant="outlined">
                          <div slot="header">
                            <p class="eyebrow">Persistence snapshot</p>
                          </div>
                          <div class="workspace-rail-stack">
                            <div class="state-tile">
                              <span class="token-path">primarySplitRatio</span>
                              <strong>{Math.round(primarySplitRatio * 100)}%</strong>
                            </div>
                            <div class="state-tile">
                              <span class="token-path">secondarySplitRatio</span>
                              <strong>{Math.round(secondarySplitRatio * 100)}%</strong>
                            </div>
                          </div>
                          <p class="detail-card-meta">{workspacePersistenceSummary}</p>
                        </Card>
                      </div>
                      <Collapsible
                        title="Host-owned persistence payload"
                        description="Serialized layout state stays explicit and reversible without becoming shell chrome."
                        defaultOpen={true}
                      >
                        <svelte:fragment slot="trigger" let:isOpen>
                          <div class="workspace-persistence-trigger">
                            <div>
                              <p class="eyebrow">Serialized layout</p>
                              <h4 class="workspace-persistence-title">Host-owned persistence payload</h4>
                            </div>
                            <div class="workspace-persistence-trigger__meta">
                              <span class="command-shortcut-hint">v{workspaceLayoutSnapshot.version}</span>
                              <Pill appearance="subtle">{isOpen ? "open" : "closed"}</Pill>
                            </div>
                          </div>
                        </svelte:fragment>
                        <div class="workspace-persistence-panel">
                          <pre>{serializedWorkspaceLayout}</pre>
                        </div>
                      </Collapsible>
                    </div>
                  </PanelSurface>
                </div>

                <div slot="secondary" class="workspace-pane">
                  <DockRegion
                    edge="right"
                    items={rightDockItems}
                    value={rightDockValue}
                    isCollapsed={rightDockCollapsed}
                    ariaLabel="Right dock"
                    on:valueChange={(event) => handleDockValueChange("right", event)}
                    on:collapsedChange={(event) => handleDockCollapsedChange("right", event)}
                    on:reorder={(event) => handleDockReorder("right", event)}
                    on:requestClose={(event) => handleDockClose("right", event)}
                    on:requestContextMenu={(event) => handleDockContextMenu("right", event)}
                    let:activeItem
                  >
                    <div class="workspace-dock-stack">
                      <div class="workspace-dock-content">
                        <p class="eyebrow">{activeRightDockMeta?.title ?? activeItem?.label ?? "Right dock"}</p>
                        <p class="detail-card-meta">{activeRightDockMeta?.summary ?? "Dock content remains host-defined."}</p>
                        <ul class="workspace-list">
                          {#each activeRightDockMeta?.items ?? [] as item}
                            <li>{item}</li>
                          {/each}
                        </ul>
                      </div>
                      <ActionDiscoveryPanel
                        title="Workspace actions"
                        description="Inline action discovery remains visible inside docked shell regions for rediscovery without forcing the modal launcher."
                        sections={commandSections}
                        invocationHint="Open global commands with ⌘K"
                        on:actionSelect={handleActionDiscoverySelect}
                      />
                      <Card variant="outlined">
                        <div slot="header">
                          <p class="eyebrow">Orchestration event</p>
                        </div>
                        <div class="workspace-rail-stack">
                          <div class="state-tile">
                            <span class="token-path">workspaceState</span>
                            <strong>{workspaceState}</strong>
                          </div>
                          <div class="state-tile">
                            <span class="token-path">commandPaletteState</span>
                            <strong>{commandPaletteState}</strong>
                          </div>
                        </div>
                        <p class="detail-card-meta">{workspaceEventLog}</p>
                      </Card>
                    </div>
                  </DockRegion>
                </div>
              </SplitView>
            </div>
          </SplitView>
        </div>

        <ShellStatusBar slot="statusBar" summary="Shell utilities and recovery status">
          <div slot="leading" class="workspace-status-pill-row">
            <span class="command-shortcut-hint">surface: {activeWorkspaceSurface?.label ?? "none"}</span>
            <span class="command-shortcut-hint">commands: {filteredCommandActions.length}</span>
            <span class="command-shortcut-hint">layout: {Math.round(primarySplitRatio * 100)} / {Math.round(secondarySplitRatio * 100)}</span>
          </div>
          <div slot="trailing" class="workspace-status-pill-row">
            <span class="command-shortcut-hint">{workspaceState}</span>
            <span class="command-shortcut-hint">{activeRightDockItem?.label ?? "no dock panel"}</span>
            <span class="command-shortcut-hint">{lastCommandId ?? "no recent command"}</span>
          </div>
        </ShellStatusBar>

        <div slot="overlay" class="workspace-overlay-note" aria-hidden="true">
          <span>utility overlays host</span>
        </div>
      </WorkspaceShell>
    </section>
    {/if}

    {#if activeSectionId === "token-summary-section"}
    <section id="token-summary-section" class="panel token-summary" aria-labelledby="token-summary-heading">
      <div class="section-header">
        <div>
          <p class="eyebrow">Key semantic tokens</p>
          <h2 id="token-summary-heading">Runtime-critical values</h2>
        </div>
      </div>
      <div class="section-meta-bar">
        <span class="command-shortcut-hint">@pug/svelte-tokens</span>
        <span class="token-path">packages/tokens/artifacts/css/</span>
        <span class="token-path">packages/tokens/artifacts/ts/</span>
      </div>
      <div class="summary-grid">
        {#each keySemanticTokens as token}
          <article class="summary-tile">
            <span class="token-path">{token.path}</span>
            <strong>{token.value}</strong>
          </article>
        {/each}
      </div>
    </section>
    {/if}

    {#if activeSectionId === "token-inspector"}
    <section id="token-inspector" class="panel token-inspector" aria-labelledby="inspector-heading">
      <div class="section-header inspector-header">
        <div>
          <p class="eyebrow">Semantic token inspector</p>
          <h2 id="inspector-heading">Search the emitted token tree</h2>
        </div>
        <div class="filter-field">
          <SearchField
            id="token-inspector-query"
            value={inspectorQuery}
            placeholder="Filter tokens by path"
            ariaLabel="Filter semantic tokens"
            on:valueChange={handleInspectorQueryChange}
            on:clear={handleInspectorQueryClear}
          />
        </div>
      </div>
      <p class="inspector-count">{matchingTokenCount} semantic tokens shown</p>
      <div class="token-table-wrap">
        <table>
          <thead>
            <tr>
              <th scope="col">Path</th>
              <th scope="col">Value</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredTokens as token}
              <tr>
                <td class="token-path">{token.path}</td>
                <td>{token.value}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </section>
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
