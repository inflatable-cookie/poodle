use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PanelHeaderSpec {
    pub title: Option<String>,
    pub is_active: bool,
    pub is_collapsible: bool,
    pub aria_label: Option<String>,
    pub utility_action_count: usize,
    pub has_tabs: bool,
}

impl Default for PanelHeaderSpec {
    fn default() -> Self {
        Self {
            title: None,
            is_active: false,
            is_collapsible: false,
            aria_label: None,
            utility_action_count: 0,
            has_tabs: false,
        }
    }
}

impl PanelHeaderSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn with_collapsible(mut self, is_collapsible: bool) -> Self {
        self.is_collapsible = is_collapsible;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_utility_action_count(mut self, utility_action_count: usize) -> Self {
        self.utility_action_count = utility_action_count;
        self
    }

    pub fn with_tabs(mut self, has_tabs: bool) -> Self {
        self.has_tabs = has_tabs;
        self
    }

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}
