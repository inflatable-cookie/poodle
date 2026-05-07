<script lang="ts">
  import Icon from "./Icon.svelte";
  import type { IconProp } from "./types";
  import Tooltip from "./Tooltip.svelte";

  type ListCardCounterTypography = "label" | "inherit";

  export let icon: IconProp;
  export let count: number;
  export let tooltip: string | null = null;
  export let href: string | null = null;
  export let onClick: ((event: MouseEvent) => void) | null = null;
  export let typography: ListCardCounterTypography = "label";

  function handleClick(e: MouseEvent): void {
    if (href) e.stopPropagation();
    onClick?.(e);
  }
</script>

{#if tooltip}
  <Tooltip content={tooltip}>
    {#if href}
      <a class="poodle-list-card-counter" data-typography={typography} {href} onclick={handleClick}>
        <Icon icon={icon} />
        <span>{count}</span>
      </a>
    {:else}
      <span class="poodle-list-card-counter" data-typography={typography}>
        <Icon icon={icon} />
        <span>{count}</span>
      </span>
    {/if}
  </Tooltip>
{:else if href}
  <a class="poodle-list-card-counter" data-typography={typography} {href} onclick={handleClick}>
    <Icon icon={icon} />
    <span>{count}</span>
  </a>
{:else}
  <span class="poodle-list-card-counter" data-typography={typography}>
    <Icon icon={icon} />
    <span>{count}</span>
  </span>
{/if}

<style>
  .poodle-list-card-counter {
    --poodle-list-card-counter-gap: 0.25rem;
    --poodle-list-card-counter-font-size: 0.75rem;
    --poodle-list-card-counter-icon-size: 1rem;
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-list-card-counter-gap);
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-list-card-counter-font-size);
    font-variant-numeric: tabular-nums;
    text-decoration: none;
  }

  .poodle-list-card-counter :global(.poodle-icon) {
    width: var(--poodle-list-card-counter-icon-size);
    height: var(--poodle-list-card-counter-icon-size);
  }

  .poodle-list-card-counter[data-typography="inherit"] {
    --poodle-list-card-counter-gap: 0.3333em;
    --poodle-list-card-counter-font-size: 0.8571em;
    --poodle-list-card-counter-icon-size: 1.3333em;
    line-height: inherit;
  }

  a.poodle-list-card-counter:hover {
    color: var(--poodle-color-text-primary);
  }
</style>
