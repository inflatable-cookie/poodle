<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import IconButton from "./IconButton.svelte";
  import type { BulkAction } from "./types";

  export let selectionCount = 0;
  export let totalCount: number | null = null;
  export let actions: BulkAction[] = [];

  const dispatch = createEventDispatcher<{
    action: { id: string };
    clear: void;
  }>();
</script>

<div class="bulk-action-bar" role="region" aria-label="Bulk actions">
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
    gap: var(--flint-space-inline-md);
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid var(--flint-color-border-subtle);
    border-radius: var(--flint-radius-surface);
    --flint-recipe-bulk-fill: color-mix(in srgb, var(--flint-color-background-panel) 93%, var(--flint-color-text-primary));
    background: var(--flint-recipe-bulk-fill);
    --flint-surface: var(--flint-recipe-bulk-fill);
  }

  .bulk-action-bar__summary {
    display: flex;
    flex-wrap: wrap;
    gap: var(--flint-space-inline-sm);
    align-items: baseline;
    color: var(--flint-color-text-primary);
    font-family: var(--flint-typography-body-family);
    font-size: var(--flint-typography-body-size);
    line-height: var(--flint-typography-body-lineHeight);
  }

  .bulk-action-bar__summary span {
    color: var(--flint-color-text-secondary);
  }

  .bulk-action-bar__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--flint-space-inline-sm);
  }

  .bulk-action-bar__button {
    min-height: var(--flint-size-control-height);
    padding: 0 var(--flint-space-control-x);
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: var(--flint-radius-control);
    background: var(--flint-color-background-surface);
    color: var(--flint-color-text-primary);
    cursor: pointer;
  }

  .bulk-action-bar__button[data-tone="danger"] {
    border-color: color-mix(in srgb, var(--flint-color-status-danger) 65%, transparent);
    color: var(--flint-color-status-danger);
  }

  .bulk-action-bar__button:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>
