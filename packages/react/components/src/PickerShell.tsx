import "@inflatable-cookie/poodle-core/styles/picker-shell.css";

import type { ReactNode } from "react";

import { Spinner } from "./Spinner";
import type { BrowseState, PickerVariant } from "./types";

export interface PickerShellProps {
  title: string;
  description?: string | null;
  variant?: PickerVariant;
  state?: BrowseState;
  ariaLabel?: string | null;
  resultCount?: number | null;
  selectionCount?: number;
  stateTitle?: string | null;
  stateMessage?: string | null;
  statusText?: string | null;
  statusId?: string | null;
  toolbar?: ReactNode;
  selection?: ReactNode;
  stateContent?: ReactNode;
  footer?: ReactNode;
  children?: ReactNode;
}

export function PickerShell({
  title,
  description = null,
  variant = "inline",
  state = "ready",
  ariaLabel = null,
  resultCount = null,
  selectionCount = 0,
  stateTitle = null,
  stateMessage = null,
  statusText = null,
  statusId = null,
  toolbar,
  selection,
  stateContent,
  footer,
  children,
}: PickerShellProps) {
  return (
    <section className="poodle-picker-shell" data-variant={variant} data-state={state} aria-label={ariaLabel ?? undefined}>
      <div className="poodle-picker-shell__header">
        <div>
          <h3 className="poodle-picker-shell__title">{title}</h3>
          {description ? <p className="poodle-picker-shell__description">{description}</p> : null}
        </div>
        <div className="poodle-picker-shell__meta">
          {resultCount !== null ? <span>{resultCount} results</span> : null}
          <span>{selectionCount} selected</span>
        </div>
      </div>

      {toolbar ? <div className="poodle-picker-shell__toolbar">{toolbar}</div> : null}

      {selection ? <div className="poodle-picker-shell__selection">{selection}</div> : null}

      {statusText ? (
        <p
          className="poodle-picker-shell__status poodle-sr-only"
          id={statusId ?? undefined}
          role="status"
          aria-live="polite"
          aria-atomic="true"
        >
          {statusText}
        </p>
      ) : null}

      {state === "ready" ? (
        <div className="poodle-picker-shell__body">{children}</div>
      ) : (
        <div className="poodle-picker-shell__state">
          {stateContent ?? (
            <>
              {state === "loading" ? (
                <span className="poodle-picker-shell__spinner" aria-hidden="true">
                  <Spinner variant="grid" tone="accent" />
                </span>
              ) : null}
              <strong>{stateTitle ?? "Picker state"}</strong>
              {stateMessage ? <p>{stateMessage}</p> : null}
            </>
          )}
        </div>
      )}

      {footer ? <div className="poodle-picker-shell__footer">{footer}</div> : null}
    </section>
  );
}
