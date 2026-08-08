<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/text.css";
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

