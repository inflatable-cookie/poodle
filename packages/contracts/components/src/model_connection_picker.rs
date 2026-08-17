use poodle_headless::model_connection::{ModelConnectionOption, ModelConnectionPickerState};

use crate::composite_types::PickerVariant;
use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// ModelConnectionPicker — a searchable, grouped radio-card picker for one
/// exact configured model route.
///
/// Contract: `docs/contracts/components/model-connection-picker.md`
///
/// Controlled display data only. `value` and `query` are the current values
/// the host owns; the web `defaultValue`/`defaultQuery` seeds have no Rust
/// counterpart because GPUI/AppState holds the current value and rerenders
/// after a callback requests a change. The provider mark is host-composed
/// content keyed by option id (see `poodle_render::model_connection_picker`);
/// Poodle resolves no provider catalogue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelConnectionPickerSpec {
    /// Host-ordered exact route options. Filtering preserves this order.
    pub options: Vec<ModelConnectionOption>,
    /// Controlled selected option id.
    pub value: Option<String>,
    /// Controlled search text.
    pub query: String,
    /// Supplied catalogue posture.
    pub state: ModelConnectionPickerState,
    pub title: String,
    pub description: Option<String>,
    pub search_placeholder: String,
    /// Root name; falls back to `title`.
    pub aria_label: Option<String>,
    /// Disables search and every option; the selection is retained.
    pub is_disabled: bool,
    pub variant: PickerVariant,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ModelConnectionPickerSpec {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            value: None,
            query: String::new(),
            state: ModelConnectionPickerState::Ready,
            title: "Choose a connection".to_string(),
            description: None,
            search_placeholder: "Search connections".to_string(),
            aria_label: None,
            is_disabled: false,
            variant: PickerVariant::Inline,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl ModelConnectionPickerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(mut self, options: Vec<ModelConnectionOption>) -> Self {
        self.options = options;
        self
    }

    pub fn with_option(mut self, option: ModelConnectionOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn with_value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    pub fn with_state(mut self, state: ModelConnectionPickerState) -> Self {
        self.state = state;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_search_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.search_placeholder = placeholder.into();
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_variant(mut self, variant: PickerVariant) -> Self {
        self.variant = variant;
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

    /// The root's accessible name: the explicit label, else the title.
    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.title)
    }
}
