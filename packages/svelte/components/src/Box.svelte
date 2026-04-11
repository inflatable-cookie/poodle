<script lang="ts">
  import { joinStyles, overflowValue, scaleToSpace } from "./internal";

  import type { OverflowMode, SpaceScale } from "./types";

  export let padding: SpaceScale = "none";
  export let width: string | null = null;
  export let height: string | null = null;
  export let minWidth: string | null = null;
  export let minHeight: string | null = null;
  export let overflow: OverflowMode = "visible";
  export let asRole: string | null = null;
  export let ariaLabel: string | null = null;

  $: style = joinStyles([
    `padding: ${scaleToSpace(padding)}`,
    width ? `width: ${width}` : null,
    height ? `height: ${height}` : null,
    minWidth ? `min-width: ${minWidth}` : null,
    minHeight ? `min-height: ${minHeight}` : null,
    `overflow: ${overflowValue(overflow)}`,
  ]);
</script>

<div class="box" role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
  <slot />
</div>

<style>
  .box {
    min-width: 0;
    min-height: 0;
  }
</style>
