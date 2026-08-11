import { useState } from "react";

import "@inflatable-cookie/poodle-core/styles/agent-subagent.css";

import {
  isTerminalSubagentStatus,
  subagentStatusLabel,
  subagentStatusSpins,
} from "@inflatable-cookie/poodle-core";

import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  AgentSubagentItem,
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
} from "./types";

export interface AgentSubagentProps {
  item: AgentSubagentItem;
  expanded?: boolean;
  detailLines?: string[];
  expandLabel?: string;
  collapseLabel?: string;
  openChildLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onToggle?: (expanded: boolean) => void;
  onOpenChild?: () => void;
}

export function AgentSubagent({
  item,
  expanded,
  detailLines = [],
  expandLabel = "Show activity",
  collapseLabel = "Hide activity",
  openChildLabel = "Open child work",
  size = null,
  sizeRole = "control",
  density = null,
  onToggle,
  onOpenChild,
}: AgentSubagentProps) {
  const presentation = useUiPresentation();
  const resolvedSize =
    size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;
  const [uncontrolledExpanded, setUncontrolledExpanded] = useState(false);
  const isExpanded = expanded ?? uncontrolledExpanded;
  const isTerminal = isTerminalSubagentStatus(item.status);
  const spins = subagentStatusSpins(item.status);
  const showsToggle = detailLines.length > 0;

  const toggle = () => {
    const nextExpanded = !isExpanded;
    if (expanded === undefined) setUncontrolledExpanded(nextExpanded);
    onToggle?.(nextExpanded);
  };

  return (
    <div
      className="poodle-agent-subagent"
      data-status={item.status}
      data-expanded={String(isExpanded)}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <div className="poodle-agent-subagent__header">
        <span className="poodle-agent-subagent__label">{item.label}</span>
        <span
          className="poodle-agent-subagent__badge"
          data-status={item.status}
        >
          {subagentStatusLabel(item.status)}
        </span>
      </div>

      {isTerminal ? (
        item.summary ? (
          <p className="poodle-agent-subagent__summary">{item.summary}</p>
        ) : null
      ) : (
        <div className="poodle-agent-subagent__activity">
          {spins ? (
            <Spinner variant="dots" size={resolvedSize} tone="muted" />
          ) : null}
          {item.activityLine ? (
            <span className="poodle-agent-subagent__activity-line">
              {item.activityLine}
            </span>
          ) : null}
        </div>
      )}

      {isExpanded && showsToggle ? (
        <ul className="poodle-agent-subagent__detail">
          {detailLines.map((line, index) => (
            <li key={`${index}-${line}`}>{line}</li>
          ))}
        </ul>
      ) : null}

      <div className="poodle-agent-subagent__actions">
        {showsToggle ? (
          <button
            type="button"
            className="poodle-agent-subagent__action"
            data-kind="toggle"
            aria-expanded={isExpanded}
            onClick={toggle}
          >
            {isExpanded ? collapseLabel : expandLabel}
          </button>
        ) : null}
        {onOpenChild ? (
          <button
            type="button"
            className="poodle-agent-subagent__action"
            data-kind="open"
            onClick={onOpenChild}
          >
            {openChildLabel}
          </button>
        ) : null}
      </div>
    </div>
  );
}
