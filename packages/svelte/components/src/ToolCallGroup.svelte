<script lang="ts">
  import "@poodle/styles/tool-call-group.css";

  import { toolRunStatus } from "@poodle/headless";

  import Icon from "./Icon.svelte";
  import ToolCall from "./ToolCall.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    TranscriptToolCall,
  } from "./types";

  interface Props {
    id: string;
    calls?: TranscriptToolCall[];
    expanded?: boolean;
    expandedCalls?: string[];
    moreLabel?: (count: number) => string;
    fewerLabel?: string;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onToggle?: ((id: string) => void) | undefined;
    onCallToggle?: ((id: string) => void) | undefined;
  }

  let {
    id,
    calls = [],
    expanded = $bindable<boolean>(false),
    expandedCalls = $bindable<string[]>([]),
    moreLabel = (count: number) => `+${count} previous tool calls`,
    fewerLabel = "Show fewer tool calls",
    size = null,
    sizeRole = "control",
    density = null,
    onToggle = undefined,
    onCallToggle = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const glyphSize = $derived(resolveSupportingVisualSize(resolvedSize));

  const hiddenCount = $derived(Math.max(0, calls.length - 1));
  const showsToggle = $derived(hiddenCount > 0);
  const status = $derived(toolRunStatus({ kind: "tool-run", id, calls }));

  /**
   * Collapsed shows the run's newest call; expanded lists every call in order
   * and therefore ends on that same call. Expanding is safe to do while
   * reading: the row under the cursor stays put and the rest appears above it.
   */
  const renderedCalls = $derived(expanded ? calls : calls.slice(-1));

  /**
   * A collapsed failing run must not be announced identically to a passing one,
   * so a non-success status is carried in the toggle's name as well as its
   * colour.
   */
  const toggleName = $derived(
    expanded
      ? fewerLabel
      : `${moreLabel(hiddenCount)}${status === "error" ? ", contains a failure" : status === "running" ? ", in progress" : ""}`,
  );

  function toggle(): void {
    expanded = !expanded;
    onToggle?.(id);
  }

  function toggleCall(callId: string): void {
    expandedCalls = expandedCalls.includes(callId)
      ? expandedCalls.filter((value) => value !== callId)
      : [...expandedCalls, callId];
    onCallToggle?.(callId);
  }
</script>

<div
  class="poodle-tool-call-group"
  data-expanded={String(expanded)}
  data-status={status}
  data-count={calls.length}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <ul class="poodle-tool-call-group__list" id={`${id}-calls`}>
    {#each renderedCalls as call (call.id)}
      <!-- The list item lives here, not in ToolCall: a row that is an `<li>`
           can never be valid on its own, and the component is usable outside a
           group. -->
      <li>
      <ToolCall
        id={call.id}
        label={call.label}
        detail={call.detail ?? null}
        status={call.status}
        icon={call.icon ?? null}
        output={call.output ?? null}
        expanded={expandedCalls.includes(call.id)}
        size={resolvedSize}
        density={resolvedDensity}
        onToggle={toggleCall}
      />
      </li>
    {/each}
  </ul>

  <!-- Omitted rather than hidden when there is nothing to reveal, so a
       single-call run leaves no stray tab stop. The toggle is always the last
       child, in both states, which is what keeps focus still while expanding. -->
  {#if showsToggle}
    <button
      type="button"
      class="poodle-tool-call-group__toggle"
      aria-expanded={expanded}
      aria-controls={`${id}-calls`}
      aria-label={toggleName}
      onclick={toggle}
    >
      <span class="poodle-tool-call-group__toggle-icon"><Icon name="chevron-down" size={glyphSize} /></span>
      <span>{expanded ? fewerLabel : moreLabel(hiddenCount)}</span>
    </button>
  {/if}
</div>
