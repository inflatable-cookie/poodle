//! GPUI demo app — exercises all 6 screen families from the shared demo-app contract.
//!
//! g07.012: Proves that the GPUI adapter can assemble complete screen layouts
//! from the shared spec layer. Each screen function renders a representative
//! set of components matching the Svelte demo app's coverage.
//!
//! Screen families:
//! 1. Overview shell — workspace summary with banners, state tiles, notifications
//! 2. Form and validation — form fields, validation summary, remediation
//! 3. Browse and filter — data table, filter toolbar, pagination
//! 4. Detail and review — detail shell, sections, media preview
//! 5. Picker and selection — picker shell, relation picker, selection summary
//! 6. Command and workspace — workspace shell, command palette, docks

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_composites::{
    ConfirmActionSpec, DataTableSpec, DetailSectionSpec, DetailShellSpec,
    EmptyStateSpec, FilterToolbarSpec, FormShellSpec, InlineRemediationSpec,
    LogListSpec, MediaPreviewSpec, MediaThumbnailSpec,
    PageHeaderSpec, PageLoadingSpec, PaginationSummarySpec, PickerShellSpec, RelationPickerSpec,
    RemediationBannerSpec, SelectionSummarySpec, StateTileSpec, ToastStackSpec,
    ValidationSummarySpec,
};
use poodle_primitives::{
    AccordionSpec, BadgeSpec, BannerSpec, BoxSpec, BreadcrumbsSpec, ButtonSpec, CheckboxSpec,
    FieldSpec, GridSpec, MenuSpec, NavCardGridSpec, NavCardSpec, ProgressSpec, SearchFieldSpec,
    SelectSpec, SeparatorSpec, SkeletonSpec, StackSpec, StatusIndicatorSpec, SurfaceSpec,
    SwitchSpec, TabsSpec, TextAreaSpec, TextInputSpec, ToolbarSpec,
};
use poodle_style::StyleDescriptor;
use poodle_workstation::{
    AppHeaderSpec, CommandPaletteShellSpec, CommandPaletteSpec, DockEdge, DockRegionSpec,
    PanelHeaderSpec, PanelSurfaceSpec, ProjectHeaderSpec, ShellStatusBarSpec, SplitOrientation,
    SplitViewSpec, WorkspaceShellSpec,
};

use crate::{GpuiAdapter, GpuiElementHandle};

/// Render result for a demo screen — collects all rendered element handles.
#[derive(Debug)]
pub struct DemoScreen {
    pub id: &'static str,
    pub title: &'static str,
    pub elements: Vec<GpuiElementHandle>,
}

impl DemoScreen {
    fn new(id: &'static str, title: &'static str) -> Self {
        Self {
            id,
            title,
            elements: Vec::new(),
        }
    }

    fn push(&mut self, handle: GpuiElementHandle) {
        self.elements.push(handle);
    }

    pub fn component_count(&self) -> usize {
        self.elements.len()
    }

    pub fn spec_types(&self) -> Vec<&'static str> {
        self.elements.iter().map(|e| e.spec_type).collect()
    }
}

/// Render all 6 demo screens, returning the assembled screen structures.
pub fn render_all_screens(adapter: &GpuiAdapter) -> Vec<DemoScreen> {
    let theme = adapter.theme();
    vec![
        render_overview_shell(adapter, theme),
        render_form_and_validation(adapter, theme),
        render_browse_and_filter(adapter, theme),
        render_detail_and_review(adapter, theme),
        render_picker_and_selection(adapter, theme),
        render_command_and_workspace(adapter, theme),
    ]
}

/// Screen 1: Overview shell — workspace summary with banners, tiles, notifications.
fn render_overview_shell(a: &GpuiAdapter, t: &dyn ThemeProvider) -> DemoScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoScreen::new("overview-shell", "Overview Shell");

    screen.push(a.render(&SurfaceSpec::new(), &s, t));
    screen.push(a.render(&StackSpec::new(), &s, t));
    screen.push(a.render(&PageHeaderSpec::new("Workspace Overview"), &s, t));
    screen.push(a.render(&BannerSpec::new(), &s, t));
    screen.push(a.render(&GridSpec::new(), &s, t));
    screen.push(a.render(&StateTileSpec::new("Active projects", "12"), &s, t));
    screen.push(a.render(&StateTileSpec::new("Pending reviews", "3"), &s, t));
    screen.push(a.render(&StateTileSpec::new("Open issues", "27"), &s, t));
    screen.push(a.render(&StatusIndicatorSpec::new(), &s, t));
    screen.push(a.render(&BadgeSpec::new(), &s, t));
    screen.push(a.render(&ProgressSpec::new(), &s, t));
    screen.push(a.render(&ToastStackSpec::new(), &s, t));
    screen.push(a.render(&NavCardGridSpec::new().with_columns(4), &s, t));
    screen.push(a.render(&NavCardSpec::new().with_title("Recent project"), &s, t));
    screen.push(a.render(&LogListSpec::new(), &s, t));

    screen
}

