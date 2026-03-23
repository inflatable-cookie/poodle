use flint_tokens::semantic;

use crate::types::{Orientation, TabStripItem};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabStripSpec {
    pub items: Vec<TabStripItem>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub orientation: Orientation,
    pub is_reorderable: bool,
    pub aria_label: Option<String>,
}

impl Default for TabStripSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            orientation: Orientation::Horizontal,
            is_reorderable: false,
            aria_label: None,
        }
    }
}

impl TabStripSpec {
    pub fn new(items: Vec<TabStripItem>) -> Self {
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

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
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
            .or_else(|| {
                self.items
                    .iter()
                    .find(|item| !item.is_disabled)
                    .map(|item| item.value.as_str())
            })
    }

    pub fn current_item(&self) -> Option<&TabStripItem> {
        let current = self.current_value()?;
        self.items.iter().find(|item| item.value == current)
    }

    pub fn closable_item_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_closable).count()
    }

    pub fn item_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }
}
