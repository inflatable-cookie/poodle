<script lang="ts">
  import { AgentChatInput, AgentQuestion, type AgentQuestionAnswer, type AgentQuestionItem } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const placement: AgentQuestionItem = {
    id: "placement",
    header: "Placement",
    prompt: "When the agent needs an answer mid-turn, where should the question surface appear?",
    options: [
      { value: "inline", label: "Inline in the transcript", description: "A block in the conversation, in sequence with messages and tool runs." },
      { value: "composer", label: "Anchored above the composer", description: "A card pinned over the input, always visible until answered." },
      { value: "modal", label: "Modal dialog", description: "Blocks the app until answered." },
    ],
  };

  const targets: AgentQuestionItem = {
    id: "targets",
    header: "Targets",
    prompt: "Which targets should this ship to?",
    allowMultiple: true,
    options: [
      { value: "svelte", label: "Svelte" },
      { value: "react", label: "React" },
      { value: "gpui", label: "GPUI" },
      { value: "jetstream", label: "Jetstream" },
    ],
  };

  const many: AgentQuestionItem = {
    id: "many",
    header: "Priority",
    prompt: "Which remaining check should run first?",
    options: Array.from({ length: 12 }, (_, index) => ({
      value: `step-${index + 1}`,
      label: `Step ${index + 1}`,
    })),
  };

  const batch: AgentQuestionItem[] = [placement, targets, { ...placement, id: "third", header: "Scale" }, { ...targets, id: "fourth", header: "Rollout" }];

  function answerSummary(answer: AgentQuestionAnswer | null): string {
    if (!answer) return "no answer yet";
    return `answered: ${answer.outcome} ${JSON.stringify(answer.values)}${answer.text}`;
  }

  // The arrangement that matters: the question inside the composer, with the
  // editor as its override.
  let composerValue = $state("");
  let composerSelections = $state<string[]>([]);
  let lastAnswer = $state<AgentQuestionAnswer | null>(null);
  // The composer owns the submit control; the question owns resolving the
  // answer. The host joins them, which is why `submit()` is exported.
  let questionRef = $state<{ submit: () => void } | null>(null);

  let singleAnswer = $state<AgentQuestionAnswer | null>(null);
  let multiSelections = $state<string[]>([]);
  let madeSelections = $state<string[]>(["composer"]);
  let dismissedAnswer = $state<AgentQuestionAnswer | null>(null);
</script>

<SpecimenLayout>
  <SpecimenGroup
    label="Hosted by the composer"
    description="The arrangement this component exists for: the question sits above the editor, and that editor is its free-text override. Type to see the selection clear."
  >
    <AgentChatInput
      bind:value={composerValue}
      status="questioning"
      questionCanSubmit={composerSelections.length > 0}
      onSubmit={() => questionRef?.submit()}
    >
      {#snippet question()}
        <AgentQuestion
          bind:this={questionRef}
          questions={[placement]}
          bind:selections={composerSelections}
          override={composerValue}
          onSubmit={(answer) => { lastAnswer = answer; composerValue = ""; composerSelections = []; }}
        />
      {/snippet}
    </AgentChatInput>
    <p>{answerSummary(lastAnswer)}</p>
  </SpecimenGroup>

  <SpecimenGroup
    label="Choice modes"
    description="One click both selects and submits. Checkboxes appear only for multiple selection, so the mode is visible before the first click. Descriptions sit under the option label."
  >
    <AgentQuestion questions={[placement]} onSubmit={(answer) => (singleAnswer = answer)} />
    <p>{answerSummary(singleAnswer)}</p>
    <AgentQuestion questions={[targets]} bind:selections={multiSelections} />
    <p>Selected: <strong>{multiSelections.join(", ") || "none"}</strong></p>
    <AgentQuestion questions={[placement]} bind:selections={madeSelections} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Batch progress"
    description="Progress reports position. It is not navigation — going back would change an answer the agent already has."
  >
    <AgentQuestion questions={batch} activeIndex={1} />
    <AgentQuestion questions={batch} activeIndex={3} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Dismissal"
    description="Dismissal resolves as declined and advances; it does not abandon the turn. A question is not dismissible unless the host says so."
  >
    <AgentQuestion questions={[placement]} dismissible onSubmit={(answer) => (dismissedAnswer = answer)} />
    <p>{answerSummary(dismissedAnswer)}</p>
    <AgentQuestion questions={[placement]} />
  </SpecimenGroup>

  <SpecimenGroup
    label="Shortcut limits"
    description="Digit hints cover the first nine options only. The host can withhold them entirely."
  >
    <AgentQuestion questions={[many]} />
    <AgentQuestion questions={[placement]} showShortcuts={false} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <AgentQuestion questions={[placement]} {size} />
  {/snippet}

  {#snippet densities(density)}
    <AgentQuestion questions={[placement]} {density} />
  {/snippet}
</SpecimenLayout>
