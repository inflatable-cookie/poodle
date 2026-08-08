<script lang="ts">
  import { AgentChatInput, AgentPlan, type AgentPlanStatus } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  // Named `planMarkdown`, not `plan`: inside `{#snippet plan()}` the identifier
  // `plan` is the snippet itself, so a same-named string would be shadowed and
  // the snippet function would reach `AgentPlan.plan` — a non-string that
  // crashes the markdown lexer.
  const planMarkdown = [
    "## Proposed plan",
    "",
    "1. Add the `AgentPlan` surface to the composer",
    "2. Wire the decision callbacks through the host",
    "3. Append the settled record to the transcript",
    "",
    "Then run the gates.",
  ].join("\n");

  // The arrangement that matters: the plan inside the composer, where the
  // editor is the revise channel — feedback goes as an ordinary message.
  let composerValue = $state("");
  let decision = $state<AgentPlanStatus>("pending");
  let lastMessage = $state("");
</script>

<SpecimenLayout>
  <SpecimenGroup
    title="Hosted by the composer"
    description="A proposed plan is input, so it mounts in the composer region — not the transcript. The turn is already complete; sending a message is the revise channel."
  >
    <AgentChatInput
      bind:value={composerValue}
      status="reviewing-plan"
      onSubmit={(value) => { lastMessage = value; composerValue = ""; }}
    >
      {#snippet plan()}
        <AgentPlan
          plan={planMarkdown}
          status={decision}
          onAccept={() => (decision = "accepted")}
          onRevise={() => (decision = "revised")}
          onDismiss={() => (decision = "dismissed")}
        />
      {/snippet}
    </AgentChatInput>
    <p style="color: var(--poodle-color-text-secondary); font-size: 0.8125rem;">
      {decision !== "pending" ? `decided: ${decision}` : lastMessage ? `revision sent: ${lastMessage}` : "no decision yet"}
    </p>
  </SpecimenGroup>

  <SpecimenGroup title="Pending" description="Controls render only while the plan waits on the operator.">
    <AgentPlan plan={planMarkdown} />
  </SpecimenGroup>

  <SpecimenGroup title="Settled" description="A settled status swaps the controls for the badge — the transitional render before the host swaps in the record.">
    <AgentPlan plan={planMarkdown} status="accepted" />
  </SpecimenGroup>

  <SpecimenGroup title="Not dismissible" description="Dismiss is a first-class decision for a plan, so it renders by default; a host can still withhold it.">
    <AgentPlan plan={planMarkdown} dismissible={false} />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <AgentPlan plan={planMarkdown} {size} />
  {/snippet}

  {#snippet densities(density)}
    <AgentPlan plan={planMarkdown} {density} />
  {/snippet}
</SpecimenLayout>
