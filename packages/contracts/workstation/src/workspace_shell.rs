use poodle_tokens::semantic;

use crate::types::WorkspaceShellState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkspaceShellSpec {
    pub is_disabled: bool,
    pub active_surface_label: Option<String>,
    pub aria_label: Option<String>,
    pub state: WorkspaceShellState,
    pub has_app_header: bool,
    pub has_project_header: bool,
    pub has_surface_tabs: bool,
    pub has_utility_overlays: bool,
}

impl Default for WorkspaceShellSpec {
    fn default() -> Self {
        Self {
            is_disabled: false,
            active_surface_label: None,
            aria_label: None,
            state: WorkspaceShellState::Ready,
            has_app_header: false,
            has_project_header: false,
            has_surface_tabs: false,
            has_utility_overlays: false,
        }
    }
}

impl WorkspaceShellSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_active_surface_label(mut self, active_surface_label: impl Into<String>) -> Self {
        self.active_surface_label = Some(active_surface_label.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_state(mut self, state: WorkspaceShellState) -> Self {
        self.state = state;
        self
    }

    pub fn with_app_header(mut self, has_app_header: bool) -> Self {
        self.has_app_header = has_app_header;
        self
    }

    pub fn with_project_header(mut self, has_project_header: bool) -> Self {
        self.has_project_header = has_project_header;
        self
    }

    pub fn with_surface_tabs(mut self, has_surface_tabs: bool) -> Self {
        self.has_surface_tabs = has_surface_tabs;
        self
    }

    pub fn with_utility_overlays(mut self, has_utility_overlays: bool) -> Self {
        self.has_utility_overlays = has_utility_overlays;
        self
    }

    pub fn is_multi_surface(&self) -> bool {
        self.has_surface_tabs
    }

    pub fn is_utility_open(&self) -> bool {
        self.has_utility_overlays
    }

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_CANVAS
    }
}
