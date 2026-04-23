//! `BannerSpec` — spec for the `Banner` component.
//!
//! A simple tinted status banner with icon, title, message, and optional dismiss button.
//! Backed by a tone color that controls fill, border, and icon tint.

use crate::types::StatusTone;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BannerSpec {
    pub tone: StatusTone,
    pub title: Option<String>,
    pub message: Option<String>,
    pub has_icon: bool,
    pub is_dismissible: bool,
}

impl Default for BannerSpec {
    fn default() -> Self {
        Self {
            tone: StatusTone::Info,
            title: None,
            message: None,
            has_icon: true,
            is_dismissible: false,
        }
    }
}

impl BannerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tone(mut self, tone: StatusTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_icon(mut self, has_icon: bool) -> Self {
        self.has_icon = has_icon;
        self
    }

    pub fn with_dismissible(mut self, is_dismissible: bool) -> Self {
        self.is_dismissible = is_dismissible;
        self
    }

    /// Token controlling the background tint fill color.
    pub fn fill_token(&self) -> &'static str {
        self.tone.color_token()
    }

    /// Token controlling the icon color.
    pub fn icon_color_token(&self) -> &'static str {
        self.tone.color_token()
    }

    /// Token controlling the left/border accent color.
    pub fn border_token(&self) -> &'static str {
        self.tone.color_token()
    }
}
