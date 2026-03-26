use poodle_tokens::semantic;

use crate::types::ParsedEmbed;

#[derive(Clone, Debug, PartialEq)]
pub struct EmbedPreviewSpec {
    pub parsed: Option<ParsedEmbed>,
    pub aspect_ratio: Option<f32>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub empty_message: String,
}

impl EmbedPreviewSpec {
    pub fn new() -> Self {
        Self {
            parsed: None,
            aspect_ratio: Some(16.0 / 9.0),
            is_loading: false,
            error: None,
            empty_message: String::from("No embed to preview"),
        }
    }

    pub fn with_parsed(mut self, parsed: ParsedEmbed) -> Self {
        self.parsed = Some(parsed);
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }

    pub fn with_auto_aspect_ratio(mut self) -> Self {
        self.aspect_ratio = None;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn with_empty_message(mut self, empty_message: impl Into<String>) -> Self {
        self.empty_message = empty_message.into();
        self
    }

    pub fn embed_url(&self) -> Option<String> {
        let parsed = self.parsed.as_ref()?;
        match parsed.provider.as_str() {
            "youtube" => Some(format!("https://www.youtube-nocookie.com/embed/{}", parsed.id)),
            "vimeo" => Some(format!("https://player.vimeo.com/video/{}", parsed.id)),
            _ => parsed.original_url.clone(),
        }
    }

    pub fn has_raw_embed(&self) -> bool {
        self.parsed
            .as_ref()
            .and_then(|parsed| parsed.original_embed.as_ref())
            .is_some()
    }

    pub fn effective_aspect_ratio(&self) -> Option<f32> {
        match self.parsed.as_ref().map(|parsed| parsed.provider.as_str()) {
            Some("audioboom") => None,
            _ => self.aspect_ratio,
        }
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
}
