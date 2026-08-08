import "@inflatable-cookie/poodle-core/styles/detail-shell.css";

import type { ReactNode } from "react";

import { Spinner } from "./Spinner";
import type { BrowseState } from "./types";

export interface DetailShellProps {
  title?: string | null;
  scrollMode?: "shell" | "body";
  state?: Exclude<BrowseState, "no-results">;
  ariaLabel?: string | null;
  stateTitle?: string | null;
  stateMessage?: string | null;
  header?: ReactNode;
  stateContent?: ReactNode;
  children?: ReactNode;
}

export function DetailShell({
  title = null,
  scrollMode = "body",
  state = "ready",
  ariaLabel = null,
  stateTitle = null,
  stateMessage = null,
  header,
  stateContent,
  children,
}: DetailShellProps) {
  return (
    <section className="poodle-detail-shell" data-scroll-mode={scrollMode} aria-label={ariaLabel ?? undefined}>
      {header || title ? (
        <div className="poodle-detail-shell__header">{header ?? (title ? <h2>{title}</h2> : null)}</div>
      ) : null}

      {state === "ready" ? (
        <div className="poodle-detail-shell__body">{children}</div>
      ) : (
        <div className="poodle-detail-shell__state" data-state={state}>
          {stateContent ?? (
            <>
              {state === "loading" ? (
                <span className="poodle-detail-shell__spinner" aria-hidden="true">
                  <Spinner variant="grid" tone="accent" />
                </span>
              ) : null}
              <strong>{stateTitle ?? "Detail state"}</strong>
              {stateMessage ? <p>{stateMessage}</p> : null}
            </>
          )}
        </div>
      )}
    </section>
  );
}
