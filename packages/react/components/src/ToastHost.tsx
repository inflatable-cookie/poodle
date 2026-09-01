import { useEffect, useRef, useState } from "react";
import { normalizeToast, reconcileToastTimers, uniqueToastInputs } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/toast-host.css";

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

const DEFAULT_STICKY_TONES: NonNullable<ToastItem["tone"]>[] = ["danger"];

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

function sameToastItems(left: ToastItem[], right: ToastItem[]): boolean {
  return (
    left.length === right.length &&
    left.every(
      (item, index) =>
        item.id === right[index]?.id &&
        item.title === right[index]?.title &&
        item.message === right[index]?.message &&
        item.tone === right[index]?.tone &&
        item.actionLabel === right[index]?.actionLabel,
    )
  );
}

export function ToastHost({
  store,
  autoDismissMs = 6000,
  stickyTones = DEFAULT_STICKY_TONES,
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
  const rawItems = useRef<ToastHostStoreItem[]>([]);

  const configRef = useRef({ autoDismissMs, stickyTones, store });
  configRef.current = { autoDismissMs, stickyTones, store };

  function clearTimer(id: string) {
    const timer = timers.current.get(id);
    if (!timer) return;
    clearTimeout(timer);
    timers.current.delete(id);
  }

  function applySnapshot(next: ToastHostStoreItem[]) {
    const unique = uniqueToastInputs(next);
    const nextItems = unique.map(normalizeToast);
    setItems((current) => (sameToastItems(current, nextItems) ? current : nextItems));
    const { autoDismissMs, stickyTones: sticky, store: s } = configRef.current;
    const plan = reconcileToastTimers([...timers.current.keys()], unique, {
      autoDismissMs,
      stickyTones: sticky.filter((tone): tone is NonNullable<typeof tone> => tone != null),
    });
    for (const id of plan.clear) clearTimer(id);
    for (const id of plan.start) {
      const timer = setTimeout(() => {
        s.dismiss(id);
        timers.current.delete(id);
      }, plan.delayMs);
      timers.current.set(id, timer);
    }
  }

  useEffect(() => {
    const unsubscribe = store.toasts.subscribe((next: ToastHostStoreItem[]) => {
      rawItems.current = next;
      applySnapshot(next);
    });

    const activeTimers = timers.current;
    return () => {
      unsubscribe();
      for (const timer of activeTimers.values()) clearTimeout(timer);
      activeTimers.clear();
    };
  }, [store]);

  const stickyKey = stickyTones.filter((tone): tone is NonNullable<typeof tone> => tone != null).join(",");

  useEffect(() => {
    applySnapshot(rawItems.current);
  }, [autoDismissMs, stickyKey]);

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
