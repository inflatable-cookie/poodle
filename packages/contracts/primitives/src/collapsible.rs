use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollapsibleSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl Default for CollapsibleSpec {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            title: None,
            description: None,
            is_disabled: false,
            aria_label: None,
        }
    }
}

impl CollapsibleSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn activation_allowed(&self) -> bool {
        !self.is_disabled
    }

    pub fn requires_accessible_name(&self) -> bool {
        self.title
            .as_ref()
            .map(|title| title.trim().is_empty())
            .unwrap_or(true)
            && self
                .aria_label
                .as_ref()
                .map(|label| label.trim().is_empty())
                .unwrap_or(true)
    }

    pub fn content_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }
}
