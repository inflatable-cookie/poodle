export type ComponentTag =
  | "control"
  | "input"
  | "layout"
  | "display"
  | "overlay"
  | "navigation"
  | "data"
  | "media"
  | "feedback"
  | "form"
  | "workstation";

export type ComponentEntry = {
  slug: string;
  displayName: string;
  packageName: string;
  tag: ComponentTag;
  description: string;
  hasSpecimen: boolean;
};

function slug(name: string): string {
  return name
    .replace(/([a-z])([A-Z])/g, "$1-$2")
    .toLowerCase();
}

function entry(
  displayName: string,
  tag: ComponentTag,
  description: string,
  hasSpecimen = false,
): ComponentEntry {
  return {
    slug: slug(displayName),
    displayName,
    packageName: "@poodle/svelte",
    tag,
    description,
    hasSpecimen,
  };
}

export const allComponents: ComponentEntry[] = [
  // Control
  entry("Button", "control", "Primary interactive control for triggering actions.", true),
  entry("IconButton", "control", "Button variant displaying only an icon.", true),
  entry("SplitButton", "control", "Button with primary action and dropdown menu.", true),
  entry("Checkbox", "control", "Boolean toggle with label, supporting indeterminate state.", true),
  entry("Switch", "control", "Toggle switch for on/off states.", true),
  entry("TriStateSwitch", "control", "Three-position switch for on/off/indeterminate.", true),
  entry("Radio", "control", "Single radio option with native name-based group exclusivity.", true),
  entry("RadioGroup", "control", "Single-selection option group.", true),
  entry("SegmentedControl", "control", "Inline toggle between mutually exclusive options.", true),
  entry("ToggleGroup", "control", "Group of mutually exclusive toggle buttons.", true),
  entry("CollapseToggle", "control", "Directional chevron toggle for collapsing/expanding regions.", true),

  // Input
  entry("RefSelect", "input", "Version-control ref chooser with search, a current marker and a loading footer.", true),
  entry("ModelPicker", "input", "Combined model and capability-axis picker in one popover widget.", true),
  entry("AgentChatInput", "input", "Agent composer: auto-growing editor, control toolbar, context ring, submit/stop action.", true),
  entry("TextInput", "input", "Single or multi-line text input with search, slug, and multiline modes.", true),
  entry("TokenInput", "input", "Tokenizing text input for badge-like multi-value entry with separator-driven commit semantics.", true),
  entry("NumberInput", "input", "Numeric input with optional steppers.", true),
  entry("Select", "input", "Dropdown selection from a list of options.", true),
  entry("ColorPicker", "input", "Color selection with native picker, hex input, and swatches.", true),
  entry("DatePicker", "input", "Date selection with calendar popup.", true),
  entry("DateRangePicker", "input", "Start and end date selection with dual calendar.", true),
  entry("DateTimePicker", "input", "Combined date and time selection.", true),
  entry("DateTimeRangePicker", "input", "Start/end date-time range selection.", true),
  entry("DateTimeZonePicker", "input", "Date-time picker with timezone awareness.", true),
  entry("TimeInput", "input", "Time-of-day input with hour/minute selection.", true),
  entry("ThemeSelect", "input", "Theme picker: popover of theme swatch tiles with an optional modular controller.", true),
  entry("TimeZoneSelect", "input", "Timezone selection dropdown.", true),
  entry("DurationInput", "input", "Segmented hours/minutes/seconds duration entry.", true),
  entry("Calendar", "input", "Date grid for picking a single date or a date range.", true),
  entry("CodeInput", "input", "Code entry with visual digit slots and mask mode.", true),
  entry("EditableLabel", "input", "Inline text that becomes editable on interaction.", true),
  entry("Slider", "input", "Single-thumb slider for selecting a numeric value.", true),
  entry("RangeSlider", "input", "Dual-thumb slider for selecting a numeric range.", true),
  entry("Rating", "input", "Star-based rating input or display.", true),
  entry("FileUpload", "input", "File input with drag-and-drop, type filtering, and upload progress.", true),
  entry("EmbedInput", "input", "URL or embed code input with provider detection and parsing.", true),

  // Layout
  entry("Box", "layout", "Generic layout container with configurable padding and alignment.", true),
  entry("Grid", "layout", "CSS Grid layout container.", true),
  entry(
    "ListGrid",
    "layout",
    "Responsive auto-fill grid for card or tile collections with optional header actions.",
    true,
  ),
  entry("Stack", "layout", "Vertical flex layout container.", true),
  entry("Stepper", "navigation", "Route through a multi-step process, with per-step status.", true),
  entry("AgentTranscript", "display", "The output surface of an agent conversation: grouped tool runs, messages and changed files.", true),
  entry("AgentMessage", "display", "One turn of agent prose, rendered from the shared markdown block model.", true),
  entry("AgentQuestion", "input", "The question an agent asks mid-turn, hosted by the composer.", true),
  entry("AgentQuestionRecord", "display", "The read-only record an answered question leaves in the transcript.", true),
  entry("ToolCallGroup", "display", "A contiguous run of tool calls behind one collapsible summary.", true),
  entry("ToolCall", "display", "One row of agent work, with its output behind a disclosure.", true),
  entry("ChangedFiles", "display", "What a turn touched: counted summary expanding into a directory tree.", true),
  entry("Spacer", "layout", "Flexible space for pushing layout elements apart.", true),
  entry("Separator", "layout", "Visual divider between content sections.", true),
  entry("Surface", "layout", "Themed container with background, border, and padding variants.", true),
  entry("Region", "layout", "Dashed placeholder block for designating layout areas.", true),
  entry("ScrollShell", "layout", "Scrollable container with overflow management.", true),
  entry("SplitView", "layout", "Resizable split pane layout with collapse toggles.", true),
  entry("ResizeHandle", "layout", "Drag and keyboard resize handle for split layouts.", true),

  // Display
  entry("Avatar", "display", "Image or initials avatar for user identity surfaces.", true),
  entry("Eyebrow", "display", "Small uppercase label used for section categorization.", true),
  entry("Pill", "display", "Small inline label chip with tone and size variants.", true),
  entry("StatusIndicator", "display", "Colored dot or icon indicating status.", true),
  entry("Text", "display", "Small text primitive for body, caption, hint, and status copy.", true),
  entry("TextLink", "display", "Inline text link or action for prose and metadata copy.", true),
  entry("Icon", "display", "SVG icon accepting direct node data or string names.", true),
  entry("IconProvider", "display", "Context provider for bulk icon set lookups.", true),
  entry("UiPresentationProvider", "display", "Scoped provider for semantic density and size defaults.", true),
  entry("Skeleton", "display", "Placeholder loading shape for content.", true),
  entry("Spinner", "display", "Animated loading indicator with ring and grid variants.", true),
  entry("Code", "display", "Syntax-highlighted code display with copy button.", true),
  entry("TimeAgo", "display", "Live-updating relative timestamp display.", true),
  entry("MetricTile", "display", "Compact label-value tile for metrics and KPIs.", true),
  entry("DetailItem", "display", "Label-value pair for metadata display.", true),
  entry("MetaBar", "display", "Inline metadata ribbon for compact header facts.", true),
  entry("MetaItem", "display", "Compact labeled metadata item for inline ribbons.", true),
  entry("EmbedPreview", "display", "Rich preview card for embedded content.", true),

  // Overlay
  entry("Dialog", "overlay", "Modal overlay for confirmations, forms, or alerts.", true),
  entry("AlertDialog", "overlay", "Focused confirmation modal for destructive actions.", true),
  entry("Drawer", "overlay", "Slide-out panel from a screen edge.", true),
  entry("Popover", "overlay", "Anchored overlay for contextual content.", true),
  entry("HoverCard", "overlay", "Rich preview card triggered by hover or focus.", true),
  entry("Tooltip", "overlay", "Hover/focus-triggered informational overlay.", true),
  entry("Menu", "overlay", "Dropdown menu with items, separators, and keyboard navigation.", true),
  entry("ContextMenu", "overlay", "Right-click triggered menu overlay.", true),
  entry("Menubar", "overlay", "Horizontal menu bar with dropdown sub-menus.", true),
  entry("ConfirmAction", "overlay", "Trigger with confirmation prompt before executing.", true),
  entry("FormDialog", "overlay", "Modal dialog with embedded form and submit/cancel.", true),
  entry("CommandPalette", "overlay", "Keyboard-driven command search and execution overlay.", true),

  // Navigation
  entry("Tabs", "navigation", "Tabbed interface with underline, card, pill, and strip variants.", true),
  entry("Breadcrumbs", "navigation", "Hierarchical navigation trail showing current location.", true),
  entry("Pagination", "navigation", "Page navigation controls for paged data sets.", true),
  entry("PaginationSummary", "navigation", "Textual summary of pagination state.", true),
  entry("NavigationMenu", "navigation", "Horizontal navigation with dropdown sub-menus.", true),
  entry("SidebarNav", "navigation", "Grouped sidebar navigation list with active item state.", true),
  entry("Tree", "navigation", "Hierarchical file-explorer tree with expand/collapse and multi-select.", true),
  entry("NavCard", "navigation", "Navigation-oriented card link with icon, badge, and arrow.", true),

  // Data
  entry("Table", "data", "Static data table with headers, rows, and alignment.", true),
  entry("DataTable", "data", "Feature-rich table with sorting, selection, and pagination.", true),
  entry("ListCard", "data", "Structured list item card with snippet-based leading, footer, and trailing composition.", true),
  entry("ListCardCounter", "data", "Compact icon-count item used in ListCard footer composition.", true),
  entry("ListContainer", "data", "List-page shell with header, filters, and pagination.", true),
  entry("EditableList", "data", "Editable/reorderable list with add, remove, and drag-and-drop.", true),
  entry("Card", "data", "Contained surface for grouped content.", true),
  entry("CardRadioGroup", "data", "Radio selection across rich card options.", true),
  entry("CardToggleGroup", "data", "Toggleable selection across rich card options.", true),
  entry("Accordion", "data", "Expandable disclosure panels with single or multiple selection.", true),
  entry("Collapsible", "data", "Show/hide content toggle without accordion grouping.", true),
  entry("OrderBy", "data", "Sort-control toolbar for data views.", true),
  entry("SelectionSummary", "data", "Summary display of current selection state.", true),
  entry("FilterBuilder", "data", "Popover filter-clause builder with editable pills and AND/OR combinator.", true),
  entry("FilterToolbar", "data", "Toolbar with filter controls for data views.", true),
  entry("LogList", "data", "Operational log viewer and audit activity list.", true),
  entry("RelationPicker", "data", "Searchable picker for selecting related items.", true),
  entry("PickerShell", "data", "Container for search-and-select picker workflows.", true),

  // Media
  entry("AudioPlayer", "media", "Audio playback with transport controls, volume, and speed.", true),
  entry("VideoPlayer", "media", "Video playback with overlay controls and fullscreen.", true),
  entry("MediaPicker", "media", "Dialog for browsing and selecting media assets.", true),
  entry("MediaBrowsePanel", "media", "Browsable media grid with search and pagination.", true),
  entry("MediaPreview", "media", "Media asset preview with metadata and fallback.", true),
  entry("MediaThumbnail", "media", "Compact media thumbnail with overlay metadata.", true),

  // Feedback
  entry("Callout", "feedback", "Informational block with tone and optional actions.", true),
  entry("Progress", "feedback", "Determinate or indeterminate progress indicator.", true),
  entry("Meter", "feedback", "Visual gauge for scalar values within a known range.", true),
  entry("EmptyState", "feedback", "Placeholder for empty data views with messaging.", true),
  entry("PageLoading", "feedback", "Full-viewport loading overlay with spinner.", true),
  entry("ToastStack", "feedback", "Stacked transient notification manager.", true),
  entry("ToastHost", "feedback", "Store-aware toast host with timer policy.", true),
  entry("BulkActionBar", "feedback", "Action bar for batch operations on selected items.", true),
  entry("PasswordRequirements", "feedback", "Password-policy checklist driven by caller rules.", true),
  entry("ErrorBoundary", "feedback", "Svelte error boundary with retryable empty-state fallback.", true),

  // Form
  entry("Field", "form", "Form field wrapper with label, help text, and validation.", true),
  entry("FieldSet", "form", "Semantic fieldset with legend and grid layout.", true),
  entry("FormActions", "form", "Action row for form submit, cancel, and secondary actions.", true),
  entry("FormLayout", "form", "Responsive form grid with error messaging.", true),
  entry("BlockEditor", "form", "Block-based content editor shell with pluggable types.", true),
  entry("MarkdownEditor", "form", "Markdown authoring with formatting toolbar and preview.", true),
  entry("InlineListSection", "form", "Compact related-list section for detail and metadata pages.", true),
  entry("DebugDialog", "form", "Developer-facing JSON debug dialog hidden when no value is supplied.", true),

  // Workstation
  entry("AppHeader", "workstation", "Top-level application header with branding and actions.", true),
  entry("PageHeader", "workstation", "Page-level header with title, actions, and breadcrumbs.", true),
  entry("StatusBar", "workstation", "Bottom status bar with leading and trailing slots.", true),
  entry("DockRegion", "workstation", "Collapsible dock container with tabs and drag-and-drop.", true),
  entry("Toolbar", "workstation", "Horizontal action bar with grouped controls.", true),
  entry("ActionDiscoveryPanel", "workstation", "Grouped action list with keyboard navigation.", true),
  entry("DetailSection", "workstation", "Titled section for grouping detail content.", true),
  entry("DetailSectionGroup", "workstation", "Responsive layout group for multiple peer detail sections.", true),
  entry("DetailShell", "workstation", "Full detail page layout with header and sections.", true),
].sort((a, b) => a.displayName.localeCompare(b.displayName));

export const tagLabels: Record<ComponentTag, string> = {
  control: "Controls",
  input: "Inputs",
  layout: "Layout",
  display: "Display",
  overlay: "Overlays",
  navigation: "Navigation",
  data: "Data",
  media: "Media",
  feedback: "Feedback",
  form: "Form",
  workstation: "Workstation",
};

export const tagOrder: ComponentTag[] = [
  "control",
  "input",
  "layout",
  "display",
  "overlay",
  "navigation",
  "data",
  "media",
  "feedback",
  "form",
  "workstation",
];

export function findComponent(slug: string): ComponentEntry | undefined {
  return allComponents.find((c) => c.slug === slug);
}

export function componentsByTag(): { tag: ComponentTag; label: string; items: ComponentEntry[] }[] {
  return tagOrder
    .map((tag) => ({
      tag,
      label: tagLabels[tag],
      items: allComponents.filter((c) => c.tag === tag),
    }))
    .filter((group) => group.items.length > 0);
}
