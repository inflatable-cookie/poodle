<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, SemanticControlSizeRole } from "./types";
  import type { PillAppearance, PillFont, PillSize, PillTone } from "./types";

  export let tone: PillTone = "neutral";
  export let appearance: PillAppearance = "solid";
  export let size: PillSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;
  export let font: PillFont = "normal";
  export let accent: string | null = null;
  export let muted = false;
  export let ariaLabel: string | null = null;

  const uiPresentation = getUiPresentation();

  $: resolvedSize = (size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole)) as PillSize;
  $: resolvedDensity = density ?? $uiPresentation.density;
</script>

<span
  class="poodle-pill"
  data-tone={tone}
  data-appearance={appearance}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-font={font}
  data-muted={muted}
  data-accent={accent ? "custom" : undefined}
  aria-label={ariaLabel ?? undefined}
  style:--poodle-pill-accent={accent ?? undefined}
>
  <slot />
</span>

<style>
  .poodle-pill {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-background-surface) 90%, transparent);
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent);
    --poodle-pill-text: var(--poodle-color-text-secondary);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 1.25rem;
    padding: 0.1875rem 0.5rem;
    border: 0.0625rem solid var(--poodle-pill-border);
    border-radius: 999px;
    background: var(--poodle-pill-fill);
    color: var(--poodle-pill-text);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.6875rem;
    font-weight: 600;
    line-height: 1;
    white-space: nowrap;
  }

  .poodle-pill[data-tone="success"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-status-success) 14%, var(--poodle-color-background-surface));
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-status-success) 38%, var(--poodle-color-border-subtle));
    --poodle-pill-text: var(--poodle-color-text-primary);
  }

  .poodle-pill[data-tone="info"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-status-info) 14%, var(--poodle-color-background-surface));
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-status-info) 38%, var(--poodle-color-border-subtle));
    --poodle-pill-text: var(--poodle-color-text-primary);
  }

  .poodle-pill[data-tone="warning"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-status-warning) 14%, var(--poodle-color-background-surface));
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-status-warning) 38%, var(--poodle-color-border-subtle));
    --poodle-pill-text: var(--poodle-color-text-primary);
  }

  .poodle-pill[data-tone="danger"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-status-danger) 14%, var(--poodle-color-background-surface));
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-status-danger) 38%, var(--poodle-color-border-subtle));
    --poodle-pill-text: var(--poodle-color-text-primary);
  }

  .poodle-pill[data-accent="custom"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-pill-accent) 18%, rgba(148, 163, 184, 0.08));
    --poodle-pill-border: color-mix(in srgb, var(--poodle-pill-accent) 30%, rgba(148, 163, 184, 0.12));
    --poodle-pill-text: color-mix(in srgb, var(--poodle-pill-accent) 88%, white);
  }

  .poodle-pill[data-appearance="subtle"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-pill-fill) 50%, transparent);
  }

  .poodle-pill[data-size="sm"] {
    min-height: 1rem;
    padding: 0.125rem 0.375rem;
    font-size: 0.625rem;
  }

  .poodle-pill[data-size="xs"] {
    min-height: 0.875rem;
    padding: 0.0625rem 0.3125rem;
    font-size: 0.5625rem;
  }

  .poodle-pill[data-size="lg"] {
    min-height: 1.375rem;
    padding: 0.25rem 0.625rem;
    font-size: 0.75rem;
  }

  .poodle-pill[data-size="xl"] {
    min-height: 1.5rem;
    padding: 0.3125rem 0.75rem;
    font-size: 0.8125rem;
  }

  .poodle-pill[data-font="mono"] {
    font-family: var(--poodle-typography-code-family);
    letter-spacing: 0.02em;
  }

  .poodle-pill[data-appearance="badge"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-accent-base) 18%, transparent);
    --poodle-pill-border: transparent;
    --poodle-pill-text: var(--poodle-color-text-primary);
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .poodle-pill[data-appearance="badge"][data-tone="neutral"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary));
    --poodle-pill-text: var(--poodle-color-text-secondary);
  }

  .poodle-pill[data-muted="true"] {
    opacity: 0.72;
  }

  /* Density variants */
  .poodle-pill[data-density="compact"] { padding: 0 0.375rem; gap: 0.125rem; }
  .poodle-pill[data-density="comfortable"] { padding: 0 0.625rem; gap: 0.25rem; }
</style>
