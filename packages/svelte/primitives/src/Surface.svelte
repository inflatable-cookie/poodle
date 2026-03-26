<script lang="ts">
  import { joinStyles } from "./internal";

  import type { SpaceScale, SurfaceBorder, SurfaceTone } from "./types";

  export let tone: SurfaceTone = "panel";
  export let border: SurfaceBorder = "subtle";
  export let padding: SpaceScale = "md";
  export let elevated = false;
  export let asRole: "region" | "group" | null = null;
  export let label: string | null = null;

  function surfacePadding(scale: SpaceScale): string {
    switch (scale) {
      case "sm": return "var(--poodle-space-panel-y)";
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
  data-elevated={elevated}
  role={asRole ?? undefined}
  aria-label={label ?? undefined}
  style={style}
>
  <slot />
</div>

<style>
  .surface {
    --poodle-surface-fill: var(
      --poodle-treatment-surface-fill,
      color-mix(in srgb, var(--poodle-color-background-surface) 96%, transparent)
    );
    --poodle-surface: var(--poodle-surface-fill);
    --poodle-surface-border: var(
      --poodle-treatment-surface-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)
    );
    --poodle-surface-shadow: var(--poodle-treatment-surface-shadow, none);
    min-width: 0;
    min-height: 0;
    border: 0.0625rem solid var(--poodle-surface-border);
    border-radius: var(--poodle-treatment-surface-radius, var(--poodle-radius-surface));
    background: var(--poodle-surface-fill);
    box-shadow: var(--poodle-surface-shadow);
  }

  .surface[data-tone="canvas"] {
    --poodle-surface-fill: color-mix(in srgb, var(--poodle-color-background-canvas) 98%, transparent);
  }

  .surface[data-tone="elevated"],
  .surface[data-elevated="true"] {
    --poodle-surface-fill: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 96%, var(--poodle-color-background-panel))
    );
    --poodle-surface-border: var(
      --poodle-treatment-surface-elevated-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)
    );
    --poodle-surface-shadow: var(
      --poodle-treatment-surface-elevated-shadow,
      var(--poodle-elevation-surface)
    );
  }

  .surface[data-border="none"] {
    border-color: transparent;
  }

  .surface[data-border="default"] {
    --poodle-surface-border: var(--poodle-color-border-default);
  }
</style>
