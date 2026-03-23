<script lang="ts">
  import type { CardVariant } from "./types";

  export let variant: CardVariant = "default";
  export let layout: "vertical" | "horizontal" | "compact" = "vertical";
  export let isInteractive = false;
  export let isSelected = false;
  export let hasMedia = false;
  export let ariaLabel: string | null = null;
</script>

<article
  class="card"
  data-variant={variant}
  data-layout={layout}
  data-interactive={isInteractive}
  data-selected={isSelected}
  aria-label={ariaLabel ?? undefined}
>
  {#if $$slots.media}
    <div class="card__media" data-has-media={hasMedia}>
      <slot name="media" />
    </div>
  {/if}

  {#if $$slots.header}
    <div class="card__header">
      <slot name="header" />
    </div>
  {/if}

  <div class="card__body">
    <slot />
  </div>

  {#if $$slots.footer}
    <div class="card__footer">
      <slot name="footer" />
    </div>
  {/if}
</article>

<style>
  .card {
    --flint-recipe-card-radius: var(--flint-treatment-surface-radius, var(--flint-radius-surface));
    --flint-recipe-card-fill: color-mix(
      in srgb,
      var(--flint-color-background-panel) 98%,
      var(--flint-color-background-elevated)
    );
    --flint-recipe-card-border: color-mix(
      in srgb,
      var(--flint-color-border-subtle) 18%,
      transparent
    );
    --flint-recipe-card-shadow:
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--flint-color-border-subtle) 18%, transparent);
    --flint-recipe-card-divider: color-mix(
      in srgb,
      var(--flint-color-border-subtle) 52%,
      transparent
    );
    --flint-recipe-card-hover-fill: var(
      --flint-treatment-surface-hover-fill,
      color-mix(in srgb, var(--flint-color-background-elevated) 94%, var(--flint-color-background-panel))
    );
    --flint-recipe-card-hover-border: var(
      --flint-treatment-surface-hover-border,
      color-mix(in srgb, var(--flint-color-accent-base) 28%, var(--flint-color-border-subtle))
    );
    --flint-recipe-card-hover-shadow: var(--flint-treatment-surface-hover-shadow, var(--flint-recipe-card-shadow));
    display: grid;
    align-content: start;
    gap: var(--flint-space-stack-md);
    padding: var(--flint-space-panel-x);
    border: 0.0625rem solid var(--flint-recipe-card-border);
    border-radius: var(--flint-recipe-card-radius);
    background: var(
      --flint-treatment-surface-fill,
      color-mix(in srgb, var(--flint-surface) 88%, var(--flint-color-text-primary))
    );
    --flint-surface: var(--flint-treatment-surface-fill, var(--flint-recipe-card-fill));
    box-shadow: var(--flint-treatment-surface-shadow, var(--flint-recipe-card-shadow));
  }

  .card[data-variant="outlined"] {
    border-color: color-mix(in srgb, var(--flint-color-border-default) 76%, transparent);
  }

  .card[data-variant="elevated"] {
    border-radius: var(--flint-treatment-surface-elevated-radius, var(--flint-recipe-card-radius));
    border-color: color-mix(
      in srgb,
      var(--flint-treatment-surface-elevated-border, var(--flint-color-border-default)) 82%,
      var(--flint-color-border-default)
    );
    background: var(
      --flint-treatment-surface-elevated-fill,
      color-mix(in srgb, var(--flint-color-background-elevated) 98%, var(--flint-color-background-panel))
    );
    box-shadow:
      0 1.125rem 2.5rem color-mix(in srgb, black 38%, transparent),
      0 0.375rem 0.875rem color-mix(in srgb, black 24%, transparent),
      inset 0 0.0625rem 0 color-mix(in srgb, var(--flint-color-text-inverse) 10%, transparent),
      0 0 0 0.0625rem color-mix(in srgb, var(--flint-color-border-default) 12%, transparent);
  }

  :global([data-theme="light"]) .card[data-variant="elevated"] {
    box-shadow:
      0 0.875rem 1.75rem rgba(49, 66, 85, 0.1),
      0 0.25rem 0.625rem rgba(49, 66, 85, 0.06),
      inset 0 0.0625rem 0 rgba(255, 255, 255, 0.72),
      0 0 0 0.0625rem color-mix(in srgb, var(--flint-color-border-default) 10%, transparent);
  }

  .card[data-selected="true"] {
    border-color: var(--flint-color-accent-base);
    box-shadow:
      0 0 0 0.0625rem var(--flint-color-accent-base),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--flint-color-accent-base) 12%, transparent);
  }

  .card[data-interactive="true"] {
    cursor: pointer;
  }

  .card[data-interactive="true"]:hover {
    border-color: var(--flint-recipe-card-hover-border);
    background: var(--flint-recipe-card-hover-fill);
    box-shadow: var(--flint-recipe-card-hover-shadow);
  }

  .card[data-interactive="true"][data-selected="true"]:hover {
    border-color: var(--flint-color-accent-base);
    box-shadow:
      0 0 0 0.0625rem var(--flint-color-accent-base),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--flint-color-accent-base) 12%, transparent);
  }

  .card[data-layout="horizontal"] {
    grid-template-columns: auto 1fr;
    grid-template-rows: auto;
  }

  .card[data-layout="horizontal"] .card__media {
    grid-row: 1 / -1;
    width: 8rem;
  }

  .card[data-layout="compact"] {
    padding: var(--flint-space-panel-y-sm, 0.5rem) var(--flint-space-panel-x-sm, 0.625rem);
    gap: var(--flint-space-stack-sm);
  }

  .card__media {
    overflow: hidden;
    border-radius: calc(var(--flint-recipe-card-radius) - 0.1875rem);
  }

  .card__footer {
    padding-top: var(--flint-space-stack-sm);
    border-top: 0.0625rem solid var(--flint-treatment-surface-divider, var(--flint-recipe-card-divider));
  }
</style>
