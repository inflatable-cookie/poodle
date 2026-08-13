import { useEffect, useReducer, useState } from "react";

import "@inflatable-cookie/poodle-core/styles/update-center.css";
import {
  updateStatusView,
  type Channel,
  type UpdateAheadOfChannel,
  type UpdateAvailabilityProjection,
  type UpdateControllerStatus,
  type UpdateDeferral,
  type UpdateProgressProjection,
  type UpdateRejectionCode,
  type UpdateStatusAction,
} from "@inflatable-cookie/poodle-core";

import { AlertDialog } from "./AlertDialog";
import { Button } from "./Button";
import { Progress } from "./Progress";
import { Spinner } from "./Spinner";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface UpdateStatusProps {
  status?: UpdateControllerStatus;
  availability?: UpdateAvailabilityProjection;
  progress?: UpdateProgressProjection;
  channel?: Channel;
  installedVersion?: string;
  deferral?: UpdateDeferral;
  lastRejection?: UpdateRejectionCode;
  aheadOfChannel?: UpdateAheadOfChannel;
  pending?: boolean;
  observe?: ((observer: () => void) => () => void) | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  installLabel?: string;
  deferLabel?: string;
  checkLabel?: string;
  retryLabel?: string;
  confirmInstall?: boolean;
  onCheck?: (() => void) | null;
  onInstall?: (() => void) | null;
  onDefer?: (() => void) | null;
}

export function UpdateStatus({
  status = { kind: "idle" },
  availability,
  progress,
  channel,
  installedVersion,
  deferral,
  lastRejection,
  aheadOfChannel,
  pending = false,
  observe = null,
  size = null,
  sizeRole = "control",
  density = null,
  installLabel = "Install and restart",
  deferLabel = "Later",
  checkLabel = "Check for updates",
  retryLabel = "Try again",
  confirmInstall = true,
  onCheck = null,
  onInstall = null,
  onDefer = null,
}: UpdateStatusProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  const [confirmOpen, setConfirmOpen] = useState(false);
  // Bumped by `observe` so the component re-renders when the authority notifies.
  const [, force] = useReducer((x: number) => x + 1, 0);

  useEffect(() => {
    if (!observe) return;
    return observe(() => force());
  }, [observe]);

  const view = updateStatusView({
    status,
    availability,
    progress,
    deferral,
    lastRejection,
    aheadOfChannel,
    channel,
    installedVersion,
  });

  const actionLabels: Record<UpdateStatusAction["type"], string> = {
    install: installLabel,
    defer: deferLabel,
    check: checkLabel,
  };

  function dispatch(action: UpdateStatusAction): void {
    if (action.type === "install") {
      if (confirmInstall) {
        setConfirmOpen(true);
        return;
      }
      onInstall?.();
    } else if (action.type === "check") {
      onCheck?.();
    } else {
      onDefer?.();
    }
  }

  const notice = view.notice;

  return (
    <>
      <div className="poodle-update-status" data-state={view.state} data-tone={view.tone}>
        <div className="poodle-update-status__head">
          {view.busy ? <Spinner variant="ring" size="sm" tone="muted" /> : null}
          <span className="poodle-update-status__title">{view.title}</span>
        </div>

        {view.body ? <p className="poodle-update-status__body">{view.body}</p> : null}

        {view.progress ? (
          <Progress
            value={view.progress.fraction === null ? null : Math.round(view.progress.fraction * 100)}
            indeterminate={view.progress.fraction === null}
            ariaLabel="Download progress"
            size="sm"
          />
        ) : null}

        {notice ? (
          <div className="poodle-update-status__notice" data-tone={notice.tone} role="status">
            <span>{notice.message}</span>
            {notice.retry ? (
              <Button
                variant="ghost"
                size="xs"
                density={resolvedDensity}
                disabled={pending}
                onClick={() => dispatch(notice.retry!)}
              >
                {retryLabel}
              </Button>
            ) : null}
          </div>
        ) : null}

        {view.actions.length > 0 ? (
          <div className="poodle-update-status__actions">
            {view.actions.map((action) => (
              <Button
                key={action.type}
                variant={action.type === "install" ? "primary" : "secondary"}
                size="sm"
                density={resolvedDensity}
                disabled={pending}
                onClick={() => dispatch(action)}
              >
                {actionLabels[action.type]}
              </Button>
            ))}
          </div>
        ) : null}
      </div>

      <AlertDialog
        open={confirmOpen}
        tone="warning"
        title="Install and restart?"
        description="The application will close and restart to finish the update."
        confirmLabel={installLabel}
        cancelLabel="Cancel"
        onConfirm={() => {
          setConfirmOpen(false);
          onInstall?.();
        }}
        onCancel={() => setConfirmOpen(false)}
        onOpenChange={(next) => {
          if (!next) setConfirmOpen(false);
        }}
        size={resolvedSize}
        density={resolvedDensity}
      />
    </>
  );
}
