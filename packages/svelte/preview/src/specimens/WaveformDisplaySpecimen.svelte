<script lang="ts">
  import { WaveformDisplay } from "@inflatable-cookie/poodle-svelte";
  import type { WaveformPeakPyramid } from "@inflatable-cookie/poodle-core";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";
  const fine = Array.from({ length: 64 }, (_, index) => ({ min: -Math.abs(Math.sin(index * .31)) * .8, max: Math.abs(Math.sin(index * .23)) * .9 }));
  const pyramid: WaveformPeakPyramid = { sampleCount: 64, levels: [{ samplesPerPeak: 1, peaks: fine }, { samplesPerPeak: 4, peaks: Array.from({ length: 16 }, (_, index) => ({ min: Math.min(...fine.slice(index * 4, index * 4 + 4).map((peak) => peak.min)), max: Math.max(...fine.slice(index * 4, index * 4 + 4).map((peak) => peak.max)) })) }] };
</script>
<SpecimenLayout variantDirection="row"><div class="page">
  <SpecimenGroup label="Peak pyramid / cursor"><WaveformDisplay {pyramid} cursorSample={24} ariaLabel="Clip preview" /></SpecimenGroup>
  <SpecimenGroup label="Zoomed viewport"><WaveformDisplay {pyramid} visibleStart={16} visibleEnd={48} columnCount={32} ariaLabel="Zoomed clip" /></SpecimenGroup>
  <SpecimenGroup label="Forward and ordered selection"><WaveformDisplay {pyramid} cursorSample={42} selection={{ start: 12, end: 42 }} ariaLabel="Selected clip" /></SpecimenGroup>
  <SpecimenGroup label="Empty"><WaveformDisplay pyramid={{ sampleCount: 0, levels: [] }} ariaLabel="Empty waveform" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><WaveformDisplay {pyramid} selection={{ start: 8, end: 28 }} disabled ariaLabel="Disabled waveform" /></SpecimenGroup>
  <SpecimenGroup label="Inspector ceiling"><WaveformDisplay {pyramid} columnCount={4096} ariaLabel="Bounded inspector waveform" /></SpecimenGroup>
</div>
  {#snippet sizes(size)}<WaveformDisplay {pyramid} cursorSample={24} {size} ariaLabel={`Waveform ${size} size`} />{/snippet}
  {#snippet densities(density)}<WaveformDisplay {pyramid} cursorSample={24} {density} ariaLabel={`Waveform ${density} density`} />{/snippet}
</SpecimenLayout>
<style>.page { display: grid; gap: 1.5rem; }</style>
