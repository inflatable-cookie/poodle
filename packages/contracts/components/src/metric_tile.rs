use crate::ControlDensity;
use poodle_tokens::semantic;

/// Direction of a metric trend relative to the previous period.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MetricTrend {
    /// Value increased — renders an up arrow with success tone.
    Up,
    /// Value decreased — renders a down arrow with danger tone.
    Down,
    /// Value is unchanged — renders a right arrow with neutral tone.
    #[default]
    Flat,
}

impl MetricTrend {
    /// Token name of the foreground colour used for the trend arrow
    /// and label.
    pub fn color_token(self) -> &'static str {
        match self {
            Self::Up => semantic::COLOR_STATUS_SUCCESS,
            Self::Down => semantic::COLOR_STATUS_DANGER,
            // Contract §8 + Svelte: flat trend uses tertiary text color.
            Self::Flat => semantic::COLOR_TEXT_TERTIARY,
        }
    }

    /// Icon name used for the arrow glyph.
    pub fn icon_name(self) -> &'static str {
        match self {
            Self::Up => "trending-up",
            Self::Down => "trending-down",
            Self::Flat => "arrow-right",
        }
    }
}

/// MetricTile — a compact metadata display tile showing a label and value,
/// optionally decorated with a trend indicator and/or sparkline chart.
#[derive(Clone, Debug, PartialEq)]
pub struct MetricTileSpec {
    pub label: String,
    pub value: String,
    pub aria_label: Option<String>,
    /// Optional trend direction — when set, the tile renders an arrow
    /// glyph and the `trend_label` text in the tone associated with
    /// the direction.
    pub trend: Option<MetricTrend>,
    /// Optional free-form label shown next to the trend arrow,
    /// e.g. "+12.4%", "−3 vs last week".
    pub trend_label: Option<String>,
    /// Optional sparkline data — a series of float values from which
    /// the component builds a small line chart. An empty or
    /// single-element vector suppresses the chart.
    pub sparkline_data: Vec<f32>,
    /// Presentation density — drives tile padding and internal gaps
    /// only (never typography or sparkline size). Contract §3/§8.
    /// Omission (`None`) inherits from the presentation context; an
    /// explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl Eq for MetricTileSpec {}

impl MetricTileSpec {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            aria_label: None,
            trend: None,
            trend_label: None,
            sparkline_data: Vec::new(),
            density: None,
        }
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_trend(mut self, trend: MetricTrend) -> Self {
        self.trend = Some(trend);
        self
    }

    pub fn with_trend_label(mut self, trend_label: impl Into<String>) -> Self {
        self.trend_label = Some(trend_label.into());
        self
    }

    pub fn with_sparkline(mut self, data: Vec<f32>) -> Self {
        self.sparkline_data = data;
        self
    }

    pub fn has_sparkline(&self) -> bool {
        self.sparkline_data.len() > 1
    }

    pub fn has_trend(&self) -> bool {
        self.trend.is_some()
    }

    /// Colour token for the trend arrow / label, or `None` when no
    /// trend is configured.
    pub fn trend_color_token(&self) -> Option<&'static str> {
        self.trend.map(|t| t.color_token())
    }

    pub fn effective_aria_label(&self) -> String {
        self.aria_label
            .clone()
            .unwrap_or_else(|| format!("{}: {}", self.label, self.value))
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn value_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn padding_x_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }

    /// Back-compat alias — historically returned SPACE_PANEL_X.
    pub fn padding_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    /// Root gap token. Contract §8 root gap is `space.inline.sm`
    /// (the default-density root gap); density variants override it via
    /// [`Self::root_gap_rem`].
    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    /// Sparkline stroke / chart color. Contract §8: `color.text.tertiary`.
    pub fn sparkline_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }

    /// Label font: code-family, 0.75rem. Contract §8: `0.75rem`.
    pub fn label_font_size_rem(&self) -> f32 {
        0.75
    }

    /// Value font: body size. Contract §8: `1rem`.
    pub fn value_font_size_rem(&self) -> f32 {
        1.0
    }

    /// Trend row font: 0.75rem. Contract §8 `.state-tile__trend`.
    pub fn trend_font_size_rem(&self) -> f32 {
        0.75
    }

    /// Trend arrow glyph font: 0.875rem. Contract §8 `.state-tile__trend-arrow`.
    pub fn trend_arrow_font_size_rem(&self) -> f32 {
        0.875
    }

    /// Trend row gap: 0.25rem. Contract §8 `.state-tile__trend` gap.
    pub fn trend_gap_rem(&self) -> f32 {
        0.25
    }

    /// Root border width: 0.0625rem (transparent border). Contract §8.
    pub fn border_width_rem(&self) -> f32 {
        0.0625
    }

    /// Sparkline width: 4rem. Contract §7/§8.
    pub fn sparkline_width_rem(&self) -> f32 {
        4.0
    }

    /// Sparkline height: 1.5rem. Contract §7/§8.
    pub fn sparkline_height_rem(&self) -> f32 {
        1.5
    }

    // ── Density-resolved spacing (rem) — contract §8 density table ──
    //
    // Density drives root gap, root padding and body gap ONLY; it never
    // touches typography or sparkline dimensions.

    /// Root gap in rem for a resolved density. Contract §8 density table.
    /// Omission is resolved by the render context — pass the resolved density in.
    pub fn root_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5, // space.inline.sm (8px)
            ControlDensity::Comfortable => 0.625,
        }
    }

    /// Root vertical padding in rem for a resolved density.
    /// Contract §8 density table (default = `0.625rem`).
    pub fn padding_y_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.625,
            ControlDensity::Comfortable => 0.75,
        }
    }

    /// Root horizontal padding in rem for a resolved density.
    /// Contract §8 density table (default = `space.panel.x`).
    pub fn padding_x_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.75,
            ControlDensity::Default => 1.0, // space.panel.x (16px)
            ControlDensity::Comfortable => 1.25,
        }
    }

    /// Body row gap in rem for a resolved density.
    /// Contract §8 density table (default = `space.inline.md`).
    pub fn body_gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75, // space.inline.md (12px)
            ControlDensity::Comfortable => 0.875,
        }
    }
}
