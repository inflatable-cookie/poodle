<script lang="ts">
  import Icon from "./Icon.svelte";
  import type { IconProp } from "./types";
  import Tooltip from "./Tooltip.svelte";

  export let icon: IconProp;
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
        <Icon icon={icon} />
        <span>{count}</span>
      </a>
    {:else}
      <span class="list-card-counter">
        <Icon icon={icon} />
        <span>{count}</span>
      </span>
    {/if}
  </Tooltip>
{:else if href}
  <a class="list-card-counter" {href} on:click={handleClick}>
    <Icon icon={icon} />
    <span>{count}</span>
  </a>
{:else}
  <span class="list-card-counter">
    <Icon icon={icon} />
    <span>{count}</span>
  </span>
{/if}

<style>
  .list-card-counter {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    font-variant-numeric: tabular-nums;
    text-decoration: none;
  }

  a.list-card-counter:hover {
    color: var(--poodle-color-text-primary);
  }
</style>
