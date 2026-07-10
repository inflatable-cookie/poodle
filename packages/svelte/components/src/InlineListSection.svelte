<script lang="ts" generics="T">
  import type { Snippet } from "svelte";

  import { default as Card } from "./Card.svelte";

  interface Props {
    title: string;
    items: T[];
    item: Snippet<[T]>;
    actions?: Snippet;
    emptyMessage?: string | null;
    count?: number | string | null;
    framed?: boolean;
  }

  let {
    title,
    items,
    item,
    actions,
    emptyMessage = "No items yet.",
    count = null,
    framed = true
  }: Props = $props();
</script>

{#if framed}
  <Card>
    <section class="poodle-inline-list-section" aria-label={title}>
      <div class="poodle-inline-list-section__header">
        <div class="poodle-inline-list-section__heading">
          <h4 class="poodle-inline-list-section__title">{title}</h4>
          {#if count !== null}
            <span class="poodle-inline-list-section__count">{count}</span>
          {/if}
        </div>
        {#if actions}
          <div class="poodle-inline-list-section__header-actions">
            {@render actions()}
          </div>
        {/if}
      </div>

      {#if items.length === 0}
        {#if emptyMessage}
          <p class="poodle-inline-list-section__empty">{emptyMessage}</p>
        {/if}
      {:else}
        <ul class="poodle-inline-list-section__items">
          {#each items as entry}
            <li class="poodle-inline-list-section__item">
              {@render item(entry)}
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  </Card>
{:else}
  <section class="poodle-inline-list-section" aria-label={title}>
    <div class="poodle-inline-list-section__header">
      <div class="poodle-inline-list-section__heading">
        <h4 class="poodle-inline-list-section__title">{title}</h4>
        {#if count !== null}
          <span class="poodle-inline-list-section__count">{count}</span>
        {/if}
      </div>
      {#if actions}
        <div class="poodle-inline-list-section__header-actions">
          {@render actions()}
        </div>
      {/if}
    </div>

    {#if items.length === 0}
      {#if emptyMessage}
        <p class="poodle-inline-list-section__empty">{emptyMessage}</p>
      {/if}
    {:else}
      <ul class="poodle-inline-list-section__items">
        {#each items as entry}
          <li class="poodle-inline-list-section__item">
            {@render item(entry)}
          </li>
        {/each}
      </ul>
    {/if}
  </section>
{/if}

<style>
  .poodle-inline-list-section {
    display: grid;
    gap: var(--poodle-space-stack-md);
  }

  .poodle-inline-list-section__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .poodle-inline-list-section__heading {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .poodle-inline-list-section__title {
    margin: 0;
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--poodle-recipe-inline-list-section-title-text, var(--poodle-color-text-secondary));
  }

  .poodle-inline-list-section__count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.875rem;
    height: 1.375rem;
    padding: 0 0.5rem;
    border-radius: 999rem;
    border: 1px solid var(--poodle-color-border);
    background: var(--poodle-recipe-inline-list-section-count-fill, var(--poodle-surface-elevated));
    color: var(--poodle-recipe-inline-list-section-count-text, var(--poodle-color-text-secondary));
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: 1;
  }

  .poodle-inline-list-section__header-actions {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .poodle-inline-list-section__items {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-inline-list-section__item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-width: 0;
    padding: 0.5rem 0.625rem;
    border-radius: calc(var(--poodle-radius-surface) - 0.1875rem);
    background: var(--poodle-recipe-inline-list-section-item-fill, color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary)));
  }

  .poodle-inline-list-section__empty {
    margin: 0;
    font-size: var(--poodle-typography-body-size);
    font-style: italic;
    color: var(--poodle-recipe-inline-list-section-empty-text, var(--poodle-color-text-secondary));
  }

  @media (max-width: 45rem) {
    .poodle-inline-list-section__header-actions {
      gap: 0.25rem;
    }
  }
</style>
