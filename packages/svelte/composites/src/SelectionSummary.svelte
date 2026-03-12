<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let items: Array<{ id: string; label: string }> = [];
  export let selectionMode: "single" | "multiple" = "multiple";
  export let maxVisibleItems = 4;

  const dispatch = createEventDispatcher<{
    remove: { id: string };
    clear: void;
  }>();

  $: visibleItems = items.slice(0, maxVisibleItems);
  $: overflowCount = Math.max(0, items.length - visibleItems.length);
</script>

<section class="selection-summary" aria-label="Current selection">
  <div class="selection-summary__header">
    <strong>
      {#if selectionMode === "single"}
        {items.length === 0 ? "No selection" : "Selected item"}
      {:else}
        {items.length} selected
      {/if}
    </strong>
    {#if items.length > 0}
      <button type="button" on:click={() => dispatch("clear")}>Clear</button>
    {/if}
  </div>

  {#if items.length > 0}
    <div class="selection-summary__chips">
      {#each visibleItems as item}
        <button
          type="button"
          class="selection-summary__chip"
          on:click={() => dispatch("remove", { id: item.id })}
          aria-label={`Remove ${item.label}`}
        >
          {item.label}
          <span aria-hidden="true">×</span>
        </button>
      {/each}
      {#if overflowCount > 0}
        <span class="selection-summary__overflow">+{overflowCount} more</span>
      {/if}
    </div>
  {/if}
</section>

<style>
  .selection-summary {
    display: grid;
    gap: var(--pug-space-stack-sm);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 1px solid transparent;
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 94%, transparent);
  }

  .selection-summary__header {
    display: flex;
    justify-content: space-between;
    gap: var(--pug-space-inline-md);
    align-items: center;
  }

  .selection-summary__header button,
  .selection-summary__chip {
    border: 1px solid transparent;
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 76%, transparent);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .selection-summary__header button {
    min-height: var(--pug-size-control-height);
    padding: 0 var(--pug-space-control-x);
  }

  .selection-summary__chips {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
  }

  .selection-summary__chip {
    display: inline-flex;
    gap: 8px;
    align-items: center;
    min-height: 32px;
    padding: 0 12px;
  }

  .selection-summary__overflow {
    color: var(--pug-color-text-secondary);
    font-size: 13px;
    line-height: 32px;
    padding: 0 10px;
    border-radius: var(--pug-radius-control);
    background: color-mix(in srgb, var(--pug-color-background-surface) 58%, transparent);
  }

  :global([data-theme="light"]) .selection-summary {
    border-color: color-mix(in srgb, var(--pug-color-border-default) 14%, transparent);
    box-shadow:
      inset 0 0 0 1px color-mix(in srgb, var(--pug-color-border-subtle) 32%, transparent),
      0 6px 16px rgba(49, 66, 85, 0.03);
  }
</style>
