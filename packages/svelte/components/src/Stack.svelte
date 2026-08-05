<script lang="ts">
  import "@poodle/styles/stack.css";
  import type { Snippet } from "svelte";

  import {
    alignItemsValue,
    joinStyles,
    justifyContentValue,
    overflowValue,
    scaleToSpace,
  } from "./internal.ts";

  import type { LayoutAlign, LayoutJustify, OverflowMode, SpaceScale } from "./types.ts";

  let {
    direction = "column",
    gap = "md",
    align = undefined,
    justify = "start",
    wrap = false,
    padding = "none",
    width = null,
    height = null,
    minWidth = null,
    minHeight = null,
    overflow = "visible",
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
    width?: string | null;
    height?: string | null;
    minWidth?: string | null;
    minHeight?: string | null;
    overflow?: OverflowMode;
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
    width ? `width: ${width}` : null,
    height ? `height: ${height}` : null,
    minWidth ? `min-width: ${minWidth}` : null,
    minHeight ? `min-height: ${minHeight}` : null,
    `overflow: ${overflowValue(overflow)}`,
  ]));
</script>

<div class={`poodle-stack ${className}`.trim()} role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
  {@render children?.()}
</div>

