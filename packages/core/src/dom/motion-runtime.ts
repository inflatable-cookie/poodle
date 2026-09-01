/**
 * Web motion runtime: WAAPI/rAF handles keyed by semantic owner.
 *
 * Architecture: docs/architecture/012-semantic-motion-policy.md
 * Laws: packages/core/src/motion-policy.ts
 */

import {
  MOTION_DURATION_MS,
  activateMotion,
  abortMotion,
  completeMotion,
  createMotionTrace,
  motionKey,
  sampleMotion,
  setMotionTracePolicy,
  type MotionDecision,
  type MotionIntent,
  type MotionPolicy,
  type MotionTrace,
} from "../motion-policy";

export interface WebMotionHandle {
  cancel(): void;
}

const handles = new Map<string, WebMotionHandle>();
const animations = new Map<string, Animation>();
const traces = new Map<string, MotionTrace>();

export function liveWebMotionCount(): number {
  return handles.size;
}

export function cancelWebMotion(key?: string): void {
  if (key) {
    const handle = handles.get(key);
    const animation = animations.get(key);
    const trace = traces.get(key);
    handle?.cancel();
    if (handles.get(key) === handle) {
      handles.delete(key);
    }
    if (traces.get(key) === trace) {
      traces.delete(key);
    }
    if (animations.get(key) === animation) {
      animations.delete(key);
    }
    return;
  }
  const currentHandles = [...handles.entries()];
  const currentAnimations = [...animations.entries()];
  const currentTraces = [...traces.entries()];
  for (const [motionKey, handle] of currentHandles) {
    handle.cancel();
    if (handles.get(motionKey) === handle) {
      handles.delete(motionKey);
    }
  }
  for (const [motionKey, trace] of currentTraces) {
    if (traces.get(motionKey) === trace) {
      traces.delete(motionKey);
    }
  }
  for (const [motionKey, animation] of currentAnimations) {
    if (animations.get(motionKey) === animation) {
      animations.delete(motionKey);
    }
  }
}

function track(key: string, handle: WebMotionHandle): void {
  const previous = handles.get(key);
  handles.set(key, handle);
  previous?.cancel();
}

function retainedTrace(key: string, policy: MotionPolicy): MotionTrace {
  const existing = traces.get(key);
  if (!existing) {
    const created = createMotionTrace(policy);
    traces.set(key, created);
    return created;
  }
  if (existing.policy !== policy) {
    setMotionTracePolicy(existing, policy);
  }
  return existing;
}

