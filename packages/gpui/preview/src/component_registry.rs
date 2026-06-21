//! Component registry — mirrors the Svelte preview's unified component-registry.ts.
//!
//! The GPUI preview keeps the existing specimen implementation roots, but exposes
//! them through the same adopter-facing tag groups as the Svelte preview.

pub struct ComponentEntry {
    pub slug: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentTag {
    Control,
    Input,
    Layout,
    Display,
    Overlay,
    Navigation,
    Data,
    Media,
    Feedback,
    Form,
    Workstation,
}

impl ComponentTag {
    const ALL: &[ComponentTag] = &[
        ComponentTag::Control,
        ComponentTag::Input,
        ComponentTag::Layout,
        ComponentTag::Display,
        ComponentTag::Overlay,
        ComponentTag::Navigation,
        ComponentTag::Data,
        ComponentTag::Media,
        ComponentTag::Feedback,
        ComponentTag::Form,
        ComponentTag::Workstation,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ComponentTag::Control => "Controls",
            ComponentTag::Input => "Inputs",
            ComponentTag::Layout => "Layout",
            ComponentTag::Display => "Display",
            ComponentTag::Overlay => "Overlays",
            ComponentTag::Navigation => "Navigation",
            ComponentTag::Data => "Data",
            ComponentTag::Media => "Media",
            ComponentTag::Feedback => "Feedback",
            ComponentTag::Form => "Form",
            ComponentTag::Workstation => "Workstation",
        }
    }
}

pub struct ComponentGroup {
    pub tag: ComponentTag,
    pub items: Vec<&'static ComponentEntry>,
}

