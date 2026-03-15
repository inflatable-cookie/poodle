//! PugWorkspaceShell — real GPUI component backed by WorkspaceShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::WorkspaceShellSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI workspace shell backed by `WorkspaceShellSpec`.
///
/// The top-level application container that assembles the app header,
/// project header, surface tabs, main content area, status bar, and
/// utility overlays into a full workspace layout.
pub struct PugWorkspaceShell {
    spec: WorkspaceShellSpec,
    theme: GpuiThemeProvider,
    /// Slot for the app header (rendered at the very top).
    app_header: Option<AnyElement>,
    /// Slot for the project header (below app header).
    project_header: Option<AnyElement>,
    /// Slot for surface tabs (below project header).
    surface_tabs: Option<AnyElement>,
    /// Slot for the main workspace content (split views, panels).
    content: Option<AnyElement>,
    /// Slot for the status bar (bottom).
    status_bar: Option<AnyElement>,
    /// Slot for utility overlays (command palette, drawers).
    utility_overlays: Option<AnyElement>,
}

impl PugWorkspaceShell {
    pub fn new(spec: WorkspaceShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            app_header: None,
            project_header: None,
            surface_tabs: None,
            content: None,
            status_bar: None,
            utility_overlays: None,
        }
    }

    pub fn with_app_header(mut self, header: impl IntoElement) -> Self {
        self.app_header = Some(header.into_any_element());
        self
    }

    pub fn with_project_header(mut self, header: impl IntoElement) -> Self {
        self.project_header = Some(header.into_any_element());
        self
    }

    pub fn with_surface_tabs(mut self, tabs: impl IntoElement) -> Self {
        self.surface_tabs = Some(tabs.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn with_status_bar(mut self, bar: impl IntoElement) -> Self {
        self.status_bar = Some(bar.into_any_element());
        self
    }

    pub fn with_utility_overlays(mut self, overlays: impl IntoElement) -> Self {
        self.utility_overlays = Some(overlays.into_any_element());
        self
    }
}

impl IntoElement for PugWorkspaceShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let bg = resolve_color(theme, spec.background_token());

        let mut shell = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(bg);

        if spec.is_disabled {
            shell = shell.opacity(0.5);
        }

        // App header
        if spec.has_app_header {
            if let Some(app_header) = self.app_header {
                shell = shell.child(app_header);
            }
        }

        // Project header
        if spec.has_project_header {
            if let Some(project_header) = self.project_header {
                shell = shell.child(project_header);
            }
        }

        // Surface tabs
        if spec.has_surface_tabs {
            if let Some(surface_tabs) = self.surface_tabs {
                shell = shell.child(surface_tabs);
            }
        }

        // Main content area (takes remaining space)
        let mut main_area = div().flex_1().overflow_hidden();
        if let Some(content) = self.content {
            main_area = main_area.child(content);
        }
        shell = shell.child(main_area);

        // Status bar
        if let Some(status_bar) = self.status_bar {
            shell = shell.child(status_bar);
        }

        // Utility overlays (positioned absolutely over the shell)
        if spec.has_utility_overlays {
            if let Some(utility_overlays) = self.utility_overlays {
                shell = shell.child(utility_overlays);
            }
        }

        shell.into_any_element()
    }
}
