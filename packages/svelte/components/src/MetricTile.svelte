<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/metric-tile.css";
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

