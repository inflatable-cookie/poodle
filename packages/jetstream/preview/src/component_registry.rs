//! Component registry — mirrors the GPUI preview's component-registry.rs.
//!
//! Lists every primitive and composite component with display name, description,
//! tier, and specimen availability.

/// Component tier classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Primitive,
    Composite,
}

impl Tier {
    pub fn label(self) -> &'static str {
        match self {
            Tier::Primitive => "Primitive",
            Tier::Composite => "Composite",
        }
    }
}

pub struct ComponentEntry {
    pub slug: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub tier: Tier,
    /// True when a live specimen renderer exists for this component.
    pub has_specimen: bool,
}

pub static PRIMITIVES: &[ComponentEntry] = &[
    ComponentEntry { slug: "accordion", display_name: "Accordion", description: "Expandable disclosure panels.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "alert-dialog", display_name: "AlertDialog", description: "Focused confirmation modal.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "badge", display_name: "Badge", description: "Small status label.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "banner", display_name: "Banner", description: "Persistent message bar.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "box", display_name: "Box", description: "Generic layout container.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "bulk-action-bar", display_name: "BulkActionBar", description: "Batch action bar.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "button", display_name: "Button", description: "Primary interactive control.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "calendar", display_name: "Calendar", description: "Date grid for picking a single date.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "callout", display_name: "Callout", description: "Highlighted informational block.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "card", display_name: "Card", description: "Contained surface for content.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "checkbox", display_name: "Checkbox", description: "Boolean toggle with label.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "code", display_name: "Code", description: "Syntax-highlighted code display.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "collapsible", display_name: "Collapsible", description: "Show/hide content toggle.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "color-picker", display_name: "ColorPicker", description: "Color selection with swatches.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "context-menu", display_name: "ContextMenu", description: "Right-click triggered menu.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "date-picker", display_name: "DatePicker", description: "Date selection with calendar.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "date-range-picker", display_name: "DateRangePicker", description: "Dual calendar date range.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "date-time-picker", display_name: "DateTimePicker", description: "Combined date and time.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "date-time-range-picker", display_name: "DateTimeRangePicker", description: "Date-time range selection.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "detail-item", display_name: "DetailItem", description: "Label-value metadata pair.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "dialog", display_name: "Dialog", description: "Modal overlay.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "drawer", display_name: "Drawer", description: "Slide-out panel from edge.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "duration-input", display_name: "DurationInput", description: "Segmented duration entry.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "editable-label", display_name: "EditableLabel", description: "Inline editable text.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "eyebrow", display_name: "Eyebrow", description: "Small uppercase label.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "field", display_name: "Field", description: "Form field wrapper.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "field-set", display_name: "FieldSet", description: "Grouped form fields with optional legend.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "file-upload", display_name: "FileUpload", description: "File input with drag-and-drop.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "floating-overlay", display_name: "FloatingOverlay", description: "Anchor-relative overlay positioning utility.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "form-actions", display_name: "FormActions", description: "Action row for forms.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "grid", display_name: "Grid", description: "CSS Grid layout container.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "hover-card", display_name: "HoverCard", description: "Rich preview on hover.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "icon", display_name: "Icon", description: "SVG icon from registry.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "icon-button", display_name: "IconButton", description: "Icon-only button.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "inline", display_name: "Inline", description: "Horizontal flex layout.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "list-card", display_name: "ListCard", description: "Structured list item card.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "list-card-counter", display_name: "ListCardCounter", description: "Icon + count for list card footers.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "list-grid", display_name: "ListGrid", description: "Responsive list/tile grid layout.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "menu", display_name: "Menu", description: "Dropdown menu.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "menubar", display_name: "Menubar", description: "Horizontal menu bar.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "meta-bar", display_name: "MetaBar", description: "Wrapping inline metadata row.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "meta-item", display_name: "MetaItem", description: "Label/value pair for metadata.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "meter", display_name: "Meter", description: "Visual gauge for values.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "nav-card", display_name: "NavCard", description: "Navigation-oriented card.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "navigation-menu", display_name: "NavigationMenu", description: "Navigation with dropdowns.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "number-input", display_name: "NumberInput", description: "Numeric input with controls.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "order-by", display_name: "OrderBy", description: "Sort-control toolbar.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "pagination", display_name: "Pagination", description: "Page navigation controls.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "pagination-summary", display_name: "PaginationSummary", description: "Pagination state summary.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "password-requirements", display_name: "PasswordRequirements", description: "Live password policy checklist.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "pill", display_name: "Pill", description: "Small inline label chip.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "code-input", display_name: "CodeInput", description: "Segmented code entry with optional masking.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "popover", display_name: "Popover", description: "Anchored overlay.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "progress", display_name: "Progress", description: "Progress indicator.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "radio-group", display_name: "RadioGroup", description: "Single-selection group.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "range-slider", display_name: "RangeSlider", description: "Dual-thumb slider.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "rating", display_name: "Rating", description: "Star-based rating.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "scroll-shell", display_name: "ScrollShell", description: "Scrollable container.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "segmented-control", display_name: "SegmentedControl", description: "Inline toggle options.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "select", display_name: "Select", description: "Dropdown selection.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "separator", display_name: "Separator", description: "Visual divider.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "skeleton", display_name: "Skeleton", description: "Placeholder loading shape.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "slider", display_name: "Slider", description: "Single-thumb slider.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "spinner", display_name: "Spinner", description: "Animated loading indicator.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "spacer", display_name: "Spacer", description: "Flexible space.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "split-button", display_name: "SplitButton", description: "Button with dropdown.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "stack", display_name: "Stack", description: "Vertical flex layout.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "status-indicator", display_name: "StatusIndicator", description: "Colored status dot.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "surface", display_name: "Surface", description: "Themed container.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "switch", display_name: "Switch", description: "Toggle switch.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "table", display_name: "Table", description: "Static data table.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "tabs", display_name: "Tabs", description: "Tabbed interface.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "text-input", display_name: "TextInput", description: "Single or multi-line text input.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "text-link", display_name: "TextLink", description: "Inline text action or navigation link.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "time-ago", display_name: "TimeAgo", description: "Relative timestamp.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "time-field", display_name: "TimeField", description: "Time-of-day input.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "time-zone-select", display_name: "TimeZoneSelect", description: "Timezone selection.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "toggle-group", display_name: "ToggleGroup", description: "Mutually exclusive toggles.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "toolbar", display_name: "Toolbar", description: "Horizontal action bar.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "tooltip", display_name: "Tooltip", description: "Informational overlay.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "tri-state-switch", display_name: "TriStateSwitch", description: "Three-position switch.", tier: Tier::Primitive, has_specimen: true },
    ComponentEntry { slug: "date-time-zone-picker", display_name: "DateTimeZonePicker", description: "Date-time with timezone.", tier: Tier::Primitive, has_specimen: true },
];

