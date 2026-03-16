<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { SortField, ActiveSort } from "./types";

  export let fields: SortField[] = [];
  export let activeSort: ActiveSort | null = null;
  export let ariaLabel = "Sort by";
  export let isDisabled = false;

  const dispatch = createEventDispatcher<{
    change: { sort: ActiveSort | null };
  }>();

  function toggleSort(field: SortField): void {
    if (isDisabled || field.isDisabled) return;

    if (activeSort?.field === field.value) {
      if (activeSort.direction === "asc") {
        activeSort = { field: field.value, direction: "desc" };
      } else {
        activeSort = null;
      }
    } else {
      activeSort = { field: field.value, direction: "asc" };
    }

    dispatch("change", { sort: activeSort });
  }

  function reset(): void {
    activeSort = null;
    dispatch("change", { sort: null });
  }
</script>

<div class="order-by" role="toolbar" aria-label={ariaLabel} data-disabled={isDisabled}>
  <span class="order-by__label">Sort by</span>
  <div class="order-by__fields">
    {#each fields as field (field.value)}
      <button
        type="button"
        class="order-by__field"
        class:order-by__field--active={activeSort?.field === field.value}
        disabled={isDisabled || field.isDisabled === true}
        aria-pressed={activeSort?.field === field.value ? "true" : "false"}
        on:click={() => toggleSort(field)}
      >
        <span>{field.label}</span>
        {#if activeSort?.field === field.value}
          <svg class="order-by__arrow" viewBox="0 0 16 16" fill="none" aria-hidden="true"
            data-direction={activeSort.direction}
          >
            <path d="M8 3v10M4 7l4-4 4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        {/if}
      </button>
    {/each}
  </div>
  {#if activeSort}
    <button
      type="button"
      class="order-by__reset"
      on:click={reset}
      aria-label="Clear sort"
    >
      <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
        <path d="M4 4l8 8M12 4l-8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
      </svg>
    </button>
  {/if}
</div>

<style>
  .order-by {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .order-by[data-disabled="true"] {
    opacity: var(--pug-state-opacity-disabled);
  }

  .order-by__label {
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: var(--pug-typography-label-weight);
    color: var(--pug-color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .order-by__fields {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .order-by__field {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    height: 1.75rem;
    padding: 0 0.5rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: var(--pug-typography-label-weight);
    transition:
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .order-by__field:hover:not(:disabled) {
    background: color-mix(in srgb, var(--pug-color-background-surface) 84%, var(--pug-color-background-elevated));
  }

  .order-by__field:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .order-by__field:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }

  .order-by__field--active {
    border-color: var(--pug-color-accent-base);
    background: color-mix(in srgb, var(--pug-color-accent-base) 12%, var(--pug-color-background-surface));
    color: var(--pug-color-accent-base);
  }

  .order-by__arrow {
    width: 0.75rem;
    height: 0.75rem;
    transition: transform var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .order-by__arrow[data-direction="desc"] {
    transform: rotate(180deg);
  }

  .order-by__reset {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: 0;
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: var(--pug-color-text-secondary);
    cursor: pointer;
    transition: color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .order-by__reset:hover {
    color: var(--pug-color-text-primary);
  }

  .order-by__reset:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
  }

  .order-by__reset svg {
    width: 0.75rem;
    height: 0.75rem;
  }
</style>
