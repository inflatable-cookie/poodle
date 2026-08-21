import { useEffect, useReducer, useState } from "react";

import "@inflatable-cookie/poodle-core/styles/update-center.css";
import {
  updateDownloadLabel,
  type Channel,
  type UpdateAheadOfChannel,
  type UpdateAvailabilityProjection,
  type UpdateControllerStatus,
  type UpdateDeferral,
  type UpdatePresence,
  type UpdateProgressProjection,
  type UpdateRejectionCode,
} from "@inflatable-cookie/poodle-core";

import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { Popover } from "./Popover";
import { UpdateStatus } from "./UpdateStatus";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole } from "./types";

export interface UpdateCenterProps {
  presence: UpdatePresence;
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
  open?: boolean | null;
  defaultOpen?: boolean;
  placement?: OverlayPlacement;
  title?: string;
  ariaLabel?: string | null;
  triggerLabel?: string | null;
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
  onOpenChange?: ((open: boolean) => void) | null;
}

export function UpdateCenter({
  presence,
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
  open = null,
  defaultOpen = false,
  placement = "bottom-end",
  title = "Updates",
  ariaLabel = null,
  triggerLabel = null,
  size = null,
  sizeRole = "chrome",
  density = null,
  installLabel = "Install and restart",
  deferLabel = "Later",
  checkLabel = "Check for updates",
  retryLabel = "Try again",
  confirmInstall = true,
  onCheck = null,
  onInstall = null,
  onDefer = null,
  onOpenChange = null,
}: UpdateCenterProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  // Bumped by `observe` so the component re-renders when the authority notifies.
  const [, force] = useReducer((x: number) => x + 1, 0);

  useEffect(() => {
    if (!observe) return;
    return observe(() => force());
  }, [observe]);

  const isOpen = open === null ? uncontrolledOpen : open;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedTriggerLabel = triggerLabel ?? title;

  const isDownloading = progress?.state === "downloading";
  const downloadFraction = progress?.state === "downloading" ? progress.fraction : null;
  const downloadingLabel = isDownloading ? updateDownloadLabel(downloadFraction) : resolvedTriggerLabel;

  function handleOpenChange(next: boolean): void {
    if (open === null) setUncontrolledOpen(next);
    onOpenChange?.(next);
  }

  if (presence === "hidden") {
    return null;
  }

  return (
    <div className="poodle-update-center">
      <Popover
        open={isOpen}
        placement={placement}
        initialFocus="content"
        triggerIsInteractive
        ariaLabel={ariaLabel ?? title}
        surfaceMinWidth="min(16rem, calc(100vw - 2rem))"
        surfaceMaxWidth="min(24rem, calc(100vw - 2rem))"
        onOpenChange={handleOpenChange}
        trigger={(state) => (
          <span className="poodle-update-center__trigger" data-presence={presence}>
            <IconButton
              icon="download"
              ariaLabel={downloadingLabel}
              tooltip={title}
              variant="ghost"
              size={resolvedSize}
              density={resolvedDensity}
              expanded={state.expanded}
              controls={state.controls}
              disabled={state.disabled}
            >
              {isDownloading ? (
                <span
                  className="poodle-update-center__ring"
                  data-indeterminate={downloadFraction === null}
                >
                  <svg viewBox="0 0 24 24" width="24" height="24" fill="none" aria-hidden="true">
                    <circle className="poodle-update-center__ring-track" cx="12" cy="12" r="9" />
                    {downloadFraction === null ? (
                      <circle className="poodle-update-center__ring-fill" cx="12" cy="12" r="9" />
                    ) : (
                      <circle
                        className="poodle-update-center__ring-fill"
                        cx="12"
                        cy="12"
                        r="9"
                        strokeDasharray={2 * Math.PI * 9}
                        strokeDashoffset={2 * Math.PI * 9 * (1 - downloadFraction)}
                      />
                    )}
                  </svg>
                </span>
              ) : (
                <Icon icon="download" size={resolvedSize} />
              )}
            </IconButton>
            {presence === "attention" ? (
              <span className="poodle-update-center__indicator" aria-hidden="true" />
            ) : null}
          </span>
        )}
      >
        <section className="poodle-update-center__surface" aria-label={ariaLabel ?? title}>
          <header className="poodle-update-center__header">
            <h2>{title}</h2>
          </header>

          <div className="poodle-update-center__body">
            <UpdateStatus
              status={status}
              availability={availability}
              progress={progress}
              channel={channel}
              installedVersion={installedVersion}
              deferral={deferral}
              lastRejection={lastRejection}
              aheadOfChannel={aheadOfChannel}
              pending={pending}
              observe={observe}
              installLabel={installLabel}
              deferLabel={deferLabel}
              checkLabel={checkLabel}
              retryLabel={retryLabel}
              confirmInstall={confirmInstall}
              onCheck={onCheck}
              onInstall={onInstall}
              onDefer={onDefer}
            />
          </div>
        </section>
      </Popover>
    </div>
  );
}
