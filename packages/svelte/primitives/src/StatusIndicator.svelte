<script lang="ts">
  import type { StatusTone } from "./types";

  export let status: StatusTone = "neutral";
  export let label: string | null = null;
  export let ariaLabel: string | null = null;
</script>

<span class="status-indicator" data-status={status} aria-label={ariaLabel ?? undefined}>
  <span class="status-indicator__dot" aria-hidden="true"></span>
  {#if label}
    <span class="status-indicator__label">{label}</span>
  {:else}
    <slot />
  {/if}
</span>

<style>
  .status-indicator {
    --pug-status-color: var(--pug-color-text-secondary);
    display: inline-flex;
    align-items: center;
    gap: 0.4375rem;
    color: var(--pug-color-text-primary);
    min-width: 0;
  }

  .status-indicator[data-status="info"] {
    --pug-status-color: var(--pug-color-accent-base);
  }

  .status-indicator[data-status="success"] {
    --pug-status-color: var(--pug-color-status-success);
  }

  .status-indicator[data-status="warning"] {
    --pug-status-color: var(--pug-color-status-warning);
  }

  .status-indicator[data-status="danger"] {
    --pug-status-color: var(--pug-color-status-danger);
  }

  .status-indicator[data-status="pending"] {
    --pug-status-color: var(--pug-color-accent-base);
  }

  .status-indicator__dot {
    flex: 0 0 auto;
    width: 0.5625rem;
    height: 0.5625rem;
    border-radius: 999px;
    background: var(--pug-status-color);
    box-shadow: 0 0 0 0.125rem color-mix(in srgb, var(--pug-status-color) 18%, transparent);
  }

  .status-indicator[data-status="pending"] .status-indicator__dot {
    animation: status-pulse 1s ease-in-out infinite alternate;
  }

  .status-indicator__label {
    min-width: 0;
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1.3;
  }

  @keyframes status-pulse {
    from {
      opacity: 0.55;
    }

    to {
      opacity: 1;
    }
  }
</style>
