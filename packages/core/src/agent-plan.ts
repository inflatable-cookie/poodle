/**
 * Agent plan machinery.
 * Contract: docs/contracts/components/agent-plan.md.
 *
 * Pure logic for the plan an agent proposes at the end of a plan-mode turn:
 * the decision lifecycle (pending → accepted / revised / dismissed), the badge
 * wording, and the summary a settled plan leaves in the transcript. Rendering
 * and key binding stay adapter-side.
 *
 * The interactive control renders inside `AgentChatInput`'s field — it is input
 * requiring the operator's attention, and the transcript already gets the
 * settled record — but nothing in this module depends on that. Unlike a
 * question, a proposed plan does not block the turn: the turn is complete, and
 * the plan waits on the operator's next action.
 *
 * The component owns no text input. Revise is a signal to the host to focus the
 * composer's editor, where revision feedback is typed as an ordinary message.
 *
 * The Rust mirror is `poodle-headless::agent_plan`. Both are driven by
 * `packages/contracts/headless/vectors/agent-plan.json`, so the decision
 * lifecycle cannot drift between the web targets and the natives.
 */

/** Where a proposed plan stands. `pending` is the only undecided state. */
export type AgentPlanStatus = "pending" | "accepted" | "revised" | "dismissed";

/** The states a decision can settle a plan into. */
export type AgentPlanSettledStatus = Exclude<AgentPlanStatus, "pending">;

/**
 * The operator's decision on a proposed plan.
 *
 * Data-only: who decided and why is provenance the host persists (nucleus
 * does); the component renders what it is given.
 */
export interface AgentPlanDecision {
  status: AgentPlanSettledStatus;
  /** ISO timestamp, formatted by the host. Absent when the host does not track it. */
  decidedAt?: string;
}

/** True while the plan still waits on the operator. */
export function canDecidePlan(status: AgentPlanStatus): boolean {
  return status === "pending";
}

/**
 * Settle a pending plan.
 *
 * Returns `null` for an already-settled plan: a decision the host has acted on
 * cannot be re-decided, for the same reason the transcript record carries no
 * re-decide affordance.
 */
export function decidePlan(
  status: AgentPlanStatus,
  next: AgentPlanSettledStatus,
  decidedAt?: string,
): AgentPlanDecision | null {
  if (!canDecidePlan(status)) return null;
  return decidedAt === undefined ? { status: next } : { status: next, decidedAt };
}

/**
 * The badge wording for a status.
 *
 * Pinned here rather than left to each renderer so the badge reads the same on
 * every target. "the plan was accepted" is a fact about the conversation, and
 * two spellings of it is drift.
 */
export function planStatusLabel(status: AgentPlanStatus): string {
  switch (status) {
    case "pending":
      return "Pending";
    case "accepted":
      return "Accepted";
    case "revised":
      return "Revised";
    case "dismissed":
      return "Dismissed";
  }
}

/**
 * One line of the plan, for the collapsed record.
 *
 * The full plan is markdown; a summary is not. Whitespace collapses to single
 * spaces — a heading marker or a list bullet has no meaning on one line, and
 * rendering markdown inline would ask the record to be a second message.
 * Truncation appends an ellipsis and counts it against the budget, so the
 * summary never exceeds `maxLength`.
 */
export function planRecordSummary(plan: string, maxLength = 160): string {
  if (maxLength <= 0) return "";

  const flat = plan.replace(/\s+/g, " ").trim();
  if (flat.length <= maxLength) return flat;

  return `${flat.slice(0, maxLength - 1).trimEnd()}…`;
}
