<script lang="ts">
  export let value = 0;
  export let min = 0;
  export let max = 100;
  export let low: number | null = null;
  export let high: number | null = null;
  export let optimum: number | null = null;
  export let ariaLabel: string | null = null;

  $: safeMax = max <= min ? min + 1 : max;
  $: safeValue = Math.min(Math.max(value, min), safeMax);
  $: percentage = ((safeValue - min) / (safeMax - min)) * 100;
</script>

<div class="meter" aria-label={ariaLabel ?? undefined}>
  <meter
    class="meter__native"
    min={min}
    max={safeMax}
    {low}
    {high}
    {optimum}
    value={safeValue}
  ></meter>
  <span class="meter__track" aria-hidden="true">
    <span class="meter__fill" style={`width: ${percentage}%;`}></span>
  </span>
</div>

<style>
  .meter {
    display: grid;
    gap: 0;
    width: 100%;
  }

  .meter__native {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .meter__track {
    position: relative;
    display: block;
    overflow: hidden;
    min-height: 0.5rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--flint-surface) 96%, var(--flint-color-text-primary));
  }

  .meter__fill {
    display: block;
    height: 100%;
    border-radius: inherit;
    background: linear-gradient(
      90deg,
      color-mix(in srgb, var(--flint-color-status-success) 82%, white),
      var(--flint-color-status-success)
    );
  }
</style>
