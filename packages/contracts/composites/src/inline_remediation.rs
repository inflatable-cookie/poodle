use pug_primitives::StatusTone;
use pug_tokens::semantic;

use crate::types::RemediationAction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineRemediationSpec {
    pub tone: StatusTone,
    pub title: Option<String>,
    pub message: String,
    pub referenced_field_ids: Vec<String>,
    pub action: Option<RemediationAction>,
}

impl InlineRemediationSpec {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            tone: StatusTone::Info,
            title: None,
            message: message.into(),
            referenced_field_ids: Vec::new(),
            action: None,
        }
    }

    pub fn with_tone(mut self, tone: StatusTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_referenced_field_ids(mut self, referenced_field_ids: Vec<String>) -> Self {
        self.referenced_field_ids = referenced_field_ids;
        self
    }

    pub fn with_action(mut self, action: RemediationAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn reference_count(&self) -> usize {
        self.referenced_field_ids.len()
    }

    pub fn is_actionable(&self) -> bool {
        self.action.is_some()
    }

    pub fn border_token(&self) -> &'static str {
        self.tone.color_token()
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }
}
