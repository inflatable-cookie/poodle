//! Jetstream rendering adapter for Poodle.
//!
//! This crate implements the renderer adapter traits from `poodle-adapter` for
//! Jetstream, a wgpu-based game engine with a retained-mode UI system. It
//! provides:
//!
//! - `JetstreamThemeProvider`: Resolves token paths to typed values compatible
//!   with Jetstream's `Vec4` colors and `f32` pixel values.
//! - `JetstreamTarget`: The render target type for Jetstream output.
//! - `JetstreamAdapter`: The main adapter struct implementing `AdapterManifest`.
//! - Style mapping utilities that convert `StyleDescriptor` → Jetstream-native
//!   `UiStyle` properties.
//!
//! ## Architecture
//!
//! ```text
//! Shared Contract Layer (poodle-components)
//!     │
//!     ▼
//! poodle-adapter traits (ThemeProvider, RenderComponent, AdapterManifest)
//!     │
//!     ▼
//! poodle-jetstream (this crate) — Jetstream-specific implementations
//!     │
//!     ▼
//! Jetstream UI tree (UiNode, UiStyle, Widget, UiEvent)
//! ```
//!
//! ## Jetstream UI Model
//!
//! Jetstream uses a retained-mode UI tree (`UiTree`) where each node has:
//! - A `Widget` variant (Panel, Label, Button, Slider, ProgressBar, Image,
//!   List, TextInput)
//! - A `UiStyle` with flexbox layout, colors, borders, corner radius, opacity
//! - Parent-child relationships for layout computation
//! - Focus state for keyboard/gamepad navigation
//!
//! Poodle specs map to one or more `UiNode` entries. The adapter produces
//! `JetstreamNodeHandle` values that track the spec-to-node mapping.

pub mod demo_scene;
mod render_action;
mod render_composites;
mod render_feedback;
mod render_input;
mod render_overlay;
mod render_selection;
mod render_structural;
mod style_map;
mod theme;

pub use style_map::{
    map_layout, map_style, JetstreamBoxShadow, JetstreamColor, JetstreamEdges,
    JetstreamMappedStyle, JetstreamVisuals,
};
pub use theme::JetstreamThemeProvider;

use std::sync::LazyLock;

use poodle_adapter::{AdapterManifest, RenderTarget};
use poodle_node::NodeRole;

/// AccessKit / ARIA role the quarantined adapter projects for a node role.
/// Exhaustive so a new `NodeRole` cannot ship unmapped.
pub fn accesskit_role(role: NodeRole) -> &'static str {
    match role {
        NodeRole::Alert => "alert",
        NodeRole::AlertDialog => "alertdialog",
        NodeRole::Banner => "banner",
        NodeRole::Button => "button",
        NodeRole::Cell => "cell",
        NodeRole::CheckBox => "checkbox",
        NodeRole::ComboBox => "combobox",
        NodeRole::Dialog => "dialog",
        NodeRole::Grid => "grid",
        NodeRole::Group => "group",
        NodeRole::Heading => "heading",
        NodeRole::Label => "label",
        NodeRole::List => "list",
        NodeRole::ListItem => "listitem",
        NodeRole::ListBox => "listbox",
        NodeRole::ListBoxOption => "option",
        NodeRole::Log => "log",
        NodeRole::Image => "img",
        NodeRole::Menu => "menu",
        NodeRole::MenuBar => "menubar",
        NodeRole::MenuItem => "menuitem",
        NodeRole::MenuItemCheckBox => "menuitemcheckbox",
        NodeRole::MenuItemRadio => "menuitemradio",
        NodeRole::Splitter => "separator",
        NodeRole::Slider => "slider",
        NodeRole::ProgressIndicator => "progressbar",
        NodeRole::RadioGroup => "radiogroup",
        NodeRole::RadioButton => "radio",
        NodeRole::Region => "region",
        NodeRole::Row => "row",
        NodeRole::SpinButton => "spinbutton",
        NodeRole::Status => "status",
        NodeRole::Switch => "switch",
        NodeRole::Tab => "tab",
        NodeRole::TabList => "tablist",
        NodeRole::TabPanel => "tabpanel",
        NodeRole::TextInput => "textbox",
        NodeRole::Toolbar => "toolbar",
        NodeRole::Tooltip => "tooltip",
        NodeRole::Tree => "tree",
        NodeRole::TreeItem => "treeitem",
    }
}

