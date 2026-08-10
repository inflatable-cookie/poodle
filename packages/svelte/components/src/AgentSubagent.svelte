<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/agent-subagent.css";

  import {
    isTerminalSubagentStatus,
    subagentStatusLabel,
    subagentStatusSpins,
    type AgentSubagentItem,
  } from "@inflatable-cookie/poodle-core";

  import Spinner from "./Spinner.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    /** The child work this group renders. */
    item?: AgentSubagentItem;
    /** Bindable disclosure state; the detail region shows while expanded. */
    expanded?: boolean;
    /** Recent activity lines shown when the group is expanded. */
    detailLines?: string[];
    /** Collapsed disclosure label. */
    expandLabel?: string;
    /** Expanded disclosure label. */
    collapseLabel?: string;
    /** The click-through action label. */
    openChildLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onToggle?: ((expanded: boolean) => void) | undefined;
    onOpenChild?: (() => void) | undefined;
  }

  let {
    item = undefined,
    expanded = $bindable<boolean>(false),
    detailLines = [],
    expandLabel = "Show activity",
    collapseLabel = "Hide activity",
    openChildLabel = "Open child work",
    size = null,
    sizeRole = "control",
    density = null,
    onToggle = undefined,
    onOpenChild = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const isTerminal = $derived(item ? isTerminalSubagentStatus(item.status) : false);
  const spins = $derived(item ? subagentStatusSpins(item.status) : false);

  /**
   * The disclosure exists only when there is something to reveal: with no
   * detail lines, an expand control would open an empty region.
   */
  const showsToggle = $derived(detailLines.length > 0);

  function toggle(): void {
    expanded = !expanded;
    onToggle?.(expanded);
  }
</script>

{#if item}
  <div
    class="poodle-agent-subagent"
    data-status={item.status}
    data-expanded={String(expanded)}
    data-size={resolvedSize}
    data-density={resolvedDensity}
  >
    <!-- The header is identity + status at a glance. -->
    <div class="poodle-agent-subagent__header">
      <span class="poodle-agent-subagent__label">{item.label}</span>
      <span class="poodle-agent-subagent__badge" data-status={item.status}>
        {subagentStatusLabel(item.status)}
      </span>
    </div>

    {#if isTerminal}
      <!-- A settled child shows what it accomplished, never a spinner. -->
      {#if item.summary}
        <p class="poodle-agent-subagent__summary">{item.summary}</p>
      {/if}
    {:else}
      <div class="poodle-agent-subagent__activity">
        <!-- Only `running` spins: a pending or waiting child is not actively
             working, and a terminal status never signals ongoing work. -->
        {#if spins}
          <Spinner variant="dots" size={resolvedSize} tone="muted" />
        {/if}
        {#if item.activityLine}
          <span class="poodle-agent-subagent__activity-line">{item.activityLine}</span>
        {/if}
      </div>
    {/if}

    <!-- Expanded: the child's recent activity lines, as plain host-supplied
         strings. Enough for v1 — richer detail is the host's to grow. -->
    {#if expanded && detailLines.length > 0}
      <ul class="poodle-agent-subagent__detail">
        {#each detailLines as line (line)}
          <li>{line}</li>
        {/each}
      </ul>
    {/if}

    <div class="poodle-agent-subagent__actions">
      {#if showsToggle}
        <button
          type="button"
          class="poodle-agent-subagent__action"
          data-kind="toggle"
          aria-expanded={expanded}
          onclick={toggle}
        >
          {expanded ? collapseLabel : expandLabel}
        </button>
      {/if}
      <!-- The click-through is the only control: it opens the child's work.
           There is no stop, cancel or steer — observation-only, because
           controlling a provider-owned child is not the transcript's job. -->
      <button
        type="button"
        class="poodle-agent-subagent__action"
        data-kind="open"
        onclick={() => onOpenChild?.()}
      >
        {openChildLabel}
      </button>
    </div>
  </div>
{/if}
