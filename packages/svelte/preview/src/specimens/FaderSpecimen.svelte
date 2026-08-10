<script lang="ts">
  import { Fader } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let gain = $state(0);
  let frequency = $state(1000);
</script>

<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>Vertical and horizontal</h3><div class="row"><Fader bind:value={gain} min={-60} max={12} format={{ type: "db" }} ariaLabel="Vertical gain" /><Fader value={0.65} orientation="horizontal" ariaLabel="Horizontal fader" /></div></section>
  <section><h3>Linear / log / bipolar laws</h3><div class="row"><Fader value={0.4} ariaLabel="Linear fader" /><Fader bind:value={frequency} min={20} max={20_000} law={{ type: "logarithmic" }} format={{ type: "hz" }} ariaLabel="Log fader" /><Fader value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} ariaLabel="Bipolar fader" /></div></section>
  <section><h3>Detents</h3><Fader value={0.5} detents={[0.25, 0.5, 0.75]} detentSnap={0.03} ariaLabel="Detented fader" /></section>
  <section><h3>Coarse / fine drag (Shift)</h3><Fader value={0.4} ariaLabel="Fine-adjust fader" /></section>
  <section><h3>Automation touch</h3><Fader value={0.7} automation="touched" ariaLabel="Touched automation" /></section>
  <section><h3>Type-in (Enter)</h3><Fader value={0.25} ariaLabel="Type-in fader" /></section>
  <section><h3>Keyboard bounds</h3><div class="row"><Fader value={0} ariaLabel="Minimum fader" /><Fader value={1} ariaLabel="Maximum fader" /></div></section>
  <section><h3>Disabled</h3><Fader value={0.5} disabled ariaLabel="Disabled fader" /></section>
</div>
  {#snippet sizes(size)}<Fader value={0.6} {size} ariaLabel={`Fader ${size} size`} />{/snippet}
  {#snippet densities(density)}<Fader value={0.6} {density} ariaLabel={`Fader ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; } section { display: grid; gap: .75rem; } h3 { margin: 0; color: var(--poodle-color-text-secondary); font-size: .75rem; } .row { display: flex; align-items: center; gap: 1.5rem; flex-wrap: wrap; }</style>
