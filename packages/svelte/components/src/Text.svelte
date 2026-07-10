<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    as?: "p" | "span" | "div";
    tone?: "default" | "secondary" | "muted" | "success" | "danger" | "warning";
    size?: "xs" | "sm" | "md";
    weight?: "normal" | "medium" | "semibold" | "bold";
    leading?: "normal" | "relaxed";
    spacing?: "none" | "compact";
    clamp?: "none" | 1 | 2 | 3;
    children?: Snippet;
  }

  let {
    as = "p",
    tone = "default",
    size = "md",
    weight = "normal",
    leading = "normal",
    spacing = "none",
    clamp = "none",
    children,
  }: Props = $props();
</script>

{#if as === "span"}
  <span class="poodle-text" data-tone={tone} data-size={size} data-weight={weight} data-leading={leading} data-spacing={spacing} data-clamp={clamp}>
    {@render children?.()}
  </span>
{:else if as === "div"}
  <div class="poodle-text" data-tone={tone} data-size={size} data-weight={weight} data-leading={leading} data-spacing={spacing} data-clamp={clamp}>
    {@render children?.()}
  </div>
{:else}
  <p class="poodle-text" data-tone={tone} data-size={size} data-weight={weight} data-leading={leading} data-spacing={spacing} data-clamp={clamp}>
    {@render children?.()}
  </p>
{/if}

<style>
  .poodle-text {
    margin: 0;
    color: var(--poodle-recipe-text-text, var(--poodle-color-text-primary));
    font-size: 0.875rem;
    line-height: 1.5;
  }

  .poodle-text[data-size="xs"] {
    font-size: 0.75rem;
  }

  .poodle-text[data-size="sm"] {
    font-size: 0.8125rem;
  }

  .poodle-text[data-leading="relaxed"] {
    line-height: 1.6;
  }

  .poodle-text[data-weight="medium"] {
    font-weight: 500;
  }

  .poodle-text[data-weight="semibold"] {
    font-weight: 600;
  }

  .poodle-text[data-weight="bold"] {
    font-weight: 700;
  }

  .poodle-text[data-spacing="compact"] {
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-text[data-clamp="1"],
  .poodle-text[data-clamp="2"],
  .poodle-text[data-clamp="3"] {
    display: -webkit-box;
    overflow: hidden;
    -webkit-box-orient: vertical;
  }

  .poodle-text[data-clamp="1"] {
    -webkit-line-clamp: 1;
    line-clamp: 1;
  }

  .poodle-text[data-clamp="2"] {
    -webkit-line-clamp: 2;
    line-clamp: 2;
  }

  .poodle-text[data-clamp="3"] {
    -webkit-line-clamp: 3;
    line-clamp: 3;
  }

  .poodle-text[data-tone="secondary"],
  .poodle-text[data-tone="muted"] {
    color: var(--poodle-recipe-text-secondary-muted-text, var(--poodle-color-text-secondary));
  }

  .poodle-text[data-tone="success"] {
    color: var(--poodle-recipe-text-success-text, var(--poodle-color-status-success));
  }

  .poodle-text[data-tone="danger"] {
    color: var(--poodle-recipe-text-danger-text, var(--poodle-color-status-danger));
  }

  .poodle-text[data-tone="warning"] {
    color: var(--poodle-recipe-text-warning-text, var(--poodle-color-status-warning));
  }
</style>
