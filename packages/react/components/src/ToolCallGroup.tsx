import { useState } from "react";

import "@poodle/styles/tool-call-group.css";

import { toolRunStatus } from "@poodle/headless";

import { Icon } from "./Icon";
import { ToolCall } from "./ToolCall";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
  TranscriptToolCall,
} from "./types";

export interface ToolCallGroupProps {
  id: string;
  calls?: TranscriptToolCall[];
  expanded?: boolean;
  expandedCalls?: string[];
  moreLabel?: (count: number) => string;
  fewerLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onToggle?: (id: string) => void;
  onCallToggle?: (id: string) => void;
}

export function ToolCallGroup({
  id,
  calls = [],
  expanded,
  expandedCalls,
  moreLabel = (count: number) => `+${count} previous tool calls`,
  fewerLabel = "Show fewer tool calls",
  size = null,
  sizeRole = "control",
  density = null,
  onToggle,
  onCallToggle,
}: ToolCallGroupProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;
  const glyphSize = resolveSupportingVisualSize(resolvedSize);

  const [uncontrolledExpanded, setUncontrolledExpanded] = useState(false);
  const [uncontrolledCalls, setUncontrolledCalls] = useState<string[]>([]);
  const isExpanded = expanded ?? uncontrolledExpanded;
  const openCalls = expandedCalls ?? uncontrolledCalls;

  const hiddenCount = Math.max(0, calls.length - 1);
  const showsToggle = hiddenCount > 0;
  const status = toolRunStatus({ kind: "tool-run", id, calls });

  /**
   * Collapsed shows the run's newest call; expanded lists every call in order
   * and therefore ends on that same call. Expanding is safe to do while
   * reading: the row under the cursor stays put and the rest appears above it.
   */
  const renderedCalls = isExpanded ? calls : calls.slice(-1);

  /**
   * A collapsed failing run must not be announced identically to a passing one,
   * so a non-success status is carried in the toggle's name as well as its
   * colour.
   */
  const toggleName = isExpanded
    ? fewerLabel
    : `${moreLabel(hiddenCount)}${
        status === "error" ? ", contains a failure" : status === "running" ? ", in progress" : ""
      }`;

  const toggle = () => {
    if (expanded === undefined) setUncontrolledExpanded((value) => !value);
    onToggle?.(id);
  };

  const toggleCall = (callId: string) => {
    if (expandedCalls === undefined) {
      setUncontrolledCalls((list) =>
        list.includes(callId) ? list.filter((value) => value !== callId) : [...list, callId],
      );
    }
    onCallToggle?.(callId);
  };

  return (
    <div
      className="poodle-tool-call-group"
      data-expanded={String(isExpanded)}
      data-status={status}
      data-count={calls.length}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <ul className="poodle-tool-call-group__list" id={`${id}-calls`}>
        {renderedCalls.map((call) => (
          // The list item lives here, not in ToolCall: a row that is an `<li>`
          // can never be valid on its own, and the component is usable outside
          // a group.
          <li key={call.id}>
            <ToolCall
              id={call.id}
              label={call.label}
              detail={call.detail ?? null}
              status={call.status}
              icon={call.icon ?? null}
              output={call.output ?? null}
              expanded={openCalls.includes(call.id)}
              size={resolvedSize}
              density={resolvedDensity}
              onToggle={toggleCall}
            />
          </li>
        ))}
      </ul>

      {/* Omitted rather than hidden when there is nothing to reveal, so a
          single-call run leaves no stray tab stop. The toggle is always the
          last child, in both states, which is what keeps focus still while
          expanding. */}
      {showsToggle ? (
        <button
          type="button"
          className="poodle-tool-call-group__toggle"
          aria-expanded={isExpanded}
          aria-controls={`${id}-calls`}
          aria-label={toggleName}
          onClick={toggle}
        >
          <span className="poodle-tool-call-group__toggle-icon">
            <Icon name="chevron-down" size={glyphSize} />
          </span>
          <span>{isExpanded ? fewerLabel : moreLabel(hiddenCount)}</span>
        </button>
      ) : null}
    </div>
  );
}
