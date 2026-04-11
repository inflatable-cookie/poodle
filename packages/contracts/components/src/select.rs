use poodle_tokens::semantic;

use crate::types::{ChoiceOption, ControlDensity, ControlSize, SemanticControlSizeRole};

/// Controls how the Select renders its dropdown.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectMode {
    /// Auto: native unless searchable or custom rendering needed.
    Auto,
    /// Always native `<select>`.
    Native,
    /// Always custom dropdown.
    Custom,
}

impl Default for SelectMode {
    fn default() -> Self {
        Self::Auto
    }
}

/// Visual variant for the Select.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectVariant {
    /// Default variant with border, background, padding, and chevron.
    Default,
    /// Ghost variant strips all field chrome (border, background, shadow,
    /// padding, min-height) and hides the chevron indicator.
    Ghost,
}

impl Default for SelectVariant {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    pub options: Vec<ChoiceOption>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub description_id: Option<String>,
    pub open: Option<bool>,
    pub default_open: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Rendering mode: Auto, Native, or Custom.
    pub mode: SelectMode,
    /// When true, shows a filter/search input inside the dropdown.
    pub searchable: bool,
    /// When true, allows typing arbitrary values not in the options list.
    pub freeform: bool,
    /// Message shown when filtering produces no matches.
    pub empty_message: String,
    /// Visual variant: Default or Ghost.
    pub variant: SelectVariant,
    /// Optional minimum width for the dropdown listbox (CSS length string, e.g. "12rem").
    /// When set, listbox uses `width: max-content` with viewport-aware anchor flipping.
    pub menu_min_width: Option<String>,
    /// When true, shows a clear button when a value is selected (custom dropdown only).
    pub clearable: bool,
    /// When true, the select is required for form submission. Surfaces
    /// as an Invalid validation state when left unset on submit.
    /// Matches Svelte `required`.
    pub is_required: bool,
    /// Current validation state. Drives border/focus-ring colour the
    /// same way TextInput does.
    pub validation_state: crate::types::ValidationState,
}

impl Default for SelectSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            placeholder: None,
            options: Vec::new(),
            is_disabled: false,
            aria_label: None,
            description_id: None,
            open: None,
            default_open: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            mode: SelectMode::default(),
            searchable: false,
            freeform: false,
            empty_message: String::from("No matches"),
            variant: SelectVariant::default(),
            menu_min_width: None,
            clearable: false,
            is_required: false,
            validation_state: crate::types::ValidationState::None,
        }
    }
}

impl SelectSpec {
    pub fn new(options: Vec<ChoiceOption>) -> Self {
        Self {
            options,
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

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn selected_option(&self) -> Option<&ChoiceOption> {
        let current = self.current_value()?;
        self.options.iter().find(|option| option.value == current)
    }

    pub fn trigger_text(&self) -> Option<&str> {
        self.selected_option()
            .map(|option| option.label.as_str())
            .or(self.placeholder.as_deref())
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
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

    pub fn with_mode(mut self, mode: SelectMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    pub fn with_freeform(mut self, freeform: bool) -> Self {
        self.freeform = freeform;
        self
    }

    pub fn with_empty_message(mut self, msg: impl Into<String>) -> Self {
        self.empty_message = msg.into();
        self
    }

    /// Returns true when the dropdown should render a search/filter input.
    /// This is the case when `searchable` is true or `freeform` is true.
    pub fn shows_search_input(&self) -> bool {
        self.searchable || self.freeform
    }

    /// Returns true when the select must use a custom dropdown
    /// (either explicitly requested or required by searchable/freeform).
    pub fn requires_custom_dropdown(&self) -> bool {
        match self.mode {
            SelectMode::Custom => true,
            SelectMode::Native => false,
            SelectMode::Auto => self.shows_search_input(),
        }
    }

    /// Set the visual variant (Default or Ghost).
    pub fn with_variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the dropdown listbox minimum width (CSS length string).
    pub fn with_menu_min_width(mut self, width: impl Into<String>) -> Self {
        self.menu_min_width = Some(width.into());
        self
    }

    /// Enable the clear button when a value is selected.
    pub fn with_clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    pub fn with_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn with_validation_state(mut self, state: crate::types::ValidationState) -> Self {
        self.validation_state = state;
        self
    }
}
