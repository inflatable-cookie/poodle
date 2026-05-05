<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone } from "./types";

  export let status: StatusTone = "neutral";
  export let label: string | null = null;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const uiPresentation = getUiPresentation();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
</script>

<span class="poodle-status-indicator" data-status={status} data-size={resolvedSize} data-density={resolvedDensity} aria-label={ariaLabel ?? undefined}>
  <span class="poodle-status-indicator__dot" aria-hidden="true"></span>
  {#if label}
    <span class="poodle-status-indicator__label">{label}</span>
  {:else}
    <slot />
  {/if}
</span>

<style>
  .poodle-status-indicator {
    --poodle-status-color: var(--poodle-color-text-secondary);
    display: inline-flex;
    align-items: center;
    gap: 0.4375rem;
    color: var(--poodle-color-text-primary);
    min-width: 0;
  }

  .poodle-status-indicator[data-status="info"] {
    --poodle-status-color: var(--poodle-color-status-info, #3b82f6);
  }

  .poodle-status-indicator[data-status="success"] {
    --poodle-status-color: var(--poodle-color-status-success);
  }

  .poodle-status-indicator[data-status="warning"] {
    --poodle-status-color: var(--poodle-color-status-warning);
  }

  .poodle-status-indicator[data-status="danger"] {
    --poodle-status-color: var(--poodle-color-status-danger);
  }

  .poodle-status-indicator[data-status="pending"] {
    --poodle-status-color: var(--poodle-color-accent-base);
  }

  .poodle-status-indicator__dot {
    flex: 0 0 auto;
    width: 0.5625rem;
    height: 0.5625rem;
    border-radius: 999px;
    background: var(--poodle-status-color);
    box-shadow: 0 0 0 0.125rem color-mix(in srgb, var(--poodle-status-color) 18%, transparent);
  }

  .poodle-status-indicator[data-status="pending"] .poodle-status-indicator__dot {
    animation: status-pulse 1s ease-in-out infinite alternate;
  }

  .poodle-status-indicator__label {
    min-width: 0;
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1.3;
  }

  /* Size variants */
  .poodle-status-indicator[data-size="xs"] .poodle-status-indicator__dot { width: 0.375rem; height: 0.375rem; }
  .poodle-status-indicator[data-size="xs"] .poodle-status-indicator__label { font-size: 0.625rem; }
  .poodle-status-indicator[data-size="xs"] { gap: 0.3125rem; }

  .poodle-status-indicator[data-size="sm"] .poodle-status-indicator__dot { width: 0.4375rem; height: 0.4375rem; }
  .poodle-status-indicator[data-size="sm"] .poodle-status-indicator__label { font-size: 0.6875rem; }
  .poodle-status-indicator[data-size="sm"] { gap: 0.375rem; }

  .poodle-status-indicator[data-size="lg"] .poodle-status-indicator__dot { width: 0.6875rem; height: 0.6875rem; }
  .poodle-status-indicator[data-size="lg"] .poodle-status-indicator__label { font-size: 0.8125rem; }
  .poodle-status-indicator[data-size="lg"] { gap: 0.5rem; }

  .poodle-status-indicator[data-size="xl"] .poodle-status-indicator__dot { width: 0.8125rem; height: 0.8125rem; }
  .poodle-status-indicator[data-size="xl"] .poodle-status-indicator__label { font-size: 0.875rem; }
  .poodle-status-indicator[data-size="xl"] { gap: 0.5625rem; }

  /* Density variants */
  .poodle-status-indicator[data-density="compact"] { gap: 0.25rem; }
  .poodle-status-indicator[data-density="comfortable"] { gap: 0.625rem; }

  @keyframes status-pulse {
    from {
      opacity: 0.55;
    }

    to {
      opacity: 1;
    }
  }
</style>
