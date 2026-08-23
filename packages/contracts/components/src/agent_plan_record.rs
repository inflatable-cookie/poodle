//! AgentPlanRecordSpec — the read-only record a decided plan leaves.
//!
//! Contract: `docs/contracts/components/agent-plan-record.md`.
//!
//! No decision affordance at all, which is what makes hosting the live plan in
//! the composer safe: a re-decide control here would let the reader change a
//! decision the agent has already acted on. The one interactive part is the
//! disclosure between the summary and the full plan.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_plan::{plan_record_summary, plan_status_label, AgentPlanStatus};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct AgentPlanRecordSpec {
    /// Raw markdown of the plan that was decided.
    pub plan: String,
    /// A settled status. The record is what a decision leaves behind, so
    /// `Pending` never reaches it.
    pub status: AgentPlanStatus,
    /// Overrides the badge wording; `None` uses the status label.
    pub decision_label: Option<String>,
    /// When the decision was made, formatted by the host.
    pub decided_at: Option<String>,
    /// Character budget for the collapsed summary, ellipsis included.
    pub summary_max_length: usize,
    pub is_expanded: bool,
    pub expand_label: String,
    pub collapse_label: String,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl AgentPlanRecordSpec {
    pub fn new(plan: impl Into<String>, status: AgentPlanStatus) -> Self {
        Self {
            plan: plan.into(),
            status,
            decision_label: None,
            decided_at: None,
            summary_max_length: 160,
            is_expanded: false,
            expand_label: "Show plan".to_string(),
            collapse_label: "Hide plan".to_string(),
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }

    pub fn with_decision_label(mut self, label: impl Into<String>) -> Self {
        self.decision_label = Some(label.into());
        self
    }
    pub fn with_decided_at(mut self, decided_at: impl Into<String>) -> Self {
        self.decided_at = Some(decided_at.into());
        self
    }
    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = expanded;
        self
    }
    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }
    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    /// The badge says what was decided, in the host's words when it supplies
    /// them and in the shared wording otherwise.
    pub fn badge_label(&self) -> String {
        self.decision_label
            .clone()
            .unwrap_or_else(|| plan_status_label(self.status).to_string())
    }

    pub fn summary(&self) -> String {
        plan_record_summary(&self.plan, self.summary_max_length)
    }

    /// An expanded record shows the whole plan; the summary is the stand-in
    /// for exactly the content it hides, so showing both says one thing twice.
    pub fn shows_summary(&self) -> bool {
        !self.is_expanded
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
    pub fn badge_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }
    /// One step down from live prose: this is history, not the current subject.
    pub fn summary_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    pub fn meta_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }

    // ── Size ─────────────────────────────────────────────────
    pub fn font_size_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.8125,
            ControlSize::Lg => 0.875,
            ControlSize::Xl => 0.9375,
        }
    }

    // ── Density ──────────────────────────────────────────────
    pub fn gap_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }
    }
    pub fn padding_inset_rem(&self, density: ControlDensity) -> f32 {
        match density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_badge_falls_back_to_the_status_label() {
        let spec = AgentPlanRecordSpec::new("Ship it.", AgentPlanStatus::Accepted);
        assert_eq!(spec.badge_label(), "Accepted");

        let labelled = spec.with_decision_label("Accepted with changes");
        assert_eq!(labelled.badge_label(), "Accepted with changes");
    }

    #[test]
    fn the_summary_yields_to_the_full_plan() {
        let spec = AgentPlanRecordSpec::new("Step one. Step two.", AgentPlanStatus::Accepted);
        assert!(spec.shows_summary());
        assert!(!spec.with_expanded(true).shows_summary());
    }

    #[test]
    fn the_summary_respects_the_budget() {
        let spec = AgentPlanRecordSpec::new("a b c d e f g h", AgentPlanStatus::Dismissed);
        assert_eq!(spec.summary(), "a b c d e f g h");

        let tight = AgentPlanRecordSpec {
            summary_max_length: 4,
            ..AgentPlanRecordSpec::new("a b c d e f g h", AgentPlanStatus::Dismissed)
        };
        assert_eq!(tight.summary(), "a b…");
    }
}
