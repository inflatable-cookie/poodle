import { WaveformDisplay } from "@inflatable-cookie/poodle-react";
import type { WaveformPeakPyramid } from "@inflatable-cookie/poodle-core";
import {AudioAxes, AudioSpecimenPage as Page} from "./AudioSpecimen";
import { SpecimenGroup } from "../SpecimenGroup";
const fine = Array.from({ length: 64 }, (_, index) => ({ min: -Math.abs(Math.sin(index * .31)) * .8, max: Math.abs(Math.sin(index * .23)) * .9 }));
const pyramid: WaveformPeakPyramid = { sampleCount: 64, levels: [{ samplesPerPeak: 1, peaks: fine }, { samplesPerPeak: 4, peaks: Array.from({ length: 16 }, (_, index) => ({ min: Math.min(...fine.slice(index * 4, index * 4 + 4).map((peak) => peak.min)), max: Math.max(...fine.slice(index * 4, index * 4 + 4).map((peak) => peak.max)) })) }] };
export function WaveformDisplaySpecimen() { return <Page>
  <SpecimenGroup label="Peak pyramid / cursor"><WaveformDisplay pyramid={pyramid} cursorSample={24} ariaLabel="Clip preview" /></SpecimenGroup>
  <SpecimenGroup label="Zoomed viewport"><WaveformDisplay pyramid={pyramid} visibleStart={16} visibleEnd={48} columnCount={32} ariaLabel="Zoomed clip" /></SpecimenGroup>
  <SpecimenGroup label="Forward and ordered selection"><WaveformDisplay pyramid={pyramid} cursorSample={42} selection={{ start: 12, end: 42 }} ariaLabel="Selected clip" /></SpecimenGroup>
  <SpecimenGroup label="Empty"><WaveformDisplay pyramid={{ sampleCount: 0, levels: [] }} ariaLabel="Empty waveform" /></SpecimenGroup>
  <SpecimenGroup label="Disabled"><WaveformDisplay pyramid={pyramid} selection={{ start: 8, end: 28 }} disabled ariaLabel="Disabled waveform" /></SpecimenGroup>
  <SpecimenGroup label="Inspector ceiling"><WaveformDisplay pyramid={pyramid} columnCount={4096} ariaLabel="Bounded inspector waveform" /></SpecimenGroup>
  <AudioAxes render={(props, label) => <WaveformDisplay {...props} pyramid={pyramid} cursorSample={24} ariaLabel={`Waveform ${label}`} />} />
</Page>; }
