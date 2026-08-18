<script lang="ts">
  import { DragNumberField } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let value = $state(0.5);
  let integer = $state(4);
  let gain = $state(-12.4);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Default"><DragNumberField bind:value min={0} max={1} step={0.01} ariaLabel="Default value" /></SpecimenGroup>
  <SpecimenGroup label="Integer step"><DragNumberField bind:value={integer} min={0} max={10} step={1} ariaLabel="Integer value" /></SpecimenGroup>
  <SpecimenGroup label="Formatted dB"><DragNumberField bind:value={gain} min={-60} max={12} step={0.1} format={{ type: "db" }} ariaLabel="Gain" /></SpecimenGroup>
  <SpecimenGroup label="Coarse / fine drag (Shift)"><DragNumberField value={0.4} min={0} max={1} step={0.01} ariaLabel="Drag value" /></SpecimenGroup>
  <SpecimenGroup label="Direct entry (click)"><DragNumberField value={250} min={0} max={5000} step={1} format={{ type: "milliseconds" }} ariaLabel="Attack time" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard bounds (Home / End)"><div class="row"><DragNumberField value={0} min={0} max={1} step={0.1} ariaLabel="Minimum" /><DragNumberField value={1} min={0} max={1} step={0.1} ariaLabel="Maximum" /></div></SpecimenGroup>
  <SpecimenGroup label="Negative range"><DragNumberField value={-7} min={-24} max={24} step={1} format={{ type: "semitones" }} ariaLabel="Transpose" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><DragNumberField value={0.5} disabled ariaLabel="Disabled value" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<DragNumberField value={-12.4} format={{ type: "db" }} {size} ariaLabel={`Drag number ${size} size`} />{/snippet}
  {#snippet densities(density)}<DragNumberField value={-12.4} format={{ type: "db" }} {density} ariaLabel={`Drag number ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1rem; }
.row { display: flex; gap: .75rem; }</style>
