use poodle_tokens::semantic;
use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub is_disabled: bool,
}

impl ComboboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            is_disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ComboboxSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub options: Vec<ComboboxOption>,
    pub placeholder: Option<String>,
    pub is_disabled: bool,
    pub is_open: bool,
    pub query: String,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ComboboxSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            options: Vec::new(),
            placeholder: None,
            is_disabled: false,
            is_open: false,
            query: String::new(),
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl ComboboxSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn with_options(mut self, options: Vec<ComboboxOption>) -> Self {
        self.options = options;
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn add_option(mut self, option: ComboboxOption) -> Self {
        self.options.push(option);
        self
    }

    // ── Helpers ────────────────────────────────────────────────

    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
    }

    pub fn filtered_options(&self) -> Vec<&ComboboxOption> {
        if self.query.is_empty() {
            return self.options.iter().collect();
        }
        let query_lower = self.query.to_lowercase();
        self.options
            .iter()
            .filter(|opt| opt.label.to_lowercase().contains(&query_lower))
            .collect()
    }

    pub fn selected_label(&self) -> Option<&str> {
        let current = self.current_value()?;
        self.options
            .iter()
            .find(|opt| opt.value == current)
            .map(|opt| opt.label.as_str())
    }

    // ── Token methods ──────────────────────────────────────────

    pub fn input_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn input_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn input_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn input_placeholder_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn input_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn input_height_token(&self) -> &'static str {
        semantic::SIZE_CONTROL_HEIGHT
    }

    pub fn list_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn list_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn list_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn list_shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }

    pub fn option_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn option_highlight_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn option_description_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn empty_text_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn body_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn body_line_height_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_LINE_HEIGHT
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