/// Represents a Jetstream UI widget type that a spec maps to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WidgetKind {
    Panel,
    Label,
    Button,
    Slider,
    ProgressBar,
    Image,
    List,
    TextInput,
}

/// Handle to a rendered Jetstream UI node, carrying the fully resolved style.
///
/// Produced by `RenderComponent::render()` implementations. The `mapped` field
/// contains the resolved `taffy::Style` layout and `JetstreamVisuals` that the
/// preview bridge converts into actual `UiTree` nodes.
#[derive(Debug, Clone)]
pub struct JetstreamNodeHandle {
    pub node_id: String,
    pub spec_type: &'static str,
    pub widget_kind: WidgetKind,
    pub mapped: JetstreamMappedStyle,
}

impl JetstreamNodeHandle {
    pub fn new(
        node_id: impl Into<String>,
        spec_type: &'static str,
        widget_kind: WidgetKind,
        mapped: JetstreamMappedStyle,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            spec_type,
            widget_kind,
            mapped,
        }
    }
}

/// Jetstream render target — produces `JetstreamNodeHandle` values.
pub struct JetstreamTarget;

impl RenderTarget for JetstreamTarget {
    type Handle = JetstreamNodeHandle;
}

/// The main Jetstream rendering adapter.
pub struct JetstreamAdapter {
    theme: JetstreamThemeProvider,
}

impl JetstreamAdapter {
    pub fn new(theme: JetstreamThemeProvider) -> Self {
        Self { theme }
    }

    pub fn theme(&self) -> &JetstreamThemeProvider {
        &self.theme
    }
}

/// Spec types with direct `RenderComponent` implementations in this crate.
/// This is not the larger shared `poodle-render` preview surface.
/// Note: AccordionItemSpec is a sub-spec of AccordionSpec, not independently rendered.
const SUPPORTED_PRIMITIVES: &[&str] = &[
    // Structural
    "BoxSpec",
    "StackSpec",
    "GridSpec",
    "SurfaceSpec",
    "SeparatorSpec",
    "ScrollShellSpec",
    "CallOutSpec",
    // Action
    "ButtonSpec",
    "IconButtonSpec",
    "FormActionsSpec",
    "ToolbarSpec",
    // Input
    "TextInputSpec",
    "FieldSpec",
    "NumberInputSpec",
    "CodeInputSpec",
    "EditableLabelSpec",
    "TimeInputSpec",
    // Selection
    "CheckboxSpec",
    "RadioGroupSpec",
    "SwitchSpec",
    "SelectSpec",
    "SliderSpec",
    "RangeSliderSpec",
    "SegmentedControlSpec",
    "TriStateSwitchSpec",
    // Feedback and display
    "ProgressSpec",
    "BadgeSpec",
    "SpinnerSpec",
    "StatusIndicatorSpec",
    "SkeletonSpec",
    "MeterSpec",
    "RatingSpec",
    // Overlay
    "DialogSpec",
    "DrawerSpec",
    "PopoverSpec",
    "MenuSpec",
    "TooltipSpec",
    "TabsSpec",
    "AccordionSpec",
    "CollapsibleSpec",
    "HoverCardSpec",
    "ContextMenuSpec",
    "TabStripSpec",
    "NavigationMenuSpec",
    "MenubarSpec",
    // Informational and temporal
    "CodeSpec",
    "EyebrowSpec",
    "PillSpec",
    "TimeAgoSpec",
    "SplitButtonSpec",
    "ColorPickerSpec",
    "FileUploadSpec",
    "DurationInputSpec",
    "TimeZoneSelectSpec",
    "DateTimeZonePickerSpec",
    "CalendarSpec",
    "DatePickerSpec",
    "DateRangePickerSpec",
    "DateTimePickerSpec",
    "DateTimeRangePickerSpec",
];

