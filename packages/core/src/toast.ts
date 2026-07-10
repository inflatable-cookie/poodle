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

export interface ToastTimerPlan {
  /** Timer ids to cancel: their toasts left the store. */
  clear: string[];
  /** Toast ids to start auto-dismiss timers for. */
  start: string[];
}

/**
 * Reconcile running auto-dismiss timers against the next store snapshot.
 * Sticky toasts and non-positive `autoDismissMs` never get timers; existing
 * timers are preserved (a toast's clock does not restart on unrelated store
 * changes).
 */
export function reconcileToastTimers(
  runningTimerIds: readonly string[],
  next: readonly ToastHostInput[],
  options: { autoDismissMs: number; stickyTones: readonly ToastTone[] },
): ToastTimerPlan {
  const nextIds = new Set(next.map((toast) => toast.id));
  const running = new Set(runningTimerIds);

  const clear = runningTimerIds.filter((id) => !nextIds.has(id));

  const start =
    options.autoDismissMs <= 0
      ? []
      : next
          .filter((toast) => !isToastSticky(toast, options.stickyTones) && !running.has(toast.id))
          .map((toast) => toast.id);

  return { clear, start };
}
