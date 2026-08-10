/**
 * Agent subagent machinery.
 * Contract: docs/contracts/components/agent-subagent.md.
 *
 * Pure logic for a provider-owned child agent (sub-agent) rendered inline in
 * the agent transcript: the status vocabulary, the badge wording, the
 * terminal-state mapping, and the running indicator rule. Rendering and the
 * click-through action stay adapter-side.
 *
 * The vocabulary is Swallowtail's `SubagentStatus` exactly
 * (`swallowtail-runtime/src/activity/subagent.rs`): unknown, pending, running,
 * waiting, completed, failed, interrupted, shutdown. The model is
 * observation-only — there are no control affordances, because steering a
 * provider-owned child is out of the transcript's hands.
 *
 * The Rust mirror is `poodle-headless::agent_subagent`. Both are driven by
 * `packages/contracts/headless/vectors/agent-subagent.json`, so the badge
 * wording and the terminal mapping cannot drift between the web targets and
 * the natives.
 */

/**
 * The provider-visible lifecycle status of a child agent.
 *
 * Mirrors Swallowtail's `SubagentStatus` value for value. `unknown` means "no
 * portable status was supplied" and renders literally as "Unknown" — never
 * inferred or prettified.
 */
export type AgentSubagentStatus =
  | "unknown"
  | "pending"
  | "running"
  | "waiting"
  | "completed"
  | "failed"
  | "interrupted"
  | "shutdown";

/**
 * A provider-owned child agent, as observed by the host.
 *
 * Data-only: provenance and persistence are the host's. Nothing here grants
 * control authority over the child.
 */
export interface AgentSubagentItem {
  /** Operation-local identity, stable across activity updates. */
  id: string;
  /** The child's short label — the name the reader can recognise it by. */
  label: string;
  status: AgentSubagentStatus;
  /** One line of live activity while the child is still working. */
  activityLine?: string;
  /** What the child accomplished, once terminal. */
  summary?: string;
}

/**
 * Whether the status means the child's work has ended.
 *
 * `completed`, `failed`, `interrupted` and `shutdown` are terminal. The rest
 * are not — including `unknown`, which cannot claim a child is finished when
 * the provider never said so.
 */
export function isTerminalSubagentStatus(status: AgentSubagentStatus): boolean {
  switch (status) {
    case "completed":
    case "failed":
    case "interrupted":
    case "shutdown":
      return true;
    case "unknown":
    case "pending":
    case "running":
    case "waiting":
      return false;
  }
}

/**
 * The badge wording for a status.
 *
 * Pinned here rather than left to each renderer so the badge reads the same on
 * every target. `unknown` stays "Unknown" by design — inferring a prettier
 * word for "the provider did not say" would be making a fact up.
 */
export function subagentStatusLabel(status: AgentSubagentStatus): string {
  switch (status) {
    case "unknown":
      return "Unknown";
    case "pending":
      return "Pending";
    case "running":
      return "Running";
    case "waiting":
      return "Waiting";
    case "completed":
      return "Completed";
    case "failed":
      return "Failed";
    case "interrupted":
      return "Interrupted";
    case "shutdown":
      return "Shutdown";
  }
}

/**
 * Whether the status signals ongoing work with a spinner.
 *
 * Only `running` spins. A pending or waiting child is not actively working,
 * and a terminal status must never signal ongoing work — the same rule as the
 * transcript's activity footer.
 */
export function subagentStatusSpins(status: AgentSubagentStatus): boolean {
  return status === "running";
}
