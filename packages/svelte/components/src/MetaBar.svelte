<script lang="ts">
  import type { Snippet } from "svelte";
  import { setPillContext } from "./pill-context";

  interface Props {
    ariaLabel?: string | null;
    showSeparators?: boolean;
    children?: Snippet;
  }

  let {
    ariaLabel = null,
    showSeparators = true,
    children,
  }: Props = $props();

  setPillContext({
    size: "md",
    typography: "inherit",
  });
</script>

<div
  class="poodle-meta-bar"
  data-separators={showSeparators}
  aria-label={ariaLabel ?? undefined}
>
  {@render children?.()}
</div>

<style>
  .poodle-meta-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    line-height: 1.4;
  }

  .poodle-meta-bar > :global(*) {
    min-width: 0;
  }

  .poodle-meta-bar[data-separators="true"] > :global(* + [data-separator="true"]) {
    position: relative;
    padding-inline-start: 1rem;
  }

  .poodle-meta-bar[data-separators="true"] > :global(* + [data-separator="true"])::before {
    content: "";
    position: absolute;
    inset-inline-start: 0.375rem;
    top: 50%;
    width: 0.25rem;
    height: 0.25rem;
    border-radius: 999px;
    background: var(--poodle-recipe-meta-bar-fill, color-mix(in srgb, var(--poodle-color-text-secondary) 72%, transparent));
    transform: translateY(-50%);
  }

  .poodle-meta-bar[data-separators="true"] > :global(*:has(.poodle-pill)[data-separator="true"]) {
    padding-inline-start: 0;
  }

  .poodle-meta-bar[data-separators="true"] > :global(*:has(.poodle-pill)[data-separator="true"])::before {
    display: none;
  }

  .poodle-meta-bar > :global(.poodle-meta-item:has(.poodle-pill)) {
    --poodle-meta-item-gap: 0;
  }

  .poodle-meta-bar > :global(.poodle-meta-item:has(.poodle-pill) .poodle-meta-item__label) {
    display: none;
  }

  .poodle-meta-bar :global(.poodle-code--inline) {
    font-size: calc(1em * var(--poodle-typography-code-adjustmentRatio));
    line-height: inherit;
  }

  .poodle-meta-bar :global(.poodle-code--inline-wrap) {
    gap: 0.1875rem;
  }

  .poodle-meta-bar :global(.poodle-code__copy--inline) {
    width: 1em;
    height: 1em;
  }

  .poodle-meta-bar :global(.poodle-code__copy--inline svg) {
    width: 0.8125em;
    height: 0.8125em;
  }

  @media (max-width: 40rem) {
    .poodle-meta-bar[data-separators="true"] > :global(* + [data-separator="true"]) {
      padding-inline-start: 0.75rem;
    }

    .poodle-meta-bar[data-separators="true"] > :global(* + [data-separator="true"])::before {
      inset-inline-start: 0.25rem;
    }
  }
</style>
