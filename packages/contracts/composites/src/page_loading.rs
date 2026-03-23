use flint_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct PageLoadingSpec {
    pub is_visible: bool,
    pub value: Option<f64>,
    pub max: f64,
    pub message: Option<String>,
    pub can_cancel: bool,
}

impl PageLoadingSpec {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            value: None,
            max: 100.0,
            message: None,
            can_cancel: false,
        }
    }

    pub fn with_visible(mut self, is_visible: bool) -> Self {
        self.is_visible = is_visible;
        self
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_can_cancel(mut self, can_cancel: bool) -> Self {
        self.can_cancel = can_cancel;
        self
    }

    pub fn backdrop_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_OVERLAY
    }

    pub fn progress_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }
}
