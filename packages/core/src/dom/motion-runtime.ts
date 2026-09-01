/**
 * Web motion runtime: WAAPI/rAF handles keyed by semantic owner.
 *
 * Architecture: docs/architecture/012-semantic-motion-policy.md
 * Laws: packages/core/src/motion-policy.ts
 */

import {
  MOTION_DURATION_MS,
  activateMotion,
  createMotionTrace,
  type MotionIntent,
  type MotionPolicy,
  type MotionTrace,
} from "../motion-policy";

export interface WebMotionHandle {
  cancel(): void;
}

const handles = new Map<string, WebMotionHandle>();

export function liveWebMotionCount(): number {
  return handles.size;
}

export function cancelWebMotion(key?: string): void {
  if (key) {
    handles.get(key)?.cancel();
    handles.delete(key);
    return;
  }
  for (const handle of handles.values()) {
    handle.cancel();
  }
  handles.clear();
}

function track(key: string, handle: WebMotionHandle): void {
  handles.get(key)?.cancel();
  handles.set(key, handle);
}

export function afterFirstFrame(onReady: () => void): () => void {
  const frame = requestAnimationFrame(() => {
    onReady();
  });
  return () => cancelAnimationFrame(frame);
}

/** True only in full policy after the first committed frame. */
export function bindMotionReady(
  policy: MotionPolicy,
  enabled: boolean,
  onReady: (ready: boolean) => void,
): () => void {
  onReady(false);
  if (!enabled || policy !== "full") {
    return () => {};
  }
  return afterFirstFrame(() => onReady(true));
}

function registerAnimation(key: string, animation: Animation): void {
  const cancel = () => {
    animation.cancel();
    handles.delete(key);
  };
  animation.finished.then(() => {
    if (handles.get(key)?.cancel === cancel) {
      handles.delete(key);
    }
  }).catch(() => {
    handles.delete(key);
  });
  track(key, { cancel });
}

export function playWebAnimation(
  trace: MotionTrace,
  intent: MotionIntent,
  element: Element,
  keyframes: Keyframe[],
  easing = "ease-out",
): boolean {
  const decision = activateMotion(trace, intent);
  if (!decision.schedule) {
    cancelWebMotion(decision.key);
    return false;
  }
  if (typeof element.animate !== "function") {
    return false;
  }
  const animation = element.animate(keyframes, {
    duration: decision.durationMs,
    easing,
    fill: "forwards",
    iterations: intent.loop ? Infinity : 1,
  });
  registerAnimation(decision.key, animation);
  return true;
}

export function playClippedHeight(
  element: HTMLElement,
  options: {
    owner: string;
    open: boolean;
    policy: MotionPolicy;
    initial: boolean;
    durationMs?: number;
  },
): boolean {
  const trace = createMotionTrace(options.policy);
  const durationMs = options.durationMs ?? MOTION_DURATION_MS.standard;
  const target = options.open ? "open" : "closed";
  const intent: MotionIntent = {
    owner: options.owner,
    role: "disclosure-height",
    channel: "panel",
    target,
    properties: ["height"],
    durationMs,
    initial: options.initial,
    reversible: true,
  };
  const from = options.open ? 0 : element.scrollHeight;
  const to = options.open ? element.scrollHeight : 0;
  element.style.overflow = "hidden";
  const scheduled = playWebAnimation(
    trace,
    intent,
    element,
    [{ height: `${from}px` }, { height: `${to}px` }],
    "ease-out",
  );
  if (!scheduled) {
    element.style.height = options.open ? "" : "0px";
    element.style.overflow = options.open ? "" : "hidden";
    return false;
  }
  const key = `${options.owner}\u001fdisclosure-height\u001fpanel`;
  const handle = handles.get(key);
  const originalCancel = handle?.cancel;
  if (handle && originalCancel) {
    handle.cancel = () => {
      originalCancel();
      if (options.open) {
        element.style.height = "";
        element.style.overflow = "";
      } else {
        element.style.height = "0px";
        element.style.overflow = "hidden";
      }
    };
  }
  return true;
}

