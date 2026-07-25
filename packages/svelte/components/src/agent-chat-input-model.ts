// AgentChatInput pure model. Renderer-neutral logic: submit gating, key-gesture
// intent and context-budget maths. This is the canonical TypeScript source; the
// React shell mirrors it and the Rust spec (`poodle-specs::agent_chat_input`)
// re-implements the same semantics.

/** What a key gesture means in the composer (contract §4). */
export type SubmitIntent = "submit" | "newline" | "stop" | "none";

/** The subset of a keyboard event the composer reads. */
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

/** Map a key gesture to composer intent. Enter during IME composition is never
 * a submit — that Enter belongs to the IME. */
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

/** Whether the action button is enabled. While busy it always is — stopping must
 * never be blocked by an empty editor. */
export function canSubmit({ disabled, isBusy, value, allowEmptySubmit }: SubmitGate): boolean {
  if (disabled) return false;
  return isBusy || value.trim().length > 0 || allowEmptySubmit;
}

/** Context usage as a percentage of the limit, or null when there is no limit. */
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
