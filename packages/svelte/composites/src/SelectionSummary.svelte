<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Icon } from "@flint/svelte-primitives";

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
          <span aria-hidden="true"><Icon name="x" size="sm" /></span>
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
    gap: var(--flint-space-stack-sm);
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid transparent;
    border-radius: var(--flint-radius-surface);
    background: color-mix(in srgb, var(--flint-color-background-panel) 94%, transparent);
    font-size: var(--flint-typography-label-size, 0.75rem);
  }

  .selection-summary__header {
    display: flex;
    justify-content: space-between;
    gap: var(--flint-space-inline-md);
    align-items: center;
  }

  .selection-summary__header button,
  .selection-summary__chip {
    border: 0.0625rem solid transparent;
    border-radius: var(--flint-radius-control);
    background: color-mix(in srgb, var(--flint-color-background-surface) 76%, transparent);
    color: var(--flint-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .selection-summary__header button {
    min-height: var(--flint-size-control-height);
    padding: 0 var(--flint-space-control-x);
  }

  .selection-summary__chips {
    display: flex;
    flex-wrap: wrap;
    gap: var(--flint-space-inline-sm);
  }

  .selection-summary__chip {
    display: inline-flex;
    gap: var(--flint-space-inline-md);
    align-items: center;
    min-height: calc(var(--flint-size-control-height) - 0.25rem);
    padding: 0 0.75rem;
  }

  .selection-summary__overflow {
    color: var(--flint-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 2rem;
    padding: 0 0.625rem;
    border-radius: var(--flint-radius-control);
    background: color-mix(in srgb, var(--flint-color-background-surface) 58%, transparent);
  }

  :global([data-theme="light"]) .selection-summary {
    border-color: color-mix(in srgb, var(--flint-color-border-default) 14%, transparent);
    box-shadow:
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--flint-color-border-subtle) 32%, transparent),
      0 0.375rem 1rem rgba(49, 66, 85, 0.03);
  }
</style>
