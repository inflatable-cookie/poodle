<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    as?: "span" | "p" | "h2" | "h3" | "h4";
    ariaLabel?: string | null;
    size?: "xs" | "sm" | "md";
    spacing?: "none" | "bottom";
    children?: Snippet;
  }

  let {
    as = "span",
    ariaLabel = null,
    size = "sm",
    spacing = "none",
    children,
  }: Props = $props();
</script>

{#if as === "p"}
  <p class="poodle-eyebrow" data-size={size} data-spacing={spacing} aria-label={ariaLabel ?? undefined}>
    {@render children?.()}
  </p>
{:else if as === "h2"}
  <h2 class="poodle-eyebrow" data-size={size} data-spacing={spacing} aria-label={ariaLabel ?? undefined}>
    {@render children?.()}
  </h2>
{:else if as === "h3"}
  <h3 class="poodle-eyebrow" data-size={size} data-spacing={spacing} aria-label={ariaLabel ?? undefined}>
    {@render children?.()}
  </h3>
{:else if as === "h4"}
  <h4 class="poodle-eyebrow" data-size={size} data-spacing={spacing} aria-label={ariaLabel ?? undefined}>
    {@render children?.()}
  </h4>
{:else}
  <span class="poodle-eyebrow" data-size={size} data-spacing={spacing} aria-label={ariaLabel ?? undefined}>
    {@render children?.()}
  </span>
{/if}

<style>
  .poodle-eyebrow {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    line-height: 1.5;
    text-transform: uppercase;
  }

  .poodle-eyebrow[data-size="xs"] {
    font-size: 0.6875rem;
  }

  .poodle-eyebrow[data-size="md"] {
    font-size: 0.85rem;
    letter-spacing: 0.04em;
  }

  .poodle-eyebrow[data-spacing="bottom"] {
    margin-bottom: 0.5rem;
  }

  .poodle-eyebrow[data-size="xs"][data-spacing="bottom"] {
    margin-bottom: 0.35rem;
  }
</style>
