use poodle_headless::update::{
    Channel, UpdateAheadOfChannel, UpdateAvailabilityProjection, UpdateControllerStatus,
    UpdateDeferral, UpdatePresence, UpdateProgressProjection, UpdateRejectionCode,
};

use crate::types::{ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole};
use crate::update_status::UpdateStatusSpec;

/// UpdateCenter — a titlebar trigger whose popover hosts [`UpdateStatusSpec`].
///
/// Contract: `docs/contracts/components/update-center.md`
///
/// Presence is an authority read, never derived here. `hidden` collapses the
/// tree; `observe` is web-only (a native host rerenders with fresh props).
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateCenterSpec {
    pub presence: UpdatePresence,
    pub status: UpdateControllerStatus,
    pub availability: Option<UpdateAvailabilityProjection>,
    pub progress: Option<UpdateProgressProjection>,
    pub channel: Option<Channel>,
    pub installed_version: Option<String>,
    pub deferral: Option<UpdateDeferral>,
    pub last_rejection: Option<UpdateRejectionCode>,
    pub ahead_of_channel: Option<UpdateAheadOfChannel>,
    pub pending: bool,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placement: OverlayPlacement,
    pub title: String,
    pub aria_label: Option<String>,
    pub trigger_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    pub install_label: String,
    pub defer_label: String,
    pub check_label: String,
    pub retry_label: String,
    pub confirm_install: bool,
    pub confirm_open: bool,
}

impl Default for UpdateCenterSpec {
    fn default() -> Self {
        Self {
            presence: UpdatePresence::Hidden,
            status: UpdateControllerStatus::Idle,
            availability: None,
            progress: None,
            channel: None,
            installed_version: None,
            deferral: None,
            last_rejection: None,
            ahead_of_channel: None,
            pending: false,
            open: None,
            default_open: false,
            placement: OverlayPlacement::BottomEnd,
            title: "Updates".to_string(),
            aria_label: None,
            trigger_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
            install_label: "Install and restart".to_string(),
            defer_label: "Later".to_string(),
            check_label: "Check for updates".to_string(),
            retry_label: "Try again".to_string(),
            confirm_install: true,
            confirm_open: false,
        }
    }
}

impl UpdateCenterSpec {
    pub fn new(presence: UpdatePresence) -> Self {
        Self {
            presence,
            ..Self::default()
        }
    }

    pub fn with_presence(mut self, presence: UpdatePresence) -> Self {
        self.presence = presence;
        self
    }

    pub fn with_status(mut self, status: UpdateControllerStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_availability(mut self, availability: UpdateAvailabilityProjection) -> Self {
        self.availability = Some(availability);
        self
    }

    pub fn with_progress(mut self, progress: UpdateProgressProjection) -> Self {
        self.progress = Some(progress);
        self
    }

    pub fn with_channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn with_installed_version(mut self, version: impl Into<String>) -> Self {
        self.installed_version = Some(version.into());
        self
    }

    pub fn with_deferral(mut self, deferral: UpdateDeferral) -> Self {
        self.deferral = Some(deferral);
        self
    }

    pub fn with_last_rejection(mut self, code: UpdateRejectionCode) -> Self {
        self.last_rejection = Some(code);
        self
    }

    pub fn with_ahead_of_channel(mut self, ahead: UpdateAheadOfChannel) -> Self {
        self.ahead_of_channel = Some(ahead);
        self
    }

    pub fn with_pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_trigger_label(mut self, label: impl Into<String>) -> Self {
        self.trigger_label = Some(label.into());
        self
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

    pub fn with_install_label(mut self, label: impl Into<String>) -> Self {
        self.install_label = label.into();
        self
    }

    pub fn with_defer_label(mut self, label: impl Into<String>) -> Self {
        self.defer_label = label.into();
        self
    }

    pub fn with_check_label(mut self, label: impl Into<String>) -> Self {
        self.check_label = label.into();
        self
    }

    pub fn with_retry_label(mut self, label: impl Into<String>) -> Self {
        self.retry_label = label.into();
        self
    }

    pub fn with_confirm_install(mut self, confirm_install: bool) -> Self {
        self.confirm_install = confirm_install;
        self
    }

    pub fn with_confirm_open(mut self, confirm_open: bool) -> Self {
        self.confirm_open = confirm_open;
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.title)
    }

    pub fn effective_trigger_label(&self) -> &str {
        self.trigger_label.as_deref().unwrap_or(&self.title)
    }

    /// The inner status surface. Size role stays `control` — the centre's
    /// `chrome` role is for the titlebar trigger only.
    pub fn status_spec(&self) -> UpdateStatusSpec {
        UpdateStatusSpec {
            status: self.status.clone(),
            availability: self.availability.clone(),
            progress: self.progress.clone(),
            channel: self.channel,
            installed_version: self.installed_version.clone(),
            deferral: self.deferral.clone(),
            last_rejection: self.last_rejection,
            ahead_of_channel: self.ahead_of_channel.clone(),
            pending: self.pending,
            size: self.size,
            size_role: SemanticControlSizeRole::Control,
            density: self.density,
            install_label: self.install_label.clone(),
            defer_label: self.defer_label.clone(),
            check_label: self.check_label.clone(),
            retry_label: self.retry_label.clone(),
            confirm_install: self.confirm_install,
            confirm_open: self.confirm_open,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hidden_presence_is_the_default_and_open_seeds_from_default_open() {
        let spec = UpdateCenterSpec::new(UpdatePresence::Quiet).with_default_open(true);
        assert_eq!(spec.presence, UpdatePresence::Quiet);
        assert!(spec.current_open());
        assert_eq!(spec.effective_trigger_label(), "Updates");
    }
}
