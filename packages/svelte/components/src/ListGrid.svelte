<script lang="ts">
  import "@inflatable-cookie/poodle-styles/list-grid.css";
  import type { Snippet } from "svelte";

  type ListGridVariant = "default" | "compact";

  let {
    variant = "default",
    minItemWidth = null,
    maxColumns = 3,
    gap = null,
    class: className = "",
    actions = undefined,
    children = undefined,
  }: {
    variant?: ListGridVariant;
    minItemWidth?: number | string | null;
    maxColumns?: number | null;
    gap?: number | string | null;
    class?: string;
    actions?: Snippet;
    children?: Snippet;
  } = $props();

  function formatValue(value: number | string | null, defaultUnit: string): string | null {
    if (value == null) return null;
    if (typeof value === "number") return `${value}${defaultUnit}`;
    return value;
  }

  const min = $derived(formatValue(minItemWidth, "em") ?? "360px");
  const gridGap = $derived(formatValue(gap, "px") ?? (variant === "compact" ? "0.5rem" : "1.25rem"));
  const columnCap = $derived(maxColumns == null ? null : Math.max(1, Math.floor(maxColumns)));
  const columns = $derived(
    variant === "compact"
      ? "1fr"
      : columnCap == null
        ? `repeat(auto-fill, minmax(min(${min}, 100%), 1fr))`
        : `repeat(auto-fill, minmax(min(100%, max(${min}, calc((100% - (${columnCap} - 1) * ${gridGap}) / ${columnCap}))), 1fr))`,
  );
</script>

<div
  class={`poodle-list-grid ${className}`.trim()}
  style={`--poodle-list-grid-gap: ${gridGap};`}
>
  {#if actions}
    <div class="poodle-list-grid__header">
      {@render actions()}
    </div>
  {/if}

  <div class="poodle-list-grid__content" style={`grid-template-columns: ${columns}; gap: ${gridGap};`}>
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>

