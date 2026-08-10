//! Agent subagent machinery. Mirror of core `agent-subagent.ts`: the status
//! vocabulary, the badge wording, the terminal-state mapping, and the running
//! indicator rule for a provider-owned child agent rendered inline in the
//! transcript.
//!
//! Contract: `docs/contracts/components/agent-subagent.md`.
//!
//! The vocabulary is Swallowtail's `SubagentStatus` exactly
//! (`swallowtail-runtime/src/activity/subagent.rs`). The model is
//! observation-only — no control affordances, because steering a
//! provider-owned child is out of the transcript's hands.
//!
//! Parity with the TS core is enforced by `vectors/agent-subagent.json`, run
//! by both runtimes.

/// The provider-visible lifecycle status of a child agent.
///
/// Mirrors Swallowtail's `SubagentStatus` value for value. `Unknown` means "no
/// portable status was supplied" and renders literally as "Unknown" — never
/// inferred or prettified.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AgentSubagentStatus {
    #[default]
    Unknown,
    Pending,
    Running,
    Waiting,
    Completed,
    Failed,
    Interrupted,
    Shutdown,
}

impl AgentSubagentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            AgentSubagentStatus::Unknown => "unknown",
            AgentSubagentStatus::Pending => "pending",
            AgentSubagentStatus::Running => "running",
            AgentSubagentStatus::Waiting => "waiting",
            AgentSubagentStatus::Completed => "completed",
            AgentSubagentStatus::Failed => "failed",
            AgentSubagentStatus::Interrupted => "interrupted",
            AgentSubagentStatus::Shutdown => "shutdown",
        }
    }

    /// Unknown is the fallback: the provider did not supply a portable status.
    pub fn from_str_or_default(value: &str) -> Self {
        match value {
            "pending" => AgentSubagentStatus::Pending,
            "running" => AgentSubagentStatus::Running,
            "waiting" => AgentSubagentStatus::Waiting,
            "completed" => AgentSubagentStatus::Completed,
            "failed" => AgentSubagentStatus::Failed,
            "interrupted" => AgentSubagentStatus::Interrupted,
            "shutdown" => AgentSubagentStatus::Shutdown,
            _ => AgentSubagentStatus::Unknown,
        }
    }
}

/// A provider-owned child agent, as observed by the host.
///
/// Data-only: provenance and persistence are the host's. Nothing here grants
/// control authority over the child.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AgentSubagentItem {
    /// Operation-local identity, stable across activity updates.
    pub id: String,
    /// The child's short label — the name the reader can recognise it by.
    pub label: String,
    pub status: AgentSubagentStatus,
    /// One line of live activity while the child is still working.
    pub activity_line: Option<String>,
    /// What the child accomplished, once terminal.
    pub summary: Option<String>,
}

/// Whether the status means the child's work has ended.
///
/// `Completed`, `Failed`, `Interrupted` and `Shutdown` are terminal. The rest
/// are not — including `Unknown`, which cannot claim a child is finished when
/// the provider never said so.
pub fn is_terminal_subagent_status(status: AgentSubagentStatus) -> bool {
    matches!(
        status,
        AgentSubagentStatus::Completed
            | AgentSubagentStatus::Failed
            | AgentSubagentStatus::Interrupted
            | AgentSubagentStatus::Shutdown
    )
}

/// The badge wording for a status.
///
/// Pinned here rather than left to each renderer so the badge reads the same
/// on every target. `Unknown` stays "Unknown" by design — inferring a prettier
/// word for "the provider did not say" would be making a fact up.
pub fn subagent_status_label(status: AgentSubagentStatus) -> &'static str {
    match status {
        AgentSubagentStatus::Unknown => "Unknown",
        AgentSubagentStatus::Pending => "Pending",
        AgentSubagentStatus::Running => "Running",
        AgentSubagentStatus::Waiting => "Waiting",
        AgentSubagentStatus::Completed => "Completed",
        AgentSubagentStatus::Failed => "Failed",
        AgentSubagentStatus::Interrupted => "Interrupted",
        AgentSubagentStatus::Shutdown => "Shutdown",
    }
}

/// Whether the status signals ongoing work with a spinner.
///
/// Only `Running` spins. A pending or waiting child is not actively working,
/// and a terminal status must never signal ongoing work — the same rule as the
/// transcript's activity footer.
pub fn subagent_status_spins(status: AgentSubagentStatus) -> bool {
    status == AgentSubagentStatus::Running
}