pub static PRIMITIVES: &[ComponentEntry] = &[
    ComponentEntry {
        slug: "accordion",
        display_name: "Accordion",
        description: "Expandable disclosure panels with single or multiple selection.",
    },
    ComponentEntry {
        slug: "avatar",
        display_name: "Avatar",
        description: "Image or initials avatar for user identity surfaces.",
    },
    ComponentEntry {
        slug: "alert-dialog",
        display_name: "AlertDialog",
        description: "Focused confirmation modal for destructive actions.",
    },
    ComponentEntry {
        slug: "box",
        display_name: "Box",
        description: "Generic layout container with configurable padding and alignment.",
    },
    ComponentEntry {
        slug: "bulk-action-bar",
        display_name: "BulkActionBar",
        description: "Action bar for batch operations on selected items.",
    },
    ComponentEntry {
        slug: "button",
        display_name: "Button",
        description: "Primary interactive control for triggering actions.",
    },
    ComponentEntry {
        slug: "calendar",
        display_name: "Calendar",
        description: "Date grid for picking a single date or a date range.",
    },
    ComponentEntry {
        slug: "callout",
        display_name: "Callout",
        description: "Informational block with tone and optional actions.",
    },
    ComponentEntry {
        slug: "card",
        display_name: "Card",
        description: "Contained surface for grouped content.",
    },
    ComponentEntry {
        slug: "checkbox",
        display_name: "Checkbox",
        description: "Boolean toggle with label, supporting indeterminate state.",
    },
    ComponentEntry {
        slug: "code",
        display_name: "Code",
        description: "Syntax-highlighted code display with copy button.",
    },
    ComponentEntry {
        slug: "collapsible",
        display_name: "Collapsible",
        description: "Show/hide content toggle without accordion grouping.",
    },
    ComponentEntry {
        slug: "color-picker",
        display_name: "ColorPicker",
        description: "Color selection with native picker, hex input, and swatches.",
    },
    ComponentEntry {
        slug: "context-menu",
        display_name: "ContextMenu",
        description: "Right-click triggered menu overlay.",
    },
    ComponentEntry {
        slug: "date-picker",
        display_name: "DatePicker",
        description: "Date selection with calendar popup.",
    },
    ComponentEntry {
        slug: "date-range-picker",
        display_name: "DateRangePicker",
        description: "Start and end date selection with dual calendar.",
    },
    ComponentEntry {
        slug: "date-time-picker",
        display_name: "DateTimePicker",
        description: "Combined date and time.",
    },
    ComponentEntry {
        slug: "date-time-range-picker",
        display_name: "DateTimeRangePicker",
        description: "Start/end date-time range selection.",
    },
    ComponentEntry {
        slug: "detail-item",
        display_name: "DetailItem",
        description: "Label-value pair for metadata display.",
    },
    ComponentEntry {
        slug: "dialog",
        display_name: "Dialog",
        description: "Modal overlay for confirmations, forms, or alerts.",
    },
    ComponentEntry {
        slug: "drawer",
        display_name: "Drawer",
        description: "Slide-out panel from a screen edge.",
    },
    ComponentEntry {
        slug: "duration-input",
        display_name: "DurationInput",
        description: "Segmented hours/minutes/seconds duration entry.",
    },
    ComponentEntry {
        slug: "editable-label",
        display_name: "EditableLabel",
        description: "Inline text that becomes editable on interaction.",
    },
    ComponentEntry {
        slug: "eyebrow",
        display_name: "Eyebrow",
        description: "Small uppercase label used for section categorization.",
    },
    ComponentEntry {
        slug: "field",
        display_name: "Field",
        description: "Form field wrapper with label, help text, and validation.",
    },
    ComponentEntry {
        slug: "field-set",
        display_name: "FieldSet",
        description: "Semantic fieldset with legend and grid layout.",
    },
    ComponentEntry {
        slug: "file-upload",
        display_name: "FileUpload",
        description: "File input with drag-and-drop, type filtering, and upload progress.",
    },
    ComponentEntry {
        slug: "form-actions",
        display_name: "FormActions",
        description: "Action row for form submit, cancel, and secondary actions.",
    },
    ComponentEntry {
        slug: "grid",
        display_name: "Grid",
        description: "CSS Grid layout container.",
    },
    ComponentEntry {
        slug: "hover-card",
        display_name: "HoverCard",
        description: "Rich preview card triggered by hover or focus.",
    },
    ComponentEntry {
        slug: "icon",
        display_name: "Icon",
        description: "SVG icon accepting direct node data or string names.",
    },
    ComponentEntry {
        slug: "icon-button",
        display_name: "IconButton",
        description: "Button variant displaying only an icon.",
    },
    ComponentEntry {
        slug: "icon-provider",
        display_name: "IconProvider",
        description: "Context provider for bulk icon set lookups.",
    },
    ComponentEntry {
        slug: "list-card",
        display_name: "ListCard",
        description: "Structured list item card with leading/trailing slots.",
    },
    ComponentEntry {
        slug: "list-card-counter",
        display_name: "ListCardCounter",
        description: "Compact icon-count item used in ListCard footer composition.",
    },
    ComponentEntry {
        slug: "list-grid",
        display_name: "ListGrid",
        description:
            "Responsive auto-fill grid for card or tile collections with optional header actions.",
    },
    ComponentEntry {
        slug: "menu",
        display_name: "Menu",
        description: "Dropdown menu with items, separators, and keyboard navigation.",
    },
    ComponentEntry {
        slug: "menubar",
        display_name: "Menubar",
        description: "Horizontal menu bar with dropdown sub-menus.",
    },
    ComponentEntry {
        slug: "meter",
        display_name: "Meter",
        description: "Visual gauge for scalar values within a known range.",
    },
    ComponentEntry {
        slug: "meta-bar",
        display_name: "MetaBar",
        description: "Inline metadata ribbon for compact header facts.",
    },
    ComponentEntry {
        slug: "meta-item",
        display_name: "MetaItem",
        description: "Compact labeled metadata item for inline ribbons.",
    },
    ComponentEntry {
        slug: "nav-card",
        display_name: "NavCard",
        description: "Navigation-oriented card link with icon, badge, and arrow.",
    },
    ComponentEntry {
        slug: "navigation-menu",
        display_name: "NavigationMenu",
        description: "Horizontal navigation with dropdown sub-menus.",
    },
    ComponentEntry {
        slug: "number-input",
        display_name: "NumberInput",
        description: "Numeric input with optional steppers.",
    },
    ComponentEntry {
        slug: "order-by",
        display_name: "OrderBy",
        description: "Sort-control toolbar for data views.",
    },
    ComponentEntry {
        slug: "pagination",
        display_name: "Pagination",
        description: "Page navigation controls for paged data sets.",
    },
    ComponentEntry {
        slug: "pagination-summary",
        display_name: "PaginationSummary",
        description: "Textual summary of pagination state.",
    },
    ComponentEntry {
        slug: "pill",
        display_name: "Pill",
        description: "Small inline label chip with tone and size variants.",
    },
    ComponentEntry {
        slug: "password-requirements",
        display_name: "PasswordRequirements",
        description: "Password-policy checklist driven by caller rules.",
    },
    ComponentEntry {
        slug: "code-input",
        display_name: "CodeInput",
        description: "Code entry with visual digit slots and mask mode.",
    },
    ComponentEntry {
        slug: "popover",
        display_name: "Popover",
        description: "Anchored overlay for contextual content.",
    },
    ComponentEntry {
        slug: "progress",
        display_name: "Progress",
        description: "Determinate or indeterminate progress indicator.",
    },
    ComponentEntry {
        slug: "radio-group",
        display_name: "RadioGroup",
        description: "Single-selection option group.",
    },
    ComponentEntry {
        slug: "range-slider",
        display_name: "RangeSlider",
        description: "Dual-thumb slider for selecting a numeric range.",
    },
    ComponentEntry {
        slug: "rating",
        display_name: "Rating",
        description: "Star-based rating input or display.",
    },
    ComponentEntry {
        slug: "scroll-shell",
        display_name: "ScrollShell",
        description: "Scrollable container with overflow management.",
    },
    ComponentEntry {
        slug: "region",
        display_name: "Region",
        description: "Dashed placeholder block for designating layout areas.",
    },
    ComponentEntry {
        slug: "segmented-control",
        display_name: "SegmentedControl",
        description: "Inline toggle between mutually exclusive options.",
    },
    ComponentEntry {
        slug: "select",
        display_name: "Select",
        description: "Dropdown selection from a list of options.",
    },
    ComponentEntry {
        slug: "separator",
        display_name: "Separator",
        description: "Visual divider between content sections.",
    },
    ComponentEntry {
        slug: "skeleton",
        display_name: "Skeleton",
        description: "Placeholder loading shape for content.",
    },
    ComponentEntry {
        slug: "slider",
        display_name: "Slider",
        description: "Single-thumb slider for selecting a numeric value.",
    },
    ComponentEntry {
        slug: "spinner",
        display_name: "Spinner",
        description: "Animated loading indicator with ring and grid variants.",
    },
    ComponentEntry {
        slug: "spacer",
        display_name: "Spacer",
        description: "Flexible space for pushing layout elements apart.",
    },
    ComponentEntry {
        slug: "split-button",
        display_name: "SplitButton",
        description: "Button with primary action and dropdown menu.",
    },
    ComponentEntry {
        slug: "stack",
        display_name: "Stack",
        description: "Vertical flex layout container.",
    },
    ComponentEntry {
        slug: "status-indicator",
        display_name: "StatusIndicator",
        description: "Colored dot or icon indicating status.",
    },
    ComponentEntry {
        slug: "surface",
        display_name: "Surface",
        description: "Themed container with background, border, and padding variants.",
    },
    ComponentEntry {
        slug: "switch",
        display_name: "Switch",
        description: "Toggle switch for on/off states.",
    },
    ComponentEntry {
        slug: "table",
        display_name: "Table",
        description: "Static data table with headers, rows, and alignment.",
    },
    ComponentEntry {
        slug: "tabs",
        display_name: "Tabs",
        description: "Tabbed interface with underline, card, pill, and strip variants.",
    },
    ComponentEntry {
        slug: "text",
        display_name: "Text",
        description: "Small text primitive for body, caption, hint, and status copy.",
    },
    ComponentEntry {
        slug: "text-input",
        display_name: "TextInput",
        description: "Single or multi-line text input with search, slug, and multiline modes.",
    },
    ComponentEntry {
        slug: "text-link",
        display_name: "TextLink",
        description: "Inline text link or action for prose and metadata copy.",
    },
    ComponentEntry {
        slug: "time-ago",
        display_name: "TimeAgo",
        description: "Live-updating relative timestamp display.",
    },
    ComponentEntry {
        slug: "time-input",
        display_name: "TimeInput",
        description: "Time-of-day input with hour/minute selection.",
    },
    ComponentEntry {
        slug: "time-zone-select",
        display_name: "TimeZoneSelect",
        description: "Timezone selection dropdown.",
    },
    ComponentEntry {
        slug: "token-input",
        display_name: "TokenInput",
        description: "Tokenizing text input for badge-like multi-value entry.",
    },
    ComponentEntry {
        slug: "toggle-group",
        display_name: "ToggleGroup",
        description: "Group of mutually exclusive toggle buttons.",
    },
    ComponentEntry {
        slug: "toolbar",
        display_name: "Toolbar",
        description: "Horizontal action bar with grouped controls.",
    },
    ComponentEntry {
        slug: "tooltip",
        display_name: "Tooltip",
        description: "Hover/focus-triggered informational overlay.",
    },
    ComponentEntry {
        slug: "tri-state-switch",
        display_name: "TriStateSwitch",
        description: "Three-position switch for on/off/indeterminate.",
    },
    ComponentEntry {
        slug: "ui-presentation-provider",
        display_name: "UiPresentationProvider",
        description: "Scoped provider for semantic density and size defaults.",
    },
    ComponentEntry {
        slug: "date-time-zone-picker",
        display_name: "DateTimeZonePicker",
        description: "Date-time picker with timezone awareness.",
    },
];

