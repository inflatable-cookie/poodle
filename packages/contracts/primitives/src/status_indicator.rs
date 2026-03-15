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
}
