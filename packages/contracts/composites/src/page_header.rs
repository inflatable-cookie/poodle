use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PageHeaderAlign {
    Start,
    #[default]
    Between,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageHeaderSpec {
    pub title: String,
    pub subtitle: Option<String>,
    pub eyebrow: Option<String>,
    pub align: PageHeaderAlign,
    pub aria_label: Option<String>,
}

impl PageHeaderSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            eyebrow: None,
            align: PageHeaderAlign::default(),
            aria_label: None,
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
        self
    }

    pub fn with_align(mut self, align: PageHeaderAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn subtitle_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn eyebrow_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn header_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }
}
