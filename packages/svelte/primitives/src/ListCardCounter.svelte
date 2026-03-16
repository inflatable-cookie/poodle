<script lang="ts">
  import Icon from "./Icon.svelte";
  import Tooltip from "./Tooltip.svelte";

  export let icon: string;
  export let count: number;
  export let tooltip: string | null = null;
  export let href: string | null = null;

  function handleClick(e: MouseEvent): void {
    if (href) e.stopPropagation();
  }
</script>

{#if tooltip}
  <Tooltip content={tooltip}>
    {#if href}
      <a class="list-card-counter" {href} on:click={handleClick}>
        <Icon name={icon} size="sm" />
        <span>{count}</span>
      </a>
    {:else}
      <span class="list-card-counter">
        <Icon name={icon} size="sm" />
        <span>{count}</span>
      </span>
    {/if}
  </Tooltip>
{:else if href}
  <a class="list-card-counter" {href} on:click={handleClick}>
    <Icon name={icon} size="sm" />
    <span>{count}</span>
  </a>
{:else}
  <span class="list-card-counter">
    <Icon name={icon} size="sm" />
    <span>{count}</span>
  </span>
{/if}

<style>
  .list-card-counter {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    color: var(--pug-color-text-secondary);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
    text-decoration: none;
  }

  a.list-card-counter:hover {
    color: var(--pug-color-text-primary);
  }
</style>
