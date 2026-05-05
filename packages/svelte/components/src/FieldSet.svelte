<script lang="ts">
  import { scaleToSpace } from "./internal";
  import type { SpaceScale } from "./types";

  export let legend: string | null = null;
  export let columns: number = 1;
  export let gap: SpaceScale = "md";
  export let span: number | "full" | null = null;

  $: gridStyle = [
    `grid-template-columns: repeat(${columns}, minmax(0, 1fr))`,
    `gap: ${scaleToSpace(gap)}`,
  ].join("; ");
</script>

<fieldset
  class="poodle-fieldset"
  style={span ? (span === "full" ? "grid-column: 1 / -1" : `grid-column: span ${span}`) : undefined}
>
  {#if legend}
    <legend class="poodle-fieldset__legend">{legend}</legend>
  {/if}
  <div class="poodle-fieldset__fields" style={gridStyle}>
    <slot />
  </div>
</fieldset>

<style>
  .poodle-fieldset {
    border: none;
    padding: 0;
    margin: 0;
    min-width: 0;
  }

  .poodle-fieldset__legend {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.12em;
    line-height: 1.5;
    text-transform: uppercase;
    padding: 0;
    margin-bottom: var(--poodle-space-stack-sm);
  }

  .poodle-fieldset__fields {
    display: grid;
    align-items: start;
    min-width: 0;
  }
</style>
