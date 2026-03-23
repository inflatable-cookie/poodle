<script lang="ts">
  import { Meter, Eyebrow } from "@flint/svelte-primitives";
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Default (50%)</Eyebrow>
    <Meter value={50} ariaLabel="Storage usage" />
  </div>

  <div class="specimen__group">
    <Eyebrow>With thresholds</Eyebrow>
    <Meter value={82} low={25} high={75} optimum={50} ariaLabel="CPU usage" />
    <p>82% — above high threshold</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Low value (optimal range)</Eyebrow>
    <Meter value={30} low={25} high={75} optimum={50} ariaLabel="Memory usage" />
    <p>30% — within normal range</p>
  </div>

  <div class="specimen__group">
    <Eyebrow>Custom range (0–500)</Eyebrow>
    <Meter value={350} min={0} max={500} ariaLabel="API calls" />
    <p>350 / 500 API calls used</p>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 20rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__group p {
    margin: 0;
    font-size: 0.875rem;
    color: var(--flint-color-text-secondary);
  }
</style>
