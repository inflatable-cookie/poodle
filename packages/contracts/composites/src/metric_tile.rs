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
            Self::Flat => semantic::COLOR_TEXT_SECONDARY,
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
        }
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

    pub fn padding_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }
}