pub static COMPOSITES: &[ComponentEntry] = &[
    ComponentEntry {
        slug: "audio-player",
        display_name: "AudioPlayer",
        description: "Audio playback with transport controls, volume, and speed.",
    },
    ComponentEntry {
        slug: "editable-list",
        display_name: "EditableList",
        description: "Editable/reorderable list with add, remove, and drag-and-drop.",
    },
    ComponentEntry {
        slug: "block-editor",
        display_name: "BlockEditor",
        description: "Block-based content editor shell with pluggable types.",
    },
    ComponentEntry {
        slug: "breadcrumbs",
        display_name: "Breadcrumbs",
        description: "Hierarchical navigation trail showing current location.",
    },
    ComponentEntry {
        slug: "card-radio-group",
        display_name: "CardRadioGroup",
        description: "Radio selection across rich card options.",
    },
    ComponentEntry {
        slug: "card-toggle-group",
        display_name: "CardToggleGroup",
        description: "Toggleable selection across rich card options.",
    },
    ComponentEntry {
        slug: "confirm-action",
        display_name: "ConfirmAction",
        description: "Trigger with confirmation prompt before executing.",
    },
    ComponentEntry {
        slug: "data-table",
        display_name: "DataTable",
        description: "Feature-rich table with sorting, selection, and pagination.",
    },
    ComponentEntry {
        slug: "debug-dialog",
        display_name: "DebugDialog",
        description: "Developer-facing JSON debug dialog hidden when no value is supplied.",
    },
    ComponentEntry {
        slug: "embed-input",
        display_name: "EmbedInput",
        description: "URL or embed code input with provider detection and parsing.",
    },
    ComponentEntry {
        slug: "embed-preview",
        display_name: "EmbedPreview",
        description: "Rich preview card for embedded content.",
    },
    ComponentEntry {
        slug: "empty-state",
        display_name: "EmptyState",
        description: "Placeholder for empty data views with messaging.",
    },
    ComponentEntry {
        slug: "error-boundary",
        display_name: "ErrorBoundary",
        description: "Svelte-style error boundary with retryable empty-state fallback.",
    },
    ComponentEntry {
        slug: "filter-toolbar",
        display_name: "FilterToolbar",
        description: "Toolbar with filter controls for data views.",
    },
    ComponentEntry {
        slug: "form-dialog",
        display_name: "FormDialog",
        description: "Modal dialog with embedded form and submit/cancel.",
    },
    ComponentEntry {
        slug: "form-layout",
        display_name: "FormLayout",
        description: "Responsive form grid with error messaging.",
    },
    ComponentEntry {
        slug: "form-shell",
        display_name: "FormShell",
        description: "Orchestrated form surface with sections, status summary, and submission gating.",
    },
    ComponentEntry {
        slug: "validation-summary",
        display_name: "ValidationSummary",
        description: "Grouped error surface listing all currently-invalid fields.",
    },
    ComponentEntry {
        slug: "inline-list-section",
        display_name: "InlineListSection",
        description: "Compact related-list section for detail and metadata pages.",
    },
    ComponentEntry {
        slug: "list-container",
        display_name: "ListContainer",
        description: "List-page shell with header, filters, and pagination.",
    },
    ComponentEntry {
        slug: "log-list",
        display_name: "LogList",
        description: "Operational log viewer and audit activity list.",
    },
    ComponentEntry {
        slug: "markdown-editor",
        display_name: "MarkdownEditor",
        description: "Markdown authoring with formatting toolbar and preview.",
    },
    ComponentEntry {
        slug: "media-browse-panel",
        display_name: "MediaBrowsePanel",
        description: "Browsable media grid with search and pagination.",
    },
    ComponentEntry {
        slug: "media-picker",
        display_name: "MediaPicker",
        description: "Dialog for browsing and selecting media assets.",
    },
    ComponentEntry {
        slug: "media-preview",
        display_name: "MediaPreview",
        description: "Media asset preview with metadata and fallback.",
    },
    ComponentEntry {
        slug: "media-thumbnail",
        display_name: "MediaThumbnail",
        description: "Compact media thumbnail with overlay metadata.",
    },
    ComponentEntry {
        slug: "page-header",
        display_name: "PageHeader",
        description: "Page-level header with title, actions, and breadcrumbs.",
    },
    ComponentEntry {
        slug: "page-loading",
        display_name: "PageLoading",
        description: "Full-viewport loading overlay with spinner.",
    },
    ComponentEntry {
        slug: "relation-picker",
        display_name: "RelationPicker",
        description: "Searchable picker for selecting related items.",
    },
    ComponentEntry {
        slug: "selection-summary",
        display_name: "SelectionSummary",
        description: "Summary display of current selection state.",
    },
    ComponentEntry {
        slug: "sidebar-nav",
        display_name: "SidebarNav",
        description: "Grouped sidebar navigation list with active item state.",
    },
    ComponentEntry {
        slug: "tree",
        display_name: "Tree",
        description: "Hierarchical file-explorer tree with expand/collapse and multi-select.",
    },
    ComponentEntry {
        slug: "metric-tile",
        display_name: "MetricTile",
        description: "Compact label-value tile for metrics and KPIs.",
    },
    ComponentEntry {
        slug: "toast-host",
        display_name: "ToastHost",
        description: "Store-aware toast host with timer policy.",
    },
    ComponentEntry {
        slug: "toast-stack",
        display_name: "ToastStack",
        description: "Stacked transient notification manager.",
    },
    ComponentEntry {
        slug: "video-player",
        display_name: "VideoPlayer",
        description: "Video playback with overlay controls and fullscreen.",
    },
];

