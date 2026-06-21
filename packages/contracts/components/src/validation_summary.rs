use crate::ValidationState;
use poodle_tokens::semantic;

use crate::composite_types::{AnnouncementMode, ValidationSummaryEntry};

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

    /// Surface fill behind the summary (contract §2 Root background). Tinted
    /// toward the tone color by the renderer; this is the neutral base.
    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Surface corner radius (contract §2 Root radius).
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    /// Inline (left/right) padding of the surface (contract §2 Root padding).
    pub fn padding_x_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    /// Block (top/bottom) padding of the surface (contract §2 Root padding).
    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }

    /// Gap between the title and the list, and between list rows
    /// (contract §2/§6 list spacing).
    pub fn list_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    /// Inline gap inside an entry row between the indicator and the text column
    /// (contract §2/§6 Entry spacing).
    pub fn entry_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    /// Gap between an entry's label and its message (contract §2 Entry).
    pub fn entry_text_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_XS
    }

    /// Title typography size (contract §2/§6 typography-label).
    pub fn title_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }

    /// Entry label/message typography size (contract §2/§6 Entry text).
    pub fn entry_text_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }
}
