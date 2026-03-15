use pug_tokens::semantic;

use crate::types::StatusTone;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BannerSpec {
    pub tone: StatusTone,
    pub title: Option<String>,
    pub message: Option<String>,
    pub is_dismissible: bool,
    pub has_icon: bool,
}

impl Default for BannerSpec {
    fn default() -> Self {
        Self {
            tone: StatusTone::Info,
            title: None,
            message: None,
            is_dismissible: false,
            has_icon: true,
        }
    }
}

impl BannerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tone(mut self, tone: StatusTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_dismissible(mut self, is_dismissible: bool) -> Self {
        self.is_dismissible = is_dismissible;
        self
    }

    pub fn with_icon(mut self, has_icon: bool) -> Self {
        self.has_icon = has_icon;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        match self.tone {
            StatusTone::Success => semantic::COLOR_STATUS_SUCCESS,
            StatusTone::Warning => semantic::COLOR_STATUS_WARNING,
            StatusTone::Danger => semantic::COLOR_STATUS_DANGER,
            StatusTone::Info | StatusTone::Neutral | StatusTone::Pending => {
                semantic::COLOR_ACCENT_BASE
            }
        }
    }

    pub fn border_token(&self) -> &'static str {
        match self.tone {
            StatusTone::Success => semantic::COLOR_STATUS_SUCCESS,
            StatusTone::Warning => semantic::COLOR_STATUS_WARNING,
            StatusTone::Danger => semantic::COLOR_STATUS_DANGER,
            StatusTone::Info | StatusTone::Neutral | StatusTone::Pending => {
                semantic::COLOR_ACCENT_BASE
            }
        }
    }

    pub fn icon_color_token(&self) -> &'static str {
        match self.tone {
            StatusTone::Success => semantic::COLOR_STATUS_SUCCESS,
            StatusTone::Warning => semantic::COLOR_STATUS_WARNING,
            StatusTone::Danger => semantic::COLOR_STATUS_DANGER,
            StatusTone::Info | StatusTone::Neutral | StatusTone::Pending => {
                semantic::COLOR_ACCENT_BASE
            }
        }
    }
}
