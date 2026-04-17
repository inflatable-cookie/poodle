//! `CallOutSpec` — spec for the `Callout` component. File is named `call_out.rs`
//! for Rust naming consistency; the contract lives at
//! `docs/contracts/components/callout.md` and the Svelte component is
//! `Callout.svelte`. Not an orphan — just a naming discrepancy.

use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallOutSpec {
    pub tone: StatusTone,
    pub title: Option<String>,
    pub content: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for CallOutSpec {
    fn default() -> Self {
        Self {
            tone: StatusTone::Info,
            title: None,
            content: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
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
