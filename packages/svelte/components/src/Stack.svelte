<script lang="ts">
  import type { Snippet } from "svelte";

  import {
    alignItemsValue,
    joinStyles,
    justifyContentValue,
    scaleToSpace,
  } from "./internal";

  import type { LayoutAlign, LayoutJustify, SpaceScale } from "./types";

  let {
    direction = "column",
    gap = "md",
    align = undefined,
    justify = "start",
    wrap = false,
    padding = "none",
    asRole = null,
    ariaLabel = null,
    class: className = "",
    children = undefined,
  }: {
    direction?: "column" | "row";
    gap?: SpaceScale;
    align?: LayoutAlign | undefined;
    justify?: LayoutJustify;
    wrap?: boolean;
    padding?: SpaceScale;
    asRole?: string | null;
    ariaLabel?: string | null;
    class?: string;
    children?: Snippet;
  } = $props();

  const resolvedAlign = $derived(align ?? (direction === "column" ? "stretch" : "center"));
  const style = $derived(joinStyles([
    `flex-direction: ${direction}`,
    `gap: ${scaleToSpace(gap)}`,
    `padding: ${scaleToSpace(padding)}`,
    `align-items: ${alignItemsValue(resolvedAlign)}`,
    `justify-content: ${justifyContentValue(justify)}`,
    `flex-wrap: ${wrap ? "wrap" : "nowrap"}`,
  ]));
</script>

<div class={`poodle-stack ${className}`.trim()} role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
  {@render children?.()}
</div>

<style>
  .poodle-stack {
    display: flex;
    min-width: 0;
    min-height: 0;
  }
</style>
