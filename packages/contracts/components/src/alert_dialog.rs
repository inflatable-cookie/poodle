use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertDialogTone {
    #[default]
    Danger,
    Warning,
}

#[derive(Debug, Clone)]
pub struct AlertDialogSpec {
    pub open: Option<bool>,
    pub title: String,
    pub description: Option<String>,
    pub tone: AlertDialogTone,
    pub confirm_label: String,
    pub cancel_label: String,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for AlertDialogSpec {
    fn default() -> Self {
        Self {
            open: None,
            title: String::new(),
            description: None,
            tone: AlertDialogTone::default(),
            confirm_label: "Confirm".to_string(),
            cancel_label: "Cancel".to_string(),
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl AlertDialogSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_tone(mut self, tone: AlertDialogTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub fn with_cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    // Token methods

    pub fn backdrop_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_OVERLAY
    }

    pub fn dialog_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn dialog_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn dialog_shadow_token(&self) -> &'static str {
        semantic::ELEVATION_DIALOG
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn description_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn confirm_fill_token(&self) -> &'static str {
        match self.tone {
            AlertDialogTone::Danger => semantic::COLOR_STATUS_DANGER,
            AlertDialogTone::Warning => semantic::COLOR_ACCENT_BASE,
        }
    }

    pub fn confirm_text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_INVERSE
    }

    pub fn cancel_text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn content_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }

    pub fn actions_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn padding_x_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn button_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
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
}
