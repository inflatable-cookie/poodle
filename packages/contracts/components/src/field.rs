use poodle_tokens::semantic;

use crate::types::ValidationState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldRelationships {
    pub described_by: Option<String>,
    pub description_id: Option<String>,
    pub error_id: Option<String>,
    pub message_id: Option<String>,
    pub validation_state: ValidationState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldSpec {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub pending_message: Option<String>,
    pub validation_state: ValidationState,
    pub is_required: bool,
    pub optional_label: Option<String>,
}

impl FieldSpec {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            error: None,
            pending_message: None,
            validation_state: ValidationState::None,
            is_required: false,
            optional_label: Some(String::from("Optional")),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_pending_message(mut self, pending_message: impl Into<String>) -> Self {
        self.pending_message = Some(pending_message.into());
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn with_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    pub fn with_optional_label(mut self, optional_label: impl Into<String>) -> Self {
        self.optional_label = Some(optional_label.into());
        self
    }

    pub fn description_id(&self) -> Option<String> {
        self.description
            .as_ref()
            .map(|_| format!("{}-description", self.id))
    }

    pub fn error_id(&self) -> Option<String> {
        self.error.as_ref().map(|_| format!("{}-error", self.id))
    }

    pub fn pending_id(&self) -> Option<String> {
        self.pending_message
            .as_ref()
            .map(|_| format!("{}-pending", self.id))
    }

    pub fn message_id(&self) -> Option<String> {
        match self.validation_state {
            ValidationState::Invalid => self.error_id(),
            ValidationState::Pending => self.pending_id(),
            ValidationState::None | ValidationState::Valid => None,
        }
    }

    pub fn described_by(&self) -> Option<String> {
        let ids = [self.description_id(), self.message_id()]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if ids.is_empty() {
            None
        } else {
            Some(ids.join(" "))
        }
    }

    pub fn relationships(&self) -> FieldRelationships {
        FieldRelationships {
            described_by: self.described_by(),
            description_id: self.description_id(),
            error_id: self.error_id(),
            message_id: self.message_id(),
            validation_state: self.validation_state,
        }
    }

    pub fn shows_optional_label(&self) -> bool {
        !self.is_required
            && self
                .optional_label
                .as_ref()
                .map(|label| !label.trim().is_empty())
                .unwrap_or(false)
    }

    pub fn label_typography_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    /// Helper/optional/error/pending copy at the `md` size stop (contract §7: `0.75rem`).
    pub fn supporting_text_typography_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_COUNTER_SIZE
    }

    pub fn error_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn description_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn row_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn header_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    /// Inline gap inside the label row (contract §7: `0.375rem`).
    pub fn label_row_gap_token(&self) -> &'static str {
        semantic::SPACE_BUTTON_GAP
    }
}
