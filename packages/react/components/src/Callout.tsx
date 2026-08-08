import "@inflatable-cookie/poodle-styles/callout.css";

import type { ReactNode } from "react";

import { Icon } from "./Icon";
import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { CalloutAnnounceMode, ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone } from "./types";

export interface CalloutProps {
  tone?: StatusTone | "neutral";
  title?: string | null;
  message?: string | null;
  ariaLabel?: string | null;
  announceMode?: CalloutAnnounceMode;
  dismissible?: boolean;
  dismissLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onDismiss?: (() => void) | undefined;
  icon?: ReactNode;
  actions?: ReactNode;
  children?: ReactNode;
}

const toneIcon: Record<string, string> = {
  success: "check",
  warning: "triangle-alert",
  danger: "circle-x",
  info: "info",
  neutral: "info",
};

export function Callout({
  tone = "neutral",
  title = null,
  message = null,
  ariaLabel = null,
  announceMode = "none",
  dismissible = false,
  dismissLabel = "Dismiss message",
  size = null,
  sizeRole = "control",
  density = null,
  onDismiss = undefined,
  icon,
  actions,
  children,
}: CalloutProps) {
  const uiPresentation = useUiPresentation();

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const role = announceMode === "assertive" ? "alert" : announceMode === "polite" ? "status" : undefined;
  const ariaLive = announceMode === "assertive" ? ("assertive" as const) : announceMode === "polite" ? ("polite" as const) : undefined;

  return (
    <section
      className="poodle-callout"
      data-tone={tone}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      aria-label={ariaLabel ?? undefined}
      role={role}
      aria-live={ariaLive}
    >
      <div className="poodle-callout__body">
        <span className="poodle-callout__icon" aria-hidden="true">
          {icon ??
            (tone === "pending" ? (
              <Spinner variant="ring" size={resolvedSize} sizeRole="chrome" tone="accent" />
            ) : (
              <Icon name={toneIcon[tone] ?? "info"} size={resolvedSize} />
            ))}
        </span>

        <div className="poodle-callout__content">
          {title ? <strong>{title}</strong> : null}
          {message ? <p>{message}</p> : null}
          {children}
        </div>
      </div>

      {actions ? <div className="poodle-callout__actions">{actions}</div> : null}

      {dismissible ? (
        <button type="button" className="poodle-callout__dismiss" aria-label={dismissLabel} onClick={() => onDismiss?.()}>
          <Icon name="x" />
        </button>
      ) : null}
    </section>
  );
}
