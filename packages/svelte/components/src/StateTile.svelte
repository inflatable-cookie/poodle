<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/state-tile.css";
  import type { Snippet } from "svelte";
  import Icon from "./Icon.svelte";

  let { label, value, trend = null, trendLabel = null, hasSparkline = false, sparkline }: {
    label: string;
    value: string;
    trend?: string | null;
    trendLabel?: string | null;
    hasSparkline?: boolean;
    sparkline?: Snippet;
  } = $props();
  const trendIcon = $derived(trend === "up" ? "trending-up" : trend === "down" ? "trending-down" : "arrow-right");
</script>

<div class="poodle-state-tile" data-component="state-tile">
  <span class="poodle-state-tile__label">{label}</span>
  <div class="poodle-state-tile__body">
    <strong class="poodle-state-tile__value">{value}</strong>
    {#if hasSparkline}<div class="poodle-state-tile__sparkline">{@render sparkline?.()}</div>{/if}
  </div>
  {#if trend}
    <span class="poodle-state-tile__trend" data-trend={trend}>
      <span class="poodle-state-tile__trend-arrow" aria-hidden="true"><Icon name={trendIcon} /></span>
      <span>{trendLabel ?? trend}</span>
    </span>
  {/if}
</div>
