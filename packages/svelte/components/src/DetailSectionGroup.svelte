<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/detail-section-group.css";
  import type { Snippet } from "svelte";
  import { getUiPresentation } from "./presentation";
  import type { ControlDensity } from "./types";

  interface Props {
    density?: ControlDensity | null;
    layout?: "grid" | "stack";
    minColumnWidth?: string;
    itemMinColumnWidth?: string;
    maxColumns?: 2 | 3 | 4 | 5;
    ariaLabel?: string | null;
    children?: Snippet;
  }

  let {
    density = null,
    layout = "grid",
    minColumnWidth = "14rem",
    itemMinColumnWidth = "12rem",
    maxColumns = 4,
    ariaLabel = null,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const style = $derived(
    `--poodle-detail-section-group-min: ${minColumnWidth}; --poodle-detail-section-group-item-min: ${itemMinColumnWidth}`
  );
</script>

<div
  class="poodle-detail-section-group"
  data-density={resolvedDensity}
  data-layout={layout}
  data-max-columns={maxColumns}
  aria-label={ariaLabel ?? undefined}
  style={style}
>
  {@render children?.()}
</div>

