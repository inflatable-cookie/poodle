use poodle_headless::model_connection::{
    ModelConnectionOption, ModelConnectionPickerState, ModelConnectionSetupContext,
    ModelConnectionSetupStage,
};

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// ModelConnectionSetup — an adaptive shell for choosing one exact model
/// connection and, when required, completing host-owned setup.
///
/// Contract: `docs/contracts/components/model-connection-setup.md`
///
/// Controlled display data only. The configuration body is host-composed
/// content passed to the renderer, never a field here: Poodle must never see
/// a credential, a provider form schema, or a validation result. `error` and
/// `success` are host-sanitised copy.
///
/// The web `defaultStage`/`defaultValue` seeds have no Rust counterpart —
/// GPUI/AppState owns the current stage and value and rerenders after a
/// callback requests a change.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelConnectionSetupSpec {
    /// Controlled workflow stage.
    pub stage: ModelConnectionSetupStage,
    /// Forwarded to the picker; `requires_configuration` selects the direct
    /// or the configured flow.
    pub options: Vec<ModelConnectionOption>,
    pub value: Option<String>,
    pub query: String,
    pub picker_state: ModelConnectionPickerState,
    pub title: String,
    pub description: Option<String>,
    /// Host-approved Add eligibility. Poodle never validates.
    pub can_submit: bool,
    /// Locks every workflow action while the host works.
    pub is_pending: bool,
    pub pending_label: String,
    /// Safe form-level error copy.
    pub error: Option<String>,
    /// Safe form-level success copy.
    pub success: Option<String>,
    pub continue_label: String,
    pub submit_label: String,
    pub back_label: String,
    pub cancel_label: String,
    pub aria_label: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for ModelConnectionSetupSpec {
    fn default() -> Self {
        Self {
            stage: ModelConnectionSetupStage::Choose,
            options: Vec::new(),
            value: None,
            query: String::new(),
            picker_state: ModelConnectionPickerState::Ready,
            title: "Add model connection".to_string(),
            description: None,
            can_submit: false,
            is_pending: false,
            pending_label: "Checking connection".to_string(),
            error: None,
            success: None,
            continue_label: "Continue".to_string(),
            submit_label: "Add connection".to_string(),
            back_label: "Back".to_string(),
            cancel_label: "Cancel".to_string(),
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl ModelConnectionSetupSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_stage(mut self, stage: ModelConnectionSetupStage) -> Self {
        self.stage = stage;
        self
    }

    pub fn with_options(mut self, options: Vec<ModelConnectionOption>) -> Self {
        self.options = options;
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

    pub fn with_picker_state(mut self, picker_state: ModelConnectionPickerState) -> Self {
        self.picker_state = picker_state;
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

    pub fn with_can_submit(mut self, can_submit: bool) -> Self {
        self.can_submit = can_submit;
        self
    }

    pub fn with_pending(mut self, is_pending: bool) -> Self {
        self.is_pending = is_pending;
        self
    }

    pub fn with_pending_label(mut self, pending_label: impl Into<String>) -> Self {
        self.pending_label = pending_label.into();
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_success(mut self, success: impl Into<String>) -> Self {
        self.success = Some(success.into());
        self
    }

    pub fn with_continue_label(mut self, label: impl Into<String>) -> Self {
        self.continue_label = label.into();
        self
    }

    pub fn with_submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }

    pub fn with_back_label(mut self, label: impl Into<String>) -> Self {
        self.back_label = label.into();
        self
    }

    pub fn with_cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
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

    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.title)
    }

    /// The pure behaviour context this spec describes. Every stage guard and
    /// transition runs through `poodle_headless::model_connection`, so the
    /// renderer and any host driving the same spec agree by construction.
    pub fn behaviour_context(&self) -> ModelConnectionSetupContext {
        ModelConnectionSetupContext {
            stage: self.stage,
            value: self.value.clone(),
            query: self.query.clone(),
            options: self.options.clone(),
            can_submit: self.can_submit,
            is_pending: self.is_pending,
        }
    }
}
