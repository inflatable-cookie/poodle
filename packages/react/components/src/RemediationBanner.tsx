import "@inflatable-cookie/poodle-core/styles/remediation-banner.css";
import { useId } from "react";
import { Button } from "./Button";
import { Icon } from "./Icon";
import { Spinner } from "./Spinner";
import type { AnnouncementMode, RemediationAction, StatusTone, ToneFill } from "./types";

export interface RemediationBannerProps {
  tone?: StatusTone;
  fill?: ToneFill;
  title: string;
  message: string;
  announceMode?: AnnouncementMode;
  primaryAction?: RemediationAction | null;
  secondaryAction?: RemediationAction | null;
  isDismissible?: boolean;
  dismissLabel?: string;
  onAction?: ((id: string) => void) | undefined;
  onDismiss?: (() => void) | undefined;
}

const toneIcon: Record<StatusTone, string> = {
  neutral: "info", info: "info", success: "check", warning: "triangle-alert",
  danger: "circle-x", pending: "loader-circle",
};

export function RemediationBanner({
  tone = "warning", title, message, announceMode = "polite", primaryAction = null,
  fill = "tint",
  secondaryAction = null, isDismissible = false, dismissLabel = "Dismiss",
  onAction = undefined, onDismiss = undefined,
}: RemediationBannerProps) {
  const titleId = `poodle-remediation-banner-title-${useId()}`;
  const role = announceMode === "assertive" ? "alert" : announceMode === "polite" ? "status" : undefined;
  const ariaLive = announceMode === "none" ? undefined : announceMode;
  return (
    <section className="poodle-remediation-banner" data-tone={tone} data-fill={fill} aria-labelledby={titleId} role={role} aria-live={ariaLive}>
      <span className="poodle-remediation-banner__icon" aria-hidden="true">
        {tone === "pending" ? <Spinner variant="ring" tone={fill === "solid" ? "current" : "accent"} /> : <Icon name={toneIcon[tone]} />}
      </span>
      <div className="poodle-remediation-banner__content">
        <strong id={titleId}>{title}</strong>
        <p>{message}</p>
      </div>
      {primaryAction || secondaryAction ? (
        <div className="poodle-remediation-banner__actions">
          {primaryAction ? <Button variant={primaryAction.variant} disabled={primaryAction.isDisabled} onClick={() => onAction?.(primaryAction.id)}>{primaryAction.label}</Button> : null}
          {secondaryAction ? <Button variant={secondaryAction.variant} disabled={secondaryAction.isDisabled} onClick={() => onAction?.(secondaryAction.id)}>{secondaryAction.label}</Button> : null}
        </div>
      ) : null}
      {isDismissible ? <button className="poodle-remediation-banner__dismiss" type="button" aria-label={dismissLabel} onClick={() => onDismiss?.()}><Icon name="x" /></button> : null}
    </section>
  );
}
