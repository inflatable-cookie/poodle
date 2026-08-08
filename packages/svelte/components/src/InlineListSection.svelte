<script lang="ts" generics="T">
  import "@inflatable-cookie/poodle-core/styles/inline-list-section.css";
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

