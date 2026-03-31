//! GPUI rendering adapter for Poodle.
//!
//! This crate implements the renderer adapter traits from `poodle-adapter` for
//! GPUI, Zed's native UI framework. It provides:
//!
//! - `GpuiThemeProvider`: Resolves token paths to typed values using GPUI's
//!   theme system (backed by `poodle-tokens::typed` constants).
//! - `GpuiTarget`: The render target type for GPUI output.
//! - `GpuiAdapter`: The main adapter struct implementing `AdapterManifest`.
//! - Style mapping utilities that convert `StyleDescriptor` → GPUI-native
//!   style properties.
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
//! poodle-gpui (this crate) — GPUI-specific implementations
//!     │
//!     ▼
//! GPUI native elements (Element, Style, events)
//! ```

pub mod demo_app;
mod render_action;
mod render_data_composites;
mod render_editing_composites;
mod render_form_composites;
mod render_informational;
mod render_overlay;
mod render_selection;
mod render_structural;
mod render_workstation;
mod style_map;
mod theme;

pub use style_map::{
    map_border, map_corner_radii, map_cursor, map_edges, map_layout, map_shadow, map_style,
    map_typography, GpuiAlignItems, GpuiColor, GpuiCornerRadii, GpuiCursorStyle, GpuiEdges,
    GpuiFlexDirection, GpuiFontFamily, GpuiJustifyContent, GpuiLength, GpuiOverflow, GpuiShadow,
    GpuiStyle, GpuiTypography,
};
pub use theme::GpuiThemeProvider;

use poodle_adapter::{AdapterManifest, RenderTarget};
pub use poodle_primitives::FieldRelationships;

/// Opaque handle to a rendered GPUI element.
///
/// In the real GPUI integration this would be `gpui::AnyElement` or a
/// `gpui::ElementId`. For now it wraps a string identifier so the adapter
/// can be tested and compiled without a GPUI runtime dependency.
#[derive(Debug, Clone, PartialEq)]
pub struct GpuiElementHandle {
    pub element_id: String,
    pub spec_type: &'static str,
}

impl GpuiElementHandle {
    pub fn new(element_id: impl Into<String>, spec_type: &'static str) -> Self {
        Self {
            element_id: element_id.into(),
            spec_type,
        }
    }
}

/// GPUI render target — produces `GpuiElementHandle` values.
pub struct GpuiTarget;

impl RenderTarget for GpuiTarget {
    type Handle = GpuiElementHandle;
}

/// The main GPUI rendering adapter.
///
/// Implements `AdapterManifest` to declare which Poodle component specs
/// this adapter supports. Individual `RenderComponent<Spec>` implementations
/// will be added in g07.002–009 as each component category is built out.
pub struct GpuiAdapter {
    theme: GpuiThemeProvider,
}

impl GpuiAdapter {
    pub fn new(theme: GpuiThemeProvider) -> Self {
        Self { theme }
    }

    pub fn theme(&self) -> &GpuiThemeProvider {
        &self.theme
    }
}

/// Primitive spec type names supported by the GPUI adapter.
///
/// This list grows as g07.002–006 milestones implement each category.
/// Initially empty — populated as rendering implementations land.
const SUPPORTED_PRIMITIVES: &[&str] = &[
    // g07.002 — structural and layout
    "BoxSpec",
    "StackSpec",
    "GridSpec",
    "SurfaceSpec",
    "SeparatorSpec",
    "ScrollShellSpec",
    "BannerSpec",
    "CallOutSpec",
    // g07.003 — action, text-entry, and field
    "ButtonSpec",
    "IconButtonSpec",
    "FieldSpec",
    "TextInputSpec",
    "TextAreaSpec",
    "SearchInputSpec",
    "FormActionsSpec",
    "TimeFieldSpec",
    "EditableLabelSpec",
    "NumberInputSpec",
    "CodeInputSpec",
    "ToolbarSpec",
    // g07.004 — selection, value, feedback, temporal
    "CheckboxSpec",
    "RadioGroupSpec",
    "SwitchSpec",
    "SelectSpec",
    "SegmentedControlSpec",
    "SliderSpec",
    "RangeSliderSpec",
    "ProgressSpec",
    "BadgeSpec",
    "StatusIndicatorSpec",
    "MeterSpec",
    "RatingSpec",
    "SkeletonSpec",
    "TriStateSwitchSpec",
    // g07.005 — overlay, disclosure, navigation, menu
    "AccordionSpec",
    "CollapsibleSpec",
    "DialogSpec",
    "DrawerSpec",
    "PopoverSpec",
    "TooltipSpec",
    "HoverCardSpec",
    "MenuSpec",
    "ContextMenuSpec",
    "TabsSpec",
    "TabStripSpec",
    "NavigationMenuSpec",
    "MenubarSpec",
    // g07.006 — informational, code, color, file, temporal
    "CodeSpec",
    "ColorPickerSpec",
    "FileUploadSpec",
    "EyebrowSpec",
    "PillSpec",
    "TimeAgoSpec",
    "DurationInputSpec",
    "TimeZoneSelectSpec",
    "ZonedDateTimePickerSpec",
    "SplitButtonSpec",
    "CalendarSpec",
    "DatePickerSpec",
    "DateRangePickerSpec",
    "DateTimePickerSpec",
    "DateTimeRangePickerSpec",
];

