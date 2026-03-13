use pug_gpui_tokens::semantic;

use crate::types::{MenuEntry, MenuItemKind, OverlayPlacement};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuSpec {
    pub items: Vec<MenuEntry>,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placement: OverlayPlacement,
    pub aria_label: Option<String>,
}

impl Default for MenuSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            open: None,
            default_open: false,
            placement: OverlayPlacement::BottomStart,
            aria_label: None,
        }
    }
}

impl MenuSpec {
    pub fn new(items: Vec<MenuEntry>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn actionable_item_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.kind != MenuItemKind::Separator && !item.is_disabled)
            .count()
    }

    pub fn checked_item_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_checked).count()
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }
}