/// Screen 2: Form and validation — form fields, validation, remediation.
fn render_form_and_validation(a: &GpuiAdapter, t: &dyn ThemeProvider) -> DemoScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoScreen::new("form-and-validation", "Form & Validation");

    screen.push(a.render(&SurfaceSpec::new(), &s, t));
    screen.push(a.render(&FormShellSpec::new("demo-form"), &s, t));
    screen.push(a.render(&FieldSpec::new("title", "Title"), &s, t));
    screen.push(a.render(&TextInputSpec::new(), &s, t));
    screen.push(a.render(&TextAreaSpec::new(), &s, t));
    screen.push(a.render(&SelectSpec::new(vec![]), &s, t));
    screen.push(a.render(&CheckboxSpec::new(), &s, t));
    screen.push(a.render(&SwitchSpec::new(), &s, t));
    screen.push(a.render(&ButtonSpec::new(), &s, t));
    screen.push(a.render(&ValidationSummarySpec::new(vec![]), &s, t));
    screen.push(a.render(&RemediationBannerSpec::new("Fix required", "Please correct the errors."), &s, t));
    screen.push(a.render(&InlineRemediationSpec::new("Field is required."), &s, t));
    screen.push(a.render(&ConfirmActionSpec::new("Confirm", "Are you sure?", "Yes", "No"), &s, t));
    screen.push(a.render(&ToolbarSpec::new(), &s, t));

    screen
}

/// Screen 3: Browse and filter — data table, filters, pagination.
fn render_browse_and_filter(a: &GpuiAdapter, t: &dyn ThemeProvider) -> DemoScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoScreen::new("browse-and-filter", "Browse & Filter");

    screen.push(a.render(&SurfaceSpec::new(), &s, t));
    screen.push(a.render(&PageHeaderSpec::new("Asset Library"), &s, t));
    screen.push(a.render(&FilterToolbarSpec::new(), &s, t));
    screen.push(a.render(&SearchFieldSpec::new(), &s, t));
    screen.push(a.render(&DataTableSpec::new(vec![], vec![]), &s, t));
    screen.push(a.render(&PaginationSummarySpec::new(1, 25, 142), &s, t));
    screen.push(a.render(&EmptyStateSpec::new("No results found"), &s, t));
    screen.push(a.render(&SkeletonSpec::new(), &s, t));
    screen.push(a.render(&PageLoadingSpec::new(), &s, t));
    screen.push(a.render(&TabsSpec::new(vec![]), &s, t));
    screen.push(a.render(&SeparatorSpec::new(), &s, t));

    screen
}

/// Screen 4: Detail and review — detail shell, sections, media.
fn render_detail_and_review(a: &GpuiAdapter, t: &dyn ThemeProvider) -> DemoScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoScreen::new("detail-and-review", "Detail & Review");

    screen.push(a.render(&SurfaceSpec::new(), &s, t));
    screen.push(a.render(&DetailShellSpec::new(), &s, t));
    screen.push(a.render(&DetailSectionSpec::new().with_title("Overview"), &s, t));
    screen.push(a.render(&DetailSectionSpec::new().with_title("Media"), &s, t));
    screen.push(a.render(&MediaThumbnailSpec::new(poodle_composites::MediaKind::Image), &s, t));
    screen.push(a.render(&MediaPreviewSpec::new(poodle_composites::MediaKind::Audio, "Track preview"), &s, t));
    screen.push(a.render(&BreadcrumbsSpec::new(vec![]), &s, t));
    screen.push(a.render(&AccordionSpec::new(vec![]), &s, t));
    screen.push(a.render(&BoxSpec::new(), &s, t));
    screen.push(a.render(&MenuSpec::new(vec![]), &s, t));

    screen
}

/// Screen 5: Picker and selection — picker shell, relation picker, summary.
fn render_picker_and_selection(a: &GpuiAdapter, t: &dyn ThemeProvider) -> DemoScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoScreen::new("picker-and-selection", "Picker & Selection");

    screen.push(a.render(&SurfaceSpec::new(), &s, t));
    screen.push(a.render(&PickerShellSpec::new("Select assets"), &s, t));
    screen.push(a.render(&RelationPickerSpec::new(vec![]), &s, t));
    screen.push(a.render(&SelectionSummarySpec::new(vec![]), &s, t));
    screen.push(a.render(&SearchFieldSpec::new(), &s, t));
    screen.push(a.render(&CheckboxSpec::new(), &s, t));
    screen.push(a.render(&ButtonSpec::new(), &s, t));

    screen
}

