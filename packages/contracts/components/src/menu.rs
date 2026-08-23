use poodle_tokens::semantic;

use crate::types::{
    ControlDensity, ControlSize, MenuEntry, MenuItemKind, OverlayPlacement, SemanticControlSizeRole,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuSpec {
    pub items: Vec<MenuEntry>,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placement: OverlayPlacement,
    /// Refuses outside-interact dismissal when false. Matches Svelte
    /// `dismissOnOutsideInteract` (default `true`).
    pub dismiss_on_outside_interact: bool,
    pub aria_label: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Accessible name for the trigger, distinct from the menu's own.
    pub trigger_aria_label: Option<String>,
}

impl Default for MenuSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            open: None,
            default_open: false,
            placement: OverlayPlacement::BottomStart,
            dismiss_on_outside_interact: true,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Chrome,
            density: None,
            trigger_aria_label: None,
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

    pub fn with_dismiss_on_outside_interact(mut self, dismiss_on_outside_interact: bool) -> Self {
        self.dismiss_on_outside_interact = dismiss_on_outside_interact;
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

    pub fn overlay_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn overlay_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn item_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn item_highlight_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
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
