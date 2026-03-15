use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListCardSpec {
    pub title: String,
    pub subtitle: Option<String>,
    pub is_interactive: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
}

impl ListCardSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            is_interactive: true,
            is_selected: false,
            is_disabled: false,
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_interactive(mut self, is_interactive: bool) -> Self {
        self.is_interactive = is_interactive;
        self
    }

    pub fn with_selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        if self.is_selected {
            semantic::COLOR_BACKGROUND_ELEVATED
        } else {
            semantic::COLOR_BACKGROUND_SURFACE
        }
    }

    pub fn border_token(&self) -> &'static str {
        if self.is_selected {
            semantic::COLOR_ACCENT_BASE
        } else {
            semantic::COLOR_BORDER_SUBTLE
        }
    }

    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
}
