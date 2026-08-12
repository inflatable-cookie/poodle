use poodle_tokens::semantic;

use crate::tabs::ActiveEdge;
use crate::tabs::ActiveFill;
use crate::types::{ControlDensity, ControlSize, NavigationMenuEntry, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationMenuSpec {
    pub items: Vec<NavigationMenuEntry>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub aria_label: Option<String>,
    /// Selection edge on the open trigger — see `ActiveEdge`. Matches Svelte
    /// `activeEdge` (default `"none"`).
    pub active_edge: ActiveEdge,
    /// Selection treatment on the open trigger: none (no fill), tint, or
    /// fully accent-filled. Matches Svelte `activeFill` (default `"tint"`).
    pub active_fill: ActiveFill,
    /// Refuses outside-interact dismissal when false. Matches Svelte
    /// `dismissOnOutsideInteract` (default `true`).
    pub dismiss_on_outside_interact: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for NavigationMenuSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            aria_label: None,
            active_edge: ActiveEdge::None,
            active_fill: ActiveFill::Tint,
            dismiss_on_outside_interact: true,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
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

    pub fn with_dismiss_on_outside_interact(mut self, dismiss_on_outside_interact: bool) -> Self {
        self.dismiss_on_outside_interact = dismiss_on_outside_interact;
        self
    }

    /// Set the selection edge on the open trigger (none, outline, or underline).
    pub fn with_active_edge(mut self, active_edge: ActiveEdge) -> Self {
        self.active_edge = active_edge;
        self
    }

    /// Set the selection treatment on the open trigger (none, tint, or solid).
    pub fn with_active_fill(mut self, active_fill: ActiveFill) -> Self {
        self.active_fill = active_fill;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tabs::{ActiveEdge, ActiveFill};

    #[test]
    fn defaults_have_no_edge_and_tint_fill() {
        let spec = NavigationMenuSpec::new(vec![]);
        assert_eq!(spec.active_edge, ActiveEdge::None);
        assert_eq!(spec.active_fill, ActiveFill::Tint);
    }

    #[test]
    fn builders_set_edge_and_fill() {
        let spec = NavigationMenuSpec::new(vec![])
            .with_active_edge(ActiveEdge::Outline)
            .with_active_fill(ActiveFill::Solid);
        assert_eq!(spec.active_edge, ActiveEdge::Outline);
        assert_eq!(spec.active_fill, ActiveFill::Solid);
    }
}
