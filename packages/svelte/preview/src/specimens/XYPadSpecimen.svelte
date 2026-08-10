<script lang="ts">
  import { XYPad } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let x = $state(0.5); let y = $state(0.5);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>Centered / default</h3><XYPad bind:x bind:y ariaLabel="Centered pad" /></section>
  <section><h3>Corners</h3><div class="row"><XYPad x={0} y={0} ariaLabel="Lower-left corner" /><XYPad x={1} y={1} ariaLabel="Upper-right corner" /></div></section>
  <section><h3>Independent nonlinear laws</h3><XYPad x={1000} y={0.6} minX={20} maxX={20_000} lawX={{ type: "logarithmic" }} lawY={{ type: "exponential", exponent: 2 }} formatX={{ type: "hz" }} formatY={{ type: "percent" }} ariaLabel="Nonlinear pad" /></section>
  <section><h3>Coarse / fine drag (Shift)</h3><XYPad x={0.4} y={0.6} ariaLabel="Fine drag pad" /></section>
  <section><h3>Reset (double-click)</h3><XYPad x={0.8} y={0.2} defaultX={0.5} defaultY={0.5} ariaLabel="Reset pad" /></section>
  <section><h3>Automation state</h3><XYPad x={0.7} y={0.2} automation="writing" ariaLabel="Automated pad" /></section>
  <section><h3>Keyboard axis bounds</h3><XYPad x={0} y={1} ariaLabel="Bounded pad" /></section>
  <section><h3>Disabled</h3><XYPad x={0.5} y={0.5} disabled ariaLabel="Disabled pad" /></section>
</div>
  {#snippet sizes(size)}<XYPad x={0.4} y={0.6} {size} ariaLabel={`XY pad ${size} size`} />{/snippet}
  {#snippet densities(density)}<XYPad x={0.4} y={0.6} {density} ariaLabel={`XY pad ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; } section { display: grid; gap: .75rem; } h3 { margin: 0; color: var(--poodle-color-text-secondary); font-size: .75rem; } .row { display: flex; gap: 1rem; flex-wrap: wrap; }</style>
