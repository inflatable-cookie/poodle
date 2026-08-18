<script lang="ts">
  import { AudioSwitch } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  let latch = $state(0); let momentary = $state(0); let multi = $state(1);
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Off / on latch"><div class="row"><AudioSwitch state={0} ariaLabel="Off latch" /><AudioSwitch bind:state={latch} ariaLabel="Interactive latch" /><AudioSwitch state={1} ariaLabel="On latch" /></div></SpecimenGroup>
  <SpecimenGroup label="Held / released momentary"><AudioSwitch mode="momentary" bind:state={momentary} ariaLabel="Momentary trigger" /></SpecimenGroup>
  <SpecimenGroup label="Three-state cycle with labels"><AudioSwitch mode="multi" bind:state={multi} stateCount={3} stateLabels={["Low", "Mid", "High"]} ariaLabel="Range" /></SpecimenGroup>
  <SpecimenGroup label="Lamp override"><div class="row"><AudioSwitch state={0} lampOn={true} ariaLabel="Off with lamp" /><AudioSwitch state={1} lampOn={false} ariaLabel="On without lamp" /></div></SpecimenGroup>
  <SpecimenGroup label="Pressed / focused"><AudioSwitch state={1} ariaLabel="Focus and press switch" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><AudioSwitch state={1} disabled ariaLabel="Disabled switch" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<AudioSwitch state={1} {size} ariaLabel={`Audio switch ${size} size`} />{/snippet}
  {#snippet densities(density)}<AudioSwitch state={1} {density} ariaLabel={`Audio switch ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1rem; }
.row { display: flex; gap: 1rem; flex-wrap: wrap; }</style>
