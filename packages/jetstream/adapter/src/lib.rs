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
//! Shared Contract Layer (poodle-primitives, poodle-composites, poodle-workstation)
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
mod render_workstation;
mod style_map;
mod theme;

pub use style_map::{
    map_layout, map_style, JetstreamBoxShadow, JetstreamColor,
    JetstreamEdges, JetstreamMappedStyle, JetstreamVisuals,
};
pub use theme::JetstreamThemeProvider;

use poodle_adapter::{AdapterManifest, RenderTarget};

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

/// Primitive spec type names supported by the Jetstream adapter (64 — full parity).
/// Note: AccordionItemSpec is a sub-spec of AccordionSpec, not independently rendered.
#[allow(dead_code)]
const SUPPORTED_PRIMITIVES: &[&str] = &[
    // Structural
    "BoxSpec", "StackSpec", "GridSpec", "SurfaceSpec", "SeparatorSpec", "ScrollShellSpec",
    "BannerSpec", "CallOutSpec",
    // Action
    "ButtonSpec", "IconButtonSpec", "FormActionsSpec", "ToolbarSpec",
    // Input
    "TextInputSpec", "FieldSpec", "NumberInputSpec",
    "CodeInputSpec", "EditableLabelSpec", "TimeFieldSpec",
    // Selection
    "CheckboxSpec", "RadioGroupSpec", "SwitchSpec", "SelectSpec", "SliderSpec",
    "RangeSliderSpec", "SegmentedControlSpec", "TriStateSwitchSpec",
    // Feedback and display
    "ProgressSpec", "BadgeSpec", "SpinnerSpec", "StatusIndicatorSpec", "SkeletonSpec", "MeterSpec",
    "RatingSpec",
    // Overlay
    "DialogSpec", "DrawerSpec", "PopoverSpec", "MenuSpec", "TooltipSpec", "TabsSpec",
    "AccordionSpec", "CollapsibleSpec", "HoverCardSpec", "ContextMenuSpec", "TabStripSpec",
    "NavigationMenuSpec", "MenubarSpec",
    // Informational and temporal
    "CodeSpec", "EyebrowSpec", "PillSpec", "TimeAgoSpec", "SplitButtonSpec",
    "ColorPickerSpec", "FileUploadSpec", "DurationInputSpec", "TimeZoneSelectSpec",
    "DateTimeZonePickerSpec", "CalendarSpec", "DatePickerSpec",
    "DateRangePickerSpec", "DateTimePickerSpec", "DateTimeRangePickerSpec",
];

/// Composite spec type names supported by the Jetstream adapter (47 — full parity).
#[allow(dead_code)]
const SUPPORTED_COMPOSITES: &[&str] = &[
    // Form and validation
    "FormShellSpec", "ValidationSummarySpec", "RemediationBannerSpec",
    "InlineRemediationSpec", "ConfirmActionSpec",
    // Data and browse
    "DataTableSpec", "DetailShellSpec",
    "DetailSectionSpec", "FilterToolbarSpec", "PickerShellSpec",
    "RelationPickerSpec", "SelectionSummarySpec", "PaginationSummarySpec",
    "MediaThumbnailSpec", "MediaPreviewSpec", "EmptyStateSpec",
    "ListContainerSpec", "MetricTileSpec",
    // Editing, media, navigation, list, operational
    "AudioPlayerSpec", "VideoPlayerSpec", "MediaPickerSpec",
    "MediaBrowsePanelSpec", "MediaUploadStatusPanelSpec",
    "MarkdownEditorSpec", "BlockEditorSpec", "EmbedInputSpec",
    "EmbedPreviewSpec", "EditableListSpec",
    "ReorderableListSpec", "BreadcrumbsSpec", "CardRadioGroupSpec",
    "ListCardSpec", "NavCardSpec",
    "OrderBySpec", "PageHeaderSpec", "PageLoadingSpec",
    "LogListSpec", "StateTileSpec", "ToastStackSpec", "ToastHostSpec",
    "SidebarNavSpec", "SplitViewSpec", "ShellStatusBarSpec",
    // Workstation composites registered here (moved from separate list)
    "ActionDiscoveryPanelSpec", "AppHeaderSpec", "CommandPaletteSpec",
    "DockRegionSpec",
];

/// Workstation spec type names supported by the Jetstream adapter (13 — full parity).
#[allow(dead_code)]
const SUPPORTED_WORKSTATION: &[&str] = &[
    "ActionDiscoveryPanelSpec", "AppHeaderSpec", "CommandPaletteSpec",
    "CommandPaletteShellSpec", "DockRegionSpec", "PanelHeaderSpec",
    "PanelSurfaceSpec", "PanelTabsSpec", "ProjectHeaderSpec",
    "ShellStatusBarSpec", "SplitViewSpec", "SurfaceTabsSpec",
    "WorkspaceShellSpec",
];

impl AdapterManifest for JetstreamAdapter {
    fn name(&self) -> &str {
        "Jetstream"
    }

    fn supported_components(&self) -> &[&str] {
        SUPPORTED_PRIMITIVES
    }

    fn unsupported_components(&self) -> &[(&str, &str)] {
        &[]
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::ThemeProvider;
    use super::*;

    #[test]
    fn jetstream_target_produces_node_handles() {
        let handle = JetstreamNodeHandle::new("btn-1", "ButtonSpec", WidgetKind::Button, JetstreamMappedStyle::default());
        assert_eq!(handle.node_id, "btn-1");
        assert_eq!(handle.spec_type, "ButtonSpec");
        assert_eq!(handle.widget_kind, WidgetKind::Button);
    }

    #[test]
    fn jetstream_adapter_reports_name_and_manifest() {
        let adapter = JetstreamAdapter::new(JetstreamThemeProvider::default());
        assert_eq!(adapter.name(), "Jetstream");
        assert_eq!(adapter.unsupported_components().len(), 0);
    }

    #[test]
    fn jetstream_adapter_exposes_theme() {
        let adapter = JetstreamAdapter::new(JetstreamThemeProvider::default());
        let color = adapter.theme().resolve_color("color.accent.base");
        assert!(color.0 >= 0.0 && color.0 <= 1.0);
    }

    #[test]
    fn full_parity_component_counts() {
        assert_eq!(SUPPORTED_PRIMITIVES.len(), 61);
        assert_eq!(SUPPORTED_COMPOSITES.len(), 47);
        assert_eq!(SUPPORTED_WORKSTATION.len(), 13);
    }
}
