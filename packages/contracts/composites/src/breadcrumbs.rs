use pug_tokens::semantic;

use crate::types::BreadcrumbItem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreadcrumbsSpec {
    pub items: Vec<BreadcrumbItem>,
    pub separator: Option<String>,
}

impl BreadcrumbsSpec {
    pub fn new(items: Vec<BreadcrumbItem>) -> Self {
        Self {
            items,
            separator: None,
        }
    }

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = Some(separator.into());
        self
    }

    pub fn current_item(&self) -> Option<&BreadcrumbItem> {
        self.items.last()
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_ICON_MUTED
    }
}
