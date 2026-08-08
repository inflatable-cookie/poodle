<script lang="ts">
  import "@inflatable-cookie/poodle-styles/card.css";
  import type { Snippet } from "svelte";
  import { getUiPresentation } from "./presentation";
  import type { ControlDensity, CardVariant } from "./types";

  interface Props {
    class?: string;
    variant?: CardVariant;
    layout?: "vertical" | "horizontal" | "compact";
    density?: ControlDensity | null;
    interactive?: boolean;
    selected?: boolean;
    media?: boolean;
    ariaLabel?: string | null;
    mediaContent?: Snippet;
    header?: Snippet;
    footer?: Snippet;
    children?: Snippet;
  }

  let {
    class: className = "",
    variant = "default",
    layout = "vertical",
    density = null,
    interactive = false,
    selected = false,
    media = false,
    ariaLabel = null,
    mediaContent,
    header,
    footer,
    children,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
</script>

<article
  class={`poodle-card ${className}`.trim()}
  data-variant={variant}
  data-layout={layout}
  data-density={resolvedDensity}
  data-interactive={interactive}
  data-selected={selected}
  aria-label={ariaLabel ?? undefined}
>
  {#if mediaContent}
    <div class="poodle-card__media" data-has-media={media}>
      {@render mediaContent()}
    </div>
  {/if}

  {#if header}
    <div class="poodle-card__header">
      {@render header()}
    </div>
  {/if}

  <div class="poodle-card__body">
    {@render children?.()}
  </div>

  {#if footer}
    <div class="poodle-card__footer">
      {@render footer()}
    </div>
  {/if}
</article>

