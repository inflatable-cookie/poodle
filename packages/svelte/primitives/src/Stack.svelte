<script lang="ts">
  import { alignItemsValue, joinStyles, scaleToSpace } from "./internal";

  import type { LayoutAlign, SpaceScale } from "./types";

  export let gap: SpaceScale = "md";
  export let align: LayoutAlign = "stretch";
  export let padding: SpaceScale = "none";
  export let asRole: string | null = null;
  export let ariaLabel: string | null = null;

  $: style = joinStyles([
    `gap: ${scaleToSpace(gap)}`,
    `padding: ${scaleToSpace(padding)}`,
    `align-items: ${alignItemsValue(align)}`,
  ]);
</script>

<div class="stack" role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
  <slot />
</div>

<style>
  .stack {
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
  }
</style>
