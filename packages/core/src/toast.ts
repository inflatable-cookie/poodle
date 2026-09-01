/**
 * Toast host behavior machinery.
 * Contract: docs/contracts/components/toast-host.md (or toast.md),
 * "Behavior Machine" section.
 *
 * Pure logic for the toast host: tone resolution, item normalization,
 * stickiness, and the auto-dismiss timer reconciliation plan. The adapter
 * owns real timers and the store; core returns which timers to clear and
 * which to start.
 */

export type ToastTone = "info" | "success" | "warning" | "danger";

export interface ToastHostInput {
  id: string;
  title?: string | null;
  message?: string | null;
  tone?: ToastTone | null;
  variant?: string | null;
  actionLabel?: string | null;
  sticky?: boolean;
}

export interface NormalizedToast {
  id: string;
  title: string;
  message: string | null;
  tone: ToastTone;
  actionLabel: string | null;
}

export function resolveToastTone(toast: ToastHostInput): ToastTone {
  if (toast.tone) {
    return toast.tone;
  }

  if (toast.variant === "error") {
    return "danger";
  }

  if (toast.variant === "warning") {
    return "warning";
  }

  if (toast.variant === "success") {
    return "success";
  }

  return "info";
}

export function normalizeToast(toast: ToastHostInput): NormalizedToast {
  const title = toast.title?.trim() || toast.message || "Notification";
  const message = toast.title?.trim() ? toast.message ?? null : null;

  return {
    id: toast.id,
    title,
    message,
    tone: resolveToastTone(toast),
    actionLabel: toast.actionLabel ?? null,
  };
}

export function isToastSticky(toast: ToastHostInput, stickyTones: readonly ToastTone[]): boolean {
  if (toast.sticky === true) {
    return true;
  }

  return stickyTones.includes(resolveToastTone(toast));
}

/**
 * One live row per `id`. First occurrence keeps order; last occurrence wins
 * copy, tone, action, and sticky fields.
 */
export function uniqueToastInputs<T extends { id: string }>(next: readonly T[]): T[] {
  const lastById = new Map<string, T>();
  const order: string[] = [];
  for (const toast of next) {
    if (!lastById.has(toast.id)) {
      order.push(toast.id);
    }
    lastById.set(toast.id, toast);
  }
  return order.map((id) => lastById.get(id)!);
}

export interface ToastTimerPlan {
  /** Timer ids to cancel: their toasts left the store or became sticky. */
  clear: string[];
  /** Toast ids to start auto-dismiss timers for. */
  start: string[];
  /** Delay used for every id in `start`; `0` when `start` is empty. */
  delayMs: number;
}

/**
 * Reconcile running auto-dismiss timers against the next store snapshot.
 *
 * Sticky rows own no clock. Become-sticky cancels a running clock.
 * Become-non-sticky starts exactly one timer using the current configured
 * `autoDismissMs` when that delay is positive. Copy, tone, or action churn
 * and `autoDismissMs` changes preserve a running non-sticky clock.
 * Non-positive `autoDismissMs` starts nothing.
 */
export function reconcileToastTimers(
  runningTimerIds: readonly string[],
  next: readonly ToastHostInput[],
  options: { autoDismissMs: number; stickyTones: readonly ToastTone[] },
): ToastTimerPlan {
  const unique = uniqueToastInputs(next);
  const byId = new Map(unique.map((toast) => [toast.id, toast]));
  const running = new Set(runningTimerIds);

  const clear = runningTimerIds.filter((id) => {
    const toast = byId.get(id);
    return !toast || isToastSticky(toast, options.stickyTones);
  });
  const clearing = new Set(clear);

  const start =
    options.autoDismissMs <= 0
      ? []
      : unique
          .filter(
            (toast) =>
              !isToastSticky(toast, options.stickyTones) &&
              !running.has(toast.id) &&
              !clearing.has(toast.id),
          )
          .map((toast) => toast.id);

  return {
    clear,
    start,
    delayMs: start.length > 0 ? options.autoDismissMs : 0,
  };
}
