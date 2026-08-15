//! HistoryCenter — the history counterpart to `MessageCenter`: a titlebar
//! trigger cluster (undo / list / redo) plus a popover rendering the flat
//! history list, with node-owned fork disclosure.
//!
//! The portable declaration surface — struct, defaults, builders — is
//! generated from the conformance interface module
//! (`packages/core/src/conformance/history-center.ts`) into
//! [`crate::generated::history_center`] (regenerate with
//! `effigy conformance:build`, gated by `effigy conformance:check`). This
//! module is the hand-written extension beside it: token recipes and derived
//! queries.
//!
//! The component is authority-agnostic. Data arrives through `pages`;
//! commands leave through the renderer's handlers. It validates no protocol
//! rule, assumes no supplied history is complete, and decides nothing about
//! what undo means. There is no Longhorn dependency and none is possible —
//! the dependency runs Longhorn → Poodle.
//!
//! Contract: `docs/contracts/components/history-center.md`

use crate::types::{ControlDensity, ControlSize};

pub use crate::generated::history_center::{
    HistoryCenterRejection, HistoryCenterSpec, HistoryCenterStatus,
};

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
    /// protocol onto the two codes; the component owns the wording, so the
    /// protocol's vocabulary never reaches an operator.
    pub fn rejection_message(&self) -> Option<&'static str> {
        match self.rejection? {
            HistoryCenterRejection::AlreadyAtTarget => Some("Already at the requested target"),
            HistoryCenterRejection::UnknownEntry => Some("Entry does not exist"),
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

    #[test]
    fn rejection_copy_is_component_owned() {
        assert_eq!(HistoryCenterSpec::new().rejection_message(), None);
        assert_eq!(
            HistoryCenterSpec::new()
                .with_rejection(HistoryCenterRejection::UnknownEntry)
                .rejection_message(),
            Some("Entry does not exist"),
        );
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
