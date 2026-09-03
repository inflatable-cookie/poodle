use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// One segment's portable contract fields (contract §3
/// `SegmentedControlOption`). Dedicated to SegmentedControl — deliberately
/// not the broad `ChoiceOption`, which cannot carry the icon, icon-only, or
/// title fields the contract specifies.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SegmentedControlOption {
    pub value: String,
    /// Visible label text, or the accessible-name and tooltip fallback for an
    /// icon-only option.
    pub label: String,
    /// Optional named icon rendered before the visible label.
    pub icon: Option<String>,
    /// Hides the visible label text when an icon exists; without an icon the
    /// label stays visible.
    pub icon_only: bool,
    pub is_disabled: bool,
    /// Accessible-name override for abbreviated labels.
    pub aria_label: Option<String>,
    /// Tooltip text for the segment wrapper; an icon-only option without one
    /// falls back to its required `label`.
    pub title: Option<String>,
}

impl SegmentedControlOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            icon_only: false,
            is_disabled: false,
            aria_label: None,
            title: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_icon_only(mut self, icon_only: bool) -> Self {
        self.icon_only = icon_only;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// True when the option actually presents icon-only: requested AND an
    /// icon exists. This is the condition that hides the visible label and
    /// engages the label's accessible-name/tooltip fallback (contract §3).
    pub fn is_icon_only(&self) -> bool {
        self.icon_only && self.icon.is_some()
    }

    /// Accessible name when it is not the visible label: an explicit
    /// `aria_label`, or the required `label` for an icon-only option.
    pub fn accessible_name_override(&self) -> Option<&str> {
        self.aria_label
            .as_deref()
            .or_else(|| self.is_icon_only().then_some(self.label.as_str()))
    }

    /// Tooltip text: explicit `title`, else the required `label` for an
    /// icon-only option. Labelled options without a title carry none.
    pub fn tooltip_text(&self) -> Option<&str> {
        self.title
            .as_deref()
            .or_else(|| self.is_icon_only().then_some(self.label.as_str()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SegmentedControlSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub options: Vec<SegmentedControlOption>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    /// When true, every segment takes equal horizontal space instead
    /// of sizing to its label content. Matches the Svelte
    /// `equalWidth` prop.
    pub equal_width: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Stable native instance scope, analogue of the web `name` prop.
    /// Shared render is stateless, so native callers provide the lifecycle
    /// identity explicitly rather than relying on render order.
    pub instance_id: String,
}

impl SegmentedControlSpec {
    pub fn new(instance_id: impl Into<String>, options: Vec<SegmentedControlOption>) -> Self {
        Self {
            value: None,
            default_value: None,
            options,
            is_disabled: false,
            aria_label: None,
            equal_width: true,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            instance_id: instance_id.into(),
        }
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_equal_width(mut self, equal_width: bool) -> Self {
        self.equal_width = equal_width;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn selected_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_spec_and_builder_methods() {
        let opt1 = SegmentedControlOption::new("grid", "Grid");
        let opt2 = SegmentedControlOption::new("list", "List");
        let mut spec = SegmentedControlSpec::new("view-picker", vec![opt1, opt2])
            .with_default_value("grid")
            .with_equal_width(false)
            .with_size(ControlSize::Lg)
            .with_size_role(SemanticControlSizeRole::Chrome)
            .with_density(ControlDensity::Compact);
        spec.aria_label = Some("View picker".to_string());
        spec.is_disabled = true;

        assert_eq!(spec.instance_id, "view-picker");
        assert_eq!(spec.default_value.as_deref(), Some("grid"));
        assert_eq!(spec.current_value(), Some("grid"));
        assert!(!spec.equal_width);
        assert!(spec.is_disabled);
        assert_eq!(spec.aria_label.as_deref(), Some("View picker"));
        assert_eq!(spec.size, Some(ControlSize::Lg));
        assert_eq!(spec.size_role, SemanticControlSizeRole::Chrome);
        assert_eq!(spec.density, Some(ControlDensity::Compact));
        assert_eq!(spec.selected_fill_token(), semantic::COLOR_ACCENT_BASE);

        // Controlled value overrides default value
        spec.value = Some("list".to_string());
        assert_eq!(spec.current_value(), Some("list"));
    }

    #[test]
    fn option_builders_icon_only_and_fallbacks() {
        // Plain labelled option
        let plain = SegmentedControlOption::new("grid", "Grid");
        assert_eq!(plain.value, "grid");
        assert_eq!(plain.label, "Grid");
        assert_eq!(plain.icon, None);
        assert!(!plain.icon_only);
        assert!(!plain.is_disabled);
        assert!(!plain.is_icon_only());
        assert_eq!(plain.accessible_name_override(), None);
        assert_eq!(plain.tooltip_text(), None);

        // Labelled option with icon
        let with_icon = SegmentedControlOption::new("list", "List")
            .with_icon("view-list")
            .with_disabled(true);
        assert_eq!(with_icon.icon.as_deref(), Some("view-list"));
        assert!(!with_icon.is_icon_only());
        assert!(with_icon.is_disabled);
        assert_eq!(with_icon.accessible_name_override(), None);
        assert_eq!(with_icon.tooltip_text(), None);

        // Icon-only option: icon_only flag + icon present -> falls back to label for accessible name and tooltip
        let icon_only = SegmentedControlOption::new("table", "Table")
            .with_icon("view-table")
            .with_icon_only(true);
        assert!(icon_only.is_icon_only());
        assert_eq!(icon_only.accessible_name_override(), Some("Table"));
        assert_eq!(icon_only.tooltip_text(), Some("Table"));

        // Icon-only requested but no icon provided -> is_icon_only is false
        let pseudo_icon_only = SegmentedControlOption::new("cards", "Cards")
            .with_icon_only(true);
        assert!(!pseudo_icon_only.is_icon_only());
        assert_eq!(pseudo_icon_only.accessible_name_override(), None);
        assert_eq!(pseudo_icon_only.tooltip_text(), None);

        // Explicit aria_label and title overrides
        let explicit = SegmentedControlOption::new("custom", "C")
            .with_icon("view-custom")
            .with_icon_only(true)
            .with_aria_label("Custom View")
            .with_title("Switch to custom view");
        assert_eq!(explicit.accessible_name_override(), Some("Custom View"));
        assert_eq!(explicit.tooltip_text(), Some("Switch to custom view"));
    }
}

