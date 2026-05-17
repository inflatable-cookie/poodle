<script lang="ts">
  import { default as Icon } from "./Icon.svelte";
  import { getUiPresentation } from "./presentation";

  import type { ControlDensity } from "./types";

  let {
    label,
    value,
    ariaLabel = null,
    trend = null,
    trendLabel = null,
    sparklineData = null,
    density = null,
  }: {
    label: string;
    value: string;
    ariaLabel?: string | null;
    trend?: "up" | "down" | "flat" | null;
    trendLabel?: string | null;
    sparklineData?: number[] | null;
    density?: ControlDensity | null;
  } = $props();

  const uiPresentation = getUiPresentation();
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const sparklinePath = $derived(
    sparklineData && sparklineData.length > 1 ? buildSparkline(sparklineData) : null
  );

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

<div class="poodle-state-tile" data-density={resolvedDensity} aria-label={ariaLabel ?? `${label}: ${value}`}>
  <span class="poodle-state-tile__label">{label}</span>
  <div class="poodle-state-tile__body">
    <strong class="poodle-state-tile__value">{value}</strong>
    {#if sparklinePath}
      <svg
        class="poodle-state-tile__sparkline"
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
    <span class="poodle-state-tile__trend" data-trend={trend}>
      <span class="poodle-state-tile__trend-arrow" aria-hidden="true">
        {#if trend === "up"}<Icon name="trending-up" />{:else if trend === "down"}<Icon name="trending-down" />{:else}<Icon name="arrow-right" />{/if}
      </span>
      {#if trendLabel}
        <span>{trendLabel}</span>
      {/if}
    </span>
  {/if}
</div>

<style>
  .poodle-state-tile {
    --poodle-state-tile-gap: var(--poodle-space-inline-sm);
    --poodle-state-tile-padding-y: 0.625rem;
    --poodle-state-tile-padding-x: var(--poodle-space-panel-x);
    --poodle-state-tile-body-gap: var(--poodle-space-inline-md);
    display: grid;
    gap: var(--poodle-state-tile-gap);
    padding: var(--poodle-state-tile-padding-y) var(--poodle-state-tile-padding-x);
    border: 0.0625rem solid transparent;
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-surface) 60%, transparent);
  }

  .poodle-state-tile__label {
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
  }

  .poodle-state-tile__body {
    display: flex;
    align-items: center;
    gap: var(--poodle-state-tile-body-gap);
  }

  .poodle-state-tile__value {
    font-size: 1rem;
  }

  .poodle-state-tile__sparkline {
    width: 4rem;
    height: 1.5rem;
    color: var(--poodle-color-text-tertiary);
    flex-shrink: 0;
  }

  .poodle-state-tile__trend {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
    font-family: var(--poodle-typography-code-family);
    color: var(--poodle-color-text-secondary);
  }

  .poodle-state-tile__trend[data-trend="up"] {
    color: var(--poodle-color-status-success, #22c55e);
  }

  .poodle-state-tile__trend[data-trend="down"] {
    color: var(--poodle-color-status-danger, #ef4444);
  }

  .poodle-state-tile__trend[data-trend="flat"] {
    color: var(--poodle-color-text-tertiary);
  }

  .poodle-state-tile__trend-arrow {
    font-size: 0.875rem;
    line-height: 1;
  }

  .poodle-state-tile[data-density="compact"] {
    --poodle-state-tile-gap: 0.375rem;
    --poodle-state-tile-padding-y: 0.5rem;
    --poodle-state-tile-padding-x: 0.75rem;
    --poodle-state-tile-body-gap: 0.5rem;
  }

  .poodle-state-tile[data-density="comfortable"] {
    --poodle-state-tile-gap: 0.625rem;
    --poodle-state-tile-padding-y: 0.75rem;
    --poodle-state-tile-padding-x: 1.25rem;
    --poodle-state-tile-body-gap: 0.875rem;
  }

  :global([data-theme="light"]) .poodle-state-tile {
    background: var(--poodle-treatment-surface-fill);
  }
</style>
