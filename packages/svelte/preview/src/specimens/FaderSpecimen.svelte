<script lang="ts">
  import { Fader } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let gain = $state(0);
  let frequency = $state(1000);
</script>

<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Vertical and horizontal"><div class="row"><Fader bind:value={gain} min={-60} max={12} format={{ type: "db" }} ariaLabel="Vertical gain" /><Fader value={0.65} orientation="horizontal" ariaLabel="Horizontal fader" /></div></SpecimenGroup>
  <SpecimenGroup label="Linear / log / bipolar laws"><div class="row"><Fader value={0.4} ariaLabel="Linear fader" /><Fader bind:value={frequency} min={20} max={20_000} law={{ type: "logarithmic" }} format={{ type: "hz" }} ariaLabel="Log fader" /><Fader value={0} min={-1} max={1} law={{ type: "bipolar-center", center: 0 }} ariaLabel="Bipolar fader" /></div></SpecimenGroup>
  <SpecimenGroup label="Detents"><Fader value={0.5} detents={[0.25, 0.5, 0.75]} detentSnap={0.03} ariaLabel="Detented fader" /></SpecimenGroup>
  <SpecimenGroup label="Coarse / fine drag (Shift)"><Fader value={0.4} ariaLabel="Fine-adjust fader" /></SpecimenGroup>
  <SpecimenGroup label="Automation touch"><Fader value={0.7} automation="touched" ariaLabel="Touched automation" /></SpecimenGroup>
  <SpecimenGroup label="Type-in (Enter)"><Fader value={0.25} ariaLabel="Type-in fader" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard bounds"><div class="row"><Fader value={0} ariaLabel="Minimum fader" /><Fader value={1} ariaLabel="Maximum fader" /></div></SpecimenGroup>
  <SpecimenGroup label="Disabled"><Fader value={0.5} disabled ariaLabel="Disabled fader" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<Fader value={0.6} {size} ariaLabel={`Fader ${size} size`} />{/snippet}
  {#snippet densities(density)}<Fader value={0.6} {density} ariaLabel={`Fader ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; }
.row { display: flex; align-items: center; gap: 1.5rem; flex-wrap: wrap; }</style>
