<script lang="ts">
  import { XYPad } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let x = $state(0.5); let y = $state(0.5);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Centered / default"><XYPad bind:x bind:y ariaLabel="Centered pad" /></SpecimenGroup>
  <SpecimenGroup label="Corners"><div class="row"><XYPad x={0} y={0} ariaLabel="Lower-left corner" /><XYPad x={1} y={1} ariaLabel="Upper-right corner" /></div></SpecimenGroup>
  <SpecimenGroup label="Independent nonlinear laws"><XYPad x={1000} y={0.6} minX={20} maxX={20_000} lawX={{ type: "logarithmic" }} lawY={{ type: "exponential", exponent: 2 }} formatX={{ type: "hz" }} formatY={{ type: "percent" }} ariaLabel="Nonlinear pad" /></SpecimenGroup>
  <SpecimenGroup label="Coarse / fine drag (Shift)"><XYPad x={0.4} y={0.6} ariaLabel="Fine drag pad" /></SpecimenGroup>
  <SpecimenGroup label="Reset (double-click)"><XYPad x={0.8} y={0.2} defaultX={0.5} defaultY={0.5} ariaLabel="Reset pad" /></SpecimenGroup>
  <SpecimenGroup label="Automation state"><XYPad x={0.7} y={0.2} automation="writing" ariaLabel="Automated pad" /></SpecimenGroup>
  <SpecimenGroup label="Keyboard axis bounds"><XYPad x={0} y={1} ariaLabel="Bounded pad" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><XYPad x={0.5} y={0.5} disabled ariaLabel="Disabled pad" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<XYPad x={0.4} y={0.6} {size} ariaLabel={`XY pad ${size} size`} />{/snippet}
  {#snippet densities(density)}<XYPad x={0.4} y={0.6} {density} ariaLabel={`XY pad ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; }
.row { display: flex; gap: 1rem; flex-wrap: wrap; }</style>
