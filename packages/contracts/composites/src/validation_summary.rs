use pug_primitives::ValidationState;
use pug_tokens::semantic;

use crate::types::{AnnouncementMode, ValidationSummaryEntry};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationSummarySpec {
    pub title: Option<String>,
    pub entries: Vec<ValidationSummaryEntry>,
    pub announce_mode: AnnouncementMode,
    pub include_pending: bool,
}

impl Default for ValidationSummarySpec {
    fn default() -> Self {
        Self {
            title: None,
            entries: Vec::new(),
            announce_mode: AnnouncementMode::Polite,
            include_pending: false,
        }
    }
}

impl ValidationSummarySpec {
    pub fn new(entries: Vec<ValidationSummaryEntry>) -> Self {
        Self {
            entries,
            ..Self::default()
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_announce_mode(mut self, announce_mode: AnnouncementMode) -> Self {
        self.announce_mode = announce_mode;
        self
    }

    pub fn with_include_pending(mut self, include_pending: bool) -> Self {
        self.include_pending = include_pending;
        self
    }

    pub fn active_entries(&self) -> Vec<&ValidationSummaryEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.validation_state == ValidationState::Invalid
                    || (self.include_pending && entry.validation_state == ValidationState::Pending)
            })
            .collect()
    }

    pub fn blocking_entry_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.is_blocking())
            .count()
    }

    pub fn accessibility_role(&self) -> Option<&'static str> {
        self.announce_mode.accessibility_role()
    }

    pub fn border_token(&self) -> &'static str {
        if self.blocking_entry_count() > 0 {
            semantic::COLOR_STATUS_DANGER
        } else {
            semantic::COLOR_ACCENT_BASE
        }
    }
}
