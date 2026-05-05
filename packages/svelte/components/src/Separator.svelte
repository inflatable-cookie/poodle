<script lang="ts">
  import type { Orientation, SeparatorTone } from "./types";

  export let orientation: Orientation = "horizontal";
  export let decorative = true;
  export let tone: SeparatorTone = "subtle";
</script>

<div
  class="poodle-separator"
  data-orientation={orientation}
  data-tone={tone}
  role={decorative ? undefined : "separator"}
  aria-hidden={decorative ? "true" : undefined}
  aria-orientation={decorative ? undefined : orientation}
></div>

<style>
  .poodle-separator {
    --poodle-separator-color: color-mix(in srgb, var(--poodle-color-border-subtle) 72%, transparent);
    flex: 0 0 auto;
    background: var(--poodle-separator-color);
  }

  .poodle-separator[data-tone="default"] {
    --poodle-separator-color: var(--poodle-color-border-default);
  }

  .poodle-separator[data-orientation="horizontal"] {
    width: 100%;
    min-height: 0.0625rem;
  }

  .poodle-separator[data-orientation="vertical"] {
    width: 0.0625rem;
    align-self: stretch;
    min-height: 100%;
  }
</style>
