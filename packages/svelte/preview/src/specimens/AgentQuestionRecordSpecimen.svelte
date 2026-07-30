<script lang="ts">
  import { AgentQuestionRecord, type AgentQuestionAnswer, type AgentQuestionItem } from "@poodle/svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  const selected: AgentQuestionAnswer = { questionId: "placement", outcome: "selected", values: ["composer"], text: "" };
  const several: AgentQuestionAnswer = { questionId: "targets", outcome: "selected", values: ["svelte", "gpui"], text: "" };
  const override: AgentQuestionAnswer = { questionId: "placement", outcome: "override", values: [], text: "Neither — put it in the sidebar." };
  const declined: AgentQuestionAnswer = { questionId: "placement", outcome: "declined", values: [], text: "" };
</script>

<SpecimenLayout>
  <SpecimenGroup
    title="Selected"
    description="Every option stays. Why the agent did something is usually answered by what it did not do, and a record showing only the chosen option cannot tell you whether the choice was between three reasonable things or the only one on offer."
  >
    <AgentQuestionRecord question={placement} answer={selected} />
  </SpecimenGroup>

  <SpecimenGroup title="Several chosen">
    <AgentQuestionRecord question={targets} answer={several} />
  </SpecimenGroup>

  <SpecimenGroup title="Override" description="No option list — none was taken.">
    <AgentQuestionRecord question={placement} answer={override} />
  </SpecimenGroup>

  <SpecimenGroup title="Declined">
    <AgentQuestionRecord question={placement} answer={declined} />
  </SpecimenGroup>

  <SpecimenGroup title="Without options">
    <AgentQuestionRecord question={placement} answer={selected} showOptions={false} />
  </SpecimenGroup>

  <SpecimenGroup title="Without a header">
    <AgentQuestionRecord question={targets} answer={several} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <AgentQuestionRecord question={placement} answer={selected} {size} />
  {/snippet}

  {#snippet densities(density)}
    <AgentQuestionRecord question={placement} answer={selected} {density} />
  {/snippet}
</SpecimenLayout>