pub static SHELLS: &[ComponentEntry] = &[
    ComponentEntry {
        slug: "action-discovery-panel",
        display_name: "ActionDiscoveryPanel",
        description: "Grouped action list with keyboard navigation.",
    },
    ComponentEntry {
        slug: "app-header",
        display_name: "AppHeader",
        description: "Top-level application header with branding and actions.",
    },
    ComponentEntry {
        slug: "collapse-toggle",
        display_name: "CollapseToggle",
        description: "Directional chevron toggle for collapsing/expanding regions.",
    },
    ComponentEntry {
        slug: "command-palette",
        display_name: "CommandPalette",
        description: "Keyboard-driven command search and execution overlay.",
    },
    ComponentEntry {
        slug: "detail-section",
        display_name: "DetailSection",
        description: "Titled section for grouping detail content.",
    },
    ComponentEntry {
        slug: "detail-section-group",
        display_name: "DetailSectionGroup",
        description: "Responsive layout group for multiple peer detail sections.",
    },
    ComponentEntry {
        slug: "detail-shell",
        display_name: "DetailShell",
        description: "Full detail page layout with header and sections.",
    },
    ComponentEntry {
        slug: "dock-region",
        display_name: "DockRegion",
        description: "Collapsible dock container with tabs and drag-and-drop.",
    },
    ComponentEntry {
        slug: "picker-shell",
        display_name: "PickerShell",
        description: "Container for search-and-select picker workflows.",
    },
    ComponentEntry {
        slug: "resize-handle",
        display_name: "ResizeHandle",
        description: "Drag and keyboard resize handle for split layouts.",
    },
    ComponentEntry {
        slug: "split-view",
        display_name: "SplitView",
        description: "Resizable split pane layout with collapse toggles.",
    },
    ComponentEntry {
        slug: "status-bar",
        display_name: "StatusBar",
        description: "Bottom status bar with leading and trailing slots.",
    },
];

