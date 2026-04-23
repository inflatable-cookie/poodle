use poodle_tokens::semantic;

#[derive(Clone)]
pub struct NavCardSpec {
    pub title: String,
    pub description: Option<String>,
    pub href: Option<String>,
    pub badge: Option<String>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl NavCardSpec {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: None,
            href: None,
            badge: None,
            is_disabled: false,
            aria_label: None,
        }
    }

    // Builder methods

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    // Token methods

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn hover_border_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn description_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn icon_bg_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn icon_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn badge_bg_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn badge_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn arrow_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    /// Title typography: label-size token. Contract §8: `typography.label.size`.
    pub fn title_typography_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    /// Root padding-x. Contract §8: `1rem`.
    pub fn padding_x_rem(&self) -> f32 {
        1.0
    }

    /// Root padding-y. Contract §8: `0.875rem`.
    pub fn padding_y_rem(&self) -> f32 {
        0.875
    }

    /// Gap between title and description. Contract §8: `0.25rem`.
    pub fn content_gap_rem(&self) -> f32 {
        0.25
    }

    /// Description font-size. Contract §8: `0.8125rem`.
    pub fn description_font_size_rem(&self) -> f32 {
        0.8125
    }
}
