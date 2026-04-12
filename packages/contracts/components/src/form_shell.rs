use crate::{FormActionAlign, StatusTone, ValidationState};
use poodle_tokens::semantic;

use crate::composite_types::{
    FormActionLayout, FormFieldState, FormSectionSpec, FormStatusSummary,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormShellSpec {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub sections: Vec<FormSectionSpec>,
    pub fields: Vec<FormFieldState>,
    pub actions: FormActionLayout,
    pub status_summary: Option<FormStatusSummary>,
    pub is_disabled: bool,
    pub is_busy: bool,
}

impl FormShellSpec {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: None,
            description: None,
            sections: Vec::new(),
            fields: Vec::new(),
            actions: FormActionLayout::new(FormActionAlign::End, 0),
            status_summary: None,
            is_disabled: false,
            is_busy: false,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_sections(mut self, sections: Vec<FormSectionSpec>) -> Self {
        self.sections = sections;
        self
    }

    pub fn with_fields(mut self, fields: Vec<FormFieldState>) -> Self {
        self.fields = fields;
        self
    }

    pub fn with_actions(mut self, align: FormActionAlign, action_count: usize) -> Self {
        self.actions = FormActionLayout::new(align, action_count);
        self
    }

    pub fn with_status_summary(mut self, tone: StatusTone, message: impl Into<String>) -> Self {
        self.status_summary = Some(FormStatusSummary::new(tone, message));
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_busy(mut self, is_busy: bool) -> Self {
        self.is_busy = is_busy;
        self
    }

    pub fn invalid_field_count(&self) -> usize {
        self.fields
            .iter()
            .filter(|field| field.validation_state == ValidationState::Invalid)
            .count()
    }

    pub fn pending_field_count(&self) -> usize {
        self.fields
            .iter()
            .filter(|field| field.validation_state == ValidationState::Pending)
            .count()
    }

    pub fn blocks_submission(&self) -> bool {
        self.is_disabled || self.is_busy || self.invalid_field_count() > 0
    }

    pub fn resolved_status_tone(&self) -> StatusTone {
        if self.invalid_field_count() > 0 {
            StatusTone::Danger
        } else if self.pending_field_count() > 0 || self.is_busy {
            StatusTone::Pending
        } else {
            self.status_summary
                .as_ref()
                .map(|summary| summary.tone)
                .unwrap_or(StatusTone::Neutral)
        }
    }

    pub fn stack_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_LG
    }

    pub fn section_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }
}
