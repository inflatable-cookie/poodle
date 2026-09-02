import { lazy, Suspense } from "react";

import "@inflatable-cookie/poodle-core/styles/agent-plan.css";

import { canDecidePlan, planStatusLabel, type AgentPlanStatus } from "@inflatable-cookie/poodle-core";

import { loadAgentMessage } from "./agent-message-load";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

const AgentMessage = lazy(async () => {
  const module = await loadAgentMessage();
  return { default: module.AgentMessage };
});

export interface AgentPlanProps {
  /** Raw markdown of the proposed plan. Rendered, never pre-rendered by the host. */
  plan?: string;
  status?: AgentPlanStatus;
  /** Dismiss is a first-class decision for a plan, so the control renders by default. */
  dismissible?: boolean;
  dismissLabel?: string;
  acceptLabel?: string;
  reviseLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onAccept?: (() => void) | undefined;
  onRevise?: (() => void) | undefined;
  onDismiss?: (() => void) | undefined;
}

export function AgentPlan({
  plan = "",
  status = "pending",
  dismissible = true,
  dismissLabel = "Dismiss plan",
  acceptLabel = "Accept plan",
  reviseLabel = "Revise",
  size = null,
  sizeRole = "control",
  density = null,
  onAccept,
  onRevise,
  onDismiss,
}: AgentPlanProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;

  /**
   * Controls render only while the plan waits on the operator. A settled status
   * shows the badge instead, covering the moment between the decision and the
   * host swapping in the record — settled display proper is AgentPlanRecord.
   */
  const isPending = canDecidePlan(status);

  return (
    <div
      className="poodle-agent-plan"
      data-status={status}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {/* The plan is markdown, rendered by the same path as the turn's prose. */}
      <div className="poodle-agent-plan__body">
        <Suspense fallback={null}>
          <AgentMessage markdown={plan} role="assistant" size={resolvedSize} density={resolvedDensity} />
        </Suspense>
      </div>

      {isPending ? (
        <div className="poodle-agent-plan__actions">
          <button
            type="button"
            className="poodle-agent-plan__action"
            data-variant="primary"
            onClick={() => onAccept?.()}
          >
            {acceptLabel}
          </button>
          {/* Revise owns no text input: the host focuses the composer, where the
              feedback is typed as an ordinary message. */}
          <button
            type="button"
            className="poodle-agent-plan__action"
            data-variant="secondary"
            onClick={() => onRevise?.()}
          >
            {reviseLabel}
          </button>
          {dismissible ? (
            <button
              type="button"
              className="poodle-agent-plan__action"
              data-variant="ghost"
              onClick={() => onDismiss?.()}
            >
              {dismissLabel}
            </button>
          ) : null}
        </div>
      ) : (
        <span className="poodle-agent-plan__badge" data-status={status}>
          {planStatusLabel(status)}
        </span>
      )}
    </div>
  );
}