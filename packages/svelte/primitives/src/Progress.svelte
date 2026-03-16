<script lang="ts">
  export let value: number | null = null;
  export let max = 100;
  export let isIndeterminate = false;
  export let ariaLabel: string | null = null;
  export let valueText: string | null = null;

  $: safeMax = max <= 0 ? 100 : max;
  $: safeValue = value === null ? null : Math.min(Math.max(value, 0), safeMax);
  $: percentage = safeValue === null ? 0 : safeValue / safeMax;
  $: computedValueText =
    !isIndeterminate && safeValue !== null ? `${Math.round(percentage * 100)}%` : null;
</script>

<div
  class="progress"
  data-indeterminate={isIndeterminate}
  role="progressbar"
  aria-label={ariaLabel ?? undefined}
  aria-valuemin={isIndeterminate ? undefined : 0}
  aria-valuemax={isIndeterminate ? undefined : safeMax}
  aria-valuenow={isIndeterminate || safeValue === null ? undefined : safeValue}
  aria-valuetext={valueText ?? computedValueText ?? undefined}
>
  <span
    class="progress__indicator"
    style={isIndeterminate ? undefined : `transform: scaleX(${percentage});`}
  ></span>
</div>

<style>
  .progress {
    position: relative;
    overflow: hidden;
    width: 100%;
    min-height: 0.5rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--pug-surface) 96%, var(--pug-color-text-primary));
  }

  .progress__indicator {
    position: absolute;
    inset: 0;
    transform-origin: left center;
    border-radius: inherit;
    background: linear-gradient(
      90deg,
      color-mix(in srgb, var(--pug-color-accent-base) 88%, white),
      var(--pug-color-accent-base)
    );
    transition: transform var(--pug-motion-duration-standard) var(--pug-motion-easing-standard);
  }

  .progress[data-indeterminate="true"] .progress__indicator {
    width: 40%;
    transform: translateX(-100%);
    animation: progress-indeterminate 1.2s ease-in-out infinite;
  }

  @keyframes progress-indeterminate {
    to {
      transform: translateX(250%);
    }
  }
</style>
