<script lang="ts">
  import AgentChatInput from "../src/AgentChatInput.svelte";
  import AgentPlan from "../src/AgentPlan.svelte";
  import type { AgentPlanStatus } from "@inflatable-cookie/poodle-core";

  interface Props {
    plan: string;
    status?: AgentPlanStatus;
    onAccept?: () => void;
    onRevise?: () => void;
    onDismiss?: () => void;
    onSubmit?: (value: string) => void;
  }

  // The markdown is aliased off `plan`: inside `{#snippet plan()}` the
  // identifier `plan` is the snippet itself, so a same-named binding would be
  // shadowed and the snippet function would reach `AgentPlan.plan` — a
  // non-string that crashes the markdown lexer.
  let {
    plan: planMarkdown,
    status = "pending",
    onAccept = undefined,
    onRevise = undefined,
    onDismiss = undefined,
    onSubmit = undefined,
  }: Props = $props();
</script>

<!-- The composition the contract prescribes: AgentPlan mounted through the
     composer's `plan` snippet while `status="reviewing-plan"`. -->
<AgentChatInput status="reviewing-plan" {onSubmit}>
  {#snippet plan()}
    <AgentPlan plan={planMarkdown} {status} {onAccept} {onRevise} {onDismiss} />
  {/snippet}
</AgentChatInput>
