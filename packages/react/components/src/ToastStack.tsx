import { useEffect, useRef, useState, type FocusEvent } from "react";
import {
  dropToastVisual,
  moveToastFocus,
  nextToastVisuals,
  playToastPresence,
  settleToastVisual,
  type ToastVisual,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/toast-stack.css";

import { Button } from "./Button";
import { Icon } from "./Icon";
import { useMotionPolicy } from "./motion-policy";
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
  const policy = useMotionPolicy();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const stackRef = useRef<HTMLUListElement | null>(null);
  const enteredFrom = useRef<Element | null>(null);
  const initialPass = useRef(true);
  const [visuals, setVisuals] = useState<ToastVisual[]>([]);
  const [retained, setRetained] = useState(() => new Map(items.map((item) => [item.id, item])));

  useEffect(() => {
    const liveIds = items.map((item) => item.id);
    setRetained((previous) => {
      const next = new Map(previous);
      for (const item of items) {
        next.set(item.id, item);
      }
      return next;
    });
    setVisuals((previous) => nextToastVisuals(previous, liveIds, initialPass.current));
    initialPass.current = false;
  }, [items]);

  function handleDismiss(id: string, toastEl: HTMLElement | null) {
    if (stackRef.current && toastEl) {
      moveToastFocus(stackRef.current, toastEl, enteredFrom.current);
    }
    onDismiss?.(id);
  }

  return (
    <ul
      ref={stackRef}
      className="poodle-toast-stack"
      aria-label={ariaLabel}
      aria-live="polite"
      aria-atomic="false"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      onFocusCapture={(event: FocusEvent<HTMLUListElement>) => {
        if (!stackRef.current?.contains(event.relatedTarget as Node | null)) {
          enteredFrom.current = event.relatedTarget as Element | null;
        }
      }}
    >
      {visuals.map((visual) => {
        const item = retained.get(visual.id);
        if (!item) {
          return null;
        }
        return (
          <ToastRow
            key={visual.id}
            item={item}
            visual={visual}
            policy={policy}
            resolvedSize={resolvedSize}
            resolvedDensity={resolvedDensity}
            onDismiss={handleDismiss}
            onAction={onAction}
            onEnterDone={() => setVisuals((current) => settleToastVisual(current, visual.id))}
            onExitDone={() => setVisuals((current) => dropToastVisual(current, visual.id))}
          />
        );
      })}
    </ul>
  );
}

function ToastRow({
  item,
  visual,
  policy,
  resolvedSize,
  resolvedDensity,
  onDismiss,
  onAction,
  onEnterDone,
  onExitDone,
}: {
  item: ToastItem;
  visual: ToastVisual;
  policy: ReturnType<typeof useMotionPolicy>;
  resolvedSize: ControlSize;
  resolvedDensity: ControlDensity;
  onDismiss: (id: string, toastEl: HTMLElement | null) => void;
  onAction?: (id: string) => void;
  onEnterDone: () => void;
  onExitDone: () => void;
}) {
  const ref = useRef<HTMLLIElement | null>(null);

  useEffect(() => {
    if (!ref.current) {
      return;
    }
    playToastPresence(ref.current, {
      owner: visual.id,
      phase: visual.phase === "exit" ? "exit" : "enter",
      policy,
      initial: visual.phase === "settled",
    });
  }, [policy, visual.id, visual.phase]);

  return (
    <li
      ref={ref}
      className="poodle-toast"
      data-tone={item.tone ?? "info"}
      data-motion={visual.phase}
      data-motion-inert={visual.phase === "exit" ? "true" : undefined}
      aria-live={visual.phase === "exit" ? undefined : item.tone === "danger" ? "assertive" : "polite"}
      aria-atomic="true"
      aria-hidden={visual.phase === "exit" ? true : undefined}
      onAnimationEnd={() => {
        if (visual.phase === "exit") {
          onExitDone();
        } else if (visual.phase === "enter") {
          onEnterDone();
        }
      }}
    >
      <button
        type="button"
        className="poodle-toast__dismiss"
        aria-label={`Dismiss ${item.title}`}
        onClick={(event) => onDismiss(item.id, event.currentTarget.closest(".poodle-toast"))}
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
  );
}
