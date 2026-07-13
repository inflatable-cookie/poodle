<script lang="ts">
  import "@poodle/styles/eyebrow.css";
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

