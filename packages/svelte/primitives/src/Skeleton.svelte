<script lang="ts">
  import { pxToRem } from "@pug/svelte-tokens";
  import type { SkeletonShape } from "./types";

  export let shape: SkeletonShape = "line";
  export let width: string | null = null;
  export let height: string | null = null;
  export let isAnimated = true;

  $: resolvedWidth =
    width ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? "100%" : "100%");
  $: resolvedHeight =
    height ?? (shape === "circle" ? pxToRem(40) : shape === "block" ? pxToRem(96) : pxToRem(14));
</script>

<span
  class="skeleton"
  data-shape={shape}
  data-animated={isAnimated}
  style={`--pug-skeleton-width: ${resolvedWidth}; --pug-skeleton-height: ${resolvedHeight};`}
  aria-hidden="true"
></span>

<style>
  .skeleton {
    display: block;
    width: var(--pug-skeleton-width);
    height: var(--pug-skeleton-height);
    border-radius: var(--pug-radius-control);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--pug-color-background-elevated) 88%, transparent) 0%,
        color-mix(in srgb, var(--pug-color-background-surface) 92%, white) 48%,
        color-mix(in srgb, var(--pug-color-background-elevated) 88%, transparent) 100%
      );
    background-size: 220% 100%;
  }

  .skeleton[data-shape="circle"] {
    border-radius: 999rem;
  }

  .skeleton[data-shape="block"] {
    border-radius: calc(var(--pug-radius-surface) - 0.25rem);
  }

  .skeleton[data-animated="true"] {
    animation: skeleton-shimmer 1.6s linear infinite;
  }

  @keyframes skeleton-shimmer {
    from {
      background-position: 200% 0;
    }

    to {
      background-position: -20% 0;
    }
  }
</style>
