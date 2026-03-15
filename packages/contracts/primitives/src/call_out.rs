use pug_tokens::semantic;

use crate::types::StatusTone;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallOutSpec {
    pub tone: StatusTone,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl Default for CallOutSpec {
    fn default() -> Self {
        Self {
            tone: StatusTone::Info,
            title: None,
            content: None,
        }
    }
}

impl CallOutSpec {
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

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
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
}
