use poodle_tokens::semantic;

use crate::types::{
    AccordionItemSpec, AccordionSelectionValue, ControlDensity, ControlSize,
    SemanticControlSizeRole,
};

/// Whether one panel opens at a time or several.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AccordionSelectionMode {
    #[default]
    Single,
    Multiple,
}

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
    /// Whether one panel opens at a time or several (contract §3).
    pub selection_mode: AccordionSelectionMode,
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
            selection_mode: AccordionSelectionMode::Single,
        }
    }
}

impl AccordionSpec {
    pub fn with_selection_mode(mut self, value: AccordionSelectionMode) -> Self {
        self.selection_mode = value;
        self
    }

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

    /// Root gap between items (contract §8 Root `gap` = `space.stack.md`).
    pub fn root_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }

    /// Summary gap, title ↔ description (contract §8 Summary `gap` = `space.inline.sm`).
    pub fn summary_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    /// Trigger grid gap, summary ↔ indicator (contract §8 Trigger `gap` = `space.inline.md`).
    pub fn trigger_grid_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    /// Item internal gap, trigger ↔ panel (contract §8 Item `gap`, `0.625rem`).
    pub fn item_internal_gap_rem(&self) -> f32 {
        0.625
    }

    /// Item block (vertical) padding (contract §8 Item `padding` block = `0.625rem`).
    pub fn block_padding_rem(&self) -> f32 {
        0.625
    }

    /// Item inline (horizontal) padding for the resolved density
    /// (contract §8 Item `padding` inline = `space-panel-x`, density-overridden:
    /// compact `0.5rem`, default/comfortable `1rem`).
    pub fn inline_padding_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.0,
        }
    }

    /// Item background source tokens + mix ratio
    /// (contract §8 Item `background` = `color-mix(elevated 40%, panel)`).
    pub fn item_bg_elevated_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn item_bg_panel_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn item_bg_elevated_ratio(&self) -> f32 {
        0.40
    }

    /// Item border alpha fraction (contract §8 Item `border` = `border-subtle 36%`).
    pub fn border_subtle_alpha(&self) -> f32 {
        0.36
    }

    /// Inset top-highlight alpha + offset (contract §8 Item `box-shadow`
    /// = `inset 0 0.0625rem 0 text-inverse 8%`).
    pub fn inset_highlight_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_INVERSE
    }

    pub fn inset_highlight_alpha(&self) -> f32 {
        0.08
    }

    pub fn inset_highlight_offset_rem(&self) -> f32 {
        0.0625
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
