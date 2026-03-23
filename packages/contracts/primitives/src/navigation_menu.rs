use poodle_tokens::semantic;

use crate::types::NavigationMenuEntry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationMenuSpec {
    pub items: Vec<NavigationMenuEntry>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for NavigationMenuSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            aria_label: None,
        }
    }
}

impl NavigationMenuSpec {
    pub fn new(items: Vec<NavigationMenuEntry>) -> Self {
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

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
            .or_else(|| {
                self.items
                    .iter()
                    .find(|item| !item.is_disabled)
                    .map(|item| item.value.as_str())
            })
    }

    pub fn current_item(&self) -> Option<&NavigationMenuEntry> {
        let current = self.current_value()?;
        self.items.iter().find(|item| item.value == current)
    }

    pub fn viewport_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }

    pub fn trigger_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn viewport_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }
}
