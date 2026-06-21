use crate::types::ControlDensity;
use poodle_tokens::semantic;

#[derive(Clone)]
pub struct NavCardSpec {
    pub title: String,
    pub description: Option<String>,
    pub href: Option<String>,
    pub badge: Option<String>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    /// Density override for card padding, icon box size, and internal spacing.
    /// Contract §3/§8. Defaults to `Default`.
    pub density: ControlDensity,
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
            density: ControlDensity::Default,
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

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
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

    /// Focus outline width token. Contract §8 Root focus: `border-width-focus`.
    pub fn focus_border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
    }

    /// Icon box corner radius. Contract §8 Icon: `radius.control`.
    pub fn icon_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Badge pill radius. Contract §8 Badge: `999px` (pill).
    pub fn badge_radius_token(&self) -> &'static str {
        semantic::RADIUS_PILL
    }

    /// Title typography: label-size token. Contract §8: `typography.label.size`.
    pub fn title_typography_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    // ── Density-aware dimensions (contract §8 Density Overrides table) ──
    //
    // | density      | gap     | pad-y    | pad-x  | icon   | content | title  |
    // | compact      | 0.625   | 0.5      | 0.75   | 1.75   | 0.0625  | 0.3125 |
    // | default      | 0.75    | 0.625    | panel-x| 2.0    | 0.125   | 0.375  |
    // | comfortable  | 0.875   | 0.75     | 1.25   | 2.25   | 0.1875  | 0.4375 |
    //
    // `default` pad-x equals `space.panel.x` (1rem); the rem helper returns the
    // resolved value so a single code path covers all three densities. The
    // panel-x token is the authoritative source for the default — see NOTE.

    /// Root gap between icon, content, and arrow. Contract §8.
    pub fn root_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }
    }

    /// Root padding-x. Contract §8 (default resolves `space.panel.x` == 1rem).
    pub fn padding_x_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.75,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.25,
        }
    }

    /// Root padding-y. Contract §8.
    pub fn padding_y_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.625,
            ControlDensity::Comfortable => 0.75,
        }
    }

    /// Icon box edge length. Contract §8.
    pub fn icon_size_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 1.75,
            ControlDensity::Default => 2.0,
            ControlDensity::Comfortable => 2.25,
        }
    }

    /// Gap between title row and description. Contract §8 Content Gap.
    pub fn content_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.0625,
            ControlDensity::Default => 0.125,
            ControlDensity::Comfortable => 0.1875,
        }
    }

    /// Gap between title text and badge. Contract §8 Title Gap.
    pub fn title_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.3125,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.4375,
        }
    }

    /// Icon glyph font-size. Contract §8 Icon: `1rem`.
    pub fn icon_font_size_rem(&self) -> f32 {
        1.0
    }

    /// Badge font-size. Contract §8 Badge: `0.625rem`.
    pub fn badge_font_size_rem(&self) -> f32 {
        0.625
    }

    /// Description font-size. Contract §8: `0.8125rem`.
    pub fn description_font_size_rem(&self) -> f32 {
        0.8125
    }
}
