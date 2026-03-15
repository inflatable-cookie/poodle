use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilterToolbarSpec {
    pub query: Option<String>,
    pub active_filter_count: usize,
    pub result_count: Option<usize>,
    pub show_clear_action: bool,
}

impl Default for FilterToolbarSpec {
    fn default() -> Self {
        Self {
            query: None,
            active_filter_count: 0,
            result_count: None,
            show_clear_action: false,
        }
    }
}

impl FilterToolbarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn with_active_filter_count(mut self, active_filter_count: usize) -> Self {
        self.active_filter_count = active_filter_count;
        self
    }

    pub fn with_result_count(mut self, result_count: usize) -> Self {
        self.result_count = Some(result_count);
        self
    }

    pub fn with_show_clear_action(mut self, show_clear_action: bool) -> Self {
        self.show_clear_action = show_clear_action;
        self
    }

    pub fn has_active_filters(&self) -> bool {
        self.active_filter_count > 0 || self.query.as_ref().is_some_and(|query| !query.is_empty())
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }
}
