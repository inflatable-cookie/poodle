<script lang="ts">
  import type { Snippet } from "svelte";

  import { joinStyles } from "./internal";

  import type { SpaceScale, SurfaceBorder, SurfaceTone } from "./types";

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

<style>
  .poodle-surface {
    --poodle-surface-fill: var(--poodle-recipe-surface-fill, var(
      --poodle-treatment-surface-fill,
      color-mix(in srgb, var(--poodle-color-background-surface) 96%, transparent)
    ));
    --poodle-surface: var(--poodle-surface-fill);
    --poodle-surface-border: var(--poodle-recipe-surface-border, var(
      --poodle-treatment-surface-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)
    ));
    --poodle-surface-shadow: var(--poodle-recipe-surface-shadow, var(--poodle-treatment-surface-shadow, none));
    min-width: 0;
    min-height: 0;
    border: 0.0625rem solid var(--poodle-surface-border);
    border-radius: var(--poodle-treatment-surface-radius, var(--poodle-radius-surface));
    background: var(--poodle-surface-fill);
    box-shadow: var(--poodle-surface-shadow);
  }

  .poodle-surface[data-tone="canvas"] {
    --poodle-surface-fill: var(--poodle-recipe-surface-canvas-fill, color-mix(in srgb, var(--poodle-color-background-canvas) 98%, transparent));
  }

  .poodle-surface[data-tone="elevated"],
  .poodle-surface[data-elevated="true"] {
    --poodle-surface-fill: var(--poodle-recipe-surface-elevated-fill, var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 96%, var(--poodle-color-background-panel))
    ));
    --poodle-surface-border: var(--poodle-recipe-surface-elevated-border, var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)
    ));
    --poodle-surface-shadow: var(--poodle-recipe-surface-elevated-shadow, var(
      --poodle-treatment-surface-elevated-shadow,
      var(--poodle-elevation-surface)
    ));
  }

  .poodle-surface[data-border="none"] {
    border-color: transparent;
  }

  .poodle-surface[data-border="default"] {
    --poodle-surface-border: var(--poodle-recipe-surface-border, var(--poodle-color-border-default));
  }
</style>
