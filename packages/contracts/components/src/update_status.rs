use poodle_headless::update::{
    Channel, UpdateAheadOfChannel, UpdateAvailabilityProjection, UpdateControllerStatus,
    UpdateDeferral, UpdateProgressProjection, UpdateRejectionCode,
};

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// UpdateStatus — the update mechanism as data in and commands out.
///
/// Contract: `docs/contracts/components/update-status.md`
///
/// Display copy resolves once through `poodle_headless::update::update_status_view`.
/// The host owns every authority read; this spec never fetches, downloads, or
/// installs. `observe` is web-only: a native host rerenders with fresh props.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStatusSpec {
    pub status: UpdateControllerStatus,
    pub availability: Option<UpdateAvailabilityProjection>,
    pub progress: Option<UpdateProgressProjection>,
    pub channel: Option<Channel>,
    pub installed_version: Option<String>,
    pub deferral: Option<UpdateDeferral>,
    pub last_rejection: Option<UpdateRejectionCode>,
    pub ahead_of_channel: Option<UpdateAheadOfChannel>,
    pub pending: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    pub install_label: String,
    pub defer_label: String,
    pub check_label: String,
    pub retry_label: String,
    pub confirm_install: bool,
    /// Host-owned confirmation dialog. Native has no internal overlay store.
    pub confirm_open: bool,
}

impl Default for UpdateStatusSpec {
    fn default() -> Self {
        Self {
            status: UpdateControllerStatus::Idle,
            availability: None,
            progress: None,
            channel: None,
            installed_version: None,
            deferral: None,
            last_rejection: None,
            ahead_of_channel: None,
            pending: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
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

impl UpdateStatusSpec {
    pub fn new() -> Self {
        Self::default()
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
}
