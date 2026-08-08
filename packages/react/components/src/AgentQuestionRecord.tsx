import "@inflatable-cookie/poodle-styles/agent-question-record.css";

import {
  answeredQuestionSummary,
  isChosenOption,
  type AgentQuestionAnswer,
  type AgentQuestionItem,
} from "@inflatable-cookie/poodle-headless";

import { Eyebrow } from "./Eyebrow";
import { Icon } from "./Icon";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface AgentQuestionRecordProps {
  question: AgentQuestionItem;
  answer: AgentQuestionAnswer;
  showOptions?: boolean;
  declinedLabel?: string;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
}

export function AgentQuestionRecord({
  question,
  answer,
  showOptions = true,
  declinedLabel = "Declined",
  size = null,
  sizeRole = "control",
  density = null,
}: AgentQuestionRecordProps) {
  const presentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(presentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? presentation.density;
  const glyphSize = resolveSupportingVisualSize(resolvedSize);

  const record = { question, answer };
  const showsOptions = showOptions && answer.outcome === "selected";
  const summary = answeredQuestionSummary(record);

  return (
    <div
      className="poodle-agent-question-record"
      data-outcome={answer.outcome}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      {question.header ? <Eyebrow>{question.header}</Eyebrow> : null}

      <p className="poodle-agent-question-record__prompt">{question.prompt}</p>

      {showsOptions ? (
        <ul className="poodle-agent-question-record__options">
          {question.options.map((option) => {
            const chosen = isChosenOption(record, option.value);
            return (
              // The tick alone is not the signal: the chosen option says so in
              // its accessible name too.
              <li
                key={option.value}
                className="poodle-agent-question-record__option"
                data-chosen={String(chosen)}
                aria-label={chosen ? `chosen: ${option.label}` : option.label}
              >
                <span className="poodle-agent-question-record__option-mark" aria-hidden="true">
                  <Icon name="check" size={glyphSize} />
                </span>
                <span className="poodle-agent-question-record__option-label">{option.label}</span>
              </li>
            );
          })}
        </ul>
      ) : (
        <p className="poodle-agent-question-record__answer">
          {answer.outcome === "declined" ? declinedLabel : summary}
        </p>
      )}
    </div>
  );
}
