use flint_tokens::semantic;

use crate::types::SurfaceTabItem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceTabsSpec {
    pub items: Vec<SurfaceTabItem>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub is_reorderable: bool,
    pub aria_label: Option<String>,
    pub add_enabled: bool,
}

impl Default for SurfaceTabsSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            is_reorderable: true,
            aria_label: None,
            add_enabled: false,
        }
    }
}

impl SurfaceTabsSpec {
    pub fn new(items: Vec<SurfaceTabItem>) -> Self {
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

    pub fn with_add_enabled(mut self, add_enabled: bool) -> Self {
        self.add_enabled = add_enabled;
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
