use pug_gpui_tokens::semantic;

#[derive(Clone)]
pub struct BreadcrumbItem {
    pub value: String,
    pub label: String,
    pub href: Option<String>,
    pub is_current: bool,
}

impl BreadcrumbItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            href: None,
            is_current: false,
        }
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_is_current(mut self, is_current: bool) -> Self {
        self.is_current = is_current;
        self
    }
}

#[derive(Clone)]
pub struct BreadcrumbsSpec {
    pub items: Vec<BreadcrumbItem>,
    pub aria_label: String,
    pub max_visible_items: Option<usize>,
}

impl BreadcrumbsSpec {
    pub fn new(items: Vec<BreadcrumbItem>) -> Self {
        Self {
            items,
            aria_label: "Breadcrumb".to_string(),
            max_visible_items: None,
        }
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn with_max_visible_items(mut self, max: usize) -> Self {
        self.max_visible_items = Some(max);
        self
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn current_text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn hover_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }
}
