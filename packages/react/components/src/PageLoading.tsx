import "@inflatable-cookie/poodle-core/styles/page-loading.css";

import { Progress } from "./Progress";
import { Spinner } from "./Spinner";

export type PageLoadingPresentation = "overlay" | "inline";

export interface PageLoadingProps {
  visible?: boolean;
  value?: number | null;
  max?: number;
  message?: string | null;
  canCancel?: boolean;
  ariaLabel?: string | null;
  presentation?: PageLoadingPresentation;
  onCancel?: (() => void) | undefined;
}

export function PageLoading({
  visible = true,
  value = null,
  max = 100,
  message = null,
  canCancel = false,
  ariaLabel = null,
  presentation = "overlay",
  onCancel = undefined,
}: PageLoadingProps) {
  const isIndeterminate = value === null;
  const isOverlay = presentation === "overlay";

  if (!visible) {
    return null;
  }

  return (
    <div
      className="poodle-page-loading"
      data-presentation={presentation}
      role="status"
      aria-label={ariaLabel ?? "Loading"}
      aria-live="polite"
    >
      {isOverlay ? <div className="poodle-page-loading__backdrop" aria-hidden="true" /> : null}
      <div className="poodle-page-loading__card">
        <Spinner className="poodle-page-loading__spinner" variant="ring" sizeRole="prominent" tone="accent" />

        {!isIndeterminate ? (
          <div className="poodle-page-loading__progress">
            <Progress value={value} max={max} ariaLabel={message ?? "Loading progress"} />
          </div>
        ) : null}

        {message ? <p className="poodle-page-loading__message">{message}</p> : null}

        {canCancel ? (
          <button type="button" className="poodle-page-loading__cancel" onClick={() => onCancel?.()}>
            Cancel
          </button>
        ) : null}
      </div>
    </div>
  );
}
