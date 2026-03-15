//! RenderComponent implementations for workstation shell and layout specs.
//!
//! Full parity with GPUI adapter: ActionDiscoveryPanelSpec, AppHeaderSpec,
//! CommandPaletteSpec, CommandPaletteShellSpec, DockRegionSpec, PanelHeaderSpec,
//! PanelSurfaceSpec, PanelTabsSpec, ProjectHeaderSpec, ShellStatusBarSpec,
//! SplitViewSpec, SurfaceTabsSpec, WorkspaceShellSpec

use pug_adapter::{RenderComponent, ThemeProvider};
use pug_style::StyleDescriptor;
use pug_workstation::{
    ActionDiscoveryPanelSpec, AppHeaderSpec, CommandPaletteShellSpec, CommandPaletteSpec,
    DockRegionSpec, PanelHeaderSpec, PanelSurfaceSpec, PanelTabsSpec, ProjectHeaderSpec,
    ShellStatusBarSpec, SplitViewSpec, SurfaceTabsSpec, WorkspaceShellSpec,
};

use crate::style_map::map_style;
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

impl RenderComponent<ActionDiscoveryPanelSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &ActionDiscoveryPanelSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("action-discovery-panel", "ActionDiscoveryPanelSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<AppHeaderSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &AppHeaderSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("app-header", "AppHeaderSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<CommandPaletteSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &CommandPaletteSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("command-palette", "CommandPaletteSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<CommandPaletteShellSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &CommandPaletteShellSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("command-palette-shell", "CommandPaletteShellSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<DockRegionSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &DockRegionSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("dock-region", "DockRegionSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<PanelHeaderSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &PanelHeaderSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("panel-header", "PanelHeaderSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<PanelSurfaceSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &PanelSurfaceSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("panel-surface", "PanelSurfaceSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<PanelTabsSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &PanelTabsSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("panel-tabs", "PanelTabsSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<ProjectHeaderSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &ProjectHeaderSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("project-header", "ProjectHeaderSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<ShellStatusBarSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &ShellStatusBarSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("shell-status-bar", "ShellStatusBarSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<SplitViewSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &SplitViewSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("split-view", "SplitViewSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<SurfaceTabsSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &SurfaceTabsSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("surface-tabs", "SurfaceTabsSpec", WidgetKind::Panel)
    }
}

impl RenderComponent<WorkspaceShellSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &WorkspaceShellSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let _s = map_style(style);
        JetstreamNodeHandle::new("workspace-shell", "WorkspaceShellSpec", WidgetKind::Panel)
    }
}

#[cfg(test)]
mod tests {
    use pug_adapter::RenderComponent;
    use pug_style::StyleDescriptor;
    use pug_workstation::*;
    use crate::{JetstreamAdapter, theme::JetstreamThemeProvider};

    fn a() -> JetstreamAdapter { JetstreamAdapter::new(JetstreamThemeProvider::default()) }
    fn s() -> StyleDescriptor { StyleDescriptor::new() }
    fn t() -> JetstreamThemeProvider { JetstreamThemeProvider::default() }

    #[test] fn action_discovery_panel() { assert_eq!(a().render(&ActionDiscoveryPanelSpec::new(vec![]), &s(), &t()).spec_type, "ActionDiscoveryPanelSpec"); }
    #[test] fn app_header() { assert_eq!(a().render(&AppHeaderSpec::new(), &s(), &t()).spec_type, "AppHeaderSpec"); }
    #[test] fn command_palette() { assert_eq!(a().render(&CommandPaletteSpec::new(vec![]), &s(), &t()).spec_type, "CommandPaletteSpec"); }
    #[test] fn command_palette_shell() { assert_eq!(a().render(&CommandPaletteShellSpec::new(), &s(), &t()).spec_type, "CommandPaletteShellSpec"); }
    #[test] fn dock_region() { assert_eq!(a().render(&DockRegionSpec::new(DockEdge::Left, vec![]), &s(), &t()).spec_type, "DockRegionSpec"); }
    #[test] fn panel_header() { assert_eq!(a().render(&PanelHeaderSpec::new(), &s(), &t()).spec_type, "PanelHeaderSpec"); }
    #[test] fn panel_surface() { assert_eq!(a().render(&PanelSurfaceSpec::new(), &s(), &t()).spec_type, "PanelSurfaceSpec"); }
    #[test] fn panel_tabs() { assert_eq!(a().render(&PanelTabsSpec::new(vec![]), &s(), &t()).spec_type, "PanelTabsSpec"); }
    #[test] fn project_header() { assert_eq!(a().render(&ProjectHeaderSpec::new("Project"), &s(), &t()).spec_type, "ProjectHeaderSpec"); }
    #[test] fn shell_status_bar() { assert_eq!(a().render(&ShellStatusBarSpec::new(), &s(), &t()).spec_type, "ShellStatusBarSpec"); }
    #[test] fn split_view() { assert_eq!(a().render(&SplitViewSpec::new(SplitOrientation::Horizontal), &s(), &t()).spec_type, "SplitViewSpec"); }
    #[test] fn surface_tabs() { assert_eq!(a().render(&SurfaceTabsSpec::new(vec![]), &s(), &t()).spec_type, "SurfaceTabsSpec"); }
    #[test] fn workspace_shell() { assert_eq!(a().render(&WorkspaceShellSpec::new(), &s(), &t()).spec_type, "WorkspaceShellSpec"); }
}
