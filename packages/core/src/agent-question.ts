/**
 * Agent question machinery.
 * Contract: docs/contracts/components/agent-question.md.
 *
 * Pure logic for the question an agent asks mid-turn: answer resolution,
 * selection toggling, batch advance, and the record an answered question leaves
 * in the transcript. Rendering and key binding stay adapter-side.
 *
 * The component renders inside `AgentChatInput`'s field because the free-text
 * override *is* the composer's editor — see the contract's §1. That is a
 * rendering decision; nothing in this module depends on it.
 *
 * The Rust mirror is `poodle-headless::agent_question`. Both are driven by
 * `packages/contracts/headless/vectors/agent-question.json`, so answer
 * resolution cannot drift between the web targets and the natives.
 */

export interface AgentQuestionOption {
  value: string;
  label: string;
  description?: string;
}

export interface AgentQuestionItem {
  id: string;
  /** Short label shown as an eyebrow above the prompt. */
  header?: string;
  prompt: string;
  options: AgentQuestionOption[];
  /**
   * Opt-in, per question.
   *
   * Single-select is the default because it can resolve on one click: the first
   * click is also the last. With several answers a click cannot be told from a
   * first-of-several, so multi-select always needs an explicit submit.
   */
  allowMultiple?: boolean;
}

export type AgentQuestionOutcome = "selected" | "override" | "declined";

export interface AgentQuestionAnswer {
  questionId: string;
  outcome: AgentQuestionOutcome;
  /** Chosen option values. Empty for `override` and `declined`. */
  values: string[];
  /** The free-text answer. Empty unless `outcome` is `override`. */
  text: string;
}

/** True when a click on an option should also resolve the question. */
export function submitsOnSelect(question: AgentQuestionItem | null): boolean {
  return Boolean(question) && question?.allowMultiple !== true;
}

/**
 * Apply a click on an option.
 *
 * Single-select replaces; multi-select toggles. Selecting anything clears a
 * pending override, because an answer is either "these options" or "this
 * instead" — carrying both would leave the agent to guess which the reader
 * meant.
 */
export function toggleQuestionSelection(
  question: AgentQuestionItem | null,
  selections: readonly string[],
  value: string,
): string[] {
  if (!question) return [];

  if (question.allowMultiple !== true) {
    return [value];
  }

  return selections.includes(value)
    ? selections.filter((entry) => entry !== value)
    : [...selections, value];
}

/**
 * Resolve the answer for a question, given what is selected and what is typed.
 *
 * Override wins. Typing clears the selection rather than the editor locking
 * once an option is picked: locking traps the reader, who ticks a box, finds
 * that none of the options fit, and then has to untick before they can type.
 *
 * Clearing the text does not restore the cleared selections. There is only ever
 * one answer in flight, and what is on screen is exactly what will be sent.
 */
export function resolveQuestionAnswer(
  question: AgentQuestionItem | null,
  selections: readonly string[],
  override: string,
): AgentQuestionAnswer | null {
  if (!question) return null;

  const text = override.trim();
  if (text.length > 0) {
    return { questionId: question.id, outcome: "override", values: [], text };
  }

  if (selections.length > 0) {
    // Answer order follows the question's options, not the order they were
    // clicked: the agent reads a set, and click order is not information.
    const ordered = question.options
      .map((option) => option.value)
      .filter((value) => selections.includes(value));

    return { questionId: question.id, outcome: "selected", values: ordered, text: "" };
  }

  return null;
}

/**
 * Dismissal resolves the question as declined.
 *
 * A turn cannot finish with an open question, so dismissal has to send
 * something. It is a resolution, not an escape from one — abandoning the turn
 * is the composer's stop action.
 */
export function declineQuestion(question: AgentQuestionItem): AgentQuestionAnswer {
  return { questionId: question.id, outcome: "declined", values: [], text: "" };
}

/** True when an answer can be submitted from this state. */
export function canSubmitQuestion(
  question: AgentQuestionItem | null,
  selections: readonly string[],
  override: string,
): boolean {
  return resolveQuestionAnswer(question, selections, override) !== null;
}

// ── Batch progress ──

export type QuestionProgressState = "answered" | "current" | "pending";

export interface QuestionProgress {
  /** One entry per question, in order. */
  states: QuestionProgressState[];
  /** 1-based position, for the "2 of 4" label. */
  current: number;
  total: number;
}

/**
 * Position in a batch.
 *
 * Reported, never navigable: going back would mean changing an answer the agent
 * already has. The dots are a picture of the label, not controls.
 */
export function questionProgress(
  questions: readonly AgentQuestionItem[],
  activeIndex: number,
): QuestionProgress {
  const total = questions.length;
  const clamped = Math.min(Math.max(activeIndex, 0), Math.max(0, total - 1));

  return {
    states: questions.map((_, index) =>
      index < clamped ? "answered" : index === clamped ? "current" : "pending",
    ),
    current: total === 0 ? 0 : clamped + 1,
    total,
  };
}

/** Progress is chrome for a batch; a lone question does not need a picture of "1 of 1". */
export function showsQuestionProgress(questions: readonly AgentQuestionItem[]): boolean {
  return questions.length > 1;
}

/** The next index after resolving one, clamped at the end of the batch. */
export function nextQuestionIndex(
  questions: readonly AgentQuestionItem[],
  activeIndex: number,
): number {
  return Math.min(activeIndex + 1, questions.length);
}

/** True once every question has been resolved and the turn may continue. */
export function questionBatchComplete(
  questions: readonly AgentQuestionItem[],
  activeIndex: number,
): boolean {
  return activeIndex >= questions.length;
}

// ── The record an answered question leaves ──

export interface AnsweredQuestion {
  question: AgentQuestionItem;
  answer: AgentQuestionAnswer;
}

/**
 * What the transcript shows for an answered question.
 *
 * Read-only by construction: the pending question lives in the composer, and
 * this is the record it leaves behind, so there is never a second input on
 * screen. It carries the options as well as the answer, because "why did it
 * pick that" needs the alternatives that were on offer.
 */
export function answeredQuestionSummary(record: AnsweredQuestion): string {
  const { question, answer } = record;

  if (answer.outcome === "declined") return "Declined";
  if (answer.outcome === "override") return answer.text;

  const labels = answer.values.map(
    (value) => question.options.find((option) => option.value === value)?.label ?? value,
  );

  return labels.join(", ");
}

/** True when an option was the one chosen, for rendering the record. */
export function isChosenOption(record: AnsweredQuestion, value: string): boolean {
  return record.answer.outcome === "selected" && record.answer.values.includes(value);
}