pub fn component_tag(slug: &str) -> ComponentTag {
    match slug {
        "button" | "icon-button" | "split-button" | "checkbox" | "switch" | "tri-state-switch"
        | "radio-group" | "segmented-control" | "toggle-group" => ComponentTag::Control,
        "text-input"
        | "number-input"
        | "select"
        | "color-picker"
        | "date-picker"
        | "date-range-picker"
        | "date-time-picker"
        | "date-time-range-picker"
        | "date-time-zone-picker"
        | "time-input"
        | "time-zone-select"
        | "duration-input"
        | "calendar"
        | "code-input"
        | "editable-label"
        | "slider"
        | "range-slider"
        | "rating"
        | "file-upload"
        | "embed-input" => ComponentTag::Input,
        "token-input" => ComponentTag::Input,
        "box" | "grid" | "list-grid" | "stack" | "spacer" | "separator" | "surface"
        | "scroll-shell" | "region" | "split-view" | "resize-handle" => ComponentTag::Layout,
        "avatar"
        | "eyebrow"
        | "pill"
        | "status-indicator"
        | "text"
        | "text-link"
        | "icon"
        | "icon-provider"
        | "ui-presentation-provider"
        | "skeleton"
        | "spinner"
        | "code"
        | "time-ago"
        | "metric-tile"
        | "detail-item"
        | "meta-bar"
        | "meta-item"
        | "embed-preview" => ComponentTag::Display,
        "dialog" | "alert-dialog" | "drawer" | "popover" | "hover-card" | "tooltip" | "menu"
        | "context-menu" | "menubar" | "confirm-action" | "form-dialog" | "command-palette" => {
            ComponentTag::Overlay
        }
        "tabs" | "breadcrumbs" | "pagination" | "pagination-summary" | "navigation-menu"
        | "sidebar-nav" | "nav-card" => ComponentTag::Navigation,
        "table" | "data-table" | "list-card" | "list-container" | "editable-list" | "card"
        | "list-card-counter" | "card-radio-group" | "card-toggle-group" | "accordion"
        | "collapsible" | "order-by" | "selection-summary" | "filter-toolbar" | "log-list"
        | "relation-picker" | "picker-shell" => ComponentTag::Data,
        "audio-player" | "video-player" | "media-picker" | "media-browse-panel"
        | "media-preview" | "media-thumbnail" => ComponentTag::Media,
        "callout"
        | "progress"
        | "meter"
        | "empty-state"
        | "page-loading"
        | "toast-stack"
        | "toast-host"
        | "bulk-action-bar"
        | "password-requirements"
        | "error-boundary" => ComponentTag::Feedback,
        "field"
        | "field-set"
        | "form-actions"
        | "form-layout"
        | "form-shell"
        | "validation-summary"
        | "block-editor"
        | "markdown-editor"
        | "inline-list-section"
        | "debug-dialog" => ComponentTag::Form,
        "app-header"
        | "page-header"
        | "status-bar"
        | "dock-region"
        | "toolbar"
        | "action-discovery-panel"
        | "detail-section"
        | "detail-section-group"
        | "detail-shell" => ComponentTag::Workstation,
        _ => ComponentTag::Workstation,
    }
}