/// Composite spec types with direct implementations in this crate.
const SUPPORTED_COMPOSITES: &[&str] = &[
    // Form and validation
    "FormShellSpec",
    "ValidationSummarySpec",
    "RemediationBannerSpec",
    "InlineRemediationSpec",
    "ConfirmActionSpec",
    // Data and browse
    "DataTableSpec",
    "DetailShellSpec",
    "DetailSectionSpec",
    "FilterBuilderSpec",
    "ThemeSelectSpec",
    "FilterToolbarSpec",
    "PickerShellSpec",
    "RelationPickerSpec",
    "SelectionSummarySpec",
    "PaginationSummarySpec",
    "MediaThumbnailSpec",
    "MediaPreviewSpec",
    "EmptyStateSpec",
    "ListContainerSpec",
    "MetricTileSpec",
    // Editing, media, navigation, list, operational
    "AudioPlayerSpec",
    "VideoPlayerSpec",
    "MediaPickerSpec",
    "MediaBrowsePanelSpec",
    "MarkdownEditorSpec",
    "BlockEditorSpec",
    "EmbedInputSpec",
    "EmbedPreviewSpec",
    "EditableListSpec",
    "BreadcrumbsSpec",
    "CardRadioGroupSpec",
    "ListCardSpec",
    "NavCardSpec",
    "OrderBySpec",
    "PageHeaderSpec",
    "PageLoadingSpec",
    "LogListSpec",
    "StateTileSpec",
    "ToastStackSpec",
    "ToastHostSpec",
    "SidebarNavSpec",
    "SplitViewSpec",
    "ShellStatusBarSpec",
    // Workstation composites registered here (moved from separate list)
    "ActionDiscoveryPanelSpec",
    "MessageCenterSpec",
    "AppHeaderSpec",
    "CommandPaletteSpec",
    "DockRegionSpec",
];

fn supported_components() -> &'static [&'static str] {
    static ALL: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
        SUPPORTED_PRIMITIVES
            .iter()
            .chain(SUPPORTED_COMPOSITES)
            .copied()
            .collect()
    });
    ALL.as_slice()
}

impl AdapterManifest for JetstreamAdapter {
    fn name(&self) -> &str {
        "Jetstream"
    }

    fn supported_components(&self) -> &[&str] {
        supported_components()
    }

    fn unsupported_components(&self) -> &[(&str, &str)] {
        &[]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;

    #[test]
    fn jetstream_target_produces_node_handles() {
        let handle = JetstreamNodeHandle::new(
            "btn-1",
            "ButtonSpec",
            WidgetKind::Button,
            JetstreamMappedStyle::default(),
        );
        assert_eq!(handle.node_id, "btn-1");
        assert_eq!(handle.spec_type, "ButtonSpec");
        assert_eq!(handle.widget_kind, WidgetKind::Button);
    }

    #[test]
    fn jetstream_adapter_reports_name_and_manifest() {
        let adapter = JetstreamAdapter::new(JetstreamThemeProvider::default());
        assert_eq!(adapter.name(), "Jetstream");
        assert_eq!(adapter.supported_components(), supported_components());
        assert_eq!(adapter.unsupported_components().len(), 0);
    }

    #[test]
    fn jetstream_adapter_exposes_theme() {
        let adapter = JetstreamAdapter::new(JetstreamThemeProvider::default());
        let color = adapter.theme().resolve_color("color.accent.base");
        assert!(color.0 >= 0.0 && color.0 <= 1.0);
    }

    /// Pins the direct implementation inventory. Shared-render coverage has its
    /// own preview and parity gates.
    #[test]
    fn direct_component_counts() {
        assert_eq!(SUPPORTED_PRIMITIVES.len(), 60);
        assert_eq!(SUPPORTED_COMPOSITES.len(), 48);
        assert_eq!(supported_components().len(), 108);
    }

    #[test]
    fn heading_and_banner_roles_map() {
        assert_eq!(accesskit_role(NodeRole::Heading), "heading");
        assert_eq!(accesskit_role(NodeRole::Banner), "banner");
    }
}
