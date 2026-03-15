<script lang="ts">
  import {
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
  } from "@pug/svelte-composites";
  import {
    type BannerTone,
    type ToggleGroupOption,
  } from "@pug/svelte-primitives";
  import {
    CommandPalette,
    type ActionDiscoverySection,
    type CommandActionItem,
    type DiscoveryState,
    type PanelTabItem,
    type SurfaceTabItem,
    type WorkspaceLayoutSnapshot,
    type WorkspaceShellState,
    parseWorkspaceLayoutSnapshot,
    serializeWorkspaceLayoutSnapshot,
  } from "@pug/svelte-workstation";
  import { pxToRem } from "@pug/svelte-tokens";
  import SharedDemoApp from "../components/SharedDemoApp.svelte";

  export let theme = "loophole-studio";
  export let density = "compact";
  export let controlSize = "md";
  export let disabled = false;
  export let invalid = true;
  export let busy = false;
  export let activeSubscreen: string | undefined = undefined;

  type SharedDemoScreenId =
    | "overview-shell"
    | "form-and-validation"
    | "browse-and-table"
    | "detail-and-related-data"
    | "picker-and-media"
    | "command-and-workspace";

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

  function remHeight(px: number): string {
    return pxToRem(px);
  }

  // ── Static data ──────────────────────────────────────────────────────

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
      id: "asset-001", title: "Approval still", assetId: "img.approval-still.v4",
      eyebrow: "Image preview",
      description: "Static artwork and frame captures need stable aspect ratio, fallback, and metadata posture.",
      caption: "Image previews should keep labels and fallback meaning even when no bitmap renderer is available.",
      kind: "image", aspectRatio: "landscape", badge: "PNG", thumbnailMeta: "3840\u00d72160",
      meta: ["still frame", "client-facing", "annotated"],
    },
    {
      id: "asset-002", title: "Stem waveform", assetId: "aud.dialogue-stem.preview",
      eyebrow: "Audio preview",
      description: "Audio-like previews surface timeline cues, peak context, and playback affordances without owning playback engines.",
      caption: "Playback transport remains host-owned; the shared surface only freezes preview framing, metadata, and fallback posture.",
      kind: "audio", aspectRatio: "landscape", badge: "WAV", thumbnailMeta: "01:42",
      meta: ["waveform", "dialogue stem", "48 kHz"],
    },
    {
      id: "asset-003", title: "Review clip", assetId: "vid.session-review.take-7",
      eyebrow: "Video preview",
      description: "Motion previews need explicit framed state, title, and optional duration even when embed or playback is unavailable.",
      caption: "Video surfaces should preserve preview, fallback, and title semantics consistently across web and desktop runtimes.",
      kind: "video", aspectRatio: "video", badge: "MOV", thumbnailMeta: "00:38",
      meta: ["review clip", "commentary", "client pass"],
    },
    {
      id: "asset-004", title: "Delivery brief", assetId: "doc.delivery-brief.final",
      eyebrow: "Document preview",
      description: "Document-style previews need a portrait surface, clear fallback, and metadata without pretending all content is renderable inline.",
      caption: "Document previews should degrade to textual summary and explicit open/download actions when embedded rendering is unavailable.",
      kind: "document", aspectRatio: "portrait", badge: "PDF", thumbnailMeta: "12 pages",
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
    { value: "navigator", label: "Navigator", icon: "\u25eb" },
    { value: "sources", label: "Sources", icon: "\u25a9", isClosable: true },
    { value: "queue", label: "Queue", icon: "\u2263", isClosable: true },
  ];
  const initialRightDockItems: PanelTabItem[] = [
    { value: "metadata", label: "Metadata", icon: "i" },
    { value: "review", label: "Review", icon: "\u2713", isClosable: true },
    { value: "activity", label: "Activity", icon: "\u21ba", isClosable: true },
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
    navigator: { title: "Navigator", summary: "Left docks usually hold browse, jump, and source-scoped context without taking over the center surface.", items: ["Mix checklist", "Scene markers", "Open references"] },
    sources: { title: "Sources", summary: "Source lists can collapse, reorder, and restore without mutating the meaning of the active center tab.", items: ["Dialogue stem", "Music print", "Impulse archive"] },
    queue: { title: "Queue", summary: "Queued work should preserve explicit order and active selection even when the dock is collapsed and restored.", items: ["Render export", "Waveform index", "Delivery zip"] },
  };
  const rightDockCatalog: Record<string, DockPanelMeta> = {
    metadata: { title: "Metadata", summary: "The right dock often carries detail and context that should stay reachable without obscuring the center work area.", items: ["Owner: Aura", "Revision: r6", "Delivery target: client review"] },
    review: { title: "Review", summary: "Review utilities remain local to the active surface but do not become the main surface themselves.", items: ["2 approvals pending", "1 blocking note", "Latest pass: 19:42"] },
    activity: { title: "Activity", summary: "Activity and shell events belong in persistent utility regions when they need to survive tab switches and layout changes.", items: ["Sync resumed", "Layout restored", "Validation rerun"] },
  };
  const commandActions: CommandActionItem[] = [
    { id: "cmd-open-command-palette", title: "Open command palette", description: "Focus the shell launcher and search the full action registry.", group: "Navigation", shortcut: "\u2318K", badge: "global", keywords: ["launcher", "palette", "search"] },
    { id: "cmd-show-mix-browser", title: "Show mix browser", description: "Jump to the central list and grid browse surface.", group: "Navigation", shortcut: "G B", keywords: ["browse", "browser", "list", "grid"] },
    { id: "cmd-focus-review-panel", title: "Focus review panel", description: "Move attention to the right-side review and approval panel.", group: "Workspace", shortcut: "\u23252", badge: "panel", keywords: ["panel", "focus", "review"] },
    { id: "cmd-toggle-metadata-dock", title: "Toggle metadata dock", description: "Collapse or restore the metadata dock without changing the active surface.", group: "Workspace", shortcut: "\u2325M", badge: "dock", keywords: ["dock", "metadata", "toggle"] },
    { id: "cmd-attach-reference-asset", title: "Attach reference asset", description: "Open the relation picker scoped to reference assets.", group: "Assets", shortcut: "A R", badge: "asset", keywords: ["asset", "attach", "reference"] },
    { id: "cmd-reveal-current-export", title: "Reveal current export", description: "Open the host file browser at the active delivery location.", group: "Assets", shortcut: "\u21e7\u2318R", keywords: ["export", "reveal", "finder", "explorer"] },
    { id: "cmd-rerun-validation", title: "Rerun validation", description: "Retry the latest async validation flow against the current form and detail surface.", group: "Recent", shortcut: "\u2318\u21a9", badge: "recent", keywords: ["retry", "validation", "recent"] },
    { id: "cmd-open-review-brief", title: "Open review brief", description: "Jump to the document-style media preview for the delivery brief.", group: "Recent", shortcut: "G D", badge: "recent", keywords: ["brief", "document", "review"] },
  ];
  const commandGroupOrder = ["Navigation", "Workspace", "Assets", "Recent"] as const;
  const commandSectionDescriptions: Record<(typeof commandGroupOrder)[number], string> = {
    Navigation: "Route between major surfaces quickly.",
    Workspace: "High-frequency shell and panel actions.",
    Assets: "Asset-centric commands tied to preview and relation flows.",
    Recent: "Previously used actions kept close for rediscovery.",
  };

  // ── Search indexes ──────────────────────────────────────────────────

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

  // ── Screen definitions ──────────────────────────────────────────────

  const sharedDemoScreens: Array<{
    id: SharedDemoScreenId;
    title: string;
    eyebrow: string;
    summary: string;
  }> = [
    { id: "overview-shell", title: "Overview shell", eyebrow: "Shared demo", summary: "Top-level shell posture, visible remediation, and shared runtime status." },
    { id: "form-and-validation", title: "Form and validation", eyebrow: "Shared demo", summary: "Real form workflow using field, text-entry, selection, dialog, and remediation primitives." },
    { id: "browse-and-table", title: "Browse and table", eyebrow: "Shared demo", summary: "Search, filter, selection, list/grid switching, and structured data in one browse workflow." },
    { id: "detail-and-related-data", title: "Detail and related data", eyebrow: "Shared demo", summary: "Headers, metadata, summaries, and related-data posture." },
    { id: "picker-and-media", title: "Picker and media", eyebrow: "Shared demo", summary: "Picker workflow, selection summary, preview framing, and embed fallback." },
    { id: "command-and-workspace", title: "Command and workspace", eyebrow: "Shared demo", summary: "Command discovery, docks, tabs, split views, and persistence." },
  ];
  const sharedDemoScreenOptions: ToggleGroupOption[] = sharedDemoScreens.map((screen) => ({
    value: screen.id,
    label: screen.title.replace(" and ", " + "),
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

  // ── Mutable state ───────────────────────────────────────────────────

  let projectTitle = "Aura mix review";
  let assetSearch = "track lane automation";
  let validationLog = "No validation cycle has run yet.";
  let tableStatus = "Bulk actions remain hidden until at least one visible row is selected.";
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
    { id: "toast-001", title: "Autosave complete", message: "The current review surface checkpoint was stored successfully.", tone: "success" },
    { id: "toast-002", title: "Remote sync delayed", message: "Background sync is still retrying. Local work remains available.", tone: "warning", actionLabel: "View sync" },
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

  // ── Active screen ───────────────────────────────────────────────────

  $: currentScreenId = (activeSubscreen ?? "overview-shell") as SharedDemoScreenId;
  $: currentScreen = sharedDemoScreens.find((s) => s.id === currentScreenId) ?? sharedDemoScreens[0];

  // ── Reactive derivations ────────────────────────────────────────────

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
  $: filteredRows = allRows;
  $: sortedRows = [...filteredRows].sort((left, right) => {
    if (!sortColumnId) return 0;
    const leftValue = left.cells[sortColumnId] ?? "";
    const rightValue = right.cells[sortColumnId] ?? "";
    return leftValue.localeCompare(rightValue) * (sortDirection === "asc" ? 1 : -1);
  });
  $: totalPages = Math.max(1, Math.ceil(sortedRows.length / pageSize));
  $: if (currentPage > totalPages) { currentPage = totalPages; }
  $: visibleRows = sortedRows.slice((currentPage - 1) * pageSize, currentPage * pageSize);
  $: visibleRowIds = visibleRows.map((row) => row.id);
  $: sortedRowIdSet = new Set(sortedRows.map((row) => row.id));
  $: selectedRowIds = selectedRowIds.filter((rowId) => sortedRowIdSet.has(rowId));
  $: browseNormalizedQuery = normalizeSearchText(browseQuery);
  $: filteredBrowseRows = browseRowSearchIndex
    .filter((entry) =>
      (browseStatus === "all" || entry.item.status === browseStatus) &&
      (browseNormalizedQuery.length === 0 || entry.haystack.includes(browseNormalizedQuery)),
    )
    .map((entry) => entry.item);
  $: filteredBrowseCards =
    browseNormalizedQuery.length === 0
      ? browseCards
      : browseCardSearchIndex.filter((entry) => entry.haystack.includes(browseNormalizedQuery)).map((entry) => entry.item);
  $: listShellState =
    browseStateOverride !== "auto" ? browseStateOverride
      : browseRows.length === 0 ? "empty"
      : filteredBrowseRows.length === 0 ? "no-results"
      : "ready";
  $: gridShellState =
    browseStateOverride !== "auto" ? browseStateOverride
      : browseCards.length === 0 ? "empty"
      : filteredBrowseCards.length === 0 ? "no-results"
      : "ready";
  $: visibleBrowseRows = filteredBrowseRows.slice(0, browseVisibleCount);
  $: gridTotalPages = Math.max(1, Math.ceil(filteredBrowseCards.length / gridPageSize));
  $: if (gridPage > gridTotalPages) { gridPage = gridTotalPages; }
  $: visibleBrowseCards = filteredBrowseCards.slice((gridPage - 1) * gridPageSize, gridPage * gridPageSize);
  $: browseSummary =
    browseStateOverride === "loading" ? "Loading browse results..."
      : browseStateOverride === "error" ? "Browse results unavailable."
      : `${filteredBrowseRows.length} list results, ${filteredBrowseCards.length} grid results, ${browseStatus === "all" ? "all statuses" : browseStatus}.`;
  $: pickerNormalizedQuery = normalizeSearchText(pickerQuery);
  $: filteredRelationItems =
    pickerNormalizedQuery.length === 0
      ? relationItems
      : relationItemSearchIndex.filter((entry) => entry.haystack.includes(pickerNormalizedQuery)).map((entry) => entry.item);
  $: pickerState =
    pickerStateOverride !== "auto" ? pickerStateOverride
      : relationItems.length === 0 ? "empty"
      : filteredRelationItems.length === 0 ? "no-results"
      : "ready";
  $: relationItemIdSet = new Set(relationItems.map((item) => item.id));
  $: selectedRelationIds = selectedRelationIds.filter((id) => relationItemIdSet.has(id));
  $: activeMedia = mediaAssets.find((asset) => asset.id === activeMediaId) ?? mediaAssets[0];
  $: secondaryMediaAssets = mediaAssets.filter((asset) => asset.id !== activeMedia.id).slice(0, 2);
  $: mediaStatus =
    mediaState === "ready" ? `${activeMedia.title} is active. Shared preview shells own framing, not playback engines or host-specific media tooling.`
      : mediaState === "loading" ? "Media previews are reserving layout and metadata while the renderer resolves."
      : mediaState === "error" ? "Preview rendering failed. Fallback text and asset actions remain reachable."
      : "No generated preview is available. Asset identity still remains visible.";
  $: embedStatus =
    embedState === "ready" ? "Embed shells are available. Host code still owns origin, permissions, and actual embedded runtime selection."
      : embedState === "loading" ? "Embed shells stay framed while external content or host-native views initialize."
      : embedState === "error" ? "Embed rendering failed. Fallback posture must still preserve title, context, and recovery actions."
      : "No embedded surface is available. Users still need an explicit fallback path.";
  $: notificationSummary = `${toastItems.length} transient notification(s) queued. Persistent banner is ${showPersistentBanner ? "visible" : "dismissed"}.`;
  $: scopedCommandActionEntries = commandActionSearchIndex.filter(({ action }) => {
    if (commandScope === "all") return true;
    if (commandScope === "recent") return action.group === "Recent";
    if (commandScope === "workspace") return action.group === "Workspace";
    if (commandScope === "navigation") return action.group === "Navigation";
    return action.group === "Assets";
  });
  $: scopedCommandActions = scopedCommandActionEntries.map((entry) => entry.action);
  $: normalizedCommandQuery = normalizeSearchText(commandQuery);
  $: filteredCommandEntries = scopedCommandActionEntries
    .map((entry) => {
      if (normalizedCommandQuery.length === 0) return { action: entry.action, score: 0 };
      if (!entry.haystack.includes(normalizedCommandQuery)) return null;
      const score = entry.title.startsWith(normalizedCommandQuery) ? 3 : entry.title.includes(normalizedCommandQuery) ? 2 : 1;
      return { action: entry.action, score };
    })
    .filter((entry): entry is { action: CommandActionItem; score: number } => entry !== null)
    .sort((left, right) => right.score - left.score || left.action.title.localeCompare(right.action.title));
  $: filteredCommandActions = filteredCommandEntries.map((entry) => entry.action);
  $: commandPaletteState =
    commandStateOverride !== "auto" ? commandStateOverride
      : scopedCommandActions.length === 0 ? "empty"
      : filteredCommandActions.length === 0 ? "no-results"
      : "ready";
  $: commandSections = (() => {
    const groupedActions: Record<(typeof commandGroupOrder)[number], CommandActionItem[]> = { Navigation: [], Workspace: [], Assets: [], Recent: [] };
    for (const action of filteredCommandActions) {
      const group = action.group as (typeof commandGroupOrder)[number];
      const limit = group === "Recent" ? 2 : 3;
      if (groupedActions[group].length < limit) groupedActions[group].push(action);
    }
    return commandGroupOrder.reduce<ActionDiscoverySection[]>((acc, group) => {
      const actions = groupedActions[group];
      if (actions.length > 0) acc.push({ id: group.toLowerCase(), title: group, description: commandSectionDescriptions[group], actions });
      return acc;
    }, []);
  })();
  $: commandStatus =
    commandPaletteState === "ready" ? `${filteredCommandActions.length} commands available in the ${commandScope} scope.`
      : commandPaletteState === "loading" ? "Command discovery is loading."
      : commandPaletteState === "error" ? "Command discovery failed."
      : commandPaletteState === "empty" ? "No commands configured for the current scope."
      : "No commands match the current query.";
  $: workspaceStatus =
    workspaceState === "ready" ? "Workspace shell is interactive."
      : workspaceState === "loading" ? "Workspace chrome stays visible while panels load."
      : workspaceState === "offline" ? "The shell remains usable in offline mode."
      : workspaceState === "disconnected" ? "Connection loss detected."
      : "No workspace content configured.";
  $: activeWorkspaceSurface = workspaceSurfaceItems.find((item) => item.value === workspaceSurfaceValue) ?? workspaceSurfaceItems[0] ?? null;
  $: activeWorkspaceSurfaceMeta = workspaceSurfaceCatalog[activeWorkspaceSurface?.value ?? "mix-review"] ?? workspaceSurfaceCatalog["mix-review"];
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
    leftDock: { edge: "left", isCollapsed: leftDockCollapsed, activePanel: activeLeftDockItem?.value ?? null, order: leftDockItems.map((item) => item.value) },
    rightDock: { edge: "right", isCollapsed: rightDockCollapsed, activePanel: activeRightDockItem?.value ?? null, order: rightDockItems.map((item) => item.value) },
  }) as WorkspaceLayoutSnapshot;
  $: serializedWorkspaceLayout = serializeWorkspaceLayoutSnapshot(workspaceLayoutSnapshot);
  $: parsedWorkspaceLayout = parseWorkspaceLayoutSnapshot(serializedWorkspaceLayout);
  $: workspacePersistenceSummary =
    parsedWorkspaceLayout.activeSurface === workspaceLayoutSnapshot.activeSurface &&
    parsedWorkspaceLayout.primarySplitRatio === workspaceLayoutSnapshot.primarySplitRatio &&
    parsedWorkspaceLayout.secondarySplitRatio === workspaceLayoutSnapshot.secondarySplitRatio
      ? "Round-trip serialization preserves the current shell layout snapshot."
      : "Layout serialization drift detected.";

  // ── Handlers ────────────────────────────────────────────────────────

  function handleScreenChange(screenId: string): void {
    if (typeof window !== "undefined") {
      window.location.hash = `demo/${screenId}`;
    }
  }

  function handleTitleChange(event: CustomEvent<{ value: string }>): void { projectTitle = event.detail.value; }
  function handleSearchChange(event: CustomEvent<{ value: string }>): void { assetSearch = event.detail.value; currentPage = 1; }
  function handleSearchClear(): void { assetSearch = ""; currentPage = 1; }
  function handleBrowseSearchChange(event: CustomEvent<{ value: string }>): void { browseQuery = event.detail.value; browseVisibleCount = 4; gridPage = 1; }
  function handleBrowseSearchClear(): void { browseQuery = ""; browseVisibleCount = 4; gridPage = 1; }
  function handleSubmit(event: Event): void { event.preventDefault(); validationLog = `Submitted title "${projectTitle}" with search query "${assetSearch}".`; }
  function handleSortChange(event: CustomEvent<{ columnId: string; direction: TableSortDirection }>): void { sortColumnId = event.detail.columnId; sortDirection = event.detail.direction; tableStatus = `Sorted by ${event.detail.columnId} in ${event.detail.direction} order.`; }
  function handleRowToggle(event: CustomEvent<{ rowId: string; selected: boolean }>): void { selectedRowIds = event.detail.selected ? [...selectedRowIds, event.detail.rowId] : selectedRowIds.filter((id) => id !== event.detail.rowId); tableStatus = `${selectedRowIds.length} row(s) selected.`; }
  function handleToggleAll(event: CustomEvent<{ selected: boolean }>): void { selectedRowIds = event.detail.selected ? Array.from(new Set([...selectedRowIds, ...visibleRowIds])) : selectedRowIds.filter((id) => !visibleRowIds.includes(id)); tableStatus = event.detail.selected ? `Selected all ${visibleRowIds.length} visible rows.` : "Cleared selection."; }
  function handleBulkAction(event: CustomEvent<{ id: string }>): void { tableStatus = `Bulk action "${event.detail.id}" requested for ${selectedRowIds.length} row(s).`; }
  function clearTableSelection(): void { selectedRowIds = []; tableStatus = "Selection cleared."; }
  function handleRowAction(event: CustomEvent<{ rowId: string }>): void { tableStatus = `Opened row action for ${event.detail.rowId}.`; }
  function handlePageChange(event: CustomEvent<{ page: number }>): void { currentPage = event.detail.page; tableStatus = `Moved to page ${event.detail.page}.`; }
  function handleBrowseStatusChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") { browseStatus = event.detail.value; browseVisibleCount = 4; gridPage = 1; } }
  function handleBrowseStateChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") { browseStateOverride = event.detail.value as "auto" | BrowseState; } }
  function loadMoreBrowseRows(): void { browseVisibleCount = Math.min(filteredBrowseRows.length, browseVisibleCount + 3); }
  function handleGridPageChange(event: CustomEvent<{ page: number }>): void { gridPage = event.detail.page; }
  function handleBreadcrumbNavigate(event: CustomEvent<{ value: string }>): void { validationLog = `Breadcrumb navigation for "${event.detail.value}".`; }
  function handlePickerQueryChange(event: CustomEvent<{ value: string }>): void { pickerQuery = event.detail.value; }
  function handleRelationSelectionChange(event: CustomEvent<{ selectedIds: string[] }>): void { selectedRelationIds = event.detail.selectedIds; pickerStatus = `${event.detail.selectedIds.length} relation item(s) selected in ${pickerMode} mode.`; }
  function handlePickerConfirm(event: CustomEvent<{ selectedIds: string[] }>): void { pickerStatus = `Confirmed ${event.detail.selectedIds.length} relation item(s).`; }
  function handlePickerCancel(): void { pickerStatus = "Picker cancelled."; }
  function handleDetailStateChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") detailState = event.detail.value as typeof detailState; }
  function handlePickerVariantChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") pickerVariant = event.detail.value as PickerVariant; }
  function handlePickerModeChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") { pickerMode = event.detail.value as SelectionMode; selectedRelationIds = pickerMode === "single" ? selectedRelationIds.slice(0, 1) : selectedRelationIds; } }
  function handlePickerStateChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") pickerStateOverride = event.detail.value as "auto" | BrowseState; }
  function getMediaStateTitle(state: MediaState, kind: MediaKind): string { return state === "loading" ? `Loading ${kind} preview` : state === "error" ? `${kind[0].toUpperCase()}${kind.slice(1)} preview unavailable` : `No ${kind} preview`; }
  function getMediaStateMessage(state: MediaState, kind: MediaKind): string | null { return state === "loading" ? "Reserve the preview frame while metadata and shell actions remain stable." : state === "error" ? `Keep ${kind} identity, fallback copy, and recovery actions visible when rendering fails.` : state === "empty" ? "Not every asset guarantees a generated preview." : null; }
  function getEmbedStateMessage(state: MediaState): string | null { return state === "loading" ? "Maintain a stable framed region while the embedded surface initializes." : state === "error" ? "Render fallback guidance and recovery actions." : state === "empty" ? "Some destinations may choose an open-in-host path." : null; }
  function resetBrowseFilters(): void { browseQuery = ""; browseStatus = "all"; browseVisibleCount = 4; gridPage = 1; browseStateOverride = "auto"; }
  function retryBrowseState(): void { browseStateOverride = "auto"; toastSequence += 1; toastItems = [{ id: `toast-${String(toastSequence).padStart(3, "0")}`, title: "Browse retry requested", message: "Shell posture reset to automatic.", tone: "info" }, ...toastItems]; }
  function resetPickerState(): void { pickerStateOverride = "auto"; pickerQuery = ""; }
  function handleMediaStateChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") mediaState = event.detail.value as MediaState; }
  function handleActiveMediaChange(mediaId: string): void { activeMediaId = mediaId; }
  function handleEmbedStateChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") embedState = event.detail.value as MediaState; }
  function handleBannerToneChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") bannerTone = event.detail.value as BannerTone; }
  function enqueueToast(tone: ToastTone): void {
    toastSequence += 1;
    toastItems = [{
      id: `toast-${String(toastSequence).padStart(3, "0")}`,
      title: tone === "success" ? "Publish succeeded" : tone === "warning" ? "Review attention needed" : tone === "danger" ? "Export failed" : "Background refresh started",
      message: tone === "success" ? "Action completed." : tone === "warning" ? "Persistent banner may be more appropriate." : tone === "danger" ? "Recoverable path needed." : "Transient notification sent.",
      tone,
      actionLabel: tone === "danger" || tone === "warning" ? "Review" : null,
    }, ...toastItems];
  }
  function dismissToast(event: CustomEvent<{ id: string }>): void { toastItems = toastItems.filter((item) => item.id !== event.detail.id); }
  function handleToastAction(event: CustomEvent<{ id: string }>): void { validationLog = `Toast action for "${event.detail.id}".`; }
  function openCommandPalette(): void { commandPaletteOpen = true; }
  function closeCommandPalette(): void { commandPaletteOpen = false; }
  function clearCommandDiscovery(): void { commandQuery = ""; commandScope = "all"; commandStateOverride = "auto"; }
  function handleCommandScopeChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") commandScope = event.detail.value as CommandResultScope; }
  function handleCommandStateChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") commandStateOverride = event.detail.value as "auto" | DiscoveryState; }
  function reorderByValue<T extends { value: string }>(items: T[], orderedValues: string[]): T[] { const map = new Map(items.map((item) => [item.value, item])); return orderedValues.map((v) => map.get(v)).filter((item): item is T => item !== undefined); }
  function handleSurfaceTabChange(event: CustomEvent<{ value: string }>): void { workspaceSurfaceValue = event.detail.value; workspaceEventLog = `Activated "${event.detail.value}".`; }
  function handleSurfaceReorder(event: CustomEvent<{ items: string[] }>): void { workspaceSurfaceItems = reorderByValue(workspaceSurfaceItems, event.detail.items); }
  function handleSurfaceRename(event: CustomEvent<{ value: string }>): void { surfaceSequence += 1; workspaceSurfaceItems = workspaceSurfaceItems.map((item) => item.value === event.detail.value ? { ...item, label: `Scratch ${surfaceSequence}` } : item); }
  function handleSurfaceMove(event: CustomEvent<{ value: string }>): void { workspaceEventLog = `Move request for "${event.detail.value}".`; }
  function handleSurfaceClose(event: CustomEvent<{ value: string }>): void {
    if (workspaceSurfaceItems.length <= 1) return;
    const idx = workspaceSurfaceItems.findIndex((item) => item.value === event.detail.value);
    const next = workspaceSurfaceItems.filter((item) => item.value !== event.detail.value);
    workspaceSurfaceItems = next;
    if (workspaceSurfaceValue === event.detail.value) {
      const fallback = next[Math.max(0, idx - 1)] ?? next[0] ?? null;
      workspaceSurfaceValue = fallback?.value ?? "mix-review";
    }
  }
  function handleSurfaceAdd(): void {
    surfaceSequence += 1;
    const v = `surface-${surfaceSequence}`;
    workspaceSurfaceItems = [...workspaceSurfaceItems, { value: v, label: `Scratch ${surfaceSequence}`, isClosable: true }];
    workspaceSurfaceValue = v;
  }
  function handlePrimarySplitChange(event: CustomEvent<{ ratio: number }>): void { primarySplitRatio = event.detail.ratio; }
  function handleSecondarySplitChange(event: CustomEvent<{ ratio: number }>): void { secondarySplitRatio = event.detail.ratio; }
  function handleDockValueChange(edge: "left" | "right", event: CustomEvent<{ value: string }>): void { if (edge === "left") leftDockValue = event.detail.value; else rightDockValue = event.detail.value; }
  function handleDockCollapsedChange(edge: "left" | "right", event: CustomEvent<{ collapsed: boolean }>): void { if (edge === "left") leftDockCollapsed = event.detail.collapsed; else rightDockCollapsed = event.detail.collapsed; }
  function handleDockReorder(edge: "left" | "right", event: CustomEvent<{ items: string[] }>): void { if (edge === "left") leftDockItems = reorderByValue(leftDockItems, event.detail.items); else rightDockItems = reorderByValue(rightDockItems, event.detail.items); }
  function handleDockClose(edge: "left" | "right", event: CustomEvent<{ value: string }>): void {
    const items = edge === "left" ? leftDockItems : rightDockItems;
    const val = edge === "left" ? leftDockValue : rightDockValue;
    const idx = items.findIndex((item) => item.value === event.detail.value);
    const next = items.filter((item) => item.value !== event.detail.value);
    const fallback = next[Math.max(0, idx - 1)]?.value ?? next[0]?.value ?? null;
    if (edge === "left") { leftDockItems = next; leftDockValue = val === event.detail.value ? fallback : val; }
    else { rightDockItems = next; rightDockValue = val === event.detail.value ? fallback : val; }
  }
  function handleDockContextMenu(edge: "left" | "right", event: CustomEvent<{ value: string | null }>): void { workspaceEventLog = `${edge} dock context menu for "${event.detail.value ?? "none"}".`; }
  function handleCommandSelect(event: CustomEvent<{ id: string }>): void { lastCommandId = event.detail.id; commandPaletteOpen = false; commandEventLog = `Command "${event.detail.id}" selected.`; enqueueToast("success"); }
  function handleActionDiscoverySelect(event: CustomEvent<{ id: string }>): void { handleCommandSelect(new CustomEvent("commandSelect", { detail: { id: event.detail.id } })); }
  function handleCommandPaletteQueryChange(event: CustomEvent<{ value: string }>): void { commandQuery = event.detail.value; }
  function handleWorkspaceStateChange(event: CustomEvent<{ value: string | string[] }>): void { if (typeof event.detail.value === "string") workspaceState = event.detail.value as WorkspaceShellState; }
  function handlePreviewKeydown(event: KeyboardEvent): void { if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") { event.preventDefault(); openCommandPalette(); } }
</script>

<svelte:window on:keydown={handlePreviewKeydown} />

<SharedDemoApp
  screenId={currentScreenId}
  screenTabs={sharedDemoScreenOptions}
  screenMeta={currentScreen}
  {theme}
  {density}
  {controlSize}
  onScreenChange={handleScreenChange}
  demo={{
    invalid, busy, disabled,
    projectTitle, assetSearch,
    titleError, titlePendingMessage, titleValidationState, searchValidationState, validationLog,
    handleTitleChange, handleSearchChange, handleSearchClear, handleSubmit,
    tableColumns, visibleRows, selectedRowIds, sortColumnId, sortDirection,
    handleSortChange, handleRowToggle, handleToggleAll, handleRowAction,
    tableStatus, currentPage, totalPages, sortedRows, pageSize,
    handlePageChange, bulkActions, handleBulkAction, clearTableSelection,
    browseQuery, browseStatus, browseStatusOptions, browseStateOptions,
    handleBrowseSearchChange, handleBrowseSearchClear, handleBrowseStatusChange, handleBrowseStateChange,
    listShellState, gridShellState,
    filteredBrowseRows, visibleBrowseRows, filteredBrowseCards, visibleBrowseCards, browseSummary,
    loadMoreBrowseRows, gridPage, gridTotalPages, resetBrowseFilters, retryBrowseState,
    detailBreadcrumbs, detailCards, detailState, handleDetailStateChange, handleBreadcrumbNavigate,
    relationItems, pickerQuery, handlePickerQueryChange,
    pickerVariant, pickerVariantOptions, handlePickerVariantChange,
    pickerMode, pickerModeOptions: selectionModeOptions, handlePickerModeChange,
    pickerState, pickerStateOptions, handlePickerStateChange,
    filteredRelationItems, selectedRelationIds,
    handleRelationSelectionChange, handlePickerConfirm, handlePickerCancel,
    pickerStatus, resetPickerState,
    mediaState, embedState, mediaStateOptions,
    handleMediaStateChange, handleEmbedStateChange,
    activeMedia, activeMediaId, secondaryMediaAssets, handleActiveMediaChange,
    getMediaStateTitle, getMediaStateMessage, getEmbedStateMessage, mediaStatus, embedStatus,
    bannerTone, bannerToneOptions: notificationToneOptions,
    handleBannerToneChange, showPersistentBanner,
    enqueueToast, toastItems, dismissToast, handleToastAction, notificationSummary,
    openCommandPalette, commandScope, commandScopeOptions, handleCommandScopeChange,
    commandStateOverride, commandStateOptions, handleCommandStateChange, clearCommandDiscovery,
    lastCommandId, filteredCommandActions, commandSections, commandStatus, commandEventLog,
    handleActionDiscoverySelect,
    workspaceState, workspaceStateOptions, handleWorkspaceStateChange,
    workspaceSurfaceItems, workspaceSurfaceValue,
    handleSurfaceTabChange, handleSurfaceReorder, handleSurfaceRename, handleSurfaceMove, handleSurfaceClose, handleSurfaceAdd,
    leftDockItems, leftDockValue, leftDockCollapsed,
    rightDockItems, rightDockValue, rightDockCollapsed,
    handleDockValueChange, handleDockCollapsedChange, handleDockReorder, handleDockClose, handleDockContextMenu,
    primarySplitRatio, secondarySplitRatio,
    handlePrimarySplitChange, handleSecondarySplitChange,
    activeWorkspaceSurface, activeWorkspaceSurfaceMeta, activeLeftDockMeta, activeRightDockMeta,
    workspacePersistenceSummary, serializedWorkspaceLayout, workspaceEventLog,
  }}
/>

<CommandPalette
  open={commandPaletteOpen}
  title="Workspace commands"
  description="Search across navigation, workspace, asset, and recent actions."
  query={commandQuery}
  items={filteredCommandActions}
  state={commandPaletteState}
  invocationHint="Esc to close"
  on:queryChange={handleCommandPaletteQueryChange}
  on:commandSelect={handleCommandSelect}
  on:requestClose={closeCommandPalette}
/>
