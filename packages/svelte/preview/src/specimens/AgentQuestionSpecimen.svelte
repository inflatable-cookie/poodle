<script lang="ts">
  import { AgentChatInput, AgentQuestion, type AgentQuestionAnswer, type AgentQuestionItem } from "@poodle/svelte";
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

  const batch: AgentQuestionItem[] = [placement, targets, { ...placement, id: "third", header: "Scale" }, { ...targets, id: "fourth", header: "Rollout" }];

  // The arrangement that matters: the question inside the composer, with the
  // editor as its override.
  let composerValue = $state("");
  let composerSelections = $state<string[]>([]);
  let lastAnswer = $state<AgentQuestionAnswer | null>(null);
  // The composer owns the submit control; the question owns resolving the
  // answer. The host joins them, which is why `submit()` is exported.
  let questionRef = $state<{ submit: () => void } | null>(null);

  let multiSelections = $state<string[]>([]);
  let batchIndex = $state(1);
  let batchSelections = $state<string[]>([]);
</script>

<SpecimenLayout>
  <SpecimenGroup
    title="Hosted by the composer"
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
          onSubmit={(answer) => { lastAnswer = answer; composerValue = ""; }}
        />
      {/snippet}
    </AgentChatInput>
    <p style="color: var(--poodle-color-text-secondary); font-size: 0.8125rem;">
      {lastAnswer ? `answered: ${lastAnswer.outcome} ${JSON.stringify(lastAnswer.values)}${lastAnswer.text}` : "no answer yet"}
    </p>
  </SpecimenGroup>

  <SpecimenGroup title="Single select" description="One click both selects and submits — the first click is also the last.">
    <AgentQuestion questions={[placement]} />
  </SpecimenGroup>

  <SpecimenGroup
    title="Multi select"
    description="Checkboxes appear only here, so the mode is visible before the first click. Submit is always explicit."
  >
    <AgentQuestion questions={[targets]} bind:selections={multiSelections} />
  </SpecimenGroup>

  <SpecimenGroup title="Batch" description="Progress reports position. It is not navigation — going back would change an answer the agent already has.">
    <AgentQuestion questions={batch} bind:activeIndex={batchIndex} bind:selections={batchSelections} />
  </SpecimenGroup>

  <SpecimenGroup title="Dismissible" description="Dismissal resolves as declined and advances; it does not abandon the turn.">
    <AgentQuestion questions={[placement]} dismissible />
  </SpecimenGroup>

  <SpecimenGroup title="Without shortcuts">
    <AgentQuestion questions={[placement]} showShortcuts={false} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <AgentQuestion questions={[placement]} {size} />
  {/snippet}

  {#snippet densities(density)}
    <AgentQuestion questions={[placement]} {density} />
  {/snippet}
</SpecimenLayout>
