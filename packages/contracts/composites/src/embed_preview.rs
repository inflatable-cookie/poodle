use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbedPreviewSpec {
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub provider: Option<String>,
    pub is_loading: bool,
}

impl EmbedPreviewSpec {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            thumbnail_url: None,
            provider: None,
            is_loading: false,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_thumbnail_url(mut self, thumbnail_url: impl Into<String>) -> Self {
        self.thumbnail_url = Some(thumbnail_url.into());
        self
    }

    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn with_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
}
