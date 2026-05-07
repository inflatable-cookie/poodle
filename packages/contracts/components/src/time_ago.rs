use poodle_tokens::semantic;

use crate::InlineTypographyMode;

#[derive(Clone, Debug, PartialEq)]
pub struct TimeAgoSpec {
    pub timestamp: String,
    pub live: bool,
    /// When true (default), renders compact forms like "2m ago".
    /// When false, renders long forms like "2 minutes ago".
    pub short: bool,
    pub aria_label: Option<String>,
    pub typography: InlineTypographyMode,
}

impl Default for TimeAgoSpec {
    fn default() -> Self {
        Self {
            timestamp: String::new(),
            live: false,
            short: true,
            aria_label: None,
            typography: InlineTypographyMode::default(),
        }
    }
}

impl TimeAgoSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    pub fn with_live(mut self, live: bool) -> Self {
        self.live = live;
        self
    }

    pub fn with_short(mut self, short: bool) -> Self {
        self.short = short;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_typography(mut self, typography: InlineTypographyMode) -> Self {
        self.typography = typography;
        self
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn inherits_typography(&self) -> bool {
        self.typography == InlineTypographyMode::Inherit
    }
}
