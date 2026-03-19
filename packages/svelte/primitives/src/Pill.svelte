<script lang="ts">
  import type { PillAppearance, PillFont, PillSize, PillTone } from "./types";

  export let tone: PillTone = "neutral";
  export let appearance: PillAppearance = "solid";
  export let size: PillSize = "xs";
  export let font: PillFont = "normal";
  export let isMuted = false;
  export let ariaLabel: string | null = null;
</script>

<span
  class="pill"
  data-tone={tone}
  data-appearance={appearance}
  data-size={size}
  data-font={font}
  data-muted={isMuted}
  aria-label={ariaLabel ?? undefined}
>
  <slot />
</span>

<style>
  .pill {
    --pug-pill-fill: color-mix(in srgb, var(--pug-color-background-surface) 90%, transparent);
    --pug-pill-border: color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent);
    --pug-pill-text: var(--pug-color-text-secondary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 1.25rem;
    padding: 0.1875rem 0.5rem;
    border: 0.0625rem solid var(--pug-pill-border);
    border-radius: 999px;
    background: var(--pug-pill-fill);
    color: var(--pug-pill-text);
    font-family: var(--pug-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    line-height: 1;
    white-space: nowrap;
  }

  .pill[data-tone="success"] {
    --pug-pill-fill: color-mix(in srgb, var(--pug-color-status-success) 14%, var(--pug-color-background-surface));
    --pug-pill-border: color-mix(in srgb, var(--pug-color-status-success) 38%, var(--pug-color-border-subtle));
    --pug-pill-text: var(--pug-color-text-primary);
  }

  .pill[data-tone="danger"] {
    --pug-pill-fill: color-mix(in srgb, var(--pug-color-status-danger) 14%, var(--pug-color-background-surface));
    --pug-pill-border: color-mix(in srgb, var(--pug-color-status-danger) 38%, var(--pug-color-border-subtle));
    --pug-pill-text: var(--pug-color-text-primary);
  }

  .pill[data-appearance="subtle"] {
    --pug-pill-fill: color-mix(in srgb, var(--pug-pill-fill) 50%, transparent);
  }

  .pill[data-size="xxs"] {
    min-height: 1rem;
    padding: 0.125rem 0.375rem;
    font-size: 0.625rem;
  }

  .pill[data-size="sm"] {
    min-height: 1.375rem;
    padding: 0.25rem 0.625rem;
    font-size: 0.75rem;
  }

  .pill[data-font="mono"] {
    font-family: var(--pug-typography-code-family);
    letter-spacing: 0.02em;
  }

  .pill[data-appearance="badge"] {
    --pug-pill-fill: color-mix(in srgb, var(--pug-color-accent-base) 18%, transparent);
    --pug-pill-border: transparent;
    --pug-pill-text: var(--pug-color-text-primary);
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .pill[data-appearance="badge"][data-tone="neutral"] {
    --pug-pill-fill: color-mix(in srgb, var(--pug-surface) 96%, var(--pug-color-text-primary));
    --pug-pill-text: var(--pug-color-text-secondary);
  }

  .pill[data-muted="true"] {
    opacity: 0.72;
  }
</style>
