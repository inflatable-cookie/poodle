<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import IconButton from "./IconButton.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { BulkAction, ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let selectionCount = 0;
  export let totalCount: number | null = null;
  export let actions: BulkAction[] = [];
  export let sizeRole: SemanticControlSizeRole = "control";
  export let size: ControlSize | null = null;
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    action: { id: string };
    clear: void;
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
</script>

<div class="bulk-action-bar" role="region" aria-label="Bulk actions" data-size={resolvedSize} data-density={resolvedDensity}>
  <div class="bulk-action-bar__summary">
    <strong>{selectionCount} selected</strong>
    {#if totalCount !== null}
      <span>of {totalCount} visible rows</span>
    {/if}
  </div>

  <div class="bulk-action-bar__actions">
    {#each actions as action}
      {#if action.icon}
        <IconButton
          icon={action.icon}
          ariaLabel={action.label}
          variant="secondary"
          tone={action.tone ?? "default"}
          on:click={() => dispatch("action", { id: action.id })}
        />
      {:else}
        <button
          type="button"
          class="bulk-action-bar__button"
          data-tone={action.tone ?? "default"}
          on:click={() => dispatch("action", { id: action.id })}
        >
          {action.label}
        </button>
      {/if}
    {/each}
    <IconButton
      icon="x"
      ariaLabel="Clear selection"
      variant="ghost"
      on:click={() => dispatch("clear")}
    />
  </div>
</div>

<style>
  .bulk-action-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    --poodle-recipe-bulk-fill: color-mix(in srgb, var(--poodle-color-background-panel) 93%, var(--poodle-color-text-primary));
    background: var(--poodle-recipe-bulk-fill);
    --poodle-surface: var(--poodle-recipe-bulk-fill);
  }

  .bulk-action-bar__summary {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
    align-items: baseline;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .bulk-action-bar__summary span {
    color: var(--poodle-color-text-secondary);
  }

  .bulk-action-bar__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
  }

  .bulk-action-bar__button {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
  }

  .bulk-action-bar__button[data-tone="danger"] {
    border-color: color-mix(in srgb, var(--poodle-color-status-danger) 65%, transparent);
    color: var(--poodle-color-status-danger);
  }

  .bulk-action-bar__button:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  /* Size variants */
  .bulk-action-bar[data-size="xs"] .bulk-action-bar__button {
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
    font-size: 0.75rem;
  }

  .bulk-action-bar[data-size="xs"] .bulk-action-bar__summary {
    font-size: 0.75rem;
  }

  .bulk-action-bar[data-size="sm"] .bulk-action-bar__button {
    min-height: calc(var(--poodle-size-control-height) - 0.375rem);
  }

  .bulk-action-bar[data-size="lg"] .bulk-action-bar__button {
    min-height: calc(var(--poodle-size-control-height) + 0.375rem);
    font-size: 0.9375rem;
  }

  .bulk-action-bar[data-size="xl"] .bulk-action-bar__button {
    min-height: calc(var(--poodle-size-control-height) + 0.5rem);
    font-size: 1rem;
  }

  /* Density variants */
  .bulk-action-bar[data-density="compact"] { padding: 0.375rem 0.5rem; gap: var(--poodle-space-inline-sm); }
  .bulk-action-bar[data-density="comfortable"] { padding: 0.625rem 1rem; gap: var(--poodle-space-inline-lg); }
</style>
