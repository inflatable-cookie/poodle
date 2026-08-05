<script lang="ts">
  import "@poodle/styles/surface.css";
  import type { Snippet } from "svelte";

  import { joinStyles } from "./internal.ts";

  import type { SpaceScale, SurfaceBorder, SurfaceTone } from "./types.ts";

  let {
    tone = "panel",
    border = "subtle",
    padding = "md",
    elevated = false,
    asRole = null,
    label = null,
    children = undefined,
  }: {
    tone?: SurfaceTone;
    border?: SurfaceBorder;
    padding?: SpaceScale;
    elevated?: boolean;
    asRole?: "region" | "group" | null;
    label?: string | null;
    children?: Snippet;
  } = $props();

  function surfacePadding(scale: SpaceScale): string {
    switch (scale) {
      case "sm": return "var(--poodle-space-panel-y)";
      case "md": return "1rem";
      case "lg": return "1.5rem";
      default: return "0";
    }
  }

  const style = $derived(joinStyles([`padding: ${surfacePadding(padding)}`]));
</script>

<div
  class="poodle-surface"
  data-tone={tone}
  data-border={border}
  data-elevated={elevated}
  role={asRole ?? undefined}
  aria-label={label ?? undefined}
  style={style}
>
  {@render children?.()}
</div>

