use pug_gpui_tokens::semantic;

/// Tone for a toast notification.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastTone {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

/// Position for the toast stack on screen.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToastPosition {
    TopRight,
    TopLeft,
    #[default]
    BottomRight,
    BottomLeft,
}

/// A single toast notification entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Toast {
    pub id: String,
    pub title: String,
    pub message: Option<String>,
    pub tone: ToastTone,
    pub action_label: Option<String>,
}

impl Toast {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            message: None,
            tone: ToastTone::default(),
            action_label: None,
        }
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_tone(mut self, tone: ToastTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_action_label(mut self, action_label: impl Into<String>) -> Self {
        self.action_label = Some(action_label.into());
        self
    }
}

/// ToastStack — a notification stack for transient messages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToastStackSpec {
    pub toasts: Vec<Toast>,
    pub position: ToastPosition,
}

impl ToastStackSpec {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            position: ToastPosition::default(),
        }
    }

    pub fn with_toasts(mut self, toasts: Vec<Toast>) -> Self {
        self.toasts = toasts;
        self
    }

    pub fn add_toast(mut self, toast: Toast) -> Self {
        self.toasts.push(toast);
        self
    }

    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn message_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn padding_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    pub fn dismiss_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn tone_color(&self, tone: &ToastTone) -> &'static str {
        match tone {
            ToastTone::Info => semantic::COLOR_ACCENT_BASE,
            ToastTone::Success => semantic::COLOR_STATUS_SUCCESS,
            ToastTone::Warning => semantic::COLOR_STATUS_WARNING,
            ToastTone::Danger => semantic::COLOR_STATUS_DANGER,
        }
    }
}
