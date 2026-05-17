<script lang="ts">
  import Icon from "./Icon.svelte";
  import type { IconProp } from "./types";
  import Tooltip from "./Tooltip.svelte";

  type ListCardCounterTypography = "label" | "inherit";

  interface Props {
    icon: IconProp;
    count: number;
    tooltip?: string | null;
    href?: string | null;
    onClick?: ((event: MouseEvent) => void) | null;
    typography?: ListCardCounterTypography;
  }

  let {
    icon,
    count,
    tooltip = null,
    href = null,
    onClick = null,
    typography = "label",
  }: Props = $props();

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
    --poodle-list-card-counter-font-size: 0.6875rem;
    --poodle-list-card-counter-icon-size: 0.75rem;
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-list-card-counter-gap);
    color: color-mix(in srgb, currentColor 36%, transparent);
    font-size: var(--poodle-list-card-counter-font-size);
    font-variant-numeric: tabular-nums;
    text-decoration: none;
  }

  .poodle-list-card-counter :global(.poodle-icon) {
    width: var(--poodle-list-card-counter-icon-size);
    height: var(--poodle-list-card-counter-icon-size);
    opacity: 0.82;
  }

  .poodle-list-card-counter[data-typography="inherit"] {
    --poodle-list-card-counter-gap: 0.3333em;
    --poodle-list-card-counter-font-size: 0.8571em;
    --poodle-list-card-counter-icon-size: 1em;
    line-height: inherit;
  }

  a.poodle-list-card-counter:hover {
    color: color-mix(in srgb, currentColor 58%, transparent);
  }
</style>
