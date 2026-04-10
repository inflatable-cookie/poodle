<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { Icon, getUiPresentation, resolveSemanticControlSize } from "@poodle/svelte-primitives";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "@poodle/svelte-primitives";

  export let items: Array<{ id: string; label: string }> = [];
  export let maxVisibleItems = 4;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    remove: { id: string };
    clear: void;
  }>();

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: visibleItems = items.slice(0, maxVisibleItems);
  $: overflowCount = Math.max(0, items.length - visibleItems.length);
</script>

<section class="selection-summary" aria-label="Current selection" data-size={resolvedSize} data-density={resolvedDensity}>
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
    {#if items.length > 0}
      <button type="button" class="selection-summary__clear" on:click={() => dispatch("clear")}>Clear</button>
    {/if}
  </div>
</section>

<style>
  .selection-summary {
    font-size: var(--poodle-typography-label-size, 0.75rem);
    padding-bottom: 0.625rem;
  }

  .selection-summary__chips {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
  }

  .selection-summary__chip {
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-control);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 76%, transparent);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font: inherit;
  }

  .selection-summary__clear {
    margin-left: auto;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
    font: inherit;
    font-size: var(--poodle-typography-label-size, 0.75rem);
  }

  .selection-summary__clear:hover {
    color: var(--poodle-color-text-primary);
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


  /* ── Size variants ──────────────────────────────────────────── */

  .selection-summary[data-size="xs"] { font-size: 0.6875rem; }
  .selection-summary[data-size="xs"] .selection-summary__chip { min-height: 1rem; padding: 0 0.5rem; font-size: 0.6875rem; }

  .selection-summary[data-size="sm"] { font-size: 0.71875rem; }
  .selection-summary[data-size="sm"] .selection-summary__chip { min-height: 1.125rem; font-size: 0.71875rem; }

  .selection-summary[data-size="lg"] { font-size: 0.8125rem; }
  .selection-summary[data-size="lg"] .selection-summary__chip { min-height: 1.75rem; padding: 0 0.875rem; font-size: 0.8125rem; }

  .selection-summary[data-size="xl"] { font-size: 0.875rem; }
  .selection-summary[data-size="xl"] .selection-summary__chip { min-height: 2rem; padding: 0 1rem; font-size: 0.875rem; }
</style>