export function tabIndicatorBox(
  list: HTMLElement,
  selected: HTMLElement | null,
  orientation: "horizontal" | "vertical",
): { left: number; top: number; width: number; height: number } | null {
  if (!selected) {
    return null;
  }
  const listBox = list.getBoundingClientRect();
  const tabBox = selected.getBoundingClientRect();
  if (orientation === "vertical") {
    return {
      left: tabBox.left - listBox.left + tabBox.width - 2,
      top: tabBox.top - listBox.top,
      width: 2,
      height: tabBox.height,
    };
  }
  return {
    left: tabBox.left - listBox.left,
    top: tabBox.top - listBox.top + tabBox.height - 2,
    width: tabBox.width,
    height: 2,
  };
}

export type ToastVisualPhase = "enter" | "settled" | "exit";

export interface ToastVisual {
  id: string;
  phase: ToastVisualPhase;
}

export function nextToastVisuals(
  previous: ToastVisual[],
  liveIds: string[],
  initial: boolean,
): ToastVisual[] {
  if (initial) {
    return liveIds.map((id) => ({ id, phase: "settled" as const }));
  }
  const live = new Set(liveIds);
  const previousById = new Map(previous.map((item) => [item.id, item]));
  const next: ToastVisual[] = liveIds.map((id) => {
    const prior = previousById.get(id);
    if (!prior || prior.phase === "exit") {
      return { id, phase: "enter" as const };
    }
    return { id, phase: prior.phase };
  });
  for (const prior of previous) {
    if (!live.has(prior.id)) {
      next.push({ id: prior.id, phase: "exit" });
    }
  }
  return next;
}

export function dropToastVisual(visuals: ToastVisual[], id: string): ToastVisual[] {
  return visuals.filter((item) => item.id !== id);
}

export function settleToastVisual(visuals: ToastVisual[], id: string): ToastVisual[] {
  return visuals.map((item) =>
    item.id === id && item.phase === "enter" ? { ...item, phase: "settled" as const } : item,
  );
}

export function moveToastFocus(
  stack: HTMLElement,
  dismissed: HTMLElement,
  enteredFrom: Element | null,
): void {
  if (!dismissed.contains(document.activeElement)) {
    return;
  }
  const toasts = [...stack.querySelectorAll<HTMLElement>(".poodle-toast")];
  const remaining = toasts.filter(
    (toast) => toast !== dismissed && toast.getAttribute("data-motion") !== "exit",
  );
  const dismissedIndex = toasts.indexOf(dismissed);
  const next = remaining.find((toast) => toasts.indexOf(toast) > dismissedIndex);
  const previous = [...remaining].reverse().find((toast) => toasts.indexOf(toast) < dismissedIndex);
  const target = next ?? previous;
  const focusable = target?.querySelector<HTMLElement>(
    "button, [href], input, select, textarea, [tabindex]:not([tabindex='-1'])",
  );
  if (focusable) {
    focusable.focus();
    return;
  }
  if (enteredFrom instanceof HTMLElement && document.contains(enteredFrom)) {
    enteredFrom.focus();
  }
}

export function playToastPresence(
  element: HTMLElement,
  options: {
    owner: string;
    phase: "enter" | "exit";
    policy: MotionPolicy;
    initial: boolean;
    durationMs?: number;
  },
): boolean {
  const trace = createMotionTrace(options.policy);
  const durationMs = options.durationMs ?? MOTION_DURATION_MS.standard;
  const opacity = options.phase === "enter" ? [0, 1] : [1, 0];
  const translate = options.phase === "enter" ? ["0.5rem", "0"] : ["0", "0.5rem"];
  const intent: MotionIntent = {
    owner: options.owner,
    role: options.phase === "enter" ? "toast-enter" : "toast-exit",
    channel: "item",
    target: options.phase,
    properties: ["opacity", "translateY"],
    durationMs,
    initial: options.initial,
    reducedOpacity: true,
  };
  const keyframes: Keyframe[] =
    options.policy === "full"
      ? [
          { opacity: opacity[0], transform: `translateY(${translate[0]})` },
          { opacity: opacity[1], transform: `translateY(${translate[1]})` },
        ]
      : [{ opacity: opacity[0] }, { opacity: opacity[1] }];
  return playWebAnimation(trace, intent, element, keyframes, "ease-out");
}