export function afterFirstFrame(onReady: () => void): () => void {
  let cancelled = false;
  const raf = globalThis.requestAnimationFrame;
  if (typeof raf !== "function") {
    return () => {
      cancelled = true;
    };
  }
  const frame = raf(() => {
    if (!cancelled) onReady();
  });
  return () => {
    cancelled = true;
    const cancel = globalThis.cancelAnimationFrame;
    if (typeof cancel === "function") {
      cancel(frame);
    }
  };
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

function registerAnimation(
  key: string,
  animation: Animation,
  onComplete?: (status: "finish" | "cancel") => void,
): void {
  let settled = false;
  let handle: WebMotionHandle;
  const finish = (status: "finish" | "cancel") => {
    if (settled) {
      return;
    }
    settled = true;
    if (handles.get(key) === handle) {
      handles.delete(key);
    }
    if (animations.get(key) === animation) {
      animations.delete(key);
    }
    onComplete?.(status);
  };
  const cancel = () => {
    if (settled) {
      return;
    }
    animation.cancel();
    finish("cancel");
  };
  handle = { cancel };
  animations.set(key, animation);
  animation.finished.then(
    () => finish("finish"),
    () => finish("cancel"),
  );
  track(key, handle);
}

function liveClippedHeight(
  element: HTMLElement,
  key: string,
  trace: MotionTrace,
): number | null {
  const animation = animations.get(key);
  const naturalHeight = element.scrollHeight;
  if (!animation || naturalHeight <= 0) {
    return null;
  }

  const clock = trace.clocks.find((entry) => entry.key === key);
  const sampleAxis = (axis: number) => {
    if (!clock) {
      return;
    }
    const span = clock.axisTo - clock.axisFrom;
    const progress = span === 0 ? 0 : (axis - clock.axisFrom) / span;
    sampleMotion(trace, key, progress);
  };

  try {
    const computedHeight = Number.parseFloat(globalThis.getComputedStyle(element).height);
    if (Number.isFinite(computedHeight) && computedHeight >= 0) {
      sampleAxis(Math.min(1, Math.max(0, computedHeight / naturalHeight)));
      return computedHeight;
    }
  } catch {
    // Headless/non-DOM hosts may not expose computed style; use WAAPI time below.
  }

  const currentTime = animation.currentTime;
  if (typeof currentTime === "number") {
    const timing = animation.effect?.getComputedTiming();
    const duration = timing?.duration;
    if (typeof duration === "number" && duration > 0) {
      const progress = Math.min(1, Math.max(0, currentTime / duration));
      sampleMotion(trace, key, progress);
      if (clock) {
        const axis = clock.axisFrom + (clock.axisTo - clock.axisFrom) * progress;
        return naturalHeight * axis;
      }
    }
  }
  return null;
}

export function playWebAnimation(
  trace: MotionTrace,
  intent: MotionIntent,
  element: Element,
  keyframes: Keyframe[],
  easing = "ease-out",
  onComplete?: (status: "finish" | "cancel") => void,
): MotionDecision {
  const decision = activateMotion(trace, intent);
  if (!decision.schedule) {
    if (decision.interruption !== "inert") {
      const existing = handles.get(decision.key);
      if (existing) {
        existing.cancel();
        if (handles.get(decision.key) === existing) {
          handles.delete(decision.key);
        }
      }
    }
    return decision;
  }
  if (typeof element.animate !== "function") {
    const existing = handles.get(decision.key);
    existing?.cancel();
    if (handles.get(decision.key) === existing) {
      handles.delete(decision.key);
    }
    abortMotion(trace, decision.key);
    return { ...decision, schedule: false, liveClock: false, paintEndpoint: true };
  }
  const animation = element.animate(keyframes, {
    duration: decision.durationMs,
    easing,
    fill: "forwards",
    iterations: intent.loop ? Infinity : 1,
  });
  registerAnimation(decision.key, animation, (status) => {
    if (status === "finish") {
      completeMotion(trace, decision.key);
    }
    onComplete?.(status);
  });
  return decision;
}

export function playClippedHeight(
  element: HTMLElement,
  options: {
    owner: string;
    open: boolean;
    policy: MotionPolicy;
    initial: boolean;
    durationMs?: number;
    onComplete?: (status: "finish" | "cancel") => void;
  },
): MotionDecision {
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
  const key = motionKey(intent.owner, intent.role, intent.channel);
  const trace = retainedTrace(key, options.policy);
  const from = liveClippedHeight(element, key, trace) ?? (options.open ? 0 : element.scrollHeight);
  const to = options.open ? element.scrollHeight : 0;
  element.style.overflow = "hidden";
  const settle = () => {
    element.style.height = options.open ? "" : "0px";
    element.style.overflow = options.open ? "" : "hidden";
  };
  const decision = playWebAnimation(
    trace,
    intent,
    element,
    [{ height: `${from}px` }, { height: `${to}px` }],
    "ease-out",
    (status) => {
      if (status === "finish") {
        settle();
      }
      options.onComplete?.(status);
    },
  );
  if (!decision.schedule) {
    if (decision.interruption !== "inert") {
      settle();
      options.onComplete?.("finish");
    }
    return decision;
  }
  const handle = handles.get(key);
  const originalCancel = handle?.cancel;
  if (handle && originalCancel) {
    handle.cancel = () => {
      originalCancel();
      settle();
    };
  }
  return decision;
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

export function applyToastExitInert(element: HTMLElement, inert: boolean): void {
  element.inert = inert;
  if (inert) {
    element.setAttribute("inert", "");
  } else {
    element.removeAttribute("inert");
  }
  for (const control of element.querySelectorAll<HTMLElement>(
    "button, a[href], input, select, textarea, [tabindex]",
  )) {
    if (inert) {
      control.setAttribute("tabindex", "-1");
    } else if (control.getAttribute("tabindex") === "-1") {
      control.removeAttribute("tabindex");
    }
  }
}

function toastOwnsFocus(dismissed: HTMLElement, activator: EventTarget | Node | null): boolean {
  const active = document.activeElement;
  return (
    (activator instanceof Node && dismissed.contains(activator)) ||
    (active instanceof Node && dismissed.contains(active))
  );
}

function holdToastFocus(node: HTMLElement): void {
  node.focus();
  const restore = () => {
    if (node.isConnected && document.activeElement !== node) {
      node.focus();
    }
  };
  queueMicrotask(restore);
  globalThis.requestAnimationFrame?.(restore);
}

export function moveToastFocus(
  stack: HTMLElement,
  dismissed: HTMLElement,
  enteredFrom: Element | null,
  activator: EventTarget | Node | null = document.activeElement,
): void {
  if (!toastOwnsFocus(dismissed, activator)) {
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
    holdToastFocus(focusable);
    return;
  }
  if (enteredFrom instanceof HTMLElement && document.contains(enteredFrom)) {
    holdToastFocus(enteredFrom);
  }
}

export function cancelToastPresence(owner: string): void {
  cancelWebMotion(motionKey(owner, "toast-enter", "item"));
  cancelWebMotion(motionKey(owner, "toast-exit", "item"));
}

export function playToastPresence(
  element: HTMLElement,
  options: {
    owner: string;
    phase: "enter" | "exit";
    policy: MotionPolicy;
    initial: boolean;
    durationMs?: number;
    onComplete?: (status: "finish" | "cancel") => void;
  },
): MotionDecision {
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
  const siblingRole = options.phase === "enter" ? "toast-exit" : "toast-enter";
  cancelWebMotion(motionKey(options.owner, siblingRole, "item"));
  const key = motionKey(intent.owner, intent.role, intent.channel);
  const trace = retainedTrace(key, options.policy);
  const keyframes: Keyframe[] =
    options.policy === "full"
      ? [
          { opacity: opacity[0], transform: `translateY(${translate[0]})` },
          { opacity: opacity[1], transform: `translateY(${translate[1]})` },
        ]
      : [{ opacity: opacity[0] }, { opacity: opacity[1] }];
  const decision = playWebAnimation(
    trace,
    intent,
    element,
    keyframes,
    "ease-out",
    options.onComplete,
  );
  if (!decision.schedule && decision.interruption !== "inert") {
    options.onComplete?.("finish");
  }
  return decision;
}
