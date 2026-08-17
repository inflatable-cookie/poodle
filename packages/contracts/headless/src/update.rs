//! Update status — structural mirrors of the update authority's shapes, plus
//! the pure view derivation every renderer consumes.
//!
//! The Rust mirror of `packages/core/src/update.ts`. Poodle renders; the host
//! supplies behaviour. No Longhorn import: the record types below are
//! structural mirrors so a bridge can assert them against the authority's
//! generated field maps.
//!
//! `update_status_view` is the single display-copy authority. The five wrong
//! messages a naive rendering tells — a deferral styled as a failure, a null
//! download fraction drawn as zero, `aheadOfChannel` read as broken, a
//! managed-elsewhere install read as an error, and a signature rejection
//! offered a retry — are all decided here, once, so the native surface cannot
//! disagree with the web pair.

// ── Structural mirrors of the authority's shapes ─────────────────────────

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Channel {
    Production,
    Beta,
    Nightly,
}

impl Channel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Production => "production",
            Self::Beta => "beta",
            Self::Nightly => "nightly",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OfferReason {
    Staged,
    BelowMinimumVersion,
    UserInitiated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstallManager {
    MacAppStore,
    HomebrewCask,
    Flatpak,
    Snap,
    AppImage,
    Nix,
    LinuxDistribution,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpdateAvailabilityProjection {
    Offer {
        version: String,
        reason: OfferReason,
        notes: Option<String>,
    },
    UpToDate,
    AheadOfChannel {
        installed: String,
        channel: String,
    },
    WithheldByRollout {
        version: String,
    },
    ManagedElsewhere {
        version: String,
        manager: InstallManager,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum UpdateProgressProjection {
    Idle,
    Downloading { fraction: Option<f64> },
    Verifying,
    ReadyToInstall { version: String },
    Installing { version: String },
}

impl UpdateProgressProjection {
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeferralCause {
    UserPostponed,
    WorkInFlight { detail: String },
    InstallationNotWritable { detail: String },
    ExternallyManaged {
        manager: InstallManager,
        command: Option<String>,
    },
    InstallFailed { detail: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdateRejectionCode {
    StaleAuthority,
    NoOffer,
    Unavailable,
    ChannelMismatch,
    Unreachable,
    SignatureRejected,
    NotWritable,
    InstallFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdatePresence {
    Hidden,
    Quiet,
    Attention,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpdateControllerStatus {
    Idle,
    Loading,
    Ready,
    Failed { error: Option<String> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateDeferral {
    pub version: String,
    pub cause: DeferralCause,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateAheadOfChannel {
    pub installed: String,
    pub channel: String,
}

// ── Display copy ─────────────────────────────────────────────────────────

pub fn install_manager_label(manager: InstallManager) -> &'static str {
    match manager {
        InstallManager::MacAppStore => "Mac App Store",
        InstallManager::HomebrewCask => "Homebrew",
        InstallManager::Flatpak => "Flatpak",
        InstallManager::Snap => "Snap",
        InstallManager::AppImage => "AppImage",
        InstallManager::Nix => "Nix",
        InstallManager::LinuxDistribution => "your Linux distribution",
    }
}

pub fn update_rejection_message(code: UpdateRejectionCode) -> &'static str {
    match code {
        UpdateRejectionCode::StaleAuthority => "The update service returned a stale result.",
        UpdateRejectionCode::NoOffer => "No update is available right now.",
        UpdateRejectionCode::Unavailable => "The update service is unavailable.",
        UpdateRejectionCode::ChannelMismatch => {
            "The update feed answered for a different channel."
        }
        UpdateRejectionCode::Unreachable => "The update service could not be reached.",
        UpdateRejectionCode::SignatureRejected => {
            "The update failed its signature check and was not installed."
        }
        UpdateRejectionCode::NotWritable => "The update could not be written to disk.",
        UpdateRejectionCode::InstallFailed => "The update could not be installed.",
    }
}

/// The retry a rejection may offer. A signature rejection offers none.
pub fn update_rejection_retry(code: UpdateRejectionCode) -> Option<UpdateStatusAction> {
    match code {
        UpdateRejectionCode::SignatureRejected => None,
        UpdateRejectionCode::NotWritable | UpdateRejectionCode::InstallFailed => {
            Some(UpdateStatusAction::Install)
        }
        _ => Some(UpdateStatusAction::Check),
    }
}

/// Accessible name for the update-centre trigger while a download runs.
pub fn update_download_label(fraction: Option<f64>) -> String {
    match fraction {
        None => "Downloading update".to_string(),
        Some(value) => format!("Downloading update, {}%", (value * 100.0).round() as i64),
    }
}

pub fn update_error_message(error: Option<&str>) -> String {
    match error {
        Some(message) if !message.is_empty() => message.to_string(),
        _ => "Something went wrong.".to_string(),
    }
}

// ── View model ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdateStatusAction {
    Check,
    Install,
    Defer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdateStatusNoticeTone {
    Neutral,
    Danger,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateStatusNotice {
    pub tone: UpdateStatusNoticeTone,
    pub message: String,
    pub retry: Option<UpdateStatusAction>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdateStatusTone {
    Neutral,
    Info,
    Attention,
    Danger,
}

impl UpdateStatusTone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Info => "info",
            Self::Attention => "attention",
            Self::Danger => "danger",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpdateStatusViewState {
    Offer,
    UpToDate,
    AheadOfChannel,
    WithheldByRollout,
    ManagedElsewhere,
    Downloading,
    Verifying,
    ReadyToInstall,
    Installing,
    Checking,
    Failed,
    Rejected,
    Idle,
}

impl UpdateStatusViewState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offer => "offer",
            Self::UpToDate => "upToDate",
            Self::AheadOfChannel => "aheadOfChannel",
            Self::WithheldByRollout => "withheldByRollout",
            Self::ManagedElsewhere => "managedElsewhere",
            Self::Downloading => "downloading",
            Self::Verifying => "verifying",
            Self::ReadyToInstall => "readyToInstall",
            Self::Installing => "installing",
            Self::Checking => "checking",
            Self::Failed => "failed",
            Self::Rejected => "rejected",
            Self::Idle => "idle",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStatusView {
    pub state: UpdateStatusViewState,
    pub tone: UpdateStatusTone,
    pub title: String,
    pub body: Option<String>,
    /// A download bar descriptor. `fraction: None` is indeterminate, not zero.
    pub progress: Option<UpdateStatusProgress>,
    pub notice: Option<UpdateStatusNotice>,
    pub busy: bool,
    pub actions: Vec<UpdateStatusAction>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateStatusProgress {
    pub fraction: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStatusInput {
    pub status: UpdateControllerStatus,
    pub availability: Option<UpdateAvailabilityProjection>,
    pub progress: Option<UpdateProgressProjection>,
    pub deferral: Option<UpdateDeferral>,
    pub last_rejection: Option<UpdateRejectionCode>,
    pub ahead_of_channel: Option<UpdateAheadOfChannel>,
    pub channel: Option<Channel>,
    pub installed_version: Option<String>,
}

pub fn update_status_view(input: UpdateStatusInput) -> UpdateStatusView {
    if let Some(progress) = input.progress.as_ref().filter(|p| !p.is_idle()) {
        return progress_view(progress);
    }

    if matches!(input.status, UpdateControllerStatus::Loading) {
        return UpdateStatusView {
            state: UpdateStatusViewState::Checking,
            tone: UpdateStatusTone::Neutral,
            title: "Checking for updates…".to_string(),
            body: None,
            progress: None,
            notice: None,
            busy: true,
            actions: Vec::new(),
        };
    }

    if let UpdateControllerStatus::Failed { error } = &input.status {
        return UpdateStatusView {
            state: UpdateStatusViewState::Failed,
            tone: UpdateStatusTone::Danger,
            title: "Update check failed".to_string(),
            body: Some(update_error_message(error.as_deref())),
            progress: None,
            notice: None,
            busy: false,
            actions: vec![UpdateStatusAction::Check],
        };
    }

    if let Some(availability) = &input.availability {
        return availability_view(availability, &input);
    }

    if let Some(code) = input.last_rejection {
        return rejection_view(code);
    }

    UpdateStatusView {
        state: UpdateStatusViewState::Idle,
        tone: UpdateStatusTone::Neutral,
        title: "Updates".to_string(),
        body: None,
        progress: None,
        notice: None,
        busy: false,
        actions: vec![UpdateStatusAction::Check],
    }
}

fn progress_view(progress: &UpdateProgressProjection) -> UpdateStatusView {
    match progress {
        UpdateProgressProjection::Downloading { fraction } => UpdateStatusView {
            state: UpdateStatusViewState::Downloading,
            tone: UpdateStatusTone::Info,
            title: "Downloading update…".to_string(),
            body: None,
            progress: Some(UpdateStatusProgress {
                fraction: *fraction,
            }),
            notice: None,
            busy: false,
            actions: Vec::new(),
        },
        UpdateProgressProjection::Verifying => UpdateStatusView {
            state: UpdateStatusViewState::Verifying,
            tone: UpdateStatusTone::Info,
            title: "Verifying update…".to_string(),
            body: None,
            progress: None,
            notice: None,
            busy: true,
            actions: Vec::new(),
        },
        UpdateProgressProjection::ReadyToInstall { version } => UpdateStatusView {
            state: UpdateStatusViewState::ReadyToInstall,
            tone: UpdateStatusTone::Attention,
            title: format!("Version {version} is ready to install"),
            body: None,
            progress: None,
            notice: None,
            busy: false,
            actions: vec![UpdateStatusAction::Install],
        },
        UpdateProgressProjection::Installing { version } => UpdateStatusView {
            state: UpdateStatusViewState::Installing,
            tone: UpdateStatusTone::Info,
            title: format!("Installing {version}…"),
            body: None,
            progress: None,
            notice: None,
            busy: true,
            actions: Vec::new(),
        },
        UpdateProgressProjection::Idle => UpdateStatusView {
            state: UpdateStatusViewState::Idle,
            tone: UpdateStatusTone::Neutral,
            title: "Updates".to_string(),
            body: None,
            progress: None,
            notice: None,
            busy: false,
            actions: Vec::new(),
        },
    }
}

fn availability_view(
    availability: &UpdateAvailabilityProjection,
    input: &UpdateStatusInput,
) -> UpdateStatusView {
    let notice = notice_for(input);

    match availability {
        UpdateAvailabilityProjection::Offer {
            version, notes, ..
        } => UpdateStatusView {
            state: UpdateStatusViewState::Offer,
            tone: UpdateStatusTone::Attention,
            title: format!("Version {version} is available"),
            body: notes.clone(),
            progress: None,
            notice,
            busy: false,
            actions: vec![UpdateStatusAction::Install, UpdateStatusAction::Defer],
        },
        UpdateAvailabilityProjection::UpToDate => UpdateStatusView {
            state: UpdateStatusViewState::UpToDate,
            tone: UpdateStatusTone::Neutral,
            title: "You're up to date".to_string(),
            body: up_to_date_body(input),
            progress: None,
            notice,
            busy: false,
            actions: Vec::new(),
        },
        UpdateAvailabilityProjection::AheadOfChannel {
            installed,
            channel,
        } => {
            let ahead = input.ahead_of_channel.clone().unwrap_or(UpdateAheadOfChannel {
                installed: installed.clone(),
                channel: channel.clone(),
            });
            UpdateStatusView {
                state: UpdateStatusViewState::AheadOfChannel,
                tone: UpdateStatusTone::Neutral,
                title: "You're ahead of your channel".to_string(),
                body: Some(format!(
                    "Installed {} · channel {}",
                    ahead.installed, ahead.channel
                )),
                progress: None,
                notice,
                busy: false,
                actions: Vec::new(),
            }
        }
        UpdateAvailabilityProjection::WithheldByRollout { version } => UpdateStatusView {
            state: UpdateStatusViewState::WithheldByRollout,
            tone: UpdateStatusTone::Neutral,
            title: format!("Version {version} exists"),
            body: Some("Not staged to you yet.".to_string()),
            progress: None,
            notice,
            busy: false,
            actions: Vec::new(),
        },
        UpdateAvailabilityProjection::ManagedElsewhere { version, manager } => UpdateStatusView {
            state: UpdateStatusViewState::ManagedElsewhere,
            tone: UpdateStatusTone::Info,
            title: format!("Version {version} is available"),
            body: Some(format!(
                "Managed by {}.",
                install_manager_label(*manager)
            )),
            progress: None,
            notice,
            busy: false,
            actions: Vec::new(),
        },
    }
}

fn up_to_date_body(input: &UpdateStatusInput) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(installed) = &input.installed_version {
        parts.push(format!("Version {installed}"));
    }
    if let Some(channel) = input.channel {
        parts.push(format!("{} channel", channel.as_str()));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
    }
}

fn rejection_view(code: UpdateRejectionCode) -> UpdateStatusView {
    UpdateStatusView {
        state: UpdateStatusViewState::Rejected,
        tone: UpdateStatusTone::Danger,
        title: "Update unavailable".to_string(),
        body: None,
        progress: None,
        notice: Some(UpdateStatusNotice {
            tone: UpdateStatusNoticeTone::Danger,
            message: update_rejection_message(code).to_string(),
            retry: update_rejection_retry(code),
        }),
        busy: false,
        actions: Vec::new(),
    }
}

fn notice_for(input: &UpdateStatusInput) -> Option<UpdateStatusNotice> {
    if let Some(code) = input.last_rejection {
        return Some(UpdateStatusNotice {
            tone: UpdateStatusNoticeTone::Danger,
            message: update_rejection_message(code).to_string(),
            retry: update_rejection_retry(code),
        });
    }
    input.deferral.as_ref().map(|deferral| deferral_notice(&deferral.cause))
}

fn deferral_notice(cause: &DeferralCause) -> UpdateStatusNotice {
    match cause {
        DeferralCause::UserPostponed => UpdateStatusNotice {
            tone: UpdateStatusNoticeTone::Neutral,
            message: "Update postponed.".to_string(),
            retry: None,
        },
        DeferralCause::WorkInFlight { detail } => UpdateStatusNotice {
            tone: UpdateStatusNoticeTone::Neutral,
            message: format!("Install is on hold: {detail}"),
            retry: Some(UpdateStatusAction::Install),
        },
        DeferralCause::InstallationNotWritable { detail } => UpdateStatusNotice {
            tone: UpdateStatusNoticeTone::Neutral,
            message: format!("Install location is not writable: {detail}"),
            retry: None,
        },
        DeferralCause::ExternallyManaged { manager, command } => {
            let label = install_manager_label(*manager);
            let message = match command.as_deref().filter(|c| !c.is_empty()) {
                Some(command) => format!("{label} manages this install. Upgrade with: {command}"),
                None => format!("{label} manages this install."),
            };
            UpdateStatusNotice {
                tone: UpdateStatusNoticeTone::Neutral,
                message,
                retry: None,
            }
        }
        DeferralCause::InstallFailed { detail } => UpdateStatusNotice {
            tone: UpdateStatusNoticeTone::Neutral,
            message: format!("Install could not be applied: {detail}"),
            retry: Some(UpdateStatusAction::Install),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ready_input() -> UpdateStatusInput {
        UpdateStatusInput {
            status: UpdateControllerStatus::Ready,
            availability: None,
            progress: None,
            deferral: None,
            last_rejection: None,
            ahead_of_channel: None,
            channel: None,
            installed_version: None,
        }
    }

    fn offer() -> UpdateAvailabilityProjection {
        UpdateAvailabilityProjection::Offer {
            version: "1.4.0".to_string(),
            reason: OfferReason::Staged,
            notes: None,
        }
    }

    #[test]
    fn offer_carries_version_notes_and_both_actions() {
        let mut input = ready_input();
        input.availability = Some(UpdateAvailabilityProjection::Offer {
            version: "1.4.0".to_string(),
            reason: OfferReason::Staged,
            notes: Some("Bug fixes and improvements.".to_string()),
        });
        let view = update_status_view(input);
        assert_eq!(view.state, UpdateStatusViewState::Offer);
        assert_eq!(view.title, "Version 1.4.0 is available");
        assert_eq!(view.body.as_deref(), Some("Bug fixes and improvements."));
        assert_eq!(
            view.actions,
            vec![UpdateStatusAction::Install, UpdateStatusAction::Defer]
        );
    }

    #[test]
    fn up_to_date_is_calm_and_actionless() {
        let mut input = ready_input();
        input.availability = Some(UpdateAvailabilityProjection::UpToDate);
        input.installed_version = Some("1.3.0".to_string());
        input.channel = Some(Channel::Production);
        let view = update_status_view(input);
        assert_eq!(view.state, UpdateStatusViewState::UpToDate);
        assert_eq!(view.title, "You're up to date");
        assert_eq!(
            view.body.as_deref(),
            Some("Version 1.3.0 · production channel")
        );
        assert!(view.actions.is_empty());
    }

    #[test]
    fn ahead_of_channel_is_not_up_to_date() {
        let mut input = ready_input();
        input.availability = Some(UpdateAvailabilityProjection::AheadOfChannel {
            installed: "1.3.0-nightly.4".to_string(),
            channel: "1.2.9".to_string(),
        });
        input.ahead_of_channel = Some(UpdateAheadOfChannel {
            installed: "1.3.0-nightly.4".to_string(),
            channel: "1.2.9".to_string(),
        });
        let view = update_status_view(input);
        assert_eq!(view.state, UpdateStatusViewState::AheadOfChannel);
        assert_eq!(view.title, "You're ahead of your channel");
        assert_eq!(
            view.body.as_deref(),
            Some("Installed 1.3.0-nightly.4 · channel 1.2.9")
        );
        assert_ne!(view.title, "You're up to date");
    }

    #[test]
    fn withheld_by_rollout_is_information() {
        let mut input = ready_input();
        input.availability = Some(UpdateAvailabilityProjection::WithheldByRollout {
            version: "2.0.0".to_string(),
        });
        let view = update_status_view(input);
        assert_eq!(view.title, "Version 2.0.0 exists");
        assert_eq!(view.body.as_deref(), Some("Not staged to you yet."));
        assert!(view.actions.is_empty());
    }

    #[test]
    fn managed_elsewhere_is_not_an_error() {
        let mut input = ready_input();
        input.availability = Some(UpdateAvailabilityProjection::ManagedElsewhere {
            version: "1.4.0".to_string(),
            manager: InstallManager::HomebrewCask,
        });
        let view = update_status_view(input);
        assert_eq!(view.tone, UpdateStatusTone::Info);
        assert_eq!(view.title, "Version 1.4.0 is available");
        assert_eq!(view.body.as_deref(), Some("Managed by Homebrew."));
        assert!(view.actions.is_empty());
    }

    #[test]
    fn a_deferral_is_not_a_failure() {
        let mut input = ready_input();
        input.availability = Some(offer());
        input.deferral = Some(UpdateDeferral {
            version: "1.4.0".to_string(),
            cause: DeferralCause::WorkInFlight {
                detail: "A transfer is running.".to_string(),
            },
        });
        let view = update_status_view(input);
        let notice = view.notice.expect("deferral notice");
        assert_eq!(notice.tone, UpdateStatusNoticeTone::Neutral);
        assert_eq!(
            notice.message,
            "Install is on hold: A transfer is running."
        );
        assert_eq!(notice.retry, Some(UpdateStatusAction::Install));
        assert_eq!(view.tone, UpdateStatusTone::Attention);
    }

    #[test]
    fn null_fraction_is_indeterminate_and_zero_is_zero() {
        let mut null_input = ready_input();
        null_input.progress = Some(UpdateProgressProjection::Downloading { fraction: None });
        let null_view = update_status_view(null_input);
        assert_eq!(
            null_view.progress,
            Some(UpdateStatusProgress { fraction: None })
        );

        let mut zero_input = ready_input();
        zero_input.progress = Some(UpdateProgressProjection::Downloading {
            fraction: Some(0.0),
        });
        let zero_view = update_status_view(zero_input);
        assert_eq!(
            zero_view.progress,
            Some(UpdateStatusProgress {
                fraction: Some(0.0)
            })
        );
    }

    #[test]
    fn signature_rejection_offers_no_retry() {
        let mut input = ready_input();
        input.last_rejection = Some(UpdateRejectionCode::SignatureRejected);
        let view = update_status_view(input);
        let notice = view.notice.expect("rejection notice");
        assert_eq!(notice.tone, UpdateStatusNoticeTone::Danger);
        assert!(notice.retry.is_none());
        assert!(!notice.message.to_lowercase().contains("reach"));
        assert!(!notice.message.to_lowercase().contains("network"));
    }

    #[test]
    fn progress_supersedes_availability() {
        let mut input = ready_input();
        input.availability = Some(offer());
        input.progress = Some(UpdateProgressProjection::Verifying);
        let view = update_status_view(input);
        assert_eq!(view.state, UpdateStatusViewState::Verifying);
        assert!(view.busy);
        assert!(view.actions.is_empty());
    }

    #[test]
    fn download_label_never_says_zero_for_a_null_fraction() {
        assert_eq!(update_download_label(None), "Downloading update");
        assert_eq!(update_download_label(Some(0.42)), "Downloading update, 42%");
    }
}
