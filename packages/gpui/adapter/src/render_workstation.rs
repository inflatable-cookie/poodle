//! RenderComponent implementations for workstation shell and layout specs.
//!
//! g07.010: ActionDiscoveryPanelSpec, AppHeaderSpec, CommandPaletteSpec,
//! CommandPaletteShellSpec, DockRegionSpec, PanelHeaderSpec, PanelSurfaceSpec,
//! PanelTabsSpec, ProjectHeaderSpec, ShellStatusBarSpec, SplitViewSpec,
//! SurfaceTabsSpec, WorkspaceShellSpec

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_style::StyleDescriptor;
use poodle_workstation::{
    ActionDiscoveryPanelSpec, AppHeaderSpec, CommandPaletteShellSpec, CommandPaletteSpec,
    DockRegionSpec, PanelHeaderSpec, PanelSurfaceSpec, PanelTabsSpec, ProjectHeaderSpec,
    ShellStatusBarSpec, SplitViewSpec, SurfaceTabsSpec, WorkspaceShellSpec,
};

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<ActionDiscoveryPanelSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ActionDiscoveryPanelSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("action-discovery-panel", "ActionDiscoveryPanelSpec")
    }
}

impl RenderComponent<AppHeaderSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &AppHeaderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("app-header", "AppHeaderSpec")
    }
}

impl RenderComponent<CommandPaletteSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &CommandPaletteSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("command-palette", "CommandPaletteSpec")
    }
}

impl RenderComponent<CommandPaletteShellSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &CommandPaletteShellSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("command-palette-shell", "CommandPaletteShellSpec")
    }
}

impl RenderComponent<DockRegionSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &DockRegionSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("dock-region", "DockRegionSpec")
    }
}

impl RenderComponent<PanelHeaderSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &PanelHeaderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("panel-header", "PanelHeaderSpec")
    }
}

impl RenderComponent<PanelSurfaceSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &PanelSurfaceSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("panel-surface", "PanelSurfaceSpec")
    }
}

impl RenderComponent<PanelTabsSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &PanelTabsSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("panel-tabs", "PanelTabsSpec")
    }
}

impl RenderComponent<ProjectHeaderSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ProjectHeaderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("project-header", "ProjectHeaderSpec")
    }
}

impl RenderComponent<ShellStatusBarSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ShellStatusBarSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("shell-status-bar", "ShellStatusBarSpec")
    }
}

impl RenderComponent<SplitViewSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &SplitViewSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("split-view", "SplitViewSpec")
    }
}

impl RenderComponent<SurfaceTabsSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &SurfaceTabsSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("surface-tabs", "SurfaceTabsSpec")
    }
}

impl RenderComponent<WorkspaceShellSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &WorkspaceShellSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("workspace-shell", "WorkspaceShellSpec")
    }
}

#[cfg(test)]
mod tests {
    use crate::{theme::GpuiThemeProvider, GpuiAdapter};
    use poodle_adapter::RenderComponent;
    use poodle_style::StyleDescriptor;
    use poodle_workstation::*;

    fn a() -> GpuiAdapter {
        GpuiAdapter::new(GpuiThemeProvider::default())
    }
    fn s() -> StyleDescriptor {
        StyleDescriptor::new()
    }
    fn t() -> GpuiThemeProvider {
        GpuiThemeProvider::default()
    }

    #[test]
    fn action_discovery_panel() {
        assert_eq!(
            a().render(&ActionDiscoveryPanelSpec::new(vec![]), &s(), &t())
                .spec_type,
            "ActionDiscoveryPanelSpec"
        );
    }
    #[test]
    fn app_header() {
        assert_eq!(
            a().render(&AppHeaderSpec::new(), &s(), &t()).spec_type,
            "AppHeaderSpec"
        );
    }
    #[test]
    fn command_palette() {
        assert_eq!(
            a().render(&CommandPaletteSpec::new(vec![]), &s(), &t())
                .spec_type,
            "CommandPaletteSpec"
        );
    }
    #[test]
    fn command_palette_shell() {
        assert_eq!(
            a().render(&CommandPaletteShellSpec::new(), &s(), &t())
                .spec_type,
            "CommandPaletteShellSpec"
        );
    }
    #[test]
    fn dock_region() {
        assert_eq!(
            a().render(&DockRegionSpec::new(DockEdge::Left, vec![]), &s(), &t())
                .spec_type,
            "DockRegionSpec"
        );
    }
    #[test]
    fn panel_header() {
        assert_eq!(
            a().render(&PanelHeaderSpec::new(), &s(), &t()).spec_type,
            "PanelHeaderSpec"
        );
    }
    #[test]
    fn panel_surface() {
        assert_eq!(
            a().render(&PanelSurfaceSpec::new(), &s(), &t()).spec_type,
            "PanelSurfaceSpec"
        );
    }
    #[test]
    fn panel_tabs() {
        assert_eq!(
            a().render(&PanelTabsSpec::new(vec![]), &s(), &t())
                .spec_type,
            "PanelTabsSpec"
        );
    }
    #[test]
    fn project_header() {
        assert_eq!(
            a().render(&ProjectHeaderSpec::new("Project"), &s(), &t())
                .spec_type,
            "ProjectHeaderSpec"
        );
    }
    #[test]
    fn shell_status_bar() {
        assert_eq!(
            a().render(&ShellStatusBarSpec::new(), &s(), &t()).spec_type,
            "ShellStatusBarSpec"
        );
    }
    #[test]
    fn split_view() {
        assert_eq!(
            a().render(
                &SplitViewSpec::new(SplitOrientation::Horizontal),
                &s(),
                &t()
            )
            .spec_type,
            "SplitViewSpec"
        );
    }
    #[test]
    fn surface_tabs() {
        assert_eq!(
            a().render(&SurfaceTabsSpec::new(vec![]), &s(), &t())
                .spec_type,
            "SurfaceTabsSpec"
        );
    }
    #[test]
    fn workspace_shell() {
        assert_eq!(
            a().render(&WorkspaceShellSpec::new(), &s(), &t()).spec_type,
            "WorkspaceShellSpec"
        );
    }
}
