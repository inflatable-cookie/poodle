use pug_tokens::semantic;

use crate::types::PanelTabItem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PanelTabsSpec {
    pub items: Vec<PanelTabItem>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub is_reorderable: bool,
    pub aria_label: Option<String>,
}

impl Default for PanelTabsSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            is_reorderable: true,
            aria_label: None,
        }
    }
}

impl PanelTabsSpec {
    pub fn new(items: Vec<PanelTabItem>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_reorderable(mut self, is_reorderable: bool) -> Self {
        self.is_reorderable = is_reorderable;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
            .or_else(|| self.items.first().map(|item| item.value.as_str()))
    }

    pub fn closable_item_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_closable).count()
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }
}
