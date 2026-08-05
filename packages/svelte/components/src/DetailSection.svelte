<script lang="ts">
  import "@poodle/styles/detail-section.css";
  import type { Snippet } from "svelte";
  import { getUiPresentation } from "./presentation.ts";
  import type { ControlDensity } from "./types.ts";

  interface Props {
    density?: ControlDensity | null;
    title?: string | null;
    description?: string | null;
    separated?: boolean;
    ariaLabel?: string | null;
    columns?: "auto" | 1 | 2 | 3 | 4;
    itemMinColumnWidth?: string | null;
    maxAutoColumns?: 2 | 3 | 4 | 5;
    actions?: Snippet;
    children?: Snippet;
  }

  let {
    density = null,
    title = null,
    description = null,
    separated = true,
    ariaLabel = null,
    columns = "auto",
    itemMinColumnWidth = null,
    maxAutoColumns = 4,
    actions,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const style = $derived(
    itemMinColumnWidth ? `--poodle-detail-section-item-min: ${itemMinColumnWidth}` : undefined
  );
</script>

<section
  class="poodle-detail-section"
  data-density={resolvedDensity}
  data-separated={separated}
  data-columns={columns}
  data-max-auto-columns={maxAutoColumns}
  aria-label={ariaLabel ?? undefined}
  style={style}
>
  {#if title || description || actions}
    <div class="poodle-detail-section__header">
      <div class="poodle-detail-section__title-block">
        {#if title}
          <h3 class="poodle-detail-section__title">{title}</h3>
        {/if}
        {#if description}
          <p class="poodle-detail-section__description">{description}</p>
        {/if}
      </div>
      {#if actions}
        <div class="poodle-detail-section__actions">
          {@render actions()}
        </div>
      {/if}
    </div>
  {/if}
  <div class="poodle-detail-section__body">
    {@render children?.()}
  </div>
</section>

