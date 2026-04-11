<script lang="ts" generics="T">
  import type { Snippet } from "svelte";

  import Card from "./Card.svelte";

  interface Props {
    title: string;
    items: T[];
    item: Snippet<[T]>;
    actions?: Snippet;
    emptyMessage?: string | null;
  }

  let {
    title,
    items,
    item,
    actions,
    emptyMessage = "No items yet."
  }: Props = $props();
</script>

<Card>
  <section class="inline-list-section" aria-label={title}>
    <div class="inline-list-section__header">
      <h4 class="inline-list-section__title">{title}</h4>
      {#if actions}
        <div class="inline-list-section__header-actions">
          {@render actions()}
        </div>
      {/if}
    </div>

    {#if items.length === 0}
      {#if emptyMessage}
        <p class="inline-list-section__empty">{emptyMessage}</p>
      {/if}
    {:else}
      <ul class="inline-list-section__items">
        {#each items as entry}
          <li class="inline-list-section__item">
            {@render item(entry)}
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</Card>

<style>
  .inline-list-section {
    display: grid;
    gap: 0.75rem;
  }

  .inline-list-section__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }

  .inline-list-section__title {
    margin: 0;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
  }

  .inline-list-section__header-actions {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .inline-list-section__items {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .inline-list-section__item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-width: 0;
    padding: 0.5rem 0.625rem;
    border-radius: var(--underlay-radius-sm, 0.375rem);
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
  }

  .inline-list-section__empty {
    margin: 0;
    font-size: 0.9rem;
    font-style: italic;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.7));
  }
</style>
