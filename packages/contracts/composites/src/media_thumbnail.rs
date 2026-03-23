use poodle_tokens::semantic;

use crate::types::{AspectRatio, MediaKind, MediaState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaThumbnailSpec {
    pub kind: MediaKind,
    pub state: MediaState,
    pub aspect_ratio: AspectRatio,
    pub title: Option<String>,
    pub meta: Option<String>,
    pub badge_label: Option<String>,
    pub show_caption: bool,
}

impl MediaThumbnailSpec {
    pub fn new(kind: MediaKind) -> Self {
        Self {
            kind,
            state: MediaState::Ready,
            aspect_ratio: AspectRatio::Landscape,
            title: None,
            meta: None,
            badge_label: None,
            show_caption: true,
        }
    }

    pub fn with_state(mut self, state: MediaState) -> Self {
        self.state = state;
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn with_badge_label(mut self, badge_label: impl Into<String>) -> Self {
        self.badge_label = Some(badge_label.into());
        self
    }

    pub fn with_show_caption(mut self, show_caption: bool) -> Self {
        self.show_caption = show_caption;
        self
    }

    pub fn shows_fallback_copy(&self) -> bool {
        self.state != MediaState::Ready
    }

    pub fn caption_visible(&self) -> bool {
        self.show_caption && self.title.is_some()
    }

    pub fn frame_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}
