use crate::{StatusTone, ToneFill};
use poodle_tokens::semantic;

use crate::composite_types::{AnnouncementMode, RemediationAction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemediationBannerSpec {
    pub tone: StatusTone,
    pub fill: ToneFill,
    pub title: String,
    pub message: String,
    pub announce_mode: AnnouncementMode,
    pub primary_action: Option<RemediationAction>,
    pub secondary_action: Option<RemediationAction>,
    pub is_dismissible: bool,
    pub dismiss_label: String,
}

impl RemediationBannerSpec {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            tone: StatusTone::Warning,
            fill: ToneFill::Tint,
            title: title.into(),
            message: message.into(),
            announce_mode: AnnouncementMode::Polite,
            primary_action: None,
            secondary_action: None,
            is_dismissible: false,
            dismiss_label: "Dismiss".to_string(),
        }
    }

    pub fn with_tone(mut self, tone: StatusTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_fill(mut self, fill: ToneFill) -> Self {
        self.fill = fill;
        self
    }

    pub fn is_neutral_tone(&self) -> bool {
        matches!(self.tone, StatusTone::Neutral)
    }

    pub fn is_solid_fill(&self) -> bool {
        matches!(self.fill, ToneFill::Solid)
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

    pub fn with_dismiss_label(mut self, dismiss_label: impl Into<String>) -> Self {
        self.dismiss_label = dismiss_label.into();
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

    /// Icon color (contract §6: Icon = tone → `color.status.*`).
    pub fn icon_color_token(&self) -> &'static str {
        self.tone.color_token()
    }

    /// Root corner radius (contract §6: root `radius` target).
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    /// Root border width (contract §6: root `border` target).
    pub fn border_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_DEFAULT
    }

    /// Title color (contract §2 Content: `<strong>` heading).
    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Message color (contract §2 Content: `<p>` body).
    pub fn message_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Disabled-action opacity (contract §3 `RemediationAction.is_disabled`).
    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    /// Tone → leading icon name (contract §2 Icon: tone-based default).
    /// Shared so GPUI and Jetstream resolve the same glyph per tone.
    pub fn tone_icon_name(&self) -> &'static str {
        match self.tone {
            StatusTone::Neutral | StatusTone::Info => "info",
            StatusTone::Success => "check-circle",
            StatusTone::Warning => "alert-triangle",
            StatusTone::Danger => "x-circle",
            StatusTone::Pending => "loader",
        }
    }

    /// Proportion of the tone color kept when tinting the panel fill — the
    /// banner surface is `color-mix(tone, panel)` at this ratio (contract §6
    /// root bg is `color.background.panel`; the tone tint mirrors `Callout`).
    /// TOKEN GAP: no semantic surface-tint ratio token exists; the 0.08 value
    /// matches the pending-tone fill ratio used by `Callout`.
    pub fn fill_tone_ratio(&self) -> f32 {
        0.08
    }
}
