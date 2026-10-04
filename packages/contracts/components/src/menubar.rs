use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, MenubarEntry, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenubarSpec {
    pub items: Vec<MenubarEntry>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub aria_label: Option<String>,
    /// Refuses outside-interact dismissal when false. Matches Svelte
    /// `dismissOnOutsideInteract` (default `true`).
    pub dismiss_on_outside_interact: bool,
    /// Host-tracked keyboard focus (Svelte `focusIndex`): which trigger the
    /// tab stop sits on. `None` falls back to the open trigger, then the
    /// first enabled trigger.
    pub focused_value: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for MenubarSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            aria_label: None,
            dismiss_on_outside_interact: true,
            focused_value: None,
            size: None,
            size_role: SemanticControlSizeRole::Chrome,
            density: None,
        }
    }
}

impl MenubarSpec {
    pub fn new(items: Vec<MenubarEntry>) -> Self {
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

    pub fn with_dismiss_on_outside_interact(mut self, dismiss_on_outside_interact: bool) -> Self {
        self.dismiss_on_outside_interact = dismiss_on_outside_interact;
        self
    }

    /// Set the host-tracked focus value for roving-tab sync.
    pub fn with_focused_value(mut self, focused_value: impl Into<String>) -> Self {
        self.focused_value = Some(focused_value.into());
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        // No first-enabled fallback: Svelte defaults to all closed
        // (`value ?? uncontrolledValue`, both null), so an unset spec
        // mounts no open menu. Focus still starts at the first enabled
        // trigger; only the open value stays empty.
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn current_menu(&self) -> Option<&MenubarEntry> {
        let current = self.current_value()?;
        self.items.iter().find(|item| item.value == current)
    }

    pub fn trigger_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn list_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn list_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn list_bg_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
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
