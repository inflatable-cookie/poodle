use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToggleGroupSelectionMode {
    Single,
    Multiple,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleGroupOption {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl ToggleGroupOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            aria_label: None,
        }
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleGroupSpec {
    pub value: Option<Vec<String>>,
    pub default_value: Option<Vec<String>>,
    pub options: Vec<ToggleGroupOption>,
    pub selection_mode: ToggleGroupSelectionMode,
    /// In single mode, selecting the active item clears the value to `None`.
    /// Contract §3/§5; mirrors Svelte `allowDeactivation` (no effect in multiple
    /// mode, where each item already toggles independently).
    pub allow_deactivation: bool,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ToggleGroupSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            options: Vec::new(),
            selection_mode: ToggleGroupSelectionMode::Single,
            allow_deactivation: false,
            is_disabled: false,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl ToggleGroupSpec {
    pub fn new(options: Vec<ToggleGroupOption>) -> Self {
        Self {
            options,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: Vec<String>) -> Self {
        self.value = Some(value);
        self
    }
    pub fn with_default_value(mut self, value: Vec<String>) -> Self {
        self.default_value = Some(value);
        self
    }
    pub fn with_selection_mode(mut self, mode: ToggleGroupSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }
    pub fn with_allow_deactivation(mut self, allow: bool) -> Self {
        self.allow_deactivation = allow;
        self
    }
    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn selected_values(&self) -> &[String] {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
            .unwrap_or(&[])
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_values().iter().any(|v| v == value)
    }

    /// Compute the next selection set when `value` is toggled, mirroring the
    /// Svelte `toggle()` logic:
    /// - multiple mode: add when absent, remove when present (independent toggle);
    /// - single mode: select `value`, or clear to empty when `allow_deactivation`
    ///   is set and `value` is already the active selection.
    pub fn next_value_on_toggle(&self, value: &str) -> Vec<String> {
        match self.selection_mode {
            ToggleGroupSelectionMode::Multiple => {
                let current = self.selected_values();
                if current.iter().any(|v| v == value) {
                    current
                        .iter()
                        .filter(|v| v.as_str() != value)
                        .cloned()
                        .collect()
                } else {
                    let mut next: Vec<String> = current.to_vec();
                    next.push(value.to_string());
                    next
                }
            }
            ToggleGroupSelectionMode::Single => {
                let already_active = self.selected_values() == [value.to_string()];
                if self.allow_deactivation && already_active {
                    Vec::new()
                } else {
                    vec![value.to_string()]
                }
            }
        }
    }

    pub fn item_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
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

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> Vec<ToggleGroupOption> {
        vec![
            ToggleGroupOption::new("grid", "Grid"),
            ToggleGroupOption::new("list", "List"),
            ToggleGroupOption::new("board", "Board"),
        ]
    }

    #[test]
    fn single_select_replaces_value() {
        let spec = ToggleGroupSpec::new(opts()).with_value(vec!["grid".into()]);
        assert_eq!(spec.next_value_on_toggle("list"), vec!["list".to_string()]);
    }

    #[test]
    fn single_select_without_deactivation_reselects_same() {
        let spec = ToggleGroupSpec::new(opts()).with_value(vec!["grid".into()]);
        // allow_deactivation defaults to false: re-toggling the active item keeps it.
        assert_eq!(spec.next_value_on_toggle("grid"), vec!["grid".to_string()]);
    }

    #[test]
    fn single_select_with_deactivation_clears_active() {
        let spec = ToggleGroupSpec::new(opts())
            .with_value(vec!["grid".into()])
            .with_allow_deactivation(true);
        assert!(spec.next_value_on_toggle("grid").is_empty());
        // A different item still selects normally.
        assert_eq!(spec.next_value_on_toggle("list"), vec!["list".to_string()]);
    }

    #[test]
    fn multiple_select_adds_and_removes_independently() {
        let spec = ToggleGroupSpec::new(opts())
            .with_selection_mode(ToggleGroupSelectionMode::Multiple)
            .with_value(vec!["grid".into(), "board".into()]);
        // Toggling an absent value appends it.
        assert_eq!(
            spec.next_value_on_toggle("list"),
            vec!["grid".to_string(), "board".to_string(), "list".to_string()]
        );
        // Toggling a present value removes it (allow_deactivation has no effect here).
        assert_eq!(spec.next_value_on_toggle("grid"), vec!["board".to_string()]);
    }
}
