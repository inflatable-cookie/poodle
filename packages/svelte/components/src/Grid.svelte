<script lang="ts">
  import { joinStyles, scaleToSpace } from "./internal";

  import type { SpaceScale } from "./types";

  export let columns = "1fr";
  export let rows: string | null = null;
  export let gap: SpaceScale = "md";
  export let padding: SpaceScale = "none";
  export let asRole: string | null = null;
  export let ariaLabel: string | null = null;

  $: style = joinStyles([
    `grid-template-columns: ${columns}`,
    rows ? `grid-template-rows: ${rows}` : null,
    `gap: ${scaleToSpace(gap)}`,
    `padding: ${scaleToSpace(padding)}`,
  ]);
</script>

<div class="poodle-grid" role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
  <slot />
</div>

<style>
  .poodle-grid {
    display: grid;
    min-width: 0;
    min-height: 0;
  }
</style>
