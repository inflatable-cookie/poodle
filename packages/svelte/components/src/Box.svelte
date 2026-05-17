<script lang="ts">
  import type { Snippet } from "svelte";

  import { joinStyles, overflowValue, scaleToSpace } from "./internal";

  import type { OverflowMode, SpaceScale } from "./types";

  let {
    padding = "none",
    width = null,
    height = null,
    minWidth = null,
    minHeight = null,
    overflow = "visible",
    asRole = null,
    ariaLabel = null,
    children = undefined,
  }: {
    padding?: SpaceScale;
    width?: string | null;
    height?: string | null;
    minWidth?: string | null;
    minHeight?: string | null;
    overflow?: OverflowMode;
    asRole?: string | null;
    ariaLabel?: string | null;
    children?: Snippet;
  } = $props();

  const style = $derived(joinStyles([
    `padding: ${scaleToSpace(padding)}`,
    width ? `width: ${width}` : null,
    height ? `height: ${height}` : null,
    minWidth ? `min-width: ${minWidth}` : null,
    minHeight ? `min-height: ${minHeight}` : null,
    `overflow: ${overflowValue(overflow)}`,
  ]));
</script>

<div class="poodle-box" role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
  {@render children?.()}
</div>

<style>
  .poodle-box {
    min-width: 0;
    min-height: 0;
  }
</style>
