<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    label?: string | null;
    ariaLabel?: string | null;
    typography?: "body" | "inherit";
    separator?: boolean;
    children?: Snippet;
  }

  let {
    label = null,
    ariaLabel = null,
    typography = "body",
    separator = true,
    children,
  }: Props = $props();
</script>

<span
  class="poodle-meta-item"
  data-typography={typography}
  data-separator={separator}
  aria-label={ariaLabel ?? undefined}
>
  {#if label}
    <span class="poodle-meta-item__label">{label}</span>
  {/if}
  <span class="poodle-meta-item__value">
    {@render children?.()}
  </span>
</span>

<style>
  .poodle-meta-item {
    --poodle-meta-item-gap: 0.375rem;
    --poodle-meta-item-label-font-size: 0.6875rem;
    --poodle-meta-item-value-font-size: 0.875rem;
    display: inline-flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--poodle-meta-item-gap);
    min-width: 0;
  }

  .poodle-meta-item__label {
    color: var(--poodle-recipe-meta-item-label-text, var(--poodle-color-text-secondary));
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-meta-item-label-font-size);
    font-weight: var(--poodle-typography-label-weight);
    letter-spacing: 0.08em;
    line-height: 1;
    text-transform: uppercase;
    white-space: nowrap;
  }

  .poodle-meta-item__value {
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-meta-item-gap);
    min-width: 0;
    color: var(--poodle-recipe-meta-item-value-text, var(--poodle-color-text-primary));
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-meta-item-value-font-size);
    line-height: 1.4;
  }

  .poodle-meta-item[data-typography="inherit"] {
    --poodle-meta-item-gap: 0.375em;
    --poodle-meta-item-label-font-size: 0.6875em;
    --poodle-meta-item-value-font-size: 1em;
    line-height: inherit;
  }

  .poodle-meta-item__value :global(*) {
    min-width: 0;
  }

</style>
