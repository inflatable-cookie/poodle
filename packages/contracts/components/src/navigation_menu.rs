use poodle_tokens::semantic;

use crate::tabs::ActiveFill;
use crate::types::{ControlDensity, ControlSize, NavigationMenuEntry, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationMenuSpec {
    pub items: Vec<NavigationMenuEntry>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub aria_label: Option<String>,
    /// Opt-in outline on the open trigger — the border the trigger carried by
    /// default before g13.016. Matches Svelte `activeOutline` (default
    /// false).
    pub active_outline: bool,
    /// Selection treatment on the open trigger: tint or fully accent-filled.
    /// Matches Svelte `activeFill` (default `"tint"`).
    pub active_fill: ActiveFill,
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
            active_outline: false,
            active_fill: ActiveFill::Tint,
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

    /// Opt into the outline on the open trigger (g13.016; default off).
    pub fn with_active_outline(mut self, active_outline: bool) -> Self {
        self.active_outline = active_outline;
        self
    }

    /// Set the selection treatment on the open trigger (tint or solid).
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
    use crate::tabs::ActiveFill;

    #[test]
    fn defaults_have_no_outline_and_tint_fill() {
        let spec = NavigationMenuSpec::new(vec![]);
        assert!(!spec.active_outline);
        assert_eq!(spec.active_fill, ActiveFill::Tint);
    }

    #[test]
    fn builders_set_outline_and_fill() {
        let spec = NavigationMenuSpec::new(vec![])
            .with_active_outline(true)
            .with_active_fill(ActiveFill::Solid);
        assert!(spec.active_outline);
        assert_eq!(spec.active_fill, ActiveFill::Solid);
    }
}
