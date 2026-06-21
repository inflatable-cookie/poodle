use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateTileSpec {
    pub label: String,
    pub value: String,
    pub trend: Option<String>,
    pub trend_label: Option<String>,
    pub has_sparkline: bool,
}

impl StateTileSpec {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            trend: None,
            trend_label: None,
            has_sparkline: false,
        }
    }

    pub fn with_trend(mut self, trend: impl Into<String>) -> Self {
        self.trend = Some(trend.into());
        self
    }

    pub fn with_trend_label(mut self, trend_label: impl Into<String>) -> Self {
        self.trend_label = Some(trend_label.into());
        self
    }

    pub fn with_sparkline(mut self, has_sparkline: bool) -> Self {
        self.has_sparkline = has_sparkline;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn trend_color_token(&self) -> &'static str {
        match self.trend.as_deref() {
            Some("up") => semantic::COLOR_STATUS_SUCCESS,
            Some("down") => semantic::COLOR_STATUS_DANGER,
            _ => semantic::COLOR_TEXT_SECONDARY,
        }
    }

    /// Root corner radius (contract §2: root `radius` target).
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    /// Root border width (contract §2: root `border` target).
    pub fn border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }

    /// Label color (contract §2: Label → `text-secondary`).
    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Value color (contract §2: Value → primary value display).
    pub fn value_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Value typography size token (contract §2 maps Value → `typography-heading`).
    pub fn value_font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_HEADING_SIZE
    }

    /// Label typography size token (contract §2 maps Label → `typography-label`).
    pub fn label_font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    /// Trend-row typography size token (tracks the label size).
    pub fn trend_font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    /// Background used for the reserved sparkline slot (contract §2: the
    /// Sparkline area is host-owned; the tile only reserves the slot).
    /// TOKEN GAP: there is no dedicated chart-surface token, so the slot uses
    /// the surface background as a neutral reserved area.
    pub fn sparkline_slot_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// Trend → directional indicator glyph (contract §2 TrendIndicator;
    /// §7 keeps trend meaning in `trend_label` text, the glyph is decorative).
    pub fn trend_glyph(&self) -> &'static str {
        match self.trend.as_deref() {
            Some("up") => "\u{2191}",   // ↑
            Some("down") => "\u{2193}", // ↓
            _ => "\u{2192}",            // →
        }
    }
}
