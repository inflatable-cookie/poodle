use flint_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct TimeAgoSpec {
    pub timestamp: String,
    pub live: bool,
    pub aria_label: Option<String>,
}

impl Default for TimeAgoSpec {
    fn default() -> Self {
        Self {
            timestamp: String::new(),
            live: false,
            aria_label: None,
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

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }
}