/// Screen 6: Command and workspace — workspace shell, command palette, docks.
fn render_command_and_workspace(a: &GpuiAdapter, t: &dyn ThemeProvider) -> DemoScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoScreen::new("command-and-workspace", "Command & Workspace");

    screen.push(a.render(&WorkspaceShellSpec::new(), &s, t));
    screen.push(a.render(&AppHeaderSpec::new(), &s, t));
    screen.push(a.render(&ProjectHeaderSpec::new("Demo Project"), &s, t));
    screen.push(a.render(&CommandPaletteShellSpec::new(), &s, t));
    screen.push(a.render(&CommandPaletteSpec::new(vec![]), &s, t));
    screen.push(a.render(&DockRegionSpec::new(DockEdge::Left, vec![]), &s, t));
    screen.push(a.render(&DockRegionSpec::new(DockEdge::Right, vec![]), &s, t));
    screen.push(a.render(&SplitViewSpec::new(SplitOrientation::Horizontal), &s, t));
    screen.push(a.render(&PanelSurfaceSpec::new(), &s, t));
    screen.push(a.render(&PanelHeaderSpec::new(), &s, t));
    screen.push(a.render(&ShellStatusBarSpec::new(), &s, t));

    screen
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::GpuiThemeProvider;

    fn adapter() -> GpuiAdapter {
        GpuiAdapter::new(GpuiThemeProvider::default())
    }

    #[test]
    fn all_six_screens_render_without_panic() {
        let screens = render_all_screens(&adapter());
        assert_eq!(screens.len(), 6);
        for screen in &screens {
            assert!(screen.component_count() > 0, "Screen '{}' has no components", screen.id);
        }
    }

    #[test]
    fn overview_shell_covers_summary_components() {
        let screen = render_overview_shell(&adapter(), adapter().theme());
        let types = screen.spec_types();
        assert!(types.contains(&"StateTileSpec"));
        assert!(types.contains(&"BannerSpec"));
        assert!(types.contains(&"ProgressSpec"));
        assert!(types.contains(&"NavCardGridSpec"));
        assert!(screen.component_count() >= 10);
    }

    #[test]
    fn form_screen_covers_input_and_validation() {
        let screen = render_form_and_validation(&adapter(), adapter().theme());
        let types = screen.spec_types();
        assert!(types.contains(&"FormShellSpec"));
        assert!(types.contains(&"TextInputSpec"));
        assert!(types.contains(&"ValidationSummarySpec"));
        assert!(types.contains(&"RemediationBannerSpec"));
        assert!(screen.component_count() >= 10);
    }

    #[test]
    fn browse_screen_covers_table_and_filter() {
        let screen = render_browse_and_filter(&adapter(), adapter().theme());
        let types = screen.spec_types();
        assert!(types.contains(&"DataTableSpec"));
        assert!(types.contains(&"FilterToolbarSpec"));
        assert!(types.contains(&"PaginationSummarySpec"));
        assert!(screen.component_count() >= 8);
    }

    #[test]
    fn detail_screen_covers_shell_and_media() {
        let screen = render_detail_and_review(&adapter(), adapter().theme());
        let types = screen.spec_types();
        assert!(types.contains(&"DetailShellSpec"));
        assert!(types.contains(&"DetailSectionSpec"));
        assert!(types.contains(&"MediaThumbnailSpec"));
        assert!(screen.component_count() >= 8);
    }

    #[test]
    fn picker_screen_covers_selection() {
        let screen = render_picker_and_selection(&adapter(), adapter().theme());
        let types = screen.spec_types();
        assert!(types.contains(&"PickerShellSpec"));
        assert!(types.contains(&"RelationPickerSpec"));
        assert!(types.contains(&"SelectionSummarySpec"));
        assert!(screen.component_count() >= 5);
    }

    #[test]
    fn workspace_screen_covers_shell_and_docks() {
        let screen = render_command_and_workspace(&adapter(), adapter().theme());
        let types = screen.spec_types();
        assert!(types.contains(&"WorkspaceShellSpec"));
        assert!(types.contains(&"CommandPaletteSpec"));
        assert!(types.contains(&"DockRegionSpec"));
        assert!(types.contains(&"SplitViewSpec"));
        assert!(screen.component_count() >= 8);
    }

    #[test]
    fn total_component_coverage_across_all_screens() {
        let screens = render_all_screens(&adapter());
        let total: usize = screens.iter().map(|s| s.component_count()).sum();
        // At least 60 component renders across all 6 screens
        assert!(total >= 60, "Total component count {total} is below expected minimum");
    }
}
