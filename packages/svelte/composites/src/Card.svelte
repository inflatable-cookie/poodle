<script lang="ts">
  import type { CardVariant } from "./types";

  export let variant: CardVariant = "default";
  export let isInteractive = false;
  export let hasMedia = false;
  export let ariaLabel: string | null = null;
</script>

<article
  class="card"
  data-variant={variant}
  data-interactive={isInteractive}
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
    display: grid;
    gap: var(--pug-space-stack-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 1px solid transparent;
    border-radius: var(--pug-radius-surface);
    background: color-mix(in srgb, var(--pug-color-background-panel) 98%, var(--pug-color-background-elevated));
    box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--pug-color-border-subtle) 18%, transparent);
  }

  .card[data-variant="outlined"] {
    border-color: var(--pug-color-border-default);
  }

  .card[data-variant="elevated"] {
    box-shadow: var(--pug-elevation-surface);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 94%, transparent);
  }

  .card[data-interactive="true"] {
    cursor: pointer;
  }

  .card[data-interactive="true"]:hover {
    border-color: color-mix(in srgb, var(--pug-color-accent-base) 28%, var(--pug-color-border-subtle));
    background: color-mix(in srgb, var(--pug-color-background-elevated) 94%, transparent);
  }

  .card__media {
    overflow: hidden;
    border-radius: calc(var(--pug-radius-surface) - 3px);
  }

  .card__footer {
    padding-top: var(--pug-space-stack-sm);
    border-top: 1px solid var(--pug-color-border-subtle);
  }

  :global([data-theme="light"]) .card {
    border-color: color-mix(in srgb, var(--pug-color-border-default) 16%, transparent);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 96%, var(--pug-color-background-panel));
    box-shadow:
      inset 0 0 0 1px color-mix(in srgb, var(--pug-color-border-subtle) 42%, transparent),
      0 8px 20px rgba(49, 66, 85, 0.04);
  }

  :global([data-theme="light"]) .card[data-variant="outlined"] {
    border-color: color-mix(in srgb, var(--pug-color-border-default) 24%, transparent);
  }

  :global([data-theme="light"]) .card[data-variant="elevated"] {
    box-shadow:
      0 12px 28px rgba(49, 66, 85, 0.06),
      inset 0 1px 0 rgba(255, 255, 255, 0.74);
  }
</style>
