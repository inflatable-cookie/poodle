import {
  AgentQuestionRecord,
  type AgentQuestionAnswer,
  type AgentQuestionItem,
} from "@inflatable-cookie/poodle-react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const placement: AgentQuestionItem = {
  id: "placement",
  header: "Placement",
  prompt: "When the agent needs an answer mid-turn, where should the question surface appear?",
  options: [
    { value: "inline", label: "Inline in the transcript" },
    { value: "composer", label: "Anchored above the composer" },
    { value: "modal", label: "Modal dialog" },
  ],
};

const targets: AgentQuestionItem = {
  id: "targets",
  prompt: "Which targets should this ship to?",
  allowMultiple: true,
  options: [
    { value: "svelte", label: "Svelte" },
    { value: "react", label: "React" },
    { value: "gpui", label: "GPUI" },
  ],
};

const selected: AgentQuestionAnswer = {
  questionId: "placement",
  outcome: "selected",
  values: ["composer"],
  text: "",
};
const several: AgentQuestionAnswer = {
  questionId: "targets",
  outcome: "selected",
  values: ["svelte", "gpui"],
  text: "",
};
const override: AgentQuestionAnswer = {
  questionId: "placement",
  outcome: "override",
  values: [],
  text: "Neither — put it in the sidebar.",
};
const declined: AgentQuestionAnswer = {
  questionId: "placement",
  outcome: "declined",
  values: [],
  text: "",
};

export function AgentQuestionRecordSpecimen() {
  return (
    <SpecimenLayout
      sizes={(size) => <AgentQuestionRecord question={placement} answer={selected} size={size} />}
      densities={(density) => (
        <AgentQuestionRecord question={placement} answer={selected} density={density} />
      )}
    >
      <SpecimenGroup
        label="Selected"
        description="Every option stays. Why the agent did something is usually answered by what it did not do, and a record showing only the chosen option cannot tell you whether the choice was between three reasonable things or the only one on offer."
      >
        <AgentQuestionRecord question={placement} answer={selected} />
      </SpecimenGroup>

      <SpecimenGroup label="Several chosen">
        <AgentQuestionRecord question={targets} answer={several} />
      </SpecimenGroup>

      <SpecimenGroup label="Override" description="No option list — none was taken.">
        <AgentQuestionRecord question={placement} answer={override} />
      </SpecimenGroup>

      <SpecimenGroup label="Declined">
        <AgentQuestionRecord question={placement} answer={declined} />
      </SpecimenGroup>

      <SpecimenGroup label="Without options">
        <AgentQuestionRecord question={placement} answer={selected} showOptions={false} />
      </SpecimenGroup>

      <SpecimenGroup label="Without a header">
        <AgentQuestionRecord question={targets} answer={several} />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
