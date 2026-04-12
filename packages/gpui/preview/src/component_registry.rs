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
        description: "Expandable disclosure panels.",
    },
    ComponentEntry {
        slug: "alert-dialog",
        display_name: "AlertDialog",
        description: "Focused confirmation modal.",
    },
    ComponentEntry {
        slug: "box",
        display_name: "Box",
        description: "Generic layout container.",
    },
    ComponentEntry {
        slug: "bulk-action-bar",
        display_name: "BulkActionBar",
        description: "Batch action bar.",
    },
    ComponentEntry {
        slug: "button",
        display_name: "Button",
        description: "Primary interactive control.",
    },
    ComponentEntry {
        slug: "calendar",
        display_name: "Calendar",
        description: "Date grid for picking a single date.",
    },
    ComponentEntry {
        slug: "callout",
        display_name: "Callout",
        description: "Highlighted informational block.",
    },
    ComponentEntry {
        slug: "card",
        display_name: "Card",
        description: "Contained surface for content.",
    },
    ComponentEntry {
        slug: "checkbox",
        display_name: "Checkbox",
        description: "Boolean toggle with label.",
    },
    ComponentEntry {
        slug: "code",
        display_name: "Code",
        description: "Syntax-highlighted code display.",
    },
    ComponentEntry {
        slug: "collapsible",
        display_name: "Collapsible",
        description: "Show/hide content toggle.",
    },
    ComponentEntry {
        slug: "color-picker",
        display_name: "ColorPicker",
        description: "Color selection with swatches.",
    },
    ComponentEntry {
        slug: "context-menu",
        display_name: "ContextMenu",
        description: "Right-click triggered menu.",
    },
    ComponentEntry {
        slug: "date-picker",
        display_name: "DatePicker",
        description: "Date selection with calendar.",
    },
    ComponentEntry {
        slug: "date-range-picker",
        display_name: "DateRangePicker",
        description: "Dual calendar date range.",
    },
    ComponentEntry {
        slug: "date-time-picker",
        display_name: "DateTimePicker",
        description: "Combined date and time.",
    },
    ComponentEntry {
        slug: "date-time-range-picker",
        display_name: "DateTimeRangePicker",
        description: "Date-time range selection.",
    },
    ComponentEntry {
        slug: "detail-item",
        display_name: "DetailItem",
        description: "Label-value metadata pair.",
    },
    ComponentEntry {
        slug: "dialog",
        display_name: "Dialog",
        description: "Modal overlay.",
    },
    ComponentEntry {
        slug: "drawer",
        display_name: "Drawer",
        description: "Slide-out panel from edge.",
    },
    ComponentEntry {
        slug: "duration-input",
        display_name: "DurationInput",
        description: "Segmented duration entry.",
    },
    ComponentEntry {
        slug: "editable-label",
        display_name: "EditableLabel",
        description: "Inline editable text.",
    },
    ComponentEntry {
        slug: "eyebrow",
        display_name: "Eyebrow",
        description: "Small uppercase label.",
    },
    ComponentEntry {
        slug: "field",
        display_name: "Field",
        description: "Form field wrapper.",
    },
    ComponentEntry {
        slug: "field-set",
        display_name: "FieldSet",
        description: "Groups related form controls with optional legend and multi-column layout.",
    },
    ComponentEntry {
        slug: "file-upload",
        display_name: "FileUpload",
        description: "File input with drag-and-drop.",
    },
    ComponentEntry {
        slug: "form-actions",
        display_name: "FormActions",
        description: "Action row for forms.",
    },
    ComponentEntry {
        slug: "grid",
        display_name: "Grid",
        description: "CSS Grid layout container.",
    },
    ComponentEntry {
        slug: "hover-card",
        display_name: "HoverCard",
        description: "Rich preview on hover.",
    },
    ComponentEntry {
        slug: "icon",
        display_name: "Icon",
        description: "SVG icon from registry.",
    },
    ComponentEntry {
        slug: "icon-button",
        display_name: "IconButton",
        description: "Icon-only button.",
    },
    ComponentEntry {
        slug: "icon-provider",
        display_name: "IconProvider",
        description: "Context boundary for descendant icons.",
    },
    ComponentEntry {
        slug: "list-card",
        display_name: "ListCard",
        description: "Structured list item card.",
    },
    ComponentEntry {
        slug: "menu",
        display_name: "Menu",
        description: "Dropdown menu.",
    },
    ComponentEntry {
        slug: "menubar",
        display_name: "Menubar",
        description: "Horizontal menu bar.",
    },
    ComponentEntry {
        slug: "meter",
        display_name: "Meter",
        description: "Visual gauge for values.",
    },
    ComponentEntry {
        slug: "meta-bar",
        display_name: "MetaBar",
        description: "Wrapping ribbon for inline metadata.",
    },
    ComponentEntry {
        slug: "meta-item",
        display_name: "MetaItem",
        description: "Compact labeled metadata value.",
    },
    ComponentEntry {
        slug: "nav-card",
        display_name: "NavCard",
        description: "Navigation-oriented card.",
    },
    ComponentEntry {
        slug: "navigation-menu",
        display_name: "NavigationMenu",
        description: "Navigation with dropdowns.",
    },
    ComponentEntry {
        slug: "number-input",
        display_name: "NumberInput",
        description: "Numeric input with controls.",
    },
    ComponentEntry {
        slug: "order-by",
        display_name: "OrderBy",
        description: "Sort-control toolbar.",
    },
    ComponentEntry {
        slug: "pagination",
        display_name: "Pagination",
        description: "Page navigation controls.",
    },
    ComponentEntry {
        slug: "pagination-summary",
        display_name: "PaginationSummary",
        description: "Pagination state summary.",
    },
    ComponentEntry {
        slug: "pill",
        display_name: "Pill",
        description: "Small inline label chip.",
    },
    ComponentEntry {
        slug: "password-requirements",
        display_name: "PasswordRequirements",
        description: "Live password policy checklist.",
    },
    ComponentEntry {
        slug: "code-input",
        display_name: "CodeInput",
        description: "Segmented code entry with optional masking.",
    },
    ComponentEntry {
        slug: "popover",
        display_name: "Popover",
        description: "Anchored overlay.",
    },
    ComponentEntry {
        slug: "progress",
        display_name: "Progress",
        description: "Progress indicator.",
    },
    ComponentEntry {
        slug: "radio-group",
        display_name: "RadioGroup",
        description: "Single-selection group.",
    },
    ComponentEntry {
        slug: "range-slider",
        display_name: "RangeSlider",
        description: "Dual-thumb slider.",
    },
    ComponentEntry {
        slug: "rating",
        display_name: "Rating",
        description: "Star-based rating.",
    },
    ComponentEntry {
        slug: "scroll-shell",
        display_name: "ScrollShell",
        description: "Scrollable container.",
    },
    ComponentEntry {
        slug: "segmented-control",
        display_name: "SegmentedControl",
        description: "Inline toggle options.",
    },
    ComponentEntry {
        slug: "select",
        display_name: "Select",
        description: "Dropdown selection.",
    },
    ComponentEntry {
        slug: "separator",
        display_name: "Separator",
        description: "Visual divider.",
    },
    ComponentEntry {
        slug: "skeleton",
        display_name: "Skeleton",
        description: "Placeholder loading shape.",
    },
    ComponentEntry {
        slug: "slider",
        display_name: "Slider",
        description: "Single-thumb slider.",
    },
    ComponentEntry {
        slug: "spinner",
        display_name: "Spinner",
        description: "Animated loading indicator.",
    },
    ComponentEntry {
        slug: "spacer",
        display_name: "Spacer",
        description: "Flexible space.",
    },
    ComponentEntry {
        slug: "split-button",
        display_name: "SplitButton",
        description: "Button with dropdown.",
    },
    ComponentEntry {
        slug: "stack",
        display_name: "Stack",
        description: "Vertical flex layout.",
    },
    ComponentEntry {
        slug: "status-indicator",
        display_name: "StatusIndicator",
        description: "Colored status dot.",
    },
    ComponentEntry {
        slug: "surface",
        display_name: "Surface",
        description: "Themed container.",
    },
    ComponentEntry {
        slug: "switch",
        display_name: "Switch",
        description: "Toggle switch.",
    },
    ComponentEntry {
        slug: "table",
        display_name: "Table",
        description: "Static data table.",
    },
    ComponentEntry {
        slug: "tabs",
        display_name: "Tabs",
        description: "Tabbed interface.",
    },
    ComponentEntry {
        slug: "text-input",
        display_name: "TextInput",
        description: "Single or multi-line text input.",
    },
    ComponentEntry {
        slug: "time-ago",
        display_name: "TimeAgo",
        description: "Relative timestamp.",
    },
    ComponentEntry {
        slug: "time-input",
        display_name: "TimeInput",
        description: "Time-of-day input.",
    },
    ComponentEntry {
        slug: "time-zone-select",
        display_name: "TimeZoneSelect",
        description: "Timezone selection.",
    },
    ComponentEntry {
        slug: "toggle-group",
        display_name: "ToggleGroup",
        description: "Mutually exclusive toggles.",
    },
    ComponentEntry {
        slug: "toolbar",
        display_name: "Toolbar",
        description: "Horizontal action bar.",
    },
    ComponentEntry {
        slug: "tooltip",
        display_name: "Tooltip",
        description: "Informational overlay.",
    },
    ComponentEntry {
        slug: "tri-state-switch",
        display_name: "TriStateSwitch",
        description: "Three-position switch.",
    },
    ComponentEntry {
        slug: "date-time-zone-picker",
        display_name: "DateTimeZonePicker",
        description: "Date-time with timezone.",
    },
];

