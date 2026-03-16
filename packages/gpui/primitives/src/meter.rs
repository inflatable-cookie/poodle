use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct MeterSpec {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub optimum: Option<f64>,
    pub aria_label: Option<String>,
}

impl Default for MeterSpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            low: None,
            high: None,
            optimum: None,
            aria_label: None,
        }
    }
}

impl MeterSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn with_min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn with_low(mut self, low: f64) -> Self {
        self.low = Some(low);
        self
    }

    pub fn with_high(mut self, high: f64) -> Self {
        self.high = Some(high);
        self
    }

    pub fn with_optimum(mut self, optimum: f64) -> Self {
        self.optimum = Some(optimum);
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn normalized_progress(&self) -> f64 {
        let range = self.max - self.min;
        if range <= 0.0 {
            0.0
        } else {
            ((self.value - self.min) / range).clamp(0.0, 1.0)
        }
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn track_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}
