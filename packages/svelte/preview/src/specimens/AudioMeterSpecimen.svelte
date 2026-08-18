<script lang="ts">
  import { audioMeterTransition, createAudioMeterContext, type AudioMeterMode } from "@inflatable-cookie/poodle-core";
  import { AudioMeter } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const make = (mode: AudioMeterMode, peak: number, atMs = 300) => audioMeterTransition(createAudioMeterContext({ mode }), { type: "PUSH_FRAME", frame: { atMs, peak, meanSquare: peak * peak * 0.6, durationMs: atMs } }).context;
  const vu = make("vu", 0.72);
  const ppm = make("ppm", 0.72, 10);
  const samplePeak = make("sample-peak", 0.86, 16);
  const rms = make("rms", 0.55);
  const clipped = make("sample-peak", 1.1, 16);
  const reset = audioMeterTransition(clipped, { type: "RESET_CLIP" }).context;
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Batched rendering"><p class="note">High-count consoles opt into the <a href="#components/meter-surface">MeterSurface specimen</a>; this page stays the standalone evidence.</p></SpecimenGroup>
  <SpecimenGroup label="VU — 300 ms integration"><AudioMeter context={vu} ariaLabel="VU meter" /></SpecimenGroup>
  <SpecimenGroup label="PPM"><AudioMeter context={ppm} ariaLabel="PPM meter" /></SpecimenGroup>
  <SpecimenGroup label="Sample peak"><AudioMeter context={samplePeak} ariaLabel="Sample peak meter" /></SpecimenGroup>
  <SpecimenGroup label="RMS window"><AudioMeter context={rms} ariaLabel="RMS meter" /></SpecimenGroup>
  <SpecimenGroup label="Bar and segment styles"><div class="row"><AudioMeter context={ppm} style="bar" orientation="horizontal" ariaLabel="Bar meter" /><AudioMeter context={ppm} style="segments" orientation="horizontal" ariaLabel="Segment meter" /></div></SpecimenGroup>
  <SpecimenGroup label="Mono and stereo"><div class="row"><AudioMeter context={ppm} ariaLabel="Mono meter" /><AudioMeter context={ppm} rightContext={rms} ariaLabel="Stereo meter" /></div></SpecimenGroup>
  <SpecimenGroup label="Vertical and horizontal"><div class="row"><AudioMeter context={ppm} ariaLabel="Vertical meter" /><AudioMeter context={ppm} orientation="horizontal" ariaLabel="Horizontal meter" /></div></SpecimenGroup>
  <SpecimenGroup label="Peak hold"><AudioMeter context={samplePeak} ariaLabel="Peak hold meter" /></SpecimenGroup>
  <SpecimenGroup label="Clip latch and manual reset"><div class="row"><AudioMeter context={clipped} ariaLabel="Clipped meter" /><AudioMeter context={reset} ariaLabel="Reset clip meter" /></div></SpecimenGroup>
</div>
  {#snippet sizes(size)}<AudioMeter context={ppm} {size} ariaLabel={`Meter ${size} size`} />{/snippet}
  {#snippet densities(density)}<AudioMeter context={ppm} {density} ariaLabel={`Meter ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; }
.note { margin: 0; color: var(--poodle-color-text-secondary); font-size: .8125rem; } .row { display: flex; align-items: end; gap: 1.5rem; flex-wrap: wrap; }</style>
