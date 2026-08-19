//! `CallOutSpec` — spec for the `Callout` component. File is named `call_out.rs`
//! for Rust naming consistency; the contract lives at
//! `docs/contracts/components/callout.md` and the Svelte component is
//! `Callout.svelte`. Not an orphan — just a naming discrepancy.

use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone, ToneFill};

/// ARIA live-region behavior for the callout (contract §3 `CalloutAnnounceMode`).
///
/// `None` — no role / aria-live (default).
/// `Polite` — `role="status"`, `aria-live="polite"`.
/// `Assertive` — `role="alert"`, `aria-live="assertive"`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalloutAnnounceMode {
    #[default]
    None,
    Polite,
    Assertive,
}

impl CalloutAnnounceMode {
    /// ARIA `role` derived from the announce mode, if any.
    pub fn role(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Polite => Some("status"),
            Self::Assertive => Some("alert"),
        }
    }

    /// ARIA `aria-live` value derived from the announce mode, if any.
    pub fn aria_live(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Polite => Some("polite"),
            Self::Assertive => Some("assertive"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallOutSpec {
    pub tone: StatusTone,
    pub fill: ToneFill,
    pub title: Option<String>,
    pub content: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Shows the dismiss (close) button when true (contract §3).
    pub dismissible: bool,
    /// Accessible label for the dismiss button (contract §3 default).
    pub dismiss_label: String,
    /// ARIA live-region behavior (contract §3 / §6).
    pub announce_mode: CalloutAnnounceMode,
    /// Optional accessible label for the callout region (contract §3).
    pub aria_label: Option<String>,
}

impl Default for CallOutSpec {
    fn default() -> Self {
        Self {
            tone: StatusTone::Neutral,
            fill: ToneFill::Tint,
            title: None,
            content: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            dismissible: false,
            dismiss_label: "Dismiss message".to_string(),
            announce_mode: CalloutAnnounceMode::None,
            aria_label: None,
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

    pub fn with_fill(mut self, fill: ToneFill) -> Self {
        self.fill = fill;
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

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    pub fn with_dismiss_label(mut self, label: impl Into<String>) -> Self {
        self.dismiss_label = label.into();
        self
    }

    pub fn with_announce_mode(mut self, mode: CalloutAnnounceMode) -> Self {
        self.announce_mode = mode;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Tone base color token used as the mix source for fill and border.
    ///
    /// Matches `StatusTone::color_token()` (info → dedicated status-info,
    /// pending → accent-base, neutral → text-secondary). The actual
    /// `color-mix` ratios live in the platform component code per contract §8
    /// (neutral: panel 94% / border-subtle 88%; pending: accent 8% / 26%;
    /// info/success/warning/danger: status 10% / 34%).
    pub fn tone_color_token(&self) -> &'static str {
        self.tone.color_token()
    }

    /// True when the tone has no accent/status tint (neutral): fill resolves to
    /// `panel 94%` and border to `border-subtle 88%` instead of a tone mix.
    pub fn is_neutral_tone(&self) -> bool {
        matches!(self.tone, StatusTone::Neutral)
    }

    /// True when the tone is `pending`: fill/border use the lighter accent
    /// `8%` / `26%` mix and the icon badge hosts a ring spinner (contract §6/§8).
    pub fn is_pending_tone(&self) -> bool {
        matches!(self.tone, StatusTone::Pending)
    }

    pub fn is_solid_fill(&self) -> bool {
        matches!(self.fill, ToneFill::Solid)
    }

    /// Token for the neutral-tone fill source (panel) before the 94% mix.
    pub fn neutral_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Token for the neutral-tone border source (border-subtle) before the 88% mix.
    pub fn neutral_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Background token for the circular icon badge (surface at 78% — contract §8).
    pub fn icon_badge_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// Border-width token for the root border (`0.0625rem`, contract §8).
    pub fn border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }

    /// Background panel token used as the mix target for fill (contract §8).
    pub fn fill_mix_target_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    /// Border-default token used as the mix target for tone borders (contract §8).
    pub fn border_mix_target_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    /// Tone fill color token (mix source). For toned variants this is the
    /// status/accent base; neutral falls back to the panel token. The platform
    /// component applies the per-tone mix ratio.
    pub fn fill_token(&self) -> &'static str {
        if self.is_neutral_tone() {
            semantic::COLOR_BACKGROUND_PANEL
        } else {
            self.tone.color_token()
        }
    }

    /// Tone border color token (mix source). Neutral falls back to border-subtle.
    pub fn border_token(&self) -> &'static str {
        if self.is_neutral_tone() {
            semantic::COLOR_BORDER_SUBTLE
        } else {
            self.tone.color_token()
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