pub static COMPOSITES: &[ComponentEntry] = &[
    ComponentEntry {
        slug: "audio-player",
        display_name: "AudioPlayer",
        description: "Audio playback controls.",
    },
    ComponentEntry {
        slug: "editable-list",
        display_name: "EditableList",
        description: "Self-managing editable list.",
    },
    ComponentEntry {
        slug: "block-editor",
        display_name: "BlockEditor",
        description: "Block-based content editor.",
    },
    ComponentEntry {
        slug: "breadcrumbs",
        display_name: "Breadcrumbs",
        description: "Navigation trail.",
    },
    ComponentEntry {
        slug: "card-radio-group",
        display_name: "CardRadioGroup",
        description: "Radio selection across cards.",
    },
    ComponentEntry {
        slug: "confirm-action",
        display_name: "ConfirmAction",
        description: "Confirmation before action.",
    },
    ComponentEntry {
        slug: "data-table",
        display_name: "DataTable",
        description: "Feature-rich table.",
    },
    ComponentEntry {
        slug: "embed-input",
        display_name: "EmbedInput",
        description: "URL or embed code input.",
    },
    ComponentEntry {
        slug: "embed-preview",
        display_name: "EmbedPreview",
        description: "Rich preview for embeds.",
    },
    ComponentEntry {
        slug: "empty-state",
        display_name: "EmptyState",
        description: "Placeholder for empty views.",
    },
    ComponentEntry {
        slug: "filter-toolbar",
        display_name: "FilterToolbar",
        description: "Filter controls toolbar.",
    },
    ComponentEntry {
        slug: "form-dialog",
        display_name: "FormDialog",
        description: "Modal dialog with form.",
    },
    ComponentEntry {
        slug: "form-layout",
        display_name: "FormLayout",
        description: "Structured layout for forms and validation messaging.",
    },
    ComponentEntry {
        slug: "list-container",
        display_name: "ListContainer",
        description: "Paginated list view with header and state handling.",
    },
    ComponentEntry {
        slug: "log-list",
        display_name: "LogList",
        description: "Timestamped log viewer.",
    },
    ComponentEntry {
        slug: "markdown-editor",
        display_name: "MarkdownEditor",
        description: "Markdown authoring with preview.",
    },
    ComponentEntry {
        slug: "media-browse-panel",
        display_name: "MediaBrowsePanel",
        description: "Grid of selectable media items.",
    },
    ComponentEntry {
        slug: "media-picker",
        display_name: "MediaPicker",
        description: "Media asset selection dialog.",
    },
    ComponentEntry {
        slug: "media-preview",
        display_name: "MediaPreview",
        description: "Media asset preview.",
    },
    ComponentEntry {
        slug: "media-thumbnail",
        display_name: "MediaThumbnail",
        description: "Compact media thumbnail.",
    },
    ComponentEntry {
        slug: "media-upload-status-panel",
        display_name: "MediaUploadStatusPanel",
        description: "Upload workflow status panel.",
    },
    ComponentEntry {
        slug: "page-header",
        display_name: "PageHeader",
        description: "Page-level header.",
    },
    ComponentEntry {
        slug: "page-loading",
        display_name: "PageLoading",
        description: "Full-viewport loading overlay.",
    },
    ComponentEntry {
        slug: "relation-picker",
        display_name: "RelationPicker",
        description: "Searchable related items picker.",
    },
    ComponentEntry {
        slug: "selection-summary",
        display_name: "SelectionSummary",
        description: "Current selection state.",
    },
    ComponentEntry {
        slug: "sidebar-nav",
        display_name: "SidebarNav",
        description: "Grouped vertical navigation for shells and catalogues.",
    },
    ComponentEntry {
        slug: "metric-tile",
        display_name: "MetricTile",
        description: "Compact label-value tile.",
    },
    ComponentEntry {
        slug: "toast-host",
        display_name: "ToastHost",
        description: "Host-owned notification surface with fixed placement.",
    },
    ComponentEntry {
        slug: "toast-stack",
        display_name: "ToastStack",
        description: "Stacked notification manager.",
    },
    ComponentEntry {
        slug: "video-player",
        display_name: "VideoPlayer",
        description: "Video playback controls.",
    },
];

