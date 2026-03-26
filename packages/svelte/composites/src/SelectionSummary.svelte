<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Icon, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";
  import type { ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  export let items: Array<{ id: string; label: string }> = [];
  export let selectionMode: "single" | "multiple" = "multiple";
  export let maxVisibleItems = 4;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";

  const dispatch = createEventDispatcher<{
    remove: { id: string };
    clear: void;
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: visibleItems = items.slice(0, maxVisibleItems);
  $: overflowCount = Math.max(0, items.length - visibleItems.length);
</script>

<section class="selection-summary" aria-label="Current selection" data-size={resolvedSize}>
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
          <span aria-hidden="true"><Icon name="x" /></span>
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
    gap: var(--poodle-space-stack-sm);
    padding: var(--poodle-space-panel-y) var(--poodle-space-panel-x);
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
    font-size: var(--poodle-typography-label-size, 0.75rem);
  }

  .selection-summary__header {
    display: flex;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    align-items: center;
  }

  .selection-summary__header button,
  .selection-summary__chip {
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .selection-summary__header button {
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
  }

  .selection-summary__chips {
    display: flex;
    flex-wrap: wrap;
    gap: var(--poodle-space-inline-sm);
  }

  .selection-summary__chip {
    display: inline-flex;
    gap: var(--poodle-space-inline-md);
    align-items: center;
    min-height: calc(var(--poodle-size-control-height) - 0.25rem);
    padding: 0 0.75rem;
  }

  .selection-summary__overflow {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 2rem;
    padding: 0 0.625rem;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 58%, transparent);
  }

  :global([data-theme="light"]) .selection-summary {
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 14%, transparent);
    box-shadow:
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent),
      0 0.375rem 1rem rgba(49, 66, 85, 0.03);
  }

  /* ── Size variants ──────────────────────────────────────────── */

  .selection-summary[data-size="xs"] {
    font-size: 0.6875rem;
  }

  .selection-summary[data-size="xs"] .selection-summary__header button {
    min-height: 1.25rem;
    padding: 0 0.375rem;
  }

  .selection-summary[data-size="xs"] .selection-summary__chip {
    min-height: 1rem;
    padding: 0 0.5rem;
    font-size: 0.6875rem;
  }

  .selection-summary[data-size="xs"] .selection-summary__overflow {
    font-size: 0.6875rem;
    line-height: 1.5rem;
    padding: 0 0.375rem;
  }

  .selection-summary[data-size="sm"] {
    font-size: 0.71875rem;
  }

  .selection-summary[data-size="sm"] .selection-summary__header button {
    min-height: 1.375rem;
  }

  .selection-summary[data-size="sm"] .selection-summary__chip {
    min-height: 1.125rem;
    font-size: 0.71875rem;
  }

  .selection-summary[data-size="sm"] .selection-summary__overflow {
    font-size: 0.71875rem;
    line-height: 1.625rem;
  }

  .selection-summary[data-size="lg"] {
    font-size: 0.8125rem;
  }

  .selection-summary[data-size="lg"] .selection-summary__header button {
    min-height: 2.125rem;
    padding: 0 0.875rem;
  }

  .selection-summary[data-size="lg"] .selection-summary__chip {
    min-height: 1.75rem;
    padding: 0 0.875rem;
    font-size: 0.8125rem;
  }

  .selection-summary[data-size="lg"] .selection-summary__overflow {
    font-size: 0.875rem;
    line-height: 2.25rem;
    padding: 0 0.75rem;
  }

  .selection-summary[data-size="xl"] {
    font-size: 0.875rem;
  }

  .selection-summary[data-size="xl"] .selection-summary__header button {
    min-height: 2.25rem;
    padding: 0 1rem;
  }

  .selection-summary[data-size="xl"] .selection-summary__chip {
    min-height: 2rem;
    padding: 0 1rem;
    font-size: 0.875rem;
  }

  .selection-summary[data-size="xl"] .selection-summary__overflow {
    font-size: 0.9375rem;
    line-height: 2.5rem;
    padding: 0 0.875rem;
  }
</style>
