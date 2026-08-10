<script lang="ts">
  import { WaveformDisplay } from "@inflatable-cookie/poodle-svelte";
  import type { WaveformPeakPyramid } from "@inflatable-cookie/poodle-core";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const fine = Array.from({ length: 64 }, (_, index) => ({ min: -Math.abs(Math.sin(index * .31)) * .8, max: Math.abs(Math.sin(index * .23)) * .9 }));
  const pyramid: WaveformPeakPyramid = { sampleCount: 64, levels: [{ samplesPerPeak: 1, peaks: fine }, { samplesPerPeak: 4, peaks: Array.from({ length: 16 }, (_, index) => ({ min: Math.min(...fine.slice(index * 4, index * 4 + 4).map((peak) => peak.min)), max: Math.max(...fine.slice(index * 4, index * 4 + 4).map((peak) => peak.max)) })) }] };
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <section><h3>Peak pyramid / cursor</h3><WaveformDisplay {pyramid} cursorSample={24} ariaLabel="Clip preview" /></section>
  <section><h3>Zoomed viewport</h3><WaveformDisplay {pyramid} visibleStart={16} visibleEnd={48} columnCount={32} ariaLabel="Zoomed clip" /></section>
  <section><h3>Forward and ordered selection</h3><WaveformDisplay {pyramid} cursorSample={42} selection={{ start: 12, end: 42 }} ariaLabel="Selected clip" /></section>
  <section><h3>Empty</h3><WaveformDisplay pyramid={{ sampleCount: 0, levels: [] }} ariaLabel="Empty waveform" /></section>
  <section><h3>Disabled</h3><WaveformDisplay {pyramid} selection={{ start: 8, end: 28 }} disabled ariaLabel="Disabled waveform" /></section>
  <section><h3>Inspector ceiling</h3><WaveformDisplay {pyramid} columnCount={4096} ariaLabel="Bounded inspector waveform" /></section>
</div>
  {#snippet sizes(size)}<WaveformDisplay {pyramid} cursorSample={24} {size} ariaLabel={`Waveform ${size} size`} />{/snippet}
  {#snippet densities(density)}<WaveformDisplay {pyramid} cursorSample={24} {density} ariaLabel={`Waveform ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page{display:grid;gap:1.5rem}section{display:grid;gap:.75rem}h3{margin:0;color:var(--poodle-color-text-secondary);font-size:.75rem}</style>
