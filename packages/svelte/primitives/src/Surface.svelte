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
      case "sm": return "var(--flint-space-panel-y)";
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
    --flint-surface-fill: var(
      --flint-treatment-surface-fill,
      color-mix(in srgb, var(--flint-color-background-surface) 96%, transparent)
    );
    --flint-surface: var(--flint-surface-fill);
    --flint-surface-border: var(
      --flint-treatment-surface-border,
      color-mix(in srgb, var(--flint-color-border-subtle) 74%, transparent)
    );
    --flint-surface-shadow: var(--flint-treatment-surface-shadow, none);
    min-width: 0;
    min-height: 0;
    border: 0.0625rem solid var(--flint-surface-border);
    border-radius: var(--flint-treatment-surface-radius, var(--flint-radius-surface));
    background: var(--flint-surface-fill);
    box-shadow: var(--flint-surface-shadow);
  }

  .surface[data-tone="canvas"] {
    --flint-surface-fill: color-mix(in srgb, var(--flint-color-background-canvas) 98%, transparent);
  }

  .surface[data-tone="elevated"],
  .surface[data-elevated="true"] {
    --flint-surface-fill: var(
      --flint-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--flint-color-background-elevated) 96%, var(--flint-color-background-panel))
    );
    --flint-surface-border: var(
      --flint-treatment-surface-elevated-border,
      color-mix(in srgb, var(--flint-color-border-subtle) 74%, transparent)
    );
    --flint-surface-shadow: var(
      --flint-treatment-surface-elevated-shadow,
      var(--flint-elevation-surface)
    );
  }

  .surface[data-border="none"] {
    border-color: transparent;
  }

  .surface[data-border="default"] {
    --flint-surface-border: var(--flint-color-border-default);
  }
</style>
