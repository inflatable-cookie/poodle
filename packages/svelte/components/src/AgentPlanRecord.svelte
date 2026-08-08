<script lang="ts">
  import "@poodle/styles/agent-plan-record.css";

  import {
    planRecordSummary,
    planStatusLabel,
    type AgentPlanSettledStatus,
  } from "@poodle/headless";

  import AgentMessage from "./AgentMessage.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    /** Raw markdown of the plan that was decided. */
    plan: string;
    /** A settled status. The record is what a decision leaves behind, so `pending` never reaches it. */
    status: AgentPlanSettledStatus;
    /** Overrides the badge wording; defaults to the status label. */
    decisionLabel?: string;
    /** When the decision was made, formatted by the host. */
    decidedAt?: string;
    /** Character budget for the collapsed summary, ellipsis included. */
    summaryMaxLength?: number;
    expanded?: boolean;
    expandLabel?: string;
    collapseLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onToggle?: ((expanded: boolean) => void) | undefined;
  }

  let {
    plan,
    status,
    decisionLabel = undefined,
    decidedAt = undefined,
    summaryMaxLength = 160,
    expanded = $bindable<boolean>(false),
    expandLabel = "Show plan",
    collapseLabel = "Hide plan",
    size = null,
    sizeRole = "control",
    density = null,
    onToggle = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const badge = $derived(decisionLabel ?? planStatusLabel(status));
  const summary = $derived(planRecordSummary(plan, summaryMaxLength));

  function toggle(): void {
    expanded = !expanded;
    onToggle?.(expanded);
  }
</script>

<div
  class="poodle-agent-plan-record"
  data-status={status}
  data-expanded={String(expanded)}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <div class="poodle-agent-plan-record__header">
    <span class="poodle-agent-plan-record__badge" data-status={status}>{badge}</span>
    {#if decidedAt}
      <span class="poodle-agent-plan-record__meta">{decidedAt}</span>
    {/if}
  </div>

  <!-- The summary is a stand-in for exactly the content it hides, so the two
       never render together. -->
  {#if !expanded}
    <p class="poodle-agent-plan-record__summary">{summary}</p>
  {:else}
    <div class="poodle-agent-plan-record__body">
      <AgentMessage markdown={plan} role="assistant" size={resolvedSize} density={resolvedDensity} />
    </div>
  {/if}

  <button
    type="button"
    class="poodle-agent-plan-record__toggle"
    aria-expanded={expanded}
    onclick={toggle}
  >
    {expanded ? collapseLabel : expandLabel}
  </button>
</div>
