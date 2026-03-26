export type ComponentTier = "primitive" | "composite";

export type ComponentEntry = {
  slug: string;
  displayName: string;
  packageName: string;
  tier: ComponentTier;
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
  tier: ComponentTier,
  packageName: string,
  description: string,
  hasSpecimen = false,
): ComponentEntry {
  return {
    slug: slug(displayName),
    displayName,
    packageName,
    tier,
    description,
    hasSpecimen,
  };
}

const P = "@poodle/svelte-primitives";
const C = "@poodle/svelte-composites";

export const primitiveComponents: ComponentEntry[] = [
  entry("Accordion", "primitive", P, "Expandable disclosure panels with single or multiple selection.", true),
  entry("AlertDialog", "primitive", P, "Focused confirmation modal for destructive or dangerous actions.", true),
  entry("Box", "primitive", P, "Generic layout container with configurable padding and alignment.", true),
  entry("Breadcrumbs", "primitive", P, "Hierarchical navigation trail showing current location.", true),
  entry("BulkActionBar", "primitive", P, "Action bar for batch operations on selected items.", true),
  entry("Button", "primitive", P, "Primary interactive control for triggering actions.", true),
  entry("Calendar", "primitive", P, "Date grid for picking a single date.", true),
  entry("Callout", "primitive", P, "Informational block with tone, optional dismissal, actions, and ARIA announcements.", true),
  entry("Card", "primitive", P, "Contained surface for grouped content with optional header and actions.", true),
  entry("Checkbox", "primitive", P, "Boolean toggle with label, supporting indeterminate state.", true),
  entry("Code", "primitive", P, "Syntax-highlighted code display with copy button and line numbers.", true),
  entry("ColorPicker", "primitive", P, "Color selection with native picker, hex input, and swatches.", true),
  entry("CollapseToggle", "primitive", P, "Directional chevron toggle for collapsing/expanding regions.", true),
  entry("Collapsible", "primitive", P, "Show/hide content toggle without accordion grouping.", true),
  entry("Combobox", "primitive", P, "Text input with filtered dropdown suggestions.", true),
  entry("ContextMenu", "primitive", P, "Right-click triggered menu overlay.", true),
  entry("DatePicker", "primitive", P, "Date selection with calendar popup.", true),
  entry("DetailRow", "primitive", P, "Label-value pair for metadata display.", true),
  entry("DateRangePicker", "primitive", P, "Start and end date selection with dual calendar.", true),
  entry("DateTimePicker", "primitive", P, "Combined date and time selection.", true),
  entry("DateTimeRangePicker", "primitive", P, "Start/end date-time range selection.", true),
  entry("Dialog", "primitive", P, "Modal overlay for confirmations, forms, or alerts.", true),
  entry("Drawer", "primitive", P, "Slide-out panel from a screen edge.", true),
  entry("DurationInput", "primitive", P, "Segmented hours/minutes/seconds duration entry.", true),
  entry("EditableLabel", "primitive", P, "Inline text that becomes editable on interaction.", true),
  entry("Eyebrow", "primitive", P, "Small uppercase label used for section categorization.", true),
  entry("Field", "primitive", P, "Form field wrapper with label, help text, hint, and validation.", true),
  entry("FieldSet", "primitive", P, "Semantic fieldset grouping with legend and grid layout for related form fields.", true),
  entry("FileUpload", "primitive", P, "File input with drag-and-drop, type filtering, and upload progress.", true),
  entry("FormActions", "primitive", P, "Action row for form submit, cancel, and secondary actions.", true),
  entry("Grid", "primitive", P, "CSS Grid layout container.", true),
  entry("HoverCard", "primitive", P, "Rich preview card triggered by hover or focus.", true),
  entry("Icon", "primitive", P, "SVG icon accepting direct node data or string names. Tree-shakeable via @poodle/icons-lucide.", true),
  entry("IconButton", "primitive", P, "Button variant displaying only an icon.", true),
  entry("IconProvider", "primitive", P, "Context provider that supplies a bulk icon set for string-based icon lookups.", true),
  entry("ListCard", "primitive", P, "Structured list item card with leading/trailing slots.", true),
  entry("Menu", "primitive", P, "Dropdown menu with items, separators, and keyboard navigation.", true),
  entry("Menubar", "primitive", P, "Horizontal menu bar with dropdown sub-menus.", true),
  entry("Meter", "primitive", P, "Visual gauge for scalar values within a known range.", true),
  entry("NavCard", "primitive", P, "Navigation-oriented card link with icon, badge, and arrow.", true),
  entry("NavCardGrid", "primitive", P, "Responsive grid layout for NavCard navigation panels.", true),
  entry("NavigationMenu", "primitive", P, "Horizontal navigation with dropdown sub-menus.", true),
  entry("NumberEntry", "primitive", P, "Numeric input with increment/decrement controls.", true),
  entry("OrderBy", "primitive", P, "Sort-control toolbar for data views with direction toggle.", true),
  entry("Pagination", "primitive", P, "Page navigation controls for paged data sets.", true),
  entry("PaginationSummary", "primitive", P, "Textual summary of pagination state.", true),
  entry("Pill", "primitive", P, "Small inline label chip with tone and size variants.", true),
  entry("PinInput", "primitive", P, "Multi-digit code entry with separate character fields.", true),
  entry("Popover", "primitive", P, "Anchored overlay for contextual content.", true),
  entry("Progress", "primitive", P, "Determinate or indeterminate progress indicator.", true),
  entry("RadioGroup", "primitive", P, "Single-selection option group.", true),
  entry("RangeCalendar", "primitive", P, "Calendar grid for selecting a date range.", true),
  entry("RangeSlider", "primitive", P, "Dual-thumb slider for selecting a numeric range.", true),
  entry("Rating", "primitive", P, "Star-based rating input or display.", true),
  entry("ResizeHandle", "primitive", P, "Drag and keyboard resize handle for split layouts.", true),
  entry("StatusBar", "primitive", P, "Bottom status bar with leading and trailing slots.", true),
  entry("Region", "primitive", P, "Dashed placeholder block for designating layout areas.", true),
  entry("ScrollShell", "primitive", P, "Scrollable container with overflow management.", true),
  entry("SearchField", "primitive", P, "Text input optimized for search with clear action.", true),
  entry("SegmentedControl", "primitive", P, "Inline toggle between mutually exclusive options.", true),
  entry("Select", "primitive", P, "Dropdown selection from a list of options.", true),
  entry("Separator", "primitive", P, "Visual divider between content sections.", true),
  entry("SplitButton", "primitive", P, "Button with primary action and dropdown menu of secondary actions.", true),
  entry("Skeleton", "primitive", P, "Placeholder loading shape for content.", true),
  entry("Slider", "primitive", P, "Single-thumb slider for selecting a numeric value.", true),
  entry("Spinner", "primitive", P, "Animated loading indicator with ring and CLI-grid variants.", true),
  entry("Spacer", "primitive", P, "Flexible space for pushing layout elements apart.", true),
  entry("Stack", "primitive", P, "Vertical flex layout container.", true),
  entry("StatusIndicator", "primitive", P, "Colored dot or icon indicating status.", true),
  entry("Surface", "primitive", P, "Themed container with background, border, and padding variants.", true),
  entry("Switch", "primitive", P, "Toggle switch for on/off states.", true),
  entry("Table", "primitive", P, "Static data table with headers, rows, and alignment.", true),
  entry("Tabs", "primitive", P, "Unified tabbed interface with underline, card, pill, and strip variants. Horizontal and vertical orientation with icons, close, and reorder.", true),
  entry("TextArea", "primitive", P, "Multi-line text input.", true),
  entry("TextInput", "primitive", P, "Single-line text input.", true),
  entry("TimeAgo", "primitive", P, "Live-updating relative timestamp display.", true),
  entry("TimeField", "primitive", P, "Time-of-day input with hour/minute selection.", true),
  entry("TimeZoneSelect", "primitive", P, "Timezone selection dropdown.", true),
  entry("Toggle", "primitive", P, "Pressable toggle button with active/inactive states.", true),
  entry("ToggleGroup", "primitive", P, "Group of mutually exclusive toggle buttons.", true),
  entry("Toolbar", "primitive", P, "Horizontal action bar with grouped controls.", true),
  entry("Tooltip", "primitive", P, "Hover/focus-triggered informational overlay.", true),
  entry("TriStateSwitch", "primitive", P, "Three-position switch for on/off/indeterminate.", true),
  entry("ZonedDateTimePicker", "primitive", P, "Date-time picker with timezone awareness.", true),
].sort((a, b) => a.displayName.localeCompare(b.displayName));