pub static SHELLS: &[ComponentEntry] = &[
    ComponentEntry {
        slug: "action-discovery-panel",
        display_name: "ActionDiscoveryPanel",
        description: "Command-oriented discovery surface.",
    },
    ComponentEntry {
        slug: "app-header",
        display_name: "AppHeader",
        description: "Application header for workstation shells.",
    },
    ComponentEntry {
        slug: "collapse-toggle",
        display_name: "CollapseToggle",
        description: "Chevron toggle for collapsing adjacent regions.",
    },
    ComponentEntry {
        slug: "command-palette",
        display_name: "CommandPalette",
        description: "Modal command discovery surface.",
    },
    ComponentEntry {
        slug: "detail-section",
        display_name: "DetailSection",
        description: "Titled detail section.",
    },
    ComponentEntry {
        slug: "detail-shell",
        display_name: "DetailShell",
        description: "Full detail page layout.",
    },
    ComponentEntry {
        slug: "dock-region",
        display_name: "DockRegion",
        description: "Dockable workstation region scaffold.",
    },
    ComponentEntry {
        slug: "picker-shell",
        display_name: "PickerShell",
        description: "Search-and-select picker.",
    },
    ComponentEntry {
        slug: "resize-handle",
        display_name: "ResizeHandle",
        description: "Grab handle for adjacent resizable panels.",
    },
    ComponentEntry {
        slug: "split-view",
        display_name: "SplitView",
        description: "Two-pane layout with companion region.",
    },
    ComponentEntry {
        slug: "status-bar",
        display_name: "StatusBar",
        description: "Workstation footer status surface.",
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
        "box" | "grid" | "stack" | "spacer" | "separator" | "surface" | "scroll-shell"
        | "split-view" | "resize-handle" => ComponentTag::Layout,
        "eyebrow" | "pill" | "status-indicator" | "icon" | "icon-provider" | "skeleton"
        | "spinner" | "code" | "time-ago" | "metric-tile" | "detail-item" | "meta-bar"
        | "meta-item" | "embed-preview" => ComponentTag::Display,
        "dialog" | "alert-dialog" | "drawer" | "popover" | "hover-card" | "tooltip" | "menu"
        | "context-menu" | "menubar" | "confirm-action" | "form-dialog" | "command-palette" => {
            ComponentTag::Overlay
        }
        "tabs" | "breadcrumbs" | "pagination" | "pagination-summary" | "navigation-menu"
        | "sidebar-nav" | "nav-card" => ComponentTag::Navigation,
        "table" | "data-table" | "list-card" | "list-container" | "editable-list" | "card"
        | "card-radio-group" | "accordion" | "collapsible" | "order-by" | "selection-summary"
        | "filter-toolbar" | "log-list" | "relation-picker" | "picker-shell" => ComponentTag::Data,
        "audio-player"
        | "video-player"
        | "media-picker"
        | "media-browse-panel"
        | "media-preview"
        | "media-thumbnail"
        | "media-upload-status-panel" => ComponentTag::Media,
        "callout"
        | "progress"
        | "meter"
        | "empty-state"
        | "page-loading"
        | "toast-stack"
        | "toast-host"
        | "bulk-action-bar"
        | "password-requirements" => ComponentTag::Feedback,
        "field" | "field-set" | "form-actions" | "form-layout" | "block-editor"
        | "markdown-editor" => ComponentTag::Form,
        "app-header"
        | "page-header"
        | "status-bar"
        | "dock-region"
        | "toolbar"
        | "action-discovery-panel"
        | "detail-section"
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

pub fn implementation_root(slug: &str) -> &'static str {
    match slug {
        "accordion"
        | "alert-dialog"
        | "box"
        | "bulk-action-bar"
        | "button"
        | "calendar"
        | "callout"
        | "card"
        | "checkbox"
        | "code"
        | "collapsible"
        | "color-picker"
        | "context-menu"
        | "date-picker"
        | "date-range-picker"
        | "date-time-picker"
        | "date-time-range-picker"
        | "detail-item"
        | "dialog"
        | "drawer"
        | "duration-input"
        | "editable-label"
        | "eyebrow"
        | "field"
        | "field-set"
        | "file-upload"
        | "form-actions"
        | "grid"
        | "hover-card"
        | "icon"
        | "icon-button"
        | "icon-provider"
        | "list-card"
        | "menu"
        | "menubar"
        | "meter"
        | "meta-bar"
        | "meta-item"
        | "nav-card"
        | "navigation-menu"
        | "number-input"
        | "order-by"
        | "pagination"
        | "pagination-summary"
        | "pill"
        | "password-requirements"
        | "code-input"
        | "popover"
        | "progress"
        | "radio-group"
        | "range-slider"
        | "rating"
        | "scroll-shell"
        | "segmented-control"
        | "select"
        | "separator"
        | "skeleton"
        | "slider"
        | "spinner"
        | "spacer"
        | "split-button"
        | "stack"
        | "status-indicator"
        | "surface"
        | "switch"
        | "table"
        | "tabs"
        | "text-input"
        | "time-ago"
        | "time-input"
        | "time-zone-select"
        | "toggle-group"
        | "toolbar"
        | "tooltip"
        | "tri-state-switch"
        | "date-time-zone-picker" => "packages/gpui/components/src/primitives/",
        "audio-player"
        | "editable-list"
        | "block-editor"
        | "breadcrumbs"
        | "card-radio-group"
        | "confirm-action"
        | "data-table"
        | "embed-input"
        | "embed-preview"
        | "empty-state"
        | "filter-toolbar"
        | "form-dialog"
        | "form-layout"
        | "list-container"
        | "log-list"
        | "markdown-editor"
        | "media-browse-panel"
        | "media-picker"
        | "media-preview"
        | "media-thumbnail"
        | "media-upload-status-panel"
        | "page-header"
        | "page-loading"
        | "relation-picker"
        | "selection-summary"
        | "sidebar-nav"
        | "metric-tile"
        | "toast-host"
        | "toast-stack"
        | "video-player"
        | "action-discovery-panel"
        | "app-header"
        | "command-palette"
        | "detail-section"
        | "detail-shell"
        | "dock-region"
        | "picker-shell"
        | "resize-handle"
        | "split-view"
        | "status-bar" => "packages/gpui/components/src/composites/",
        _ => "packages/gpui/components/src/",
    }
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
