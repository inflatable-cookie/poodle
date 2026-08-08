<script lang="ts">
  import "@inflatable-cookie/poodle-styles/meta-item.css";
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

