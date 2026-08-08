<script lang="ts">
  import "@inflatable-cookie/poodle-styles/field-set.css";
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

