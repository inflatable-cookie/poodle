<script lang="ts">
  import { ValueReadout } from "@inflatable-cookie/poodle-svelte";
</script>
<div class="poodle-readout-specimen">
  <ValueReadout value={-12.4} min={-60} max={6} format={{ type: "db" }} ariaLabel="Gain" />
  <ValueReadout value={12_500} min={20} max={20_000} format={{ type: "hz" }} ariaLabel="Frequency" />
  <ValueReadout value={60} min={0} max={127} format={{ type: "note" }} ariaLabel="Note" />
  <ValueReadout value={0.375} format={{ type: "percent" }} ariaLabel="Mix" />
</div>
<style>.poodle-readout-specimen { display: flex; flex-wrap: wrap; gap: 0.75rem; }</style>
