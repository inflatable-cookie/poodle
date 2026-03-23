<script lang="ts">
  import { Icon } from "@flint/svelte-primitives";

  export let label: string;
  export let value: string;
  export let ariaLabel: string | null = null;
  export let trend: "up" | "down" | "flat" | null = null;
  export let trendLabel: string | null = null;
  export let sparklineData: number[] | null = null;

  $: sparklinePath = sparklineData && sparklineData.length > 1 ? buildSparkline(sparklineData) : null;

  function buildSparkline(data: number[]): string {
    const min = Math.min(...data);
    const max = Math.max(...data);
    const range = max - min || 1;
    const w = 64;
    const h = 24;
    const pad = 1;

    return data
      .map((v, i) => {
        const x = pad + (i / (data.length - 1)) * (w - pad * 2);
        const y = pad + (1 - (v - min) / range) * (h - pad * 2);
        return `${i === 0 ? "M" : "L"}${x.toFixed(1)} ${y.toFixed(1)}`;
      })
      .join(" ");
  }
</script>

<div class="state-tile" aria-label={ariaLabel ?? `${label}: ${value}`}>
  <span class="state-tile__label">{label}</span>
  <div class="state-tile__body">
    <strong class="state-tile__value">{value}</strong>
    {#if sparklinePath}
      <svg
        class="state-tile__sparkline"
        viewBox="0 0 64 24"
        fill="none"
        aria-hidden="true"
      >
        <path
          d={sparklinePath}
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    {/if}
  </div>
  {#if trend}
    <span class="state-tile__trend" data-trend={trend}>
      <span class="state-tile__trend-arrow" aria-hidden="true">
        {#if trend === "up"}<Icon name="trending-up" size="sm" />{:else if trend === "down"}<Icon name="trending-down" size="sm" />{:else}<Icon name="arrow-right" size="sm" />{/if}
      </span>
      {#if trendLabel}
        <span>{trendLabel}</span>
      {/if}
    </span>
  {/if}
</div>

<style>
  .state-tile {
    display: grid;
    gap: var(--flint-space-inline-sm);
    padding: var(--flint-space-panel-y) var(--flint-space-panel-x);
    border: 0.0625rem solid transparent;
    border-radius: var(--flint-radius-surface);
    background: color-mix(in srgb, var(--flint-color-background-surface) 60%, transparent);
  }

  .state-tile__label {
    color: var(--flint-color-text-secondary);
    font-family: var(--flint-typography-code-family);
    font-size: 0.75rem;
  }

  .state-tile__body {
    display: flex;
    align-items: center;
    gap: var(--flint-space-inline-md);
  }

  .state-tile__value {
    font-size: 1rem;
  }

  .state-tile__sparkline {
    width: 4rem;
    height: 1.5rem;
    color: var(--flint-color-text-tertiary);
    flex-shrink: 0;
  }

  .state-tile__trend {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
    font-family: var(--flint-typography-code-family);
    color: var(--flint-color-text-secondary);
  }

  .state-tile__trend[data-trend="up"] {
    color: var(--flint-color-status-success, #22c55e);
  }

  .state-tile__trend[data-trend="down"] {
    color: var(--flint-color-status-danger, #ef4444);
  }

  .state-tile__trend[data-trend="flat"] {
    color: var(--flint-color-text-tertiary);
  }

  .state-tile__trend-arrow {
    font-size: 0.875rem;
    line-height: 1;
  }

  :global([data-theme="light"]) .state-tile {
    background: var(--flint-treatment-surface-fill);
  }
</style>
