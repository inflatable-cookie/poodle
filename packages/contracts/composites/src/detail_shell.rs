use pug_tokens::semantic;

use crate::types::ScrollOwner;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetailState {
    Ready,
    Empty,
    Loading,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailShellSpec {
    pub title: Option<String>,
    pub scroll_owner: ScrollOwner,
    pub state: DetailState,
    pub aria_label: Option<String>,
}

impl Default for DetailShellSpec {
    fn default() -> Self {
        Self {
            title: None,
            scroll_owner: ScrollOwner::Content,
            state: DetailState::Ready,
            aria_label: None,
        }
    }
}

impl DetailShellSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_scroll_owner(mut self, scroll_owner: ScrollOwner) -> Self {
        self.scroll_owner = scroll_owner;
        self
    }

    pub fn with_state(mut self, state: DetailState) -> Self {
        self.state = state;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn has_ready_content(&self) -> bool {
        self.state == DetailState::Ready
    }

    pub fn body_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }
}
