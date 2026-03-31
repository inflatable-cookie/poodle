<script lang="ts">
  import type { Snippet } from "svelte";

  type ListGridVariant = "default" | "compact";

  export let variant: ListGridVariant = "default";
  export let minItemWidth: number | string | null = null;
  export let gap: number | string | null = null;
  export let className = "";
  export { className as class };
  export let actions: Snippet | undefined = undefined;
  export let children: Snippet | undefined = undefined;

  function formatValue(value: number | string | null, defaultUnit: string): string | null {
    if (value == null) return null;
    if (typeof value === "number") return `${value}${defaultUnit}`;
    return value;
  }

  $: min = formatValue(minItemWidth, "em") ?? "360px";
  $: gridGap = formatValue(gap, "px") ?? (variant === "compact" ? "0.5rem" : "1.25rem");
  $: columns = variant === "compact"
    ? "1fr"
    : `repeat(auto-fill, minmax(min(${min}, 100%), 1fr))`;
</script>

<div
  class={`poodle-list-grid list-grid ${className}`.trim()}
  style={`--poodle-list-grid-gap: ${gridGap};`}
>
  {#if actions}
    <div class="list-grid__header">
      {@render actions()}
    </div>
  {/if}

  <div class="list-grid__content" style={`grid-template-columns: ${columns}; gap: ${gridGap};`}>
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>

<style>
  .list-grid {
    margin-block: var(--poodle-list-grid-gap);
  }

  .list-grid__header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: var(--poodle-list-grid-gap);
  }

  .list-grid__content {
    display: grid;
    align-items: stretch;
  }
</style>
