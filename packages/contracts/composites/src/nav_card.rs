use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavCardSpec {
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub href: Option<String>,
    pub is_active: bool,
}

impl NavCardSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
            href: None,
            is_active: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        if self.is_active {
            semantic::COLOR_BACKGROUND_ELEVATED
        } else {
            semantic::COLOR_BACKGROUND_SURFACE
        }
    }

    pub fn border_token(&self) -> &'static str {
        if self.is_active {
            semantic::COLOR_ACCENT_BASE
        } else {
            semantic::COLOR_BORDER_SUBTLE
        }
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
}
