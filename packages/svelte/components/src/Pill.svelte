<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import { getPillContext, type PillTypography } from "./pill-context";
  import type { ControlDensity, SemanticControlSizeRole } from "./types";
  import type { PillAppearance, PillFont, PillSize, PillTone } from "./types";

  export let tone: PillTone = "neutral";
  export let appearance: PillAppearance = "solid";
  export let size: PillSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;
  export let font: PillFont = "normal";
  export let typography: PillTypography = "label";
  export let accent: string | null = null;
  export let muted = false;
  export let ariaLabel: string | null = null;

  const uiPresentation = getUiPresentation();
  const pillContext = getPillContext();

  $: resolvedSize = (pillContext?.size ?? size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole)) as PillSize;
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: resolvedTypography = pillContext?.typography ?? typography;
</script>

<span
  class="poodle-pill"
  data-tone={tone}
  data-appearance={appearance}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-font={font}
  data-typography={resolvedTypography}
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
    --poodle-pill-min-height: 1.25rem;
    --poodle-pill-padding-y: 0.1875rem;
    --poodle-pill-padding-x: 0.5rem;
    --poodle-pill-font-size: 0.6875rem;
    --poodle-pill-gap: 0.25rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--poodle-pill-gap);
    min-height: var(--poodle-pill-min-height);
    padding: var(--poodle-pill-padding-y) var(--poodle-pill-padding-x);
    border: 0.0625rem solid var(--poodle-pill-border);
    border-radius: 999px;
    background: var(--poodle-pill-fill);
    color: var(--poodle-pill-text);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-pill-font-size);
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
    --poodle-pill-min-height: 1rem;
    --poodle-pill-padding-y: 0.125rem;
    --poodle-pill-padding-x: 0.375rem;
    --poodle-pill-font-size: 0.625rem;
    --poodle-pill-gap: 0.1875rem;
  }

  .poodle-pill[data-size="xs"] {
    --poodle-pill-min-height: 0.875rem;
    --poodle-pill-padding-y: 0.0625rem;
    --poodle-pill-padding-x: 0.3125rem;
    --poodle-pill-font-size: 0.5625rem;
    --poodle-pill-gap: 0.15625rem;
  }

  .poodle-pill[data-size="lg"] {
    --poodle-pill-min-height: 1.375rem;
    --poodle-pill-padding-y: 0.25rem;
    --poodle-pill-padding-x: 0.625rem;
    --poodle-pill-font-size: 0.75rem;
  }

  .poodle-pill[data-size="xl"] {
    --poodle-pill-min-height: 1.5rem;
    --poodle-pill-padding-y: 0.3125rem;
    --poodle-pill-padding-x: 0.75rem;
    --poodle-pill-font-size: 0.8125rem;
  }

  .poodle-pill[data-typography="inherit"] {
    --poodle-pill-font-size: 0.7071em;
  }

  .poodle-pill[data-typography="inherit"][data-size="xs"] {
    --poodle-pill-font-size: 0.5786em;
    --poodle-pill-min-height: 1.5556em;
    --poodle-pill-padding-y: 0.1111em;
    --poodle-pill-padding-x: 0.5556em;
  }

  .poodle-pill[data-typography="inherit"][data-size="sm"] {
    --poodle-pill-font-size: 0.6429em;
    --poodle-pill-min-height: 1.6em;
    --poodle-pill-padding-y: 0.2em;
    --poodle-pill-padding-x: 0.6em;
  }

  .poodle-pill[data-typography="inherit"][data-size="md"] {
    --poodle-pill-font-size: 0.7071em;
    --poodle-pill-min-height: 1.8182em;
    --poodle-pill-padding-y: 0.2727em;
    --poodle-pill-padding-x: 0.7273em;
  }

  .poodle-pill[data-typography="inherit"][data-size="lg"] {
    --poodle-pill-font-size: 0.7714em;
    --poodle-pill-min-height: 1.8333em;
    --poodle-pill-padding-y: 0.3333em;
    --poodle-pill-padding-x: 0.8333em;
  }

  .poodle-pill[data-typography="inherit"][data-size="xl"] {
    --poodle-pill-font-size: 0.8357em;
    --poodle-pill-min-height: 1.8462em;
    --poodle-pill-padding-y: 0.3846em;
    --poodle-pill-padding-x: 0.9231em;
  }

  .poodle-pill[data-font="mono"] {
    font-family: var(--poodle-typography-code-family);
    letter-spacing: 0.02em;
  }

  .poodle-pill[data-appearance="badge"] {
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .poodle-pill[data-appearance="badge"][data-tone="neutral"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary));
    --poodle-pill-text: var(--poodle-color-text-secondary);
  }

  .poodle-pill[data-appearance="badge"][data-tone="success"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-status-success) 18%, transparent);
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-status-success) 42%, transparent);
  }

  .poodle-pill[data-appearance="badge"][data-tone="info"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-status-info) 18%, transparent);
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-status-info) 42%, transparent);
  }

  .poodle-pill[data-appearance="badge"][data-tone="warning"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-status-warning) 18%, transparent);
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-status-warning) 42%, transparent);
  }

  .poodle-pill[data-appearance="badge"][data-tone="danger"] {
    --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-status-danger) 18%, transparent);
    --poodle-pill-border: color-mix(in srgb, var(--poodle-color-status-danger) 42%, transparent);
  }

  .poodle-pill[data-muted="true"] {
    opacity: 0.72;
  }

  /* Density variants */
  .poodle-pill[data-density="compact"] {
    --poodle-pill-padding-y: 0;
    --poodle-pill-padding-x: 0.375rem;
    --poodle-pill-gap: 0.125rem;
  }

  .poodle-pill[data-density="comfortable"] {
    --poodle-pill-padding-y: 0;
    --poodle-pill-padding-x: 0.625rem;
    --poodle-pill-gap: 0.25rem;
  }

  .poodle-pill[data-typography="inherit"][data-density="compact"] {
    --poodle-pill-padding-y: 0;
    --poodle-pill-padding-x: 0.5455em;
  }

  .poodle-pill[data-typography="inherit"][data-density="comfortable"] {
    --poodle-pill-padding-y: 0;
    --poodle-pill-padding-x: 0.9091em;
  }

  .poodle-pill :global(svg),
  .poodle-pill :global(.poodle-icon) {
    flex-shrink: 0;
    width: 1em;
    height: 1em;
  }
</style>
