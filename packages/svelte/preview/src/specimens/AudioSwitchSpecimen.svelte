<script lang="ts">
  import { AudioSwitch } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let latch = $state(0); let momentary = $state(0); let multi = $state(1);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>Off / on latch</h3><div class="row"><AudioSwitch state={0} ariaLabel="Off latch" /><AudioSwitch bind:state={latch} ariaLabel="Interactive latch" /><AudioSwitch state={1} ariaLabel="On latch" /></div></section>
  <section><h3>Held / released momentary</h3><AudioSwitch mode="momentary" bind:state={momentary} ariaLabel="Momentary trigger" /></section>
  <section><h3>Three-state cycle with labels</h3><AudioSwitch mode="multi" bind:state={multi} stateCount={3} stateLabels={["Low", "Mid", "High"]} ariaLabel="Range" /></section>
  <section><h3>Lamp override</h3><div class="row"><AudioSwitch state={0} lampOn={true} ariaLabel="Off with lamp" /><AudioSwitch state={1} lampOn={false} ariaLabel="On without lamp" /></div></section>
  <section><h3>Pressed / focused</h3><AudioSwitch state={1} ariaLabel="Focus and press switch" /></section>
  <section><h3>Disabled</h3><AudioSwitch state={1} disabled ariaLabel="Disabled switch" /></section>
</div>
  {#snippet sizes(size)}<AudioSwitch state={1} {size} ariaLabel={`Audio switch ${size} size`} />{/snippet}
  {#snippet densities(density)}<AudioSwitch state={1} {density} ariaLabel={`Audio switch ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1rem; } section { display: grid; gap: .5rem; } h3 { margin: 0; color: var(--poodle-color-text-secondary); font-size: .75rem; } .row { display: flex; gap: 1rem; flex-wrap: wrap; }</style>
