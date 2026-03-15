<script lang="ts">
  import Icon from "./Icon.svelte";
  import type { StatusTone } from "./types";

  export let tone: StatusTone | "neutral" = "neutral";
  export let title: string | null = null;
  export let ariaLabel: string | null = null;

  const toneIcon: Record<string, string> = {
    success: "check",
    warning: "alert-triangle",
    danger: "x-circle",
    pending: "loader",
    info: "info",
    neutral: "info",
  };
</script>

<section class="callout" data-tone={tone} aria-label={ariaLabel ?? undefined}>
  <span class="callout__icon" aria-hidden="true">
    {#if $$slots.icon}
      <slot name="icon" />
    {:else}
      <Icon name={toneIcon[tone] ?? "info"} size="sm" />
    {/if}
  </span>

  <div class="callout__content">
    {#if title}
      <strong>{title}</strong>
    {/if}
    <slot />
  </div>
</section>

<style>
  .callout {
    --pug-callout-fill: color-mix(in srgb, var(--pug-color-background-panel) 94%, transparent);
    --pug-callout-border: color-mix(in srgb, var(--pug-color-border-subtle) 88%, transparent);
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: start;
    gap: var(--pug-space-inline-md);
    padding: var(--pug-space-panel-y) var(--pug-space-panel-x);
    border: 0.0625rem solid var(--pug-callout-border);
    border-radius: var(--pug-radius-surface);
    background: var(--pug-callout-fill);
    color: var(--pug-color-text-primary);
  }

  .callout[data-tone="info"] {
    --pug-callout-fill: color-mix(in srgb, var(--pug-color-accent-base) 10%, var(--pug-color-background-panel));
    --pug-callout-border: color-mix(in srgb, var(--pug-color-accent-base) 34%, var(--pug-color-border-default));
  }

  .callout[data-tone="success"] {
    --pug-callout-fill: color-mix(in srgb, var(--pug-color-status-success) 10%, var(--pug-color-background-panel));
    --pug-callout-border: color-mix(in srgb, var(--pug-color-status-success) 34%, var(--pug-color-border-default));
  }

  .callout[data-tone="warning"] {
    --pug-callout-fill: color-mix(in srgb, var(--pug-color-status-warning) 10%, var(--pug-color-background-panel));
    --pug-callout-border: color-mix(in srgb, var(--pug-color-status-warning) 34%, var(--pug-color-border-default));
  }

  .callout[data-tone="danger"] {
    --pug-callout-fill: color-mix(in srgb, var(--pug-color-status-danger) 10%, var(--pug-color-background-panel));
    --pug-callout-border: color-mix(in srgb, var(--pug-color-status-danger) 34%, var(--pug-color-border-default));
  }

  .callout[data-tone="pending"] {
    --pug-callout-fill: color-mix(in srgb, var(--pug-color-accent-base) 8%, var(--pug-color-background-panel));
    --pug-callout-border: color-mix(in srgb, var(--pug-color-accent-base) 26%, var(--pug-color-border-default));
  }

  .callout__icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.375rem;
    height: 1.375rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--pug-color-background-surface) 78%, transparent);
    font-family: var(--pug-typography-code-family);
    font-size: 0.75rem;
    font-weight: 700;
    line-height: 1;
  }

  .callout__content {
    display: grid;
    gap: 0.25rem;
    min-width: 0;
  }

  .callout__content :global(p) {
    margin: 0;
    color: var(--pug-color-text-secondary);
  }

  .callout__content strong {
    font-family: var(--pug-typography-label-family);
    font-size: var(--pug-typography-label-size);
    line-height: var(--pug-typography-label-lineHeight);
  }
</style>
