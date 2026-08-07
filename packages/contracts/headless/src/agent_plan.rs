//! Agent plan machinery. Mirror of core `agent-plan.ts`: the decision
//! lifecycle (pending → accepted / revised / dismissed), the badge wording,
//! and the summary a settled plan leaves in the transcript.
//!
//! Contract: `docs/contracts/components/agent-plan.md`.
//!
//! Unlike a question, a proposed plan does not block the turn: the turn is
//! complete, and the plan waits on the operator's next action.
//!
//! Parity with the TS core is enforced by `vectors/agent-plan.json`, run by
//! both runtimes.

/// Where a proposed plan stands. `Pending` is the only undecided state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AgentPlanStatus {
    #[default]
    Pending,
    Accepted,
    Revised,
    Dismissed,
}

impl AgentPlanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            AgentPlanStatus::Pending => "pending",
            AgentPlanStatus::Accepted => "accepted",
            AgentPlanStatus::Revised => "revised",
            AgentPlanStatus::Dismissed => "dismissed",
        }
    }

    pub fn from_str_or_default(value: &str) -> Self {
        match value {
            "accepted" => AgentPlanStatus::Accepted,
            "revised" => AgentPlanStatus::Revised,
            "dismissed" => AgentPlanStatus::Dismissed,
            _ => AgentPlanStatus::Pending,
        }
    }

    /// The states a decision can settle a plan into.
    pub fn is_settled(self) -> bool {
        !matches!(self, AgentPlanStatus::Pending)
    }
}

/// The operator's decision on a proposed plan.
///
/// Data-only: who decided and why is provenance the host persists; the
/// component renders what it is given.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentPlanDecision {
    pub status: AgentPlanStatus,
    /// ISO timestamp, formatted by the host. `None` when the host does not
    /// track it.
    pub decided_at: Option<String>,
}

/// True while the plan still waits on the operator.
pub fn can_decide_plan(status: AgentPlanStatus) -> bool {
    !status.is_settled()
}

/// Settle a pending plan.
///
/// Returns `None` for an already-settled plan: a decision the host has acted
/// on cannot be re-decided, for the same reason the transcript record carries
/// no re-decide affordance.
pub fn decide_plan(
    status: AgentPlanStatus,
    next: AgentPlanStatus,
    decided_at: Option<&str>,
) -> Option<AgentPlanDecision> {
    if !can_decide_plan(status) || !next.is_settled() {
        return None;
    }
    Some(AgentPlanDecision {
        status: next,
        decided_at: decided_at.map(str::to_string),
    })
}

/// The badge wording for a status.
///
/// Pinned here rather than left to each renderer so the badge reads the same
/// on every target.
pub fn plan_status_label(status: AgentPlanStatus) -> &'static str {
    match status {
        AgentPlanStatus::Pending => "Pending",
        AgentPlanStatus::Accepted => "Accepted",
        AgentPlanStatus::Revised => "Revised",
        AgentPlanStatus::Dismissed => "Dismissed",
    }
}

/// One line of the plan, for the collapsed record.
///
/// The full plan is markdown; a summary is not. Whitespace collapses to single
/// spaces, and truncation appends an ellipsis counted against the budget, so
/// the summary never exceeds `max_length`.
pub fn plan_record_summary(plan: &str, max_length: usize) -> String {
    if max_length == 0 {
        return String::new();
    }

    let flat = plan.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= max_length {
        return flat;
    }

    let kept: String = flat.chars().take(max_length - 1).collect();
    format!("{}…", kept.trim_end())
}
