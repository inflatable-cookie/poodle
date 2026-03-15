use pug_tokens::semantic;

use crate::types::DrawerEdge;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrawerSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub edge: DrawerEdge,
    pub is_modal: bool,
    pub dismiss_on_escape: bool,
    pub dismiss_on_backdrop: bool,
    pub aria_label: Option<String>,
}

impl Default for DrawerSpec {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            title: None,
            description: None,
            edge: DrawerEdge::Right,
            is_modal: true,
            dismiss_on_escape: true,
            dismiss_on_backdrop: true,
            aria_label: None,
        }
    }
}

impl DrawerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_edge(mut self, edge: DrawerEdge) -> Self {
        self.edge = edge;
        self
    }

    pub fn with_modal(mut self, is_modal: bool) -> Self {
        self.is_modal = is_modal;
        self
    }

    pub fn with_dismiss_on_escape(mut self, dismiss_on_escape: bool) -> Self {
        self.dismiss_on_escape = dismiss_on_escape;
        self
    }

    pub fn with_dismiss_on_backdrop(mut self, dismiss_on_backdrop: bool) -> Self {
        self.dismiss_on_backdrop = dismiss_on_backdrop;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn shows_backdrop(&self) -> bool {
        self.is_modal && self.current_open()
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn backdrop_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_OVERLAY
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_DIALOG
    }
}
