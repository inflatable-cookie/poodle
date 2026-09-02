<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/agent-plan.css";

  import { canDecidePlan, planStatusLabel, type AgentPlanStatus } from "@inflatable-cookie/poodle-core";

  import AgentMessage from "./AgentMessage.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    /** Raw markdown of the proposed plan. Rendered, never pre-rendered by the host. */
    plan?: string;
    status?: AgentPlanStatus;
    /** Dismiss is a first-class decision for a plan, so the control renders by default. */
    dismissible?: boolean;
    dismissLabel?: string;
    acceptLabel?: string;
    reviseLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onAccept?: (() => void) | undefined;
    onRevise?: (() => void) | undefined;
    onDismiss?: (() => void) | undefined;
  }

  let {
    plan = "",
    status = "pending",
    dismissible = true,
    dismissLabel = "Dismiss plan",
    acceptLabel = "Accept plan",
    reviseLabel = "Revise",
    size = null,
    sizeRole = "control",
    density = null,
    onAccept = undefined,
    onRevise = undefined,
    onDismiss = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  /**
   * Controls render only while the plan waits on the operator. A settled status
   * shows the badge instead, covering the moment between the decision and the
   * host swapping in the record — settled display proper is AgentPlanRecord.
   */
  const isPending = $derived(canDecidePlan(status));
</script>

<div
  class="poodle-agent-plan"
  data-status={status}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <!-- The plan is markdown, rendered by the same path as the turn's prose. -->
  <div class="poodle-agent-plan__body">
    <AgentMessage markdown={plan} role="assistant" size={resolvedSize} density={resolvedDensity} />
  </div>

  {#if isPending}
    <div class="poodle-agent-plan__actions">
      <button
        type="button"
        class="poodle-agent-plan__action"
        data-variant="primary"
        onclick={() => onAccept?.()}
      >
        {acceptLabel}
      </button>
      <!-- Revise owns no text input: the host focuses the composer, where the
           feedback is typed as an ordinary message. -->
      <button
        type="button"
        class="poodle-agent-plan__action"
        data-variant="secondary"
        onclick={() => onRevise?.()}
      >
        {reviseLabel}
      </button>
      {#if dismissible}
        <button
          type="button"
          class="poodle-agent-plan__action"
          data-variant="ghost"
          onclick={() => onDismiss?.()}
        >
          {dismissLabel}
        </button>
      {/if}
    </div>
  {:else}
    <span class="poodle-agent-plan__badge" data-status={status}>{planStatusLabel(status)}</span>
  {/if}
</div>
