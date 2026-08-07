//! AgentPlanSpec — the plan an agent proposes at the end of a plan-mode turn.
//!
//! Contract: `docs/contracts/components/agent-plan.md`.
//!
//! On the natives this renders a proposed plan and its decision state; it does
//! not drive the decision, matching the render-only posture of every other
//! native control. See the contract's §14.
//!
//! The spec carries the settled statuses too, so the moment between a decision
//! and the host swapping in the record still renders — but a settled spec
//! shows the badge, not the controls.

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::agent_plan::{
    can_decide_plan, decide_plan, plan_status_label, AgentPlanDecision, AgentPlanStatus,
};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct AgentPlanSpec {
    /// Raw markdown of the proposed plan. Rendered, never pre-rendered by the host.
    pub plan: String,
    pub status: AgentPlanStatus,
    /// Dismiss is a first-class decision for a plan — unlike a question, where
    /// it is the exceptional path — so the control renders by default.
    pub is_dismissible: bool,
    pub dismiss_label: String,
    pub accept_label: String,
    pub revise_label: String,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl AgentPlanSpec {
    pub fn new(plan: impl Into<String>) -> Self {
        Self {
            plan: plan.into(),
            status: AgentPlanStatus::Pending,
            is_dismissible: true,
            dismiss_label: "Dismiss plan".to_string(),
            accept_label: "Accept plan".to_string(),
            revise_label: "Revise".to_string(),
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_status(mut self, status: AgentPlanStatus) -> Self {
        self.status = status;
        self
    }
    pub fn with_dismissible(mut self, dismissible: bool) -> Self {
        self.is_dismissible = dismissible;
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

    /// Controls render only while the plan waits on the operator.
    pub fn can_decide(&self) -> bool {
        can_decide_plan(self.status)
    }

    pub fn decide(&self, next: AgentPlanStatus) -> Option<AgentPlanDecision> {
        decide_plan(self.status, next, None)
    }

    pub fn status_label(&self) -> &'static str {
        plan_status_label(self.status)
    }

    // ── Tokens ────────────────────────────────────────────────
    pub fn plan_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
    pub fn accent_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }
    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }
    pub fn action_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }
    /// The accept control is the one filled surface, so its label inverts.
    pub fn primary_action_token(&self) -> &'static str {
        semantic::COLOR_TEXT_INVERSE
    }
    pub fn badge_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
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
    pub fn action_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.625,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pending_plan_can_be_decided_and_a_settled_one_cannot() {
        let spec = AgentPlanSpec::new("Ship it.");
        assert!(spec.can_decide());
        let decision = spec.decide(AgentPlanStatus::Accepted).expect("a decision");
        assert_eq!(decision.status, AgentPlanStatus::Accepted);

        let settled = spec.with_status(AgentPlanStatus::Accepted);
        assert!(!settled.can_decide());
        assert!(settled.decide(AgentPlanStatus::Dismissed).is_none());
    }

    #[test]
    fn the_badge_wording_comes_from_the_headless_core() {
        assert_eq!(
            AgentPlanSpec::new("x").with_status(AgentPlanStatus::Revised).status_label(),
            "Revised"
        );
    }

    #[test]
    fn density_moves_spacing_but_not_the_type_scale() {
        let base = AgentPlanSpec::new("x");
        let dense = base.clone().with_density(ControlDensity::Compact);

        assert_ne!(dense.gap_rem(), base.gap_rem());
        assert_eq!(dense.font_size_rem(), base.font_size_rem());
    }
}
