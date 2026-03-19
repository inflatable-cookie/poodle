<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";

  import type { BreadcrumbItem } from "./types";

  export let items: BreadcrumbItem[] = [];
  export let ariaLabel = "Breadcrumb";
  export let maxVisibleItems: number | null = null;

  const dispatch = createEventDispatcher<{
    navigate: { value: string };
  }>();

  $: visibleItems =
    maxVisibleItems !== null && items.length > maxVisibleItems
      ? [items[0], { value: "__ellipsis__", label: "…", isCurrent: false }, ...items.slice(items.length - (maxVisibleItems - 1))]
      : items;

  function handleNavigate(item: BreadcrumbItem): void {
    if (item.isCurrent || item.value === "__ellipsis__") {
      return;
    }

    dispatch("navigate", { value: item.value });
  }
</script>

<nav class="breadcrumbs" aria-label={ariaLabel}>
  <ol class="breadcrumbs__list">
    {#each visibleItems as item, index}
      <li class="breadcrumbs__item">
        {#if item.isCurrent || index === visibleItems.length - 1}
          <span aria-current="page">{item.label}</span>
        {:else if item.href}
          <a href={item.href}>{item.label}</a>
        {:else if item.value === "__ellipsis__"}
          <span aria-hidden="true">{item.label}</span>
        {:else}
          <button type="button" on:click={() => handleNavigate(item)}>{item.label}</button>
        {/if}
        {#if index < visibleItems.length - 1}
          <span class="breadcrumbs__separator" aria-hidden="true"><Icon name="chevron-right" size="sm" /></span>
        {/if}
      </li>
    {/each}
  </ol>
</nav>

<style>
  .breadcrumbs__list {
    display: flex;
    flex-wrap: wrap;
    gap: var(--pug-space-inline-sm);
    margin: 0;
    padding: 0;
    list-style: none;
    color: var(--pug-color-text-secondary);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
  }

  .breadcrumbs__item {
    display: inline-flex;
    align-items: center;
    gap: var(--pug-space-inline-sm);
  }

  .breadcrumbs a,
  .breadcrumbs button {
    border: 0;
    padding: 0;
    background: transparent;
    color: inherit;
    cursor: pointer;
    font: inherit;
    text-decoration: none;
  }

  .breadcrumbs__separator {
    display: inline-flex;
    opacity: 0.4;
  }

  .breadcrumbs [aria-current="page"] {
    color: var(--pug-color-text-primary);
  }
</style>
