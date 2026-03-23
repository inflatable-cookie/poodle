use poodle_tokens::semantic;

use crate::types::{MediaKind, MediaState, RemediationAction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPreviewSpec {
    pub kind: MediaKind,
    pub state: MediaState,
    pub title: String,
    pub description: Option<String>,
    pub metadata: Vec<String>,
    pub footer_actions: Vec<RemediationAction>,
}

impl MediaPreviewSpec {
    pub fn new(kind: MediaKind, title: impl Into<String>) -> Self {
        Self {
            kind,
            state: MediaState::Ready,
            title: title.into(),
            description: None,
            metadata: Vec::new(),
            footer_actions: Vec::new(),
        }
    }

    pub fn with_state(mut self, state: MediaState) -> Self {
        self.state = state;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_metadata(mut self, metadata: Vec<String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_footer_actions(mut self, footer_actions: Vec<RemediationAction>) -> Self {
        self.footer_actions = footer_actions;
        self
    }

    pub fn metadata_count(&self) -> usize {
        self.metadata.len()
    }

    pub fn has_footer_actions(&self) -> bool {
        !self.footer_actions.is_empty()
    }

    pub fn shows_fallback_copy(&self) -> bool {
        self.state != MediaState::Ready
    }

    pub fn frame_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }
}
