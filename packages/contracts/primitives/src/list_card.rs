use flint_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeadingShape {
    Circle,
    RoundedSquare,
}

impl Default for LeadingShape {
    fn default() -> Self {
        Self::Circle
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeadingFill {
    Tint,
    Solid,
}

impl Default for LeadingFill {
    fn default() -> Self {
        Self::Tint
    }
}

#[derive(Clone, Debug)]
pub struct ListCardSpec {
    pub title: String,
    pub subtitle: Option<String>,
    pub meta: Option<String>,
    pub leading_shape: LeadingShape,
    pub leading_fill: LeadingFill,
    pub accent_color: Option<String>,
    pub is_interactive: bool,
    pub is_disabled: bool,
    pub is_not_live: bool,
    pub sash: Option<String>,
    pub sash_color: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for ListCardSpec {
    fn default() -> Self {
        Self {
            title: String::new(),
            subtitle: None,
            meta: None,
            leading_shape: LeadingShape::Circle,
            leading_fill: LeadingFill::Tint,
            accent_color: None,
            is_interactive: false,
            is_disabled: false,
            is_not_live: false,
            sash: None,
            sash_color: None,
            aria_label: None,
        }
    }
}

impl ListCardSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn with_leading_shape(mut self, shape: LeadingShape) -> Self {
        self.leading_shape = shape;
        self
    }

    pub fn with_leading_fill(mut self, fill: LeadingFill) -> Self {
        self.leading_fill = fill;
        self
    }

    pub fn with_interactive(mut self, interactive: bool) -> Self {
        self.is_interactive = interactive;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_not_live(mut self, not_live: bool) -> Self {
        self.is_not_live = not_live;
        self
    }

    pub fn with_sash(mut self, sash: impl Into<String>) -> Self {
        self.sash = Some(sash.into());
        self
    }

    pub fn with_sash_color(mut self, color: impl Into<String>) -> Self {
        self.sash_color = Some(color.into());
        self
    }

    pub fn with_accent_color(mut self, color: impl Into<String>) -> Self {
        self.accent_color = Some(color.into());
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    // ── Token methods ──────────────────────────────────────────

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn hover_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn subtitle_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn meta_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn leading_tint_bg_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn leading_solid_bg_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
