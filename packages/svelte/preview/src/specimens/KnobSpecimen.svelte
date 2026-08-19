<script lang="ts">
  import { Knob } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let gain = $state(-12);
  let frequency = $state(1000);
  let stepped = $state(4);
</script>

<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Linear / default reset"><Knob bind:value={gain} min={-60} max={6} defaultValue={0} keyboardStep={1} format={{ type: "db" }} ariaLabel="Gain" /></SpecimenGroup>
  <SpecimenGroup label="Logarithmic frequency"><Knob bind:value={frequency} min={20} max={20_000} law={{ type: "logarithmic" }} defaultValue={440} keyboardStep={10} format={{ type: "hz" }} ariaLabel="Frequency" /></SpecimenGroup>
  <SpecimenGroup label="Bipolar center"><Knob value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} keyboardStep={0.1} ariaLabel="Pan" /></SpecimenGroup>
  <SpecimenGroup label="Stepped values"><Knob bind:value={stepped} min={0} max={10} law={{ type: "stepped", step: 1 }} keyboardStep={1} ariaLabel="Stepped value" /></SpecimenGroup>
  <SpecimenGroup label="Fine drag (Shift)"><Knob value={0.42} ariaLabel="Fine drag" /></SpecimenGroup>
  <SpecimenGroup label="Circular mode"><Knob value={0.8} dragMode="circular" ariaLabel="Circular drag" /></SpecimenGroup>
  <SpecimenGroup label="Automation state"><Knob value={0.7} automation="writing" ariaLabel="Automated value" /></SpecimenGroup>
  <SpecimenGroup label="Type-in and keyboard bounds"><div class="row"><Knob value={0.25} ariaLabel="Type-in value" /><Knob value={0} ariaLabel="Minimum" /><Knob value={1} ariaLabel="Maximum" /></div></SpecimenGroup>
  <SpecimenGroup label="Disabled"><Knob value={0.5} disabled ariaLabel="Disabled knob" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<Knob value={0.6} {size} ariaLabel={`Knob ${size} size`} />{/snippet}
  {#snippet densities(density)}<Knob value={0.6} {density} ariaLabel={`Knob ${density} density`} />{/snippet}
</SpecimenLayout>

<style>
  .page { display: grid; gap: 1.5rem; }
.row { display: flex; gap: 1rem; }
</style>
