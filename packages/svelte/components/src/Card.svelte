<script lang="ts">
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

<style>
  .poodle-card {
    --poodle-recipe-card-radius: var(--poodle-treatment-surface-radius, var(--poodle-radius-surface));
    --poodle-recipe-card-fill: color-mix(
      in srgb,
      var(--poodle-color-background-panel) 10%,
      var(--poodle-color-background-elevated)
    );
    --poodle-recipe-card-border: color-mix(
      in srgb,
      var(--poodle-color-border-subtle) 18%,
      transparent
    );
    --poodle-recipe-card-shadow:
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 18%, transparent);
    --poodle-recipe-card-divider: color-mix(
      in srgb,
      var(--poodle-color-border-subtle) 52%,
      transparent
    );
    --poodle-recipe-card-hover-fill: var(
      --poodle-treatment-surface-hover-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 94%, var(--poodle-color-background-panel))
    );
    --poodle-recipe-card-hover-border: var(
      --poodle-treatment-surface-hover-border,
      color-mix(in srgb, var(--poodle-color-accent-base) 28%, var(--poodle-color-border-subtle))
    );
    --poodle-recipe-card-hover-shadow: var(--poodle-treatment-surface-hover-shadow, var(--poodle-recipe-card-shadow));
    --poodle-card-gap: var(--poodle-space-stack-md);
    --poodle-card-padding-block: var(--poodle-space-panel-x);
    --poodle-card-padding-inline: var(--poodle-space-panel-x);
    --poodle-card-footer-padding-top: var(--poodle-space-stack-sm);
    display: grid;
    align-content: start;
    gap: var(--poodle-card-gap);
    padding: var(--poodle-card-padding-block) var(--poodle-card-padding-inline);
    border: 0.0625rem solid var(--poodle-recipe-card-border);
    border-radius: var(--poodle-recipe-card-radius);
    background: var(
      --poodle-treatment-surface-fill,
      var(--poodle-recipe-card-fill)
    );
    box-shadow: var(--poodle-treatment-surface-shadow, var(--poodle-recipe-card-shadow));
  }

  .poodle-card[data-variant="outlined"] {
    border-color: color-mix(in srgb, var(--poodle-color-border-default) 76%, transparent);
  }

  .poodle-card[data-variant="elevated"] {
    border-radius: var(--poodle-treatment-surface-elevated-radius, var(--poodle-recipe-card-radius));
    border-color: color-mix(
      in srgb,
      var(--poodle-treatment-surface-elevated-border, var(--poodle-color-border-default)) 82%,
      var(--poodle-color-border-default)
    );
    background: var(
      --poodle-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--poodle-color-background-elevated) 98%, var(--poodle-color-background-panel))
    );
    box-shadow:
      0 1.125rem 2.5rem color-mix(in srgb, black 38%, transparent),
      0 0.375rem 0.875rem color-mix(in srgb, black 24%, transparent),
      inset 0 0.0625rem 0 color-mix(in srgb, var(--poodle-color-text-inverse) 10%, transparent),
      0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 12%, transparent);
  }

  :global([data-theme="light"]) .poodle-card[data-variant="elevated"] {
    box-shadow:
      0 0.875rem 1.75rem rgba(49, 66, 85, 0.1),
      0 0.25rem 0.625rem rgba(49, 66, 85, 0.06),
      inset 0 0.0625rem 0 rgba(255, 255, 255, 0.72),
      0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 10%, transparent);
  }

  .poodle-card[data-selected="true"] {
    border-color: var(--poodle-color-accent-base);
    box-shadow:
      0 0 0 0.0625rem var(--poodle-color-accent-base),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }

  .poodle-card[data-interactive="true"] {
    cursor: pointer;
  }

  .poodle-card[data-interactive="true"]:hover {
    border-color: var(--poodle-recipe-card-hover-border);
    background: var(--poodle-recipe-card-hover-fill);
    box-shadow: var(--poodle-recipe-card-hover-shadow);
  }

  .poodle-card[data-interactive="true"][data-selected="true"]:hover {
    border-color: var(--poodle-color-accent-base);
    box-shadow:
      0 0 0 0.0625rem var(--poodle-color-accent-base),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }

  .poodle-card[data-layout="horizontal"] {
    grid-template-columns: auto 1fr;
    grid-template-rows: auto;
  }

  .poodle-card[data-layout="horizontal"] .poodle-card__media {
    grid-row: 1 / -1;
    width: 8rem;
  }

  .poodle-card[data-layout="compact"] {
    --poodle-card-padding-block: var(--poodle-space-panel-y-sm, 0.5rem);
    --poodle-card-padding-inline: var(--poodle-space-panel-x-sm, 0.625rem);
    --poodle-card-gap: var(--poodle-space-stack-sm);
  }

  .poodle-card[data-density="compact"] {
    --poodle-card-gap: 0.625rem;
    --poodle-card-padding-block: 0.75rem;
    --poodle-card-padding-inline: 0.75rem;
    --poodle-card-footer-padding-top: 0.625rem;
  }

  .poodle-card[data-density="default"] {
    --poodle-card-gap: var(--poodle-space-stack-md);
    --poodle-card-padding-block: var(--poodle-space-panel-x);
    --poodle-card-padding-inline: var(--poodle-space-panel-x);
    --poodle-card-footer-padding-top: var(--poodle-space-stack-sm);
  }

  .poodle-card[data-density="comfortable"] {
    --poodle-card-gap: 1rem;
    --poodle-card-padding-block: 1rem;
    --poodle-card-padding-inline: 1rem;
    --poodle-card-footer-padding-top: 0.875rem;
  }

  .poodle-card__media {
    overflow: hidden;
    border-radius: calc(var(--poodle-recipe-card-radius) - 0.1875rem);
  }

  .poodle-card__footer {
    padding-top: var(--poodle-card-footer-padding-top);
    border-top: 0.0625rem solid var(--poodle-treatment-surface-divider, var(--poodle-recipe-card-divider));
  }
</style>
