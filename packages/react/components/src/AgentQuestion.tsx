import { useCallback, useEffect, useMemo } from "react";

import "@inflatable-cookie/poodle-styles/agent-question.css";

import {
  canSubmitQuestion,
  declineQuestion,
  questionProgress,
  resolveQuestionAnswer,
  showsQuestionProgress,
  submitsOnSelect,
  toggleQuestionSelection,
  type AgentQuestionAnswer,
  type AgentQuestionItem,
} from "@inflatable-cookie/poodle-headless";

import { Checkbox } from "./Checkbox";
import { Eyebrow } from "./Eyebrow";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface AgentQuestionProps {
  questions?: AgentQuestionItem[];
  activeIndex?: number;
  selections?: string[];
  /** The composer's editor text. Passed in so the answer can be resolved here. */
  override?: string;
  dismissible?: boolean;
  dismissLabel?: string;
  progressLabel?: (current: number, total: number) => string;
  showShortcuts?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onSelectionChange?: (values: string[]) => void;
  onSubmit?: (answer: AgentQuestionAnswer) => void;
  onDismiss?: (id: string) => void;
}

export function AgentQuestion({
  questions = [],
  activeIndex = 0,
  selections = [],
  override = "",
  dismissible = false,
  dismissLabel = "Skip this question",
  progressLabel = (current: number, total: number) => `${current} of ${total}`,
  showShortcuts = true,
  size = null,
  sizeRole = "control",
  density = null,
  onSelectionChange,
  onSubmit,
  onDismiss,
}: AgentQuestionProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;

  const activeQuestion = questions[activeIndex] ?? null;
  const isMultiSelect = activeQuestion?.allowMultiple === true;
  const progress = useMemo(() => questionProgress(questions, activeIndex), [questions, activeIndex]);
  const showsProgress = showsQuestionProgress(questions);
  const instanceId = activeQuestion?.id ?? "agent-question";

  /**
   * Entering override text clears the selection.
   *
   * The alternative — locking the editor once an option is picked — traps the
   * reader: tick a box, find that none of the options fit, and now you have to
   * untick before you can type. See agent-question.md §5.
   */
  useEffect(() => {
    if (override.trim().length > 0 && selections.length > 0) {
      onSelectionChange?.([]);
    }
  }, [override, selections.length, onSelectionChange]);

  const choose = useCallback(
    (value: string) => {
      if (!activeQuestion) return;

      const next = toggleQuestionSelection(activeQuestion, selections, value);
      onSelectionChange?.(next);

      // Single-select resolves on one click: the first click is also the last.
      // Multi-select cannot, because a click is indistinguishable from a
      // first-of-several.
      if (submitsOnSelect(activeQuestion)) {
        const answer = resolveQuestionAnswer(activeQuestion, next, override);
        if (answer) onSubmit?.(answer);
      }
    },
    [activeQuestion, selections, override, onSelectionChange, onSubmit],
  );

  /**
   * Digit shortcuts, ignored while focus is in a text field. Without that
   * guard, typing "1" into the override would select an option instead of
   * reaching the editor.
   */
  useEffect(() => {
    if (!activeQuestion || !showShortcuts) return;

    const onKeyDown = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      if (target && /^(INPUT|TEXTAREA)$/.test(target.tagName)) return;
      if (event.metaKey || event.ctrlKey || event.altKey) return;

      const digit = Number.parseInt(event.key, 10);
      if (!Number.isFinite(digit) || digit < 1 || digit > 9) return;

      const option = activeQuestion.options[digit - 1];
      if (!option) return;

      event.preventDefault();
      choose(option.value);
    };

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [activeQuestion, showShortcuts, choose]);

  if (!activeQuestion) return null;

  const dismiss = () => {
    onDismiss?.(activeQuestion.id);
    onSubmit?.(declineQuestion(activeQuestion));
  };

  return (
    <div
      className="poodle-agent-question"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-multi-select={String(isMultiSelect)}
    >
      {showsProgress ? (
        <div className="poodle-agent-question__progress">
          {/* Decorative: the label beside them carries the same fact in words. */}
          <span className="poodle-agent-question__progress-dots" aria-hidden="true">
            {progress.states.map((state, index) => (
              <span key={index} className="poodle-agent-question__progress-dot" data-state={state} />
            ))}
          </span>
          <span className="poodle-agent-question__progress-label">
            {progressLabel(progress.current, progress.total)}
          </span>
        </div>
      ) : null}

      {activeQuestion.header ? <Eyebrow>{activeQuestion.header}</Eyebrow> : null}

      <p className="poodle-agent-question__prompt" id={`${instanceId}-prompt`}>
        {activeQuestion.prompt}
      </p>

      <div
        className="poodle-agent-question__options"
        role={isMultiSelect ? "group" : "radiogroup"}
        aria-labelledby={`${instanceId}-prompt`}
      >
        {activeQuestion.options.map((option, index) => (
          <button
            key={option.value}
            type="button"
            className="poodle-agent-question__option"
            role={isMultiSelect ? "checkbox" : "radio"}
            aria-checked={selections.includes(option.value)}
            data-selected={String(selections.includes(option.value))}
            onClick={() => choose(option.value)}
          >
            {/* Decorative: the option itself carries the state, and announcing
                it twice is worse than announcing it once. */}
            {isMultiSelect ? (
              <span className="poodle-agent-question__option-check" aria-hidden="true">
                <Checkbox checked={selections.includes(option.value)} size={resolvedSize} />
              </span>
            ) : null}

            <span className="poodle-agent-question__option-body">
              <span className="poodle-agent-question__option-label">{option.label}</span>
              {option.description ? (
                <span className="poodle-agent-question__option-description">{option.description}</span>
              ) : null}
            </span>

            {showShortcuts && index < 9 ? (
              <kbd className="poodle-agent-question__option-shortcut" aria-hidden="true">
                {index + 1}
              </kbd>
            ) : null}
          </button>
        ))}
      </div>

      {dismissible ? (
        <button type="button" className="poodle-agent-question__dismiss" onClick={dismiss}>
          {dismissLabel}
        </button>
      ) : null}
    </div>
  );
}

/** Whether the live question could be answered from its own state alone. */
export function agentQuestionCanSubmit(
  questions: AgentQuestionItem[],
  activeIndex: number,
  selections: string[],
  override: string,
): boolean {
  return canSubmitQuestion(questions[activeIndex] ?? null, selections, override);
}
