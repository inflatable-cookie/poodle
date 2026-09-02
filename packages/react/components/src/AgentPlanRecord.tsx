import { useState } from "react";

import "@inflatable-cookie/poodle-core/styles/agent-plan-record.css";

import {
  planRecordSummary,
  planStatusLabel,
  type AgentPlanSettledStatus,
} from "@inflatable-cookie/poodle-core";

import { AgentMessage } from "./AgentMessage";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface AgentPlanRecordProps {
  /** Raw markdown of the plan that was decided. */
  plan: string;
  /** A settled status. The record is what a decision leaves behind, so `pending` never reaches it. */
  status: AgentPlanSettledStatus;
  /** Overrides the badge wording; defaults to the status label. */
  decisionLabel?: string;
  /** When the decision was made, formatted by the host. */
  decidedAt?: string;
  /** Character budget for the collapsed summary, ellipsis included. */
  summaryMaxLength?: number;
  expanded?: boolean;
  expandLabel?: string;
  collapseLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onToggle?: ((expanded: boolean) => void) | undefined;
}

export function AgentPlanRecord({
  plan,
  status,
  decisionLabel,
  decidedAt,
  summaryMaxLength = 160,
  expanded,
  expandLabel = "Show plan",
  collapseLabel = "Hide plan",
  size = null,
  sizeRole = "control",
  density = null,
  onToggle,
}: AgentPlanRecordProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;

  const [uncontrolledExpanded, setUncontrolledExpanded] = useState(false);
  const isExpanded = expanded ?? uncontrolledExpanded;

  const badge = decisionLabel ?? planStatusLabel(status);
  const summary = planRecordSummary(plan, summaryMaxLength);

  const toggle = () => {
    const nextExpanded = !isExpanded;
    if (expanded === undefined) setUncontrolledExpanded(nextExpanded);
    onToggle?.(nextExpanded);
  };

  return (
    <div
      className="poodle-agent-plan-record"
      data-status={status}
      data-expanded={String(isExpanded)}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div className="poodle-agent-plan-record__header">
        <span className="poodle-agent-plan-record__badge" data-status={status}>
          {badge}
        </span>
        {decidedAt ? <span className="poodle-agent-plan-record__meta">{decidedAt}</span> : null}
      </div>

      {/* The summary is a stand-in for exactly the content it hides, so the two
          never render together. */}
      {!isExpanded ? (
        <p className="poodle-agent-plan-record__summary">{summary}</p>
      ) : (
        <div className="poodle-agent-plan-record__body">
          <AgentMessage markdown={plan} role="assistant" size={resolvedSize} density={resolvedDensity} />
        </div>
      )}

      <button type="button" className="poodle-agent-plan-record__toggle" aria-expanded={isExpanded} onClick={toggle}>
        {isExpanded ? collapseLabel : expandLabel}
      </button>
    </div>
  );
}