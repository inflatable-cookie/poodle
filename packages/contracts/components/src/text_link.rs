#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextLinkTone {
    #[default]
    Accent,
    Inherit,
    Secondary,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextLinkSpec {
    pub label: String,
    pub href: Option<String>,
    pub target: Option<String>,
    pub rel: Option<String>,
    pub aria_label: Option<String>,
    pub disabled: bool,
    pub tone: TextLinkTone,
}

impl TextLinkSpec {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_tone(mut self, tone: TextLinkTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn renders_anchor(&self) -> bool {
        self.href.is_some() && !self.disabled
    }

    pub fn color_token(&self) -> &'static str {
        match self.tone {
            TextLinkTone::Accent => "color.accent.base",
            TextLinkTone::Inherit => "color.text.primary",
            TextLinkTone::Secondary => "color.text.secondary",
        }
    }
}
