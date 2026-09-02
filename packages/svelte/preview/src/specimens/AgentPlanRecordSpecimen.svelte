<script lang="ts">
  import { AgentPlanRecord } from "@inflatable-cookie/poodle-svelte/markdown";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

  const plan = [
    "## Proposed plan",
    "",
    "1. Add the `AgentPlan` surface to the composer",
    "2. Wire the decision callbacks through the host",
    "3. Append the settled record to the transcript",
    "",
    "Then run the gates.",
  ].join("\n");

  const longPlan = [
    "## Rollout",
    "",
    ...Array.from({ length: 12 }, (_, index) => `${index + 1}. Step ${index + 1} of the rollout, with enough prose to matter`),
  ].join("\n");
</script>

<SpecimenLayout>
  <SpecimenGroup label="Accepted" description="Collapsed by default: the badge and a one-line summary, with the full plan behind the disclosure.">
    <AgentPlanRecord {plan} status="accepted" />
  </SpecimenGroup>

  <SpecimenGroup label="Revised" description="The badge wording can be overridden when the host has more to say than the status.">
    <AgentPlanRecord {plan} status="revised" decisionLabel="Revised with operator feedback" />
  </SpecimenGroup>

  <SpecimenGroup label="Dismissed" description="A non-event reads as one: the badge drops to meta strength.">
    <AgentPlanRecord {plan} status="dismissed" />
  </SpecimenGroup>

  <SpecimenGroup label="Expanded" description="The full plan, rendered as markdown by the same path as the turn's prose.">
    <AgentPlanRecord {plan} status="accepted" expanded />
  </SpecimenGroup>

  <SpecimenGroup label="With provenance" description="The host formats and supplies the timestamp; the record renders what it is given.">
    <AgentPlanRecord {plan} status="accepted" decidedAt="2026-08-07 10:00" />
  </SpecimenGroup>

  <SpecimenGroup label="Long plan" description="The summary truncates to its budget, ellipsis included.">
    <AgentPlanRecord plan={longPlan} status="accepted" />
  </SpecimenGroup>

  {#snippet sizes(size)}
    <AgentPlanRecord {plan} status="accepted" {size} />
  {/snippet}

  {#snippet densities(density)}
    <AgentPlanRecord {plan} status="accepted" {density} />
  {/snippet}
</SpecimenLayout>
