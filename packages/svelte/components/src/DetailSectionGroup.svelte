<script lang="ts">
  import type { Snippet } from "svelte";
  import { getUiPresentation } from "./presentation";
  import type { ControlDensity } from "./types";

  interface Props {
    density?: ControlDensity | null;
    minColumnWidth?: string;
    itemMinColumnWidth?: string;
    maxColumns?: 2 | 3 | 4 | 5;
    ariaLabel?: string | null;
    children?: Snippet;
  }

  let {
    density = null,
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
  data-max-columns={maxColumns}
  aria-label={ariaLabel ?? undefined}
  style={style}
>
  {@render children?.()}
</div>

<style>
  .poodle-detail-section-group {
    --poodle-detail-section-group-gap: var(--poodle-space-stack-lg);

    display: grid;
    grid-template-columns: repeat(
      auto-fit,
      minmax(min(100%, var(--poodle-detail-section-group-min)), 1fr)
    );
    gap: var(--poodle-detail-section-group-gap);
    align-items: start;
    container-type: inline-size;
  }

  .poodle-detail-section-group[data-max-columns="2"] {
    grid-template-columns: repeat(
      auto-fit,
      minmax(
        min(
          100%,
          max(
            var(--poodle-detail-section-group-min),
            calc((100% - var(--poodle-detail-section-group-gap)) / 2)
          )
        ),
        1fr
      )
    );
  }

  .poodle-detail-section-group[data-max-columns="3"] {
    grid-template-columns: repeat(
      auto-fit,
      minmax(
        min(
          100%,
          max(
            var(--poodle-detail-section-group-min),
            calc((100% - (var(--poodle-detail-section-group-gap) * 2)) / 3)
          )
        ),
        1fr
      )
    );
  }

  .poodle-detail-section-group[data-max-columns="4"] {
    grid-template-columns: repeat(
      auto-fit,
      minmax(
        min(
          100%,
          max(
            var(--poodle-detail-section-group-min),
            calc((100% - (var(--poodle-detail-section-group-gap) * 3)) / 4)
          )
        ),
        1fr
      )
    );
  }

  .poodle-detail-section-group[data-max-columns="5"] {
    grid-template-columns: repeat(
      auto-fit,
      minmax(
        min(
          100%,
          max(
            var(--poodle-detail-section-group-min),
            calc((100% - (var(--poodle-detail-section-group-gap) * 4)) / 5)
          )
        ),
        1fr
      )
    );
  }

  .poodle-detail-section-group :global(.poodle-detail-section__body) {
    grid-template-columns: repeat(
      auto-fit,
      minmax(min(100%, var(--poodle-detail-section-group-item-min)), 1fr)
    );
  }

  .poodle-detail-section-group[data-density="compact"] {
    --poodle-detail-section-group-gap: var(--poodle-space-stack-md);
  }

  .poodle-detail-section-group[data-density="default"] {
    --poodle-detail-section-group-gap: var(--poodle-space-stack-lg);
  }

  .poodle-detail-section-group[data-density="comfortable"] {
    --poodle-detail-section-group-gap: calc(var(--poodle-space-stack-lg) + 0.25rem);
  }

  @container (max-width: 34rem) {
    .poodle-detail-section-group {
      grid-template-columns: 1fr;
    }
  }
</style>