pub fn package_name() -> &'static str {
    "poodle-gpui-components"
}

pub fn contract_root() -> &'static str {
    "docs/contracts/components/"
}

pub fn contract_doc_path(slug: &str) -> String {
    format!("{}{}.md", contract_root(), slug)
}

pub fn all_components() -> Vec<&'static ComponentEntry> {
    let mut components: Vec<&'static ComponentEntry> = PRIMITIVES
        .iter()
        .chain(COMPOSITES.iter())
        .chain(SHELLS.iter())
        .collect();
    components.sort_by(|a, b| a.display_name.cmp(b.display_name));
    components
}

pub fn find_component(slug: &str) -> Option<&'static ComponentEntry> {
    all_components()
        .into_iter()
        .find(|component| component.slug == slug)
}

pub fn grouped_components(search: &str) -> Vec<ComponentGroup> {
    let query = search.trim().to_ascii_lowercase();
    let components = all_components();

    ComponentTag::ALL
        .iter()
        .filter_map(|tag| {
            let items: Vec<&'static ComponentEntry> = components
                .iter()
                .copied()
                .filter(|component| component_tag(component.slug) == *tag)
                .filter(|component| {
                    query.is_empty()
                        || component.display_name.to_ascii_lowercase().contains(&query)
                        || component.description.to_ascii_lowercase().contains(&query)
                })
                .collect();

            if items.is_empty() {
                None
            } else {
                Some(ComponentGroup { tag: *tag, items })
            }
        })
        .collect()
}
