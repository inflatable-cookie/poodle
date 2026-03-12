<script lang="ts">
  import type { Orientation, SeparatorTone } from "./types";

  export let orientation: Orientation = "horizontal";
  export let decorative = true;
  export let tone: SeparatorTone = "subtle";
</script>

<div
  class="separator"
  data-orientation={orientation}
  data-tone={tone}
  role={decorative ? undefined : "separator"}
  aria-hidden={decorative ? "true" : undefined}
  aria-orientation={decorative ? undefined : orientation}
></div>

<style>
  .separator {
    --pug-separator-color: color-mix(in srgb, var(--pug-color-border-subtle) 72%, transparent);
    flex: 0 0 auto;
    background: var(--pug-separator-color);
  }

  .separator[data-tone="default"] {
    --pug-separator-color: var(--pug-color-border-default);
  }

  .separator[data-orientation="horizontal"] {
    width: 100%;
    min-height: 0.0625rem;
  }

  .separator[data-orientation="vertical"] {
    width: 0.0625rem;
    align-self: stretch;
    min-height: 100%;
  }
</style>
