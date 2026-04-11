<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let summary: string | null = null;
  export let ariaLabel: string | null = null;
  /** When true, renders with border-top and background. */
  export let chrome = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "chrome";
  export let density: ControlDensity | null = null;

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
</script>

<footer
  class="status-bar"
  class:status-bar--chrome={chrome}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  aria-label={ariaLabel ?? summary ?? "Status"}
>
  <div class="status-bar__leading">
    {#if $$slots.leading}
      <slot name="leading" />
    {:else if summary}
      <span>{summary}</span>
    {/if}
  </div>

  {#if $$slots.trailing}
    <div class="status-bar__trailing">
      <slot name="trailing" />
    </div>
  {/if}
</footer>

<style>
  .status-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--poodle-space-inline-md);
    padding: 0.375rem 0.75rem;
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  .status-bar--chrome {
    border-top: 0.0625rem solid var(--poodle-color-border-subtle);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 94%, transparent);
  }

  .status-bar__leading,
  .status-bar__trailing {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
  }

  /* Size variants */
  .status-bar[data-size="xs"] { font-size: 0.6875rem; padding-block: 0.25rem; }
  .status-bar[data-size="sm"] { font-size: 0.75rem; padding-block: 0.3125rem; }
  .status-bar[data-size="lg"] { font-size: 0.875rem; padding-block: 0.4375rem; }
  .status-bar[data-size="xl"] { font-size: 0.9375rem; padding-block: 0.5rem; }

  /* Density variants */
  .status-bar[data-density="compact"] { padding-inline: 0.5rem; gap: 0.375rem; }
  .status-bar[data-density="comfortable"] { padding-inline: 1.125rem; gap: 1rem; }
</style>
