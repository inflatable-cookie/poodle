import "@inflatable-cookie/poodle-styles/toast-stack.css";

import { Button } from "./Button";
import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, ToastItem } from "./types";

export interface ToastStackProps {
  items?: ToastItem[];
  ariaLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onDismiss?: (id: string) => void;
  onAction?: (id: string) => void;
}

export function ToastStack({
  items = [],
  ariaLabel = "Notifications",
  size = null,
  sizeRole = "chrome",
  density = null,
  onDismiss,
  onAction,
}: ToastStackProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <ul
      className="poodle-toast-stack"
      aria-label={ariaLabel}
      aria-live="polite"
      aria-atomic="false"
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {items.map((item) => (
        <li
          key={item.id}
          className="poodle-toast"
          data-tone={item.tone ?? "info"}
          aria-live={item.tone === "danger" ? "assertive" : "polite"}
          aria-atomic="true"
        >
          <button
            type="button"
            className="poodle-toast__dismiss"
            aria-label={`Dismiss ${item.title}`}
            onClick={() => onDismiss?.(item.id)}
          >
            <Icon name="x" />
          </button>

          <div className="poodle-toast__copy">
            <strong>{item.title}</strong>
            {item.message ? <p>{item.message}</p> : null}
          </div>

          {item.actionLabel ? (
            <div className="poodle-toast__actions">
              <Button variant="secondary" size={resolvedSize} density={resolvedDensity} onClick={() => onAction?.(item.id)}>
                {item.actionLabel}
              </Button>
            </div>
          ) : null}
        </li>
      ))}
    </ul>
  );
}
