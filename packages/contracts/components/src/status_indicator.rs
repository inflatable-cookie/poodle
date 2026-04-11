use crate::types::StatusTone;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusIndicatorSpec {
    pub status: StatusTone,
    pub label: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for StatusIndicatorSpec {
    fn default() -> Self {
        Self {
            status: StatusTone::Neutral,
            label: None,
            aria_label: None,
        }
    }
}

impl StatusIndicatorSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_status(mut self, status: StatusTone) -> Self {
        self.status = status;
        self
    }

    pub fn status_color_token(&self) -> &'static str {
        self.status.color_token()
    }

    /// Contract: dot size 0.5625rem (9px).
    pub fn dot_size_rem(&self) -> f32 {
        0.5625
    }

    /// Contract: gap between dot and label 0.4375rem (7px).
    pub fn gap_rem(&self) -> f32 {
        0.4375
    }

    /// Contract: label font-size 0.75rem (12px).
    pub fn label_font_size_rem(&self) -> f32 {
        0.75
    }

    /// Contract: label color token.
    pub fn label_color_token(&self) -> &'static str {
        "color.text.primary"
    }
}
