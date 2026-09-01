//! HistoryCenter — the history counterpart to `MessageCenter`: a titlebar
//! trigger cluster (undo / list / redo) plus a popover rendering the flat
//! history list, with node-owned fork disclosure.
//!
//! This module is the single authority for the HistoryCenter declaration
//! surface: the struct, its defaults and builders, then the token recipes and
//! derived queries beside them. `g14.007` briefly generated the first half
//! from a TypeScript interface; `g14.008` rejected that path and `g14.021`
//! restored the hand-written declaration.
//!
//! The component is authority-agnostic. Data arrives through `pages`;
//! commands leave through the renderer's handlers. It validates no protocol
//! rule, assumes no supplied history is complete, and decides nothing about
//! what undo means. There is no Longhorn dependency and none is possible —
//! the dependency runs Longhorn → Poodle.
//!
//! Contract: `docs/contracts/components/history-center.md`

use crate::types::{ControlDensity, ControlSize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum HistoryCenterStatus {
    #[default] Idle,
    Loading,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum HistoryCenterRejection {
    #[default] AlreadyAtTarget,
    UnknownEntry,
    StaleHistory,
    ProtectedEntry,
    DeletionUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoryCenterSpec {
    pub pages: Option<Vec<crate::types::HistoryPathPage>>,
    pub can_undo: bool,
    pub can_redo: bool,
    pub is_busy: bool,
    pub status: HistoryCenterStatus,
    pub status_message: Option<String>,
    pub rejection: Option<HistoryCenterRejection>,
    pub open: Option<bool>,
    pub default_open: bool,
    pub placement: crate::types::OverlayPlacement,
    pub undo_label: String,
    pub redo_label: String,
    pub list_label: String,
    pub title: String,
    pub empty_message: String,
    pub aria_label: Option<String>,
    pub max_branch_name_bytes: usize,
    pub size: Option<crate::types::ControlSize>,
    pub size_role: crate::types::SemanticControlSizeRole,
    pub density: Option<crate::types::ControlDensity>,
}

impl Default for HistoryCenterSpec {
    fn default() -> Self {
        Self {
            pages: None,
            can_undo: false,
            can_redo: false,
            is_busy: false,
            status: HistoryCenterStatus::Idle,
            status_message: None,
            rejection: None,
            open: None,
            default_open: false,
            placement: crate::types::OverlayPlacement::BottomEnd,
            undo_label: "Undo".to_owned(),
            redo_label: "Redo".to_owned(),
            list_label: "History".to_owned(),
            title: "History".to_owned(),
            empty_message: "No history entries yet.".to_owned(),
            aria_label: None,
            max_branch_name_bytes: 256,
            size: None,
            size_role: crate::types::SemanticControlSizeRole::Chrome,
            density: None,
        }
    }
}

impl HistoryCenterSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pages(mut self, value: Vec<crate::types::HistoryPathPage>) -> Self {
        self.pages = Some(value);
        self
    }
    pub fn with_can_undo(mut self, value: bool) -> Self {
        self.can_undo = value;
        self
    }
    pub fn with_can_redo(mut self, value: bool) -> Self {
        self.can_redo = value;
        self
    }
    pub fn with_busy(mut self, value: bool) -> Self {
        self.is_busy = value;
        self
    }
    pub fn with_status(mut self, value: HistoryCenterStatus) -> Self {
        self.status = value;
        self
    }
    pub fn with_status_message(mut self, value: impl Into<String>) -> Self {
        self.status_message = Some(value.into());
        self
    }
    pub fn with_rejection(mut self, value: HistoryCenterRejection) -> Self {
        self.rejection = Some(value);
        self
    }
    pub fn with_open(mut self, value: bool) -> Self {
        self.open = Some(value);
        self
    }
    pub fn with_default_open(mut self, value: bool) -> Self {
        self.default_open = value;
        self
    }
    pub fn with_placement(mut self, value: crate::types::OverlayPlacement) -> Self {
        self.placement = value;
        self
    }
    pub fn with_undo_label(mut self, value: impl Into<String>) -> Self {
        self.undo_label = value.into();
        self
    }
    pub fn with_redo_label(mut self, value: impl Into<String>) -> Self {
        self.redo_label = value.into();
        self
    }
    pub fn with_list_label(mut self, value: impl Into<String>) -> Self {
        self.list_label = value.into();
        self
    }
    pub fn with_title(mut self, value: impl Into<String>) -> Self {
        self.title = value.into();
        self
    }
    pub fn with_empty_message(mut self, value: impl Into<String>) -> Self {
        self.empty_message = value.into();
        self
    }
    pub fn with_aria_label(mut self, value: impl Into<String>) -> Self {
        self.aria_label = Some(value.into());
        self
    }
    pub fn with_max_branch_name_bytes(mut self, value: usize) -> Self {
        self.max_branch_name_bytes = value;
        self
    }
    pub fn with_size(mut self, value: crate::types::ControlSize) -> Self {
        self.size = Some(value);
        self
    }
    pub fn with_size_role(mut self, value: crate::types::SemanticControlSizeRole) -> Self {
        self.size_role = value;
        self
    }
    pub fn with_density(mut self, value: crate::types::ControlDensity) -> Self {
        self.density = Some(value);
        self
    }
}


impl HistoryCenterSpec {
    /// The resolved control size: the explicit size when set, else the
    /// inherited presentation scale stepped by the semantic role.
    pub fn resolved_size(&self, inherited: ControlSize) -> ControlSize {
        match self.size {
            Some(size) => size,
            None => crate::types::resolve_semantic_control_size(inherited, self.size_role),
        }
    }

    pub fn resolved_density(&self, inherited: ControlDensity) -> ControlDensity {
        self.density.unwrap_or(inherited)
    }

    /// The surface's accessible name: the override when supplied, else the
    /// heading. A surface without either would announce as an unnamed dialog.
    pub fn surface_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.title)
    }

    /// Whether the undo trigger is inert: the host says undo is unavailable,
    /// or an authority operation is already running.
    pub fn undo_is_disabled(&self) -> bool {
        !self.can_undo || self.is_busy
    }

    pub fn redo_is_disabled(&self) -> bool {
        !self.can_redo || self.is_busy
    }

    /// The display copy for a mapped rejection. The host's bridge maps its
    /// protocol onto the five codes; the component owns the wording, so the
    /// protocol's vocabulary never reaches an operator. The three deletion
    /// refusals stay distinct from `UnknownEntry`: a stale, protected, or
    /// unavailable deletion is not a missing entry.
    pub fn rejection_message(&self) -> Option<&'static str> {
        match self.rejection? {
            HistoryCenterRejection::AlreadyAtTarget => Some("Already at the requested target"),
            HistoryCenterRejection::UnknownEntry => Some("Entry does not exist"),
            HistoryCenterRejection::StaleHistory => {
                Some("History changed; this entry was not deleted")
            }
            HistoryCenterRejection::ProtectedEntry => Some("This history entry is protected"),
            HistoryCenterRejection::DeletionUnavailable => Some("History deletion is unavailable"),
        }
    }

    /// The status row's copy: the host's message when the source failed, else
    /// the component's own loading line.
    pub fn status_line(&self) -> Option<&str> {
        match self.status {
            HistoryCenterStatus::Idle => None,
            HistoryCenterStatus::Loading => {
                Some(self.status_message.as_deref().unwrap_or("Loading history…"))
            }
            HistoryCenterStatus::Failed => Some(
                self.status_message
                    .as_deref()
                    .unwrap_or("History unavailable"),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triggers_are_inert_without_availability_or_while_busy() {
        let spec = HistoryCenterSpec::new();
        assert!(spec.undo_is_disabled());
        assert!(spec.redo_is_disabled());

        let ready = HistoryCenterSpec::new().with_can_undo(true).with_can_redo(true);
        assert!(!ready.undo_is_disabled());
        assert!(!ready.redo_is_disabled());

        let busy = ready.clone().with_busy(true);
        assert!(busy.undo_is_disabled());
        assert!(busy.redo_is_disabled());
    }

    #[test]
    fn the_surface_falls_back_to_its_heading_for_a_name() {
        let spec = HistoryCenterSpec::new();
        assert_eq!(spec.surface_label(), "History");
        assert_eq!(
            spec.with_aria_label("Project history").surface_label(),
            "Project history",
        );
    }

    /// The exact contract table (`docs/contracts/components/history-center.md`
    /// §"Rejection handling"). Both rejection proofs below read this list, so
    /// dropping a category or pointing two codes at one message fails here.
    const REJECTION_COPY: [(HistoryCenterRejection, &str); 5] = [
        (
            HistoryCenterRejection::AlreadyAtTarget,
            "Already at the requested target",
        ),
        (HistoryCenterRejection::UnknownEntry, "Entry does not exist"),
        (
            HistoryCenterRejection::StaleHistory,
            "History changed; this entry was not deleted",
        ),
        (
            HistoryCenterRejection::ProtectedEntry,
            "This history entry is protected",
        ),
        (
            HistoryCenterRejection::DeletionUnavailable,
            "History deletion is unavailable",
        ),
    ];

    #[test]
    fn rejection_copy_is_component_owned() {
        assert_eq!(HistoryCenterSpec::new().rejection_message(), None);
        for (code, message) in REJECTION_COPY {
            assert_eq!(
                HistoryCenterSpec::new()
                    .with_rejection(code)
                    .rejection_message(),
                Some(message),
                "{code:?} must carry its own component-owned copy",
            );
        }
    }

    /// The papercut this replaced: a stale revision, a protected entry, and an
    /// unavailable deletion all told the operator "Entry does not exist".
    #[test]
    fn every_refusal_meaning_stays_distinct() {
        let mut messages: Vec<&str> = REJECTION_COPY.iter().map(|(_, m)| *m).collect();
        let distinct = messages.len();
        messages.sort_unstable();
        messages.dedup();
        assert_eq!(messages.len(), distinct, "no two codes may share copy");

        let unknown = HistoryCenterSpec::new()
            .with_rejection(HistoryCenterRejection::UnknownEntry)
            .rejection_message();
        for code in [
            HistoryCenterRejection::StaleHistory,
            HistoryCenterRejection::ProtectedEntry,
            HistoryCenterRejection::DeletionUnavailable,
        ] {
            assert_ne!(
                HistoryCenterSpec::new().with_rejection(code).rejection_message(),
                unknown,
                "{code:?} is a refused deletion, not a missing entry",
            );
        }
    }

    #[test]
    fn a_failed_source_prefers_the_hosts_message() {
        let spec = HistoryCenterSpec::new().with_status(HistoryCenterStatus::Failed);
        assert_eq!(spec.status_line(), Some("History unavailable"));
        assert_eq!(
            spec.with_status_message("Source offline").status_line(),
            Some("Source offline"),
        );
    }

    #[test]
    fn chrome_steps_the_inherited_scale_down() {
        let spec = HistoryCenterSpec::new();
        assert_eq!(spec.resolved_size(ControlSize::Md), ControlSize::Sm);
        assert_eq!(
            spec.with_size(ControlSize::Lg).resolved_size(ControlSize::Md),
            ControlSize::Lg,
        );
    }
}
