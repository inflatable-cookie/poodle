use pug_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressSpec {
    pub value: Option<f64>,
    pub max: f64,
    pub is_indeterminate: bool,
    pub aria_label: Option<String>,
    pub value_text: Option<String>,
}

impl Default for ProgressSpec {
    fn default() -> Self {
        Self {
            value: None,
            max: 100.0,
            is_indeterminate: false,
            aria_label: None,
            value_text: None,
        }
    }
}

impl ProgressSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_indeterminate(mut self, is_indeterminate: bool) -> Self {
        self.is_indeterminate = is_indeterminate;
        self
    }

    pub fn normalized_progress(&self) -> Option<f64> {
        if self.is_indeterminate {
            None
        } else {
            self.value.map(|value| {
                if self.max <= 0.0 {
                    0.0
                } else {
                    (value / self.max).clamp(0.0, 1.0)
                }
            })
        }
    }

    pub fn indicator_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }
}
