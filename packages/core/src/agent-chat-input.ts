// Renderer-neutral AgentChatInput behavior. Framework packages own editor and
// attachment rendering; this module owns submit intent, gating, and context math.

export type AgentChatAttachment = {
  id: string;
  label: string;
  /** Host-defined kind, surfaced as `data-kind` by framework components. */
  kind?: string;
  icon?: string;
  /** Replaces the compact chip with a thumbnail tile. */
  thumbnailUrl?: string;
  disabled?: boolean;
};

export type SubmitIntent = "submit" | "newline" | "stop" | "none";

/** The keyboard-event subset used by the composer. */
export type ComposerKeyEvent = {
  key: string;
  shiftKey: boolean;
  metaKey: boolean;
  ctrlKey: boolean;
  /** True while an IME composition session is open. */
  isComposing?: boolean;
};

export type SubmitIntentOptions = {
  submitOnEnter: boolean;
  isBusy: boolean;
};

/** Maps a key gesture to composer intent. Enter during IME composition belongs
 * to the IME and never submits. */
export function resolveSubmitIntent(
  event: ComposerKeyEvent,
  { submitOnEnter, isBusy }: SubmitIntentOptions,
): SubmitIntent {
  if (event.key === "Escape") return isBusy ? "stop" : "none";
  if (event.key !== "Enter") return "none";
  if (event.isComposing) return "newline";
  if (event.metaKey || event.ctrlKey) return "submit";
  if (event.shiftKey || !submitOnEnter) return "newline";
  return "submit";
}

export type SubmitGate = {
  disabled: boolean;
  isBusy: boolean;
  value: string;
  allowEmptySubmit: boolean;
};

/** Whether the action button is enabled. A busy composer allows stop unless
 * the whole control is disabled. */
export function canSubmit({ disabled, isBusy, value, allowEmptySubmit }: SubmitGate): boolean {
  if (disabled) return false;
  return isBusy || value.trim().length > 0 || allowEmptySubmit;
}

/** Context usage as a percentage, or null when no positive limit exists. */
export function contextPercentage(
  used: number | null,
  limit: number | null,
): number | null {
  if (limit === null || limit <= 0) return null;
  const clamped = Math.min(Math.max(used ?? 0, 0), limit);
  return (clamped / limit) * 100;
}

export function actionIcon(isBusy: boolean): string {
  return isBusy ? "square" : "arrow-up";
}

export function actionState(isBusy: boolean): "submit" | "stop" {
  return isBusy ? "stop" : "submit";
}