pub static COMPOSITES: &[ComponentEntry] = &[
    ComponentEntry { slug: "action-discovery", display_name: "ActionDiscovery", description: "Categorized action list with shortcuts.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "app-header", display_name: "AppHeader", description: "Application header with logo and nav.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "audio-player", display_name: "AudioPlayer", description: "Audio playback controls.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "autonomous-list", display_name: "AutonomousList", description: "Self-managing list.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "block-editor", display_name: "BlockEditor", description: "Block-based content editor.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "breadcrumbs", display_name: "Breadcrumbs", description: "Navigation trail.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "card-radio-group", display_name: "CardRadioGroup", description: "Radio selection across cards.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "command-palette", display_name: "CommandPalette", description: "Searchable command list.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "command-palette-shell", display_name: "CommandPaletteShell", description: "Command palette activation shell.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "confirm-action", display_name: "ConfirmAction", description: "Confirmation before action.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "data-table", display_name: "DataTable", description: "Feature-rich table.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "detail-section", display_name: "DetailSection", description: "Titled detail section.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "detail-shell", display_name: "DetailShell", description: "Full detail page layout.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "dock-region", display_name: "DockRegion", description: "Resizable docked layout region.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "editable-list", display_name: "EditableList", description: "Add/remove list with inline entry.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "embed-input", display_name: "EmbedInput", description: "URL or embed code input.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "embed-preview", display_name: "EmbedPreview", description: "Rich preview for embeds.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "empty-state", display_name: "EmptyState", description: "Placeholder for empty views.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "filter-toolbar", display_name: "FilterToolbar", description: "Filter controls toolbar.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "form-dialog", display_name: "FormDialog", description: "Modal dialog with form.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "form-layout", display_name: "FormLayout", description: "Responsive form grid with error/success messaging.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "form-shell", display_name: "FormShell", description: "Form container with sections.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "inline-editable-field", display_name: "InlineEditableField", description: "Click-to-edit text field.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "inline-remediation", display_name: "InlineRemediation", description: "Inline fix suggestion.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "list-container", display_name: "ListContainer", description: "Paginated list with state handling.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "log-list", display_name: "LogList", description: "Timestamped log viewer.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "markdown-editor", display_name: "MarkdownEditor", description: "Markdown authoring with preview.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "media-browse-panel", display_name: "MediaBrowsePanel", description: "Media grid with loading states.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "media-picker", display_name: "MediaPicker", description: "Media asset selection dialog.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "media-preview", display_name: "MediaPreview", description: "Media asset preview.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "media-thumbnail", display_name: "MediaThumbnail", description: "Compact media thumbnail.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "page-header", display_name: "PageHeader", description: "Page-level header.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "page-loading", display_name: "PageLoading", description: "Full-viewport loading overlay.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "panel-header", display_name: "PanelHeader", description: "Panel title with collapse and close controls.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "panel-surface", display_name: "PanelSurface", description: "Panel container with header and content.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "panel-tabs", display_name: "PanelTabs", description: "Tabbed panel views.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "picker-shell", display_name: "PickerShell", description: "Search-and-select picker.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "project-header", display_name: "ProjectHeader", description: "Project name with branch and actions.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "relation-picker", display_name: "RelationPicker", description: "Searchable related items picker.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "remediation-banner", display_name: "RemediationBanner", description: "Dismissible fix suggestion banner.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "reorderable-list", display_name: "ReorderableList", description: "Drag-and-drop list.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "selection-summary", display_name: "SelectionSummary", description: "Current selection state.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "shell-status-bar", display_name: "ShellStatusBar", description: "Status bar with left, center, and right slots.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "sidebar-nav", display_name: "SidebarNav", description: "Vertical navigation with groups.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "tree", display_name: "Tree", description: "Hierarchical file-explorer tree with multi-select.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "split-view", display_name: "SplitView", description: "Horizontal and vertical split panes.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "surface-tabs", display_name: "SurfaceTabs", description: "Tab strip for major app surfaces.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "state-tile", display_name: "StateTile", description: "Compact label-value tile.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "toast-host", display_name: "ToastHost", description: "Positioned toast container.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "toast-stack", display_name: "ToastStack", description: "Stacked notification manager.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "validation-summary", display_name: "ValidationSummary", description: "Validation error and warning list.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "video-player", display_name: "VideoPlayer", description: "Video playback controls.", tier: Tier::Composite, has_specimen: true },
    ComponentEntry { slug: "workspace-shell", display_name: "WorkspaceShell", description: "Full workspace layout shell.", tier: Tier::Composite, has_specimen: true },
];

/// Components per section.
pub fn components_for_section(section: crate::app_state::Section) -> &'static [ComponentEntry] {
    match section {
        crate::app_state::Section::Primitives => PRIMITIVES,
        crate::app_state::Section::Composites => COMPOSITES,
        _ => &[],
    }
}


/// Count of components with live specimens in a section.
pub fn specimen_count(section: crate::app_state::Section) -> usize {
    components_for_section(section).iter().filter(|c| c.has_specimen).count()
}
