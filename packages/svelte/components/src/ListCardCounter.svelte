<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/list-card-counter.css";
  import { default as Icon } from "./Icon.svelte";
  import type { IconProp } from "./types";
  import { default as Tooltip } from "./Tooltip.svelte";

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

