<script lang="ts">
  import { audioMeterTransition, createAudioMeterContext, type AudioMeterMode } from "@inflatable-cookie/poodle-core";
  import { AudioMeter } from "@inflatable-cookie/poodle-svelte";
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
  <section><h3>Batched rendering</h3><p class="note">High-count consoles opt into the <a href="#components/meter-surface">MeterSurface specimen</a>; this page stays the standalone evidence.</p></section>
  <section><h3>VU — 300 ms integration</h3><AudioMeter context={vu} ariaLabel="VU meter" /></section>
  <section><h3>PPM</h3><AudioMeter context={ppm} ariaLabel="PPM meter" /></section>
  <section><h3>Sample peak</h3><AudioMeter context={samplePeak} ariaLabel="Sample peak meter" /></section>
  <section><h3>RMS window</h3><AudioMeter context={rms} ariaLabel="RMS meter" /></section>
  <section><h3>Bar and segment styles</h3><div class="row"><AudioMeter context={ppm} style="bar" orientation="horizontal" ariaLabel="Bar meter" /><AudioMeter context={ppm} style="segments" orientation="horizontal" ariaLabel="Segment meter" /></div></section>
  <section><h3>Mono and stereo</h3><div class="row"><AudioMeter context={ppm} ariaLabel="Mono meter" /><AudioMeter context={ppm} rightContext={rms} ariaLabel="Stereo meter" /></div></section>
  <section><h3>Vertical and horizontal</h3><div class="row"><AudioMeter context={ppm} ariaLabel="Vertical meter" /><AudioMeter context={ppm} orientation="horizontal" ariaLabel="Horizontal meter" /></div></section>
  <section><h3>Peak hold</h3><AudioMeter context={samplePeak} ariaLabel="Peak hold meter" /></section>
  <section><h3>Clip latch and manual reset</h3><div class="row"><AudioMeter context={clipped} ariaLabel="Clipped meter" /><AudioMeter context={reset} ariaLabel="Reset clip meter" /></div></section>
</div>
  {#snippet sizes(size)}<AudioMeter context={ppm} {size} ariaLabel={`Meter ${size} size`} />{/snippet}
  {#snippet densities(density)}<AudioMeter context={ppm} {density} ariaLabel={`Meter ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; } section { display: grid; gap: .75rem; } h3 { margin: 0; color: var(--poodle-color-text-secondary); font-size: .75rem; } .note { margin: 0; color: var(--poodle-color-text-secondary); font-size: .8125rem; } .row { display: flex; align-items: end; gap: 1.5rem; flex-wrap: wrap; }</style>
