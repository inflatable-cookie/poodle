<script lang="ts">
  import type { Snippet } from "svelte";
  import { scaleToSpace } from "./internal";
  import type { SpaceScale } from "./types";

  interface Props {
    legend?: string | null;
    description?: string | null;
    columns?: number;
    gap?: SpaceScale;
    span?: number | "full" | null;
    children?: Snippet;
  }

  let {
    legend = null,
    description = null,
    columns = 1,
    gap = "md",
    span = null,
    children,
  }: Props = $props();

  const gridStyle = $derived([
    `grid-template-columns: repeat(${columns}, minmax(0, 1fr))`,
    `row-gap: calc(${scaleToSpace(gap)} + 0.5rem)`,
    `column-gap: ${scaleToSpace(gap)}`,
  ].join("; "));
</script>

<fieldset
  class="poodle-fieldset"
  style={span ? (span === "full" ? "grid-column: 1 / -1" : `grid-column: span ${span}`) : undefined}
>
  {#if legend}
    <legend class="poodle-fieldset__legend">{legend}</legend>
  {/if}
  {#if description}
    <p class="poodle-fieldset__description">{description}</p>
  {/if}
  <div class="poodle-fieldset__fields" style={gridStyle}>
    {@render children?.()}
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

  .poodle-fieldset__description {
    margin: calc(var(--poodle-space-stack-sm) * -0.5) 0 var(--poodle-space-stack-md);
    color: var(--poodle-color-text-secondary);
    font-size: var(--poodle-typography-body-size, 0.875rem);
    line-height: var(--poodle-typography-body-lineHeight, 1.5);
  }

  .poodle-fieldset__fields {
    display: grid;
    align-items: start;
    min-width: 0;
  }
</style>
