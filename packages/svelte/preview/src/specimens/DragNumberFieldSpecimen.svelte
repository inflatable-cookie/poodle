<script lang="ts">
  import { DragNumberField } from "@inflatable-cookie/poodle-svelte";
  let gain = $state(-6);
  let time = $state(250);
</script>
<div class="poodle-drag-number-specimen">
  <DragNumberField bind:value={gain} min={-60} max={12} step={0.1} format={{ type: "db" }} ariaLabel="Gain" />
  <DragNumberField bind:value={time} min={0} max={5000} step={1} format={{ type: "milliseconds" }} ariaLabel="Attack time" />
</div>
<style>.poodle-drag-number-specimen { display: flex; gap: 1rem; }</style>
