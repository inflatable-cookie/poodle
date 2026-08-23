use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

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

/// Sentinel value carried by the synthesized ellipsis crumb that replaces the
/// collapsed middle items when `max_visible_items` truncates the trail. Matches
/// the Svelte `value: "__ellipsis__"` marker — the crumb is non-interactive.
pub const ELLIPSIS_VALUE: &str = "__ellipsis__";

#[derive(Clone)]
pub struct BreadcrumbsSpec {
    pub items: Vec<BreadcrumbItem>,
    pub aria_label: String,
    pub max_visible_items: Option<usize>,
    /// When true (default), the last visible item is treated as the current
    /// page (`aria-current="page"`) even without `is_current`. Set false to opt
    /// out. Mirrors the Svelte `forceLastItemCurrent` prop.
    pub force_last_item_current: bool,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl BreadcrumbsSpec {
    pub fn new(items: Vec<BreadcrumbItem>) -> Self {
        Self {
            items,
            aria_label: "Breadcrumb".to_string(),
            max_visible_items: None,
            force_last_item_current: true,
            size: None,
            size_role: SemanticControlSizeRole::Chrome,
            density: None,
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

    pub fn with_force_last_item_current(mut self, force: bool) -> Self {
        self.force_last_item_current = force;
        self
    }

    /// Resolve the items to actually render, applying the ellipsis-collapse
    /// when `max_visible_items` is set and the trail exceeds it. Returns
    /// `first + ellipsis + last (max - 1)` to match the Svelte truncation rule.
    pub fn visible_items(&self) -> Vec<BreadcrumbItem> {
        match self.max_visible_items {
            Some(max) if max >= 2 && self.items.len() > max => {
                let mut visible = Vec::with_capacity(max + 1);
                visible.push(self.items[0].clone());
                visible.push(BreadcrumbItem {
                    value: ELLIPSIS_VALUE.to_string(),
                    label: "\u{2026}".to_string(),
                    href: None,
                    is_current: false,
                });
                for item in self.items.iter().skip(self.items.len() - (max - 1)) {
                    visible.push(item.clone());
                }
                visible
            }
            _ => self.items.clone(),
        }
    }

    /// Whether the visible item at `index` (within `visible_items()`) should
    /// render as the current page. True when the item is explicitly current, or
    /// it is the last visible item and `force_last_item_current` is set. The
    /// ellipsis sentinel is never current.
    pub fn is_current_at(&self, item: &BreadcrumbItem, index: usize, visible_len: usize) -> bool {
        if item.value == ELLIPSIS_VALUE {
            return false;
        }
        item.is_current || (self.force_last_item_current && index + 1 == visible_len)
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

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }
}
