use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// Radio — a single option. Group exclusivity is host-owned on native
/// (the web uses the browser's `name`); a click never unchecks this control.
///
/// Contract: `docs/contracts/components/radio.md`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioSpec {
    pub id: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub checked: Option<bool>,
    pub default_checked: bool,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub label: Option<String>,
    pub aria_label: Option<String>,
    pub description_id: Option<String>,
    /// Custom color override for the checked border and dot (CSS hex string).
    pub selected_color: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for RadioSpec {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            value: None,
            checked: None,
            default_checked: false,
            is_disabled: false,
            is_read_only: false,
            label: None,
            aria_label: None,
            description_id: None,
            selected_color: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl RadioSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn with_default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_read_only(mut self, is_read_only: bool) -> Self {
        self.is_read_only = is_read_only;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_description_id(mut self, description_id: impl Into<String>) -> Self {
        self.description_id = Some(description_id.into());
        self
    }

    pub fn with_selected_color(mut self, color: impl Into<String>) -> Self {
        self.selected_color = Some(color.into());
        self
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

    pub fn is_checked(&self) -> bool {
        self.checked.unwrap_or(self.default_checked)
    }

    pub fn is_interactive(&self) -> bool {
        !self.is_disabled && !self.is_read_only
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controlled_checked_wins_over_the_uncontrolled_seed() {
        let spec = RadioSpec::new()
            .with_default_checked(true)
            .with_checked(false);
        assert!(!spec.is_checked());
        assert!(RadioSpec::new().with_default_checked(true).is_checked());
    }

    #[test]
    fn disabled_or_readonly_is_not_interactive() {
        assert!(!RadioSpec::new().with_disabled(true).is_interactive());
        assert!(!RadioSpec::new().with_read_only(true).is_interactive());
        assert!(RadioSpec::new().is_interactive());
    }
}
