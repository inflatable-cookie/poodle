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

<div class="poodle-meter" aria-label={ariaLabel ?? undefined}>
  <meter
    class="poodle-meter__native"
    min={min}
    max={safeMax}
    {low}
    {high}
    {optimum}
    value={safeValue}
  ></meter>
  <span class="poodle-meter__track" aria-hidden="true">
    <span class="poodle-meter__fill" style={`width: ${percentage}%;`}></span>
  </span>
</div>

<style>
  .poodle-meter {
    display: grid;
    gap: 0;
    width: 100%;
  }

  .poodle-meter__native {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .poodle-meter__track {
    position: relative;
    display: block;
    overflow: hidden;
    min-height: 0.5rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary));
  }

  .poodle-meter__fill {
    display: block;
    height: 100%;
    border-radius: inherit;
    background: linear-gradient(
      90deg,
      color-mix(in srgb, var(--poodle-color-status-success) 82%, white),
      var(--poodle-color-status-success)
    );
  }
</style>
