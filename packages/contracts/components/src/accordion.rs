use poodle_tokens::semantic;

use crate::types::{AccordionItemSpec, AccordionSelectionValue, ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccordionSpec {
    pub items: Vec<AccordionItemSpec>,
    pub value: Option<AccordionSelectionValue>,
    pub default_value: Option<AccordionSelectionValue>,
    pub allow_multiple: bool,
    pub is_collapsible: bool,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for AccordionSpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            value: None,
            default_value: None,
            allow_multiple: false,
            is_collapsible: true,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl AccordionSpec {
    pub fn new(items: Vec<AccordionItemSpec>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: AccordionSelectionValue) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_default_value(mut self, default_value: AccordionSelectionValue) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn with_allow_multiple(mut self, allow_multiple: bool) -> Self {
        self.allow_multiple = allow_multiple;
        self
    }

    pub fn with_collapsible(mut self, is_collapsible: bool) -> Self {
        self.is_collapsible = is_collapsible;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_value(&self) -> Option<&AccordionSelectionValue> {
        self.value.as_ref().or(self.default_value.as_ref())
    }

    pub fn expanded_values(&self) -> Vec<&str> {
        match self.current_value() {
            Some(AccordionSelectionValue::Single(value)) => vec![value.as_str()],
            Some(AccordionSelectionValue::Multiple(values)) => {
                values.iter().map(String::as_str).collect()
            }
            None => Vec::new(),
        }
    }

    pub fn expanded_item_count(&self) -> usize {
        self.expanded_values().len()
    }

    pub fn trigger_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn item_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn border_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
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
