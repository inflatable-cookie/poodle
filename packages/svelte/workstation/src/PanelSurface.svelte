<script lang="ts">
  import type { PanelVariant } from "./types";

  export let title: string | null = null;
  export let variant: PanelVariant = "standard";
  export let isActive = false;
  export let isElevated = false;
  export let hasHeader = true;
  export let bodyPadding: "none" | "sm" | "md" = "md";
  export let scrollMode: "panel" | "content" = "panel";
  export let ariaLabel: string | null = null;
</script>

<section
  class="panel-surface"
  data-variant={variant}
  data-active={isActive}
  data-elevated={isElevated}
  data-body-padding={bodyPadding}
  data-scroll-mode={scrollMode}
  aria-label={ariaLabel ?? title ?? undefined}
>
  {#if hasHeader && ($$slots.header || title)}
    <div class="panel-surface__header">
      {#if $$slots.header}
        <slot name="header" />
      {:else}
        <div class="panel-surface__header-title">
          <strong>{title}</strong>
        </div>
      {/if}
    </div>
  {/if}

  <div class="panel-surface__body">
    <slot />
  </div>
</section>

<style>
  .panel-surface {
    --pug-recipe-panel-surface-radius: var(--pug-treatment-surface-radius, var(--pug-radius-surface));
    --pug-recipe-panel-surface-fill: color-mix(
      in srgb,
      var(--pug-color-background-panel) 96%,
      transparent
    );
    --pug-recipe-panel-surface-border: color-mix(
      in srgb,
      var(--pug-color-border-subtle) 74%,
      transparent
    );
    --pug-recipe-panel-surface-shadow: none;
    --pug-recipe-panel-surface-divider: var(
      --pug-treatment-surface-divider,
      color-mix(in srgb, var(--pug-color-border-subtle) 56%, transparent)
    );
    --pug-recipe-panel-surface-header-fill: var(
      --pug-treatment-surface-header-fill,
      color-mix(in srgb, var(--pug-color-background-elevated) 36%, transparent)
    );
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
    min-height: 0;
    border: 0.0625rem solid var(--pug-treatment-surface-border, var(--pug-recipe-panel-surface-border));
    border-radius: calc(var(--pug-recipe-panel-surface-radius) - 0.0625rem);
    background: var(--pug-treatment-surface-fill, var(--pug-recipe-panel-surface-fill));
    box-shadow: var(--pug-treatment-surface-shadow, var(--pug-recipe-panel-surface-shadow));
  }

  .panel-surface[data-variant="utility"] {
    border-color: color-mix(in srgb, var(--pug-color-border-subtle) 50%, transparent);
    background: color-mix(in srgb, var(--pug-color-background-panel) 80%, transparent);
  }

  .panel-surface[data-variant="utility"] .panel-surface__header-title strong {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--pug-color-text-secondary);
  }

  .panel-surface[data-variant="focused"] {
    border-color: color-mix(in srgb, var(--pug-color-border-default) 90%, var(--pug-color-accent-base));
    background: var(--pug-color-background-panel);
  }

  .panel-surface[data-variant="focused"] .panel-surface__header-title strong {
    font-size: 0.9375rem;
    font-weight: 600;
  }

  .panel-surface[data-active="true"] {
    border-color: color-mix(
      in srgb,
      var(--pug-color-accent-base) 22%,
      var(--pug-treatment-surface-border, var(--pug-recipe-panel-surface-border))
    );
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, var(--pug-color-accent-base) 45%, transparent);
  }

  .panel-surface[data-elevated="true"] {
    border-color: var(
      --pug-treatment-surface-elevated-border,
      var(--pug-treatment-surface-border, var(--pug-recipe-panel-surface-border))
    );
    background: var(
      --pug-treatment-surface-elevated-fill,
      var(--pug-treatment-surface-fill, var(--pug-recipe-panel-surface-fill))
    );
    box-shadow: var(--pug-treatment-surface-elevated-shadow, var(--pug-elevation-surface));
  }

  .panel-surface__body {
    min-height: 0;
    overflow: auto;
  }

  .panel-surface[data-body-padding="sm"] .panel-surface__body {
    padding: 0.625rem;
  }

  .panel-surface[data-body-padding="md"] .panel-surface__body {
    padding: 0.875rem;
  }

  .panel-surface[data-scroll-mode="content"] .panel-surface__body {
    overflow: visible;
  }

  .panel-surface__header-title {
    padding: 0.625rem 0.75rem;
    border-bottom: 0.0625rem solid var(--pug-recipe-panel-surface-divider);
    background: var(--pug-recipe-panel-surface-header-fill);
  }

  .panel-surface__header-title strong {
    font-size: 0.875rem;
    line-height: 1.2;
  }
</style>
