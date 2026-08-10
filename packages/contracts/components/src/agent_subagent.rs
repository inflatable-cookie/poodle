//! AgentSubagentSpec — an inline group for a provider-owned child agent's work
//! in the transcript.
//!
//! Contract: `docs/contracts/components/agent-subagent.md`.
//!
//! On the natives this renders identity + status, a one-line activity while
//! the child runs, an expandable detail, and a click-through to the child's
//! work. Observation-only: the only handlers are the disclosure and the
//! click-through, matching the render-only posture of every other native
//! control. There is no stop, cancel or steer affordance.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_subagent::{
    is_terminal_subagent_status, subagent_status_label, subagent_status_spins,
    AgentSubagentItem,
};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct AgentSubagentSpec {
    /// The child work this group renders.
    pub item: AgentSubagentItem,
    /// Bindable disclosure state: the detail region shows while expanded.
    pub is_expanded: bool,
    /// Recent activity lines shown when the group is expanded.
    pub detail_lines: Vec<String>,
    /// Collapsed disclosure label.
    pub expand_label: String,
    /// Expanded disclosure label.
    pub collapse_label: String,
    /// Click-through action label — "Open child work".
    pub open_child_label: String,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl AgentSubagentSpec {
    pub fn new(item: AgentSubagentItem) -> Self {
        Self {
            item,
            is_expanded: false,
            detail_lines: Vec::new(),
            expand_label: "Show activity".to_string(),
            collapse_label: "Hide activity".to_string(),
            open_child_label: "Open child work".to_string(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }
    pub fn with_detail_lines(mut self, lines: Vec<String>) -> Self {
        self.detail_lines = lines;
        self
    }
    pub fn with_open_child_label(mut self, label: impl Into<String>) -> Self {
        self.open_child_label = label.into();
        self
    }
    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    /// Whether the child's work has ended; `summary` renders only then.
    pub fn is_terminal(&self) -> bool {
        is_terminal_subagent_status(self.item.status)
    }

    /// Whether the running indicator (spinner) is shown.
    pub fn spins(&self) -> bool {
        subagent_status_spins(self.item.status)
    }

    /// The badge wording, pinned by the shared headless core.
    pub fn status_label(&self) -> &'static str {
        subagent_status_label(self.item.status)
    }

    /// The disclosure is pointless without anything to reveal.
    pub fn shows_toggle(&self) -> bool {
        !self.detail_lines.is_empty()
    }

    /// The disclosure label reflects the next state, like the record's.
    pub fn toggle_label(&self) -> &str {
        if self.is_expanded {
            &self.collapse_label
        } else {
            &self.expand_label
        }
    }

    // ── Tokens ────────────────────────────────────────────────
    pub fn surface_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }
    pub fn label_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
    pub fn activity_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn meta_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }
    /// The badge's colour follows the status: running is the active case,
    /// failed and completed carry their status colours, and everything else
    /// — including `unknown`, which claims nothing — reads at meta strength.
    /// Mirrored by `data-status` rules in `agent-subagent.css`.
    pub fn badge_token(&self) -> &'static str {
        use poodle_headless::agent_subagent::AgentSubagentStatus;
        match self.item.status {
            AgentSubagentStatus::Running => semantic::COLOR_ACCENT_BASE,
            AgentSubagentStatus::Failed => semantic::COLOR_STATUS_DANGER,
            AgentSubagentStatus::Completed => semantic::COLOR_STATUS_SUCCESS,
            _ => semantic::COLOR_TEXT_SECONDARY,
        }
    }

    // ── Size ─────────────────────────────────────────────────
    pub fn font_size_rem(&self) -> f32 {
        match self.size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }

    // ── Density ──────────────────────────────────────────────
    pub fn gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.75,
        }
    }
    pub fn inset_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_subagent::AgentSubagentStatus;

    fn item(status: AgentSubagentStatus) -> AgentSubagentItem {
        AgentSubagentItem {
            id: "child-1".to_string(),
            label: "Scout".to_string(),
            status,
            activity_line: None,
            summary: None,
        }
    }

    #[test]
    fn terminal_statuses_show_the_summary_side_and_running_spins() {
        let running = AgentSubagentSpec::new(item(AgentSubagentStatus::Running));
        assert!(!running.is_terminal());
        assert!(running.spins());

        let done = AgentSubagentSpec::new(item(AgentSubagentStatus::Completed));
        assert!(done.is_terminal());
        assert!(!done.spins());
    }

    #[test]
    fn the_badge_wording_comes_from_the_headless_core() {
        assert_eq!(
            AgentSubagentSpec::new(item(AgentSubagentStatus::Unknown)).status_label(),
            "Unknown"
        );
    }

    #[test]
    fn the_disclosure_only_exists_when_there_is_detail() {
        let bare = AgentSubagentSpec::new(item(AgentSubagentStatus::Running));
        assert!(!bare.shows_toggle());

        let with_detail = bare.clone().with_detail_lines(vec!["line one".to_string()]);
        assert!(with_detail.shows_toggle());
        assert_eq!(with_detail.toggle_label(), "Show activity");
        assert_eq!(with_detail.with_expanded(true).toggle_label(), "Hide activity");
    }

    #[test]
    fn density_moves_spacing_but_not_the_type_scale() {
        let base = AgentSubagentSpec::new(item(AgentSubagentStatus::Running));
        let dense = base.clone().with_density(ControlDensity::Compact);

        assert_ne!(dense.inset_rem(), base.inset_rem());
        assert_eq!(dense.font_size_rem(), base.font_size_rem());
    }
}
