import "@inflatable-cookie/poodle-core/styles/model-connection.css";

import { useId, useLayoutEffect, useRef, useState, type ReactNode } from "react";

import {
  disclosureTransition,
  modelConnectionReadinessTone,
} from "@inflatable-cookie/poodle-core";

import { Icon } from "./Icon";
import { IconButton } from "./IconButton";
import { StatusIndicator } from "./StatusIndicator";
import { Switch } from "./Switch";

export interface ModelConnectionCardIdProps {
  id: string;
}

export interface ModelConnectionCardActionsProps {
  id: string;
  isOpen: boolean;
}

export interface ModelConnectionCardDetailsProps {
  id: string;
  isEnabled: boolean;
}

export interface ModelConnectionCardProps {
  id: string;
  title: string;
  providerLabel: string;
  routeLabel?: string | null;
  version?: string | null;
  accessSummary?: string | null;
  readiness?:
    | "ready"
    | "checking"
    | "attention"
    | "unavailable"
    | "unknown"
    | "error";
  readinessLabel?: string;
  open?: boolean | undefined;
  defaultOpen?: boolean;
  isEnabled?: boolean;
  isEnableDisabled?: boolean;
  isDisabled?: boolean;
  ariaLabel?: string | null;
  onOpenChange?: ((open: boolean) => void) | null;
  onEnabledChange?: ((enabled: boolean) => void) | null;
  leading?: (props: ModelConnectionCardIdProps) => ReactNode;
  badges?: (props: ModelConnectionCardIdProps) => ReactNode;
  closedAccessory?: (props: ModelConnectionCardIdProps) => ReactNode;
  actions?: (props: ModelConnectionCardActionsProps) => ReactNode;
  details?: (props: ModelConnectionCardDetailsProps) => ReactNode;
}

export function ModelConnectionCard({
  id,
  title,
  providerLabel,
  routeLabel = null,
  version = null,
  accessSummary = null,
  readiness = "unknown",
  readinessLabel = "Status unknown",
  open = undefined,
  defaultOpen = false,
  isEnabled = true,
  isEnableDisabled = false,
  isDisabled = false,
  ariaLabel = null,
  onOpenChange = null,
  onEnabledChange = null,
  leading,
  badges,
  closedAccessory,
  actions,
  details,
}: ModelConnectionCardProps) {
  const [uncontrolledOpen, setUncontrolledOpen] = useState(defaultOpen);
  const summaryRef = useRef<HTMLDivElement | null>(null);
  const detailsHadFocusRef = useRef(false);
  const detailsId = `${useId()}-details`;

  const isOpenControlled = open !== undefined;
  const isOpen = isOpenControlled ? open === true : uncontrolledOpen;
  const meta = [routeLabel, version].filter((part): part is string => Boolean(part)).join(" · ");
  const statusLabel = readiness === "ready" && accessSummary ? accessSummary : readinessLabel;

  function disclosureControl(): HTMLElement | null {
    return summaryRef.current?.querySelector<HTMLElement>("[data-model-connection-disclosure] button");
  }

  function toggleOpen(): void {
    const result = disclosureTransition(
      { open: isOpen, disabled: isDisabled },
      { type: "TOGGLE" },
    );

    for (const effect of result.effects) {
      if (effect.type !== "emitOpenChange") continue;
      if (!isOpenControlled) setUncontrolledOpen(effect.open);
      onOpenChange?.(effect.open);
      if (!effect.open) {
        queueMicrotask(() => disclosureControl()?.focus());
      }
    }
  }

  useLayoutEffect(() => {
    if (!isOpen && detailsHadFocusRef.current) {
      detailsHadFocusRef.current = false;
      disclosureControl()?.focus();
    }
  }, [isOpen]);

  return (
    <section
      className="poodle-model-connection-card"
      data-open={isOpen ? "true" : "false"}
      data-readiness={readiness}
      data-enabled={isEnabled ? "true" : "false"}
      aria-label={ariaLabel ?? title}
    >
      <div ref={summaryRef} className="poodle-model-connection-card__summary">
        <div className="poodle-model-connection-card__identity">
          <div className="poodle-model-connection-card__title-row">
            <span className="poodle-model-connection-card__leading" aria-hidden="true">
              {leading ? leading({ id }) : <Icon name="package" />}
            </span>
            <h3 className="poodle-model-connection-card__title">{title}</h3>
            {badges ? badges({ id }) : null}
          </div>
          {meta ? <p className="poodle-model-connection-card__meta">{meta}</p> : null}
          <span className="poodle-sr-only">{providerLabel}</span>
        </div>

        <div className="poodle-model-connection-card__controls">
          <StatusIndicator
            status={modelConnectionReadinessTone(readiness)}
            label={statusLabel}
          />
          {!isOpen && closedAccessory ? closedAccessory({ id }) : null}
          {actions ? actions({ id, isOpen }) : null}
          <span data-model-connection-disclosure>
            <IconButton
              icon={isOpen ? "chevron-up" : "chevron-down"}
              ariaLabel={isOpen ? `Collapse ${title}` : `Expand ${title}`}
              variant="ghost"
              sizeRole="chrome"
              disabled={isDisabled}
              expanded={isOpen}
              controls={detailsId}
              onClick={() => toggleOpen()}
            />
          </span>
          <Switch
            checked={isEnabled}
            disabled={isDisabled || isEnableDisabled}
            ariaLabel={`Enable ${title}`}
            onCheckedChange={(next) => {
              if (isDisabled || isEnableDisabled) return;
              onEnabledChange?.(next);
            }}
          />
        </div>
      </div>

      {isOpen ? (
        <div
          className="poodle-model-connection-card__details"
          id={detailsId}
          role="region"
          aria-label={`${title} details`}
          onFocusCapture={() => {
            detailsHadFocusRef.current = true;
          }}
          onBlurCapture={(event) => {
            const next = event.relatedTarget;
            if (!(next instanceof Node) || !event.currentTarget.contains(next)) {
              detailsHadFocusRef.current = false;
            }
          }}
        >
          {details ? details({ id, isEnabled }) : null}
        </div>
      ) : null}
    </section>
  );
}
