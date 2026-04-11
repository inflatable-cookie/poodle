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
}
