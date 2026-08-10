<script lang="ts">
  import { ValueReadout, XYPad } from "@inflatable-cookie/poodle-svelte";
  let x = $state(0.35);
  let y = $state(0.7);
</script>

<div class="specimen">
  <XYPad bind:x bind:y ariaLabel="Filter position" />
  <span>X <ValueReadout value={x} format={{ type: "percent" }} /></span>
  <span>Y <ValueReadout value={y} format={{ type: "percent" }} /></span>
</div>
<style>.specimen { display: flex; align-items: end; gap: 1rem; } span { display: grid; gap: 0.25rem; }</style>
