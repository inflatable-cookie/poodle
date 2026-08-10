<script lang="ts">
  import { createGainReductionMeterContext } from "@inflatable-cookie/poodle-core";
  import { GainReductionMeter } from "@inflatable-cookie/poodle-svelte";
  let vertical = $state(createGainReductionMeterContext({ reductionDb: 12, ballisticDb: 12 }));
  let horizontal = $state(createGainReductionMeterContext({ maxReductionDb: 20, reductionDb: 7, ballisticDb: 7 }));
</script>

<div class="specimen">
  <GainReductionMeter bind:context={vertical} ariaLabel="Compressor gain reduction" />
  <GainReductionMeter bind:context={horizontal} orientation="horizontal" style="bar" ariaLabel="Limiter gain reduction" />
</div>
<style>.specimen { display: flex; align-items: center; gap: 2rem; }</style>
