<script lang="ts">
  import { joinStyles } from "./internal";

  import type { SpaceScale, SurfaceBorder, SurfaceTone } from "./types";

  export let tone: SurfaceTone = "panel";
  export let border: SurfaceBorder = "subtle";
  export let padding: SpaceScale = "md";
  export let isElevated = false;
  export let asRole: "region" | "group" | null = null;
  export let label: string | null = null;

  function surfacePadding(scale: SpaceScale): string {
    switch (scale) {
      case "sm": return "var(--pug-space-panel-y)";
      case "md": return "1rem";
      case "lg": return "1.5rem";
      default: return "0";
    }
  }

  $: style = joinStyles([`padding: ${surfacePadding(padding)}`]);
</script>

<div
  class="surface"
  data-tone={tone}
  data-border={border}
  data-elevated={isElevated}
  role={asRole ?? undefined}
  aria-label={label ?? undefined}
  style={style}
>
  <slot />
</div>

<style>
  .surface {
    --pug-surface-fill: color-mix(in srgb, var(--pug-color-background-surface) 96%, transparent);
    --pug-surface: var(--pug-surface-fill);
    --pug-surface-border: color-mix(in srgb, var(--pug-color-border-subtle) 74%, transparent);
    --pug-surface-shadow: none;
    min-width: 0;
    min-height: 0;
    border: 0.0625rem solid var(--pug-surface-border);
    border-radius: var(--pug-treatment-surface-radius, var(--pug-radius-surface));
    background: var(--pug-surface-fill);
    box-shadow: var(--pug-surface-shadow);
  }

  .surface[data-tone="canvas"] {
    --pug-surface-fill: color-mix(in srgb, var(--pug-color-background-canvas) 98%, transparent);
  }

  .surface[data-tone="elevated"],
  .surface[data-elevated="true"] {
    --pug-surface-fill: color-mix(
      in srgb,
      var(--pug-color-background-elevated) 96%,
      var(--pug-color-background-panel)
    );
    --pug-surface-shadow: var(--pug-elevation-surface);
  }

  .surface[data-border="none"] {
    border-color: transparent;
  }

  .surface[data-border="default"] {
    --pug-surface-border: var(--pug-color-border-default);
  }
</style>
