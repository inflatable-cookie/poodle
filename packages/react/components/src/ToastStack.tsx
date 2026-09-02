import { useEffect, useId, useLayoutEffect, useRef, useState, type FocusEvent } from "react";
import {
  applyToastExitInert,
  cancelToastPresence,
  dropToastVisual,
  moveToastFocus,
  moveToastFocusFromRemovedAction,
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

const EMPTY_TOAST_ITEMS: ToastItem[] = [];

export function ToastStack({
  items = EMPTY_TOAST_ITEMS,
  ariaLabel = "Notifications",
  size = null,
  sizeRole = "chrome",
  density = null,
  onDismiss,
  onAction,
}: ToastStackProps) {
  const uiPresentation = useUiPresentation();
  const policy = useMotionPolicy();
  const stackId = useId();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const stackRef = useRef<HTMLUListElement | null>(null);
  const enteredFrom = useRef<Element | null>(null);
  const focusedActionId = useRef<string | null>(null);
  const previousItems = useRef(items);
  const mounted = useRef(true);
  const initialPass = useRef(false);
  const [visuals, setVisuals] = useState<ToastVisual[]>(() =>
    nextToastVisuals([], items.map((item) => item.id), true),
  );
  const [retained, setRetained] = useState(() => new Map(items.map((item) => [item.id, item])));

  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
    };
  }, []);

  useLayoutEffect(() => {
    const stack = stackRef.current;
    const previous = previousItems.current;
    previousItems.current = items;
    if (!stack) {
      return;
    }
    for (const prev of previous) {
      const next = items.find((item) => item.id === prev.id);
      if (!prev.actionLabel || next?.actionLabel) {
        continue;
      }
      if (focusedActionId.current !== prev.id) {
        continue;
      }
      const toastEl = [...stack.querySelectorAll<HTMLElement>(".poodle-toast")].find(
        (node) => node.dataset.toastId === prev.id,
      );
      if (toastEl) {
        moveToastFocusFromRemovedAction(stack, toastEl, enteredFrom.current, toastEl);
      }
      focusedActionId.current = null;
    }
  }, [items]);

  useEffect(() => {
    const liveIds = items.map((item) => item.id);
    setRetained((previous) => {
      let changed = false;
      const next = new Map(previous);
      for (const item of items) {
        if (next.get(item.id) !== item) {
          changed = true;
          next.set(item.id, item);
        }
      }
      return changed ? next : previous;
    });
    setVisuals((previous) => {
      const next = nextToastVisuals(previous, liveIds, initialPass.current);
      initialPass.current = false;
      if (
        next.length === previous.length &&
        next.every((visual, index) => visual.id === previous[index].id && visual.phase === previous[index].phase)
      ) {
        return previous;
      }
      return next;
    });
  }, [items]);

  function handleDismiss(id: string, toastEl: HTMLElement | null, activator: EventTarget | null) {
    if (stackRef.current && toastEl) {
      moveToastFocus(stackRef.current, toastEl, enteredFrom.current, activator);
    }
    onDismiss?.(id);
  }

  function prune(id: string) {
    if (!mounted.current) {
      return;
    }
    setVisuals((current) => dropToastVisual(current, id));
    setRetained((current) => {
      const next = new Map(current);
      next.delete(id);
      return next;
    });
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
        const target = event.target as HTMLElement | null;
        const toast = target?.closest<HTMLElement>(".poodle-toast");
        focusedActionId.current = target?.closest(".poodle-toast__actions") && toast?.dataset.toastId
          ? toast.dataset.toastId
          : focusedActionId.current;
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
            stackId={stackId}
            item={item}
            visual={visual}
            policy={policy}
            resolvedSize={resolvedSize}
            resolvedDensity={resolvedDensity}
            onDismiss={handleDismiss}
            onAction={onAction}
            onEnterDone={() => {
              if (mounted.current) {
                setVisuals((current) => settleToastVisual(current, visual.id));
              }
            }}
            onExitDone={() => prune(visual.id)}
          />
        );
      })}
    </ul>
  );
}

function ToastRow({
  stackId,
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
  stackId: string;
  item: ToastItem;
  visual: ToastVisual;
  policy: ReturnType<typeof useMotionPolicy>;
  resolvedSize: ControlSize;
  resolvedDensity: ControlDensity;
  onDismiss: (id: string, toastEl: HTMLElement | null, activator: EventTarget | null) => void;
  onAction?: (id: string) => void;
  onEnterDone: () => void;
  onExitDone: () => void;
}) {
  const ref = useRef<HTMLLIElement | null>(null);
  const owner = `${stackId}:${visual.id}`;
  const exiting = visual.phase === "exit";

  useEffect(() => {
    const node = ref.current;
    if (!node) {
      return;
    }
    applyToastExitInert(node, exiting);
    if (visual.phase === "settled") {
      return () => cancelToastPresence(owner);
    }
    playToastPresence(node, {
      owner,
      phase: exiting ? "exit" : "enter",
      policy,
      initial: false,
      onComplete: (status) => {
        if (status !== "finish") {
          return;
        }
        if (exiting) {
          onExitDone();
        } else {
          onEnterDone();
        }
      },
    });
    return () => cancelToastPresence(owner);
  }, [exiting, owner, policy, visual.phase]);

  return (
    <li
      ref={ref}
      className="poodle-toast"
      data-toast-id={item.id}
      data-tone={item.tone ?? "info"}
      data-motion={visual.phase}
      data-motion-inert={exiting ? "true" : undefined}
      inert={exiting ? true : undefined}
      aria-live={exiting ? undefined : item.tone === "danger" ? "assertive" : "polite"}
      aria-atomic="true"
      aria-hidden={exiting ? true : undefined}
    >
      <button
        type="button"
        className="poodle-toast__dismiss"
        aria-label={`Dismiss ${item.title}`}
        tabIndex={exiting ? -1 : undefined}
        onClick={(event) =>
          onDismiss(item.id, event.currentTarget.closest(".poodle-toast"), event.currentTarget)
        }
      >
        <Icon name="x" />
      </button>

      <div className="poodle-toast__copy">
        <strong>{item.title}</strong>
        {item.message ? <p>{item.message}</p> : null}
      </div>

      {item.actionLabel ? (
        <div className="poodle-toast__actions">
          <Button
            variant="secondary"
            size={resolvedSize}
            density={resolvedDensity}
            onClick={() => onAction?.(item.id)}
          >
            {item.actionLabel}
          </Button>
        </div>
      ) : null}
    </li>
  );
}
