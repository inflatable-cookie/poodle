import { useState } from "react";

import "@poodle/styles/tool-call.css";

import { Code } from "./Code";
import { Icon } from "./Icon";
import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, ToolCallStatus } from "./types";

export interface ToolCallProps {
  id: string;
  label: string;
  detail?: string | null;
  status?: ToolCallStatus;
  icon?: string | null;
  output?: string | null;
  outputLanguage?: string | null;
  expanded?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onToggle?: (id: string) => void;
}

/**
 * The icon for a kind of work.
 *
 * Shared with the natives through the spec so every target agrees. `icon`
 * exists so a host with its own vocabulary is never stuck with the fallback.
 */
export function iconForToolCallLabel(value: string): string {
  const key = value.trim().toLowerCase();
  if (key.startsWith("ran command") || key.startsWith("command")) return "terminal";
  if (key.startsWith("file change") || key.startsWith("edited")) return "file-pen";
  if (key.startsWith("search")) return "search";
  if (key.startsWith("read")) return "file-text";
  return "dot";
}

export function ToolCall({
  id,
  label,
  detail = null,
  status = "success",
  icon = null,
  output = null,
  outputLanguage = null,
  expanded,
  size = null,
  sizeRole = "control",
  density = null,
  onToggle,
}: ToolCallProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;
  const glyphSize = resolveSupportingVisualSize(resolvedSize);

  const [uncontrolled, setUncontrolled] = useState(false);
  const isControlled = expanded !== undefined;
  const isExpanded = isControlled ? expanded : uncontrolled;

  const hasOutput = typeof output === "string" && output.length > 0;
  const resolvedIcon = icon ?? iconForToolCallLabel(label);

  /**
   * Status reaches assistive technology through the name; colour and glyph do
   * not. `success` is omitted as the unremarkable case, and the detail is
   * carried in full — the truncation is visual, and the whole command is
   * exactly what a truncated row is hiding.
   */
  const accessibleName = `${label}${detail ? `: ${detail}` : ""}${status === "success" ? "" : `, ${status}`}`;

  const toggle = () => {
    if (!hasOutput) return;
    if (!isControlled) setUncontrolled((value) => !value);
    onToggle?.(id);
  };

  const body = (
    <>
      <span className="poodle-tool-call__icon">
        <Icon name={resolvedIcon} size={glyphSize} />
      </span>
      <span className="poodle-tool-call__label">{label}</span>
      {detail ? <code className="poodle-tool-call__detail">{detail}</code> : null}
      {hasOutput ? (
        <span className="poodle-tool-call__disclosure">
          <Icon name="chevron-down" size={glyphSize} />
        </span>
      ) : null}
      <span className="poodle-tool-call__status" aria-hidden="true">
        {status === "running" ? (
          <Spinner variant="ring" size={glyphSize} tone="current" />
        ) : status === "error" ? (
          <Icon name="x" size={glyphSize} />
        ) : (
          <Icon name="check" size={glyphSize} />
        )}
      </span>
    </>
  );

  return (
    <div
      className="poodle-tool-call"
      data-status={status}
      data-expanded={hasOutput ? String(isExpanded) : undefined}
      data-interactive={String(hasOutput)}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {/* A control that cannot do anything should not be in the tab order at
          all, so a row with no output is a div rather than a disabled button. */}
      {hasOutput ? (
        <button
          type="button"
          className="poodle-tool-call__trigger"
          aria-expanded={isExpanded}
          aria-controls={`${id}-output`}
          aria-label={accessibleName}
          onClick={toggle}
        >
          {body}
        </button>
      ) : (
        <div className="poodle-tool-call__trigger" role="presentation">
          {body}
        </div>
      )}

      {/* Lazily constructed: a transcript of a thousand rows must not build a
          thousand code blocks for output nobody opened. */}
      {hasOutput && isExpanded ? (
        <div className="poodle-tool-call__output" id={`${id}-output`}>
          <Code source={output ?? ""} language={outputLanguage} size={resolvedSize} />
        </div>
      ) : null}
    </div>
  );
}
