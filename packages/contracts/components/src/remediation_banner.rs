use crate::StatusTone;
use poodle_tokens::semantic;

use crate::composite_types::{AnnouncementMode, RemediationAction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemediationBannerSpec {
    pub tone: StatusTone,
    pub title: String,
    pub message: String,
    pub announce_mode: AnnouncementMode,
    pub primary_action: Option<RemediationAction>,
    pub secondary_action: Option<RemediationAction>,
    pub is_dismissible: bool,
}

impl RemediationBannerSpec {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            tone: StatusTone::Warning,
            title: title.into(),
            message: message.into(),
            announce_mode: AnnouncementMode::Polite,
            primary_action: None,
            secondary_action: None,
            is_dismissible: false,
        }
    }

    pub fn with_tone(mut self, tone: StatusTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_announce_mode(mut self, announce_mode: AnnouncementMode) -> Self {
        self.announce_mode = announce_mode;
        self
    }

    pub fn with_primary_action(mut self, action: RemediationAction) -> Self {
        self.primary_action = Some(action);
        self
    }

    pub fn with_secondary_action(mut self, action: RemediationAction) -> Self {
        self.secondary_action = Some(action);
        self
    }

    pub fn with_dismissible(mut self, is_dismissible: bool) -> Self {
        self.is_dismissible = is_dismissible;
        self
    }

    pub fn action_count(&self) -> usize {
        [self.primary_action.as_ref(), self.secondary_action.as_ref()]
            .into_iter()
            .flatten()
            .count()
    }

    pub fn accessibility_role(&self) -> Option<&'static str> {
        self.announce_mode.accessibility_role()
    }

    pub fn border_token(&self) -> &'static str {
        self.tone.color_token()
    }

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }
}