export const compositeComponents: ComponentEntry[] = [
  entry("ActionDiscoveryPanel", "composite", C, "Grouped action list with keyboard navigation, shortcuts, and badges.", true),
  entry("AppHeader", "composite", C, "Top-level application header with branding and global actions.", true),
  entry("AudioPlayer", "composite", C, "Audio playback with transport controls, volume, and speed.", true),
  entry("EditableList", "composite", C, "Self-managing list with add, remove, and optional reorder.", true),
  entry("BlockEditor", "composite", C, "Block-based content editor with paragraph, heading, code, quote, list, image, and divider blocks.", true),
  entry("CardRadioGroup", "composite", C, "Radio selection across rich card options with keyboard navigation.", true),
  entry("CommandPalette", "composite", C, "Keyboard-driven command search and execution overlay.", true),
  entry("ConfirmAction", "composite", C, "Trigger element with confirmation prompt before executing dangerous actions.", true),
  entry("DataTable", "composite", C, "Feature-rich table with sorting, selection, and pagination.", true),
  entry("DetailSection", "composite", C, "Titled section for grouping detail content.", true),
  entry("DetailShell", "composite", C, "Full detail page layout with header, sections, and sidebar.", true),
  entry("EmbedInput", "composite", C, "URL or embed code input with provider detection and parsing.", true),
  entry("EmbedPreview", "composite", C, "Rich preview card for embedded content with aspect ratio and loading states.", true),
  entry("EmptyState", "composite", C, "Placeholder for empty data views with messaging and actions.", true),
  entry("FilterToolbar", "composite", C, "Toolbar with filter controls for data views.", true),
  entry("FormDialog", "composite", C, "Modal dialog with embedded form, validation, and submit/cancel actions.", true),
  entry("FormLayout", "composite", C, "Responsive form grid with error messaging and field layout.", true),
  entry("ListContainer", "composite", C, "List-page shell with header, filters, batch actions, state handling, content, and pagination placement.", true),
  entry("LogList", "composite", C, "Timestamped log viewer with level filtering and auto-scroll.", true),
  entry("MarkdownEditor", "composite", C, "Markdown authoring with formatting toolbar and live preview.", true),
  entry("PageLoading", "composite", C, "Full-viewport loading overlay with spinner, progress, and cancel.", true),
  entry("MediaPicker", "composite", C, "Dialog for browsing and selecting media assets with upload.", true),
  entry("MediaPreview", "composite", C, "Media asset preview with metadata and fallback.", true),
  entry("MediaThumbnail", "composite", C, "Compact media thumbnail with overlay metadata.", true),
  entry("PageHeader", "composite", C, "Page-level header with title, actions, and breadcrumbs.", true),
  entry("PickerShell", "composite", C, "Container for search-and-select picker workflows.", true),
  entry("RelationPicker", "composite", C, "Searchable picker for selecting related items.", true),
  entry("ReorderableList", "composite", C, "Drag-and-drop reorderable list with keyboard support.", true),
  entry("SelectionSummary", "composite", C, "Summary display of current selection state.", true),
  entry("MetricTile", "composite", C, "Compact label-value tile for metrics and KPIs.", true),
  entry("ToastStack", "composite", C, "Stacked transient notification manager.", true),
  entry("VideoPlayer", "composite", C, "Video playback with overlay controls, fullscreen, and captions.", true),
  entry("DockRegion", "composite", C, "Collapsible dock container for panels with tabs, collapse, and drag-and-drop.", true),
  entry("SplitView", "composite", C, "Resizable split pane layout with collapse toggles and drag-to-collapse.", true),
].sort((a, b) => a.displayName.localeCompare(b.displayName));

export const allComponents: ComponentEntry[] = [
  ...primitiveComponents,
  ...compositeComponents,
];

export function findComponent(
  slug: string,
  tier?: ComponentTier,
): ComponentEntry | undefined {
  const list = tier === "primitive"
    ? primitiveComponents
    : tier === "composite"
      ? compositeComponents
      : allComponents;
  return list.find((c) => c.slug === slug);
}
