<script lang="ts">
  import { Knob, ValueReadout } from "@inflatable-cookie/poodle-svelte";
  let gain = $state(-12);
  let frequency = $state(1000);
</script>

<div class="poodle-audio-specimen">
  <label>Gain <Knob bind:value={gain} min={-60} max={6} defaultValue={0} keyboardStep={1} format={{ type: "db" }} ariaLabel="Gain" /></label>
  <ValueReadout value={gain} min={-60} max={6} format={{ type: "db" }} ariaLabel="Gain value" />
  <label>Frequency <Knob bind:value={frequency} min={20} max={20_000} law={{ type: "logarithmic" }} defaultValue={440} keyboardStep={10} format={{ type: "hz" }} dragMode="circular" ariaLabel="Frequency" /></label>
  <label>Pan <Knob value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} keyboardStep={0.1} ariaLabel="Pan" /></label>
</div>

<style>.poodle-audio-specimen { display: flex; align-items: end; gap: 1.5rem; } label { display: grid; gap: 0.5rem; color: var(--poodle-color-text-secondary); }</style>
