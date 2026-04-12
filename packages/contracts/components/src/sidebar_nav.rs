use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// A single navigation item in a sidebar group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SidebarNavItem {
    pub value: String,
    pub label: String,
    pub href: Option<String>,
    pub is_disabled: bool,
}

impl SidebarNavItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            href: None,
            is_disabled: false,
        }
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

/// A labelled group of navigation items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SidebarNavGroup {
    pub id: String,
    pub label: Option<String>,
    pub items: Vec<SidebarNavItem>,
}

impl SidebarNavGroup {
    pub fn new(id: impl Into<String>, items: Vec<SidebarNavItem>) -> Self {
        Self {
            id: id.into(),
            label: None,
            items,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// SidebarNav -- a vertical navigation component with grouped, labelled items.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SidebarNavSpec {
    pub groups: Vec<SidebarNavGroup>,
    pub value: Option<String>,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl SidebarNavSpec {
    pub fn new(groups: Vec<SidebarNavGroup>) -> Self {
        Self {
            groups,
            value: None,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    /// Groups that have at least one item.
    pub fn visible_groups(&self) -> Vec<&SidebarNavGroup> {
        self.groups.iter().filter(|g| !g.items.is_empty()).collect()
    }

    /// Total number of navigation items across all visible groups.
    pub fn total_item_count(&self) -> usize {
        self.visible_groups().iter().map(|g| g.items.len()).sum()
    }

    /// Whether the given item value is currently active.
    pub fn is_active(&self, item_value: &str) -> bool {
        self.value.as_deref() == Some(item_value)
    }

    pub fn item_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn item_active_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn group_title_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn active_indicator_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
