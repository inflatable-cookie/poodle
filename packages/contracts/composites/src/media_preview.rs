use crate::types::{AspectRatio, MediaKind, MediaState, RemediationAction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPreviewSpec {
    pub kind: MediaKind,
    pub state: MediaState,
    pub aspect_ratio: AspectRatio,
    pub title: String,
    pub description: Option<String>,
    pub badge: Option<String>,
    pub thumbnail_meta: Option<String>,
    pub state_title: Option<String>,
    pub state_message: Option<String>,
    pub metadata: Vec<String>,
    pub footer_actions: Vec<RemediationAction>,
}

impl MediaPreviewSpec {
    pub fn new(kind: MediaKind, title: impl Into<String>) -> Self {
        Self {
            kind,
            state: MediaState::Ready,
            aspect_ratio: AspectRatio::Landscape,
            title: title.into(),
            description: None,
            badge: None,
            thumbnail_meta: None,
            state_title: None,
            state_message: None,
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

    pub fn with_aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_thumbnail_meta(mut self, thumbnail_meta: impl Into<String>) -> Self {
        self.thumbnail_meta = Some(thumbnail_meta.into());
        self
    }

    pub fn with_state_title(mut self, state_title: impl Into<String>) -> Self {
        self.state_title = Some(state_title.into());
        self
    }

    pub fn with_state_message(mut self, state_message: impl Into<String>) -> Self {
        self.state_message = Some(state_message.into());
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
}
