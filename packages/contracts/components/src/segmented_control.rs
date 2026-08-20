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
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Optional explicit instance scope, analogue of the web `name` prop.
    /// Semantic option ids stay readable (`segmented:<value>`). Focus keys are
    /// always `segmented:{scope}:option:{value}` — never the unscoped option
    /// id. When omitted, shared render allocates a unique per-frame serial.
    pub instance_id: Option<String>,
}

impl Default for SegmentedControlSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            options: Vec::new(),
            is_disabled: false,
            aria_label: None,
            equal_width: true,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            instance_id: None,
        }
    }
}

impl SegmentedControlSpec {
    pub fn new(options: Vec<SegmentedControlOption>) -> Self {
        Self {
            options,
            ..Self::default()
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
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.instance_id = Some(instance_id.into());
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
