<script lang="ts">
  import { createEventDispatcher } from "svelte";

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
      <button
        type="button"
        class="bulk-action-bar__button"
        data-tone={action.tone ?? "default"}
        on:click={() => dispatch("action", { id: action.id })}
      >
        {action.label}
      </button>
    {/each}
    <button type="button" class="bulk-action-bar__button" on:click={() => dispatch("clear")}>
      Clear selection
    </button>
  </div>
</div>

<style>
  .bulk-action-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--pug-space-inline-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid var(--pug-color-border-subtle);
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-accent-base) 10%, var(--pug-color-background-panel));
  }

  .bulk-action-bar__summary {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    align-items: baseline;
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }

  .bulk-action-bar__summary span {
    color: var(--pug-color-text-secondary);
  }

  .bulk-action-bar__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
  }

  .bulk-action-bar__button {
    min-height: var(--pug-size-control-height);
    padding: 0 var(--pug-space-control-x);
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    cursor: pointer;
  }

  .bulk-action-bar__button[data-tone="danger"] {
    border-color: color-mix(in srgb, var(--pug-color-status-danger) 65%, transparent);
    color: var(--pug-color-status-danger);
  }

  .bulk-action-bar__button:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>
