import { useEffect, useRef, useState } from "react";
import { normalizeToast, reconcileToastTimers } from "@poodle/headless";

import "@poodle/styles/toast-host.css";

import { ToastStack } from "./ToastStack";
import type {
  ControlDensity,
  ControlSize,
  SemanticControlSizeRole,
  ToastHostPlacement,
  ToastHostStore,
  ToastHostStoreItem,
  ToastItem,
} from "./types";

export interface ToastHostProps {
  store: ToastHostStore;
  autoDismissMs?: number;
  stickyTones?: ToastItem["tone"][];
  placement?: ToastHostPlacement;
  ariaLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onAction?: ((id: string) => void) | null;
  onDismiss?: ((id: string) => void) | null;
}

export function ToastHost({
  store,
  autoDismissMs = 6000,
  stickyTones = ["danger"],
  placement = "bottom-end",
  ariaLabel = "Notifications",
  size = null,
  sizeRole = "chrome",
  density = null,
  onAction = null,
  onDismiss = null,
}: ToastHostProps) {
  const [items, setItems] = useState<ToastItem[]>([]);
  const timers = useRef(new Map<string, ReturnType<typeof setTimeout>>());

  const configRef = useRef({ autoDismissMs, stickyTones, store });
  configRef.current = { autoDismissMs, stickyTones, store };

  useEffect(() => {
    function clearTimer(id: string) {
      const timer = timers.current.get(id);
      if (!timer) return;
      clearTimeout(timer);
      timers.current.delete(id);
    }

    const unsubscribe = store.toasts.subscribe((next: ToastHostStoreItem[]) => {
      setItems(next.map(normalizeToast));
      const { autoDismissMs: dismissMs, stickyTones: sticky, store: s } = configRef.current;
      const plan = reconcileToastTimers([...timers.current.keys()], next, {
        autoDismissMs: dismissMs,
        stickyTones: sticky.filter((tone): tone is NonNullable<typeof tone> => tone != null),
      });
      for (const id of plan.clear) clearTimer(id);
      for (const id of plan.start) {
        const timer = setTimeout(() => {
          s.dismiss(id);
          timers.current.delete(id);
        }, dismissMs);
        timers.current.set(id, timer);
      }
    });

    const activeTimers = timers.current;
    return () => {
      unsubscribe();
      for (const timer of activeTimers.values()) clearTimeout(timer);
      activeTimers.clear();
    };
  }, [store]);

  function handleDismiss(id: string) {
    const timer = timers.current.get(id);
    if (timer) {
      clearTimeout(timer);
      timers.current.delete(id);
    }
    store.dismiss(id);
    onDismiss?.(id);
  }

  if (items.length === 0) return null;

  return (
    <div className="poodle-toast-host" data-placement={placement}>
      <ToastStack
        items={items}
        ariaLabel={ariaLabel}
        size={size}
        sizeRole={sizeRole}
        density={density}
        onDismiss={handleDismiss}
        onAction={(id) => onAction?.(id)}
      />
    </div>
  );
}
