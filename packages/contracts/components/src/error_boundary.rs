#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorBoundarySpec {
    pub title: String,
    pub retry_label: String,
    pub error_message: Option<String>,
}

impl Default for ErrorBoundarySpec {
    fn default() -> Self {
        Self {
            title: String::from("Something went wrong"),
            retry_label: String::from("Try again"),
            error_message: None,
        }
    }
}

impl ErrorBoundarySpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_retry_label(mut self, retry_label: impl Into<String>) -> Self {
        self.retry_label = retry_label.into();
        self
    }

    pub fn with_error_message(mut self, error_message: impl Into<String>) -> Self {
        self.error_message = Some(error_message.into());
        self
    }

    pub fn is_error(&self) -> bool {
        self.error_message.is_some()
    }
}
