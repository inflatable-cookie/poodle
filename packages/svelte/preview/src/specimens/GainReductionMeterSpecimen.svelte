<script lang="ts">
  import { createGainReductionMeterContext, gainReductionMeterTransition } from "@inflatable-cookie/poodle-core";
  import { GainReductionMeter } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const context = (value: number, enabled = true) => createGainReductionMeterContext({ reductionDb: value, ballisticDb: value, enabled });
  const attacked = gainReductionMeterTransition(createGainReductionMeterContext(), { type: "PUSH_FRAME", frame: { atMs: 10, reductionDb: 18, durationMs: 10 } }).context;
  const released = gainReductionMeterTransition(attacked, { type: "PUSH_FRAME", frame: { atMs: 310, reductionDb: 4, durationMs: 300 } }).context;
  const invalid = gainReductionMeterTransition(context(12), { type: "PUSH_FRAME", frame: { atMs: Number.NaN, reductionDb: -1, durationMs: 0 } }).context;
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="No reduction / reset"><GainReductionMeter context={context(0)} ariaLabel="No gain reduction" /></SpecimenGroup>
  <SpecimenGroup label="Attack"><GainReductionMeter context={attacked} ariaLabel="Attack response" /></SpecimenGroup>
  <SpecimenGroup label="Release"><GainReductionMeter context={released} ariaLabel="Release response" /></SpecimenGroup>
  <SpecimenGroup label="Maximum reduction"><GainReductionMeter context={context(30)} ariaLabel="Maximum reduction" /></SpecimenGroup>
  <SpecimenGroup label="Bar and segment styles"><div class="row"><GainReductionMeter context={context(12)} style="bar" orientation="horizontal" ariaLabel="Bar reduction" /><GainReductionMeter context={context(12)} style="segments" orientation="horizontal" ariaLabel="Segment reduction" /></div></SpecimenGroup>
  <SpecimenGroup label="Vertical and horizontal"><div class="row"><GainReductionMeter context={context(12)} ariaLabel="Vertical reduction" /><GainReductionMeter context={context(12)} orientation="horizontal" ariaLabel="Horizontal reduction" /></div></SpecimenGroup>
  <SpecimenGroup label="Invalid-frame rejection"><GainReductionMeter context={invalid} ariaLabel="Invalid frame rejected" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><GainReductionMeter context={context(12, false)} ariaLabel="Disabled reduction" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<GainReductionMeter context={context(12)} {size} ariaLabel={`Gain reduction ${size} size`} />{/snippet}
  {#snippet densities(density)}<GainReductionMeter context={context(12)} {density} ariaLabel={`Gain reduction ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; }
.row { display: flex; align-items: center; gap: 1.5rem; flex-wrap: wrap; }</style>
