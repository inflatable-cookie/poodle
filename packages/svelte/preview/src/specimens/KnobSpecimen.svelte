<script lang="ts">
  import { Knob } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let gain = $state(-12);
  let frequency = $state(1000);
  let stepped = $state(4);
</script>

<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>Linear / default reset</h3><Knob bind:value={gain} min={-60} max={6} defaultValue={0} keyboardStep={1} format={{ type: "db" }} ariaLabel="Gain" /></section>
  <section><h3>Logarithmic frequency</h3><Knob bind:value={frequency} min={20} max={20_000} law={{ type: "logarithmic" }} defaultValue={440} keyboardStep={10} format={{ type: "hz" }} ariaLabel="Frequency" /></section>
  <section><h3>Bipolar center</h3><Knob value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} keyboardStep={0.1} ariaLabel="Pan" /></section>
  <section><h3>Stepped values</h3><Knob bind:value={stepped} min={0} max={10} law={{ type: "stepped", step: 1 }} keyboardStep={1} ariaLabel="Stepped value" /></section>
  <section><h3>Fine drag (Shift)</h3><Knob value={0.42} ariaLabel="Fine drag" /></section>
  <section><h3>Circular mode</h3><Knob value={0.8} dragMode="circular" ariaLabel="Circular drag" /></section>
  <section><h3>Automation state</h3><Knob value={0.7} automation="writing" ariaLabel="Automated value" /></section>
  <section><h3>Type-in (Enter)</h3><Knob value={0.25} ariaLabel="Type-in value" /></section>
  <section><h3>Keyboard bounds (Home / End)</h3><div class="row"><Knob value={0} ariaLabel="Minimum" /><Knob value={1} ariaLabel="Maximum" /></div></section>
  <section><h3>Disabled</h3><Knob value={0.5} disabled ariaLabel="Disabled knob" /></section>
</div>
  {#snippet sizes(size)}<Knob value={0.6} {size} ariaLabel={`Knob ${size} size`} />{/snippet}
  {#snippet densities(density)}<Knob value={0.6} {density} ariaLabel={`Knob ${density} density`} />{/snippet}
</SpecimenLayout>

<style>
  .page { display: grid; gap: 1.5rem; } section { display: grid; gap: .75rem; } h3 { margin: 0; color: var(--poodle-color-text-secondary); font-size: .75rem; } .row { display: flex; gap: 1rem; }
</style>
