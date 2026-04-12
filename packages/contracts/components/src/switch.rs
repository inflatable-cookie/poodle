use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// Tone for a Switch's left/right labels. Resolves to a token-side color
/// used to tint the corresponding track side (off-color for `left_tone`,
/// on-color for `right_tone`) when no explicit custom color is supplied.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SwitchTone {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
}

impl SwitchTone {
    /// Semantic token that backs this tone. `Default` returns `None`
    /// meaning "use the Switch's native track colour".
    pub fn color_token(self) -> Option<&'static str> {
        match self {
            Self::Default => None,
            Self::Primary => Some(semantic::COLOR_ACCENT_BASE),
            Self::Success => Some(semantic::COLOR_STATUS_SUCCESS),
            Self::Warning => Some(semantic::COLOR_STATUS_WARNING),
            Self::Danger => Some(semantic::COLOR_STATUS_DANGER),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwitchSpec {
    pub checked: Option<bool>,
    pub default_checked: bool,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub label: Option<String>,
    /// Left-side label for dual-label mode. When either `left_label` or
    /// `right_label` is set the Switch renders in dual-label layout
    /// (label, track, label) instead of (track, label).
    pub left_label: Option<String>,
    /// Right-side label for dual-label mode.
    pub right_label: Option<String>,
    pub aria_label: Option<String>,
    /// Custom color for the on (checked) track state (CSS hex string).
    pub on_color: Option<String>,
    /// Custom color for the off (unchecked) track state (CSS hex string).
    pub off_color: Option<String>,
    /// Tone applied to the left (off) track side. Only takes effect when
    /// `off_color` is not set — a custom colour always wins.
    pub left_tone: SwitchTone,
    /// Tone applied to the right (on) track side. Only takes effect when
    /// `on_color` is not set.
    pub right_tone: SwitchTone,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for SwitchSpec {
    fn default() -> Self {
        Self {
            checked: None,
            default_checked: false,
            is_disabled: false,
            is_read_only: false,
            label: None,
            left_label: None,
            right_label: None,
            aria_label: None,
            on_color: None,
            off_color: None,
            left_tone: SwitchTone::Default,
            // Right tone defaults to Primary to match the Svelte prop default.
            right_tone: SwitchTone::Primary,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl SwitchSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn with_default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn with_on_color(mut self, color: impl Into<String>) -> Self {
        self.on_color = Some(color.into());
        self
    }

    pub fn with_off_color(mut self, color: impl Into<String>) -> Self {
        self.off_color = Some(color.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_left_label(mut self, left_label: impl Into<String>) -> Self {
        self.left_label = Some(left_label.into());
        self
    }

    pub fn with_right_label(mut self, right_label: impl Into<String>) -> Self {
        self.right_label = Some(right_label.into());
        self
    }

    pub fn with_left_tone(mut self, tone: SwitchTone) -> Self {
        self.left_tone = tone;
        self
    }

    pub fn with_right_tone(mut self, tone: SwitchTone) -> Self {
        self.right_tone = tone;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_read_only(mut self, is_read_only: bool) -> Self {
        self.is_read_only = is_read_only;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    /// Whether either dual-label side is populated.
    pub fn is_dual_label(&self) -> bool {
        self.left_label.is_some() || self.right_label.is_some()
    }

    pub fn current_checked(&self) -> bool {
        self.checked.unwrap_or(self.default_checked)
    }

    pub fn track_fill_token(&self) -> &'static str {
        if self.current_checked() {
            semantic::COLOR_ACCENT_BASE
        } else {
            semantic::COLOR_BACKGROUND_SURFACE
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
