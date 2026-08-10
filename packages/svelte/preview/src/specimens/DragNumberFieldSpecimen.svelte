<script lang="ts">
  import { DragNumberField } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let value = $state(0.5);
  let integer = $state(4);
  let gain = $state(-12.4);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>Default</h3><DragNumberField bind:value min={0} max={1} step={0.01} ariaLabel="Default value" /></section>
  <section><h3>Integer step</h3><DragNumberField bind:value={integer} min={0} max={10} step={1} ariaLabel="Integer value" /></section>
  <section><h3>Formatted dB</h3><DragNumberField bind:value={gain} min={-60} max={12} step={0.1} format={{ type: "db" }} ariaLabel="Gain" /></section>
  <section><h3>Coarse / fine drag (Shift)</h3><DragNumberField value={0.4} min={0} max={1} step={0.01} ariaLabel="Drag value" /></section>
  <section><h3>Direct entry (click)</h3><DragNumberField value={250} min={0} max={5000} step={1} format={{ type: "milliseconds" }} ariaLabel="Attack time" /></section>
  <section><h3>Keyboard bounds (Home / End)</h3><div class="row"><DragNumberField value={0} min={0} max={1} step={0.1} ariaLabel="Minimum" /><DragNumberField value={1} min={0} max={1} step={0.1} ariaLabel="Maximum" /></div></section>
  <section><h3>Negative range</h3><DragNumberField value={-7} min={-24} max={24} step={1} format={{ type: "semitones" }} ariaLabel="Transpose" /></section>
  <section><h3>Disabled</h3><DragNumberField value={0.5} disabled ariaLabel="Disabled value" /></section>
</div>
  {#snippet sizes(size)}<DragNumberField value={-12.4} format={{ type: "db" }} {size} ariaLabel={`Drag number ${size} size`} />{/snippet}
  {#snippet densities(density)}<DragNumberField value={-12.4} format={{ type: "db" }} {density} ariaLabel={`Drag number ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1rem; } section { display: grid; gap: .5rem; } h3 { margin: 0; color: var(--poodle-color-text-secondary); font-size: .75rem; } .row { display: flex; gap: .75rem; }</style>
