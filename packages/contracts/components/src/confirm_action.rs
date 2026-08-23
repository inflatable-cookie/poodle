use crate::{ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfirmActionSpec {
    pub title: String,
    pub description: String,
    /// Label for the default trigger button (the button that opens
    /// the dialog). Ignored when a custom trigger element is
    /// provided via a slot / builder. Matches `triggerLabel` prop on
    /// the contract doc.
    pub trigger_label: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub tone: StatusTone,
    pub is_open: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl ConfirmActionSpec {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        confirm_label: impl Into<String>,
        cancel_label: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            trigger_label: String::from("Delete"),
            confirm_label: confirm_label.into(),
            cancel_label: cancel_label.into(),
            tone: StatusTone::Neutral,
            is_open: false,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_tone(mut self, tone: StatusTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_trigger_label(mut self, trigger_label: impl Into<String>) -> Self {
        self.trigger_label = trigger_label.into();
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn is_destructive(&self) -> bool {
        self.tone == StatusTone::Danger
    }

    pub fn backdrop_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_OVERLAY
    }

    pub fn confirm_fill_token(&self) -> &'static str {
        match self.tone {
            StatusTone::Danger => semantic::COLOR_STATUS_DANGER,
            StatusTone::Warning => semantic::COLOR_STATUS_WARNING,
            _ => semantic::COLOR_ACCENT_BASE,
        }
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
}
