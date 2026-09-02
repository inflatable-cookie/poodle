use poodle_headless::licence::{LicenceActivationMode, LicenceActivationRoute};

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// Opt-in segmented licence-key entry (contract §4 `keyCodeInput`). Group
/// lengths are a complete positive-integer partition of `length`;
/// `separator` is presentation-only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceKeyCodeInputOptions {
    pub length: usize,
    pub groups: Option<Vec<usize>>,
    pub separator: Option<String>,
}

impl LicenceKeyCodeInputOptions {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            groups: None,
            separator: None,
        }
    }

    pub fn with_groups(mut self, groups: impl IntoIterator<Item = usize>) -> Self {
        self.groups = Some(groups.into_iter().collect());
        self
    }

    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = Some(separator.into());
        self
    }
}

/// LicenceActivation — one host-selected activation model: licence-key entry,
/// or account activation with licence-file fallback.
///
/// Contract: `docs/contracts/components/licence-activation.md`
///
/// The spec is cloneable data. Host callbacks, parsers, and async account
/// work stay in `LicenceActivationHandlers` (render crate) and the pure
/// submit resolution in `poodle_headless::licence`; nothing here emits a
/// credential or runs a journey.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceActivationSpec {
    /// The host-selected activation product model.
    pub mode: LicenceActivationMode,
    /// The current account/offline route (meaningful in account mode).
    pub route: LicenceActivationRoute,
    /// Disables submission while the host command runs.
    pub pending: bool,
    pub is_disabled: bool,
    pub title: String,
    /// Opted-in machine naming and its current draft: `None` hides the
    /// control entirely; `Some("")` is an opt-in empty name (renders
    /// `unnamed machine` copy, which is never emitted as the label).
    pub machine_label: Option<String>,
    /// Whether the machine-name EditableLabel is editing (Rust targets own
    /// the edit state, like every other composed control).
    pub machine_label_editing: bool,
    /// Session-private live draft while the machine name is editing.
    pub machine_label_draft: Option<String>,
    /// Caret into the machine-name draft.
    pub machine_label_selection: (usize, usize),
    /// One-shot: the next view-mode paint of the machine name requests
    /// display focus after Enter/Escape.
    pub machine_label_request_focus: bool,
    /// Submit override; `None` resolves mode/view copy.
    pub activate_label: Option<String>,
    /// Opt-in segmented key entry; `None` renders the free-form TextInput.
    pub key_code_input: Option<LicenceKeyCodeInputOptions>,
    /// The key draft exactly as typed. Never pre-normalized.
    pub key_draft: String,
    /// Caret/selection into the key draft (Rust targets only).
    pub key_selection: (usize, usize),
    /// Local key-format feedback after a rejected submit (typo/too-short/
    /// unreadable copy; never a verdict on the key).
    pub key_message: Option<String>,
    /// Local account/file route feedback (file required, unreadable file,
    /// generic account failure).
    pub route_message: Option<String>,
    /// Narrows accepted offline licence files (the generic FileUpload rule).
    pub file_accept: Option<String>,
    /// Selected licence file's display name. Contents never render.
    pub file_name: Option<String>,
    /// The read payload — bare base64, no data-URL prefix. Never rendered
    /// or logged.
    pub file_contents_base64: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for LicenceActivationSpec {
    fn default() -> Self {
        Self {
            mode: LicenceActivationMode::Account,
            route: LicenceActivationRoute::AccountToken,
            pending: false,
            is_disabled: false,
            title: "Activate licence".to_string(),
            machine_label: None,
            machine_label_editing: false,
            machine_label_draft: None,
            machine_label_selection: (0, 0),
            machine_label_request_focus: false,
            activate_label: None,
            key_code_input: None,
            key_draft: String::new(),
            key_selection: (0, 0),
            key_message: None,
            route_message: None,
            file_accept: None,
            file_name: None,
            file_contents_base64: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl LicenceActivationSpec {
    pub fn new() -> Self {
        Self::default()
    }

    /// The effective route for the current mode: account mode follows the
    /// account/offline route; key mode is always key.
    pub fn effective_route(&self) -> LicenceActivationRoute {
        match self.mode {
            LicenceActivationMode::Key => LicenceActivationRoute::Key,
            LicenceActivationMode::Account => self.route,
        }
    }

    /// The trimmed committed label, or `None` — the value the host emits.
    /// The `unnamed machine` empty-state copy is never emitted.
    pub fn committed_label(&self) -> Option<String> {
        self.machine_label
            .as_deref()
            .and_then(poodle_headless::licence::licence_machine_label)
    }

    /// Whether a route switch control should render (account mode only).
    pub fn shows_route_switch(&self) -> bool {
        self.mode == LicenceActivationMode::Account
    }

    /// Default submit copy: `Continue with account` in the account view,
    /// `Activate` in key/offline views.
    pub fn default_submit_label(&self) -> &'static str {
        match self.effective_route() {
            LicenceActivationRoute::AccountToken => "Continue with account",
            _ => "Activate",
        }
    }

    /// The label the submit button renders, after the override.
    pub fn submit_label(&self) -> &str {
        self.activate_label
            .as_deref()
            .unwrap_or_else(|| self.default_submit_label())
    }

    /// Whether the whole form is frozen (disabled or a command pending).
    pub fn interaction_frozen(&self) -> bool {
        self.is_disabled || self.pending
    }

    pub fn with_mode(mut self, mode: LicenceActivationMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_route(mut self, route: LicenceActivationRoute) -> Self {
        self.route = route;
        self
    }

    pub fn with_pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_machine_label(mut self, machine_label: Option<String>) -> Self {
        self.machine_label = machine_label;
        self
    }

    pub fn with_activate_label(mut self, activate_label: impl Into<String>) -> Self {
        self.activate_label = Some(activate_label.into());
        self
    }

    pub fn with_key_code_input(mut self, options: LicenceKeyCodeInputOptions) -> Self {
        self.key_code_input = Some(options);
        self
    }

    pub fn with_key_draft(mut self, key_draft: impl Into<String>) -> Self {
        self.key_draft = key_draft.into();
        self
    }

    pub fn with_key_selection(mut self, start: usize, end: usize) -> Self {
        self.key_selection = (start, end);
        self
    }

    pub fn with_key_message(mut self, message: Option<String>) -> Self {
        self.key_message = message;
        self
    }

    pub fn with_route_message(mut self, message: Option<String>) -> Self {
        self.route_message = message;
        self
    }

    pub fn with_file_accept(mut self, accept: impl Into<String>) -> Self {
        self.file_accept = Some(accept.into());
        self
    }

    pub fn with_file(
        mut self,
        name: impl Into<String>,
        contents_base64: impl Into<String>,
    ) -> Self {
        self.file_name = Some(name.into());
        self.file_contents_base64 = Some(contents_base64.into());
        self
    }

    pub fn with_file_name(mut self, name: impl Into<String>) -> Self {
        self.file_name = Some(name.into());
        self
    }

    pub fn with_file_contents_base64(mut self, contents_base64: impl Into<String>) -> Self {
        self.file_contents_base64 = Some(contents_base64.into());
        self
    }

    pub fn with_machine_label_editing(mut self, editing: bool) -> Self {
        self.machine_label_editing = editing;
        self
    }

    pub fn with_machine_label_draft(mut self, draft: impl Into<Option<String>>) -> Self {
        self.machine_label_draft = draft.into();
        self
    }

    pub fn with_machine_label_selection(mut self, start: usize, end: usize) -> Self {
        self.machine_label_selection = (start, end);
        self
    }

    pub fn with_machine_label_request_focus(mut self, request_focus: bool) -> Self {
        self.machine_label_request_focus = request_focus;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_route_follows_mode_and_view() {
        let spec = LicenceActivationSpec::new();
        assert_eq!(spec.effective_route(), LicenceActivationRoute::AccountToken);
        assert_eq!(spec.default_submit_label(), "Continue with account");

        let keyed = spec.clone().with_mode(LicenceActivationMode::Key);
        assert_eq!(keyed.effective_route(), LicenceActivationRoute::Key);
        assert_eq!(keyed.default_submit_label(), "Activate");

        let offline = spec.with_route(LicenceActivationRoute::LicenceFile);
        assert_eq!(
            offline.effective_route(),
            LicenceActivationRoute::LicenceFile
        );
        assert_eq!(offline.default_submit_label(), "Activate");
        assert!(offline.shows_route_switch());
    }

    #[test]
    fn activate_label_overrides_the_mode_copy() {
        let spec = LicenceActivationSpec::new()
            .with_mode(LicenceActivationMode::Key)
            .with_activate_label("Redeem");
        assert_eq!(spec.submit_label(), "Redeem");
    }

    #[test]
    fn machine_label_commits_trimmed_or_null() {
        let spec = LicenceActivationSpec::new().with_machine_label(Some("  rig-01  ".to_string()));
        assert_eq!(spec.committed_label().as_deref(), Some("rig-01"));

        let empty = LicenceActivationSpec::new().with_machine_label(Some(String::new()));
        assert_eq!(empty.committed_label(), None);

        let absent = LicenceActivationSpec::new();
        assert_eq!(absent.committed_label(), None);
    }

    #[test]
    fn pending_or_disabled_freezes_interaction() {
        assert!(LicenceActivationSpec::new()
            .with_pending(true)
            .interaction_frozen());
        assert!(LicenceActivationSpec::new()
            .with_disabled(true)
            .interaction_frozen());
        assert!(!LicenceActivationSpec::new().interaction_frozen());
    }
}
