<script lang="ts">
  import { createGainReductionMeterContext, gainReductionMeterTransition } from "@inflatable-cookie/poodle-core";
  import { GainReductionMeter } from "@inflatable-cookie/poodle-svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const context = (value: number, enabled = true) => createGainReductionMeterContext({ reductionDb: value, ballisticDb: value, enabled });
  const attacked = gainReductionMeterTransition(createGainReductionMeterContext(), { type: "PUSH_FRAME", frame: { atMs: 10, reductionDb: 18, durationMs: 10 } }).context;
  const released = gainReductionMeterTransition(attacked, { type: "PUSH_FRAME", frame: { atMs: 310, reductionDb: 4, durationMs: 300 } }).context;
  const invalid = gainReductionMeterTransition(context(12), { type: "PUSH_FRAME", frame: { atMs: Number.NaN, reductionDb: -1, durationMs: 0 } }).context;
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>No reduction</h3><GainReductionMeter context={context(0)} ariaLabel="No gain reduction" /></section>
  <section><h3>Attack</h3><GainReductionMeter context={attacked} ariaLabel="Attack response" /></section>
  <section><h3>Release</h3><GainReductionMeter context={released} ariaLabel="Release response" /></section>
  <section><h3>Maximum reduction</h3><GainReductionMeter context={context(30)} ariaLabel="Maximum reduction" /></section>
  <section><h3>Bar and segment styles</h3><div class="row"><GainReductionMeter context={context(12)} style="bar" orientation="horizontal" ariaLabel="Bar reduction" /><GainReductionMeter context={context(12)} style="segments" orientation="horizontal" ariaLabel="Segment reduction" /></div></section>
  <section><h3>Vertical and horizontal</h3><div class="row"><GainReductionMeter context={context(12)} ariaLabel="Vertical reduction" /><GainReductionMeter context={context(12)} orientation="horizontal" ariaLabel="Horizontal reduction" /></div></section>
  <section><h3>Invalid-frame rejection</h3><GainReductionMeter context={invalid} ariaLabel="Invalid frame rejected" /></section>
  <section><h3>Reset</h3><GainReductionMeter context={context(0)} ariaLabel="Reset reduction" /></section>
  <section><h3>Disabled</h3><GainReductionMeter context={context(12, false)} ariaLabel="Disabled reduction" /></section>
</div>
  {#snippet sizes(size)}<GainReductionMeter context={context(12)} {size} ariaLabel={`Gain reduction ${size} size`} />{/snippet}
  {#snippet densities(density)}<GainReductionMeter context={context(12)} {density} ariaLabel={`Gain reduction ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; } section { display: grid; gap: .75rem; } h3 { margin: 0; color: var(--poodle-color-text-secondary); font-size: .75rem; } .row { display: flex; align-items: center; gap: 1.5rem; flex-wrap: wrap; }</style>
