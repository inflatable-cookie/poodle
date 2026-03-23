use flint_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShellStatusBarSpec {
    pub summary: Option<String>,
    pub leading_item_count: usize,
    pub trailing_item_count: usize,
}

impl Default for ShellStatusBarSpec {
    fn default() -> Self {
        Self {
            summary: None,
            leading_item_count: 0,
            trailing_item_count: 0,
        }
    }
}

impl ShellStatusBarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_leading_item_count(mut self, leading_item_count: usize) -> Self {
        self.leading_item_count = leading_item_count;
        self
    }

    pub fn with_trailing_item_count(mut self, trailing_item_count: usize) -> Self {
        self.trailing_item_count = trailing_item_count;
        self
    }

    pub fn is_dense(&self) -> bool {
        self.leading_item_count + self.trailing_item_count > 3
    }

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}
