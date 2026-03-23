use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct RatingSpec {
    pub value: f64,
    pub max: u8,
    pub is_readonly: bool,
    pub is_disabled: bool,
    pub precision: f64,
}

impl Default for RatingSpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            max: 5,
            is_readonly: false,
            is_disabled: false,
            precision: 1.0,
        }
    }
}

impl RatingSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn with_max(mut self, max: u8) -> Self {
        self.max = max;
        self
    }

    pub fn with_readonly(mut self, is_readonly: bool) -> Self {
        self.is_readonly = is_readonly;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_precision(mut self, precision: f64) -> Self {
        self.precision = precision;
        self
    }

    pub fn filled_count(&self) -> u8 {
        let clamped = self.value.clamp(0.0, self.max as f64);
        clamped.floor() as u8
    }

    pub fn partial_fill(&self) -> f64 {
        let clamped = self.value.clamp(0.0, self.max as f64);
        clamped.fract()
    }

    pub fn active_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn inactive_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
}
