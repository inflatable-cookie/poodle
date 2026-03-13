use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectHeaderSpec {
    pub title: String,
    pub is_dirty: bool,
    pub subtitle: Option<String>,
    pub aria_label: Option<String>,
    pub action_count: usize,
    pub status_item_count: usize,
}

impl ProjectHeaderSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_dirty: false,
            subtitle: None,
            aria_label: None,
            action_count: 0,
            status_item_count: 0,
        }
    }

    pub fn with_dirty(mut self, is_dirty: bool) -> Self {
        self.is_dirty = is_dirty;
        self
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_action_count(mut self, action_count: usize) -> Self {
        self.action_count = action_count;
        self
    }

    pub fn with_status_item_count(mut self, status_item_count: usize) -> Self {
        self.status_item_count = status_item_count;
        self
    }

    pub fn is_descriptive(&self) -> bool {
        self.subtitle.is_some() || self.status_item_count > 0
    }

    pub fn dirty_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_WARNING
    }
}
