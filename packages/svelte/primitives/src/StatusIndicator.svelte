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
    --poodle-status-color: var(--poodle-color-text-secondary);
    display: inline-flex;
    align-items: center;
    gap: 0.4375rem;
    color: var(--poodle-color-text-primary);
    min-width: 0;
  }

  .status-indicator[data-status="info"] {
    --poodle-status-color: var(--poodle-color-accent-base);
  }

  .status-indicator[data-status="success"] {
    --poodle-status-color: var(--poodle-color-status-success);
  }

  .status-indicator[data-status="warning"] {
    --poodle-status-color: var(--poodle-color-status-warning);
  }

  .status-indicator[data-status="danger"] {
    --poodle-status-color: var(--poodle-color-status-danger);
  }

  .status-indicator[data-status="pending"] {
    --poodle-status-color: var(--poodle-color-accent-base);
  }

  .status-indicator__dot {
    flex: 0 0 auto;
    width: 0.5625rem;
    height: 0.5625rem;
    border-radius: 999px;
    background: var(--poodle-status-color);
    box-shadow: 0 0 0 0.125rem color-mix(in srgb, var(--poodle-status-color) 18%, transparent);
  }

  .status-indicator[data-status="pending"] .status-indicator__dot {
    animation: status-pulse 1s ease-in-out infinite alternate;
  }

  .status-indicator__label {
    min-width: 0;
    font-family: var(--poodle-typography-label-family);
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
