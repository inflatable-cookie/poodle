use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppHeaderSpec {
    pub title: Option<String>,
    pub is_drag_region: bool,
    pub aria_label: Option<String>,
    pub primary_action_count: usize,
    pub utility_item_count: usize,
}

impl Default for AppHeaderSpec {
    fn default() -> Self {
        Self {
            title: None,
            is_drag_region: false,
            aria_label: None,
            primary_action_count: 0,
            utility_item_count: 0,
        }
    }
}

impl AppHeaderSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_drag_region(mut self, is_drag_region: bool) -> Self {
        self.is_drag_region = is_drag_region;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_primary_action_count(mut self, primary_action_count: usize) -> Self {
        self.primary_action_count = primary_action_count;
        self
    }

    pub fn with_utility_item_count(mut self, utility_item_count: usize) -> Self {
        self.utility_item_count = utility_item_count;
        self
    }

    pub fn is_utility_heavy(&self) -> bool {
        self.utility_item_count > 0
    }

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }
}