/// Composite spec type names supported by the GPUI adapter.
///
/// Populated in g07.007–009.
#[allow(dead_code)]
const SUPPORTED_COMPOSITES: &[&str] = &[
    // g07.007 — form, validation, and remediation
    "FormShellSpec",
    "ValidationSummarySpec",
    "RemediationBannerSpec",
    "InlineRemediationSpec",
    "ConfirmActionSpec",
    // g07.008 — data, browse, detail, and media
    "DataTableSpec",
    "DetailShellSpec",
    "DetailSectionSpec",
    "FilterToolbarSpec",
    "PickerShellSpec",
    "RelationPickerSpec",
    "SelectionSummarySpec",
    "PaginationSummarySpec",
    "MediaThumbnailSpec",
    "MediaPreviewSpec",
    // g07.009 — editing, navigation, list interaction, operational
    "AudioPlayerSpec",
    "VideoPlayerSpec",
    "MediaPickerSpec",
    "MarkdownEditorSpec",
    "BlockEditorSpec",
    "EmbedInputSpec",
    "EmbedPreviewSpec",
    "ReorderableListSpec",
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
    "EmptyStateSpec",
];

/// Workstation spec type names supported by the GPUI adapter.
///
/// Populated in g07.010.
#[allow(dead_code)]
const SUPPORTED_WORKSTATION: &[&str] = &[
    // g07.010 — workstation shell and layout
    "ActionDiscoveryPanelSpec",
    "AppHeaderSpec",
    "CommandPaletteSpec",
    "CommandPaletteShellSpec",
    "DockRegionSpec",
    "PanelHeaderSpec",
    "PanelSurfaceSpec",
    "PanelTabsSpec",
    "ProjectHeaderSpec",
    "ShellStatusBarSpec",
    "SplitViewSpec",
    "SurfaceTabsSpec",
    "WorkspaceShellSpec",
];

/// Components intentionally unsupported in GPUI (with reasons).
///
/// GPUI supports the full desktop interaction model, so very few
/// components are unsupported. This list documents any intentional gaps.
const UNSUPPORTED_COMPONENTS: &[(&str, &str)] = &[];

impl AdapterManifest for GpuiAdapter {
    fn name(&self) -> &str {
        "GPUI"
    }

    fn supported_components(&self) -> &[&str] {
        // Concatenate all three layers. In a real implementation this would
        // be a static slice built at compile time. For now we return the
        // primitives list (which will grow across g07 milestones).
        SUPPORTED_PRIMITIVES
    }

    fn unsupported_components(&self) -> &[(&str, &str)] {
        UNSUPPORTED_COMPONENTS
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::ThemeProvider;

    use super::*;

    #[test]
    fn gpui_target_produces_element_handles() {
        let handle = GpuiElementHandle::new("btn-1", "ButtonSpec");
        assert_eq!(handle.element_id, "btn-1");
        assert_eq!(handle.spec_type, "ButtonSpec");
    }

    #[test]
    fn gpui_adapter_reports_name_and_manifest() {
        let adapter = GpuiAdapter::new(GpuiThemeProvider::default());
        assert_eq!(adapter.name(), "GPUI");
        assert_eq!(adapter.unsupported_components().len(), 0);
    }

    #[test]
    fn gpui_adapter_exposes_theme() {
        let adapter = GpuiAdapter::new(GpuiThemeProvider::default());
        let color = adapter.theme().resolve_color("semantic.color.accent.base");
        // Default theme resolves from typed token constants
        assert!(color.0 >= 0.0 && color.0 <= 1.0);
    }
}
