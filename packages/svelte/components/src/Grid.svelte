<script lang="ts">
  import "@poodle/styles/grid.css";
  import type { Snippet } from "svelte";

  import { joinStyles, scaleToSpace } from "./internal";

  import type { SpaceScale } from "./types";

  let {
    columns = "1fr",
    rows = null,
    gap = "md",
    padding = "none",
    asRole = null,
    ariaLabel = null,
    children = undefined,
  }: {
    columns?: string;
    rows?: string | null;
    gap?: SpaceScale;
    padding?: SpaceScale;
    asRole?: string | null;
    ariaLabel?: string | null;
    children?: Snippet;
  } = $props();

  const style = $derived(joinStyles([
    `grid-template-columns: ${columns}`,
    rows ? `grid-template-rows: ${rows}` : null,
    `gap: ${scaleToSpace(gap)}`,
    `padding: ${scaleToSpace(padding)}`,
  ]));
</script>

<div class="poodle-grid" role={asRole ?? undefined} aria-label={ariaLabel ?? undefined} style={style}>
  {@render children?.()}
</div>

