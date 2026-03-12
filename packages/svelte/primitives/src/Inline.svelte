<script lang="ts">
  import {
    alignItemsValue,
    joinStyles,
    justifyContentValue,
    scaleToSpace,
  } from "./internal";

  import type { LayoutAlign, LayoutJustify, SpaceScale } from "./types";

  export let gap: SpaceScale = "md";
  export let align: LayoutAlign = "center";
  export let justify: LayoutJustify = "start";
  export let wrap = false;
  export let padding: SpaceScale = "none";
  export let asRole: string | null = null;
  export let ariaLabel: string | null = null;

  $: style = joinStyles([
    `gap: ${scaleToSpace(gap)}`,
    `padding: ${scaleToSpace(padding)}`,
    `align-items: ${alignItemsValue(align)}`,
    `justify-content: ${justifyContentValue(justify)}`,
    `flex-wrap: ${wrap ? "wrap" : "nowrap"}`,
  ]);
</script>

<div class="inline" role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
  <slot />
</div>

<style>
  .inline {
    display: flex;
    min-width: 0;
    min-height: 0;
  }
</style>
